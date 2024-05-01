import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import classNames from 'classnames'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import ChevronDownIcon from '../icons/chevron-down'
import ClockIcon from '../icons/clock'
import { Button } from '../lib/button'

export default function HistoryButton(props: HTMLAttributes<HTMLButtonElement>) {
  const {
    state: { isNavmenuCollaped },
    toggleNavmenuState,
  } = useNavmenuStore()

  const toggleHistoryCollapse = () => {
    toggleNavmenuState('isHistoryCollapsed')
  }

  return (
    <Button
      className={twMerge(
        classNames(
          'w-full rounded-full bg-background-light dark:bg-foreground-light text-foreground-dark dark:text-background-light',
          [isNavmenuCollaped ? 'justify-center' : 'justify-start']
        ),
        props.className
      )}
      label={isNavmenuCollaped ? undefined : 'History'}
      iconLeft={<ClockIcon size={isNavmenuCollaped ? 20 : 16} className="text-inherit" />}
      iconRight={
        isNavmenuCollaped ? undefined : <ChevronDownIcon size={18} className="text-inherit absolute right-2" />
      }
      onClick={toggleHistoryCollapse}
    />
  )
}
