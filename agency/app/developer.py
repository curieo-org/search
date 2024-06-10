import asyncio

from app.settings import Settings
from app.utils.logging import setup_logger
from app.query_node_process.nodeprocessengine import QueryProcessorEngine
from app.pubmed_retrieval.parentretrievalengine import ParentRetrievalEngine
from app.pubmed_retrieval.clusterretrievalengine import ClusterRetrievalEngine
from app.utils.custom_vectorstore import (
    CurieoVectorStore,
    CurieoQueryBundle,
    CurieoVectorIndexRetriever
)

settings = Settings()
#orchestrator = Orchestrator(settings)
queryprocessengine = QueryProcessorEngine(settings)
parent = ParentRetrievalEngine(settings)
cluster = ClusterRetrievalEngine(settings)
logger = setup_logger("Developer")

query = "Parasetamol in covid"
query = "CAR T-cell therapies"
query = "glp-1 combination therapy"


async def get_search_results(query: str = "") -> None:
    # data = await orchestrator.handle_pubmed_web_search(search_text=query)

    # logger.info(f"Search results for query: {query}")
    # logger.info(f"Search results: {data}")

    nodes = await queryprocessengine.query_process(query)
    parent_nodes = await parent.retrieve_parent_nodes(CurieoQueryBundle(
        query_str=nodes["query_str"],
        embedding=nodes["embedding"],
        sparse_embedding=nodes["sparse_embedding"]
    ))
    
    cluster_nodes = await cluster.retrieve_cluster_nodes(CurieoQueryBundle(
        query_str=nodes["query_str"],
        embedding=nodes["embedding"],
        sparse_embedding=nodes["sparse_embedding"]
    ))


    print("Parent Nodes: ", parent_nodes)
    print("Cluster Nodes: ", cluster_nodes)


if __name__ == "__main__":
    asyncio.run(get_search_results(query))
