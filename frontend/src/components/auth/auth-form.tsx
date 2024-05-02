'use client'

import { useAuthStore } from '@/stores/auth/auth-store'
import { HTMLAttributes, useState } from 'react'
import { Button } from '../lib/button'
import { Input, PasswordInput } from '../lib/form'
import { H1, Span } from '../lib/typography'
import { useLoginQuery } from '@/queries/auth/login-query'
import { useRegisterQuery } from '@/queries/auth/register-query'
import { toast } from 'react-toastify'
import { useRouter } from 'next/navigation'
import { AuthResponse } from '@/types/auth'

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

  const handleToggleAuthPurpose = () => {
    setAuthState('purpose', purpose === 'register' ? 'login' : 'register')
  }

  return (
    <div className="w-96 flex flex-col items-center">
      <img src="/images/curieo-logo.svg" className="mb-8" />
      <H1 className="text-3xl mb-6">{purpose === 'register' ? 'Create your account' : 'Welcome back'}</H1>
      <Input placeholder="Email" className="mb-6" value={email} onChange={e => setAuthState('email', e.target.value)} />
      <PasswordInput className="mb-6" value={password} onChange={e => setAuthState('password', e.target.value)} />
      <Button label="Continue" className="w-full mb-8" onClick={handleAuth} isLoading={isLoading} />
      <div className="flex items-center">
        <Span className="font-medium">
          {purpose === 'register' ? 'Already have an account?' : 'Don’t have an account yet?'}
        </Span>
        <Button
          className="bg-transparent hover:bg-transparent text-primary"
          label={purpose === 'register' ? 'Login' : 'Sign up'}
          onClick={handleToggleAuthPurpose}
        />
      </div>
    </div>
  )
}
