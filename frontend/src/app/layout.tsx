import App from '@/components/wrappers/app'
import Providers from '@/components/wrappers/providers'
import { appDescription, appTitle } from '@/constants/app'
import '@/styles/globals.css'
import type { Metadata } from 'next'
import { Manrope } from 'next/font/google'
import { ReactNode } from 'react'
import 'react-toastify/dist/ReactToastify.css'

const manrope = Manrope({ subsets: ['latin'] })

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
      <body className={manrope.className} suppressHydrationWarning={true}>
        <Providers>
          <App>{children}</App>
        </Providers>
      </body>
    </html>
  )
}
