import App from '@/components/wrappers/app'
import Providers from '@/components/wrappers/providers'
import '@/styles/globals.css'
import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import { ReactNode } from 'react'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'Curieo Frontend',
  description: 'One stop search engine to fast-track your research.',
}

export type LayoutProps = Readonly<{
  children: ReactNode
}>

export default function RootLayout({ children }: LayoutProps) {
  return (
    <html lang="en">
      <head>
        <link rel="icon" type="image/x-icon" href="/images/curieo-logo.svg" />
      </head>
      <body className={inter.className} suppressHydrationWarning={true}>
        <Providers>
          <App>{children}</App>
        </Providers>
      </body>
    </html>
  )
}
