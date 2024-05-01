import { ButtonHTMLAttributes, ReactNode } from 'react'
import { FaSpinner } from 'react-icons/fa'
import { twMerge } from 'tailwind-merge'

export type ButtonProps = {
  label?: string
  iconLeft?: ReactNode
  iconRight?: ReactNode
  isLoading?: boolean
  isDisabled?: boolean
} & ButtonHTMLAttributes<HTMLButtonElement>

export function Button(props: ButtonProps) {
  return (
    <button
      type="button"
      disabled={props.isDisabled ?? false}
      onClick={props.onClick}
      className={twMerge(
        'relative bg-primary hover:bg-primary/90 flex h-12 w-auto shrink-0 items-center justify-center rounded-md px-6 py-3 text-base font-medium text-white transition-all duration-300',
        props.className
      )}
    >
      {!!props.isLoading && <FaSpinner className="animate-spin" />}
      {!props.isLoading && (
        <div className="flex items-center justify-center gap-x-2">
          {!!props.iconLeft && props.iconLeft}
          {!!props.label && <span className="line-clamp-1">{props.label}</span>}
          {!!props.iconRight && props.iconRight}
        </div>
      )}
    </button>
  )
}
