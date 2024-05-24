import { BackendAPIClient } from '@/helpers/backend-api-client'
import { SearchResult } from '@/types/search'
import { useInfiniteQuery } from '@tanstack/react-query'
import _ from 'lodash'

export const useFetchSearchHistoryQuery = () => {
  return useInfiniteQuery({
    queryKey: ['search-history'],
    initialPageParam: { limit: 10, offset: 0 },
    getNextPageParam: (lastPage, pages) => {
      const hasNextPage = (pages[pages.length - 1] as SearchResult[]).length !== 0
      return hasNextPage ? { offset: _.flatten(pages).length, limit: 10 } : undefined
    },
    async queryFn({ pageParam }) {
      const { data } = await BackendAPIClient.get(
        `/search/history?limit=${pageParam.limit ?? 10}${pageParam.offset ? `&offset=${pageParam.offset}` : ``}`
      )
      return data as SearchResult[]
    },
  })
}
