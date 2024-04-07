import logging

import redis.asyncio as aioredis
import random
from app.settings import RedisSettings


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

    async def get_value(self, key: str) -> str:

        value = await self.connection.get(key)

        return str(value, 'utf-8') if value else None

    async def set_value(self, key: str, value: str,
                        expire: int | None = None) -> None:
        if not expire:
            expire = self.max_age
        await self.connection.set(key, value, ex=expire)

    async def add_to_sorted_set(self, space: str, key: str) -> None:
        await self.connection.zincrby(space, 1, key)

    async def get_sorted_set(self, space: str, start: int, stop: int) -> list[str]:
        random_number = random.random()
        if random_number < 0.1:
            await self.connection.zremrangebyrank(space, 0,
                                                  -self.max_sorted_set - 1)

        values = await self.connection.zrevrange(space, start, stop, withscores=False)

        return [str(value, 'utf-8') for value in values]


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
        _redis_client = Redis(url=settings.url,
                              max_age=settings.max_age,
                              max_sorted_set=settings.max_sorted_set)
    else:
        logging.warning("Tried initializing redis client more than once, skipping ...")
    
    return _redis_client
