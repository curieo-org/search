import { ButtonHTMLAttributes, Fragment, ReactNode } from 'react'
import { FaSpinner } from 'react-icons/fa'
import { twMerge } from 'tailwind-merge'

export type ButtonProps = {
  label?: string
  iconLeft?: ReactNode
  iconRight?: ReactNode
  isLoading?: boolean
} & ButtonHTMLAttributes<HTMLButtonElement>

export function Button(props: ButtonProps) {
  const { label, iconLeft, iconRight, isLoading, className, ...rest } = props
  return (
    <button
      type="button"
      {...rest}
      className={twMerge(
        'relative bg-primary hover:bg-primary/90 flex gap-x-2 h-12 w-auto shrink-0 items-center justify-center rounded-md px-4 xl:px-6 py-2 xl:py-3 text-sm xl:text-base font-medium text-white transition-all duration-100',
        className
      )}
    >
      {!!isLoading && <FaSpinner className="animate-spin" />}
      {!isLoading && (
        <Fragment>
          {!!iconLeft && iconLeft}
          {!!label && <span className="line-clamp-1">{label}</span>}
          {!!iconRight && iconRight}
        </Fragment>
      )}
    </button>
  )
}
