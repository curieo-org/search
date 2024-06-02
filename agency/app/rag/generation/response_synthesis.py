from llama_index.core.prompts.default_prompt_selectors import DEFAULT_TEXT_QA_PROMPT
from llama_index.llms.huggingface import TextGenerationInference

from app.settings import BioLLMSettings
from app.utils.logging import setup_logger

logger = setup_logger("ResponseSynthesisEngine")


class ResponseSynthesisEngine:
    def __init__(self, settings: BioLLMSettings):
        self.settings = settings
        self.summarizer_llm = TextGenerationInference(
            model_name=self.settings.model_name,
            model_url=self.settings.api_url,
            temperature=self.settings.temperature,
            max_tokens=self.settings.max_tokens,
        )
        self.language = "en-US"

    @staticmethod
    def clean_response_text(response_text: str) -> str:
        return response_text.replace("\n", "")

    def get_prompt_v3(
        self,
        search_text: str,
        context_str: str,
    ) -> str:
        context_str = context_str[: self.settings.prompt_token_limit]
        return DEFAULT_TEXT_QA_PROMPT.format(
            context_str=context_str, query_str=search_text
        )

    async def call_llm_service(
        self,
        search_text: str,
        context_str: str,
    ) -> str:
        logger.info("call_llm_service_api. search_text: " + search_text)

        try:
            prompt = self.get_prompt_v3(search_text, context_str)
            response = self.summarizer_llm.complete(prompt)
            return self.clean_response_text(response)

        except Exception as ex:
            logger.exception(
                "call_llm_service_api Exception -",
                exc_info=ex,
                stack_info=True,
            )
            return None
