import { SearchByIdParams } from '@/types/search'
import { useQuery } from '@tanstack/react-query'
import { searchById } from '@/actions/search'

export const useFetchSearchByIdQuery = ({ searchId }: SearchByIdParams) => {
  return useQuery({
    queryKey: ['search-by-id', searchId],
    async queryFn() {
      return await searchById(searchId)
    },
  })
}
