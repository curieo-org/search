import asyncio
import json

from llama_index.core import StorageContext
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from loguru import logger
from qdrant_client import AsyncQdrantClient

from app.grpc_types.agency_pb2 import (
    Double2D,
    Embeddings,
    Int2D,
    PubmedSource,
)
from app.settings import Settings
from app.utils.custom_vectorstore import (
    CurieoQueryBundle,
    CurieoVectorIndexRetriever,
    CurieoVectorStore,
    CurieoVectorStoreIndex,
)
from app.utils.database_helper import PubmedDatabaseUtils

logger.add(
    "file.log",
    rotation="500 MB",
    format="{time:YYYY-MM-DD at HH:mm:ss} | {level} | {message}",
)


class ClusterRetrievalEngine:
    def __init__(self, settings: Settings):
        self.settings = settings
        self.cluster_client = AsyncQdrantClient(
            url=self.settings.pubmed_cluster_qdrant.api_url,
            port=self.settings.pubmed_cluster_qdrant.api_port,
            api_key=self.settings.pubmed_cluster_qdrant.api_key.get_secret_value(),
            https=False,
        )
        self.cluster_retriever = CurieoVectorIndexRetriever(
            index=CurieoVectorStoreIndex(
                storage_context=StorageContext.from_defaults(
                    vector_store=CurieoVectorStore(
                        aclient=self.cluster_client,
                        collection_name=self.settings.pubmed_cluster_qdrant.collection_name,
                    )
                ),
                embed_model=TextEmbeddingsInference(
                    model_name="",
                    base_url=self.settings.embedding.api_url,
                    auth_token=self.settings.embedding.api_key.get_secret_value(),
                    timeout=60,
                    embed_batch_size=self.settings.embedding.batch_size,
                ),
            ),
            similarity_top_k=self.settings.pubmed_cluster_qdrant.top_k,
            sparse_top_k=self.settings.pubmed_cluster_qdrant.sparse_top_k,
        )
        self.cluster_relevance_criteria = (
            self.settings.pubmed_retrieval.cluster_relevance_criteria
        )
        self.pubmed_database = PubmedDatabaseUtils(settings.pubmed_database)

    async def retrieve_cluster_nodes(
        self, query: CurieoQueryBundle
    ) -> list[PubmedSource]:
        logger.info(f"search_text: {query.query_str}")
        if not len(query.embedding) and not len(query.sparse_embedding):
            return []

        extracted_nodes = await self.cluster_retriever.aretrieve(query)
        if not len(extracted_nodes):
            return []

        filtered_nodes = [
            n
            for n in extracted_nodes
            if n.score >= float(self.cluster_relevance_criteria)
        ]
        if not len(filtered_nodes):
            return []

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

        pubmed_titles, children_node_texts = await asyncio.gather(
            self.pubmed_database.get_pubmed_record_titles(pubmed_ids),
            self.pubmed_database.get_children_node_text(all_children_node_ids),
        )

        return [
            PubmedSource(
                pubmed_id=str(pubmed_id),
                title=str(pubmed_titles.get(pubmed_id, "")),
                abstract=child_node_json["text"],
                embeddings=Embeddings(
                    dense_embedding=child_node_json["dense_embedding"],
                    sparse_embedding=[
                        Double2D(values=child_node_json["sparse_embedding"]["vector"])
                    ],
                    sparse_indices=[
                        Int2D(values=child_node_json["sparse_embedding"]["indices"])
                    ],
                ),
            )
            for pubmed_id in nodes_dict
            for child_node_id in nodes_dict[pubmed_id].get("children_node_ids", [])
            if child_node_id in children_node_texts
            for child_node_json in [json.loads(children_node_texts.get(child_node_id))]
        ]
