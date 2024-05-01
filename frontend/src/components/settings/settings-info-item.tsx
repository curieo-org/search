import { HTMLAttributes } from 'react'
import PencilIcon from '../icons/pencil'
import { Span } from '../lib/typography'

type SettingsInfoItemProps = HTMLAttributes<HTMLDivElement> & {
  label: string
  value: string
}

export default function SettingsInfoItem(props: SettingsInfoItemProps) {
  return (
    <div className="w-full p-4 flex items-center justify-between">
      <div className="flex flex-col items-start gap-y-2">
        <Span>{props.label}</Span>
        <Span className="text-input-placeholder">{props.value}</Span>
      </div>
      <Span className="cursor-pointer">
        <PencilIcon size={16} />
      </Span>
    </div>
  )
}
