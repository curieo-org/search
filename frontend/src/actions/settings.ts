'use server'

import { UpdateProfileBody, UserProfile } from '@/types/settings'
import { curieoFetch } from './fetch'

export async function fetchUserProfile(): Promise<UserProfile> {
  const response = await curieoFetch('/users/me')
  if (response.ok) {
    return (await response.json()) as UserProfile
  }
  throw new Error('Could not retrieve user profile')
}

export async function updateUserProfile(payload: UpdateProfileBody): Promise<UserProfile> {
  const response = await curieoFetch('/users/me', {
    method: 'PATCH',
    body: JSON.stringify(payload),
  })
  if (response.ok) {
    return (await response.json()) as UserProfile
  }
  throw new Error('Could not update user profile')
}
