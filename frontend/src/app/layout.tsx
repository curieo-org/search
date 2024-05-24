import App from '@/components/wrappers/app'
import Providers from '@/components/wrappers/providers'
import { appDescription, appTitle } from '@/constants/app'
import '@/styles/globals.css'
import type { Metadata } from 'next'
import { Onest } from 'next/font/google'
import { ReactNode } from 'react'
import 'react-toastify/dist/ReactToastify.css'
import { SessionProvider } from 'next-auth/react'
import { auth } from '@/middleware/auth/config'

const onest = Onest({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: appTitle,
  description: appDescription,
}

export type LayoutProps = Readonly<{
  children: ReactNode
}>

export default async function RootLayout({ children }: LayoutProps) {
  return (
    <html lang="en">
      <head>
        <link rel="icon" type="image/x-icon" href="/images/curieo-logo.svg" />
        <title>Curieo Search</title>
      </head>
      <body className={onest.className} suppressHydrationWarning={true}>
        <Providers>
          <App>{children}</App>
        </Providers>
      </body>
    </html>
  )
}
