import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import classNames from 'classnames'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import ShiftLeft from '../icons/shift-left'
import ShiftRight from '../icons/shift-right'
import { Span } from '../lib/typography'
import { useRouter } from 'next/navigation'
import EngineIcon from '../icons/engine'

type NavmenuFooterProps = HTMLAttributes<HTMLDivElement>

export default function NavmenuFooter(props: NavmenuFooterProps) {
  const router = useRouter()
  const {
    state: { isNavmenuCollaped },
    toggleNavmenuState,
  } = useNavmenuStore()

  const toggleNavmenuCollaped = () => {
    toggleNavmenuState('isNavmenuCollaped')
  }

  const handleNavigateToSettingsPage = () => {
    router.push('/settings')
  }

  return (
    <div
      className={twMerge(
        classNames('w-full flex flex-col mb-4', {
          'items-center': isNavmenuCollaped,
        }),
        props.className
      )}
    >
      {isNavmenuCollaped ? (
        <ShiftRight size={35} className="cursor-pointer" onClick={toggleNavmenuCollaped} />
      ) : (
        <div>
          <ShiftLeft size={35} className="float-right mr-0 cursor-pointer" onClick={toggleNavmenuCollaped} />
        </div>
      )}
      <div className="my-2 h-px w-full bg-custom-gray/25"></div>
      <div className="relative flex items-center gap-x-2 cursor-pointer" onClick={handleNavigateToSettingsPage}>
        <img src="/images/sample-user.png" className="h-10 w-auto" />
        {!isNavmenuCollaped && <Span className="font-medium">Musharof</Span>}
        {!isNavmenuCollaped && <EngineIcon size={18} className="absolute right-2" />}
      </div>
    </div>
  )
}
