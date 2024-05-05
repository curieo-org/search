'use client'

import { LayoutProps } from '@/app/layout'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import _ from 'lodash'
import { ThemeProvider } from 'next-themes'
import { toast } from 'react-toastify'
import posthog from "posthog-js"
import { PostHogProvider } from 'posthog-js/react'

const posthog_enabled = process.env.NEXT_PUBLIC_POSTHOG_KEY != null

if (typeof window !== 'undefined' && posthog_enabled) {
  posthog.init(process.env.NEXT_PUBLIC_POSTHOG_KEY || '', {
    api_host: process.env.NEXT_PUBLIC_POSTHOG_API_HOST || '',
    ui_host: process.env.NEXT_PUBLIC_POSTHOG_UI_HOST || '',
    capture_pageview: false,
  })
}

export function PosthogProvider({
  children,
}: {
  children: React.ReactNode
}) {
  return <PostHogProvider client={posthog}>{children}</PostHogProvider>
}

const queryClient = new QueryClient({
  defaultOptions: {
    mutations: {
      onError(error: any, variables, context) {
        const message = _.get(error, 'response.data.message', 'Something wrong ! Please try again')
        toast.error(message)
      },
    },
  },
})

export default function Providers({ children }: LayoutProps) {
  return (
    <ThemeProvider attribute="class" defaultTheme="dark" enableSystem={false}>
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    </ThemeProvider>
  )
}
