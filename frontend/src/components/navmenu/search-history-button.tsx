import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import { Thread } from '@/types/search'
import classNames from 'classnames'
import { usePathname, useRouter } from 'next/navigation'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import { Button } from '../lib/button'

type SearchHistoryButtonProps = HTMLAttributes<HTMLButtonElement> & {
  thread: Thread
}

export default function SearchHistoryButton(props: SearchHistoryButtonProps) {
  const router = useRouter()
  const pathname = usePathname()

  const {
    state: { isNavmenuCollapsed },
  } = useNavmenuStore()

  const threadPagePath = `/search?thread_id=${props.thread.thread_id}`

  const handleNavigateToThreadPage = () => {
    router.push(threadPagePath)
  }

  const isActive = pathname === threadPagePath

  const { className, thread, ...rest } = props

  return (
    <Button
      className={twMerge(
        classNames(
          'w-full h-auto rounded-md text-xs font-light hover:bg-white/3 text-typography-light dark:text-typography-dark/80',
          [
            isNavmenuCollapsed ? 'justify-center' : 'justify-start text-start pl-5 xl:pl-6',
            isActive ? 'bg-white/2' : 'bg-transparent',
          ]
        ),
        className
      )}
      label={thread.title}
      onClick={handleNavigateToThreadPage}
      {...rest}
    />
  )
}
