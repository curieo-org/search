import { useFetchThreadByIdQuery } from '@/queries/search/fetch-thread-by-id-query'
import { Fragment, HTMLAttributes, useEffect } from 'react'
import { twMerge } from 'tailwind-merge'
import { H1 } from '../lib/typography'
import OldSearchResponse from './old-search-response'
import SearchInput from './search-input'
import { useSearchQuery } from '@/queries/search/search-query'

type ThreadProps = HTMLAttributes<HTMLDivElement> & {
  threadId: string
}

export default function Thread(props: ThreadProps) {
  const { data } = useFetchThreadByIdQuery({ threadId: props.threadId })
  const { data: newSearchResult, isLoading, isSuccess, isError, refetch: fetchSearchResult } = useSearchQuery()
  const handleSearch = () => {
    fetchSearchResult().then(r => r)
  }

  useEffect(() => {
    console.log(data)
  }, [data])

  if (!data) {
    return null
  }

  return (
    <div className={twMerge('w-full', props.className)}>
      <H1 className="px-10 py-4 text-2xl xl:text-3xl font-semibold">{data?.thread.title}</H1>
      <div className="pt-4 flex flex-col gap-y-4">
        {data?.searches.map((search, index) => (
          <Fragment key={`search-response-${data.thread.thread_id}-${index}`}>
            <OldSearchResponse search={search} />
            <div className="h-0.5 bg-white/20 ml-10 mr-6"></div>
          </Fragment>
        ))}
      </div>
      <div className="sticky bottom-0 pb-4 px-8 -mb-4 flex justify-start backdrop-blur-sm">
        <SearchInput handleSearch={handleSearch} />
      </div>
    </div>
  )
}
