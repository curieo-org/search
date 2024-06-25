import { useSearchQuery } from '@/queries/search/search-query'
import { ThreadByIdResponse } from '@/types/search'
import { QueryObserverResult } from '@tanstack/react-query'
import _ from 'lodash'
import { Fragment, HTMLAttributes, useEffect, useRef, useState } from 'react'
import { toast } from 'react-toastify'
import { twMerge } from 'tailwind-merge'
import { H1 } from '../lib/typography'
import NewThreadSearch from './new-thread-search'
import OldSearchResponse from './old-search-response'
import SearchInput from './search-input'

type ThreadProps = HTMLAttributes<HTMLDivElement> & {
  data: ThreadByIdResponse
  refetch: () => Promise<QueryObserverResult<ThreadByIdResponse, Error>>
}

export default function Thread(props: ThreadProps) {
  const pageEndRef = useRef<null | HTMLDivElement>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [queryTrigger, setQueryTrigger] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [isStreaming, setIsStreaming] = useState(false)
  const {
    data: newSearchResult,
    isCompleted,
    isError,
    isTimedOut,
  } = useSearchQuery(searchQuery, queryTrigger, setIsStreaming, props.data.thread.thread_id)

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
    setSearchQuery('')
  }, [isStreaming])

  useEffect(() => {
    if (isCompleted) {
      setSearchQuery('')
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
      setSearchQuery('')
      setQueryTrigger(false)
      setIsLoading(false)
      setIsStreaming(false)
      toast.error('The server took too long to respond. Please try again later.')
    }
  }, [isTimedOut])

  return (
    <div className={twMerge('w-full min-h-screen flex flex-col justify-between', props.className)}>
      <div className="flex flex-col">
        <H1 className="px-10 py-4 text-2xl xl:text-3xl font-semibold">{props.data?.thread.title}</H1>
        <div className="pt-4 flex flex-col gap-y-4">
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

      <div className="w-full sticky bottom-0 pb-4 px-8 flex justify-start backdrop-blur-sm max-w-[840px]">
        <SearchInput handleSearch={handleSearch} searchQuery={searchQuery} setSearchQuery={setSearchQuery} />
      </div>

      <div ref={pageEndRef} />
    </div>
  )
}
