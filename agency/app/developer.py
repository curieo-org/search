import asyncio

from router.orchestrator import Orchestrator
from settings import Settings

settings = Settings()
orchestrator = Orchestrator(settings)

# query = "CAR T-cell therapies"
query = "glp-1 combination therapy"
# query = "Parasetamol in covid"


async def get_search_results(query: str = ""):
    data = await orchestrator.handle_pubmed_bioxriv_web_search(search_text=query)

    # pubmed_query_engine = PubmedSearchQueryEngine(settings)
    # data = await pubmed_query_engine.call_pubmed_parent_vectors(search_text=query)
    # data = await pubmed_query_engine.call_pubmed_cluster_vectors(search_text=query)

    # brave_query_engine = BraveSearchQueryEngine(settings.brave)
    # data = await brave_query_engine.call_brave_search_api(search_text=query)

    print(data)


if __name__ == "__main__":
    asyncio.run(get_search_results(query))
