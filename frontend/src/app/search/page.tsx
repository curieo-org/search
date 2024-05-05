'use client'

import { P } from '@/components/lib/typography'
import SearchInput from '@/components/search/search-input'
import SpinnerLoading from '@/components/util/spinner-loading'
import { searchLoadingMessage } from '@/constants/messages'
import { useSearchQuery } from '@/queries/search/search-query'
import { useSearchStore } from '@/stores/search/search-store'
import { useRouter } from 'next/navigation'
import { Fragment, useEffect } from 'react'

export default function Search() {
  const router = useRouter()
  const { reset } = useSearchStore()
  const { data, isLoading, isSuccess, refetch: fetchSearchResult } = useSearchQuery()
  const handleSearch = () => {
    fetchSearchResult()
  }

  useEffect(() => {
    if (isSuccess) {
      reset()
      router.push(`/search/${data.search_history_id}`)
    }
  }, [isSuccess])

  return (
    <Fragment>
      {isLoading ? (
        <SpinnerLoading message={searchLoadingMessage} />
      ) : (
        <div className="w-full h-[90vh] flex justify-center items-center">
          <div className="w-full flex flex-col items-center px-4">
            <P className="mb-10 text-2xl xl:text-3xl transition-all duration-300">Fast-Track Your Research</P>
            <SearchInput handleSearch={handleSearch} />
          </div>
        </div>
      )}
    </Fragment>
  )
}
