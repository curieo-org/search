import { BackendAPIClient } from '@/helpers/backend-api-client'
import { UserProfile } from '@/types/settings'
import { useQuery } from '@tanstack/react-query'

export function fetchUserProfile(): Promise<UserProfile> {
  return new Promise(async function (resolve, reject) {
    BackendAPIClient.get('/users/me')
      .then(res => {
        resolve(res.data as UserProfile)
      })
      .catch(err => reject(err))
  })
}

export const useFetchUserProfile = () => {
  return useQuery({
    queryKey: ['user-profile'],
    queryFn: fetchUserProfile,
  })
}
