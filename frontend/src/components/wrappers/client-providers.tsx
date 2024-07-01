'use client'

import { LayoutProps } from '@/app/layout'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import AnalyticsProvider from './analytics-provider'
import _ from 'lodash'
import { toast } from 'react-toastify'
import React from 'react'

export default function ClientProviders({ children }: LayoutProps) {
  const [queryClient] = React.useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            // With SSR, we usually want to set some default staleTime
            // above 0 to avoid re-fetching immediately on the client
            staleTime: 60 * 1000,
          },
          mutations: {
            onError(error: any) {
              const message = _.get(error, 'response.data.message', 'Something went wrong! Please try again')
              toast.error(message)
            },
          },
        },
      })
  )
  return (
    <AnalyticsProvider>
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    </AnalyticsProvider>
  )
}
