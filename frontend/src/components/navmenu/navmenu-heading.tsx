import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import classNames from 'classnames'
import { HTMLAttributes, useState } from 'react'
import { twMerge } from 'tailwind-merge'
import { Span } from '../lib/typography'

type NavmenuHeadingProps = HTMLAttributes<HTMLDivElement>

export default function NavmenuHeading(props: NavmenuHeadingProps) {
  const [mouseEntered, setMouseEntered] = useState(false)
  const [mouseLeft, setMouseLeft] = useState(false)
  const {
    state: { isNavmenuCollapsed },
  } = useNavmenuStore()

  return (
    <div
      className={twMerge(
        classNames('flex items-center gap-x-2 my-6 xl:my-8', {
          'justify-center': isNavmenuCollapsed,
        }),
        props.className
      )}
    >
      <img
        src="/images/curieo-logo.svg"
        className={classNames('h-10 w-auto', {
          'animate-spin-once': mouseEntered,
          'animate-spin-once-reverse': mouseLeft,
        })}
        onMouseEnter={() => {
          setMouseLeft(false)
          setMouseEntered(true)
        }}
        onMouseLeave={() => {
          setMouseEntered(false)
          setMouseLeft(true)
        }}
      />
      {!isNavmenuCollapsed && <Span className="text-xl font-semibold">Curieo</Span>}
    </div>
  )
}
