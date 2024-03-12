import collections
import json
import requests
from urllib.parse import urlparse

from app.services.search_utility import setup_logger
from app.config import TOGETHER_API, TOGETHER_KEY, TOGETHER_MODEL, TOGETHER_PROMPT_CONFIG, PROMPT_LANGUAGE

logger = setup_logger('ResponseSynthesisEngine')
    

class ResponseSynthesisEngine:
    """
    This class implements the logic to call the llm service in the last layer and returns the results.
    It uses the preprocessed service and prompt template.
    It returns the output in list format.
    """
    def __init__(self, config):
        self.config = config


    def clean_response_text(self, response_text: str):
        return response_text.replace("\n", "")
    
    def get_prompt_v3(self, search_text: str, reranked_results: collections.defaultdict[list]):
        
        language = PROMPT_LANGUAGE
        logger.info(f"LLMService.get_prompt_v3. search_text: {search_text}, reranked_results.len: {len(reranked_results)}")
        
        context_str = ""
        urls = []
        for result in reranked_results:
            domain = urlparse(result['url']).netloc.replace('www.', '')
            urls.append(result['url'])
            context_str += f"Source {domain}\n"

            context_str += f"{result['text']}\n"
            context_str += "\n\n"
        
        prompt_length_limit = TOGETHER_PROMPT_CONFIG.get('prompt').get('prompt_token_limit')
        context_str = context_str[:prompt_length_limit]
        prompt = \
            f"""
        Web search result:
        {context_str}

        Instructions: Using the provided web search results, write a comprehensive reply to the given query. 
        Make sure to cite results using [number] notation after the reference.
        If the provided search results refer to multiple subjects with the same name, write separate answers for each subject.
        Answer in language: {language}
        If the context is insufficient, reply "I cannot answer because my reference sources don't have related info" in language {language}.
        Query: {search_text}
        """

        return prompt, urls

    def call_llm_service_api(self, search_text: str, reranked_results: collections.defaultdict[list]) -> collections.defaultdict[list]:
        
        logger.info("ResponseSynthesisEngine.call_llm_service_api. search_text: " + search_text)
        logger.info("ResponseSynthesisEngine.call_llm_service_api. reranked_results length: " + str(len(reranked_results)))
        
        try:
            headers = {
                'Authorization': str(TOGETHER_KEY),
                'accept': 'application/json',
                'content-type': 'application/json'
                } 

            prompt, urls = self.get_prompt_v3(search_text, reranked_results)          
            
            payload = json.dumps({
                "model": TOGETHER_MODEL,
                "prompt": "<s>[INST] " + prompt +" [/INST]",
                "max_tokens": 1024,
                "stop": [
                    "</s>",
                    "[/INST]"
                ],
                "temperature": 0.1,
                "top_p": 0.7,
                "top_k": 50,
                "repetition_penalty": 1,
                "n": 1
                })
            
            response = requests.request("POST", TOGETHER_API, headers=headers, data=payload)
        
        except Exception as ex:
            logger.exception("ResponseSynthesisEngine.call_llm_service_api Exception -", exc_info = ex, stack_info=True)
            raise ex
        return {
            "result" : self.clean_response_text(response.json()['choices'][0]['text']),
            "source" : list(urls)
        }
                
