from llama_index.core import VectorStoreIndex
from llama_index.core.retrievers import VectorIndexRetriever
from llama_index.core.schema import NodeWithScore
from llama_index.core.vector_stores.types import VectorStoreQueryMode
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from llama_index.vector_stores.qdrant import QdrantVectorStore
from qdrant_client import AsyncQdrantClient

from app.settings import Settings
from app.utils.logging import setup_logger

logger = setup_logger("PubmedSearchQueryEngine")


class PubmedSearchQueryEngine:
    """Calls the pubmed database, processes the data and returns the result."""

    def __init__(self, settings: Settings):
        self.settings = settings

        self.relevance_criteria = settings.llama_index.relevance_criteria

        qdrant_settings = settings.qdrant

        self.client = AsyncQdrantClient(
            url=qdrant_settings.api_url,
            port=qdrant_settings.api_port,
            api_key=qdrant_settings.api_key.get_secret_value(),
            https=False,  # TODO: use https in prod
        )

        self.vector_store = QdrantVectorStore(
            aclient=self.client,
            collection_name=qdrant_settings.collection_name,
            enable_hybrid=True,
            batch_size=20,
        )

        self.retriever = VectorIndexRetriever(
            index=VectorStoreIndex.from_vector_store(vector_store=self.vector_store),
            similarity_top_k=qdrant_settings.top_k,
            sparse_top_k=qdrant_settings.sparse_top_k,
            vector_store_query_mode=VectorStoreQueryMode.HYBRID,
            embed_model=TextEmbeddingsInference(
                base_url=settings.embedding.api,
                model_name="",  # TODO: is "" correct?
            ),
        )

    async def call_pubmed_vectors(self, search_text: str) -> list[NodeWithScore]:
        logger.info("PubmedSearchQueryEngine.call_pubmed_vectors query: " + search_text)

        try:
            return [
                n
                for n in self.retriever.retrieve(search_text)
                if n.score >= float(self.relevance_criteria)
            ]
        except Exception as e:
            logger.exception("Pubmed search failed", e)
            return []
