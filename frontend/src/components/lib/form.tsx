'use client'

import classNames from 'classnames'
import { InputHTMLAttributes, ReactNode, TextareaHTMLAttributes, forwardRef, useState } from 'react'
import { MdOutlineVisibility, MdOutlineVisibilityOff } from 'react-icons/md'
import { twMerge } from 'tailwind-merge'

const defaultTextInputClasses =
  'flex w-full rounded-md border border-custom-blue-600 bg-white resize-none px-4 py-2.5 text-base text-input-default placeholder:text-input-placeholder disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-custom-purple-900 focus-visible:ring-offset-0'

export type TextareaProps = TextareaHTMLAttributes<HTMLTextAreaElement> & {
  button?: ReactNode
  containerClass?: string
  innerContainerClass?: string
}
const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ className, containerClass, innerContainerClass, button, ...props }, ref) => {
    const hasButton = !!button

    return (
      <div className={twMerge('relative w-full', containerClass)}>
        <div className={twMerge('w-full', innerContainerClass)}>
          <textarea className={twMerge(defaultTextInputClasses, className)} ref={ref} {...props} />
          {!!button && button}
        </div>
      </div>
    )
  }
)
Textarea.displayName = 'Textarea'

export type InputProps = InputHTMLAttributes<HTMLInputElement> & {
  icon?: ReactNode
  containerClass?: string
  errorMessage?: string
}
const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ className, containerClass, errorMessage, icon, ...props }, ref) => {
    const hasIcon = !!icon

    return (
      <div className={twMerge('relative w-full', containerClass)}>
        <input
          className={twMerge(defaultTextInputClasses, classNames('h-12', { 'pr-10': hasIcon }), className)}
          ref={ref}
          {...props}
        />
        <div className="absolute right-4 top-5">{!!icon && icon}</div>
        {errorMessage && <span className="text-xs italic text-red-600">{errorMessage}</span>}
      </div>
    )
  }
)
Input.displayName = 'Input'

const PasswordInput = forwardRef<HTMLInputElement, InputProps>(({ className, ...props }, ref) => {
  const [showPassword, setShowPassword] = useState(false)
  return (
    <Input
      ref={ref}
      className={className}
      {...props}
      placeholder="Password"
      type={showPassword ? 'text' : 'password'}
      icon={
        showPassword ? (
          <MdOutlineVisibility
            className="text-gray-500 cursor-pointer"
            size={16}
            onClick={() => setShowPassword(false)}
          />
        ) : (
          <MdOutlineVisibilityOff
            className="text-gray-500 cursor-pointer"
            size={16}
            onClick={() => setShowPassword(true)}
          />
        )
      }
    />
  )
})
PasswordInput.displayName = 'PasswordInput'

export { Input, PasswordInput, Textarea }
