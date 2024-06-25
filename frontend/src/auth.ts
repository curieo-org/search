import { authConfig } from '@/auth.config'
import { AuthParams } from '@/types/auth'
import NextAuth, { AuthError, Session, User } from 'next-auth'
import { AccessDenied } from '@auth/core/errors'
import Credentials from 'next-auth/providers/credentials'
import { cookies } from 'next/headers'
import Google from '@auth/core/providers/google'
import { Provider } from '@auth/core/providers'
import Apple from '@auth/core/providers/apple'
import { curieoCredentialsSignIn } from '@/actions/auth'

const providers: Provider[] = [
  Credentials({
    // You can specify which fields should be submitted, by adding keys to the `credentials` object.
    // e.g. domain, username, password, 2FA token, etc.
    credentials: {
      username: { label: 'username', type: 'text' },
      password: { label: 'password', type: 'password' },
    },
    authorize: async (credentials, req) => {
      try {
        const response = await curieoCredentialsSignIn(credentials as AuthParams)
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
