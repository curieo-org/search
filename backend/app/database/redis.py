import redis.asyncio as aioredis
import os
import random
from app.config import REDIS_URL, CACHE_MAX_AGE, CACHE_MAX_SORTED_SET

connection = None

class Redis:
    def __init__(self):
        pass
    
    async def connect(self):
        global connection

        if not connection:
            connection = await aioredis.Redis.from_url(str(REDIS_URL))
    

    async def disconnect(self):
        global connection

        await connection.close()


    async def get_value(self, key: str) -> str:
        global connection

        value = await connection.get(key)
        return str(value, 'utf-8') if value else None
    

    async def set_value(self, key: str, value: str, expire=CACHE_MAX_AGE):
        global connection

        await connection.set(key, value, ex=expire)


    async def add_to_sorted_set(self, space: str, key: str):
        global connection

        await connection.zincrby(space, 1, key)


    async def get_sorted_set(self, space: str, start: int, stop: int) -> list[str]:
        global connection

        random_number = random.random()
        if random_number < 0.1:
            await connection.zremrangebyrank(space, 0, -CACHE_MAX_SORTED_SET-1)
            
        values = await connection.zrevrange(space, start, stop, withscores=False)
        return [str(value, 'utf-8') for value in values]