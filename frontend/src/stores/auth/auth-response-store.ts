import { AuthResponse } from '@/types/auth'
import { create } from 'zustand'
import { devtools, persist } from 'zustand/middleware'

type AuthResponseState = {
  authResponse: AuthResponse | null
}

type AuthResponseStore = {
  state: AuthResponseState
  setAuthResponseState: <T extends keyof AuthResponseState>(key: T, value: AuthResponseState[T]) => void
  reset: () => void
}

const initialState: AuthResponseState = {
  authResponse: null,
}

export const useAuthResponseStore = create<AuthResponseStore>()(
  devtools(
    persist(
      set => ({
        state: initialState,

        setAuthResponseState: (key, value) =>
          set(store => {
            return { state: { ...store.state, [key]: value } }
          }),

        reset: () =>
          set(() => {
            return { state: initialState }
          }),
      }),
      { name: 'auth-response-store' }
    )
  )
)
