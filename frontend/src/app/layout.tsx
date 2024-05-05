import App from '@/components/wrappers/app'
import Providers, { PosthogProvider } from '@/components/wrappers/providers'
import { appDescription, appTitle } from '@/constants/app'
import '@/styles/globals.css'
import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import { ReactNode } from 'react'
import 'react-toastify/dist/ReactToastify.css'
import dynamic from 'next/dynamic'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: appTitle,
  description: appDescription,
}

export type LayoutProps = Readonly<{
  children: ReactNode
}>

const PostHogPageView = dynamic(() => import('@/services/posthog'), {
  ssr: false,
})

export default function RootLayout({ children }: LayoutProps) {
  return (
    <html lang="en">
      <PosthogProvider>
        <head>
          <link rel="icon" type="image/x-icon" href="/images/curieo-logo.svg" />
        </head>
        <body className={inter.className} suppressHydrationWarning={true}>
          <Providers>
            <App>
              <PostHogPageView />
              {children}
            </App>
          </Providers>
        </body>
      </PosthogProvider>
    </html>
  )
}
