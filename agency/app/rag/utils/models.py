from abc import abstractmethod

from pydantic import BaseModel, ConfigDict

from app.grpc_types.agency_pb2 import Source
from app.rag.retrieval.web.types import SearchResult as BraveSearchResult


class SourceModel(BaseModel):
    @abstractmethod
    def to_grpc_source(self) -> Source: ...


class BraveSourceRecord(SourceModel, BraveSearchResult):
    def to_grpc_source(self) -> Source:
        return Source(
            url=str(self.url),
            metadata={
                "title": self.title,
                "description": self.description,
                "page_age": self.page_age or "",
            },
        )


class PubmedSourceRecord(SourceModel):
    url: str
    helper_text: str = ""

    def to_grpc_source(self) -> Source:
        return Source(url=self.url, metadata={"helper_text": self.helper_text})


SourceRecord = BraveSourceRecord | PubmedSourceRecord


class SearchResultRecord(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    result: str
    sources: list[SourceRecord]
