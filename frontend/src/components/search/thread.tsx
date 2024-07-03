import { useSearchQuery } from '@/queries/search/search-query'
import { ThreadByIdResponse } from '@/types/search'
import { QueryObserverResult } from '@tanstack/react-query'
import _ from 'lodash'
import { Fragment, HTMLAttributes, useEffect, useRef, useState } from 'react'
import { toast } from 'react-toastify'
import { twMerge } from 'tailwind-merge'
import LayersIcon from '../icons/layers'
import { H1, H2 } from '../lib/typography'
import NewThreadSearch from './new-thread-search'
import OldSearchResponse from './old-search-response'
import SearchInput from './search-input'

type ThreadProps = HTMLAttributes<HTMLDivElement> & {
  data: ThreadByIdResponse
  refetch: () => Promise<QueryObserverResult<ThreadByIdResponse, Error>>
  searchQuery: string
  handleSetSearchQuery: (query: string) => void
}

export default function Thread(props: ThreadProps) {
  const pageEndRef = useRef<null | HTMLDivElement>(null)
  const [queryTrigger, setQueryTrigger] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [isStreaming, setIsStreaming] = useState(false)
  const {
    data: newSearchResult,
    isCompleted,
    isError,
    isTimedOut,
  } = useSearchQuery(props.searchQuery, queryTrigger, setIsStreaming, props.data.thread.thread_id)

  const handleSearch = () => {
    setIsLoading(true)
    setQueryTrigger(true)
  }

  const scrollToBottom = () => {
    pageEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }

  useEffect(() => {
    scrollToBottom()
  }, [isLoading, newSearchResult])

  useEffect(() => {
    props.handleSetSearchQuery('')
  }, [isStreaming])

  useEffect(() => {
    if (isCompleted) {
      props.handleSetSearchQuery('')
      setQueryTrigger(false)
      if (newSearchResult.length === 0) {
        toast.error('Failed to fetch search result. Please try again later...')
        setIsLoading(false)
        setIsStreaming(false)
      } else {
        props.refetch().then(() => {
          setIsLoading(false)
          setIsStreaming(false)
        })
      }
    }
  }, [isCompleted])

  useEffect(() => {
    if (isTimedOut) {
      props.handleSetSearchQuery('')
      setQueryTrigger(false)
      setIsLoading(false)
      setIsStreaming(false)
      toast.error('The server took too long to respond. Please try again later.')
    }
  }, [isTimedOut])

  return (
    <div className={twMerge('w-full min-h-screen flex flex-col justify-between', props.className)}>
      <div className="flex flex-col">
        <div className="flex justify-between pt-4">
          <H1 className="px-10 text-2xl xl:text-3xl font-light">{props.data?.thread.title}</H1>
          <div className="mr-2.5 w-[268px] h-10 flex gap-x-2 items-center justify-center border border-white/10 rounded-lg">
            <LayersIcon className="text-typography-light dark:text-typography-dark" size={14} />
            <H2 className="font-medium text-[#DDDDE3] text-sm">Sources</H2>
          </div>
        </div>
        <div className="mt-2 flex flex-col gap-y-4">
          {_.reverse([...props.data?.searches]).map((search, index) => (
            <Fragment key={`search-response-${props.data.thread.thread_id}-${index}`}>
              <OldSearchResponse search={search} shortenSourcesLength={props.data.searches.length !== 1 || isLoading} />
              {props.data.searches.length !== 1 && <div className="h-0.5 bg-white/20 ml-10 mr-6"></div>}
            </Fragment>
          ))}
          {isLoading ? <NewThreadSearch response={newSearchResult} isStreaming={isStreaming} /> : <></>}
          <div className="h-4"></div>
        </div>
      </div>

      <div className="w-[calc(100vw-576px)] sticky bottom-0 pb-4 px-4 flex justify-start backdrop-blur-sm">
        <SearchInput
          handleSearch={handleSearch}
          searchQuery={props.searchQuery}
          setSearchQuery={props.handleSetSearchQuery}
        />
      </div>

      <div ref={pageEndRef} />
    </div>
  )
}
