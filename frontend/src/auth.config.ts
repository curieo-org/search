import type { NextAuthConfig } from 'next-auth'

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
    async signIn() {
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
}
