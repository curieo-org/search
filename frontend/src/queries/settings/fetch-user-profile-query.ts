import { fetchUserProfile } from '@/actions/settings'
import { useQuery } from '@tanstack/react-query'

export const useFetchUserProfile = () =>
  useQuery({
    queryKey: ['user-profile'],
    async queryFn() {
      return await fetchUserProfile()
    },
  })
