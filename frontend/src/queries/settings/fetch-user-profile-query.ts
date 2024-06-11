'use server'

import { UserProfile } from '@/types/settings'
import { curieoFetch } from '@/actions/fetch'

export async function fetchUserProfile(): Promise<UserProfile | null> {
  const response = await curieoFetch('/users/me')
  if (response.ok) {
    return (await response.json()) as UserProfile
  }
  throw new Error('Could not retrieve user profile')
}
