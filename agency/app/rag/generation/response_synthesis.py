import collections
import json
from urllib.parse import urlparse

import pydantic
import requests

from app.services.search_utility import setup_logger
from app.settings import Settings

logger = setup_logger("ResponseSynthesisEngine")


class ResponseSynthesisRecord(pydantic.BaseModel):
    result: str
    source: list[str]


class ResponseSynthesisEngine:
    """Implements the logic to call the LLM in the last layer and returns the results.

    It uses the preprocessed service and prompt template. Returns the output as a list.
    """

    def __init__(self, settings: Settings):
        self.settings = settings
        self.language = settings.project.prompt_language
        self.together = settings.together
        self.prompt_config = self.together.prompt_config

    @staticmethod
    def clean_response_text(response_text: str) -> str:
        return response_text.replace("\n", "")

    def get_prompt_v3(
        self, search_text: str, reranked_results: collections.defaultdict[list],
    ) -> (str, list[str]):
        logger.info(
            f"LLMService.get_prompt_v3. search_text: {search_text}, reranked_results.len: {len(reranked_results)}",
        )

        context_str = ""
        urls = []
        for result in reranked_results:
            domain = urlparse(result["url"]).netloc.replace("www.", "")
            urls.append(result["url"])
            context_str += f"Source {domain}\n"

            context_str += f"{result['text']}\n"
            context_str += "\n\n"

        prompt_token_limit = self.prompt_config.prompt_token_limit
        context_str = context_str[:prompt_token_limit]
        prompt = f"""
        Web search result:
        {context_str}

        Instructions: Using the provided web search results, write a comprehensive
        reply to the given query. Make sure to cite results using [number] notation
        after the reference. If the provided search results refer to multiple
        subjects with the same name, write separate answers for each subject. Answer
        in language: {self.language} If the context is insufficient, reply "I cannot
        answer because my reference sources don't have related info" in language
        {self.language}.
        Query: {search_text}
        """

        return prompt, urls

    async def call_llm_service_api(
        self, search_text: str, reranked_results: collections.defaultdict[list],
    ) -> ResponseSynthesisRecord:
        logger.info("call_llm_service_api. search_text: " + search_text)
        logger.info(
            "call_llm_service_api. reranked_results length: "
            + str(len(reranked_results)),
        )

        try:
            headers = {
                "Authorization": "Bearer " + self.together.api_key.get_secret_value(),
                "accept": "application/json",
                "content-type": "application/json",
            }

            prompt, urls = self.get_prompt_v3(search_text, reranked_results)

            payload = json.dumps(
                {
                    "model": self.together.model,
                    "prompt": "<s>[INST] " + prompt + " [/INST]",
                    "max_tokens": 1024,
                    "stop": ["</s>", "[/INST]"],
                    "temperature": 0.1,
                    "top_p": 0.7,
                    "top_k": 50,
                    "repetition_penalty": 1,
                    "n": 1,
                },
            )

            response = requests.request(
                "POST", self.together.api_root, headers=headers, data=payload,
            )

        except Exception as ex:
            logger.exception(
                "call_llm_service_api Exception -", exc_info=ex, stack_info=True,
            )
            raise ex

        logger.info("call_llm_service_api. response: " + response.text)

        return ResponseSynthesisRecord(
            result=self.clean_response_text(response.json()["choices"][0]["text"]),
            source=list(urls),
        )
