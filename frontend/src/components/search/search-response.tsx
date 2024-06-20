import classNames from 'classnames'
import { HTMLAttributes, useState } from 'react'
import { twMerge } from 'tailwind-merge'
import { P } from '../lib/typography'

type SearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  response: string
}

export default function SearchResponse(props: SearchResponseProps) {
  return (
    <div className={twMerge('flex flex-col gap-y-6', props.className)}>
      {props.response.split('\n').map((paragraph, index) => (
        <P
          className="pr-4 text-sm text-white/80"
          style={{ animation: `fade-in ${1 + index}s` }}
          key={`response-paragraph-${index}`}
        >
          {paragraph}
        </P>
      ))}
    </div>
  )
}
