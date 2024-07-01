'use client'

import { emailErrorMessage, passwordErrorMessage } from '@/constants/messages'
import { useInputValidation } from '@/hooks/form/use-input-validation'
import { useAuthFormStore } from '@/stores/auth/auth-form-store'
import { z } from 'zod'
import { Button } from '../lib/button'
import { Input, PasswordInput } from '../lib/form'

export default function AuthFormContents({ csrfToken }: { csrfToken: string }) {
  const {
    state: { email, password },
    setAuthFormState,
  } = useAuthFormStore()

  const { errorMessage: emailError, isError: isEmailError } = useInputValidation(
    email,
    z.string().email({ message: emailErrorMessage })
  )
  const { errorMessage: passwordError, isError: isPasswordError } = useInputValidation(
    password,
    z.string().min(6, { message: passwordErrorMessage })
  )
  return (
    <>
      <input type="hidden" name="csrfToken" value={csrfToken} />
      <Input
        containerClass="mb-4"
        placeholder="Email"
        name="username"
        onChange={e => setAuthFormState('email', e.target.value)}
        errorMessage={email.length > 0 ? emailError : undefined}
      />
      <PasswordInput
        containerClass="mb-4"
        placeholder="Password"
        name="password"
        onChange={e => setAuthFormState('password', e.target.value)}
        errorMessage={password.length > 0 ? passwordError : undefined}
      />
      <Button label="Continue" className="w-full mb-4" type="submit" disabled={isEmailError || isPasswordError} />
    </>
  )
}
