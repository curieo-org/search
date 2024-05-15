import enum
from abc import abstractmethod
from collections.abc import Hashable
from typing import Protocol, TypeVar, runtime_checkable

KeyT = TypeVar("KeyT", bound=Hashable)
KeyTCo = TypeVar("KeyTCo", covariant=True, bound=Hashable)
KeyTCon = TypeVar("KeyTCon", contravariant=True, bound=Hashable)
ValueT = TypeVar("ValueT")
ValueTCo = TypeVar("ValueTCo", covariant=True)

T = TypeVar("T")
T_co = TypeVar("T_co", covariant=True)
T_contra = TypeVar("T_contra", contravariant=True)


@enum.unique
class SyncPrimitive(enum.Enum):
    SYNC = "sync"
    ASYNC = "async"


class SyncType(Protocol):
    primitive: SyncPrimitive


@runtime_checkable
class Cache(SyncType, Protocol[KeyTCon, ValueT]):
    primitive: SyncPrimitive = SyncPrimitive.SYNC

    @abstractmethod
    def get(self, key: KeyTCon) -> ValueT | None: ...

    @abstractmethod
    def set(self, key: KeyTCon, value: ValueT) -> None: ...

    @abstractmethod
    def delete(self, key: KeyTCon) -> None: ...


@runtime_checkable
class AsyncCache(SyncType, Protocol[KeyTCon, ValueT]):
    primitive: SyncPrimitive = SyncPrimitive.ASYNC

    @abstractmethod
    async def aget(self, key: KeyTCon) -> ValueT | None: ...

    @abstractmethod
    async def aset(self, key: KeyTCon, value: ValueT) -> None: ...

    @abstractmethod
    async def adelete(self, key: KeyTCon) -> None: ...


GenericCache = Cache[KeyTCon, ValueT] | AsyncCache[KeyTCon, ValueT]
