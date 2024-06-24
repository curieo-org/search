'use server'

import { UserProfile } from '@/types/settings'
import { curieoFetch } from './fetch'

export async function fetchUserProfile(): Promise<UserProfile> {
  const response = await curieoFetch('/users/me')
  if (response.ok) {
    return (await response.json()) as UserProfile
  }
  throw new Error('Could not retrieve user profile')
}
