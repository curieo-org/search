import { searchReaction } from '@/actions/search'
import { SearchReactionBody } from '@/types/search'
import { useMutation } from '@tanstack/react-query'

export const useSearchReactionMutation = (setReaction: (reaction: boolean) => void) => {
  return useMutation({
    mutationKey: ['search-reaction'],
    async mutationFn(payload: SearchReactionBody) {
      return await searchReaction(payload)
    },
    onSuccess: async (data, payload) => {
      setReaction(payload.reaction)
    },
  })
}
