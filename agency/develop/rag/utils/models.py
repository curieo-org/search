from abc import abstractmethod

from pydantic import BaseModel, ConfigDict

from app.grpc_types.agency_pb2 import Source, SourceType
from app.pubmed_retrieval.types import SearchResult as PubmedSearchResult
from app.rag.retrieval.web.types import SearchResult as BraveSearchResult


class SourceModel(BaseModel):
    @abstractmethod
    def to_grpc_source(self) -> Source: ...


class BraveSourceRecord(SourceModel, BraveSearchResult):
    def to_grpc_source(self) -> Source:
        return Source(
            url=str(self.url),
            title=self.title,
            description=self.description,
            source_type=SourceType.Url,
            metadata={"page_age": self.page_age or ""},
        )


class PubmedSourceRecord(SourceModel, PubmedSearchResult):
    def to_grpc_source(self) -> Source:
        return Source(
            url=str(self.url),
            title=self.title,
            description=self.abstract,
            source_type=SourceType.Url,
            metadata={},
        )


SourceRecord = BraveSourceRecord | PubmedSourceRecord


class RetrievedResult(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    text: str
    source: SourceRecord


class PubmedSourceResult(BaseModel):
    pubmed_id: str
    title: str
    abstract: str


class PromptCompressorResult(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    prompt_list: list[str]
    sources: list[SourceRecord]


class SearchResultRecord(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    result: str
    sources: list[SourceRecord]
