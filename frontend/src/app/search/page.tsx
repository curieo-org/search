'use client'

import { P } from '@/components/lib/typography'
import SearchInput from '@/components/search/search-input'
import SearchResultPageSkeleton from '@/components/skeletons/search-result-page-skeleton'
import { useSearchQuery } from '@/queries/search/search-query'
import { useFetchThreadByIdQuery } from '@/queries/search/fetch-thread-by-id-query'
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
  const { data: thread } = useFetchThreadByIdQuery({ threadId: threadId as string })

  useEffect(() => {
    if (isSuccess) {
      reset()
      router.push(`/search/${data.search_id}`)
    }
  }, [isSuccess])

  useEffect(() => {
    console.log(thread)
  }, [thread])

  useEffect(() => {
    if (isError) {
      toast.error('Failed to fetch search result. Please try again later...')
    }
  }, [isError])

  return (
    <>
      {isLoading ? (
        <SearchResultPageSkeleton />
      ) : (
        <div className="w-full h-[90vh] flex justify-center items-center">
          <div className="w-full max-w-[720px] flex flex-col items-center px-4">
            <P className="mb-10 text-2xl xl:text-3xl transition-all duration-300">How can I help you today?</P>
            <SearchInput handleSearch={handleSearch} />
          </div>
        </div>
      )}
    </>
  )
}
