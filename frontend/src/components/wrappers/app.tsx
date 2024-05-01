'use client'

import { LayoutProps } from '@/app/layout'
import { usePathname } from 'next/navigation'
import { Fragment } from 'react'
import { Slide, ToastContainer } from 'react-toastify'
import { P } from '../lib/typography'
import PageWithNavMenu from './page-with-navmenu'

const pagesWithNavMenu = ['/search', '/settings']

export default function App({ children }: LayoutProps) {
  const pathName = usePathname()
  const isNavMenuVisible = pagesWithNavMenu.some(path => pathName.startsWith(path))

  return (
    <div className="w-full">
      <ToastContainer
        transition={Slide}
        position="top-center"
        toastClassName="rounded-lg"
        progressClassName="h-0.5 bg-primary"
        autoClose={3000}
      />

      <div className="min-h-screen w-full bg-background-light dark:bg-foreground-light">
        <div className="md:hidden w-full h-screen flex flex-col items-center justify-center">
          <img src="/images/curieo-logo.svg" className="mb-4" />
          <P className="text-center text-xl">Please return to desktop view</P>
        </div>

        <div className="hidden md:block">
          {isNavMenuVisible ? <PageWithNavMenu>{children}</PageWithNavMenu> : <Fragment>{children}</Fragment>}
        </div>
      </div>
    </div>
  )
}
