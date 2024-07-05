import { HTMLAttributes } from 'react'
import Markdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { twMerge } from 'tailwind-merge'

type SearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  response: string
}

export default function SearchResponse(props: SearchResponseProps) {
  return (
    <div className={twMerge('flex flex-col gap-y-2', props.className)}>
      {props.response.split('\\n').map((paragraph, index) => (
        <div key={`response-paragraph-${index}`} style={{ animation: `fade-in ${Math.min(1 + index * 0.5, 5)}s` }}>
          <Markdown className="text-sm text-white/80" remarkPlugins={[remarkGfm]}>
            {paragraph}
          </Markdown>
        </div>
      ))}
    </div>
  )
}
