import collections
import requests

from app.services.search_utility import setup_logger
from app.config import BRAVE_RESULT_COUNT, BRAVE_SEARCH_API, BRAVE_SUBSCRIPTION_KEY

logger = setup_logger("BraveSearchQueryEngine")


class BraveSearchQueryEngine:
    """
    This class implements the logic brave search api and returns the results.
    It calls the brave api and processes the data and returns the result.
    """

    def __init__(self, config):
        self.config = config

    #@storage_cached('brave_search_website', 'search_text')
    async def call_brave_search_api(
        self,
        search_text: str
    ) -> collections.defaultdict[list]:
        logger.info("call_brave_search_api. query: " + search_text)

        endpoint = "{url_address}?count={count}&q={search_text}&search_lang=en&extra_snippets=True".format(
            url_address=BRAVE_SEARCH_API,
            count=BRAVE_RESULT_COUNT,
            search_text=search_text
        )

        headers = {
            'Accept': 'application/json',
            'Accept-Encoding': 'gzip',
            'X-Subscription-Token': str(BRAVE_SUBSCRIPTION_KEY)
        }
        results = collections.defaultdict(list)

        try:
            logger.info("call_brave_search_api. endpoint: " + endpoint)
            logger.info("call_brave_search_api. headers: " + str(headers))

            response = requests.get(endpoint, headers=headers)
            response.raise_for_status()
            web_response = response.json().get('web').get('results')
            i = 0
            if web_response:
                for resp in web_response:
                    detailed_text = resp.get('description') + ''.join(resp.get('extra_snippets') if resp.get('extra_snippets') else '')
                    results[i] = {
                        "text": detailed_text,
                        "url": resp['url'],
                        "page_age": resp.get('page_age')
                    }
                    i = i + 1

        except Exception as ex:
            logger.exception("call_brave_search_api Exception -", exc_info=ex, stack_info=True)
            raise ex
        
        logger.info("call_brave_search_api. result: " + str(results))
        return results
