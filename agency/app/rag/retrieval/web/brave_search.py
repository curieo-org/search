from typing import List

import requests
from llama_index.core.schema import TextNode

from app.services.search_utility import setup_logger
from app.settings import BraveSettings

logger = setup_logger("BraveSearchQueryEngine")


class BraveSearchQueryEngine:
    """
    The BraveSearchQueryEngine class is a utility for interacting with the Brave
    search API within a larger application framework, likely aimed at providing
    search capabilities or integrating search results into an application's
    functionality. It abstracts the details of making API requests, handling
    responses, and error logging, providing a simple interface (call_brave_search_api)
    for obtaining processed search results in an asynchronous manner.
    This class leverages a configuration object for flexibility, allowing it
    to adapt to different settings or requirements without changing the core
    implementation.
    """

    def __init__(self, settings: BraveSettings):
        self.settings = settings

    async def call_brave_search_api(self, search_text: str) -> List[TextNode]:
        logger.info("call_brave_search_api. query: " + search_text)

        endpoint = (
            "{url_address}?count={count}&q={"
            "search_text}&search_lang=en&extra_snippets=True"
        ).format(
            url_address=self.settings.api_root,
            count=self.settings.result_count,
            search_text=search_text,
        )

        headers = {
            "Accept": "application/json",
            "Accept-Encoding": "gzip",
            "X-Subscription-Token": str(
                self.settings.subscription_key.get_secret_value()
            ),
        }

        try:
            logger.info("call_brave_search_api. endpoint: " + endpoint)

            response = requests.get(endpoint, headers=headers)
            response.raise_for_status()
            web_response = response.json().get("web").get("results")

            if web_response:
                return [
                    TextNode(
                        text=resp.get("description")
                        + "".join(resp.get("extra_snippets", [])),
                        metadata={"url": resp["url"], "page_age": resp.get("page_age")},
                    )
                    for resp in web_response
                ]

        except Exception as e:
            logger.exception("Brave search failed", exc_info=e, stack_info=True)
            return []
