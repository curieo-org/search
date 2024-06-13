from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Double2D(_message.Message):
    __slots__ = ("values",)
    VALUES_FIELD_NUMBER: _ClassVar[int]
    values: _containers.RepeatedScalarFieldContainer[float]
    def __init__(self, values: _Optional[_Iterable[float]] = ...) -> None: ...

class Int2D(_message.Message):
    __slots__ = ("values",)
    VALUES_FIELD_NUMBER: _ClassVar[int]
    values: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, values: _Optional[_Iterable[int]] = ...) -> None: ...

class Embeddings(_message.Message):
    __slots__ = ("dense_embedding", "sparse_embedding", "sparse_indices")
    DENSE_EMBEDDING_FIELD_NUMBER: _ClassVar[int]
    SPARSE_EMBEDDING_FIELD_NUMBER: _ClassVar[int]
    SPARSE_INDICES_FIELD_NUMBER: _ClassVar[int]
    dense_embedding: _containers.RepeatedScalarFieldContainer[float]
    sparse_embedding: _containers.RepeatedCompositeFieldContainer[Double2D]
    sparse_indices: _containers.RepeatedCompositeFieldContainer[Int2D]
    def __init__(self, dense_embedding: _Optional[_Iterable[float]] = ..., sparse_embedding: _Optional[_Iterable[_Union[Double2D, _Mapping]]] = ..., sparse_indices: _Optional[_Iterable[_Union[Int2D, _Mapping]]] = ...) -> None: ...

class SearchInput(_message.Message):
    __slots__ = ("query",)
    QUERY_FIELD_NUMBER: _ClassVar[int]
    query: str
    def __init__(self, query: _Optional[str] = ...) -> None: ...

class EmbeddingsOutput(_message.Message):
    __slots__ = ("status", "embeddings")
    STATUS_FIELD_NUMBER: _ClassVar[int]
    EMBEDDINGS_FIELD_NUMBER: _ClassVar[int]
    status: int
    embeddings: Embeddings
    def __init__(self, status: _Optional[int] = ..., embeddings: _Optional[_Union[Embeddings, _Mapping]] = ...) -> None: ...

class PubmedSource(_message.Message):
    __slots__ = ("pubmed_id", "title", "abstract", "embedding")
    PUBMED_ID_FIELD_NUMBER: _ClassVar[int]
    TITLE_FIELD_NUMBER: _ClassVar[int]
    ABSTRACT_FIELD_NUMBER: _ClassVar[int]
    EMBEDDING_FIELD_NUMBER: _ClassVar[int]
    pubmed_id: str
    title: str
    abstract: str
    embedding: Embeddings
    def __init__(self, pubmed_id: _Optional[str] = ..., title: _Optional[str] = ..., abstract: _Optional[str] = ..., embedding: _Optional[_Union[Embeddings, _Mapping]] = ...) -> None: ...

class PubmedResponse(_message.Message):
    __slots__ = ("status", "sources")
    STATUS_FIELD_NUMBER: _ClassVar[int]
    SOURCES_FIELD_NUMBER: _ClassVar[int]
    status: int
    sources: _containers.RepeatedCompositeFieldContainer[PubmedSource]
    def __init__(self, status: _Optional[int] = ..., sources: _Optional[_Iterable[_Union[PubmedSource, _Mapping]]] = ...) -> None: ...
