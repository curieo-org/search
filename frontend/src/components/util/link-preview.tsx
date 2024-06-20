'use client'

import { handleOpenLinkInNewTab } from '@/utils/navigation'
import { HTMLAttributes, MouseEvent, useEffect, useState } from 'react'
import { twMerge } from 'tailwind-merge'
import { H3, P } from '../lib/typography'
import SourceSkeleton from '../skeletons/source-skeleton'
import TextLink from './text-link'
import { Source } from '@/types/search'
import BookIcon from '../icons/book'

type LinkPreviewProps = HTMLAttributes<HTMLDivElement> & {
  source: Source
}

export default function LinkPreview(props: LinkPreviewProps) {
  const { source, className, ...rest } = props
  const handleOpenLink = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
    e.preventDefault()
    e.stopPropagation()
    handleOpenLinkInNewTab(source.url)
  }

  return (
    <div {...rest} onClick={handleOpenLink} className={twMerge('cursor-pointer', className)}>
      <div className="flex flex-col pt-1">
        <BookIcon size={16} />
      </div>
      <div className="flex flex-col">
        <H3 className="mb-2 text-sm text-opacity-80 font-semibold line-clamp-2">{source.title}</H3>
        <P className="text-2xs text-opacity-70 line-clamp-4">{source.description}</P>
      </div>
    </div>
  )
}
