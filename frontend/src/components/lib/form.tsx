import classNames from 'classnames'
import { InputHTMLAttributes, ReactNode, TextareaHTMLAttributes, forwardRef, useState } from 'react'
import { MdOutlineVisibility, MdOutlineVisibilityOff } from 'react-icons/md'
import { twMerge } from 'tailwind-merge'

const defaultTextInputClasses =
  'flex w-full rounded-md border border-custom-royal-blue bg-white resize-none px-4 py-2.5 text-base text-input-default placeholder:text-input-placeholder disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-custom-navy-blue focus-visible:ring-offset-0'

export type TextareaProps = TextareaHTMLAttributes<HTMLTextAreaElement> & {
  button?: ReactNode
  containerClass?: string
}
const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ className, containerClass, button, ...props }, ref) => {
    const hasButton = !!button

    return (
      <div className={twMerge('relative w-full', containerClass)}>
        <textarea className={twMerge(defaultTextInputClasses, className)} ref={ref} {...props} />
        {!!button && button}
      </div>
    )
  }
)
Textarea.displayName = 'Textarea'

export type InputProps = InputHTMLAttributes<HTMLInputElement> & { icon?: ReactNode; containerClass?: string }
const Input = forwardRef<HTMLInputElement, InputProps>(({ className, containerClass, type, icon, ...props }, ref) => {
  const hasIcon = !!icon

  return (
    <div className={twMerge('relative w-full', containerClass)}>
      <input
        type={type}
        className={twMerge(defaultTextInputClasses, classNames('h-12', { 'pr-10': hasIcon }), className)}
        ref={ref}
        {...props}
      />
      <div className="absolute right-4 top-5">{!!icon && icon}</div>
    </div>
  )
})
Input.displayName = 'Input'

const PasswordInput = forwardRef<HTMLInputElement, InputProps>(({ className, ...props }, ref) => {
  const [showPassword, setShowPassword] = useState(false)

  return (
    <Input
      ref={ref}
      className={className}
      {...props}
      placeholder="Password"
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
