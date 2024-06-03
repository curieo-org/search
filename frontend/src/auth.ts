import { authConfig } from '@/auth.config'
import { AuthParams, AuthResponse } from '@/types/auth'
import { encodeAsUrlSearchParams } from '@/utils'
import { BackendAPIClient } from '@/utils/backend-api-client'
import { AxiosResponse } from 'axios'
import NextAuth, { Session, User } from 'next-auth'
import { AccessDenied, OAuthCallbackError } from '@auth/core/errors'
import Credentials from 'next-auth/providers/credentials'
import { cookies } from 'next/headers'

export const {
  handlers: { GET, POST },
  auth: next_auth,
  signIn,
  signOut,
} = NextAuth({
  ...authConfig,
  providers: [
    Credentials({
      // You can specify which fields should be submitted, by adding keys to the `credentials` object.
      // e.g. domain, username, password, 2FA token, etc.
      credentials: {
        username: { label: 'username', type: 'text' },
        password: { label: 'password', type: 'password' },
      },
      authorize: async (credentials, req) => {
        async function login(p: AuthParams): Promise<AxiosResponse<AuthResponse>> {
          return BackendAPIClient.post(
            '/auth/login',
            encodeAsUrlSearchParams({
              username: p.username.trim(),
              password: p.password,
            })
          )
        }
        const response = await login(credentials as AuthParams)
        console.error('response.data', response.data)
        if (response.status === 200 && response.data) {
          return {
            id: response.data.user_id,
            name: response.data.email,
          } as User
        }
        throw new AccessDenied('Could not log in')
      },
    }),
    //Google({
    //  clientId: process.env.GOOGLE_CLIENT_ID,
    //  clientSecret: process.env.GOOGLE_CLIENT_SECRET,
    //}),
  ],
})

export function getCsrfToken() {
  return cookies().get('next-auth.csrf-token')?.value.split('|')[0]
}

export async function auth(): Promise<Session | null> {
  const session = await next_auth()
  if (session?.user) {
    console.error('session', session)
    session.user = {
      name: session.user.name,
      email: session.user.email,
      image: session.user.image,
    }
  }
  return session
}

export async function signUp(p: AuthParams): Promise<AxiosResponse<AuthResponse>> {
  console.error('p', p)
  return BackendAPIClient.post(
    '/auth/signup',
    encodeAsUrlSearchParams({
      username: p.username.trim(),
      password: p.password,
    })
  )
}
