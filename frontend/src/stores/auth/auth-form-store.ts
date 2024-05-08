import { create } from 'zustand'
import { devtools } from 'zustand/middleware'

type AuthFormState = {
  email: string
  password: string
}

type AuthFormStore = {
  state: AuthFormState
  setAuthFormState: <T extends keyof AuthFormState>(key: T, value: AuthFormState[T]) => void
  reset: () => void
}

const initialState: AuthFormState = {
  email: '',
  password: '',
}

export const useAuthFormStore = create<AuthFormStore>()(
  devtools(
    set => ({
      state: initialState,

      setAuthFormState: (key, value) =>
        set(store => {
          return { state: { ...store.state, [key]: value } }
        }),

      reset: () =>
        set(() => {
          return { state: initialState }
        }),
    }),
    { name: 'auth-form-store' }
  )
)
