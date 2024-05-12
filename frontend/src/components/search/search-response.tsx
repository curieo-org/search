import { HTMLAttributes } from 'react'
import { P } from '../lib/typography'
import { twMerge } from 'tailwind-merge'

type SearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  response: string
}

export default function SearchResponse(props: SearchResponseProps) {
  return (
    <div className={twMerge('flex flex-col gap-y-6 max-h-[520px] overflow-y-scroll', props.className)}>
      {props.response.split('\n').map((paragraph, index) => (
        <P className="pr-4" style={{ animation: `fade-in ${1 + index}s` }} key={`response-paragraph-${index}`}>
          {paragraph}
        </P>
      ))}
    </div>
  )
}
