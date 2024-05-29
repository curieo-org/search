from pydantic import BaseModel, Field, HttpUrl


class SearchResult(BaseModel):
    """Represents a pubmed search result, relevant to the query"""

    type: str = "pubmed_search_result"

    title: str = Field(description="The title of the pubmed web page.")
    url: HttpUrl = Field(description="The URL where the page is served.")
    abstract: str = Field(description="An abstract for the pubmed record.")
