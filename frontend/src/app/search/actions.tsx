'use server'

import { headers } from 'next/headers'
import { SearchResult } from '@/types/search'
import { z } from 'zod'
import { auth } from '@/auth'

export async function search(prevState: SearchResult, formData: FormData) {
  //  const { user } = await auth()
  //  if (!user) {
  //    throw new Error('You must be signed in to perform this action')
  //  }
  //
  //  const schema = z.object({
  //    query: z.string().min(1),
  //  })
  //  const { query } = schema.parse(formData)
  //
  //  const authorization = headers().get('authorization')!
  //  const res = await fetch(`backend-api/search?query=${query.trim()}`, {
  //    headers: { authorization },
  //  })
  //  if (!res.ok) {
  //    // This will activate the closest `error.js` Error Boundary
  //    throw new Error('Failed to fetch data')
  //  }
  //  const result = await res.json()
  //
  //  return result as SearchResult
}
