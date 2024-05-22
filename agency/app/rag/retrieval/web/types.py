from pydantic import BaseModel, Field, HttpUrl


class Result(BaseModel):
    """Represents a brave search result.

    https://api.search.brave.com/app/documentation/web-search/responses#Result
    """

    title: str = Field(description="The title of the web page.")
    url: HttpUrl = Field(description="The URL where the page is served.")
    description: str = Field(description="A description for the web page.")
    page_age: str | None = Field(
        default=None,
        description="A date representing the age of the web page.",
    )
    language: str | None = Field(
        default=None,
        description="A language classification for the web page.",
    )


class SearchResult(Result):
    """Aggregated information on a brave search result, relevant to the query.

    https://api.search.brave.com/app/documentation/web-search/responses#SearchResult
    """

    type: str = "search_result"

    subtype: str = Field(
        default="generic",
        description="A sub type identifying the web search result type.",
    )
    age: str | None = Field(
        default=None,
        description="A string representing the age of the web search result.",
    )
    language: str = Field(description="The main language on the web search result.")

    extra_snippets: list[str] | None = Field(
        default=None,
        description="A list of extra alternate snippets for the web page.",
    )

    def get_extra_snippets(self) -> list[str]:
        return self.extra_snippets or []


class Search(BaseModel):
    """Collection of web search results.

    https://api.search.brave.com/app/documentation/web-search/responses#Search
    """

    type: str = "search"
    results: list[SearchResult] = Field(description="A list of web search results.")


class WebSearchApiResponse(BaseModel):
    """Brave Search API response object."""

    web: Search | None = Field(
        default=None,
        description="Web search results relevant to the query.",
    )

    def web_results(self) -> list[SearchResult]:
        return self.web.results if self.web else []
