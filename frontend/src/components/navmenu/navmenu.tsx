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
    state: { isNavmenuCollaped, isHistoryCollapsed },
  } = useNavmenuStore()

  return (
    <div className="sticky top-0 w-full">
      <div className="h-screen w-full">
        <div
          className={classNames(
            `flex h-full w-full flex-col justify-between bg-background-dark dark:bg-foreground-dark border-r border-foreground-dark/10 dark:border-background-dark/10 rounded-r-2xl`,
            [isNavmenuCollaped ? 'items-center' : 'items-start px-4']
          )}
        >
          <div className="w-full flex flex-col">
            <NavmenuHeading />
            <NewSearchButton className="mb-2" />
            <HistoryButton className="mb-2" />
            {!isNavmenuCollaped && !isHistoryCollapsed && <SearchHistoryNav />}
          </div>
          <NavmenuFooter />
        </div>
      </div>
    </div>
  )
}
