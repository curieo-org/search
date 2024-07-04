import React, { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import SearchTitle from './search-title'
import { Span } from '../lib/typography'
import PropagateLoader from 'react-spinners/PropagateLoader'

type LoadingSearchResultProps = HTMLAttributes<HTMLDivElement> & {
  searchQuery: string
}

export default function LoadingSearchResult(props: LoadingSearchResultProps) {
  return (
    <div className={twMerge('w-full flex', props.className)}>
      <div className={twMerge('w-full max-w-[900px] mx-auto px-10 flex flex-col justify-between', props.className)}>
        <SearchTitle title={props.searchQuery} />
        <div className="flex flex-col">
          <div className="flex items-center gap-x-3 my-6">
            <img src="images/answer-logo.svg" className="h-4 w-auto" alt="answer-logo" />
            <Span className="font-light text-white/80 text-lg">Answer</Span>
          </div>
        </div>
        <PropagateLoader className="mx-auto" color="white" aria-label="Loading Spinner" size={10} loading={true} />
      </div>
      <div className="w-72 flex-none"></div>
    </div>
  )
}
