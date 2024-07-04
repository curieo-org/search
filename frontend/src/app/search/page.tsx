'use client'

import LoadingSearchResult from '@/components/search/loading-search-result'
import NewSearch from '@/components/search/new-search'
import NewSearchResponse from '@/components/search/new-search-response'
import Thread from '@/components/search/thread'
import { MAX_QUERY_LENGTH } from '@/constants/search'
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
  } = useSearchQuery(searchQuery.trim(), queryTrigger, setIsStreaming)
  const handleSearch = () => {
    if (searchQuery.trim().length === 0) {
      toast.error(`Search query can not be empty!`)
      return
    }
    setIsLoading(true)
    setQueryTrigger(true)
  }
  const threadIdFromParams = searchParams.get('thread_id')
  const [threadId, setThreadId] = useState('')
  const { data: thread, refetch: refetchThread } = useFetchThreadByIdQuery({ threadId: threadId as string })

  const handleSetSearchQuery = (query: string) => {
    if (query.length > MAX_QUERY_LENGTH) {
      toast.error(`Maximum allowed length for search query is ${MAX_QUERY_LENGTH} charcaters.`)
      setSearchQuery(query.substring(0, MAX_QUERY_LENGTH))
      return
    }
    setSearchQuery(query)
  }

  useEffect(() => {
    setThreadId(threadIdFromParams as string)
  }, [threadIdFromParams])

  useEffect(() => {
    if (isCompleted) {
      setSearchQuery('')
      setQueryTrigger(false)
      setIsLoading(false)
      setIsStreaming(false)
      if (newSearchResult.length === 0) {
        toast.error('Failed to fetch search result. Please try again later...')
      } else {
        setThreadId(newSearchResult[0].search.thread_id)
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
    if (isError) {
      setIsLoading(false)
      setIsStreaming(false)
      setSearchQuery('')
      setQueryTrigger(false)
      toast.error('The server ran into an error while generating stream.')
    }
  }, [isError])

  useEffect(() => {
    if (Boolean(threadId)) {
      refetchThread().then(r => r)
    }
  }, [threadId])

  return (
    <>
      {!!thread ? (
        <Thread
          data={thread}
          refetch={refetchThread}
          searchQuery={searchQuery}
          handleSetSearchQuery={handleSetSearchQuery}
        />
      ) : isLoading ? (
        <>
          {isStreaming ? (
            <NewSearchResponse response={newSearchResult} />
          ) : (
            <LoadingSearchResult className="mt-4" searchQuery={searchQuery} />
          )}
        </>
      ) : (
        <NewSearch
          handleSearch={() => {
            if (!isLoading) {
              handleSearch()
            }
          }}
          searchQuery={searchQuery}
          handleSetSearchQuery={handleSetSearchQuery}
        />
      )}
    </>
  )
}
