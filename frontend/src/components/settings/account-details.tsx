'use server'

import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import { Span } from '../lib/typography'
import SettingsInfoItem from './settings-info-item'
import { auth } from '@/auth'

type AccountDetailsProps = HTMLAttributes<HTMLDivElement>

export default async function AccountDetails(props: AccountDetailsProps) {
  const session = await auth()

  return (
    <div className={twMerge('flex flex-col w-full', props.className)}>
      <Span className="mb-4 text-custom-gray-100">Account</Span>
      <div className="mb-2 w-full h-px bg-custom-gray-200/25"></div>
      <div className="px-4 py-3 flex flex-col rounded-2xl bg-background-dark/3 dark:bg-background-light/3">
        <SettingsInfoItem label="Email" value={session?.user?.name ?? session?.user?.email ?? ''} />
        <SettingsInfoItem label="Password" value="************" />
      </div>
    </div>
  )
}
