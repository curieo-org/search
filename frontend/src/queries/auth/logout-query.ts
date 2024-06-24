import { logoutMessage } from '@/constants/messages'
import { AxiosClient } from '@/helpers/axios-client'
import { LogoutResponse } from '@/types/auth'

export function useLogoutQuery() {
  function logout(): Promise<LogoutResponse> {
    return new Promise(async function (resolve, reject) {
      AxiosClient.get('/auth/logout')
        .then(res => resolve({ message: logoutMessage } as LogoutResponse))
        .catch(err => reject(err))
    })
  }

  return logout
}