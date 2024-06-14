import asyncio

from app.pubmed_retrieval.clusterretrievalengine import ClusterRetrievalEngine
from app.pubmed_retrieval.parentretrievalengine import ParentRetrievalEngine
from app.query_node_process.nodeprocessengine import QueryProcessorEngine
from app.settings import Settings
from app.utils.custom_vectorstore import CurieoQueryBundle
from app.utils.logging import setup_logger

settings = Settings()
queryprocessengine = QueryProcessorEngine(settings)
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
            query_str=nodes["query_str"],
            embedding=nodes["embedding"],
            sparse_embedding=nodes["sparse_embedding"],
        )
    )

    cluster_nodes = await cluster.retrieve_cluster_nodes(
        CurieoQueryBundle(
            query_str=nodes["query_str"],
            embedding=nodes["embedding"],
            sparse_embedding=nodes["sparse_embedding"],
        )
    )

    logger.info(f"Parent Nodes: {parent_nodes}")
    logger.info(f"Cluster Nodes: {cluster_nodes}")


if __name__ == "__main__":
    asyncio.run(get_search_results(query))
