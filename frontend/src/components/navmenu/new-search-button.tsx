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
    router.push('/search')
  }

  return (
    <Button
      className={twMerge(
        classNames(
          'w-full rounded-full bg-background-light dark:bg-foreground-light text-foreground-dark dark:text-background-light hover:ring-2 hover:ring-custom-navy-blue dark:hover:bg-foreground-hover',
          [isNavmenuCollaped ? 'justify-center' : 'justify-start']
        ),
        props.className
      )}
      label={isNavmenuCollaped ? undefined : 'New'}
      iconLeft={<PencilOutlineIcon size={isNavmenuCollaped ? 18 : 14} className="text-inherit" />}
      onClick={handleNavigateToNewSearchPage}
    />
  )
}
