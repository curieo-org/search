'use server'

import { UserProfile } from '@/types/settings'
import { SearchResult } from '@/types/search'

export async function fetchUserProfile(): Promise<UserProfile | null> {
  const response = await fetch(`${process.env.NEXT_AUTH_URL}/backend-api/users/me`)
  if (response.ok) {
    return (await response.json()) as UserProfile
  }
  return null
}

export async function fetchSearchHistory(pageParam): Promise<SearchResult[] | null> {
  const response = await fetch(
    `${process.env.NEXT_AUTH_URL}/backend-api/search/history?limit=${pageParam.limit ?? 10}${pageParam.offset ? `&offset=${pageParam.offset}` : ``}`
  )
  if (response.ok) {
    return (await response.json()) as SearchResult[]
  }
  return null
}
