'use client'

import { useAuthStore } from '@/stores/auth/auth-store'
import { HTMLAttributes } from 'react'
import { Button } from '../lib/button'
import { Input, PasswordInput } from '../lib/form'
import { H1, Span } from '../lib/typography'

type AuthFormProps = HTMLAttributes<HTMLDivElement> & {}

export default function AuthForm(props: AuthFormProps) {
  const {
    state: { purpose },
    setAuthState,
  } = useAuthStore()

  const handleToggleAuthPurpose = () => {
    setAuthState('purpose', purpose === 'register' ? 'login' : 'register')
  }

  return (
    <div className="w-96 flex flex-col items-center">
      <img src="/images/curieo-logo.svg" className="mb-8" />
      <H1 className="text-3xl mb-6">{purpose === 'register' ? 'Create your account' : 'Welcome back'}</H1>
      <Input placeholder="Email" className="mb-6" />
      <PasswordInput className="mb-6" />
      <Button label="Continue" className="w-full mb-8" />
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
