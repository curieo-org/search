import asyncio

from router.orchestrator import Orchestrator
from settings import Settings

from app.rag.retrieval.pubmed.pubmedqueryengine import PubmedSearchQueryEngine
from app.rag.retrieval.web.brave_engine import BraveSearchQueryEngine

settings = Settings()
orchestrator = Orchestrator(settings)

#query = "CAR T-cell therapies"
query = "glp-1 combination therapy"
# query = "Parasetamol in covid"


async def get_search_results(query: str = ""):
    data = await orchestrator.handle_pubmed_bioxriv_web_search(search_text=query, rerank_llm_lingua_call=True)

    #pubmed_query_engine = PubmedSearchQueryEngine(settings)
    #data = await pubmed_query_engine.call_pubmed_parent_vectors(search_text=query)
    #data = await pubmed_query_engine.call_pubmed_cluster_vectors(search_text=query)

    # brave_query_engine = BraveSearchQueryEngine(settings.brave)
    # data = await brave_query_engine.call_brave_search_api(search_text=query)

    print(data)


if __name__ == "__main__":
    asyncio.run(get_search_results(query))
