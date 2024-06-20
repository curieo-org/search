'use client'

import NewSearch from '@/components/search/new-search'
import NewSearchResponse from '@/components/search/new-search-response'
import Thread from '@/components/search/thread'
import SearchResultPageSkeleton from '@/components/skeletons/search-result-page-skeleton'
import { useFetchThreadByIdQuery } from '@/queries/search/fetch-thread-by-id-query'
import { useSearchQuery } from '@/queries/search/search-query'
import { useSearchParams } from 'next/navigation'
import { useEffect, useState } from 'react'
import { toast } from 'react-toastify'

export default function Search() {
  const searchParams = useSearchParams()
  const [searchQuery, setSearchQuery] = useState('')
  const [queryTrigger, setQueryTrigger] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [isStreaming, setIsStreaming] = useState(false)
  const {
    data: newSearchResult,
    isCompleted,
    isError,
    isTimedOut,
  } = useSearchQuery(searchQuery, queryTrigger, setIsStreaming)
  const handleSearch = () => {
    setIsLoading(true)
    setQueryTrigger(true)
  }
  const threadIdFromParams = searchParams.get('thread_id')
  const [threadId, setThreadId] = useState('')
  const { data: thread, refetch: refetchThread } = useFetchThreadByIdQuery({ threadId: threadId as string })

  useEffect(() => {
    setThreadId(threadIdFromParams as string)
  }, [threadIdFromParams])

  useEffect(() => {
    if (isCompleted) {
      setSearchQuery('')
      setQueryTrigger(false)
      if (newSearchResult.length === 0) {
        toast.error('Failed to fetch search result. Please try again later...')
        setIsLoading(false)
        setIsStreaming(false)
      } else {
        setThreadId(newSearchResult[0].search.thread_id)
        setIsLoading(false)
        setIsStreaming(false)

        const params = new URLSearchParams()
        params.set('thread_id', newSearchResult[0].search.thread_id)
        window.history.pushState(null, '', `?${params.toString()}`)
      }
    }
  }, [isCompleted])

  useEffect(() => {
    if (isTimedOut) {
      setIsLoading(false)
      setIsStreaming(false)
      setSearchQuery('')
      setQueryTrigger(false)
      toast.error('The server took too long to respond. Please try again later.')
    }
  }, [isTimedOut])

  useEffect(() => {
    if (Boolean(threadId)) {
      refetchThread().then(r => r)
    }
  }, [threadId])

  return (
    <>
      {!!thread ? (
        <Thread data={thread} refetch={refetchThread} />
      ) : isLoading ? (
        <>{isStreaming ? <NewSearchResponse response={newSearchResult} /> : <SearchResultPageSkeleton />}</>
      ) : (
        <NewSearch handleSearch={handleSearch} searchQuery={searchQuery} setSearchQuery={setSearchQuery} />
      )}
    </>
  )
}
