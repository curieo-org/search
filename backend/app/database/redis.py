import redis.asyncio as aioredis
import random
import opentelemetry
from app.config import REDIS_URL, CACHE_MAX_AGE, CACHE_MAX_SORTED_SET
from app.services.tracing import SentryTracer

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


    async def get_value(self, key: str, parent_trace_span: opentelemetry.trace.Span) -> str:
        global connection

        tracer_span = await SentryTracer().create_child_span(parent_trace_span, 'redis_get_value')

        with tracer_span:
            tracer_span.set_attribute('description', f"Get Value from Redis. key: {key}")
            value = await connection.get(key)
            tracer_span.set_attribute('result', f"Key: {key}, Value: {value}")

        return str(value, 'utf-8') if value else None
    

    async def set_value(self, key: str, value: str, parent_trace_span: opentelemetry.trace.Span) -> None:
        global connection

        tracer_span = await SentryTracer().create_child_span(parent_trace_span, 'redis_set_value')

        with tracer_span:
            tracer_span.set_attribute('description', f"Set Value to Redis. key: {key}, value: {value}, expire: {CACHE_MAX_AGE}")
            await connection.set(key, value, ex=CACHE_MAX_AGE)
            tracer_span.set_attribute('result', 'success')


    async def add_to_sorted_set(self, space: str, key: str, parent_trace_span: opentelemetry.trace.Span) -> None:
        global connection

        tracer_span = await SentryTracer().create_child_span(parent_trace_span, 'redis_add_to_sorted_set')

        with tracer_span:
            tracer_span.set_attribute('description', f"Add to Sorted Set in Redis. space: {space}, key: {key}")
            await connection.zincrby(space, 1, key)
            tracer_span.set_attribute('result', 'success')


    async def get_sorted_set(self, space: str, start: int, stop: int, parent_trace_span: opentelemetry.trace.Span) -> list[str]:
        global connection

        tracer_span = await SentryTracer().create_child_span(parent_trace_span, 'redis_get_sorted_set')

        with tracer_span:
            tracer_span.set_attribute('description', f"Get Sorted Set from Redis. space: {space}, start: {start}, stop: {stop}")

            random_number = random.random()
            if random_number < 0.1:
                await connection.zremrangebyrank(space, 0, -CACHE_MAX_SORTED_SET-1)
                
            values = await connection.zrevrange(space, start, stop, withscores=False)
            tracer_span.set_attribute('result', str(values))

        return [str(value, 'utf-8') for value in values]