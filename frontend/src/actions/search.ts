'use server'

import { curieoFetch } from '@/actions/fetch'
import { SearchHistoryResponse, SearchReactionBody, SearchResult, ThreadByIdResponse } from '@/types/search'

export async function searchById(id: string): Promise<SearchResult> {
  const response = await curieoFetch(`/search/one?search_history_id=${id}`)
  if (response.ok) {
    return (await response.json()) as SearchResult
  }
  throw new Error('Retrieving search failed')
}

export async function searchHistory({
  limit,
  offset,
}: {
  limit: number
  offset: number
}): Promise<SearchHistoryResponse> {
  const response = await curieoFetch(`/search/history?limit=${limit ?? 10}${offset ? `&offset=${offset}` : ``}`)
  if (response.ok) {
    return (await response.json()) as SearchHistoryResponse
  }
  throw new Error('Retrieving search history failed')
}

export async function searchReaction(reaction: SearchReactionBody): Promise<void> {
  const response = await curieoFetch('/search/reaction', {
    method: 'PATCH',
    body: JSON.stringify(reaction),
  })
  if (response.ok) {
    return
  }
  throw new Error('Could not submit reaction')
}

export async function threadById(id: string): Promise<ThreadByIdResponse> {
  const response = await curieoFetch(`/search/threads?thread_id=${id}`)
  if (response.ok) {
    return (await response.json()) as ThreadByIdResponse
  }
  throw new Error('Retrieving search failed')
}
