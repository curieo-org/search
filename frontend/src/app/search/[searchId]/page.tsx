import SearchInput from '@/components/search/search-input'
import SearchResponse from '@/components/search/search-response'
import SearchTitle from '@/components/search/search-title'
import SourcesMenu from '@/components/search/sources-menu'
import { searchResults } from '@/develop/dummy-data/search-result'

export default function SearchResult() {
  const searchResult = searchResults[0]

  return (
    <div className="w-full flex">
      <div className="w-full flex flex-col justify-between">
        <div className="w-full p-20">
          <SearchTitle className="mb-6" title={searchResult.query} />
          <SearchResponse response={searchResult.result} />
        </div>
        <div className="w-full flex justify-center">
          <SearchInput />
        </div>
      </div>
      <SourcesMenu
        className="w-96 h-screen overflow-y-scroll -mb-5 bg-foreground-dark over p-2 pt-20"
        sources={searchResult.sources}
      />
    </div>
  )
}
