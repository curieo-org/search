import type { NextAuthConfig } from 'next-auth'

export const authConfig: NextAuthConfig = {
  theme: {
    logo: `${process.env.NEXTAUTH_URL}/images/curieo-logo.svg`,
    colorScheme: 'auto', // "auto" | "dark" | "light"
    brandColor: '#9655FDFF', // Hex color code
    buttonText: '#9655FDFF', // Hex color code
  },
  //adapter: adapter,
  pages: {
    signIn: '/auth/signin',
    signOut: '/auth/signout',
    error: '/auth/error',
    verifyRequest: '/auth/verify-request',
    newUser: '/auth/new-user',
  },
  providers: [], // Set in auth.ts
  callbacks: {
    async signIn({ user, account, profile, email, credentials }) {
      console.debug(user, account, profile, email, credentials)
      return user.id == '1' ? true : false
    },
    authorized({ auth, request: { nextUrl } }) {
      console.error('oh dear god', auth, nextUrl)
      let isLoggedIn = !!auth?.user
      let isOnDashboard = nextUrl.pathname.startsWith('/')

      if (isOnDashboard) {
        return isLoggedIn
      } else if (!isLoggedIn) {
        return Response.redirect(new URL('auth/signin', nextUrl))
      }

      return true
    },
  },
  debug: process.env.NODE_ENV !== 'production',
}
