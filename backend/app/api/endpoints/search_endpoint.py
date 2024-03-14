from fastapi import (APIRouter, Depends, HTTPException)
from fastapi.responses import JSONResponse
from fastapi.logger import logger
from fastapi_versioning import version
from authx import AuthX, AuthXConfig
from fastapi_redis_cache import cache

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
    "/Search/",
    summary="List all Search Results",
    description="List all Search Results",
    dependencies=[Depends(security.access_token_required)]
)
@cache(expire=31536000)
@version(1, 0)
async def get_search_results(
    query: str = ""
) -> JSONResponse:
    logger.debug(f"Search_Endpoint.get_search_results. query: {query}")
    data = await orchestrator.query_and_get_answer(search_text=query)
    logger.debug(f"Search_Endpoint.get_search_results. result: {data}")

    return JSONResponse(status_code=200, content=data)
