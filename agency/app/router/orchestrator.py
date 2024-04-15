import abc
import asyncio
import re

import dspy
import pydantic
from llama_index.core.response_synthesizers import SimpleSummarize
from llama_index.core.schema import QueryBundle
from llama_index.llms.together import TogetherLLM

from app.grpc_types.agency_pb2 import Source
from app.rag.reranker.response_reranker import TextEmbeddingInferenceRerankEngine
from app.rag.retrieval.pubmed.pubmedqueryengine import PubmedSearchQueryEngine
from app.rag.retrieval.web.brave_search import BraveSearchQueryEngine
from app.services.search_utility import setup_logger
from app.settings import Settings

logger = setup_logger("Orchestrator")
TAG_RE = re.compile(r"<[^>]+>")


class AbstractSourceRecord(abc.ABC):
    @abc.abstractmethod
    def to_grpc_source(self) -> Source:
        raise NotImplementedError


class BraveSourceRecord(pydantic.BaseModel, AbstractSourceRecord):
    url: str
    page_age: str = ""

    def to_grpc_source(self) -> Source:
        return Source(url=self.url, metadata={"page_age": self.page_age})


class PubmedSourceRecord(pydantic.BaseModel, AbstractSourceRecord):
    url: str
    helper_text: str = ""

    def to_grpc_source(self) -> Source:
        return Source(url=self.url, metadata={"helper_text": self.helper_text})


SourceRecord = BraveSourceRecord | PubmedSourceRecord


class SearchResultRecord(pydantic.BaseModel):
    result: str
    sources: list[SourceRecord]


class Orchestrator:
    """
    Orchestrator is responsible for routing the search engine query.
    It routes the query into three routes now.The first one is clinical trails, second one is drug related information,
    and third one is pubmed brave.
    """

    def __init__(self, settings: Settings):
        self.settings = settings
        self.llm = dspy.OpenAI(
            model=settings.ai_models.router,
            api_key=settings.openai.api_key.get_secret_value(),
        )
        dspy.settings.configure(lm=self.llm)
        # self.router = RouterModule()
        # self.router.load(settings.dspy.orchestrator_router_prompt_program)

        # self.clinical_trial_search = ClinicalTrialText2SQLEngine(settings)
        # self.drug_chembl_search = DrugChEMBLText2CypherEngine(settings)
        self.pubmed_search = PubmedSearchQueryEngine(settings)
        self.brave_search = BraveSearchQueryEngine(settings.brave)

        self.rerank_engine = TextEmbeddingInferenceRerankEngine.from_settings(
            settings=self.settings.reranking
        )
        self.summarizer = SimpleSummarize(
            llm=TogetherLLM(
                model="mistralai/Mixtral-8x7B-Instruct-v0.1",
                api_key=self.settings.together.api_key.get_secret_value(),
            )
        )

    async def handle_pubmed_bioxriv_web_search(
        self, search_text: str
    ) -> SearchResultRecord | None:
        logger.info(
            f"Orchestrator.handle_pubmed_bioxriv_web_search Entered. search_text: {search_text}"
        )
        try:
            extracted_pubmed_results, extracted_web_results = await asyncio.gather(
                self.pubmed_search.call_pubmed_vectors(search_text=search_text),
                self.brave_search.call_brave_search_api(search_text=search_text),
            )
            extracted_results = extracted_pubmed_results + extracted_web_results
            logger.info(
                f"Orchestrator.handle_pubmed_bioxriv_web_search.extracted_results count: "
                f"{len(extracted_pubmed_results), len(extracted_web_results)}"
            )

            if not extracted_results:
                return None

            # rerank call
            reranked_results = self.rerank_engine.postprocess_nodes(
                nodes=extracted_results, query_bundle=QueryBundle(query_str=search_text)
            )

            result = self.summarizer.get_response(
                query_str=search_text,
                text_chunks=[
                    TAG_RE.sub("", node.get_content()) for node in reranked_results
                ],
            )

            sources = [node.node.metadata for node in reranked_results]

            return SearchResultRecord(result=result, sources=sources)

        except Exception as e:
            logger.exception(
                "Orchestrator.handle_pubmed_bioxriv_web_search failed -",
                exc_info=e,
                stack_info=True,
            )
            return None

    async def handle_clinical_trial_search(
        self, search_text: str
    ) -> SearchResultRecord | None:
        pass
        # TODO: Enable once stable and infallible
        """
        # clinical trial call
        logger.info(
            "Orchestrator.handle_clinical_trial_search.router_id clinical trial Entered."
        )
        try:
            sql_response = await self.clinical_trial_search.call_text2sql(
                search_text=search_text
            )
            result = str(sql_response)
            sources = [result]  # TODO: clinical trial sql sources impl

            logger.info(f"sql_response: {result}")

            return SearchResultRecord(result=result, sources=sources)
        except Exception as e:
            logger.exception(
                "Orchestrator.handle_clinical_trial_search.sqlResponse Exception -",
                exc_info=e,
                stack_info=True,
            )
        """

    async def handle_drug_search(self, search_text: str) -> SearchResultRecord | None:
        pass
        # TODO: Enable once stable and infallible
        """
        # drug information call
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
            )

            return SearchResultRecord(result=result, sources=sources)
        except Exception as e:
            logger.exception(
                "Orchestrator.handle_drug_search.cypher_response Exception -",
                exc_info=e,
                stack_info=True,
            )
        """

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
