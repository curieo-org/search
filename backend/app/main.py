from fastapi import FastAPI, HTTPException
from fastapi.responses import RedirectResponse
from app.settings import Settings
from app.api import api
from app.api.errors.http_error import http_error_handler
from app.api.errors.if_none_match import IfNoneMatch, if_none_match_handler
from app.middleware.process_time import ProcessTimeHeaderMiddleware
from app.database.redis import init_redis_client
from app.services.tracing import setup_tracing


def get_application() -> FastAPI:
    settings = Settings()
    redis_settings = settings.redis

    application = FastAPI(
        title=settings.project.name, debug=settings.project.debug,
        version=settings.project.version
    )

    # Initialize redis client at app level
    cache = init_redis_client(redis_settings)

    @application.get("/", include_in_schema=False)
    def redirect_to_docs() -> RedirectResponse:  # pylint: disable=W0612
        return RedirectResponse("/docs")

    @application.on_event("startup")
    async def startup():  # pylint: disable=W0612
        print()

        # connect to redis
        await cache.connect()

        # db connection
        # embedding connection
        # redis connection
        # brave connection checking function
        # llmservice connection checking function

    @application.on_event("shutdown")
    async def shutdown():  # pylint: disable=W0612
        print()

        # disconnect from redis
        await cache.disconnect()

        # db connection
        # embedding connection
        # redis connection

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


if __name__ == "__main__":
    app = get_application()
    app.run(debug=True, port=5006)
