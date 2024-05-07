import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import classNames from 'classnames'
import { HTMLAttributes } from 'react'
import HistoryButton from './history-button'
import NavmenuFooter from './navmenu-footer'
import NavmenuHeading from './navmenu-heading'
import NewSearchButton from './new-search-button'
import SearchHistoryNav from './search-history-nav'

type NavmenuProps = HTMLAttributes<HTMLDivElement>

export default function Navmenu(props: NavmenuProps) {
  const {
    state: { isNavmenuCollapsed, isHistoryCollapsed },
  } = useNavmenuStore()

  return (
    <div className="sticky top-0 w-full">
      <div className="h-screen w-full">
        <div
          className={classNames(
            `flex h-full w-full flex-col justify-between bg-background-dark/2 dark:bg-white/2 border-r border-background-dark/10 dark:border-white/10 rounded-r-2xl`,
            [isNavmenuCollapsed ? 'items-center' : 'items-start px-3 xl:px-4']
          )}
        >
          <div className="w-full flex flex-col">
            <NavmenuHeading />
            <NewSearchButton className="mb-2" />
            <HistoryButton className="mb-2" />
            {!isNavmenuCollapsed && !isHistoryCollapsed && <SearchHistoryNav />}
          </div>
          <NavmenuFooter className="sticky bottom-0 backdrop-blur-sm" />
        </div>
      </div>
    </div>
  )
}
