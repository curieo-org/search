# ruff: noqa: ERA001, ARG002, D205
import asyncio
import re
import time

from llama_index.core.schema import QueryBundle
from llama_index.llms.together import TogetherLLM
from llama_index.core.response_synthesizers import SimpleSummarize

from app.rag.post_process.prompt_compressor import PromptCompressorEngine
from app.rag.retrieval.pubmed.pubmedqueryengine import PubmedSearchQueryEngine
from app.rag.retrieval.web.brave_engine import BraveSearchQueryEngine
from app.rag.utils.models import RetrievedResult, SearchResultRecord
from app.rag.generation.response_synthesis import ResponseSynthesisEngine
from app.settings import Settings
from app.utils.logging import setup_logger

logger = setup_logger("Orchestrator")
TAG_RE = re.compile(r"<[^>]+>")


class Orchestrator:
    def __init__(self, settings: Settings):
        self.settings = settings

        self.pubmed_search = PubmedSearchQueryEngine(settings)
        self.brave_search = BraveSearchQueryEngine(settings.brave)

        self.compress_engine = PromptCompressorEngine(settings=settings.post_process)
        # self.response_synthesis = ResponseSynthesisEngine(
        #     settings=settings.biollm
        # )
        self.summarizer = SimpleSummarize(
            llm=TogetherLLM(
                model="meta-llama/Llama-3-70b-chat-hf",
                api_key=self.settings.together.api_key.get_secret_value(),
            ),
        )

    async def handle_pubmed_web_search(
        self, search_text: str
    ) -> SearchResultRecord | None:
        logger.info(f"handle_pubmed_web_search. search_text: {search_text}")
        extracted_results = list[RetrievedResult]
        try:
            (
                extracted_pubmed_results,
                #extracted_pubmed_cluster_results,
                #extracted_web_results,
            ) = await asyncio.gather(
                self.pubmed_search.call_pubmed_parent_vectors(search_text=search_text),
                #self.pubmed_search.call_pubmed_cluster_vectors(search_text=search_text),
                #self.brave_search.call_brave_search_api(search_text=search_text),
            )
            extracted_results = (
                extracted_pubmed_results
                #+ extracted_pubmed_cluster_results
                #+ extracted_web_results
            )

            # post process call
            if len(extracted_results) == 0:
                return None
            
            compressed_prompt = await self.compress_engine.compress_nodes(
                query_bundle=QueryBundle(query_str=search_text),
                nodes=extracted_results
            )

            # summarizer model
            if compressed_prompt is None:
                return None
            
            start_time = time.time()
            result = self.summarizer.get_response(
                query_str=search_text,
                text_chunks=compressed_prompt.prompt_list
            )
            print(time.time() - start_time)
            
            # result = await self.response_synthesis.call_llm_service(
            #     search_text=search_text,
            #     context_str=compressed_prompt.prompt
            # )

            return SearchResultRecord.model_validate(
                {
                    "result": result,
                    "sources": compressed_prompt.sources,
                },
            )

        except Exception as e:
            logger.exception(
                "Orchestrator.handle_pubmed_bioxriv_web_search failed -",
                exc_info=e,
                stack_info=True,
            )
            return None
