'use client'

import { LayoutProps } from '@/app/layout'
import { authPaths, signinPagePath } from '@/constants/route'
import { usePathname } from 'next/navigation'
import { RedirectedPage } from './app copy'

export default function AuthRequired({ children }: LayoutProps) {
  let pathname = usePathname()
  if (authPaths.includes(pathname)) {
    return <>{children}</>
  }
  return <RedirectedPage path={signinPagePath} />
}
