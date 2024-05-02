'use client'

import { useLoginQuery } from '@/queries/auth/login-query'
import { useRegisterQuery } from '@/queries/auth/register-query'
import { useAuthStore } from '@/stores/auth/auth-store'
import { AuthResponse } from '@/types/auth'
import { useRouter } from 'next/navigation'
import { HTMLAttributes, useState } from 'react'
import { toast } from 'react-toastify'
import GoogleIcon from '../icons/google'
import { Button } from '../lib/button'
import { Input, PasswordInput } from '../lib/form'
import { H1, Span } from '../lib/typography'
// import MicrosoftIcon from '../icons/microsoft'

type AuthFormProps = HTMLAttributes<HTMLDivElement> & {}

export default function AuthForm(props: AuthFormProps) {
  const [isLoading, setIsLoading] = useState(false)
  const login = useLoginQuery()
  const register = useRegisterQuery()
  const router = useRouter()
  const {
    state: { purpose, email, password },
    setAuthState,
  } = useAuthStore()

  const authHandler = purpose === 'register' ? register : login
  const authToast = (res: AuthResponse) =>
    purpose === 'register' ? toast.success('You have registered successfully') : toast.success(`Welcome ${res.email}`)

  const handleAuth = () => {
    setIsLoading(true)
    authHandler({ email, password })
      .then(res => {
        authToast(res)
        router.push('/search')
      })
      .catch(err => toast.error(err.message))
      .finally(() => setIsLoading(false))
  }

  const handleContinueWithGoogle = () => {}

  const handleContinueWithMicrosoft = () => {}

  const handleToggleAuthPurpose = () => {
    setAuthState('purpose', purpose === 'register' ? 'login' : 'register')
  }

  return (
    <div className="w-96 flex flex-col items-center">
      <img src="/images/curieo-logo.svg" className="mb-6" />
      <H1 className="text-3xl mb-6">{purpose === 'register' ? 'Create your account' : 'Welcome back'}</H1>
      <Input placeholder="Email" className="mb-4" value={email} onChange={e => setAuthState('email', e.target.value)} />
      <PasswordInput className="mb-4" value={password} onChange={e => setAuthState('password', e.target.value)} />
      <Button label="Continue" className="w-full mb-4" onClick={handleAuth} isLoading={isLoading} />
      <div className="flex items-center mb-6">
        <Span className="font-medium">
          {purpose === 'register' ? 'Already have an account?' : 'Don’t have an account yet?'}
        </Span>
        <Button
          className="bg-transparent hover:bg-transparent text-primary"
          label={purpose === 'register' ? 'Login' : 'Sign up'}
          onClick={handleToggleAuthPurpose}
        />
      </div>
      <div className="w-full flex items-center gap-x-3 mb-4">
        <div className="h-px grow bg-custom-gray/25"></div>
        <Span>or continue with</Span>
        <div className="h-px grow bg-custom-gray/25"></div>
      </div>
      <Button
        label="Google"
        iconLeft={<GoogleIcon size={20} />}
        className="w-full mb-2 bg-transparent hover:bg-transparent rounded-md border hover:border-2 border-foreground-dark dark:border-background-light"
        onClick={handleContinueWithGoogle}
      />
      {/* <Button
        label="Microsoft"
        iconLeft={<MicrosoftIcon size={20} />}
        className="w-full bg-transparent hover:bg-transparent rounded-md border hover:border-2 border-foreground-dark dark:border-background-light"
        onClick={handleContinueWithMicrosoft}
      /> */}
    </div>
  )
}
