import { SearchByIdResponse } from '@/types/search'
import _ from 'lodash'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import { Span } from '../lib/typography'
import SearchInput from './search-input'
import SearchResponse from './search-response'
import SearchTitle from './search-title'
import SourcesMenu from './sources-menu'

type NewSearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  response: SearchByIdResponse[]
}

export default function NewSearchResponse(props: NewSearchResponseProps) {
  console.log(props.response)
  return (
    <div className={twMerge('w-full min-h-screen flex flex-col justify-between', props.className)}>
      <div className="w-full flex">
        <div className="w-full max-w-[900px] mx-auto flex flex-col px-10">
          <SearchTitle className="mx-10 mt-10" title={props.response[0]?.search.query} />
          <div className="flex flex-col px-10">
            <div className="flex items-center gap-x-3 my-6">
              <img src="images/answer-logo.svg" className="h-4 w-auto" alt="answer-logo" />
              <Span className="font-light text-white/80 text-lg">Answer</Span>
            </div>
            <SearchResponse
              className="w-full"
              response={_.join(
                props.response.map(streamData => streamData.search.result),
                ''
              )}
            />
          </div>
        </div>
        <SourcesMenu className="mt-10" sources={_.flatten(props.response.map(streamData => streamData.sources))} />
      </div>

      <div className="w-[calc(100vw-576px)] sticky bottom-0 pb-4 px-4">
        <SearchInput
          handleSearch={() => {}}
          searchQuery=""
          setSearchQuery={query => {}}
          containerClass="max-w-[900px] mx-auto backdrop-blur-sm"
        />
      </div>
    </div>
  )
}
