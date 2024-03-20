from fastapi import (APIRouter, Depends, HTTPException)
from fastapi.responses import JSONResponse
from fastapi.logger import logger
from fastapi_versioning import version
from authx import AuthX, AuthXConfig
from app.database.redis import Redis

from app.api.router.gzip import GzipRoute
from app.router.orchestrator import Orchestrator
from app.config import config, JWT_SECRET_KEY, JWT_ALGORITHM
from app.services.search_utility import setup_logger

router = APIRouter()
router.route_class = GzipRoute

auth_config = AuthXConfig()
auth_config.JWT_ALGORITHM = JWT_ALGORITHM
auth_config.JWT_SECRET_KEY = str(JWT_SECRET_KEY)

security = AuthX(config=auth_config)
orchestrator = Orchestrator(config)

logger = setup_logger('Search_Endpoint')


@router.get(
    "/search",
    summary="List all Search Results",
    description="List all Search Results",
    dependencies=[Depends(security.access_token_required)],
    response_model=str
)
@version(1, 0)
async def get_search_results(
    query: str = ""
) -> JSONResponse:
    logger.debug(f"Search_Endpoint.get_search_results. query: {query}")

    query = query.strip()
    cache = Redis()
    search_result = await cache.get_value(query)

    if search_result:
        logger.debug(f"Search_Endpoint.get_search_results. cached_result: {search_result}")
    else:
        search_result = await orchestrator.query_and_get_answer(search_text=query)
        await cache.set_value(query, search_result)

    await cache.add_to_sorted_set("searched_queries", query)

    logger.debug(f"Search_Endpoint.get_search_results. result: {search_result}")

    return JSONResponse(status_code=200, content=search_result)



@router.get(
    "/topqueries",
    summary="List all top search queries",
    description="List all Top Search Queries",
    dependencies=[Depends(security.access_token_required)],
    response_model=list[str]
)
@version(1, 0)
async def get_top_search_queries(limit: int) -> JSONResponse:
    logger.debug(f"Search_Endpoint.get_top_search_queries")

    if limit <= 0:
        raise HTTPException(status_code=400, detail="Limit should be greater than 0")

    cache = Redis()
    last_x_keys = await cache.get_sorted_set("searched_queries", 0, limit - 1)

    logger.debug(f"Search_Endpoint.get_top_search_queries. result: {last_x_keys}")

    return JSONResponse(status_code=200, content=last_x_keys)
