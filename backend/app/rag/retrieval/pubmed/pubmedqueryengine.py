from typing import List

from llama_index.core import VectorStoreIndex
from llama_index.core.retrievers import VectorIndexRetriever
from llama_index.vector_stores.qdrant import QdrantVectorStore
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from llama_index.core.vector_stores.types import VectorStoreQueryMode
from llama_index.core.schema import BaseNode
from qdrant_client import QdrantClient

from app.services.search_utility import setup_logger
from app.config import QDRANT_API_KEY, QDRANT_API_URL, QDRANT_API_PORT, QDRANT_COLLECTION_NAME, QDRANT_TOP_K, QDRANT_SPARSE_TOP_K, EMBEDDING_MODEL_API, PUBMED_RELEVANCE_CRITERIA

logger = setup_logger("PubmedSearchQueryEngine")


class PubmedSearchQueryEngine:
    """
    This class implements the logic of call pubmed vector database.
    It calls the pubmed vector database and processes the data and returns the result.
    """

    def __init__(self, config):
        self.config = config
        
        self.client = QdrantClient(
            url=QDRANT_API_URL,
            port=QDRANT_API_PORT,
            api_key=str(QDRANT_API_KEY),
            https=False
            )
        
        self.vector_store = QdrantVectorStore(
            client=self.client,
            collection_name=QDRANT_COLLECTION_NAME,
            enable_hybrid = True,
            batch_size=20)

        self.retriever = VectorIndexRetriever(
            index=VectorStoreIndex.from_vector_store(vector_store=self.vector_store),
            similarity_top_k=int(QDRANT_TOP_K),
            sparse_top_k=int(QDRANT_SPARSE_TOP_K),
            vector_store_query_mode=VectorStoreQueryMode.HYBRID,
            embed_model=TextEmbeddingsInference(base_url=EMBEDDING_MODEL_API, model_name="")
        )z

    async def call_pubmed_vectors(self, search_text: str) -> List[BaseNode]:
        logger.info(
            "PubmedSearchQueryEngine.call_pubmed_vectors query: " + search_text
        )

        try:
            response = [eachNode.node for eachNode in self.retriever.retrieve(search_text) if eachNode.score >= float(PUBMED_RELEVANCE_CRITERIA)]
        except Exception as ex:
            raise ex
        return response