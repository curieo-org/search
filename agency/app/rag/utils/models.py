import abc

from pydantic import BaseModel

from app.grpc_types.agency_pb2 import Source


class AbstractSourceRecord(abc.ABC):
    @abc.abstractmethod
    def to_grpc_source(self) -> Source:
        raise NotImplementedError


class BraveSourceRecord(BaseModel, AbstractSourceRecord):
    url: str
    page_age: str

    def __init__(self, **data):
        url = data.get("url")
        page_age = data.get("page_age", "")
        super().__init__(url=url, page_age=page_age)

    def to_grpc_source(self) -> Source:
        return Source(url=self.url, metadata={"page_age": self.page_age})


class PubmedSourceRecord(BaseModel, AbstractSourceRecord):
    url: str
    helper_text: str

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
