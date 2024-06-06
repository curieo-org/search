'use server'

import { signIn, signUp } from '@/auth'
import { AuthError } from 'next-auth'
import { redirect } from 'next/navigation'

export async function signin(formData: FormData) {
  try {
    await signIn('credentials', formData)
  } catch (error) {
    if (error instanceof AuthError) {
      return redirect(`/auth/error?error=${error.type}`)
    }
    throw error
  }
}

export async function signup(formData: FormData) {
  try {
    await signUp(formData)
    return redirect('/signin')
  } catch (error) {
    if (error instanceof AuthError) {
      return redirect(`/auth/error?error=${error.type}`)
    }
    throw error
  }
}
