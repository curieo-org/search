import { LayoutProps } from '@/app/layout'
import { auth } from '@/auth'
import { Slide, ToastContainer } from 'react-toastify'
import { P } from '../lib/typography'
import { AuthRequired, Authenticated } from './app-middleware'
import { useSession } from 'next-auth/react'

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
          <AppMiddleware>{children}</AppMiddleware>
        </div>
      </div>
    </div>
  )
}

async function AppMiddleware({ children }: LayoutProps) {
  const session = await auth()

  if (session) {
    return <Authenticated>{children}</Authenticated>
  } else {
    return <AuthRequired>{children}</AuthRequired>
  }
}
