from contextlib import asynccontextmanager

from fastapi import FastAPI, HTTPException
from fastapi.responses import RedirectResponse

from app.api import api
from app.api.errors.http_error import http_error_handler
from app.api.errors.if_none_match import IfNoneMatch, if_none_match_handler
from app.database.redis import init_redis_client
from app.middleware.process_time import ProcessTimeHeaderMiddleware
from app.services.tracing import setup_tracing
from app.settings import Settings

settings = Settings()


@asynccontextmanager
async def lifespan(app: FastAPI):
    # Initialize redis client at app level
    cache = init_redis_client(settings.redis)

    # connect to redis
    await cache.connect()

    # db connection
    # embedding connection
    # redis connection
    # brave connection checking function
    # llmservice connection checking function

    yield

    # disconnect from redis
    await cache.disconnect()


def get_application() -> FastAPI:
    application = FastAPI(
        title=settings.project.name,
        debug=settings.project.debug,
        version=settings.project.version,
        lifespan=lifespan,
    )

    @application.get("/", include_in_schema=False)
    def redirect_to_docs() -> RedirectResponse:  # pylint: disable=W0612
        return RedirectResponse("/docs")

    # exception handlers
    application.add_exception_handler(IfNoneMatch, if_none_match_handler)
    application.add_exception_handler(HTTPException, http_error_handler)

    # middlewares
    if settings.project.show_request_process_time_header:
        application.add_middleware(ProcessTimeHeaderMiddleware)

    # routers
    application.include_router(api.router)

    # tracing
    setup_tracing(settings.sentry)

    return application


app = get_application()
