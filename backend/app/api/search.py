import sentry_sdk

from app.router.orchestrator import Orchestrator
from app.services.search_utility import setup_logger
from app.settings import Settings

import grpc
from app.grpc_types.agency_pb2_grpc import AgencyService
from app.grpc_types.agency_pb2 import SearchRequest, SearchResponse

orchestrator = Orchestrator(settings=Settings())

logger = setup_logger("Search_API")


class Search(AgencyService):
    async def search(
        self, request: SearchRequest, context: grpc.aio.ServicerContext
    ) -> SearchResponse:
        if trace_transaction := sentry_sdk.Hub.current.scope.transaction:
            trace_transaction.set_tag("title", "api_get_search_results")

        query = request.query
        query = query.strip()
        route_category = request.route_category

        logger.info(f"get_search_results. query: {query}")

        if search_result := await orchestrator.query_and_get_answer(
            search_text=query, route_category=route_category
        ):
            logger.info(f"get_search_results. result: {search_result}")

            sources = [source.to_grpc_source() for source in search_result.sources]

            return SearchResponse(
                status=200, result=search_result.result, sources=sources
            )

        logger.error("get_search_results. failed to get the search results")

        return SearchResponse(
            status=500, result="failed to get the search results", sources=[]
        )
