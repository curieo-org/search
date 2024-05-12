'use client'

import { handleOpenLinkInNewTab } from '@/helpers/navigation'
import { HTMLAttributes, MouseEvent, useEffect, useState } from 'react'
import { twMerge } from 'tailwind-merge'
import { H3, P } from '../lib/typography'
import SourceSkeleton from '../skeletons/source-skeleton'
import TextLink from './text-link'

type LinkPreviewProps = HTMLAttributes<HTMLDivElement> & {
  url: string
}

type PreviewData = { title: string; description: string; image: string }

export default function LinkPreview(props: LinkPreviewProps) {
  const [previewData, setPreviewData] = useState<PreviewData | null>(null)
  const [loading, setLoading] = useState(true)

  const { url, className, ...rest } = props

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch(url)
        const data = await response.text()
        const parser = new DOMParser()
        const doc = parser.parseFromString(data, 'text/html')
        const title = doc.querySelector('title')?.textContent || ''
        const description = doc.querySelector('meta[name="description"]')?.getAttribute('content') || ''
        const image = doc.querySelector('meta[property="og:image"]')?.getAttribute('content') || ''

        setPreviewData({ title, description, image })
        setLoading(false)
      } catch (error) {
        console.error(error)
        setLoading(false)
      }
    }

    fetchData()
  }, [url])

  const handleOpenLink = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
    e.preventDefault()
    e.stopPropagation()
    handleOpenLinkInNewTab(url)
  }

  if (loading) {
    return <SourceSkeleton className={className} />
  }

  if (!previewData) {
    return (
      <div {...rest} className={twMerge('cursor-pointer', className)} onClick={handleOpenLink}>
        <P className="text-sm text-red-500 mb-2">Failed to fetch link preview.</P>
        <TextLink href={url} />
      </div>
    )
  }

  return (
    <div {...rest} onClick={handleOpenLink} className={twMerge('cursor-pointer', className)}>
      <H3 className="mb-2 text-sm text-opacity-80 font-semibold line-clamp-2">{previewData.title}</H3>
      <P className="text-2xs text-opacity-70 line-clamp-4">{previewData.description}</P>
    </div>
  )
}
