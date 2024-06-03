'use client'

import { LayoutProps } from '@/app/layout'
import { authPaths, dynamicAppPaths, exactAppPaths, searchPagePath, signinPagePath } from '@/constants/route'
import { usePathname } from 'next/navigation'
import { RedirectedPage, WrappedWithNavMenu } from './app copy'

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
