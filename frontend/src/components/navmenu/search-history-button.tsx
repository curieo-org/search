import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import { SearchResult } from '@/types/search'
import classNames from 'classnames'
import { usePathname, useRouter } from 'next/navigation'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import { Button } from '../lib/button'

type SearchHistoryButtonProps = HTMLAttributes<HTMLButtonElement> & {
  searchResult: SearchResult
}

export default function SearchHistoryButton(props: SearchHistoryButtonProps) {
  const router = useRouter()
  const pathname = usePathname()

  const {
    state: { isNavmenuCollapsed },
  } = useNavmenuStore()

  const searchResultPagePath = `/search/${props.searchResult.search_history_id}`

  const handleNavigateToSearchResultPage = () => {
    router.push(searchResultPagePath)
  }

  const isActive = pathname === searchResultPagePath

  const { className, searchResult, ...rest } = props

  return (
    <Button
      className={twMerge(
        classNames(
          'w-full h-auto rounded-md text-xs font-light hover:bg-white/3 text-typography-light dark:text-typography-dark/80',
          [
            isNavmenuCollapsed ? 'justify-center' : 'justify-start text-start pl-5 xl:pl-6',
            isActive ? 'bg-white/2' : 'bg-transparent',
          ]
        ),
        className
      )}
      label={searchResult.query}
      onClick={handleNavigateToSearchResultPage}
      {...rest}
    />
  )
}
