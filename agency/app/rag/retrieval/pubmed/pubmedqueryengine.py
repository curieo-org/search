<<<<<<< HEAD
from collections import defaultdict
from typing import List, Tuple

=======
>>>>>>> main
from llama_index.core import VectorStoreIndex
from llama_index.core.schema import NodeWithScore
from llama_index.core.vector_stores import ExactMatchFilter
from llama_index.core.vector_stores.types import MetadataFilters, VectorStoreQueryMode
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from llama_index.vector_stores.qdrant import QdrantVectorStore
<<<<<<< HEAD
from qdrant_client import QdrantClient

from app.rag.utils.hierarchical_vector_index_retrieval import (
    HierarchialVectorIndexRetriever,
)  # type: ignore
from app.rag.utils.splade_embedding import SpladeEmbeddingsInference
from app.services.search_utility import setup_logger
=======
from llama_index.vector_stores.qdrant.utils import default_sparse_encoder
from qdrant_client import AsyncQdrantClient

>>>>>>> main
from app.settings import Settings
from app.utils.logging import setup_logger

logger = setup_logger("PubmedSearchQueryEngine")


class PubmedSearchQueryEngine:
    """Calls the pubmed database, processes the data and returns the result."""

    def sparse_query_vectors(
        self,
        texts: List[str],
    ) -> Tuple[List[List[int]], List[List[float]]]:
        """
        Computes vectors from logits and attention mask using ReLU, log, and max operations.

        Args:
            texts (List[str]): A list of strings representing the input text data.

        Returns:
            Tuple[List[List[int]], List[List[float]]]: A tuple containing two lists.
            The first list is a list of lists, where each sublist represents the indices of the input text data.
            The second list is a list of lists, where each sublist represents the corresponding vectors derived
            from the input text data.
        """
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

        self.parent_relevance_criteria = settings.llama_index.parent_relevance_criteria
        self.child_relevance_criteria = settings.llama_index.child_relevance_criteria

        self.embed_model = TextEmbeddingsInference(
            model_name="",
            base_url=self.settings.embedding.api,
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

        qdrant_settings = settings.qdrant
        # self.client = QdrantClient(
        #     url=qdrant_settings.api_url,
        #     port=None,
        #     api_key=qdrant_settings.api_key.get_secret_value(),
        #     https=True
        # )

        # For local development
        self.client = QdrantClient(
            url="localhost",
            port=6333,
            api_key=qdrant_settings.api_key.get_secret_value(),
            https=False,
        )

        self.vector_store = QdrantVectorStore(
            client=self.client,
            collection_name=qdrant_settings.collection_name,
            enable_hybrid=True,
            sparse_query_fn=self.sparse_query_vectors,
            batch_size=20,
            sparse_doc_fn=default_sparse_encoder(
                "naver/efficient-splade-VI-BT-large-doc",
            ),
            sparse_query_fn=default_sparse_encoder(
                "naver/efficient-splade-VI-BT-large-query",
            ),
        )

        self.retriever = HierarchialVectorIndexRetriever(
            index=VectorStoreIndex.from_vector_store(vector_store=self.vector_store),
            similarity_top_k=qdrant_settings.top_k,
            sparse_top_k=qdrant_settings.sparse_top_k,
            vector_store_query_mode=VectorStoreQueryMode.HYBRID,
<<<<<<< HEAD
            embed_model=self.embed_model,
=======
            embed_model=TextEmbeddingsInference(
                base_url=settings.embedding.api,
                model_name="",  # TODO: is "" correct?
            ),
>>>>>>> main
        )

    async def call_pubmed_vectors(self, search_text: str) -> list[NodeWithScore]:
        logger.info("PubmedSearchQueryEngine.call_pubmed_vectors query: " + search_text)
        parent_to_child = defaultdict(list)
        child_to_same_cluster = defaultdict(list)

        try:
            # find the parent nodes first
            parent_nodes = [
                n
<<<<<<< HEAD
                for n in self.retriever.retrieve(
                    search_text,
                    filters=MetadataFilters(
                        filters=[ExactMatchFilter(key="node_type", value="parent")]
                    ),
                )
                if n.score >= float(self.parent_relevance_criteria)
=======
                for n in await self.retriever.aretrieve(search_text)
                if n.score >= float(self.relevance_criteria)
>>>>>>> main
            ]

            # children nodes
            for node in parent_nodes:
                parent_id = node.metadata.get("parent_id")
                children_nodes = [
                    n
                    for n in self.retriever.retrieve(
                        search_text,
                        filters=MetadataFilters(
                            filters=[
                                ExactMatchFilter(key="node_type", value="child"),
                                ExactMatchFilter(key="parent_id", value=parent_id),
                            ]
                        ),
                    )
                    if n.score >= float(self.child_relevance_criteria)
                ]
                parent_to_child[parent_id] = children_nodes
            # find same cluster candidates
            for node in children_nodes[1:3]:
                cluster_id = node.metadata.get("cluster_id")
                children_same_cluster_nodes = self.retriever.retrieve(
                    search_text,
                    filters=MetadataFilters(
                        filters=[
                            ExactMatchFilter(key="node_type", value="child"),
                            ExactMatchFilter(key="cluster_id", value=cluster_id),
                        ]
                    ),
                )
                child_to_same_cluster[node._id].append(children_same_cluster_nodes)

            return {
                "parent_to_child": parent_to_child,
                "child_to_same_cluster": child_to_same_cluster,
            }
        except Exception as e:
            logger.exception("Pubmed search failed", e)
            return {}
