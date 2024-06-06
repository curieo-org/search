'use client'

import { Button } from '../lib/button'
import GoogleIcon from '@/components/icons/google'

export default function OAuthProviders() {
  const handleContinueWithGoogle = () => {
    console.log('CLICKED!')
  }
  return (
    <Button
      label="Google"
      iconLeft={<GoogleIcon size={20} />}
      className="w-full mb-2 bg-transparent hover:bg-transparent rounded-md border hover:border-2 border-background-dark dark:border-background-light"
      onClick={handleContinueWithGoogle}
    />
  )
  /* <Button
      label="Microsoft"
      iconLeft={<MicrosoftIcon size={20} />}
      className="w-full bg-transparent hover:bg-transparent rounded-md border hover:border-2 border-background-dark dark:border-background-light"
      onClick={handleContinueWithMicrosoft}
    /> */
}
