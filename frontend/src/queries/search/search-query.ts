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

      while (true) {
        const { done, value } = await reader.read()
        if (done) {
          setIsCompleted(true)
          break
        }

        const SPLIT_PREFIX_LEN = 6 // length of data : is 6

        // TODO: split by data : and maintain a string buffer. To handle the case when data : is in value
        const text = decoder.decode(value, { stream: true })
        const newData: SearchByIdResponse = JSON.parse(text.slice(SPLIT_PREFIX_LEN))
        setData(prevData => [...prevData, newData])
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
