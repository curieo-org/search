import posthog from 'posthog-js'
import { PostHogProvider } from 'posthog-js/react'
import { LayoutProps } from '@/app/layout'
import dynamic from 'next/dynamic'

const posthog_enabled = process.env.NEXT_PUBLIC_POSTHOG_KEY != null

if (typeof window !== 'undefined' && posthog_enabled) {
  posthog.init(process.env.NEXT_PUBLIC_POSTHOG_KEY || '', {
    api_host: process.env.NEXT_PUBLIC_POSTHOG_API_HOST || '',
    ui_host: process.env.NEXT_PUBLIC_POSTHOG_UI_HOST || '',
    capture_pageview: false,
  })
}

const PostHogPageView = dynamic(() => import('@/components/analytics/posthog'), {
  ssr: false,
})

export default function AnalyticsProvider({ children }: LayoutProps) {
  return (
    <PostHogProvider client={posthog}>
      <PostHogPageView />
      {children}
    </PostHogProvider>
  )
}
