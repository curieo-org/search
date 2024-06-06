# ruff: noqa: ERA001, ARG002, D205
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from llama_index.core import StorageContext
from qdrant_client import AsyncQdrantClient

from app.settings import Settings
from loguru import logger
from app.rag.retrieval.pubmed.utils.database import PubmedDatabaseUtils
from app.rag.utils.models import RetrievedResult
from app.utils.custom_vectorstore import (
    CurieoVectorStore,
    CurieoQueryBundle,
    CurieoVectorIndexRetriever,
    CurieoVectorStoreIndex
)

logger.add("file.log", rotation="500 MB", format="{time:YYYY-MM-DD at HH:mm:ss} | {level} | {message}")


class ParentRetrievalEngine:
    def __init__(self, settings: Settings):
        self.settings = settings
        self.parent_client = AsyncQdrantClient(
            url=self.settings.pubmed_parent_qdrant.api_url,
            port=self.settings.pubmed_parent_qdrant.api_port,
            api_key=self.settings.pubmed_parent_qdrant.api_key.get_secret_value(),
            https=False,
        )
        self.parent_retriever = CurieoVectorIndexRetriever(
            index=CurieoVectorStoreIndex(
                storage_context=StorageContext.from_defaults(
                    vector_store=CurieoVectorStore(
                        aclient=self.parent_client,
                        collection_name=self.settings.pubmed_parent_qdrant.collection_name
                    )
                ),
                embed_model=TextEmbeddingsInference(
                    model_name="",
                    base_url=self.settings.embedding.api_url,
                    auth_token=self.settings.embedding.api_key.get_secret_value(),
                    timeout=60,
                    embed_batch_size=self.settings.embedding.batch_size,
                )
            ),
            similarity_top_k=self.settings.pubmed_parent_qdrant.top_k,
            sparse_top_k=self.settings.pubmed_parent_qdrant.sparse_top_k,
        )
        self.parent_relevance_criteria = self.settings.pubmed_retrieval.parent_relevance_criteria
        self.pubmed_database = PubmedDatabaseUtils(settings.pubmed_database)

    async def retrieve_parent_nodes(
        self, query: CurieoQueryBundle
    ) -> dict:
        logger.info(f"query_process. search_text: {query.query_str}")   
        if not len(query.embedding) and not len(query.sparse_embedding):
            return []
        
        extracted_nodes = await self.parent_retriever.aretrieve(query)
        if not len(extracted_nodes):
            return []

        filtered_nodes = [
                n
                for n in extracted_nodes
                if n.score >= float(self.parent_relevance_criteria)
            ]
        
        if not len(filtered_nodes):
            return []
        pubmed_ids = [node.metadata.get("pubmedid", 0) for node in filtered_nodes]
        pubmed_titles = await self.pubmed_database.get_pubmed_record_titles(pubmed_ids)

        return [
            RetrievedResult.model_validate(
                {
                    "pubmed_id": node.metadata.get("pubmedid", 0),
                    "title": pubmed_titles.get(node.metadata.get("pubmedid", 0), ""),
                    "abstract": node.get_text()
                }
            )
            for node in filtered_nodes
        ]
    