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
    dependencies=[Depends(security.access_token_required)]
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

    logger.debug(f"Search_Endpoint.get_search_results. result: {search_result}")

    return JSONResponse(status_code=200, content=search_result)
