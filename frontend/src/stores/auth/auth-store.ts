import { create } from 'zustand'
import { devtools } from 'zustand/middleware'

export type AuthPurpose = 'register' | 'login'

type AuthState = {
  purpose: AuthPurpose
  email: string
  password: string
}

type AuthStore = {
  state: AuthState
  setAuthState: <T extends keyof AuthState>(key: T, value: AuthState[T]) => void
  reset: () => void
}

const initialState: AuthState = {
  purpose: 'register',
  email: '',
  password: '',
}

export const useAuthStore = create<AuthStore>()(
  devtools(
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
