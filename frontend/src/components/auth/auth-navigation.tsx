'use client'

import { signinPagePath, signupPagePath } from '@/constants/route'
import { useRouter } from 'next/navigation'
import { Button } from '../lib/button'
import { Span } from '../lib/typography'
import { AuthPurpose } from '@/components/auth/auth-form'

export default function AuthNavigation({ authPurpose }: { authPurpose: AuthPurpose }) {
  const router = useRouter()
  const handleToggleAuthPurpose = () => {
    authPurpose === 'register' ? router.push(signinPagePath) : router.push(signupPagePath)
  }
  return (
    <div className="flex items-center mb-6">
      <Span className="font-medium">
        {authPurpose === 'register' ? 'Already have an account?' : 'Don’t have an account yet?'}
      </Span>
      <Button
        className="bg-transparent hover:bg-transparent text-primary"
        label={authPurpose === 'register' ? 'Login' : 'Sign up'}
        onClick={handleToggleAuthPurpose}
      />
    </div>
  )
}
