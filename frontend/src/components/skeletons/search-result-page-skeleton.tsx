import { HTMLAttributes } from 'react'
import LayersIcon from '../icons/layers'
import { H2 } from '../lib/typography'
import SearchResultSkeleton from './search-result-skeleton'
import SourceSkeleton from './source-skeleton'

type SearchResultPageSkeletonProps = HTMLAttributes<HTMLDivElement>

export default function SearchResultPageSkeleton(props: SearchResultPageSkeletonProps) {
  return (
    <div className="w-full flex">
      <div className="w-full flex flex-col justify-between">
        <div className="w-full px-6 py-10 lg:px-12 xl:px-20 xl:py-20 mx-auto xl:max-w-[880px] transition-all duration-300">
          <SearchResultSkeleton />
        </div>
      </div>
      <div className="w-60 xl:w-96 py-10 xl:py-20">
        <div className="flex items-center gap-x-2 pl-2 mb-4 animate-pulse">
          <LayersIcon className="text-typography-light dark:text-typography-dark" size={20} />
          <H2 className="font-medium">Sources</H2>
        </div>
        <div className="flex flex-col gap-y-3">
          {Array(5)
            .fill(0)
            .map((item, index) => (
              <SourceSkeleton
                className="p-2 xl:p-3 rounded-2xl border border-background-dark/20 dark:border-background-light/20"
                key={`source-skeleton-${index}`}
              />
            ))}
        </div>
      </div>
    </div>
  )
}
