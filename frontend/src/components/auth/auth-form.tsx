'use client'

import {emailErrorMessage, passwordErrorMessage, welcomeMessage} from '@/constants/messages'
import {loginPagePath} from '@/constants/route'
import {useInputValidation} from '@/hooks/form/use-input-validation'
import {useLoginQuery} from '@/queries/auth/login-query'
import {useRegisterQuery} from '@/queries/auth/register-query'
import {useAuthFormStore} from '@/stores/auth/auth-form-store'
import {AuthResponse} from '@/types/auth'
import {useRouter} from 'next/navigation'
import {HTMLAttributes, useEffect, useState, MouseEvent} from 'react'
import {toast} from 'react-toastify'
import {z} from 'zod'
import GoogleIcon from '../icons/google'
import {Button} from '../lib/button'
import {Input, PasswordInput} from '../lib/form'
import {H1, Span} from '../lib/typography'
import {useAppContext} from '../wrappers/app'

type AuthFormProps = HTMLAttributes<HTMLDivElement> & {
    authPurpose: 'register' | 'login'
}

export default function AuthForm(props: AuthFormProps) {
    const [isLoading, setIsLoading] = useState(false)
    const router = useRouter()
    const {updateAuthStatus} = useAppContext()

    useEffect(() => updateAuthStatus('loading'), [])

    const login = useLoginQuery()
    const register = useRegisterQuery()
    const {
        state: {email, password},
        setAuthFormState,
        reset,
    } = useAuthFormStore()

    const {errorMessage: emailError, isError: isEmailError} = useInputValidation(
        email,
        z.string().email({message: emailErrorMessage})
    )
    const {errorMessage: passwordError, isError: isPasswordError} = useInputValidation(
        password,
        z.string().min(6, {message: passwordErrorMessage})
    )

    const authHandler = props.authPurpose === 'register' ? register : login
    const handleAuthToast = (res: AuthResponse) =>
        props.authPurpose === 'register' ? toast.success('You have registered successfully') : toast.success(welcomeMessage)
    const handleAuthRedirect = () =>
        props.authPurpose === 'register' ? router.push(loginPagePath) : updateAuthStatus('authenticated')

  const handleAuth = (event?: MouseEvent<HTMLButtonElement, globalThis.MouseEvent>) => {
    event?.preventDefault()
    setIsLoading(true)
    authHandler({ email, password })
      .then(res => {
        handleAuthToast(res)
        handleAuthRedirect()
      })
      .catch(err => toast.error(err.message))
      .finally(() => {
        setIsLoading(false)
        reset()
      })
  }

  const handleFormSubmit = () => {
    if (isEmailError || isPasswordError) {
      return
    }
    handleAuth()
  }

  const handleContinueWithGoogle = () => {}

  const handleContinueWithMicrosoft = () => {}

    const handleToggleAuthPurpose = () => {
        props.authPurpose === 'register' ? router.push(loginPagePath) : router.push('/register')
    }

    return (
        <div className="w-96 flex flex-col items-center">
            <img src="/images/curieo-logo.svg" className="mb-6"/>
            <H1 className="text-3xl mb-6">{props.authPurpose === 'register' ? 'Create your account' : 'Welcome back'}</H1>
            <form className="w-full" onSubmit={handleFormSubmit}>
        <Input
                containerClass="mb-4"
                placeholder="Email"
                value={email}
                onChange={e => setAuthFormState('email', e.target.value)}
                errorMessage={email.length > 0 ? emailError : undefined}
            />
            <PasswordInput
                containerClass="mb-4"
                value={password}
                onChange={e => setAuthFormState('password', e.target.value)}
                errorMessage={password.length > 0 ? passwordError : undefined}
            />
            <Button
                label="Continue"
                className="w-full mb-4"
                onClick={e => handleAuth(e)}
                isLoading={isLoading}
                disabled={isEmailError || isPasswordError}
            type="submit"
        />
      </form>
            <div className="flex items-center mb-6">
                <Span className="font-medium">
                    {props.authPurpose === 'register' ? 'Already have an account?' : 'Don’t have an account yet?'}
                </Span>
                <Button
                    className="bg-transparent hover:bg-transparent text-primary"
                    label={props.authPurpose === 'register' ? 'Login' : 'Sign up'}
                    onClick={handleToggleAuthPurpose}
                />
            </div>
            <div className="w-full flex items-center gap-x-3 mb-4">
                <div className="h-px grow bg-custom-gray-200/25"></div>
                <Span>or continue with</Span>
                <div className="h-px grow bg-custom-gray-200/25"></div>
            </div>
            <Button
                label="Google"
                iconLeft={<GoogleIcon size={20}/>}
                className="w-full mb-2 bg-transparent hover:bg-transparent rounded-md border hover:border-2 border-background-dark dark:border-background-light"
                onClick={handleContinueWithGoogle}
            />
            {/* <Button
        label="Microsoft"
        iconLeft={<MicrosoftIcon size={20} />}
        className="w-full bg-transparent hover:bg-transparent rounded-md border hover:border-2 border-background-dark dark:border-background-light"
        onClick={handleContinueWithMicrosoft}
      /> */}
        </div>
    )
}
