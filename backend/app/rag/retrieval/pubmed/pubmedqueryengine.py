from typing import List

from llama_index.core import VectorStoreIndex
from llama_index.core.retrievers import VectorIndexRetriever
from llama_index.core.schema import NodeWithScore
from llama_index.core.vector_stores.types import VectorStoreQueryMode
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from llama_index.vector_stores.qdrant import QdrantVectorStore
from qdrant_client import QdrantClient

from app.services.search_utility import setup_logger
from app.settings import Settings

logger = setup_logger("PubmedSearchQueryEngine")


class PubmedSearchQueryEngine:
    """
    This class implements the logic of call pubmed vector database.
    It calls the pubmed vector database and processes the data and returns the result.
    """

    def __init__(self, settings: Settings):
        self.settings = settings

        self.relevance_criteria = settings.llama_index.relevance_criteria

        qdrant_settings = settings.qdrant

        self.client = QdrantClient(
            url=qdrant_settings.api_url,
            port=qdrant_settings.api_port,
            api_key=str(qdrant_settings.top_k),
            https=False,  # TODO: use https in prod
        )

        self.vector_store = QdrantVectorStore(
            client=self.client,
            collection_name=qdrant_settings.collection_name,
            enable_hybrid=True,
            batch_size=20,
        )

        self.retriever = VectorIndexRetriever(
            index=VectorStoreIndex.from_vector_store(vector_store=self.vector_store),
            similarity_top_k=int(qdrant_settings.top_k),
            sparse_top_k=int(qdrant_settings.sparse_top_k),
            vector_store_query_mode=VectorStoreQueryMode.HYBRID,
            embed_model=TextEmbeddingsInference(
                base_url=settings.embedding.api, model_name=""  # TODO: is "" correct?
            ),
        )

    async def call_pubmed_vectors(self, search_text: str) -> List[NodeWithScore]:
        logger.info("PubmedSearchQueryEngine.call_pubmed_vectors query: " + search_text)

        try:
            response = [
                n
                for n in self.retriever.retrieve(search_text)
                if n.score >= float(self.relevance_criteria)
            ]
        except Exception as ex:
            raise ex
        return response
