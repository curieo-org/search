import { oauthProviders, signIn } from '@/auth'
import { AuthError } from 'next-auth'
import { redirect } from 'next/navigation'
import { Button } from '../lib/button'
import { OAuthProviderIcon } from '@/components/icons/oauth-provider-icon'
import React from 'react'

export default function OAuthProviders() {
  return (
    <>
      {Object.values(oauthProviders).map(provider => (
        <form
          key={provider.id}
          action={async () => {
            'use server'
            try {
              await signIn(provider.id)
            } catch (error) {
              if (error instanceof AuthError) {
                return redirect(`/auth/error?error=${error.type}`)
              }
              throw error
            }
          }}
        >
          <Button
            type="submit"
            iconLeft={<OAuthProviderIcon id={provider.id} size={20} />}
            className="w-full mb-2 bg-transparent hover:bg-transparent rounded-md border hover:border-2 border-background-dark dark:border-background-light"
            label={provider.name}
          />
        </form>
      ))}
    </>
  )
}
