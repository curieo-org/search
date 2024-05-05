import React, { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'

const textColorClasses =
  'text-typography-light dark:text-typography-dark text-sm xl:text-base transition-all duration-300'

export function P(props: HTMLAttributes<HTMLParagraphElement>) {
  return <p className={twMerge(textColorClasses, props.className)} {...props} />
}

export function Span(props: HTMLAttributes<HTMLSpanElement>) {
  return <span className={twMerge(textColorClasses, props.className)} {...props} />
}

export function H1(props: HTMLAttributes<HTMLHeadingElement>) {
  return <h1 className={twMerge(textColorClasses, props.className)} {...props} />
}

export function H2(props: HTMLAttributes<HTMLHeadingElement>) {
  return <h2 className={twMerge(textColorClasses, props.className)} {...props} />
}

export function H3(props: HTMLAttributes<HTMLHeadingElement>) {
  return <h3 className={twMerge(textColorClasses, props.className)} {...props} />
}

export function H4(props: HTMLAttributes<HTMLHeadingElement>) {
  return <h4 className={twMerge(textColorClasses, props.className)} {...props} />
}
