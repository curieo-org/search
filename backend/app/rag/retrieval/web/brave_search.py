import collections
import requests
import opentelemetry

from app.services.search_utility import setup_logger
from app.services.tracing import SentryTracer
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
        search_text: str,
        parent_trace_span: opentelemetry.trace.Span
    ) -> collections.defaultdict[list]:
        trace_span = await SentryTracer().create_child_span(parent_trace_span, 'call_brave_search_api')

        with trace_span:
            trace_span.set_attribute('description', 'Call Brave Search API')
            logger.info("BraveSearchQueryEngine.call_brave_search_api. query: " + search_text)

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
                trace_span.set_attribute('brave_endpoint', endpoint)
                trace_span.set_attribute('brave_headers', str(headers))

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
                raise ex
            
            trace_span.set_attribute('result', str(results))
            logger.info("BraveSearchQueryEngine.call_brave_search_api. result: " + str(results))
        return results
