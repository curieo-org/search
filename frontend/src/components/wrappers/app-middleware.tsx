'use client'

import { LayoutProps } from '@/app/layout'
import {
  authPaths,
  dynamicAppPaths,
  exactAppPaths,
  pagesWithNavMenu,
  searchPagePath,
  signinPagePath,
} from '@/constants/route'
import { usePathname, useRouter } from 'next/navigation'
import { useEffect } from 'react'
import PageWithNavMenu from '@/components/wrappers/page-with-navmenu'
import SpinnerLoading from '@/components/util/spinner-loading'

export function AuthRequired({ children }: LayoutProps) {
  let pathname = usePathname()
  if (authPaths.includes(pathname)) {
    return <>{children}</>
  }
  return <RedirectedPage path={signinPagePath} />
}

export function Authenticated({ children }: LayoutProps) {
  let pathname = usePathname()
  if (authPaths.includes(pathname)) {
    return <RedirectedPage path={searchPagePath} />
  }
  const isAppPage = exactAppPaths.includes(pathname) || dynamicAppPaths.some(path => pathname.startsWith(path))
  if (!isAppPage) {
    return <RedirectedPage path={searchPagePath} />
  }
  return <WrappedWithNavMenu>{children}</WrappedWithNavMenu>
}

export function WrappedWithNavMenu({ children }: LayoutProps) {
  const pathName = usePathname()
  const isNavMenuVisible = pagesWithNavMenu.some(path => pathName.startsWith(path))

  return <>{isNavMenuVisible ? <PageWithNavMenu>{children}</PageWithNavMenu> : children}</>
}

export function RedirectedPage({ path }: { path: string }) {
  const router = useRouter()

  useEffect(() => {
    router.push(path)
  }, [])

  return <SpinnerLoading />
}
