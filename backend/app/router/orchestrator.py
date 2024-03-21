from llama_index.core.tools import ToolMetadata
from llama_index.core.selectors import LLMSingleSelector
from llama_index.llms.openai import OpenAI

from app.rag.retrieval.web.brave_search import BraveSearchQueryEngine
from app.rag.retrieval.drug_chembl.drug_chembl_graph_query_engine import (
    DrugChEMBLText2CypherEngine,
)
from app.rag.reranker.response_reranker import ReRankEngine
from app.rag.generation.response_synthesis import ResponseSynthesisEngine
from app.config import config, OPENAI_API_KEY, RERANK_TOP_COUNT

from app.services.search_utility import setup_logger
from app.services.tracing import SentryTracer
import opentelemetry

logger = setup_logger("Orchestrator")


class Orchestrator:
    """
    Orchestrator is responsible for routing the search engine query.
    It routes the query into three routes now.The first one is clinical trails, second one is drug related information,
    and third one is pubmed brave.
    """

    def __init__(self, config):
        self.config = config
        self.choices = [
            ToolMetadata(
                description="""useful for retrieving only the clinical trials information like adverse effects,eligibility details
                         of clinical trials perticipents, sponsor details, death count, condition  of many healthcare problems""",
                name="clinical_trial_choice",
            ),
            ToolMetadata(
                description="""useful only for retrieving the drug related information like molecular weights,
                        similarities,smile codes, target medicines, effects on other medicine""",
                name="drug_information_choice",
            ),
            ToolMetadata(
                description="""useful for retrieving general information about healthcare data.""",
                name="pubmed_brave_choice",
            ),
        ]

        self.ROUTER_PROMPT = "You are working as router of a healthcare search engine.Some choices are given below. It is provided in a numbered list (1 to {num_choices}) where each item in the list corresponds to a summary.\n---------------------\n{context_list}\n---------------------\nIf you are not super confident then please use choice 3 as default choice.Using only the choices above and not prior knowledge, return the choice that is most relevant to the question: '{query_str}'\n"

        self.selector = LLMSingleSelector.from_defaults(
            llm=OpenAI(model="gpt-3.5-turbo", api_key=str(OPENAI_API_KEY)),
            prompt_template_str=self.ROUTER_PROMPT,
        )

    async def query_and_get_answer(
        self,
        search_text: str,
        parent_trace_span: opentelemetry.trace.Span
    ) -> str:
        trace_span = await SentryTracer().create_child_span(parent_trace_span, 'query_and_get_answer')

        with trace_span:
            # search router call
            logger.debug(f"Orchestrator.query_and_get_answer.router_id search_text: {search_text}")
            selector_result = self.selector.select(self.choices, query=search_text)
            router_id = selector_result.selections[0].index
            logger.debug(f"Orchestrator.query_and_get_answer.router_id router_id: {router_id}")

            trace_span.set_attribute('router_id', router_id)

            breaks_sql = False

            if router_id == 0:
                # retriever call
                clinicalTrialSearch = ClinicalTrialText2SQLEngine(config)
                try:
                    sqlResponse = await clinicalTrialSearch.call_text2sql(search_text=search_text, parent_trace_span=trace_span)
                    result = sqlResponse.get('result', '')
                    logger.debug(f"Orchestrator.query_and_get_answer.sqlResponse sqlResponse: {result}")
                except Exception as e:
                    breaks_sql = True
                    logger.exception("Orchestrator.query_and_get_answer.sqlResponse Exception -", exc_info = e, stack_info=True)

                print()

            elif router_id == 1:
                # drug information call
                logger.debug(f"Orchestrator.query_and_get_answer.router_id drug_information_choice Entered.")

                drugChemblSearch = DrugChEMBLText2CypherEngine(config)
                result = []

                try:
                    cypherResponse = await drugChemblSearch.call_text2cypher(search_text=search_text, parent_trace_span=trace_span)
                    result = str(cypherResponse)

                    logger.debug(f"Orchestrator.query_and_get_answer.cypherResponse cypherResponse: {result}")
                except Exception as e:
                    breaks_sql = True
                    logger.exception("Orchestrator.query_and_get_answer.cypherResponse Exception -", exc_info = e, stack_info=True)

                print()
        
            if router_id == 2 or breaks_sql == True:
                logger.debug(f"Orchestrator.query_and_get_answer.router_id Fallback Entered.")
                bravesearch = BraveSearchQueryEngine(config)
                extracted_retrieved_results = await bravesearch.call_brave_search_api(search_text=search_text, parent_trace_span=trace_span)
                logger.debug(f"Orchestrator.query_and_get_answer.extracted_retrieved_results: {extracted_retrieved_results}")

                #rerank call
                rerank = ReRankEngine(config)
                rerankResponse = await rerank.call_embedding_api(
                    search_text=search_text,
                    retrieval_results=extracted_retrieved_results,
                    parent_trace_span=trace_span
                )
                rerankResponse_sliced = rerankResponse[:RERANK_TOP_COUNT]
                logger.debug(f"Orchestrator.query_and_get_answer.rerankResponse_sliced: {rerankResponse_sliced}")

                #generation call
                response_synthesis = ResponseSynthesisEngine(config)
                result = await response_synthesis.call_llm_service_api(
                    search_text=search_text,
                    reranked_results=rerankResponse_sliced,
                    parent_trace_span=trace_span
                )
                result = result.get('result', '') + "\n\n" + "Source: " + ', '.join(result.get('source', []))
                logger.debug(f"Orchestrator.query_and_get_answer.response_synthesis: {result}")

            trace_span.set_attribute('result', result)
            logger.debug(f"Orchestrator.query_and_get_answer. result: {result}")

        return result
