import { create } from 'zustand'
import { devtools } from 'zustand/middleware'

export type SettingsTab = 'profile' | 'security'

type SettingsState = {
  activeTab: SettingsTab
}

type SettingsStore = {
  state: SettingsState
  setActiveTab: (key: T) => void
  reset: () => void
}

const initialState: SettingsState = {
  activeTab: 'profile',
}

export const useSettingsStore = create<SettingsStore>()(
  devtools(
    set => ({
      state: initialState,

      toggleSettingsState: key =>
        set(store => {
          const value = !store.state[key]
          return { state: { ...store.state, [key]: value } }
        }),

      reset: () =>
        set(() => {
          return { state: initialState }
        }),
    }),
    { name: 'settings-store' }
  )
)
