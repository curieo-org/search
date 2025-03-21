from concurrent import futures

import grpc

from app.api import setup_grpc_api
from app.caching.redis import get_redis_cache
from app.settings import app_settings
from app.tracing.utils import setup_tracing
from app.utils.asyncio import event_loop_context
from app.utils.logging import setup_logger

logger = setup_logger("Main")

_cleanup_coroutines: list = []


async def start_services() -> None:
    # tracing
    setup_tracing(app_settings.tracing)

    # db connection
    # embedding connection
    # redis connection
    # brave connection checking function
    # llmservice connection checking function


async def stop_services(server: grpc.aio.Server) -> None:
    logger.info("Server graceful shutdown started")

    # disconnect from redis
    cache = get_redis_cache()
    await cache.close()

    # graceful shutdown
    await server.stop(app_settings.project.graceful_shutdown_period)


async def serve() -> None:
    await start_services()

    server = grpc.aio.server(
        futures.ThreadPoolExecutor(max_workers=app_settings.project.max_grpc_workers),
    )
    setup_grpc_api(server)

    port = app_settings.project.port
    server.add_insecure_port(f"[::]:{port}")

    await server.start()
    logger.info(f"Server started on port: {port}")

    _cleanup_coroutines.append(stop_services(server))
    await server.wait_for_termination()


def start_server() -> None:
    with event_loop_context() as loop:
        try:
            loop.run_until_complete(serve())
        finally:
            if _cleanup_coroutines:
                loop.run_until_complete(*_cleanup_coroutines)


app = start_server
