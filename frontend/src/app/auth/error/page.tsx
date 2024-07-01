'use client'

import { H3 } from '@/components/lib/typography'
import { useSearchParams } from 'next/navigation'

enum Error {
  Configuration = 'Configuration',
  AccessDenied = 'AccessDenied',
  SignUpError = 'SignUpError',
}

const errorMap = {
  [Error.Configuration]: (
    <p>
      There was a problem when trying to authenticate. Please contact us if this error persists.
      <br />
      Unique error code:
      <br />
      <code className="text-xs p-1 rounded-sm">Configuration</code>
    </p>
  ),
  [Error.AccessDenied]: (
    <p>
      Signin failed, please try again. Please contact us if this error persists.
      <br />
      Unique error code:
      <br /> <code className="text-xs p-1 rounded-sm">AccessDenied</code>
    </p>
  ),
  [Error.SignUpError]: (
    <p>
      There was a problem when signing up. Please contact us if this error persists.
      <br />
      Unique error code:
      <br /> <code className="text-xs p-1 rounded-sm">SignUpError</code>
    </p>
  ),
}

export default function AuthErrorPage() {
  const search = useSearchParams()
  const error = search.get('error') as Error

  return (
    <div className="w-full h-screen flex justify-center items-center">
      <div className="w-96 flex flex-col items-center">
        <div className="block max-w-sm p-6 border border-gray-300 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 text-center">
          <H3 className="text-2xl mb-5 pb-5">Something went wrong</H3>
          <div className="w-full">{errorMap[error] || 'Please contact us if this error persists.'}</div>
        </div>
      </div>
    </div>
  )
}
