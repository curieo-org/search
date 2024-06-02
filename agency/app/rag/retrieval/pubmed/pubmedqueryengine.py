import asyncio
from typing import List, Tuple

from llama_index.core import StorageContext, VectorStoreIndex
from llama_index.core.retrievers import VectorIndexRetriever
from llama_index.core.vector_stores.types import VectorStoreQueryMode
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from llama_index.vector_stores.qdrant.utils import default_sparse_encoder
from qdrant_client import AsyncQdrantClient

from app.rag.retrieval.pubmed.utils.database import PubmedDatabaseUtils
from app.rag.utils.models import PubmedSourceRecord, RetrievedResult
from app.rag.utils.splade_embedding import SpladeEmbeddingsInference
from app.settings import Settings
from app.utils.custom_vectorstore import CurieoVectorStore
from app.utils.logging import setup_logger

logger = setup_logger("PubmedSearchQueryEngine")


class PubmedSearchQueryEngine:
    """Calls the pubmed database, processes the data and returns the result."""

    def __init__(self, settings: Settings):
        self.settings = settings

        self.parent_relevance_criteria = (
            self.settings.pubmed_retrieval.parent_relevance_criteria
        )
        self.cluster_relevance_criteria = (
            self.settings.pubmed_retrieval.cluster_relevance_criteria
        )
        self.pubmed_database = PubmedDatabaseUtils(settings.pubmed_database)

        self.embed_model = TextEmbeddingsInference(
            model_name="",
            base_url=self.settings.embedding.api_url,
            auth_token=self.settings.embedding.api_key.get_secret_value(),
            timeout=60,
            batch_size=self.settings.embedding.batch_size,
        )

        self.splade_model = SpladeEmbeddingsInference(
            model_name="",
            base_url=self.settings.spladeembedding.api,
            auth_token=self.settings.spladeembedding.api_key.get_secret_value(),
            timeout=60,
            batch_size=self.settings.spladeembedding.batch_size,
        )

        self.parent_client = AsyncQdrantClient(
            url=self.settings.pubmed_parent_qdrant.api_url,
            port=self.settings.pubmed_parent_qdrant.api_port,
            api_key=self.settings.pubmed_parent_qdrant.api_key.get_secret_value(),
            https=False,
        )
        self.cluster_client = AsyncQdrantClient(
            url=self.settings.pubmed_cluster_qdrant.api_url,
            port=self.settings.pubmed_cluster_qdrant.api_port,
            api_key=self.settings.pubmed_cluster_qdrant.api_key.get_secret_value(),
            https=False,
        )

        self.parent_vector_store = CurieoVectorStore(
            aclient=self.parent_client,
            collection_name=self.settings.pubmed_parent_qdrant.collection_name,
            enable_hybrid=True,
            sparse_query_fn=self.sparse_query_vectors,
            batch_size=20,
            sparse_doc_fn=default_sparse_encoder(
                "naver/efficient-splade-VI-BT-large-doc",
            ),
        )

        self.cluster_vector_store = CurieoVectorStore(
            aclient=self.cluster_client,
            collection_name=self.settings.pubmed_cluster_qdrant.collection_name,
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
            similarity_top_k=self.settings.pubmed_parent_qdrant.top_k,
            sparse_top_k=self.settings.pubmed_parent_qdrant.sparse_top_k,
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
            similarity_top_k=self.settings.pubmed_cluster_qdrant.top_k,
            sparse_top_k=self.settings.pubmed_cluster_qdrant.sparse_top_k,
            vector_store_query_mode=VectorStoreQueryMode.HYBRID,
            embed_model=self.embed_model,
        )

    def sparse_query_vectors(
        self,
        texts: List[str],
    ) -> Tuple[List[List[int]], List[List[float]]]:
        try:
            splade_embeddings = self.splade_model.get_text_embedding_batch(texts)
            indices = [
                [entry.get("index") for entry in sublist]
                for sublist in splade_embeddings
            ]
            vectors = [
                [entry.get("value") for entry in sublist]
                for sublist in splade_embeddings
            ]

            assert len(indices) == len(vectors)
            return indices, vectors
        except Exception as e:
            logger.exception("failed to query vectors from the splade model", e)
            return [], []

    def get_pubmed_url(self, pubmed_id: int) -> str:
        return f"{self.settings.pubmed_retrieval.url_prefix}/{pubmed_id}"

    async def call_pubmed_parent_vectors(
        self, search_text: str
    ) -> list[RetrievedResult]:
        logger.info(
            "PubmedSearchQueryEngine.call_pubmed_parent_vectors query: " + search_text
        )

        try:
            extracted_nodes = await self.parent_retriever.aretrieve(search_text)
            filtered_nodes = [
                n
                for n in extracted_nodes
                if n.score >= float(self.parent_relevance_criteria)
            ]

            pubmed_ids = [node.metadata.get("pubmedid", 0) for node in filtered_nodes]
            pubmed_titles = await self.pubmed_database.get_pubmed_record_titles(
                pubmed_ids
            )

            retrieved_results = [
                RetrievedResult.model_validate(
                    {
                        "text": node.get_text(),
                        "source": PubmedSourceRecord.model_validate(
                            {
                                "url": self.get_pubmed_url(
                                    node.metadata.get("pubmedid", 0)
                                ),
                                "title": pubmed_titles.get(
                                    node.metadata.get("pubmedid", 0), ""
                                ),
                                "abstract": node.get_text(),
                            }
                        ),
                    }
                )
                for node in filtered_nodes
            ]

            return retrieved_results
        except Exception as e:
            logger.exception("failed to retrieve data from the database", e)
            return []

    async def call_pubmed_cluster_vectors(
        self, search_text: str
    ) -> list[RetrievedResult]:
        logger.info(
            "PubmedSearchQueryEngine.call_pubmed_cluster_vectors query: " + search_text
        )

        try:
            # Retrieve and filter nodes
            extracted_nodes = await self.cluster_retriever.aretrieve(search_text)
            filtered_nodes = [
                n
                for n in extracted_nodes
                if n.score >= float(self.cluster_relevance_criteria)
            ]

            # Create a dictionary of pubmed_id and children_node_ids
            pubmed_ids = [node.metadata.get("pubmedid", 0) for node in filtered_nodes]
            nodes_dict = {
                node.metadata.get("pubmedid", 0): {
                    "children_node_ids": node.metadata.get("children_node_ids", []),
                }
                for node in filtered_nodes
            }
            all_children_node_ids = [
                item
                for sublist in nodes_dict.values()
                for item in sublist["children_node_ids"]
            ]

            (
                pubmed_titles,
                children_node_texts,
            ) = await asyncio.gather(
                self.pubmed_database.get_pubmed_record_titles(pubmed_ids),
                self.pubmed_database.get_children_node_text(all_children_node_ids),
            )

            result_nodes = [
                RetrievedResult.model_validate(
                    {
                        "text": children_node_texts.get(child_node_id, ""),
                        "source": PubmedSourceRecord.model_validate(
                            {
                                "url": self.get_pubmed_url(pubmed_id),
                                "title": pubmed_titles.get(pubmed_id, ""),
                                "abstract": children_node_texts.get(child_node_id, ""),
                            }
                        ),
                    }
                )
                for pubmed_id in nodes_dict
                for child_node_id in nodes_dict[pubmed_id].get("children_node_ids", [])
                if child_node_id in children_node_texts.keys()
            ]

            return result_nodes

        except Exception as e:
            logger.exception("failed to retrieve data from the database", e)
            return []
