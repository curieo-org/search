import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import { P } from '../lib/typography'

type SearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  response: string
}

export default function SearchResponse(props: SearchResponseProps) {
  return (
    <div className={twMerge('flex flex-col gap-y-2', props.className)}>
      {props.response.split('\\n').map((paragraph, index) => (
        <P
          className="pr-4 text-sm text-white/80"
          style={{ animation: `fade-in ${Math.min(1 + index * 0.5, 5)}s` }}
          key={`response-paragraph-${index}`}
        >
          {paragraph}
        </P>
      ))}
    </div>
  )
}
