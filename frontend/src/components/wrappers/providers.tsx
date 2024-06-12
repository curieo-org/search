import { LayoutProps } from '@/app/layout'
import { ThemeProvider } from 'next-themes'
import React from 'react'
import ClientProviders from '@/components/wrappers/client-providers'
import { auth } from '@/auth'
import { SessionProvider } from 'next-auth/react'

export default async function Providers({ children }: LayoutProps) {
  const session = await auth()
  if (session?.user) {
    session.user = {
      name: session.user.name,
      email: session.user.email,
      image: session.user.image,
    }
  }
  return (
    <SessionProvider session={session}>
      <ThemeProvider attribute="class" defaultTheme="dark" enableSystem={false}>
        <ClientProviders>{children}</ClientProviders>
      </ThemeProvider>
    </SessionProvider>
  )
}
