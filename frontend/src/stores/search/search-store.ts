'use client'

import { create } from 'zustand'
import { devtools } from 'zustand/middleware'

type SearchState = {
  searchQuery: string
}

type SearchStore = {
  state: SearchState
  setSearchState: <T extends keyof SearchState>(key: T, value: SearchState[T]) => void
  reset: () => void
}

const initialState: SearchState = {
  searchQuery: '',
}

export const useSearchStore = create<SearchStore>()(
  devtools(
    set => ({
      state: initialState,

      setSearchState: (key, value) =>
        set(store => {
          return { state: { ...store.state, [key]: value } }
        }),

      reset: () =>
        set(() => {
          return { state: initialState }
        }),
    }),
    { name: 'search-store' }
  )
)
