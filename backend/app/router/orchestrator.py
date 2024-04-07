import asyncio
import re

import together
from llama_index.core.schema import QueryBundle
from llama_index.llms.together import TogetherLLM
from llama_index.core.response_synthesizers import SimpleSummarize

from app.rag.retrieval.clinical_trials.clinical_trial_sql_query_engine import \
    ClinicalTrialText2SQLEngine
from app.rag.retrieval.drug_chembl.drug_chembl_graph_query_engine import \
    DrugChEMBLText2CypherEngine
from app.rag.retrieval.web.brave_search import BraveSearchQueryEngine
from app.rag.retrieval.pubmed.pubmedqueryengine import PubmedSearchQueryEngine
from app.rag.reranker.response_reranker import TextEmbeddingInferenceRerankEngine
from app.api.common.util import RouteCategory
from app.services.search_utility import setup_logger

import dspy
from app.dspy_integration.router_prompt import RouterModule

from app.settings import Settings

logger = setup_logger("Orchestrator")
TAG_RE = re.compile(r'<[^>]+>')


class Orchestrator:
    """
    Orchestrator is responsible for routing the search engine query.
    It routes the query into three routes now.The first one is clinical trails, second one is drug related information,
    and third one is pubmed brave.
    """

    def __init__(self, settings: Settings):
        self.settings = settings
        self.llm = dspy.OpenAI(model=str(settings.ai_models.router),
                               api_key=str(settings.openai.api_key))
        dspy.settings.configure(lm=self.llm)
        self.router = RouterModule()
        self.router.load(settings.dspy.orchestrator_router_prompt_program)

        self.clinical_trial_search = ClinicalTrialText2SQLEngine(settings)
        self.drug_chembl_search = DrugChEMBLText2CypherEngine(settings)
        self.pubmed_search = PubmedSearchQueryEngine(settings)
        self.brave_search = BraveSearchQueryEngine(settings.brave)

    async def query_and_get_answer(
        self,
        search_text: str,
        routecategory: RouteCategory = RouteCategory.PBW
    ) -> dict[str, str]:
        # search router call
        logger.info(
            f"Orchestrator.query_and_get_answer.router_id search_text: {search_text}"
        )

        # initialize router with bad value
        router_id = -1

        # user not specified
        if routecategory == RouteCategory.NS:
            logger.info(f"query_and_get_answer.router_id search_text: {search_text}")
            try:
                router_id = int(self.router(search_text).answer)
            except Exception as e:
                logger.exception("query_and_get_answer.router_id Exception -",
                                 exc_info=e, stack_info=True)
            logger.info(f"query_and_get_answer.router_id router_id: {router_id}")

        # routing
        if router_id == 0 or routecategory == RouteCategory.CT:
            # clinical trial call
            logger.info(
                "Orchestrator.query_and_get_answer.router_id clinical trial Entered."
            )
            try:
                sql_response = await self.clinical_trial_search.call_text2sql(
                    search_text=search_text
                )
                result = str(sql_response)
                sources = result  # TODO

                logger.info(f"sql_response: {result} and {sources}")

                return {
                    "result": result,
                    "sources": sources
                }
            except Exception as e:
                logger.exception(
                    "Orchestrator.query_and_get_answer.sqlResponse Exception -",
                    exc_info=e, stack_info=True)
                pass

        elif router_id == 1 or routecategory == RouteCategory.DRUG:
            # drug information call
            logger.info(
                "Orchestrator.query_and_get_answer.router_id drug_information_choice "
                "Entered."
            )
            try:
                cypherResponse = await self.drug_chembl_search.call_text2cypher(
                    search_text=search_text
                )
                result = str(cypherResponse)
                sources = result
                logger.info(
                    f"Orchestrator.query_and_get_answer.cypherResponse cypherResponse: {result}"
                )

                return {
                    "result": result,
                    "sources": sources
                }
            except Exception as e:
                logger.exception(
                    "Orchestrator.query_and_get_answer.cypherResponse Exception -",
                    exc_info=e,
                    stack_info=True,
                )

        # if routing fails, sql and cypher calls fail, routing to pubmed or brave
        logger.info(
            "Orchestrator.query_and_get_answer.router_id Fallback Entered."
        )

        extracted_pubmed_results, extracted_web_results = await asyncio.gather(
            self.pubmed_search.call_pubmed_vectors(search_text=search_text),
            self.brave_search.call_brave_search_api(search_text=search_text)
        )
        extracted_results = extracted_pubmed_results + extracted_web_results
        logger.info(
            f"Orchestrator.query_and_get_answer.extracted_results count: {len(extracted_pubmed_results), len(extracted_web_results)}"
        )

        # rerank call
        rerank_engine = TextEmbeddingInferenceRerankEngine(
            settings=self.settings.reranking, top_n=2
        )

        reranked_results = rerank_engine.postprocess_nodes(
            nodes=extracted_results,
            query_bundle=QueryBundle(query_str=search_text)
        )

        summarizer = SimpleSummarize(
            llm=TogetherLLM(model="mistralai/Mixtral-8x7B-Instruct-v0.1",
                            api_key=str(together.api_key))
        )
        result = summarizer.get_response(query_str=search_text,
                                         text_chunks=[TAG_RE.sub('', node.get_content())
                                                      for node in reranked_results])
        
        sources = [node.node.metadata for node in reranked_results]

        return {
            "result": result,
            "sources": sources
        }
