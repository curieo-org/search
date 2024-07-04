'use client'

import { maxSearchInputHeight } from '@/constants/style'
import classNames from 'classnames'
import { ChangeEvent, HTMLAttributes, KeyboardEvent, useRef } from 'react'
import PaperPlaneIcon from '../icons/paper-plane'
import { IconButton } from '../lib/button'
import { Textarea } from '../lib/form'

type SearchInputProps = HTMLAttributes<HTMLDivElement> & {
  handleSearch: () => void
  searchQuery: string
  setSearchQuery: (query: string) => void
}

export default function SearchInput(props: SearchInputProps) {
  const textAreaRef = useRef<HTMLTextAreaElement | null>(null)

  const handleChange = (event: ChangeEvent<HTMLTextAreaElement>) => {
    props.setSearchQuery(event.target.value)

    const target = event.target as HTMLTextAreaElement
    if (textAreaRef?.current?.style) {
      textAreaRef.current.style.height = '64px'
      textAreaRef.current.style.height = `${Math.min(target.scrollHeight, maxSearchInputHeight)}px`
    }
  }

  const handleSubmit = () => {
    if (props.searchQuery.length > 0) {
      props.handleSearch()
    }
  }

  const handleKeyDown = (event: KeyboardEvent<HTMLTextAreaElement>) => {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault()
      handleSubmit()
    }
  }

  return (
    <Textarea
      ref={textAreaRef}
      containerClass="max-w-[840px] grow rounded-2.5xl p-2.5 bg-background-dark/4 dark:bg-background-light/4"
      innerContainerClass={classNames(
        'rounded-2xl bg-background-light/80 dark:bg-background-dark/80 border border-background-dark/40 dark:border-background-light/40 pr-2 focus-within:border-0 focus-within:outline-none focus-within:ring-1 focus-within:ring-primary focus-within:ring-offset-0',
        {
          'py-2': textAreaRef?.current?.style.height === `${maxSearchInputHeight}px`,
        }
      )}
      className={classNames(
        'h-16 pl-6 pr-16 rounded-2xl border-none focus-visible:ring-0 bg-background-light/80 dark:bg-background-dark/80 text-typography-light dark:text-typography-dark placeholder:text-typography-light/40 dark:placeholder:text-typography-dark/40 text-sm xl:text-base',
        {
          'py-5': textAreaRef?.current?.style.height !== `${maxSearchInputHeight}px` || props.searchQuery.length === 0,
          'py-3': textAreaRef?.current?.style.height === `${maxSearchInputHeight}px`,
        }
      )}
      placeholder="What can I do for you today?"
      value={props.searchQuery}
      onChange={handleChange}
      onKeyDown={handleKeyDown}
      button={
        <IconButton
          className={classNames(
            'absolute right-6 h-10 w-12 border border-background-dark/30 dark:border-background-light/30 transition-all duration-700 rounded-[10px]',
            {
              'cursor-auto': props.searchQuery.length === 0,
              'bg-primary/75': props.searchQuery.length !== 0,
              'top-6': textAreaRef?.current?.style.height === '64px' || props.searchQuery.length === 0,
              'bottom-5': textAreaRef?.current?.style.height !== '64px',
              'right-8': textAreaRef?.current?.style.height === `${maxSearchInputHeight}px`,
            }
          )}
          icon={<PaperPlaneIcon className="text-white/80" size={16} />}
          onClick={handleSubmit}
        />
      }
    />
  )
}
