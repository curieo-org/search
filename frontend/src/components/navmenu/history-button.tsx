import { useNavmenuStore } from '@/stores/navmenu/nav-menu-store'
import classNames from 'classnames'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import ClockIcon from '../icons/clock'
import { Button } from '../lib/button'

export default function HistoryButton(props: HTMLAttributes<HTMLButtonElement>) {
  const {
    state: { isNavmenuCollaped },
    toggleNavmenuState,
  } = useNavmenuStore()

  const toggleHistoryCollapse = () => {
    // toggleNavmenuState('isHistoryCollapsed')
  }

  return (
    <Button
      className={twMerge(
        classNames(
          'w-full rounded-2xl cursor-auto bg-transparent hover:bg-transparent text-typography-light dark:text-typography-dark',
          [isNavmenuCollaped ? 'justify-center mx-2 w-auto' : 'justify-start']
        ),
        props.className
      )}
      label={isNavmenuCollaped ? undefined : 'History'}
      iconLeft={<ClockIcon size={isNavmenuCollaped ? 20 : 16} className="text-inherit" />}
      onClick={toggleHistoryCollapse}
    />
  )
}
