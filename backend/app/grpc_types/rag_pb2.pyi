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

class RouteCategory(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    ClinicalTrials: _ClassVar[RouteCategory]
    DRUG: _ClassVar[RouteCategory]
    PUBMED_BIOXRIV_WEB: _ClassVar[RouteCategory]
    NOT_SELECTED: _ClassVar[RouteCategory]

ClinicalTrials: RouteCategory
DRUG: RouteCategory
PUBMED_BIOXRIV_WEB: RouteCategory
NOT_SELECTED: RouteCategory

class SearchRequest(_message.Message):
    __slots__ = ("query", "route_category")
    QUERY_FIELD_NUMBER: _ClassVar[int]
    ROUTE_CATEGORY_FIELD_NUMBER: _ClassVar[int]
    query: str
    route_category: RouteCategory
    def __init__(
        self,
        query: _Optional[str] = ...,
        route_category: _Optional[_Union[RouteCategory, str]] = ...,
    ) -> None: ...

class Metadata(_message.Message):
    __slots__ = ("key", "value")
    KEY_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    key: str
    value: str
    def __init__(
        self, key: _Optional[str] = ..., value: _Optional[str] = ...
    ) -> None: ...

class Source(_message.Message):
    __slots__ = ("url", "metadata")
    URL_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    url: str
    metadata: _containers.RepeatedCompositeFieldContainer[Metadata]
    def __init__(
        self,
        url: _Optional[str] = ...,
        metadata: _Optional[_Iterable[_Union[Metadata, _Mapping]]] = ...,
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
