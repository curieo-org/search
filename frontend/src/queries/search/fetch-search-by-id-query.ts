import { BackendAPIClient } from '@/utils/backend-api-client'
import { SearchByIdParams, SearchResult } from '@/types/search'
import { useQuery } from '@tanstack/react-query'

export const useFetchSearchByIdQuery = ({ searchHistoryId }: SearchByIdParams) => {
  return useQuery({
    queryKey: ['search-by-id', searchHistoryId],
    async queryFn() {
      const params = { search_history_id: searchHistoryId }
      const { data } = await BackendAPIClient.get('/search/one', { params })
      return data as SearchResult
    },
  })
}
