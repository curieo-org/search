import { getCsrfToken, signUp } from '@/auth'
import { Button } from '@/components/lib/button'
import { Input, PasswordInput } from '@/components/lib/form'
import { H1, Span } from '@/components/lib/typography'
import { AuthParams } from '@/types/auth'
import { AuthError } from 'next-auth'
import { redirect } from 'next/navigation'

export default function SignUp() {
  const csrfToken = getCsrfToken()
  return (
    <div className="w-full h-screen flex justify-center items-center">
      <div className="w-96 flex flex-col items-center">
        <img src="/images/curieo-logo.svg" className="mb-6" />
        <H1 className="text-3xl mb-6">Create your account</H1>
        <form
          className="w-full"
          action={async formData => {
            'use server'
            try {
              await signUp(formData as AuthParams)
            } catch (error) {
              if (error instanceof AuthError) {
                return redirect(`/auth/error?error=${error.type}`)
              }
              throw error
            }
          }}
        >
          <input type="hidden" name="csrfToken" value={csrfToken} />
          <Input
            containerClass="mb-4"
            placeholder="Email"
            name="username"
            //onChange={e => setAuthFormState('email', e.target.value)}
            //errorMessage={email.length > 0 ? emailError : undefined}
          />
          <PasswordInput
            containerClass="mb-4"
            placeholder="Password"
            name="password"
            //onChange={e => setAuthFormState('password', e.target.value)}
            //errorMessage={password.length > 0 ? passwordError : undefined}
          />
          <Button
            label="Continue"
            className="w-full mb-4"
            type="submit"
            //isLoading={isLoading}
            //disabled={isEmailError || isPasswordError}
          />
          <div className="flex items-center mb-6">
            <Span className="font-medium">Don’t have an account yet?</Span>
            <Button
              className="bg-transparent hover:bg-transparent text-primary"
              label="Sign up"
              onClick={async () => {
                'use server'
                redirect('/signup')
              }}
            />
          </div>
        </form>
      </div>
    </div>
  )
}
