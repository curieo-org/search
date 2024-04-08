import json

import sentry_sdk
from fastapi import APIRouter, HTTPException
from fastapi.responses import JSONResponse
from fastapi_versioning import version

from app.api.common.util import RouteCategory
from app.api.router.gzip import GzipRoute
from app.database.redis import get_redis_client
from app.router.orchestrator import Orchestrator
from app.services.search_utility import setup_logger
from app.settings import Settings

router = APIRouter()
router.route_class = GzipRoute
orchestrator = Orchestrator(settings=Settings())

logger = setup_logger("Search_Endpoint")


@router.get(
    "/search",
    summary="List all Search Results",
    description="List all Search Results",
    response_model=dict[str, str],
)
@version(1, 0)
async def get_search_results(
    query: str = "", route_category: RouteCategory = RouteCategory.PBW
) -> JSONResponse:
    if trace_transaction := sentry_sdk.Hub.current.scope.transaction:
        trace_transaction.set_tag("title", "api_get_search_results")

    logger.info(f"get_search_results. query: {query}")

    query = query.strip()
    cache = get_redis_client()
    cache_key = f"{query}##{route_category}"

    if search_result := await cache.get_value(cache_key):
        return JSONResponse(status_code=200, content=search_result)

    if search_result := await orchestrator.query_and_get_answer(
        search_text=query, route_category=route_category
    ):
        await cache.set_value(cache_key, json.dumps(search_result))

        await cache.add_to_sorted_set("searched_queries", query)

        logger.info(f"get_search_results. result: {search_result}")

        return JSONResponse(status_code=200, content=search_result)

    return JSONResponse(status_code=500, content={"message": "Search failed"})


@router.get(
    "/topqueries",
    summary="List all top search queries",
    description="List all Top Search Queries",
    response_model=list[str],
)
@version(1, 0)
async def get_top_search_queries(limit: int) -> JSONResponse:
    if trace_transaction := sentry_sdk.Hub.current.scope.transaction:
        trace_transaction.set_tag("title", "api_get_top_search_queries")

    logger.info("get_top_search_queries")

    if limit <= 0:
        raise HTTPException(status_code=400, detail="Limit should be greater than 0")

    cache = get_redis_client()
    last_x_keys = await cache.get_sorted_set("searched_queries", 0, limit - 1)

    logger.info(f"get_top_search_queries. result: {last_x_keys}")

    return JSONResponse(status_code=200, content=last_x_keys)
