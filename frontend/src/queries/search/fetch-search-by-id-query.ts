import { SearchByIdParams } from '@/types/search'
import { useQuery } from '@tanstack/react-query'
import { searchById } from '@/actions/search'

export const useFetchSearchByIdQuery = ({ searchHistoryId }: SearchByIdParams) => {
  return useQuery({
    queryKey: ['search-by-id', searchHistoryId],
    async queryFn() {
      return await searchById(searchHistoryId)
    },
  })
}
