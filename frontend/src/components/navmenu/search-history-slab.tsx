import { Thread } from '@/types/search'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import SearchHistoryButton from './search-history-button'

type SearchHistorySlabProps = HTMLAttributes<HTMLDivElement> & {
  title: string
  threads: Thread[]
}

export default function SearchHistorySlab(props: SearchHistorySlabProps) {
  return (
    <div className={twMerge('w-full', props.className)}>
      <span className="text-input-placeholder text-2xs ml-6">{props.title}</span>
      <div className="flex flex-col mt-0.5">
        {props.threads.map((thread, index) => (
          <SearchHistoryButton
            key={`search-history-nav-${thread.thread_id}`}
            style={{ animation: `fade-in ${Math.min(500 + index * 300, 3000)}ms` }}
            thread={thread}
          />
        ))}
      </div>
    </div>
  )
}
