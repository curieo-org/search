import abc
import pydantic
from typing import Optional

from app.grpc_types.agency_pb2 import Source


class AbstractSourceRecord(abc.ABC):
    @abc.abstractmethod
    def to_grpc_source(self) -> Source:
        raise NotImplementedError


class BraveSourceRecord(pydantic.BaseModel, AbstractSourceRecord):
    url: str
    page_age: Optional[str]

    def to_grpc_source(self) -> Source:
        return Source(url=self.url, metadata={"page_age": self.page_age or ""})


class PubmedSourceRecord(pydantic.BaseModel, AbstractSourceRecord):
    url: str
    helper_text: Optional[str]

    def to_grpc_source(self) -> Source:
        return Source(url=self.url, metadata={"helper_text": self.helper_text or ""})


SourceRecord = BraveSourceRecord | PubmedSourceRecord


class SearchResultRecord(pydantic.BaseModel):
    result: str
    sources: list[SourceRecord]
