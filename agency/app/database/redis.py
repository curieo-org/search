import logging

import redis.asyncio as aioredis

from app.services.search_utility import setup_logger
from app.settings import RedisSettings

logger = setup_logger("Redis")


class Redis:
    def __init__(self, *, url: str, max_age: int, max_sorted_set: int):
        self.url = url
        self.max_age = max_age
        self.max_sorted_set = max_sorted_set
        self.connection: aioredis.Redis | None = None

    async def connect(self):
        if not self.connection:
            self.connection = await aioredis.Redis.from_url(self.url)

    async def disconnect(self):
        await self.connection.close()

    async def get_value(self, key: str) -> str | None:
        try:
            value = await self.connection.get(key)
            return str(value, "utf-8") if value else None
        except aioredis.RedisError as e:
            logger.exception("Retrieving value from cache failed: ", e)
            return None

    async def set_value(self, key: str, value: str, expire: int | None = None):
        try:
            if not expire:
                expire = self.max_age
            await self.connection.set(key, value, ex=expire)
        except aioredis.RedisError as e:
            logger.exception("Setting value into cache failed: ", e)


_redis_client: Redis | None = None


def get_redis_client() -> Redis:
    global _redis_client
    if not _redis_client:
        raise ValueError("Redis client has not been initialized")

    return _redis_client


def init_redis_client(settings: RedisSettings) -> Redis:
    """
    Initializes Redis client. Ensures it is only done once.
    """
    global _redis_client
    if not _redis_client:
        _redis_client = Redis(
            url=settings.url.get_secret_value(),
            max_age=settings.max_age,
            max_sorted_set=settings.max_sorted_set,
        )
    else:
        logging.warning("Tried initializing redis client more than once, skipping ...")

    return _redis_client
