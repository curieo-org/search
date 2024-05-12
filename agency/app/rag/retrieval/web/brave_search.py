import httpx
from llama_index.core.schema import TextNode

from app.caching.decorators import cached
from app.rag.utils.models import BraveSourceRecord
from app.services.search_utility import setup_logger
from app.settings import BraveSettings

logger = setup_logger("BraveSearchQueryEngine")


class BraveSearchQueryEngine:
    """Utility class for interacting with the Brave Search API.

    Searches are performed in an async manner using httpx.
    Searches are cached using the `cached` decorator with `search_text` as the
    caching key.
    """

    def __init__(self, settings: BraveSettings):
        self.settings = settings

    @cached("agency.brave.search.{search_text}")
    async def cached_brave_search(self, search_text: str) -> httpx.Response | None:
        logger.info("call_brave_search_api. query: " + search_text)

        url = (
            f"{self.settings.api_root}?"
            f"count={self.settings.result_count}&"
            f"q={search_text}&"
            f"goggles_id={str(self.settings.goggles_id.get_secret_value())}&"
            f"result_filter={','.join(self.settings.result_filter)}&"
            f"search_lang=en&extra_snippets=True&safesearch=strict"
        )

        headers = {
            "Accept": "application/json",
            "Accept-Encoding": "gzip",
            "X-Subscription-Token": str(
                self.settings.subscription_key.get_secret_value(),
            ),
        }

        try:
            logger.info("call_brave_search_api. endpoint: " + url)

            async with httpx.AsyncClient() as client:
                response = await client.get(url, headers=headers, timeout=2.0)

            response.raise_for_status()

            return response

        except Exception as e:
            logger.exception("Brave search failed", exc_info=e, stack_info=True)
            return None

    async def call_brave_search_api(self, search_text: str) -> list[TextNode]:
        try:
            if response := await self.cached_brave_search(search_text):
                web_response = response.json().get("web").get("results")
                if web_response:
                    return [
                        TextNode(
                            text=resp.get("description")
                            + "".join(resp.get("extra_snippets", [])),
                            metadata=BraveSourceRecord.model_validate(resp),
                        )
                        for resp in web_response
                    ]

            return []

        except Exception as e:
            logger.exception("Brave search failed", exc_info=e, stack_info=True)
            return []
