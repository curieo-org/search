import { HTMLAttributes } from 'react'
import { P } from '../lib/typography'

type SearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  response: string
}

export default function SearchResponse(props: SearchResponseProps) {
  return (
    <div className="flex flex-col gap-y-6 max-h-[520px] overflow-y-scroll">
      {props.response.split('\n').map((paragraph, index) => (
        <P className="pr-4" key={`response-paragraph-${index}`}>
          {paragraph}
        </P>
      ))}
    </div>
  )
}
