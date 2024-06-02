import re

from llama_index.core.schema import QueryBundle

from app.rag.utils.models import PromptCompressorResult, RetrievedResult
from app.settings import PostProcessingSettings
from app.utils.httpx import call_internal_api
from app.utils.logging import setup_logger

TAG_RE = re.compile(r"<[^>]+>")
logger = setup_logger("PromptCompressor")


class PromptCompressorEngine:
    """post-processor LLM Lingua Call."""

    def __init__(self, settings: PostProcessingSettings):
        self.settings = settings

    async def compress_nodes(
        self,
        query_bundle: QueryBundle,
        nodes: list[RetrievedResult],
    ) -> PromptCompressorResult | None:
        try:
            text_list = [
                node.text[: self.settings.node_max_tokens_hard_limit] for node in nodes
            ]

            api_response = await call_internal_api(
                url=self.settings.api,
                data={
                    "query": query_bundle.query_str,
                    "target_token": self.settings.compressed_target_token,
                    "context_texts_list": text_list,
                },
            )

            result = api_response.get("response", {})
            if result.get("compressed_tokens"):
                compressed_result = PromptCompressorResult(
                    prompt_list=result.get("compressed_prompt_list", ""),
                    sources=[
                        nodes[source_index].source for source_index in result["sources"]
                    ],
                )

                compressed_result.sources = compressed_result.sources[
                    : self.settings.top_n_sources
                ]
                return compressed_result
            return None

        except Exception as e:
            logger.error(f"Error in postprocess_nodes: {e}")
            return None
