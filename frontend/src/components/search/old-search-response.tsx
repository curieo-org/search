import { SearchByIdResponse } from '@/types/search'
import { HTMLAttributes, useEffect, useRef, useState } from 'react'
import SearchActions from './search-actions'
import SearchResponse from './search-response'
import SearchTitle from './search-title'
import SourcesMenu from './sources-menu'
import { Span } from '../lib/typography'

type OldSearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  search: SearchByIdResponse
}

export default function OldSearchResponse(props: OldSearchResponseProps) {
  const { search: searchResult, sources } = props.search
  const answerContainerRef = useRef<HTMLDivElement>(null)
  // const sourcesConatinerRef = useRef<HTMLDivElement>(null)
  const [answerContainerHeight, setanswerContainerHeight] = useState<number>(0)

  useEffect(() => {
    const updateHeight = () => {
      if (answerContainerRef.current) {
        setanswerContainerHeight(Math.min(answerContainerRef.current.offsetHeight, 200))
      }
    }

    updateHeight()

    window.addEventListener('resize', updateHeight)
    return () => window.removeEventListener('resize', updateHeight)
  }, [])

  return (
    <div className="w-full flex max-h-80">
      <div className="w-full flex flex-col justify-between" ref={answerContainerRef}>
        <div className="w-full px-10 transition-all duration-300">
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
      </div>
      <SourcesMenu
        className="w-60 xl:w-96 p-3 transition-all duration-300 bg-white/2 rounded-l-2xl"
        sources={sources}
        style={{ maxHeight: `calc(100vh - ${answerContainerHeight}px)` }}
      />
    </div>
  )
}
