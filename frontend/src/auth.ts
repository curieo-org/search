import { authConfig } from '@/auth.config'
import { AuthParams, AuthResponse } from '@/types/auth'
import { encodeAsUrlSearchParams, formToUrlParams } from '@/utils'
import NextAuth, { AuthError, Session, User } from 'next-auth'
import { AccessDenied } from '@auth/core/errors'
import Credentials from 'next-auth/providers/credentials'
import { cookies } from 'next/headers'
import { curieoFetch } from '@/actions/fetch'
import { ResponseCookies } from 'next/dist/server/web/spec-extension/cookies'
import Google from '@auth/core/providers/google'
import { Provider } from '@auth/core/providers'
import Apple from '@auth/core/providers/apple'

const providers: Provider[] = [
  Credentials({
    // You can specify which fields should be submitted, by adding keys to the `credentials` object.
    // e.g. domain, username, password, 2FA token, etc.
    credentials: {
      username: { label: 'username', type: 'text' },
      password: { label: 'password', type: 'password' },
    },
    authorize: async (credentials, req) => {
      async function login(p: AuthParams): Promise<AuthResponse | null> {
        'use server'
        const response = await curieoFetch('/auth/login', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
          },
          body: encodeAsUrlSearchParams({
            username: p.username.trim(),
            password: p.password,
          }),
        })
        if (response.ok) {
          const setCookies = new ResponseCookies(response.headers)
          setCookies.getAll().forEach(cookie => cookies().set(cookie))
          return (await response.json()) as AuthResponse
        }
        return null
      }

      try {
        const response = await login(credentials as AuthParams)
        if (response !== null) {
          return {
            id: response.user_id,
            name: response.email,
          } as User
        }
      } catch (e) {
        if (e instanceof AuthError) {
          throw e
        }
      }
      throw new AccessDenied('Could not log in')
    },
  }),
  Google,
  Apple,
]

export const {
  handlers: { GET, POST },
  auth: next_auth,
  signIn,
  signOut,
} = NextAuth({
  ...authConfig,
  providers: providers,
})

export async function auth(): Promise<Session | null> {
  const session = await next_auth()
  // Strips information from the returned user as a secondary defense against oversharing user info with the client
  if (session?.user) {
    session.user = {
      name: session.user.name,
      email: session.user.email,
      image: session.user.image,
    }
  }
  return session
}

export class SignUpError extends AuthError {
  static type = 'SignUpError'
}

export async function signUp(f: FormData): Promise<AuthResponse> {
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

export function getCsrfToken() {
  return cookies().get('next-auth.csrf-token')?.value.split('|')[0]
}

export const oauthProviders = providers
  .filter(p => p.name != 'Credentials')
  .map(provider => {
    if (typeof provider === 'function') {
      const providerData = provider()
      return { id: providerData.id, name: providerData.name }
    } else {
      return { id: provider.id, name: provider.name }
    }
  })
