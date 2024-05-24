import NextAuth, { NextAuthConfig } from 'next-auth'
import Cognito from 'next-auth/providers/cognito'
import Google from 'next-auth/providers/google'
import { adapter } from '@/middleware/auth/adapter'
import Credentials from 'next-auth/providers/credentials'
import { AuthParams, AuthResponse } from '@/types/auth'
import { BackendAPIClient } from '@/helpers/backend-api-client'
import type { JSX } from 'preact'

const config: NextAuthConfig = {
  theme: {
    logo: `${process.env.NEXTAUTH_URL}/images/curieo-logo.svg`,
    colorScheme: 'auto', // "auto" | "dark" | "light"
    brandColor: '#9655FDFF', // Hex color code
    buttonText: '#9655FDFF', // Hex color code
  },
  adapter: adapter,
  providers: [
    Credentials({
      // You can specify which fields should be submitted, by adding keys to the `credentials` object.
      // e.g. domain, username, password, 2FA token, etc.
      credentials: {
        username: { label: 'email' },
        password: { label: 'password', type: 'password' },
      },
      authorize: async credentials => {
        function login(params: AuthParams): Promise<AuthResponse> {
          return new Promise(async function (resolve, reject) {
            const payload = new URLSearchParams()
            payload.append('username', params.username.trim())
            payload.append('password', params.password)
            BackendAPIClient.post('/auth/signin', payload)
              .then(res => {
                resolve(res.data as AuthResponse)
              })
              .catch(err => reject(err))
          })
        }

        let user = await login(credentials as AuthParams)
        console.log(user)

        if (!user) {
          throw new Error('User not found.')
        }
        return {}
      },
    }),
    Google({
      clientId: process.env.GOOGLE_CLIENT_ID,
      clientSecret: process.env.GOOGLE_CLIENT_SECRET,
    }),
  ], // Cognito, Google],
  basePath: '/auth',
  callbacks: {
    authorized({ request, auth }) {
      const { pathname } = request.nextUrl
      if (pathname === '/middleware-example') return !!auth
      return true
    },
    jwt({ token, trigger, session, account }) {
      if (trigger === 'update') token.name = session.user.name
      return token
    },
    async session({ session, token }) {
      if (token?.accessToken) {
        //session.accessToken = token.accessToken
      }
      return session
    },
  },
  experimental: {
    enableWebAuthn: true,
  },
  debug: process.env.NODE_ENV !== 'production',
}
export const { handlers, auth, signIn, signOut } = NextAuth(config)
