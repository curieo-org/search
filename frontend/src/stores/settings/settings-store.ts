import { UserProfile, emptyUser } from '@/types/settings'
import { create } from 'zustand'
import { devtools } from 'zustand/middleware'

export type SettingsTab = 'profile' | 'security'

type SettingsState = {
  activeTab: SettingsTab
  currentUser: UserProfile
  editedUser: UserProfile
  isEdited: boolean
}

type SettingsStore = {
  state: SettingsState
  setActiveTab: (tab: SettingsTab) => void
  setEditedUserInfo: <T extends keyof UserProfile>(key: T, value: string) => void
  setCurrentUser: (user: UserProfile) => void
  reset: () => void
}

const initialState: SettingsState = {
  activeTab: 'profile',
  currentUser: emptyUser,
  editedUser: emptyUser,
  isEdited: false,
}

export const useSettingsStore = create<SettingsStore>()(
  devtools(
    set => ({
      state: initialState,

      setActiveTab: tab =>
        set(store => {
          return { state: { ...store.state, activeTab: tab } }
        }),

      setEditedUserInfo: (key, value) =>
        set(store => {
          const editedUser = { ...store.state.editedUser, [key]: value }

          let isEdited = false
          Object.keys(emptyUser).forEach(key => {
            const key_ = key as keyof UserProfile
            if (editedUser[key_] !== store.state.currentUser[key_]) {
              isEdited = true
            }
          })

          return { state: { ...store.state, editedUser, isEdited } }
        }),

      setCurrentUser: user =>
        set(store => {
          return { state: { ...store.state, currentUser: user, editedUser: user, isEdited: false } }
        }),

      reset: () =>
        set(() => {
          return { state: initialState }
        }),
    }),
    { name: 'settings-store' }
  )
)
