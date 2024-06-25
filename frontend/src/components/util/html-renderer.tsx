import DOMPurify from 'dompurify'
import { HTMLAttributes } from 'react'

type HtmlRendererProps = HTMLAttributes<HTMLDivElement> & {
  htmlString: string
}

export default function HtmlRenderer(props: HtmlRendererProps) {
  const sanitizedHtmlString = DOMPurify.sanitize(props.htmlString)

  return <div className={props.className} dangerouslySetInnerHTML={{ __html: sanitizedHtmlString }} />
}
