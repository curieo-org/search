import { useMutation, useQueryClient } from '@tanstack/react-query'
import { SearchReactionBody } from '@/types/search'
import { searchReaction } from '@/actions/search'

export const useSearchReactionMutation = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationKey: ['search-reaction'],
    async mutationFn(payload: SearchReactionBody) {
      return await searchReaction(payload)
    },
    onSuccess: async data => {
      queryClient.setQueryData(['search-by-id', data.search_history_id], data)
      await queryClient.invalidateQueries({ queryKey: ['search-history'] })
    },
  })
}
