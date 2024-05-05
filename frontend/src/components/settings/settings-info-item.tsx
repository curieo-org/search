'use client'

import { HTMLAttributes, useState } from 'react'
import PencilIcon from '../icons/pencil'
import { Span } from '../lib/typography'
import { Input } from '../lib/form'
import { Button } from '../lib/button'

type SettingsInfoItemProps = HTMLAttributes<HTMLDivElement> & {
  label: string
  value: string
}

export default function SettingsInfoItem(props: SettingsInfoItemProps) {
  const [isEditBoxOpen, setIsEditBoxOpen] = useState(false)
  const [editedValue, setEditedValue] = useState(props.value)

  return (
    <div className="w-full p-4 flex items-center justify-between bg-transparent">
      <Span>{props.label}</Span>
      {!isEditBoxOpen && (
        <div className="flex items-center gap-x-4">
          <Span className="text-typography-light/75 dark:text-typography-dark/75">{props.value}</Span>
          <Span className="cursor-pointer" onClick={() => setIsEditBoxOpen(true)}>
            <PencilIcon size={16} />
          </Span>
        </div>
      )}
      {isEditBoxOpen && (
        <div className="flex flex-col gap-y-1">
          <Input
            className="rounded-full bg-transparent h-8 border-typography-light/20 dark:border-typography-dark/20 text-typography-light/75 dark:text-typography-dark/75 focus-visible:ring-0"
            value={editedValue}
            onChange={e => setEditedValue(e.target.value)}
          />
          <div className="flex flex-row-reverse gap-x-1 items-center">
            <Button
              className="h-8 py-0 px-3 bg-white/10 hover:bg-white/20 text-xs"
              label="Cancel"
              onClick={() => setIsEditBoxOpen(false)}
            />
            <Button
              className="h-8 py-0 px-3 bg-white/10 hover:bg-white/20 text-xs"
              label="Save"
              onClick={() => setIsEditBoxOpen(false)}
            />
          </div>
        </div>
      )}
    </div>
  )
}
