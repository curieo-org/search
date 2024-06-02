# ruff: noqa: ERA001, ARG002, D205
import asyncio
import re

from llama_index.core.response_synthesizers import SimpleSummarize
from llama_index.core.schema import QueryBundle
from llama_index.llms.together import TogetherLLM

from app.rag.post_process.prompt_compressor import PromptCompressorEngine
from app.rag.retrieval.pubmed.pubmedqueryengine import PubmedSearchQueryEngine
from app.rag.retrieval.web.brave_engine import BraveSearchQueryEngine
from app.rag.utils.models import RetrievedResult, SearchResultRecord
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
        self.summarizer = SimpleSummarize(
            llm=TogetherLLM(
                model="mistralai/Mixtral-8x7B-Instruct-v0.1",
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
            result = self.summarizer.get_response(
                query_str=search_text,
                text_chunks=[reranked_results.compressed_prompt],
            )

            return SearchResultRecord.model_validate(
                {
                    "result": result,
                    "sources": reranked_results.reranked_sources,
                },
            )

        except Exception as e:
            logger.exception(
                "Orchestrator.handle_pubmed_bioxriv_web_search failed -",
                exc_info=e,
                stack_info=True,
            )
            return None

    async def handle_clinical_trial_search(
        self,
        search_text: str,
    ) -> SearchResultRecord | None:
        # TODO: Enable once stable and infallible
        """# clinical trial call
        logger.info(
            "handle_clinical_trial_search.router_id clinical trial Entered."
        )
        try:
            sql_response = await self.clinical_trial_search.call_text2sql(
                search_text=search_text
            )
            result = str(sql_response)
            sources = [result]  # TODO: clinical trial sql sources impl.

            logger.info(f"sql_response: {result}")

            return SearchResultRecord(result=result, sources=sources)
        except Exception as e:
            logger.exception(
                "Orchestrator.handle_clinical_trial_search.sqlResponse Exception -",
                exc_info=e,
                stack_info=True,
            )
        """
        return None

    async def handle_drug_search(self, search_text: str) -> SearchResultRecord | None:
        # TODO: Enable once stable and infallible
        """# drug information call
        logger.info(
            "Orchestrator.handle_drug_search drug_information_choice "
            "Entered."
        )
        try:
            cypher_response = await self.drug_chembl_search.call_text2cypher(
                search_text=search_text
            )
            result = str(cypher_response)
            sources = [result]  # TODO: chembl cypher sources impl
            logger.info(
                f"Orchestrator.handle_drug_search.cypher_response "
                f"cypher_response: {result}"
            ).

            return SearchResultRecord(result=result, sources=sources)
        except Exception as e:
            logger.exception(
                "Orchestrator.handle_drug_search.cypher_response Exception -",
                exc_info=e,
                stack_info=True,
            )
        """
        return None

    # TODO: Enable once stable and infallible
    """
    # initialize router with bad value
    router_id = -1

    # user not specified
    if route_category == RouteCategory.NOT_SELECTED:
        logger.info(f"query_and_get_answer.router_id search_text: {search_text}")
        try:
            router_id = int(self.router(search_text).answer)
        except Exception as e:
            logger.exception(
                "query_and_get_answer.router_id Exception -",
                exc_info=e,
                stack_info=True,
            )
        logger.info(f"query_and_get_answer.router_id router_id: {router_id}")

    # if routing fails, sql and cypher calls fail, routing to pubmed or brave
    """
