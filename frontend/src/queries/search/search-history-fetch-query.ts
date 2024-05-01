import { searchResults } from '@/develop/dummy-data/search-result'
import { useQuery } from '@tanstack/react-query'

export const useSearchHistoryFetchQuery = () =>
  useQuery({
    queryKey: ['search-history-fetch'],
    async queryFn() {
      return searchResults
    },
  })
