import { SearchResult } from '@/types/search'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import SearchHistoryButton from './search-history-button'

type SearchHistorySlabProps = HTMLAttributes<HTMLDivElement> & {
  title: string
  searchHistoryList: SearchResult[]
}

export default function SearchHistorySlab(props: SearchHistorySlabProps) {
  return (
    <div className={twMerge('w-full', props.className)}>
      <span className="text-input-placeholder text-2xs ml-6">{props.title}</span>
      <div className="flex flex-col mt-0.5">
        {props.searchHistoryList.map(searchResult => (
          <SearchHistoryButton
            key={`search-history-nav-${searchResult.search_history_id}`}
            searchResult={searchResult}
          />
        ))}
      </div>
    </div>
  )
}