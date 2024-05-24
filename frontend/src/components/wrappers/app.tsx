'use client'

import { LayoutProps } from '@/app/layout'
import { profileRefreshTime } from '@/constants/config'
import {
  authPages,
  dynamicAppPaths,
  exactAppPaths,
  signinPagePath,
  pagesWithNavMenu,
  searchPagePath,
} from '@/constants/route'
import { fetchUserProfile } from '@/queries/settings/fetch-user-profile-query'
import { usePathname, useRouter } from 'next/navigation'
import { Fragment, createContext, useContext, useEffect, useState } from 'react'
import { Slide, ToastContainer } from 'react-toastify'
import { v4 as uuidv4 } from 'uuid'
import { P } from '../lib/typography'
import SpinnerLoading from '../util/spinner-loading'
import PageWithNavMenu from './page-with-navmenu'

export default function App({ children }: LayoutProps) {
  return (
    <div className="w-full">
      <ToastContainer
        transition={Slide}
        position="top-center"
        toastClassName="rounded-lg"
        progressClassName="h-0.5 bg-primary"
        autoClose={3000}
      />

      <div className="min-h-screen w-full bg-background-light dark:bg-gradient-to-br dark:from-background-dark-top-left dark:to-background-dark-bottom-right">
        <div className="md:hidden w-full h-screen flex flex-col items-center justify-center">
          <img src="/images/curieo-logo.svg" className="mb-4" alt="logo" />
          <P className="text-center text-xl">Please return to desktop view</P>
        </div>

        <div className="hidden md:block">
          <AppContextProvider>
            <AppMiddleware>{children}</AppMiddleware>
          </AppContextProvider>
        </div>
      </div>
    </div>
  )
}

function AppMiddleware({ children }: LayoutProps) {
  const pathName = usePathname()
  const { authStatus } = useAppContext()

  const isSignedIn = authStatus === 'authenticated'
  const isSignedOut = authStatus === 'unauthenticated'

  const isAuthPage = authPages.some(path => path === pathName)
  const isAppPage =
    exactAppPaths.some(path => path === pathName) || dynamicAppPaths.some(path => pathName.startsWith(path))

  if (isSignedIn) {
    if (isAuthPage || !isAppPage) {
      return <RedirectedPage path={searchPagePath} />
    }
    return <WrappedWithNavMenu>{children}</WrappedWithNavMenu>
  } else if (isAuthPage) {
    return <WrappedWithNavMenu>{children}</WrappedWithNavMenu>
  } else if (isSignedOut) {
    return <RedirectedPage path={signinPagePath} />
  } else {
    return <SpinnerLoading />
  }
}

function WrappedWithNavMenu({ children }: LayoutProps) {
  const pathName = usePathname()
  const isNavMenuVisible = pagesWithNavMenu.some(path => pathName.startsWith(path))

  return <Fragment>{isNavMenuVisible ? <PageWithNavMenu>{children}</PageWithNavMenu> : children}</Fragment>
}

function RedirectedPage({ path }: { path: string }) {
  const router = useRouter()

  useEffect(() => {
    router.push(path)
  }, [])

  return <SpinnerLoading />
}

type AuthStatus = 'authenticated' | 'unauthenticated' | 'loading'
type AppContextType = {
  authStatus: AuthStatus
  updateAuthStatus: (authStatus: AuthStatus) => void
}

const inititalContext = {
  authStatus: 'loading' as AuthStatus,
  updateAuthStatus: (authStatus: AuthStatus) => {},
}

const AppContext = createContext<AppContextType>(inititalContext)

function AppContextProvider({ children }: LayoutProps) {
  const [authStatus, setAuthStatus] = useState<AuthStatus>('loading')
  const [profileRefreshFlag, setProfileRefreshFlag] = useState(0)

  useEffect(() => {
    const profileRefreshInterval = setInterval(() => {
      setProfileRefreshFlag(prev => prev + 1)
    }, profileRefreshTime)

    return () => clearInterval(profileRefreshInterval)
  }, [])

  useEffect(() => refreshProfile(), [profileRefreshFlag])
  useEffect(() => {
    if (authStatus === 'loading') {
      refreshProfile()
    }
  }, [authStatus])

  function refreshProfile() {
    fetchUserProfile()
      .then(res => setAuthStatus('authenticated'))
      .catch(err => setAuthStatus('unauthenticated'))
  }

  const updateAuthStatus = (authStatus: AuthStatus) => setAuthStatus(authStatus)

  return <AppContext.Provider value={{ authStatus, updateAuthStatus }}>{children}</AppContext.Provider>
}

export function useAppContext() {
  return useContext(AppContext)
}
