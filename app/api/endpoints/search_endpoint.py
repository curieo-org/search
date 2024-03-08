from typing import List
from uuid import UUID

from fastapi import (APIRouter, Depends, Header, HTTPException)
from fastapi.responses import JSONResponse
from fastapi.logger import logger
from fastapi_versioning import version
from authx import AuthX, AuthXConfig
from fastapi_doc_http_response import get_responses
from fastapi_redis_cache import cache

from app.api.router.gzip import GzipRoute
from app.router.orchestrator import Orchestrator
from app.config import config, JWT_SECRET_KEY, JWT_ALGORITHM

router = APIRouter()
router.route_class = GzipRoute

auth_config = AuthXConfig()
auth_config.JWT_ALGORITHM = JWT_ALGORITHM
auth_config.JWT_SECRET_KEY = str(JWT_SECRET_KEY)

security = AuthX(config=auth_config)
orchestrator = Orchestrator(config)

@router.get('/login')
def login(username: str, password: str):
    if username == "curieo" and password == "curieo":
        token = security.create_access_token(uid=username)
        return {"access_token": token}
    raise HTTPException(401, detail={"message": "Bad credentials"})

@router.get(
    "/Search/",
    summary="List all Search Results",
    description="List all Search Results",
    dependencies=[Depends(security.access_token_required)],
    #responses=get_responses(201, 400, 401, 403)
)
@cache(expire=31536000)
@version(1, 0)
async def get_search_results(
    query: str = ""
) -> JSONResponse:
    data = await orchestrator.query_and_get_answer(search_text=query)
    
    return JSONResponse(status_code=200, content=data)