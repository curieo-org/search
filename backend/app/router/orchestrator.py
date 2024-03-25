import dspy
from app.rag.retrieval.web.brave_search import BraveSearchQueryEngine
from app.rag.retrieval.clinical_trials.clinical_trial_sql_query_engine import ClinicalTrialText2SQLEngine
from app.rag.retrieval.drug_chembl.drug_chembl_graph_query_engine import DrugChEMBLText2CypherEngine
from app.rag.reranker.response_reranker import ReRankEngine
from app.rag.generation.response_synthesis import ResponseSynthesisEngine
from app.config import config, OPENAI_API_KEY, RERANK_TOP_COUNT, ORCHESRATOR_ROUTER_PROMPT_PROGRAM

from app.services.search_utility import setup_logger

from app.dspy_integration.router_prompt import Router_module


logger = setup_logger('Orchestrator')


class Orchestrator:
    """
    Orchestrator is responsible for routing the search engine query.
    It routes the query into three routes now.The first one is clinical trails, second one is drug related information,
    and third one is pubmed brave.
    """

    def __init__(self, config):
        self.config = config

        self.llm = dspy.OpenAI(model="gpt-3.5-turbo", api_key=str(OPENAI_API_KEY))
        dspy.settings.configure(lm = self.llm)
        self.router = Router_module()
        self.router.load(ORCHESRATOR_ROUTER_PROMPT_PROGRAM)

    async def query_and_get_answer(
        self,
        search_text: str
    ) -> str:
        logger.info(f"query_and_get_answer.router_id search_text: {search_text}")
        try : 
            router_id = int(self.router(search_text).answer)
        except Exception as e:
            logger.exception("query_and_get_answer.router_id Exception -", exc_info = e, stack_info=True)
        logger.info(f"query_and_get_answer.router_id router_id: {router_id}")

        breaks_sql = False

        if router_id == 0:
            clinicalTrialSearch = ClinicalTrialText2SQLEngine(config)
            try:
                sqlResponse = await clinicalTrialSearch.call_text2sql(search_text=search_text)
                result = sqlResponse.get('result', '')
                logger.info(f"query_and_get_answer.sqlResponse sqlResponse: {result}")
            except Exception as e:
                breaks_sql = True
                logger.exception("query_and_get_answer.sqlResponse Exception -", exc_info = e, stack_info=True)

        elif router_id == 1:
            # drug information call
            logger.info("query_and_get_answer.router_id drug_information_choice Entered.")

            drugChemblSearch = DrugChEMBLText2CypherEngine(config)
            result = []

            try:
                cypherResponse = await drugChemblSearch.call_text2cypher(search_text=search_text)
                result = str(cypherResponse)

                logger.info(f"query_and_get_answer.cypherResponse cypherResponse: {result}")
            except Exception as e:
                breaks_sql = True
                logger.exception("query_and_get_answer.cypherResponse Exception -", exc_info = e, stack_info=True)

            print()
    
        if router_id == 2 or breaks_sql:
            logger.info("query_and_get_answer.router_id Fallback Entered.")
            
            bravesearch = BraveSearchQueryEngine(config)
            extracted_retrieved_results = await bravesearch.call_brave_search_api(search_text=search_text)
            
            logger.info(f"query_and_get_answer.extracted_retrieved_results: {extracted_retrieved_results}")

            
            #rerank call
            rerank = ReRankEngine(config)
            rerankResponse = await rerank.call_embedding_api(
                search_text=search_text,
                retrieval_results=extracted_retrieved_results
            )
            rerankResponse_sliced = rerankResponse[:RERANK_TOP_COUNT]
            logger.info(f"query_and_get_answer.rerankResponse_sliced: {rerankResponse_sliced}")

            #generation call
            response_synthesis = ResponseSynthesisEngine(config)
            result = await response_synthesis.call_llm_service_api(
                search_text=search_text,
                reranked_results=rerankResponse_sliced
            )
            result = result.get('result', '') + "\n\n" + "Source: " + ', '.join(result.get('source', []))
            logger.info(f"query_and_get_answer.response_synthesis: {result}")
            
        logger.info(f"query_and_get_answer. result: {result}")

        return result
