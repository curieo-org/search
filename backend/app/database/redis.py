import redis.asyncio as aioredis
import os
from app.config import REDIS_URL, CACHE_MAX_AGE

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