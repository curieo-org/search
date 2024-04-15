from app.database.redis import init_redis_client, get_redis_client
from app.services.search_utility import setup_logger
from app.services.tracing import setup_tracing
from app.settings import Settings

from concurrent import futures
import grpc
from app.api import setup_grpc_api
import asyncio

settings = Settings()
logger = setup_logger("Main")

_cleanup_coroutines = []


async def start_services():
    # Initialize redis client at app level
    cache = init_redis_client(settings.redis)

    # connect to redis
    await cache.connect()

    # tracing
    setup_tracing(settings.sentry)

    # db connection
    # embedding connection
    # redis connection
    # brave connection checking function
    # llmservice connection checking function


async def stop_services(server):
    logger.info("Server graceful shutdown started")

    # disconnect from redis
    cache = get_redis_client()
    await cache.disconnect()

    # graceful shutdown
    await server.stop(settings.project.graceful_shutdown_period)


async def serve():
    await start_services()

    server = grpc.aio.server(
        futures.ThreadPoolExecutor(max_workers=settings.project.max_grpc_workers)
    )
    setup_grpc_api(server)

    port = settings.project.port
    server.add_insecure_port(f"[::]:{port}")

    await server.start()
    logger.info(f"Server started on port: {port}")

    _cleanup_coroutines.append(stop_services(server))
    await server.wait_for_termination()


def start_server():
    loop = asyncio.get_event_loop()
    try:
        loop.run_until_complete(serve())
    finally:
        loop.run_until_complete(*_cleanup_coroutines)
        loop.close()


app = start_server()
