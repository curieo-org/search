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

        const text = decoder.decode(value, { stream: true })
        const lines = text.split('\n').filter(Boolean)
        lines.forEach(line => {
          if (line.startsWith('data: ')) {
            const newData: SearchByIdResponse = JSON.parse(line.slice(6))
            console.log(new Date().getTime() - timeNow.getTime())
            //console.log(newData)
            setData(prevData => [...prevData, newData])
          }
        })
      }
    } catch (error) {
      console.error('Fetch stream error:', error)
      setIsError(true)
      setIsStreaming(false)
    }
  }

  useEffect(() => {
    if (queryTrigger) {
      fetchStream()
      return () => {}
    }
  }, [queryTrigger])

  return { data, isCompleted, isError, isTimedOut }
}
