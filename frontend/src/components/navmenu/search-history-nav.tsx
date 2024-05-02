import { useSearchHistoryFetchQuery } from '@/queries/search/search-history-fetch-query'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import SearchHistorySlab from './search-history-slab'

type SearchHistoryNavProps = HTMLAttributes<HTMLDivElement>

export default function SearchHistoryNav(props: SearchHistoryNavProps) {
  const { data: searchResults } = useSearchHistoryFetchQuery()

  if (!searchResults) {
    return null
  }

  return (
    <div className={twMerge('w-full flex flex-col gap-y-4 max-h-96 overflow-y-scroll', props.className)}>
      <SearchHistorySlab title="Today" searchHistoryList={searchResults.slice(0, 4)} />
      <SearchHistorySlab title="Yesterday" searchHistoryList={searchResults.slice(4, 8)} />
      <SearchHistorySlab title="7 Days Ago" searchHistoryList={searchResults.slice(8, 12)} />
    </div>
  )
}
