'use server'

import { signIn, SignUpError } from '@/auth'
import { AuthError } from 'next-auth'
import { redirect } from 'next/navigation'
import { AuthParams, AuthResponse } from '@/types/auth'
import { curieoFetch } from '@/actions/fetch'
import { encodeAsUrlSearchParams, formToUrlParams } from '@/utils'
import { ResponseCookies } from 'next/dist/server/web/spec-extension/cookies'
import { cookies } from 'next/headers'

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
    await curieoCredentialsSignUp(formData)
    return redirect('/auth/signin')
  } catch (error) {
    if (error instanceof AuthError) {
      return redirect(`/auth/error?error=${error.type}`)
    }
    throw error
  }
}

export async function curieoCredentialsSignIn({ username, password }: AuthParams): Promise<AuthResponse | null> {
  /**
   * Signs in to the search server using credentials and assigns the resulting session cookies correctly.
   */
  const response = await curieoFetch('/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: encodeAsUrlSearchParams({
      username: username.trim(),
      password: password,
    }),
  })
  if (response.ok) {
    const setCookies = new ResponseCookies(response.headers)
    setCookies.getAll().forEach(cookie => cookies().set(cookie))
    return (await response.json()) as AuthResponse
  }
  return null
}

export async function curieoCredentialsSignUp(f: FormData): Promise<AuthResponse> {
  // If email is not set we use username
  if (!f.has('email')) {
    f.set('email', f.get('username') || '')
  }
  let response = await curieoFetch('/auth/register', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: formToUrlParams(f),
  })
  if (response.ok) {
    return (await response.json()) as AuthResponse
  }
  throw new SignUpError('Could not sign up')
}

export async function curieoOAuthSignInCallback({ email, accessToken }: any): Promise<AuthResponse | null> {
  /**
   * Signs in to the search server using credentials and assigns the resulting session cookies correctly.
   */
  const response = await curieoFetch('/auth/oauth_callback', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: encodeAsUrlSearchParams({
      username: username.trim(),
      password: password,
    }),
  })
  if (response.ok) {
    const setCookies = new ResponseCookies(response.headers)
    setCookies.getAll().forEach(cookie => cookies().set(cookie))
    return (await response.json()) as AuthResponse
  }
  return null
}
