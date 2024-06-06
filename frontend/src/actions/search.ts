'use server'

import { SearchResult } from '@/types/search'

export async function search(query: string, sessionId: string): Promise<SearchResult> {
  let response = await fetch(
    `${process.env.NEXT_AUTH_URL}/backend-api/search?${new URLSearchParams({
      query,
      sessionId,
    })}`
  )
  if (response.ok) {
    return (await response.json()) as SearchResult
  }
  throw new Error('Search failed')
}
