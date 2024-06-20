'use client'

import { SearchByIdResponse } from '@/types/search'
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

  useEffect(() => {
    if (queryTrigger) {
      setIsCompleted(false)
      setIsError(false)
      const eventSource = new EventSource(
        `/backend-api/search?query=${searchQuery}${threadId ? `&thread_id=${threadId}` : ``}`,
        {
          withCredentials: true,
        }
      )

      let timeoutId: NodeJS.Timeout

      const resetHeartbeatTimeout = () => {
        clearTimeout(timeoutId)
        timeoutId = setTimeout(() => {
          eventSource.close()
          setIsTimedOut(true)
        }, 5000)
      }

      resetHeartbeatTimeout()

      eventSource.onmessage = event => {
        resetHeartbeatTimeout()
        setIsStreaming(true)
        const newData = JSON.parse(event.data)
        setData(prevData => [...prevData, newData as SearchByIdResponse])
      }

      eventSource.onerror = err => {
        clearTimeout(timeoutId)
        setIsCompleted(true)
        eventSource.close()
      }

      return () => {
        eventSource.close()
      }
    }
  }, [queryTrigger])

  return { data, isCompleted, isError, isTimedOut }
}
