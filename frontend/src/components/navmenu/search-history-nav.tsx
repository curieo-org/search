import { useFetchSearchHistoryQuery } from '@/queries/search/fetch-search-history-query'
import { SearchHistoryResponse, SearchResult, Thread } from '@/types/search'
import classNames from 'classnames'
import { startOfDay, sub } from 'date-fns'
import _ from 'lodash'
import { HTMLAttributes, UIEvent, useState } from 'react'
import { twMerge } from 'tailwind-merge'
import SearchHistorySlab from './search-history-slab'

type SearchHistoryNavProps = HTMLAttributes<HTMLDivElement>

export default function SearchHistoryNav(props: SearchHistoryNavProps) {
  const { data, isFetching, fetchNextPage } = useFetchSearchHistoryQuery()
  const [showScrollbar, setShowScrollbar] = useState(false)

  if (!data?.pages[0]) {
    return null
  }

  const searchHistory = _.flatten(data.pages.map(history => (history as SearchHistoryResponse).threads)) as Thread[]

  const startOfToday = startOfDay(new Date())
  const todayData = searchHistory.filter(searchResult => new Date(searchResult.created_at) >= startOfToday)
  const yesterdayData = searchHistory.filter(
    searchResult =>
      new Date(searchResult.created_at) < startOfToday &&
      new Date(searchResult.created_at) >= sub(startOfToday, { days: 1 })
  )
  const lastWeekData = searchHistory.filter(
    searchResult =>
      new Date(searchResult.created_at) < sub(startOfToday, { days: 1 }) &&
      new Date(searchResult.created_at) >= sub(startOfToday, { days: 7 })
  )
  const moreThanAWeekAgoData = searchHistory.filter(
    searchResult => new Date(searchResult.created_at) < sub(startOfToday, { days: 7 })
  )

  const handleScroll = async (e: UIEvent<HTMLDivElement, globalThis.UIEvent>) => {
    const targetElement = e.target as HTMLElement
    const isAtBottom = targetElement.scrollHeight - targetElement.scrollTop === targetElement.clientHeight
    if (isAtBottom && !isFetching) {
      await fetchNextPage()
    }
  }

  return (
    <div
      className={twMerge(
        classNames('w-full flex flex-col gap-y-4 max-h-96 overflow-y-scroll', {
          'scrollbar-hidden': !showScrollbar,
          'scrollbar-visible': showScrollbar,
        }),
        props.className
      )}
      onScroll={e => handleScroll(e)}
      onMouseEnter={() => setShowScrollbar(true)}
      onMouseLeave={() => setShowScrollbar(false)}
    >
      {todayData.length > 0 && <SearchHistorySlab title="Today" threads={todayData} />}
      {yesterdayData.length > 0 && <SearchHistorySlab title="Yesterday" threads={yesterdayData} />}
      {lastWeekData.length > 0 && <SearchHistorySlab title="Last week" threads={lastWeekData} />}
      {moreThanAWeekAgoData.length > 0 && (
        <SearchHistorySlab title="More than a week ago" threads={moreThanAWeekAgoData} />
      )}
    </div>
  )
}
