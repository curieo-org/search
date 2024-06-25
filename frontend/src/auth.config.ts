import type { Account, NextAuthConfig, Profile, User } from 'next-auth'
import { AdapterUser } from '@auth/core/adapters'
import { CredentialInput } from 'next-auth/providers'
import type { Adapter } from 'next-auth/adapters'
import { passwordErrorMessage } from '@/constants/messages'

export const authConfig: NextAuthConfig = {
  theme: {
    logo: `${process.env.NEXTAUTH_URL}/images/curieo-logo.svg`,
    colorScheme: 'auto', // "auto" | "dark" | "light"
    brandColor: '#9655FDFF', // Hex color code
    buttonText: '#9655FDFF', // Hex color code
  },
  pages: {
    signIn: '/auth/signin',
    signOut: '/auth/signout',
    error: '/auth/error',
    // TODO: verifyRequest: '/auth/verify-request',
    // TODO: newUser: '/auth/new-user',
  },
  providers: [], // Set in auth.ts
  callbacks: {
    async signIn({
      user,
      account,
      profile,
      email,
      credentials,
    }: {
      user: User | AdapterUser
      account: Account | null
      profile?: Profile
      email?: {
        verificationRequest?: boolean
      }
      credentials?: Record<string, CredentialInput>
    }) {
      switch (account?.type) {
        case 'oidc':
          console.debug('oidc')
          break
        case 'oauth':
          console.debug('oauth')
          break
        case 'email':
          console.debug('email')
          break
        case 'credentials':
          console.debug('credentials')
          break
        case 'webauthn':
          console.debug('webauthn')
          break
      }
      console.debug('SignIn:')
      console.debug(user)
      console.debug(account)
      console.debug(profile)
      console.debug(email)
      console.debug(credentials)

      return true
    },
    authorized({ auth, request: { nextUrl } }) {
      let isLoggedIn = !!auth?.user
      if (!isLoggedIn) {
        return Response.redirect(new URL('auth/signin', nextUrl))
      }
      return true
    },
  },
  debug: process.env.NODE_ENV !== 'production',
  adapter: httpAdapter(),
}

export function httpAdapter(): Adapter {
  return {
    async createUser(user) {
      return user
    },
    async getUser(id) {
      try {
        return null
      } catch (error) {
        return null
      }
    },
    async getUserByEmail(email) {
      try {
        return null
      } catch (error) {
        return null
      }
    },
    async getUserByAccount(payload) {
      try {
        return null
      } catch (error) {
        return null
      }
    },
    async updateUser(user) {
      return { id: 1 }
    },
    async deleteUser(userId) {
      try {
        return null
      } catch (error) {
        return null
      }
    },
    async linkAccount(account) {
      try {
        return null
      } catch (error) {
        return null
      }
    },
    async unlinkAccount(args) {
      return undefined
    },
    async createSession(session) {
      return session
    },
    async getSessionAndUser(sessionToken) {
      try {
        return null
      } catch (error) {
        return null
      }
    },
    async updateSession(session) {
      try {
        return null
      } catch (error) {
        return null
      }
    },
    async deleteSession(sessionToken) {
      return null
    },
    async createVerificationToken(verificationToken) {
      try {
        return null
      } catch (error) {
        return null
      }
    },
    async useVerificationToken(params) {
      try {
        return null
      } catch (error) {
        return null
      }
    },
  }
}
