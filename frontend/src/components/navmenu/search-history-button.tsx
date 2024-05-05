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
    state: { isNavmenuCollaped },
  } = useNavmenuStore()

  const searchResultPagePath = `/search/${props.searchResult.search_history_id}`

  const handleNavigateToSearchResultPage = () => {
    router.push(searchResultPagePath)
  }

  const isActive = pathname === searchResultPagePath

  return (
    <Button
      className={twMerge(
        classNames(
          'w-full h-auto rounded-md font-normal text-xs hover:bg-white/3 text-typography-light dark:text-typography-dark',
          [
            isNavmenuCollaped ? 'justify-center' : 'justify-start text-start pl-5 xl:pl-6',
            isActive ? 'bg-white/2' : 'bg-transparent',
          ]
        ),
        props.className
      )}
      label={props.searchResult.query}
      onClick={handleNavigateToSearchResultPage}
    />
  )
}
