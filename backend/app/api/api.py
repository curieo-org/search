from fastapi import APIRouter

from app.api.router.gzip import GzipRoute

from .auth import routes as auth_routes
from .endpoints import search_endpoint

router = APIRouter()
router.route_class = GzipRoute

router.include_router(search_endpoint.router, tags=["Search Results"])
router.include_router(auth_routes.router, tags=["Authentication"])
