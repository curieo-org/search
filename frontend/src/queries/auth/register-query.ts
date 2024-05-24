import { BackendAPIClient } from '@/helpers/backend-api-client'
import { useAuthResponseStore } from '@/stores/auth/auth-response-store'
import { AuthParams, AuthResponse } from '@/types/auth'

export function useRegisterQuery() {
  const { setAuthResponseState } = useAuthResponseStore()

  function register(params: AuthParams): Promise<AuthResponse> {
    return new Promise(async function (resolve, reject) {
      const payload = new URLSearchParams()
      payload.append('email', params.username.trim())
      payload.append('username', params.username.trim())
      payload.append('password', params.password)
      BackendAPIClient.post('/auth/register', payload)
        .then(res => {
          setAuthResponseState('authResponse', res.data as AuthResponse)
          resolve(res.data as AuthResponse)
        })
        .catch(err => reject(err))
    })
  }

  return register
}
