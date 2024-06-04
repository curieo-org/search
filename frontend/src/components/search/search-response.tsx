import classNames from 'classnames'
import { HTMLAttributes, useState } from 'react'
import { twMerge } from 'tailwind-merge'
import { P } from '../lib/typography'

type SearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  response: string
}

export default function SearchResponse(props: SearchResponseProps) {
  const [showScrollbar, setShowScrollbar] = useState(false)

  return (
    <div
      className={twMerge(
        classNames('flex flex-col gap-y-6 max-h-[520px] overflow-y-scroll', {
          'scrollbar-hidden pr-1.5': !showScrollbar,
          'scrollbar-visible': showScrollbar,
        }),
        props.className
      )}
      onMouseEnter={() => setShowScrollbar(true)}
      onMouseLeave={() => setShowScrollbar(false)}
    >
      {props.response.split('\n').map((paragraph, index) => (
        <P className="pr-4" style={{ animation: `fade-in ${1 + index}s` }} key={`response-paragraph-${index}`}>
          {paragraph}
        </P>
      ))}
    </div>
  )
}
