import { HTMLAttributes } from 'react'
import { BiError } from 'react-icons/bi'
import { P } from '../lib/typography'

type ErrorPageProps = HTMLAttributes<HTMLDivElement> & {
  message?: string
}

export default function ErrorPage(props: ErrorPageProps) {
  return (
    <div className="flex flex-col justify-center items-center w-full h-[80vh]">
      <BiError size={72} className="text-red-600 mb-1" />
      <P className="text-red-600 text-center text-lg font-semibold mb-2">Error!</P>
      <P className="text-center">{props.message ?? `Something went wrong!`}</P>
    </div>
  )
}
