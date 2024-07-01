'use client'

import { HTMLAttributes } from 'react'
import { Button } from '../lib/button'
import { useSettingsStore } from '@/stores/settings/settings-store'
import classNames from 'classnames'
import { twMerge } from 'tailwind-merge'

type SettingsNavmenuProps = HTMLAttributes<HTMLDivElement> & {}

export default function SettingsNavmenu(props: SettingsNavmenuProps) {
  const {
    state: { activeTab },
    setActiveTab,
  } = useSettingsStore()

  return (
    <div className={twMerge('flex flex-col gap-y-5', props.className)}>
      <Button
        className={classNames('w-52 h-10 rounded-2.5xl bg-transparent hover:bg-white/5 border', {
          'border-white/10': activeTab === 'security',
          'border-primary-dark': activeTab === 'profile',
        })}
        label="Profile"
        onClick={() => setActiveTab('profile')}
      />
      <Button
        className={classNames('w-52 h-10 rounded-2.5xl bg-transparent hover:bg-white/5 border', {
          'border-primary-dark': activeTab === 'security',
          'border-white/10': activeTab === 'profile',
        })}
        label="Security"
        onClick={() => setActiveTab('security')}
      />
    </div>
  )
}
