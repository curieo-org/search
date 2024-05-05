'use client'

import { LayoutProps } from '@/app/layout'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import _ from 'lodash'
import { ThemeProvider } from 'next-themes'
import { toast } from 'react-toastify'

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
