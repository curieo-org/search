'use client'

import { LayoutProps } from '@/app/layout'
import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import classNames from 'classnames'
import Navmenu from '../navmenu/navmenu'

export default function PageWithNavMenu({ children }: LayoutProps) {
  const {
    state: { isNavmenuCollaped },
  } = useNavmenuStore()

  return (
    <div className="relative flex">
      <div className={classNames(['hidden md:block', isNavmenuCollaped ? 'min-w-20' : 'min-w-72'])}>
        <Navmenu />
      </div>

      <div className="h-full w-full mb-5">{children}</div>
    </div>
  )
}
