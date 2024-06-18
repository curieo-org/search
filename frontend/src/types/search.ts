export type Source = {
  source_id: string
  url: string
  title: string
  description: string
  source_type: string
  metadata: Record<string, string>
  created_at: string
  updated_at: string
}

export type SearchResult = {
  search_id: string
  thread_id: string
  query: string
  result: string
  media_urls: null | string[]
  reaction: null | boolean
  created_at: string
  updated_at: string
}

export type SearchByIdParams = {
  searchId: string
}

export type SearchByIdResponse = {
  search: SearchResult
  sources: Source[]
}

export type SearchReactionBody = {
  search_id: string
  reaction: boolean
}

export type ThreadByIdParams = {
  threadId: string
}

export type Thread = {
  thread_id: string
  user_id: string
  title: string
  context: any
  created_at: string
  updated_at: string
}

export type ThreadByIdResponse = {
  thread: Thread
  searches: SearchByIdResponse[]
}
