'use client'

import { useTextWidth } from '@/hooks/util/use-text-width'
import { useSearchStore } from '@/stores/search/search-store'
import classNames from 'classnames'
import { HTMLAttributes } from 'react'
import PaperPlaneIcon from '../icons/paper-plane'
import { Button } from '../lib/button'
import { Textarea } from '../lib/form'

type SearchInputProps = HTMLAttributes<HTMLDivElement> & {
  handleSearch: () => void
}

//480px

export default function SearchInput(props: SearchInputProps) {
  const {
    state: { searchQuery },
    setSearchState,
  } = useSearchStore()
  const textWidth = useTextWidth(searchQuery.toUpperCase())
  const lineCount = Math.max(Math.ceil(textWidth / 560), searchQuery.length > 0 ? searchQuery.split('\n').length : 0)

  return (
    <Textarea
      containerClass="max-w-[680px] rounded-2.5xl p-2.5 bg-background-dark/4 dark:bg-background-light/4"
      innerContainerClass={classNames(
        'rounded-2xl bg-background-light/80 dark:bg-background-dark/80 border border-background-dark/40 dark:border-background-light/40 pr-2 focus-within:border-0 focus-within:outline-none focus-within:ring-2 focus-within:ring-custom-violet-600/50 focus-within:ring-offset-0',
        {
          'py-2': lineCount >= 2,
        }
      )}
      className={classNames(
        'pl-4 pr-20 rounded-2xl font-light border-none focus-visible:ring-0 bg-background-light/80 dark:bg-background-dark/80 text-typography-light dark:text-typography-dark placeholder:text-typography-light/40 dark:placeholder:text-typography-dark/40 text-sm xl:text-base',
        {
          'h-16 pt-5': lineCount === 0 || lineCount === 1,
          'h-20': lineCount === 2,
          'h-28': lineCount === 3,
          'h-36': lineCount === 4,
          'h-40': lineCount === 5,
          'h-48': lineCount === 6,
          'h-52': lineCount === 7,
          'h-60': lineCount === 8,
          'h-64': lineCount === 9,
          'h-72': lineCount >= 10,
        }
      )}
      placeholder="What can I do for you today?"
      value={searchQuery}
      onChange={e => setSearchState('searchQuery', e.target.value)}
      button={
        <Button
          className={classNames(
            'absolute right-8 h-10 px-4 py-0 border border-background-dark/30 dark:border-background-light/30 transition-all duration-700',
            {
              'top-6 bg-transparent hover:bg-transparent cursor-auto': searchQuery.length === 0,
              'top-6 bg-primary/75': lineCount === 1,
              'bottom-5 bg-primary/75': lineCount >= 2,
            }
          )}
          iconLeft={<PaperPlaneIcon className="text-white/80" />}
          onClick={() => {
            if (searchQuery.length > 0) {
              props.handleSearch()
            }
          }}
        />
      }
    />
  )
}
