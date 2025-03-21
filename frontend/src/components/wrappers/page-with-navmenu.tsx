'use client'

import { LayoutProps } from '@/app/layout'
import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import classNames from 'classnames'
import Navmenu from '../navmenu/navmenu'

export default function PageWithNavMenu({ children }: LayoutProps) {
  const {
    state: { isNavmenuCollapsed },
  } = useNavmenuStore()

  return (
    <div className="relative flex">
      <div
        className={classNames([
          'hidden md:block transition-all duration-300',
          isNavmenuCollapsed ? 'w-20' : 'w-60 lg:w-72 xl:w-90',
        ])}
      >
        <Navmenu />
      </div>

      <div className="w-full pb-4">{children}</div>
    </div>
  )
}
