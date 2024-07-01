'use client'

import { SearchByIdResponse } from '@/types/search'
import { useQueryClient } from '@tanstack/react-query'
import { useEffect, useState } from 'react'

export const useSearchQuery = (
  searchQuery: string,
  queryTrigger: boolean,
  setIsStreaming: (isStreaming: boolean) => void,
  threadId?: string
) => {
  const [data, setData] = useState<SearchByIdResponse[]>([])
  const [isCompleted, setIsCompleted] = useState(false)
  const [isError, setIsError] = useState(false)
  const [isTimedOut, setIsTimedOut] = useState(false)
  const queryClient = useQueryClient()

  const initialize = () => {
    setData([])
    setIsCompleted(false)
    setIsError(false)
    setIsTimedOut(false)
  }

  const fetchStream = async () => {
    try {
      const timeNow = new Date()
      const response = await fetch(`/api/search?searchQuery=${searchQuery}${threadId ? `&threadId=${threadId}` : ``}`)

      if (!response.body) {
        throw new Error('ReadableStream not supported by the response')
      }

      setIsStreaming(true)

      queryClient.invalidateQueries({ queryKey: ['search-history'] })

      const reader = response.body.getReader()
      const decoder = new TextDecoder()

      let streamBuffer = ''

      while (true) {
        const { done, value } = await reader.read()
        if (done) {
          setIsCompleted(true)
          break
        }

        const SPLIT_PREFIX = 'data: {'
        const text = streamBuffer + decoder.decode(value, { stream: true })
        const chunks = text.split(SPLIT_PREFIX)
        streamBuffer = ''

        for (let index = 0; index < chunks.length; index++) {
          if (chunks[index].length === 0) {
            continue
          }
          try {
            const jsonString = '{' + chunks[index]
            const newData: SearchByIdResponse = JSON.parse(jsonString)
            setData(prevData => [...prevData, newData])
          } catch (error) {
            streamBuffer = SPLIT_PREFIX + chunks.slice(index).join(SPLIT_PREFIX)
            break
          }
        }
      }
    } catch (error) {
      setIsError(true)
      setIsStreaming(false)
    }
  }

  useEffect(() => {
    if (queryTrigger) {
      initialize()
      fetchStream()
      return () => {}
    }
  }, [queryTrigger])

  return { data, isCompleted, isError, isTimedOut }
}
