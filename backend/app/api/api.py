from fastapi import APIRouter

from app.api.router.gzip import GzipRoute

from .endpoints import search_endpoint

router = APIRouter()
router.route_class = GzipRoute

router.include_router(search_endpoint.router, tags=["Search Results"])
