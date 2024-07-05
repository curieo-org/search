import { HTMLAttributes, OlHTMLAttributes } from 'react'
import Markdown from 'react-markdown'
import { twMerge } from 'tailwind-merge'

type SearchResponseProps = HTMLAttributes<HTMLDivElement> & {
  response: string
}

const components = {
  ol(props: OlHTMLAttributes<HTMLOListElement>) {
    return (
      <ol className="list-decimal pl-4" start={props.start}>
        {props.children}
      </ol>
    )
  },
  ul(props: HTMLAttributes<HTMLUListElement>) {
    return <ul className="list-disc pl-8">{props.children}</ul>
  },
  h3(props: HTMLAttributes<HTMLHeadingElement>) {
    return <h3 className="font-bold text-lg">{props.children}</h3>
  },
}

export default function SearchResponse(props: SearchResponseProps) {
  return (
    <div className={twMerge('flex flex-col gap-y-2', props.className)}>
      {props.response.split('\\n').map((paragraph, index) => (
        <Markdown key={`response-paragraph-${index}`} className="text-base text-white/80" components={components}>
          {paragraph.replaceAll('    -', '   -')}
        </Markdown>
      ))}
    </div>
  )
}
