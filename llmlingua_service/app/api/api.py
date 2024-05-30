from fastapi import APIRouter

from app.api.router.gzip import GzipRoute

from .endpoints import llmlingua_endpoint

router = APIRouter()
router.route_class = GzipRoute

router.include_router(
    llmlingua_endpoint.router,
    tags=["LLM Lingua"]
)