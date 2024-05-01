import { HTMLAttributes } from 'react'
import SettingsInfoItem from './settings-info-item'
import { Button } from '../lib/button'

type AccountDetailsProps = HTMLAttributes<HTMLDivElement>

export default function AccountDetails(props: AccountDetailsProps) {
  return (
    <div className="flex flex-col border border-foreground-dark/20 dark:border-background-light/20 rounded w-2/5">
      <div className="bg-custom-black-ash px-4 py-1 h-8 text-base text-custom-ash">Account Details</div>
      <div className="flex flex-col divide-y divide-foreground-dark/20 dark:divide-background-light/20">
        <SettingsInfoItem label="Email" value="iana@curieo.org" />
        <SettingsInfoItem label="Password" value="************" />
        <Button className="bg-transparent hover:bg-transparent" label="Log out" />
      </div>
    </div>
  )
}
