import asyncio
from collections.abc import Awaitable, Coroutine, Generator
from contextlib import contextmanager
from typing import Any, TypeVar, Union

# Result type
R = TypeVar("R")


@contextmanager
def event_loop_context() -> Generator[asyncio.AbstractEventLoop, Any, Any]:
    loop = get_event_loop()
    try:
        yield loop
    finally:
        while loop.is_running():
            loop.stop()

        loop.close()


def get_event_loop() -> asyncio.AbstractEventLoop:
    """Get current asyncio event loop.

    asyncio has deprecated get_event_loop as of 3.10, with get_running_loop as the
    recommended alternative.
    We want to fall back to any event loop if running loop is not available.
    """
    try:
        return asyncio.get_running_loop()
    except RuntimeError:
        pass
    return asyncio.get_event_loop_policy().get_event_loop()


def complete_future(
    future: Union[asyncio.Future[R], Coroutine[Any, Any, R], Awaitable[R]],
) -> R:
    return get_event_loop().run_until_complete(future)
