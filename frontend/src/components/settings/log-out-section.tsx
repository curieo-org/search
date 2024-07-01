import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import { Button } from '../lib/button'
import { signOut } from '@/auth'

type LogoutSectionProps = HTMLAttributes<HTMLDivElement>

export default function LogoutSection(props: LogoutSectionProps) {
  return (
    <div className={twMerge('w-full flex flex-col items-center', props.className)}>
      <div className="mb-2 w-full h-px bg-custom-gray-200/25"></div>
      <form
        action={async () => {
          'use server'
          await signOut()
        }}
      >
        <Button
          className="w-full bg-transparent hover:bg-white/2 text-custom-gray-100"
          label="Sign out"
          type="submit"
        />
      </form>
    </div>
  )
}
