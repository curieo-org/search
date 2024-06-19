'use client'

import NewSearch from '@/components/search/new-search'
import Thread from '@/components/search/thread'
import SearchResultPageSkeleton from '@/components/skeletons/search-result-page-skeleton'
import { useFetchThreadByIdQuery } from '@/queries/search/fetch-thread-by-id-query'
import { useSearchQuery } from '@/queries/search/search-query'
import { useSearchStore } from '@/stores/search/search-store'
import { useRouter, useSearchParams } from 'next/navigation'
import { useEffect } from 'react'
import { toast } from 'react-toastify'

export default function Search() {
  const searchParams = useSearchParams()
  const router = useRouter()
  const { reset } = useSearchStore()
  const { data, isLoading, isSuccess, isError, refetch: fetchSearchResult } = useSearchQuery()
  const handleSearch = () => {
    fetchSearchResult().then(r => r)
  }
  const threadId = searchParams.get('thread_id')

  // const { data: thread } = useFetchThreadByIdQuery({ threadId: threadId as string })

  // useEffect(() => {
  //   console.log(thread)
  // }, [thread])

  useEffect(() => {
    if (isSuccess) {
      reset()
      router.push(`/search/${data.search_id}`)
    }
  }, [isSuccess])

  useEffect(() => {
    if (isError) {
      toast.error('Failed to fetch search result. Please try again later...')
    }
  }, [isError])

  return (
    <>
      {!!threadId ? (
        <Thread threadId={threadId} />
      ) : isLoading ? (
        <SearchResultPageSkeleton />
      ) : (
        <NewSearch handleSearch={handleSearch} />
      )}
    </>
  )
}
