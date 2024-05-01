'use client'

import { useTextWidth } from '@/hooks/util/use-text-width'
import { HTMLAttributes, useState } from 'react'
import { Textarea } from '../lib/form'
import classNames from 'classnames'
import SendIcon from '../icons/send'
import { Button } from '../lib/button'

type SearchInputProps = HTMLAttributes<HTMLDivElement>

//480px

export default function SearchInput(props: SearchInputProps) {
  const [searchQuery, setSearchQuery] = useState('')
  const textWidth = useTextWidth(searchQuery)
  const lineCount = Math.ceil(textWidth / 560)

  return (
    <Textarea
      containerClass="w-[680px]"
      className={classNames(
        'pl-8 pr-16 bg-background-dark dark:bg-foreground-dark border border-custom-violet-light/10 text-foreground-light dark:text-background-light placeholder:text-foreground-light/40 dark:placeholder:text-background-light/40',
        {
          'h-20 pt-7': lineCount === 0,
          'h-20': lineCount === 1 || lineCount === 2,
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
      onChange={e => setSearchQuery(e.target.value)}
      button={
        <Button
          className={classNames('absolute right-4 h-10 px-3 py-0 ', {
            'top-6 bg-primary/10': searchQuery.length === 0,
            'bottom-4 bg-primary': searchQuery.length !== 0,
          })}
          iconLeft={<SendIcon className={searchQuery.length > 0 ? 'text-white' : 'text-white/40'} />}
        />
      }
    />
  )
}
