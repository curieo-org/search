import { useAppContext } from '@/components/wrappers/app'
import { AxiosClient } from '@/helpers/axios-client'
import { useSearchStore } from '@/stores/search/search-store'
import { SearchResult } from '@/types/search'
import { useQuery, useQueryClient } from '@tanstack/react-query'

export const useSearchQuery = () => {
  const queryClient = useQueryClient()
  const { sessionId } = useAppContext()
  const {
    state: { searchQuery },
  } = useSearchStore()

  return useQuery({
    enabled: false, // otherwise it will make the api call each time a character of the searchQuery changes
    queryKey: ['search', searchQuery, sessionId],
    async queryFn() {
      const params = { query: searchQuery.trim(), session_id: sessionId }
      const { data } = await AxiosClient.get('/search', { params })
      queryClient.setQueryData(['search-by-id', data.search_history_id], data)
      queryClient.invalidateQueries({ queryKey: ['search-history'] })
      return data as SearchResult
    },
  })
}
