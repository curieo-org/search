'use client'

import { useSearchStore } from '@/stores/search/search-store'
import { useQuery, useQueryClient } from '@tanstack/react-query'
import { search } from '@/actions/search'

export const useSearchQuery = () => {
  const queryClient = useQueryClient()
  const {
    state: { searchQuery },
  } = useSearchStore()
  return useQuery({
    enabled: false,
    queryKey: ['search', searchQuery],
    queryFn: async () => {
      const result = await search(searchQuery.trim())
      queryClient.setQueryData(['search-by-id', result.search_history_id], result)
      await queryClient.invalidateQueries({ queryKey: ['search-history'] })
      return result
    },
  })
}
