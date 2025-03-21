import json
from datetime import timedelta

import pydantic
import redis.asyncio as aioredis

from app.caching.decorators import fcached_factory
from app.caching.generics import AsyncCache
from app.settings import RedisSettings, app_settings
from app.utils.logging import setup_logger

ExpiryT = int | timedelta
logger = setup_logger("RedisCache")

RedisKey = str | bytes
RedisValue = str | bytes | float | int


class RedisCache(AsyncCache[RedisKey, RedisValue]):
    # TODO: Add built in support for pydantic encoding / decoding
    def __init__(self, *, url: str, default_expiry: ExpiryT):
        self.url = url
        self.default_expiry = default_expiry
        self.redis = aioredis.Redis.from_url(self.url)

    async def close(self) -> None:
        await self.redis.close()

    async def aget(self, key: RedisKey) -> RedisValue | None:
        try:
            return await self.redis.get(key)
        except aioredis.RedisError as e:
            logger.exception("Retrieving value from redis failed: ", e)
            return None

    async def aset(
        self,
        key: RedisKey,
        value: RedisValue,
        expire: ExpiryT | None = None,
    ) -> None:
        try:
            if not expire:
                expire = self.default_expiry

            if isinstance(value, pydantic.BaseModel):
                await self.redis.set(key, value.model_dump_json(), ex=expire)
            elif isinstance(value, dict):
                await self.redis.set(key, json.dumps(value), ex=expire)
            else:
                await self.redis.set(key, value, ex=expire)

        except aioredis.RedisError as e:
            logger.exception("Setting value in redis failed: ", e)

    async def adelete(self, key: RedisKey) -> None:
        try:
            await self.redis.delete(key)
        except aioredis.RedisError as e:
            logger.exception("Deleting key/value from redis failed: ", e)

    def __repr__(self) -> str:
        """Repr RedisCache string."""
        return f"{self.__class__.__name__}(default_expiry={self.default_expiry})"


def _init_redis_cache(settings: RedisSettings) -> RedisCache:
    """Initializes Redis client. Ensures it is only done once."""
    return RedisCache(
        url=settings.url.get_secret_value(),
        default_expiry=settings.default_expiry,
    )


redis_cache = _init_redis_cache(settings=app_settings.redis)


def get_redis_cache() -> RedisCache:
    return redis_cache


fcached = fcached_factory(
    cache_fn=get_redis_cache,
)
