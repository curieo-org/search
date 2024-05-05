import { searchPagePath } from '@/constants/route'
import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import classNames from 'classnames'
import { useRouter } from 'next/navigation'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import PencilOutlineIcon from '../icons/pencil-outline'
import { Button } from '../lib/button'

export default function NewSearchButton(props: HTMLAttributes<HTMLButtonElement>) {
  const router = useRouter()
  const {
    state: { isNavmenuCollaped },
  } = useNavmenuStore()

  const handleNavigateToNewSearchPage = () => {
    router.push(searchPagePath)
  }

  return (
    <Button
      className={twMerge(
        classNames(
          'w-full rounded-2xl bg-gray-100 dark:bg-custom-black-ash text-typography-light dark:text-typography-dark border-l-0 border-custom-black-ash hover:border-l-2 hover:border-custom-violet-600/50 dark:hover:drop-shadow-xs dark:hover:bg-gradient-dark',
          [isNavmenuCollaped ? 'justify-center mx-2 w-auto' : 'justify-start']
        ),
        props.className
      )}
      label={isNavmenuCollaped ? undefined : 'New'}
      iconLeft={<PencilOutlineIcon size={isNavmenuCollaped ? 18 : 14} className="text-inherit" />}
      onClick={handleNavigateToNewSearchPage}
    />
  )
}
