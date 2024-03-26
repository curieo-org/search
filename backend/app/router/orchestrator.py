import asyncio
import re

from llama_index.core.tools import ToolMetadata
from llama_index.core.selectors import LLMSingleSelector
from llama_index.llms.openai import OpenAI
from llama_index.core.schema import QueryBundle
from llama_index.llms.together import TogetherLLM
from llama_index.core.response_synthesizers import SimpleSummarize

from app.rag.retrieval.clinical_trials.clinical_trial_sql_query_engine import ClinicalTrialText2SQLEngine
from app.rag.retrieval.drug_chembl.drug_chembl_graph_query_engine import DrugChEMBLText2CypherEngine
from app.rag.retrieval.web.brave_search import BraveSearchQueryEngine
from app.rag.retrieval.pubmed.pubmedqueryengine import PubmedSearchQueryEngine
from app.rag.reranker.response_reranker import TextEmbeddingInferenceRerankEngine
from app.api.common.util import RouteCategory
from app.config import config, OPENAI_API_KEY, TOGETHER_KEY
from app.services.search_utility import setup_logger

logger = setup_logger("Orchestrator")
TAG_RE = re.compile(r'<[^>]+>')

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

        self.clinicalTrialSearch = ClinicalTrialText2SQLEngine(config)
        self.drugChemblSearch = DrugChEMBLText2CypherEngine(config)
        self.pubmedsearch = PubmedSearchQueryEngine(config)
        self.bravesearch = BraveSearchQueryEngine(config)

    async def query_and_get_answer(
            self,
            routecategory: RouteCategory = RouteCategory.PBW,
            search_text: str = "") -> str:
        # search router call
        logger.debug(
            f"Orchestrator.query_and_get_answer.router_id search_text: {search_text}"
        )

        #initialize router with bad value
        router_id = -1

        # user not specified
        if routecategory == RouteCategory.NS:
            selector_result = self.selector.select(self.choices, query=search_text)
            router_id = selector_result.selections[0].index
            logger.debug(
                f"Orchestrator.query_and_get_answer.router_id router_id: {router_id}"
            )
            breaks_sql = False

        #routing
        if router_id == 0 or routecategory == RouteCategory.CT:
            # clinical trial call
            logger.debug(
                "Orchestrator.query_and_get_answer.router_id clinical trial Entered."
            )
            try:
                sqlResponse = self.clinicalTrialSearch.call_text2sql(search_text=search_text)
                result = str(sqlResponse)
                sources = result

                logger.debug(f"Orchestrator.query_and_get_answer.sqlResponse sqlResponse: {result}")
            except Exception as e:
                breaks_sql = True
                logger.exception("Orchestrator.query_and_get_answer.sqlResponse Exception -", exc_info = e, stack_info=True)
                pass

        elif router_id == 1 or routecategory == RouteCategory.DRUG:
            # drug information call
            logger.debug(
                "Orchestrator.query_and_get_answer.router_id drug_information_choice Entered."
            )
            try:
                cypherResponse = self.drugChemblSearch.call_text2cypher(
                    search_text=search_text
                )
                result = str(cypherResponse)
                sources = result
                logger.debug(
                    f"Orchestrator.query_and_get_answer.cypherResponse cypherResponse: {result}"
                )
            except Exception as e:
                breaks_sql = True
                logger.exception(
                    "Orchestrator.query_and_get_answer.cypherResponse Exception -",
                    exc_info=e,
                    stack_info=True,
                )

        if router_id == 2 or routecategory == RouteCategory.PBW or routecategory == RouteCategory.NS or breaks_sql:
            logger.debug(
                "Orchestrator.query_and_get_answer.router_id Fallback Entered."
            )

            extracted_pubmed_results, extracted_web_results = await asyncio.gather(
                self.pubmedsearch.call_pubmed_vectors(search_text=search_text), self.bravesearch.call_brave_search_api(search_text=search_text)
            )
            extracted_results = extracted_pubmed_results + extracted_web_results
            logger.debug(
                f"Orchestrator.query_and_get_answer.extracted_results count: {len(extracted_pubmed_results), len(extracted_web_results)}"
            )

            # rerank call
            reranked_results = TextEmbeddingInferenceRerankEngine(top_n=2)._postprocess_nodes(
                nodes = extracted_results,
                query_bundle=QueryBundle(query_str=search_text))

            summarizer = SimpleSummarize(llm=TogetherLLM(model="mistralai/Mixtral-8x7B-Instruct-v0.1", api_key=str(TOGETHER_KEY)))
            result = summarizer.get_response(query_str=search_text, text_chunks=[TAG_RE.sub('', node.get_content()) for node in reranked_results])
            sources = [node.node.metadata for node in reranked_results ]

        return {
            "result" : result,
            "sources": sources
        }