"use client"

import { Analytics } from "@vercel/analytics/react"
import { AxiomWebVitals } from "next-axiom"
import { Toaster } from "react-hot-toast"
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


export function Providers() {
  return (
    <>
      <Analytics />
      <AxiomWebVitals />
      <Toaster position="top-right" />
    </>
  )
}
