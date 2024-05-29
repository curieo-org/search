import logging
from typing import Any

import httpx
from pydantic import ValidationError

from app.caching.redis import fcached
from app.rag.retrieval.web.types import WebSearchApiResponse
from app.rag.utils.models import RetrievedResult
from app.settings import BraveSettings
from app.utils.httpx import httpx_get
from app.utils.logging import setup_logger

logger = setup_logger("BraveSearchQueryEngine")


class BraveSearchQueryEngine:
    """Utility class for interacting with the Brave Search API.

    Searches are performed in an async manner using httpx.
    Searches are cached using the `cached` decorator with `search_text` as the
    caching key.
    """

    def __init__(self, settings: BraveSettings):
        self.settings = settings
        self.default_timeout = 2.0

    async def search_request(self, search_text: str) -> httpx.Response | None:
        logger.info("brave_search: " + search_text)

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

        return await httpx_get(url=url, headers=headers, timeout=self.default_timeout)

    @fcached("agency.brave.search.{search_text}")
    async def cached_search(
        self,
        search_text: str,
    ) -> Any | None:
        # Cache result as bytes to play nicely with redis
        if response := await self.search_request(search_text):
            return response.json()
        return None

    async def brave_search(
        self,
        search_text: str,
    ) -> WebSearchApiResponse | None:
        # TODO: Merge with cached_search once cache supports pydantic models
        if response := await self.cached_search(search_text):
            try:
                return WebSearchApiResponse.model_validate(response)
            except ValidationError:
                pass
        return None

    async def call_brave_search_api(self, search_text: str) -> list[RetrievedResult]:
        results = []

        if response := await self.brave_search(search_text):
            logging.info(f"Brave search response: {response}")

            for result in response.web_results():
                text = result.description + "".join(result.get_extra_snippets())

                retrieved_result = RetrievedResult.model_validate(
                    {
                        "text": text,
                        "source": result.model_dump(),
                    }
                )

                results.append(retrieved_result)

        return results
