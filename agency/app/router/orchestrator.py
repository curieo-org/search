# ruff: noqa: ERA001, ARG002, D205
import asyncio
from concurrent.futures import ProcessPoolExecutor, as_completed
import re

from llama_index.core.response_synthesizers import SimpleSummarize
from llama_index.core.schema import QueryBundle
from llama_index.llms.together import TogetherLLM

from app.rag.reranker.response_reranker import TextEmbeddingRerankPostprocessor
from app.rag.retrieval.pubmed.pubmedqueryengine import PubmedSearchQueryEngine
from app.rag.retrieval.web.brave_engine import BraveSearchQueryEngine
from app.rag.utils.models import SearchResultRecord
from app.settings import Settings
from app.utils.logging import setup_logger

logger = setup_logger("Orchestrator")
TAG_RE = re.compile(r"<[^>]+>")


class Orchestrator:
    """Orchestrator is responsible for routing the search engine query.

    It currently supports 2 routes:
    1. Pubmed
        1.1 Pubmed Parents
        1.2 Pubmed Children
    2. Brave API search

    TODO: enable support for
    3. Clinical trials
    4. Drug chembl
    """

    def __init__(self, settings: Settings):
        self.settings = settings
        # self.router = RouterModule()
        # self.router.load(settings.dspy.orchestrator_router_prompt_program)

        # self.clinical_trial_search = ClinicalTrialText2SQLEngine(settings)
        # self.drug_chembl_search = DrugChEMBLText2CypherEngine(settings)
        self.pubmed_search = PubmedSearchQueryEngine(settings)
        self.brave_search = BraveSearchQueryEngine(settings.brave)

        self.reranker = TextEmbeddingRerankPostprocessor.from_settings(
            settings=self.settings.reranking,
        )
        self.summarizer = SimpleSummarize(
            llm=TogetherLLM(
                model="mistralai/Mixtral-8x7B-Instruct-v0.1",
                api_key=self.settings.together.api_key.get_secret_value(),
            ),
        )

    async def handle_pubmed_bioxriv_web_search(
        self,
        search_text: str,
        rerank_llm_lingua_call: bool = False
    ) -> SearchResultRecord | None:
        logger.info(f"handle_pubmed_bioxriv_web_search. search_text: {search_text}")
        extracted_results = []
        reranked_results = []
        try:
            extracted_pubmed_results, extracted_pubmed_cluster_results, extracted_web_results = await asyncio.gather(
                self.pubmed_search.call_pubmed_parent_vectors(search_text=search_text),
                self.pubmed_search.call_pubmed_cluster_vectors(search_text=search_text),
                self.brave_search.call_brave_search_api(search_text=search_text)
            )
            extracted_results = extracted_pubmed_results + extracted_pubmed_cluster_results + extracted_web_results


            # rerank call
            if rerank_llm_lingua_call:
                pass #TODO
            else:
                reranked_results = self.reranker.postprocess_nodes(
                    nodes=extracted_results,
                    query_bundle=QueryBundle(query_str=search_text),
                )

            #summarizer model
            result = self.summarizer.get_response(
                query_str=search_text,
                text_chunks=[
                    TAG_RE.sub("", node.get_content()) for node in reranked_results
                ],
            )

            # Metadata should be valid SourceRecords
            sources = [node.metadata for node in reranked_results]

            return SearchResultRecord.model_validate(
                {
                    "result": result,
                    "sources": sources,
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
