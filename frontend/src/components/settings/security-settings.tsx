import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'

type SecuritySettingsProps = HTMLAttributes<HTMLDivElement> & {}

export default function SecuritySettings(props: SecuritySettingsProps) {
  return <div className={twMerge('', props.className)}>SecuritySettings</div>
}
