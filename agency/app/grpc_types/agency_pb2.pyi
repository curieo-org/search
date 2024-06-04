from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import (
    ClassVar as _ClassVar,
    Iterable as _Iterable,
    Mapping as _Mapping,
    Optional as _Optional,
    Union as _Union,
)

DESCRIPTOR: _descriptor.FileDescriptor

class SourceType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    Pdf: _ClassVar[SourceType]
    Image: _ClassVar[SourceType]
    Url: _ClassVar[SourceType]

Pdf: SourceType
Image: SourceType
Url: SourceType

class SearchRequest(_message.Message):
    __slots__ = ("query",)
    QUERY_FIELD_NUMBER: _ClassVar[int]
    query: str
    def __init__(self, query: _Optional[str] = ...) -> None: ...

class Source(_message.Message):
    __slots__ = ("url", "title", "description", "source_type", "metadata")

    class MetadataEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(
            self, key: _Optional[str] = ..., value: _Optional[str] = ...
        ) -> None: ...

    URL_FIELD_NUMBER: _ClassVar[int]
    TITLE_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    SOURCE_TYPE_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    url: str
    title: str
    description: str
    source_type: SourceType
    metadata: _containers.ScalarMap[str, str]
    def __init__(
        self,
        url: _Optional[str] = ...,
        title: _Optional[str] = ...,
        description: _Optional[str] = ...,
        source_type: _Optional[_Union[SourceType, str]] = ...,
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
