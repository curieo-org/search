import sentry_sdk

from app.grpc_types.agency_pb2 import SearchRequest, SearchResponse
from app.grpc_types.agency_pb2_grpc import AgencyService
from app.router.orchestrator import Orchestrator
from app.settings import app_settings
from app.utils.logging import setup_logger

orchestrator = Orchestrator(settings=app_settings)

logger = setup_logger("Search_API")


class Search(AgencyService):
    @staticmethod
    async def pubmed_bioxriv_web_search(
        request: SearchRequest,
        _target,
        _options=(),
        _channel_credentials=None,
        _call_credentials=None,
        _insecure=False,
        _compression=None,
        _wait_for_ready=None,
        _timeout=None,
        _metadata=None,
    ) -> SearchResponse:
        if trace_transaction := sentry_sdk.Hub.current.scope.transaction:
            trace_transaction.set_tag("title", "pubmed_bioxriv_web_search")

        query = request.query.strip()

        logger.info(f"pubmed_bioxriv_web_search. query: {query}")
        try:
            if search_result := await orchestrator.handle_pubmed_web_search(
                query,
            ):
                logger.info(f"pubmed_bioxriv_web_search. result: {search_result}")

                sources = [source.to_grpc_source() for source in search_result.sources]

                return SearchResponse(
                    status=200,
                    result=search_result.result,
                    sources=sources,
                )

            logger.error("pubmed_bioxriv_web_search. failed to retrieve search results")

            return SearchResponse(status=500, result="Search failed", sources=[])
        except Exception as e:
            logger.exception(e)

            return SearchResponse(status=500, result="Search failed", sources=[])
