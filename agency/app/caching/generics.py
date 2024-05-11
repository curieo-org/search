from abc import abstractmethod
from collections.abc import Hashable
from typing import Generic, TypeVar

KeyT = TypeVar("KeyT", bound=Hashable)
ValueT = TypeVar("ValueT")
KeyTCo = TypeVar("KeyTCo", covariant=True, bound=Hashable)
ValueTCo = TypeVar("ValueTCo", covariant=True)


class AsyncCache(Generic[KeyT, ValueT]):
    @abstractmethod
    async def get(self, key: KeyT) -> ValueT | None: ...

    @abstractmethod
    async def set(self, key: KeyT, value: ValueT) -> None: ...

    @abstractmethod
    async def delete(self, key: KeyT) -> None: ...


class Cache(Generic[KeyT, ValueT]):
    @abstractmethod
    def get(self, key: KeyT) -> ValueT | None: ...

    @abstractmethod
    def set(self, key: KeyT, value: ValueT) -> None: ...

    @abstractmethod
    def delete(self, key: KeyT) -> None: ...


GenericCache = Cache[KeyT, ValueT] | AsyncCache[KeyT, ValueT]
