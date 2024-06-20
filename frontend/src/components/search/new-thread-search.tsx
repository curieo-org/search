import { SearchByIdResponse } from '@/types/search'
import _ from 'lodash'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import { Span } from '../lib/typography'
import SearchResultSkeleton from '../skeletons/search-result-skeleton'
import SearchResponse from './search-response'
import SourcesMenu from './sources-menu'
import SearchTitle from './search-title'

type NewThreadSearchProps = HTMLAttributes<HTMLDivElement> & {
  response: SearchByIdResponse[]
  isStreaming: boolean
}

export default function NewThreadSearch(props: NewThreadSearchProps) {
  return (
    <>
      {props.isStreaming ? (
        <div className={twMerge('w-full', props.className)}>
          <SearchTitle className="mx-10 mt-10" title={props.response[0]?.search.query} />
          <div className="w-full flex">
            <div className="w-full flex flex-col px-10 justify-between">
              <div className="flex flex-col">
                <div className="flex items-center gap-x-3 my-6">
                  <img src="images/answer-logo.svg" className="w-10 h-10" alt="answer-logo" />
                  <Span className="font-light text-white/80 text-xl">Answer</Span>
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
            <SourcesMenu
              className="w-60 xl:w-96 max-h-screen overflow-y-scroll scrollbar-visible -mb-4 -mt-10 over py-2 pr-2 mr-1 transition-all duration-300"
              sources={_.flatten(props.response.map(streamData => streamData.sources))}
            />
          </div>
        </div>
      ) : (
        <SearchResultSkeleton className="mx-10" />
      )}
    </>
  )
}
