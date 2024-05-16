'use client'

import { useLogoutQuery } from '@/queries/auth/logout-query'
import { useRouter } from 'next/navigation'
import { HTMLAttributes } from 'react'
import { toast } from 'react-toastify'
import { twMerge } from 'tailwind-merge'
import { Button } from '../lib/button'
import { useAppContext } from '../wrappers/app'

type LogoutSectionProps = HTMLAttributes<HTMLDivElement>

export default function LogoutSection(props: LogoutSectionProps) {
  const router = useRouter()
  const logout = useLogoutQuery()
  const { updateAuthStatus } = useAppContext()

  const handleLogout = async () => {
    try {
      await logout()
      updateAuthStatus('unauthenticated')
    } catch (err) {
      toast.error('Failed to log out. Please try again layer...')
    }
  }

  return (
    <div className={twMerge('w-full flex flex-col items-center', props.className)}>
      <div className="mb-2 w-full h-px bg-custom-gray-200/25"></div>
      <Button
        className="w-full bg-transparent hover:bg-white/2 text-custom-gray-100"
        label="Log out"
        onClick={handleLogout}
      />
    </div>
  )
}
