'use server'

import { SearchReactionBody, SearchResult } from '@/types/search'
import { curieoFetch } from '@/actions/fetch'

export async function search(query: string): Promise<SearchResult> {
  const response = await curieoFetch(`/search?${new URLSearchParams({ query })}`)
  if (response.ok) {
    return (await response.json()) as SearchResult
  }
  throw new Error('Search failed')
}

export async function searchById(id: string): Promise<SearchResult> {
  const response = await curieoFetch(`/search/one?search_history_id=${id}`)
  if (response.ok) {
    return (await response.json()) as SearchResult
  }
  throw new Error('Retrieving search failed')
}

export async function searchHistory({ limit, offset }: { limit: number; offset: number }): Promise<SearchResult[]> {
  const response = await curieoFetch(`/search/history?limit=${limit ?? 10}${offset ? `&offset=${offset}` : ``}`)
  if (response.ok) {
    return (await response.json()) as SearchResult[]
  }
  throw new Error('Retrieving search history failed')
}

export async function searchReaction(reaction: SearchReactionBody): Promise<SearchResult> {
  const response = await curieoFetch('/search/reaction', {
    method: 'PATCH',
    body: JSON.stringify(reaction),
  })
  if (response.ok) {
    return (await response.json()) as SearchResult
  }
  throw new Error('Could not submit reaction')
}
