import collections
import requests
import re

from app.services.search_utility import setup_logger
from app.config import  EMBEDDING_RERANK_API, EMBEDDING_CHUNK_SIZE

logger = setup_logger('Reranking')
TAG_RE = re.compile(r'<[^>]+>')

class ReRankEngine:
    """
    This class implements the logic re-ranking response and returns the results.
    It uses the embedding api that process the query and responses from the retrieval layer.
    It returns the output in list format.
    """
    def __init__(self, config):
        self.config = config

    def call_embedding_api(self, search_text: str, retrieval_results: collections.defaultdict[list]) -> collections.defaultdict[list]:
        
        logger.info("ReRankEngine.call_embedding_api. search_text: " + search_text)
        logger.info("ReRankEngine.call_embedding_api. retrieval_results length: " + str(len(retrieval_results)))

        endpoint = "{url_address}".format(
            url_address=EMBEDDING_RERANK_API
        )

        headers = { 
            'Accept': 'application/json'
        }

        results = collections.defaultdict(list)

        #clean the data
        retrieval_results_text_data = [result.get('text') for result in retrieval_results.values()]
        retrieval_results_clean_text_data = [payload.replace("\n", " ").replace("\"","") for payload in retrieval_results_text_data]
        retrieval_results_clean_html_data = [TAG_RE.sub('', payload) for payload in retrieval_results_clean_text_data]

        #chunking the data
        payload = [payload[:EMBEDDING_CHUNK_SIZE] for payload in retrieval_results_clean_html_data]
        
        request_data = {
            "query": search_text,
            "texts": payload
        }

        try:
            response = requests.request("POST", endpoint, headers=headers, json=request_data)
            response.raise_for_status()

            rerank_orders = response.json()
            results = [retrieval_results[order.get('index')] for order in rerank_orders]
        except Exception as ex:
            logger.exception("ReRankEngine.call_embedding_api Exception -", exc_info = ex, stack_info=True)
            raise ex
        return results
