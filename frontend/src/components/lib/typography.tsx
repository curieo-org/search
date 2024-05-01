import React, { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'

const textColorClasses = 'text-foreground-light dark:text-background-dark'

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
