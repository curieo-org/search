from fastapi import FastAPI, HTTPException
from fastapi.responses import RedirectResponse

# from authx import AuthX, AuthXConfig
from app.database.redis import Redis

from app import config
from app.api import api
from app.api.errors.http_error import http_error_handler
from app.api.errors.if_none_match import IfNoneMatch, if_none_match_handler

from app.middleware.process_time import ProcessTimeHeaderMiddleware


def get_application() -> FastAPI:
    application = FastAPI(
        title=config.PROJECT_NAME, debug=config.DEBUG, version=config.VERSION
    )

    @application.get("/", include_in_schema=False)
    def redirect_to_docs() -> RedirectResponse:  # pylint: disable=W0612
        return RedirectResponse("/docs")

    @application.on_event("startup")
    async def startup():  # pylint: disable=W0612
        print()

        # connect to redis
        cache = Redis()
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
        cache = Redis()
        await cache.disconnect()

        # db connection
        # embedding connection
        # redis connection

    # exception handlers
    application.add_exception_handler(IfNoneMatch, if_none_match_handler)
    application.add_exception_handler(HTTPException, http_error_handler)

    # middlewares
    if config.SHOW_REQUEST_PROCESS_TIME_HEADER:
        application.add_middleware(ProcessTimeHeaderMiddleware)

    # routers
    application.include_router(api.router)
    return application


app = get_application()

if __name__ == "__main__":
    app.run(debug=True, port=5006)
