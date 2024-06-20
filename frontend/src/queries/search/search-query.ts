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
    console.log('querytrigger', queryTrigger)
    if (queryTrigger) {
      setIsCompleted(false)
      setIsError(false)
      console.log('event source before')
      const eventSource = new EventSource(
        `/backend-api/search?query=${searchQuery}${threadId ? `&thread_id=${threadId}` : ``}`,
        {
          withCredentials: true,
        }
      )

      console.log('event source after')

      let timeoutId: NodeJS.Timeout

      const resetHeartbeatTimeout = () => {
        clearTimeout(timeoutId)
        timeoutId = setTimeout(() => {
          eventSource.close()
          setIsTimedOut(true)
          console.log('EventSource connection closed due to heartbeat timeout')
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
        console.log('event source eror', err)
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
