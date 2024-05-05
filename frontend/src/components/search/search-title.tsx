import { HTMLAttributes } from 'react'
import { P } from '../lib/typography'
import { twMerge } from 'tailwind-merge'

type SearchTitleProps = HTMLAttributes<HTMLDivElement> & {
  title: string
}

export default function SearchTitle(props: SearchTitleProps) {
  return (
    <P className={twMerge('text-lg xl:text-2xl font-semibold px-6 py-1 border-l-6 border-primary', props.className)}>
      {props.title}
    </P>
  )
}
