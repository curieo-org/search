'use server'

import { UserProfile } from '@/types/settings'

export async function fetchUserProfile(): Promise<UserProfile | null> {
  const response = await fetch(`${process.env.NEXT_AUTH_URL}/backend-api/users/me`)
  if (response.ok) {
    return (await response.json()) as UserProfile
  }
  return null
}
