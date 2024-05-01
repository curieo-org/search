import { HTMLAttributes } from 'react'
import SettingsInfoItem from './settings-info-item'

type ProfileInfoProps = HTMLAttributes<HTMLDivElement>

export default function ProfileInfo(props: ProfileInfoProps) {
  return (
    <div className="flex flex-col border border-foreground-dark/20 dark:border-background-light/20 rounded w-2/5">
      <div className="bg-custom-black-ash px-4 py-1 h-8 text-base text-custom-ash">Profile</div>
      <div className="flex flex-col divide-y divide-foreground-dark/20 dark:divide-background-light/20">
        <SettingsInfoItem label="Full name" value="Iana Ivashkina" />
        <SettingsInfoItem label="Title" value="UX Designer" />
        <SettingsInfoItem label="Company" value="Curieo" />
      </div>
    </div>
  )
}
