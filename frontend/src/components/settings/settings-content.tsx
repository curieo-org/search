'use client'

import { useFetchUserProfile } from '@/queries/settings/fetch-user-profile-query'
import { useSettingsStore } from '@/stores/settings/settings-store'
import { HTMLAttributes, useEffect } from 'react'
import { twMerge } from 'tailwind-merge'
import ProfileSettings from './profile-settings'
import SecuritySettings from './security-settings'

type SettingsContentProps = HTMLAttributes<HTMLDivElement> & {}

export default function SettingsContent(props: SettingsContentProps) {
  const { data: currentUser } = useFetchUserProfile()
  const {
    state: { activeTab },
    setCurrentUser,
  } = useSettingsStore()

  useEffect(() => {
    if (currentUser) {
      setCurrentUser(currentUser)
    }
  }, [currentUser])

  return (
    <div className={twMerge('max-w-[564px] bg-white/2 rounded-2.5xl p-5', props.className)}>
      {activeTab === 'profile' ? <ProfileSettings /> : activeTab === 'security' ? <SecuritySettings /> : null}
    </div>
  )
}
