from typing import List, Tuple

from llama_index.core import StorageContext, VectorStoreIndex
from llama_index.core.retrievers import VectorIndexRetriever
from llama_index.core.schema import NodeWithScore
from llama_index.core.vector_stores.types import VectorStoreQueryMode
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from llama_index.vector_stores.qdrant.utils import default_sparse_encoder
from qdrant_client import AsyncQdrantClient
from sqlalchemy import create_engine

from app.rag.utils.splade_embedding import SpladeEmbeddingsInference
from app.settings import Settings
from app.utils.custom_vectorstore import CurieoVectorStore
from app.utils.database_utils import run_select_sql
from app.utils.logging import setup_logger

logger = setup_logger("PubmedSearchQueryEngine")


class PubmedSearchQueryEngine:
    """Calls the pubmed database, processes the data and returns the result."""

    def sparse_query_vectors(
        self,
        texts: List[str],
    ) -> Tuple[List[List[int]], List[List[float]]]:
        splade_embeddings = self.splade_model.get_text_embedding_batch(texts)
        indices = [
            [entry.get("index") for entry in sublist] for sublist in splade_embeddings
        ]
        vectors = [
            [entry.get("value") for entry in sublist] for sublist in splade_embeddings
        ]

        assert len(indices) == len(vectors)
        return indices, vectors

    def __init__(self, settings: Settings):
        self.settings = settings

        self.parent_relevance_criteria = (
            self.settings.llama_index_helper.parent_relevance_criteria
        )
        self.cluster_relevance_criteria = (
            self.settings.llama_index_helper.cluster_relevance_criteria
        )
        self.engine = create_engine(self.settings.psql.connection.get_secret_value())

        self.embed_model = TextEmbeddingsInference(
            model_name="",
            base_url=self.settings.embedding.api_url,
            auth_token=self.settings.embedding.api_key.get_secret_value(),
            timeout=60,
            embed_batch_size=self.settings.embedding.embed_batch_size,
        )

        self.splade_model = SpladeEmbeddingsInference(
            model_name="",
            base_url=self.settings.spladeembedding.api,
            auth_token=self.settings.spladeembedding.api_key.get_secret_value(),
            timeout=60,
            embed_batch_size=self.settings.spladeembedding.embed_batch_size,
        )

        self.client = AsyncQdrantClient(
            url=self.settings.qdrant.api_url,
            port=self.settings.qdrant.api_port,
            api_key=self.settings.qdrant.api_key.get_secret_value(),
            https=False,
        )

        self.parent_vector_store = CurieoVectorStore(
            aclient=self.client,
            collection_name=self.settings.qdrant.parent_collection_name,
            enable_hybrid=True,
            sparse_query_fn=self.sparse_query_vectors,
            batch_size=20,
            sparse_doc_fn=default_sparse_encoder(
                "naver/efficient-splade-VI-BT-large-doc",
            ),
        )

        self.cluster_vector_store = CurieoVectorStore(
            aclient=self.client,
            collection_name=self.settings.qdrant.cluster_collection_name,
            enable_hybrid=True,
            sparse_query_fn=self.sparse_query_vectors,
            sparse_doc_fn=default_sparse_encoder(
                "naver/efficient-splade-VI-BT-large-doc",
            ),
        )

        self.parent_storage_context = StorageContext.from_defaults(
            vector_store=self.parent_vector_store
        )
        self.cluster_storage_context = StorageContext.from_defaults(
            vector_store=self.cluster_vector_store
        )

        self.parent_vectordb_index = VectorStoreIndex(
            storage_context=self.parent_storage_context,
            embed_model=self.embed_model,
            nodes=[],
        )
        self.parent_retriever = VectorIndexRetriever(
            index=self.parent_vectordb_index,
            similarity_top_k=self.settings.qdrant.parent_top_k,
            sparse_top_k=self.settings.qdrant.parent_sparse_top_k,
            vector_store_query_mode=VectorStoreQueryMode.HYBRID,
            embed_model=self.embed_model,
        )

        self.cluster_vectordb_index = VectorStoreIndex(
            storage_context=self.cluster_storage_context,
            embed_model=self.embed_model,
            nodes=[],
        )
        self.cluster_retriever = VectorIndexRetriever(
            index=self.cluster_vectordb_index,
            similarity_top_k=self.settings.qdrant.cluster_top_k,
            sparse_top_k=self.settings.qdrant.cluster_sparse_top_k,
            vector_store_query_mode=VectorStoreQueryMode.HYBRID,
            embed_model=self.embed_model,
        )

    async def call_pubmed_parent_vectors(self, search_text: str) -> list[NodeWithScore]:
        logger.info("PubmedSearchQueryEngine.call_pubmed_vectors query: " + search_text)

        try:
            return [
                n
                for n in await self.parent_retriever.aretrieve(search_text)
                if n.score >= float(self.parent_relevance_criteria)
            ]
        except Exception as e:
            logger.exception("Pubmed search failed", e)
            return []

    async def call_pubmed_cluster_vectors(
        self, search_text: str
    ) -> list[NodeWithScore]:
        logger.info("PubmedSearchQueryEngine.call_pubmed_vectors query: " + search_text)
        result_dict = {}

        try:
            # Retrieve and filter nodes
            extracted_nodes = await self.cluster_retriever.aretrieve(search_text)
            filtered_nodes = [
                n
                for n in extracted_nodes
                if n.score >= float(self.cluster_relevance_criteria)
            ]

            # Create a dictionary of pubmedid to children_node_ids
            nodes_dict = {
                node.metadata.get("pubmedid", 0): node.metadata.get(
                    "children_node_ids", []
                )
                for node in filtered_nodes
            }

            if len(nodes_dict):
                all_ids = [item for sublist in nodes_dict.values() for item in sublist]
                tuple_str = ", ".join(
                    f"'{item}'" if isinstance(item, str) else str(item)
                    for item in all_ids
                )
                query = self.settings.psql.ids_select_query.format(ids=tuple_str)
                result = run_select_sql(self.engine, query)
                result_dict = {
                    id: [record for record in result if record["id"] in nodes_dict[id]]
                    for id in nodes_dict
                }

            return result_dict
        except Exception as e:
            logger.exception("Pubmed search failed", e)
            return []
