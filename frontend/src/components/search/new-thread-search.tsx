import { SearchByIdResponse } from '@/types/search'
import _ from 'lodash'
import { HTMLAttributes, useEffect, useRef, useState } from 'react'
import { Span } from '../lib/typography'
import LoadingSearchResult from './loading-search-result'
import SearchResponse from './search-response'
import SearchTitle from './search-title'
import SourcesMenu from './sources-menu'

type NewThreadSearchProps = HTMLAttributes<HTMLDivElement> & {
  searchQuery: string
  response: SearchByIdResponse[]
  isStreaming: boolean
}

export default function NewThreadSearch(props: NewThreadSearchProps) {
  const answerContainerRef = useRef<HTMLDivElement>(null)
  const [answerContainerHeight, setAnswerContainerHeight] = useState<number>(0)

  useEffect(() => {
    const updateHeight = () => {
      if (answerContainerRef.current) {
        setAnswerContainerHeight(answerContainerRef.current?.offsetHeight)
      }
    }

    updateHeight()

    window.addEventListener('resize', updateHeight)
    return () => window.removeEventListener('resize', updateHeight)
  }, [props.response])

  return (
    <>
      {props.isStreaming ? (
        <div className="w-full flex justify-between">
          <div className="max-w-[900px] mx-auto flex-grow px-10 transition-all duration-300" ref={answerContainerRef}>
            <SearchTitle className="mb-6" title={props.response[0]?.search.query} />
            <div className="flex items-center gap-x-3 mb-6">
              <img src="images/answer-logo.svg" className="h-4 w-auto" alt="answer-logo" />
              <Span className="font-light text-white/80 text-lg">Answer</Span>
            </div>
            <SearchResponse
              className="mb-6"
              response={_.join(
                props.response.map(streamData => streamData.search.result),
                ''
              )}
            />
          </div>
          <SourcesMenu
            sources={_.flatten(props.response.map(streamData => streamData.sources))}
            style={{ maxHeight: answerContainerHeight }}
          />
        </div>
      ) : (
        <LoadingSearchResult searchQuery={props.searchQuery} />
      )}
    </>
  )
}
