import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'

type SearchResultSkeletonProps = HTMLAttributes<HTMLDivElement>

export default function SearchResultSkeleton(props: SearchResultSkeletonProps) {
  return (
    <div role="status" className={twMerge('animate-pulse flex flex-col gap-y-6', props.className)}>
      <div className="flex items-center gap-x-6">
        <div className="w-1.5 h-10 bg-primary"></div>
        <div className="grow h-3 my-5 md:mr-10 lg:mr-20 xl:mr-28 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
      </div>

      <div className="flex flex-col gap-y-2.5 grow">
        <div className="grow h-2 md:mr-8 lg:mr-12 xl:mr-16 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-20 lg:mr-24 xl:mr-28 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-16 lg:mr-20 xl:mr-24 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-28 lg:mr-32 xl:mr-36 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-8 lg:mr-12 xl:mr-16 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
      </div>

      <div className="flex flex-col gap-y-2.5 grow">
        <div className="grow h-2 md:mr-20 lg:mr-24 xl:mr-28 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-28 lg:mr-32 xl:mr-36 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-8 lg:mr-12 xl:mr-16 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-16 lg:mr-20 xl:mr-24 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
      </div>

      <div className="flex flex-col gap-y-2.5 grow">
        <div className="grow h-2 md:mr-20 lg:mr-24 xl:mr-28 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-16 lg:mr-20 xl:mr-24 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-28 lg:mr-32 xl:mr-36 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
      </div>

      <div className="flex flex-col gap-y-2.5 grow">
        <div className="grow h-2 md:mr-8 lg:mr-12 xl:mr-16 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-20 lg:mr-24 xl:mr-28 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-16 lg:mr-20 xl:mr-24 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-28 lg:mr-32 xl:mr-36 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
        <div className="grow h-2 md:mr-8 lg:mr-12 xl:mr-16 rounded-full bg-background-dark/10  dark:bg-background-light/10"></div>
      </div>
    </div>
  )
}
