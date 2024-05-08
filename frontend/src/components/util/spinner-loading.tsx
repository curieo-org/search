import { colors } from '@/styles/colors'
import { HTMLAttributes } from 'react'
import { SpinnerCircular } from 'spinners-react'
import { gray } from 'tailwindcss/colors'
import { P } from '../lib/typography'

type SpinnerLoadingProps = HTMLAttributes<HTMLDivElement> & {
  message?: string
}

export default function SpinnerLoading(props: SpinnerLoadingProps) {
  const { primary } = colors

  return (
    <div className="flex flex-col justify-center items-center w-full h-[80vh]">
      <SpinnerCircular color={primary} secondaryColor={gray[200]} />
      {props.message && <P className="text-center text-lg font-semibold m-1">{props.message}</P>}
    </div>
  )
}
