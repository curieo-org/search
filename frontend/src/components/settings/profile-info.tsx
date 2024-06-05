import { fetchUserProfile } from '@/queries/settings/fetch-user-profile-query'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import { Span } from '../lib/typography'
import SettingsInfoItem from './settings-info-item'

type ProfileInfoProps = HTMLAttributes<HTMLDivElement>

export default async function ProfileInfo(props: ProfileInfoProps) {
  const userProfile = await fetchUserProfile()
  return (
    <div className={twMerge('flex flex-col w-full', props.className)}>
      <Span className="mb-4 text-custom-gray-100">Profile</Span>
      <div className="mb-2 w-full h-px bg-custom-gray-200/25"></div>
      <div className="px-4 py-3 flex flex-col rounded-2xl bg-background-dark/3 dark:bg-background-light/3">
        <SettingsInfoItem label="Full name" value={userProfile?.name ?? ''} />
        <SettingsInfoItem label="Title" value={userProfile?.title ?? ''} />
        <SettingsInfoItem label="Company" value={userProfile?.company ?? ''} />
      </div>
    </div>
  )
}
