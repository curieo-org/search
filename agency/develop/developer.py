import asyncio
import sys
from pathlib import Path

sys.path.append(Path.cwd())

from app.embedding.embedding_engine import EmbeddingEngine
from app.embedding.utils.custom_vectorstore import CurieoQueryBundle
from app.pubmed_retrieval.cluster_engine import ClusterRetrievalEngine
from app.pubmed_retrieval.parent_engine import ParentRetrievalEngine
from app.settings import Settings
from app.utils.logging import setup_logger

settings = Settings()
queryprocessengine = EmbeddingEngine(settings)
parent = ParentRetrievalEngine(settings)
cluster = ClusterRetrievalEngine(settings)
logger = setup_logger("Developer")

query = "Parasetamol in covid"
query = "CAR T-cell therapies"
query = "glp-1 combination therapy"


async def get_search_results(query: str = "") -> None:
    nodes = await queryprocessengine.query_process(query)
    parent_nodes = await parent.retrieve_parent_nodes(
        CurieoQueryBundle(
            query_str=nodes.query_str,
            embedding=nodes.embedding,
            sparse_embedding=nodes.sparse_embedding,
        )
    )

    cluster_nodes = await cluster.retrieve_cluster_nodes(
        CurieoQueryBundle(
            query_str=nodes.query_str,
            embedding=nodes.embedding,
            sparse_embedding=nodes.sparse_embedding,
        )
    )

    logger.info(f"Parent Nodes: {parent_nodes}")
    logger.info(f"Cluster Nodes: {cluster_nodes}")
    logger.info(f"Length Parent Nodes: {len(parent_nodes)}")
    logger.info(f"Length Cluster Nodes: {len(cluster_nodes)}")


if __name__ == "__main__":
    asyncio.run(get_search_results(query))
