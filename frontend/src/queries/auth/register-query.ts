import { BackendAPIClient } from '@/utils/backend-api-client'
import { useAuthResponseStore } from '@/stores/auth/auth-response-store'
import { AuthParams, AuthResponse } from '@/types/auth'
import { encodeAsUrlSearchParams } from '@/utils'

export function useRegisterQuery() {
  const { setAuthResponseState } = useAuthResponseStore()

  function register(p: AuthParams): Promise<AuthResponse> {
    return new Promise(async function (resolve, reject) {
      BackendAPIClient.post(
        '/auth/register',
        encodeAsUrlSearchParams({
          email: p.username.trim(),
          username: p.username.trim(),
          password: p.password,
        })
      )
        .then(res => {
          setAuthResponseState('authResponse', res.data as AuthResponse)
          resolve(res.data as AuthResponse)
        })
        .catch(err => reject(err))
    })
  }

  return register
}
