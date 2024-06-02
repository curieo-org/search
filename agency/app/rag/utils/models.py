from abc import abstractmethod

from pydantic import BaseModel, ConfigDict

from app.grpc_types.agency_pb2 import Source
from app.rag.retrieval.pubmed.types import SearchResult as PubmedSearchResult
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


class PubmedSourceRecord(SourceModel, PubmedSearchResult):
    def to_grpc_source(self) -> Source:
        return Source(
            url=self.url,
            metadata={"title": self.title, "abstract": self.abstract},
        )


SourceRecord = BraveSourceRecord | PubmedSourceRecord


class RetrievedResult(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    text: str
    source: SourceRecord


class PromptCompressorResult(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    prompt: str
    sources: list[SourceRecord]


class SearchResultRecord(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    result: str
    sources: list[SourceRecord]
