import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'

type SourceSkeletonProps = HTMLAttributes<HTMLDivElement>

export default function SourceSkeleton(props: SourceSkeletonProps) {
  return (
    <div role="status" className={twMerge('animate-pulse flex flex-col gap-y-3', props.className)}>
      <div className="flex flex-col gap-y-1.5 grow">
        <div className="grow h-2 mr-10 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 mr-6 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
      </div>

      <div className="flex flex-col gap-y-1 grow">
        <div className="grow h-1 mr-4 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-1 mr-8 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-1 mr-10 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-1 mr-6 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
      </div>
    </div>
  )
}
