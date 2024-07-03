import { SearchByIdResponse } from '@/types/search'
import { HTMLAttributes, useEffect, useRef, useState } from 'react'
import SearchActions from './search-actions'
import SearchResponse from './search-response'
import SearchTitle from './search-title'
import SourcesMenu from './sources-menu'
import { Span } from '../lib/typography'

type OldSearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  search: SearchByIdResponse
  shortenSourcesLength: boolean
}

export default function OldSearchResponse(props: OldSearchResponseProps) {
  const { search: searchResult, sources } = props.search
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
  }, [])

  return (
    <div className="w-full flex justify-between">
      <div className="max-w-[900px] mx-auto flex-grow px-10 transition-all duration-300" ref={answerContainerRef}>
        <SearchTitle className="mb-6" title={searchResult.query} />
        <div className="flex items-center gap-x-3 mb-6">
          <img src="images/answer-logo.svg" className="w-10 h-10" alt="answer-logo" />
          <Span className="font-light text-white/80 text-xl">Answer</Span>
        </div>
        <SearchResponse className="mb-6" response={searchResult.result} />
        <SearchActions
          searchHistoryId={searchResult.search_id}
          reaction={searchResult.reaction}
          response={searchResult.result}
        />
      </div>
      <SourcesMenu
        sources={sources}
        style={{ maxHeight: props.shortenSourcesLength ? answerContainerHeight : '90vh' }}
      />
    </div>
  )
}
