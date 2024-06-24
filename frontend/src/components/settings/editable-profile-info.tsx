import { ChangeEvent, HTMLAttributes, useState } from 'react'
import { twMerge } from 'tailwind-merge'
import { Span } from '../lib/typography'
import { Input } from '../lib/form'

type EditableProfileInfoProps = HTMLAttributes<HTMLDivElement> & {
  label: string
  value: string
  setValue: (event: ChangeEvent<HTMLInputElement>) => void
  errorMessage?: string
}

export default function EditableProfileInfo(props: EditableProfileInfoProps) {
  return (
    <div className={twMerge('', props.className)}>
      <Span className="text-sm text-custom-gray-50">{props.label}</Span>
      <Input
        containerClass="mt-4"
        className="bg-transparent text-custom-gray-150 border border-white/20"
        value={props.value}
        onChange={props.setValue}
        errorMessage={props.errorMessage}
      />
    </div>
  )
}
