import { AxiosClient } from '@/helpers/axios-client'
import { useAuthResponseStore } from '@/stores/auth/auth-response-store'
import { AuthParams, AuthResponse } from '@/types/auth'

export function useRegisterQuery() {
  const { setAuthResponseState } = useAuthResponseStore()

  function register(params: AuthParams): Promise<AuthResponse> {
    return new Promise(async function (resolve, reject) {
      AxiosClient.post('/auth/register', { ...params })
        .then(res => {
          setAuthResponseState('authResponse', res.data as AuthResponse)
          resolve(res.data as AuthResponse)
        })
        .catch(err => reject(err))
    })
  }

  return register
}
