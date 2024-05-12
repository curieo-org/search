from collections.abc import Iterable as _Iterable
from collections.abc import Mapping as _Mapping
from typing import ClassVar as _ClassVar
from typing import Optional as _Optional
from typing import Union as _Union

from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf.internal import containers as _containers

DESCRIPTOR: _descriptor.FileDescriptor

class SearchRequest(_message.Message):
    __slots__ = ("query",)
    QUERY_FIELD_NUMBER: _ClassVar[int]
    query: str
    def __init__(self, query: _Optional[str] = ...) -> None: ...

class Source(_message.Message):
    __slots__ = ("url", "metadata")

    class MetadataEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(
            self,
            key: _Optional[str] = ...,
            value: _Optional[str] = ...,
        ) -> None: ...

    URL_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    url: str
    metadata: _containers.ScalarMap[str, str]
    def __init__(
        self,
        url: _Optional[str] = ...,
        metadata: _Optional[_Mapping[str, str]] = ...,
    ) -> None: ...

class SearchResponse(_message.Message):
    __slots__ = ("status", "result", "sources")
    STATUS_FIELD_NUMBER: _ClassVar[int]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    SOURCES_FIELD_NUMBER: _ClassVar[int]
    status: int
    result: str
    sources: _containers.RepeatedCompositeFieldContainer[Source]
    def __init__(
        self,
        status: _Optional[int] = ...,
        result: _Optional[str] = ...,
        sources: _Optional[_Iterable[_Union[Source, _Mapping]]] = ...,
    ) -> None: ...
