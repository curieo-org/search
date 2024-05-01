import { appName } from '@/constants/app-contants'
import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import classNames from 'classnames'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import { Span } from '../lib/typography'

type NavmenuHeadingProps = HTMLAttributes<HTMLDivElement>

export default function NavmenuHeading(props: NavmenuHeadingProps) {
  const {
    state: { isNavmenuCollaped },
  } = useNavmenuStore()

  return (
    <div
      className={twMerge(
        classNames('flex items-center gap-x-2 my-8', {
          'justify-center': isNavmenuCollaped,
        }),
        props.className
      )}
    >
      <img src="/images/curieo-logo.svg" className="h-8 w-8" />
      {!isNavmenuCollaped && <Span className="text-xl font-semibold">{appName}</Span>}
    </div>
  )
}
