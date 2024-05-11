import logging
from datetime import timedelta
from typing import Union

import redis.asyncio as aioredis

from app.caching.decorators import cached_factory
from app.caching.generics import AsyncCache
from app.services.search_utility import setup_logger
from app.settings import RedisSettings

ExpiryT = Union[int, timedelta]
logger = setup_logger("RedisCache")

RedisKey = str | bytes
RedisValue = str | bytes | float | int


class RedisCache(AsyncCache[RedisKey, RedisValue]):
    def __init__(self, *, url: str, default_expiry: ExpiryT):
        self.url = url
        self.default_expiry = default_expiry
        self._redis: aioredis.Redis | None = None

    @property
    def redis(self) -> aioredis.Redis:
        if self._redis is None:
            self._redis = aioredis.Redis.from_url(self.url)

        return self._redis

    async def close(self) -> None:
        await self.redis.close()

    async def get(self, key: RedisKey) -> RedisValue | None:
        try:
            return await self.redis.get(key)
        except aioredis.RedisError as e:
            logger.exception("Retrieving value from redis failed: ", e)
            return None

    async def set(
        self, key: RedisKey, value: RedisValue, expire: ExpiryT | None = None,
    ) -> None:
        try:
            if not expire:
                expire = self.default_expiry
            await self.redis.set(key, value, ex=expire)

        except aioredis.RedisError as e:
            logger.exception("Setting value in redis failed: ", e)

    async def delete(self, key: RedisKey) -> None:
        try:
            await self.redis.delete(key)
        except aioredis.RedisError as e:
            logger.exception("Deleting key/value from redis failed: ", e)

    def __repr__(self):
        """Repr RedisCache string."""
        return "%s(default_expiry=%i)" % (self.__class__.__name__, self.default_expiry)


_redis_cache: RedisCache | None = None


def get_redis_cache() -> RedisCache:
    global _redis_cache  # noqa: PLW0602
    if not _redis_cache:
        raise ValueError("Redis client has not been initialized")

    return _redis_cache


def init_redis_cache(settings: RedisSettings) -> RedisCache:
    """Initializes Redis client. Ensures it is only done once."""
    global _redis_cache  # noqa: PLW0603
    if not _redis_cache:
        _redis_cache = RedisCache(
            url=settings.url.get_secret_value(),
            default_expiry=settings.default_expiry,
        )
    else:
        logging.warning("Tried initializing redis client more than once, skipping ...")

    return _redis_cache


cached = cached_factory(cache_fn=get_redis_cache)
