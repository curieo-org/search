'use client'

import SearchInput from '@/components/search/search-input'
import SearchResponse from '@/components/search/search-response'
import SearchTitle from '@/components/search/search-title'
import SourcesMenu from '@/components/search/sources-menu'
import SearchResultPageSkeleton from '@/components/skeletons/search-result-page-skeleton'
import ErrorPage from '@/components/util/error-page'
import { useFetchSearchByIdQuery } from '@/queries/search/fetch-search-by-id-query'
import { useSearchQuery } from '@/queries/search/search-query'
import { useSearchStore } from '@/stores/search/search-store'
import { SearchByIdParams } from '@/types/search'
import { useParams, useRouter } from 'next/navigation'
import { useEffect } from 'react'

export default function SearchResult() {
  const router = useRouter()
  const { reset } = useSearchStore()
  const { searchHistoryId } = useParams()

  const {
    data: searchResult,
    isLoading: isPageDataLoading,
    isError: isErrorInPageData,
  } = useFetchSearchByIdQuery({
    searchHistoryId,
  } as SearchByIdParams)

  const {
    data: newSearchResult,
    isLoading: isNewSearchResultLoading,
    isSuccess: isNewSearchSuccess,
    refetch: fetchNewSearchResult,
  } = useSearchQuery()

  const handleSearch = () => {
    fetchNewSearchResult()
  }

  useEffect(() => {
    if (isNewSearchSuccess) {
      reset()
      router.push(`/search/${newSearchResult.search_history_id}`)
    }
  }, [isNewSearchSuccess])

  if (isNewSearchResultLoading || isPageDataLoading) {
    return <SearchResultPageSkeleton />
  } else if (isErrorInPageData || !searchResult) {
    return <ErrorPage message="Failed to fetch search result. Please try again later..." />
  } else {
    return (
      <div className="w-full flex">
        <div className="w-full flex flex-col justify-between">
          <div className="w-full px-6 py-10 lg:px-12 xl:px-20 xl:py-20 mx-auto xl:max-w-[880px] transition-all duration-300">
            <SearchTitle className="mb-6" title={searchResult.query} />
            <SearchResponse response={searchResult.result} />
          </div>
          <div className="sticky bottom-0 pb-4 px-4 -mb-4 w-full flex justify-center backdrop-blur-sm">
            <SearchInput handleSearch={handleSearch} />
          </div>
        </div>
        <SourcesMenu
          className="w-60 xl:w-96 max-h-screen overflow-y-scroll -mb-4 over py-2 pr-2 mr-1 pt-10 xl:pt-20 transition-all duration-300"
          sources={searchResult.sources}
        />
      </div>
    )
  }
}
