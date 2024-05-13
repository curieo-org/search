import abc
from typing import Optional

from pydantic import BaseModel

from app.grpc_types.agency_pb2 import Source


class AbstractSourceRecord(abc.ABC):
    @abc.abstractmethod
    def to_grpc_source(self) -> Source:
        raise NotImplementedError


class BraveSourceRecord(BaseModel, AbstractSourceRecord):
    url: str
    page_age: Optional[str]
    title: Optional[str]
    description: Optional[str]

    def __init__(self, **data):
        url = data.get("url")
        page_age = data.get("page_age", "")
        title = data.get("title", "")
        description = data.get("description", "")
        super().__init__(url=url, page_age=page_age, title=title, description=description)

    def to_grpc_source(self) -> Source:
        return Source(url=self.url, metadata={"page_age": self.page_age, "title": self.title, "description": self.description})


class PubmedSourceRecord(BaseModel, AbstractSourceRecord):
    url: str
    helper_text: Optional[str]

    def __init__(self, **data):
        url = data.get("url")
        helper_text = data.get("helper_text", "")
        super().__init__(url=url, helper_text=helper_text)

    def to_grpc_source(self) -> Source:
        return Source(url=self.url, metadata={"helper_text": self.helper_text})


SourceRecord = BraveSourceRecord | PubmedSourceRecord


class SearchResultRecord(BaseModel):
    result: str
    sources: list[SourceRecord]
