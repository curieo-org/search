import { create } from 'zustand'
import { devtools, persist } from 'zustand/middleware'

type NavmenuState = {
  isNavmenuCollapsed: boolean
  isHistoryCollapsed: boolean
}

type NavmenuStore = {
  state: NavmenuState
  toggleNavmenuState: <T extends keyof NavmenuState>(key: T) => void
  reset: () => void
}

const initialState: NavmenuState = {
  isNavmenuCollapsed: false,
  isHistoryCollapsed: false,
}

export const useNavmenuStore = create<NavmenuStore>()(
  devtools(
    persist(
      set => ({
        state: initialState,

        toggleNavmenuState: key =>
          set(store => {
            const value = !store.state[key]
            return { state: { ...store.state, [key]: value } }
          }),

        reset: () =>
          set(() => {
            return { state: initialState }
          }),
      }),
      { name: 'navmenu-store' }
    )
  )
)
