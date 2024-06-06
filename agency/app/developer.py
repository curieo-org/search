import asyncio

from router.orchestrator import Orchestrator
from settings import Settings
from utils.logging import setup_logger
from query_node_process.nodeprocessengine import QueryProcessorEngine
from pubmed_retrieval.parentretrievalengine import ParentRetrievalEngine
from utils.custom_vectorstore import (
    CurieoVectorStore,
    CurieoQueryBundle,
    CurieoVectorIndexRetriever
)

settings = Settings()
#orchestrator = Orchestrator(settings)
queryprocessengine = QueryProcessorEngine(settings)
parent = ParentRetrievalEngine(settings)
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
    print()


if __name__ == "__main__":
    asyncio.run(get_search_results(query))
