import { ThreadByIdParams } from '@/types/search'
import { useQuery } from '@tanstack/react-query'
import { threadById } from '@/actions/search'

export const useFetchThreadByIdQuery = ({ threadId }: ThreadByIdParams) => {
  return useQuery({
    enabled: false,
    queryKey: ['thread-by-id', threadId],
    async queryFn() {
      return await threadById(threadId)
    },
  })
}
