import { AxiosClient } from '@/helpers/axios-client'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { SearchReactionBody, SearchResult } from '@/types/search'

export const useSearchReactionQuery = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationKey: ['search-reaction'],
    async mutationFn(payload: SearchReactionBody) {
      const { data } = await AxiosClient.patch(`/search/reaction`, payload)
      return data as SearchResult
    },
    onSuccess: data => {
      queryClient.setQueryData(['search-by-id', data.search_history_id], data)
      queryClient.invalidateQueries({ queryKey: ['search-history'] })
    },
  })
}
