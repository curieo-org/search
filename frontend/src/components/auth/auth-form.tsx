import { HTMLAttributes } from 'react'
import { H1, Span } from '../lib/typography'
import AuthFormContents from '@/components/auth/auth-form-contents'
import AuthNavigation from '@/components/auth/auth-navigation'
import OAuthProviders from '@/components/auth/oauth-providers'
import { signin, signup } from '@/actions/auth'
import { getCsrfToken } from '@/auth'

export type AuthPurpose = 'register' | 'login'
export type AuthFormProps = HTMLAttributes<HTMLDivElement> & {
  authPurpose: AuthPurpose
}

export default async function AuthForm({ authPurpose }: AuthFormProps) {
  const csrfToken = getCsrfToken() || ''
  const formAction = authPurpose === 'register' ? signup : signin

  return (
    <div className="w-96 flex flex-col items-center">
      <img src="/images/curieo-logo.svg" className="mb-6" />
      <H1 className="text-3xl mb-6">{authPurpose === 'register' ? 'Create your account' : 'Welcome back'}</H1>
      <form className="w-full" action={formAction}>
        <AuthFormContents csrfToken={csrfToken}></AuthFormContents>
      </form>
      <AuthNavigation authPurpose={authPurpose} />
      <div className="w-full flex items-center gap-x-3 mb-4">
        <div className="h-px grow bg-custom-gray-200/25"></div>
        <Span>or continue with</Span>
        <div className="h-px grow bg-custom-gray-200/25"></div>
      </div>
      <OAuthProviders />
    </div>
  )
}
