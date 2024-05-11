import asyncio
import functools
import json
from collections.abc import Callable, Coroutine
from functools import _Wrapped, wraps
from inspect import signature
from typing import Any, Never

from app.caching.generics import Cache, GenericCache, KeyTCo, ValueTCo


def extract_keys(key_template: str) -> list[str]:
    keys = []

    state = 0
    initial_bracket = -1
    for i, c in enumerate(key_template):
        if c == "{" and state == 0:
            state = 1
            initial_bracket = i + 1
        elif c == "{":
            state = 0
        elif c == "}" and state == 1:
            keys.append(key_template[initial_bracket:i])
            state = 0

    return keys


def cached(key_template: str) -> Any:
    """Decorator that caches results in Redis.

    Specify cache key using standard python format string.
    Make sure you specify variable keys that match the function parameters correctly.

    Incorrect:
    @cached("user.info")
    def get_user_info(user_id: int):

    Here the key will always be "user.info" regardless of which user is being processed.

    Incorrect:
    @cached("user.{user}.info")
    def get_user_info(user_id: int):

    This will fail as the parameter "user" does not exist.

    Correct:
    @cached("user.{user_id}.info")
    def get_user_info(user_id: int):

    This will work as expected. The key will be unique across users as "user.1.info",
    "user.42.info" etc.
    """

    def inner(fn):
        @wraps(fn)
        async def wrapper(*args: Any, **kwargs: Any):
            # Get signature of fn and inspect the parameters against our key template.
            sig = signature(fn)
            parameters = sig.parameters

            keys = extract_keys(key_template)
            if keys is None:
                cache_key = key_template
            else:
                for key in list(keys):
                    if key not in parameters:
                        raise ValueError(f"Incorrect key for function signature: {key}")

                cache_key = key_template.format(keys)

            redis = None  # get_redis_cache()

            if result := await redis.get(cache_key):
                return json.loads(result)

            # Run the function and cache the result for next time.
            value = await fn(*args, **kwargs)
            await redis.set(cache_key, json.dumps(value))
            return value

        return wrapper

    return inner


CachedFn = Callable[[KeyTCo], ValueTCo]
KeyFn = Callable[..., KeyTCo]
GetCacheFn = Callable[..., GenericCache]
CachedFactory = Callable[[KeyFn], CachedFn]
Wrapped = _Wrapped[[Any, Any], Any, [Any, Any], Coroutine[Any, Any, Never]]


def cached_factory(*, cache_fn: GetCacheFn) -> CachedFactory:
    return functools.partial(cached2, cache_fn=cache_fn)


def cached2(cache_fn: GetCacheFn, key_fn: KeyFn) -> CachedFn:
    """Decorator that caches a function or coroutine."""

    def decorator(fn) -> Wrapped:
        cache: GenericCache = cache_fn()
        iscoroutinefn = asyncio.iscoroutinefunction(fn)

        if isinstance(cache, Cache):
            if iscoroutinefn:

                @wraps(fn)
                async def wrapper(*args: Any, **kwargs: Any) -> ValueTCo:
                    key = key_fn(*args, **kwargs)
                    if value := cache.get(key):
                        return value

                    value = await fn(*args, **kwargs)

                    cache.set(key, value)

                    return value

            else:

                @wraps(fn)
                def wrapper(*args: Any, **kwargs: Any):
                    key = key_fn(*args, **kwargs)
                    if value := cache.get(key):
                        return value

                    value = fn(*args, **kwargs)

                    cache.set(key, value)

                    return value

            return wrapper

        if iscoroutinefn:

            @wraps(fn)
            async def wrapper(*args: Any, **kwargs: Any) -> ValueTCo:
                key = key_fn(*args, **kwargs)
                if value := await cache.get(key):
                    return value

                value = await fn(*args, **kwargs)

                await cache.set(key, value)

                return value

        else:

            @wraps(fn)
            async def wrapper(*args: Any, **kwargs: Any) -> ValueTCo:
                key = key_fn(*args, **kwargs)
                if value := await cache.get(key):
                    return value

                value = fn(*args, **kwargs)

                await cache.set(key, value)

                return value

        return wrapper

    return decorator
