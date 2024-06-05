'use client'

import classNames from 'classnames'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import EngineIcon from '../icons/engine'
import ShiftLeft from '../icons/shift-left'
import ShiftRight from '../icons/shift-right'
import { Span } from '../lib/typography'
import { useSession } from 'next-auth/react'
import { useRouter } from 'next/navigation'
import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'

type NavmenuFooterProps = HTMLAttributes<HTMLDivElement>

export default function NavmenuFooter(props: NavmenuFooterProps) {
  const router = useRouter()
  const {
    state: { isNavmenuCollapsed },
    toggleNavmenuState,
  } = useNavmenuStore()
  const { data: session } = useSession()

  const toggleNavmenuCollaped = () => {
    toggleNavmenuState('isNavmenuCollapsed')
  }

  const handleNavigateToSettingsPage = () => {
    router.push('/settings')
  }

  return (
    <div
      className={twMerge(
        classNames('w-full flex flex-col mb-4', {
          'items-center': isNavmenuCollapsed,
        }),
        props.className
      )}
    >
      {isNavmenuCollapsed ? (
        <ShiftRight size={35} className="cursor-pointer" onClick={toggleNavmenuCollaped} />
      ) : (
        <div>
          <ShiftLeft size={35} className="float-right mr-0 cursor-pointer" onClick={toggleNavmenuCollaped} />
        </div>
      )}
      <div className="my-2 h-px w-full bg-custom-gray-200/25"></div>
      <div className="relative flex items-center gap-x-2 cursor-pointer" onClick={handleNavigateToSettingsPage}>
        <img
          src={session?.user?.image ? session?.user?.image : '/images/placeholder-user.png'}
          className="h-8 lg:h-10 w-auto"
          alt="user image"
        />
        {!isNavmenuCollapsed && (
          <Span className="text-xs lg:text-sm xl:text-base font-medium">{session?.user?.name}</Span>
        )}
        {!isNavmenuCollapsed && <EngineIcon size={18} className="absolute right-2" />}
      </div>
    </div>
  )
}
