import asyncio
from router.orchestrator import Orchestrator
from settings import Settings

settings = Settings()
orchestrator = Orchestrator(settings)

query = "CAR T-cell therapies"
async def get_search_results(query: str = ""): 
    data = await orchestrator.handle_pubmed_bioxriv_web_search(search_text=query)

    print(data)


if __name__ == "__main__":
    asyncio.run(get_search_results(query))
    
