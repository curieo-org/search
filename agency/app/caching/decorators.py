import asyncio
import functools
from collections import OrderedDict
from collections.abc import Callable, Coroutine, Hashable
from functools import wraps
from inspect import signature
from typing import Any, Generic, ParamSpec, TypeVar

from app.caching.generics import AsyncCache, Cache, GenericCache, KeyTCo, ValueT
from app.utils.logging import setup_logger

logger = setup_logger("caching")


def extract_keys(fstring: str) -> list[str]:
    keys = []
    initial_bracket = -1
    for i, c in enumerate(fstring):
        if c == "{":
            initial_bracket = i + 1
        elif c == "}" and initial_bracket != -1:
            keys.append(fstring[initial_bracket:i])
            initial_bracket = -1

    return keys


SyncFn = Callable[..., ValueT]
AsyncFn = Callable[..., Coroutine[Any, Any, ValueT]]

CachedFn = Callable[[KeyTCo], ValueT]
KeyFn = Callable[..., KeyTCo]
GetKeyFn = Callable[..., KeyFn]
GetCacheFn = Callable[..., GenericCache]
CachedFactory = Callable[[KeyFn], CachedFn]
_PWrapped = ParamSpec("_PWrapped")
_RWrapped = TypeVar("_RWrapped")
_PWrapper = ParamSpec("_PWrapper")
_RWrapper = TypeVar("_RWrapper")
Wrapped = Generic[_PWrapped, _RWrapped, _PWrapper, _RWrapper]


def fcache_key_fn(fstring: str) -> GetKeyFn:
    """Uses provided fstring and decorated signature to create unique cache keys.

    Reads the signature of the function that is being decorated.
    Compares function signature parameters to provided fstring.
    If valid, fills fstring with parameters from function at call time to generate
    cache key.
    """

    def get_key_fn(fn) -> Callable[..., Hashable]:
        sig = signature(fn)
        parameters = OrderedDict((p, k) for (p, k) in sig.parameters.items())
        keys = extract_keys(fstring)
        if keys is None:
            logger.warning(
                "No keys extracted from fstring %s. It is likely badly formatted.",
                fstring,
            )
        else:
            for key in list(keys):
                if key not in parameters:
                    raise ValueError(f"Incorrect key for function signature: {key}")

        def key_fn(*args: Any, **kwargs: Any) -> Hashable:
            if keys is None:
                cache_key = fstring
            else:
                keymap: dict[str, str] = {}

                for arg, (pk, _) in zip(args, parameters.items()):
                    if pk in keys:
                        keymap[pk] = str(arg)

                for kw, arg in kwargs.items():
                    if kw in keys:
                        keymap[kw] = str(arg)

                cache_key = fstring.format(**keymap)

            return cache_key

        return key_fn

    return get_key_fn


def fcached(fstring: str, cache_fn: GetCacheFn) -> CachedFn:
    """f-string based caching decorator.

    Specify cache key using standard python format string.
    Make sure you specify variable keys that match the function parameters correctly.

    Incorrect:
    @cached(get_cache, "user.info")
    def get_user_info(user_id: int):

    Here the key will always be "user.info" regardless of which user is being processed.

    Incorrect:
    @cached(get_cache, "user.{user}.info")
    def get_user_info(user_id: int):

    This will fail as the parameter "user" does not exist.

    Correct:
    @cached(get_cache, "user.{user_id}.info")
    def get_user_info(user_id: int):

    This will work as expected. The key will be unique across users as "user.1.info",
    "user.42.info" etc.
    """
    return cached_decorator(
        get_cache=cache_fn,
        get_key_fn=fcache_key_fn(fstring=fstring),
    )


def cached_factory(*, cache_fn: GetCacheFn) -> CachedFactory:
    """Returns cached with cache_fn already set."""
    return functools.partial(cached_decorator, cache_fn=cache_fn)


def fcached_factory(*, cache_fn: GetCacheFn) -> Callable[[str], CachedFn]:
    """Returns template_cached with cache_fn already set."""
    return functools.partial(fcached, cache_fn=cache_fn)


def into_coroutine(fn: SyncFn | AsyncFn) -> AsyncFn:
    # type: ignore
    def as_coro(sync_fn: SyncFn) -> AsyncFn:
        @wraps(sync_fn)
        async def coro(*args: Any, **kwargs: Any) -> ValueT:
            return sync_fn(*args, **kwargs)

        return coro

    if asyncio.iscoroutinefunction(fn):
        return fn

    return as_coro(fn)


def cached_decorator(get_cache: GetCacheFn, get_key_fn: GetKeyFn) -> CachedFn:
    """Decorator that caches a function or coroutine."""

    def decorator(fn: SyncFn | AsyncFn) -> AsyncFn:
        cache: GenericCache = get_cache()
        key_fn: KeyFn = get_key_fn(fn)
        coro = into_coroutine(fn)

        if isinstance(cache, Cache):

            @wraps(fn)
            async def wrapper(*args: Any, **kwargs: Any) -> ValueT:
                key = key_fn(*args, **kwargs)
                if cached_value := cache.get(key):
                    return cached_value

                value: ValueT = await coro(*args, **kwargs)
                cache.set(key, value)
                return value

            return wrapper

        if isinstance(cache, AsyncCache):

            @wraps(fn)
            async def wrapper(*args: Any, **kwargs: Any) -> ValueT:
                key = key_fn(*args, **kwargs)
                if cached_value := await cache.aget(key):
                    return cached_value

                value: ValueT = await coro(*args, **kwargs)
                await cache.aset(key, value)
                return value

            return wrapper

        raise ValueError("Unreachable state.")

    return decorator
