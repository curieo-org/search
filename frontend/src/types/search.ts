export type Source = {
  url: string
  metadata: Record<string, string>
}

export type SearchResult = {
  search_history_id: string
  user_id: string
  session_id: string
  query: string
  result: string
  sources: Source[]
  reaction: boolean | null
  created_at: string
  updated_at: string
}

export type SearchByIdParams = {
  searchHistoryId: string
}
