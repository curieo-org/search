import { create } from 'zustand'
import { devtools, persist } from 'zustand/middleware'

export type AuthPurpose = 'register' | 'login'

type AuthState = {
  purpose: AuthPurpose
}

type AuthStore = {
  state: AuthState
  setAuthState: <T extends keyof AuthState>(key: T, value: AuthState[T]) => void
  reset: () => void
}

const initialState: AuthState = {
  purpose: 'register',
}

export const useAuthStore = create<AuthStore>()(
  devtools(
    persist(
      set => ({
        state: initialState,

        setAuthState: (key, value) =>
          set(store => {
            return { state: { ...store.state, [key]: value } }
          }),

        reset: () =>
          set(() => {
            return { state: initialState }
          }),
      }),
      { name: 'auth-store' }
    )
  )
)
