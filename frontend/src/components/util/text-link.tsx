import { handleOpenLinkInNewTab } from '@/helpers/navigation'
import { HTMLAttributes, MouseEvent } from 'react'
import { twMerge } from 'tailwind-merge'

type TextLinkProps = HTMLAttributes<HTMLDivElement> & {
  href: string
}

export default function TextLink(props: TextLinkProps) {
  const handleOpenLink = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
    e.preventDefault()
    e.stopPropagation()
    handleOpenLinkInNewTab(props.href)
  }

  return (
    <div
      className={twMerge('cursor-pointer underline text-blue-600 text-xs line-clamp-1', props.className)}
      onClick={handleOpenLink}
    >
      {props.href}
    </div>
  )
}
