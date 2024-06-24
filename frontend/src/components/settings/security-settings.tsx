import { emailErrorMessage } from '@/constants/messages'
import { useInputValidation } from '@/hooks/form/use-input-validation'
import { useFetchUserProfile } from '@/queries/settings/fetch-user-profile-query'
import { useUpdateUserProfileMutation } from '@/queries/settings/update-user-profile-mutation'
import { UpdateProfileBody } from '@/types/settings'
import { HTMLAttributes, useEffect, useState } from 'react'
import { toast } from 'react-toastify'
import { twMerge } from 'tailwind-merge'
import { z } from 'zod'
import { Button } from '../lib/button'
import { P, Span } from '../lib/typography'
import EditableProfileInfo from './editable-profile-info'

type SecuritySettingsProps = HTMLAttributes<HTMLDivElement> & {}

export default function SecuritySettings(props: SecuritySettingsProps) {
  const { mutate: updateUserProfile, isError, isSuccess } = useUpdateUserProfileMutation()
  const { data: currentUser } = useFetchUserProfile()
  const [email, setEmail] = useState('')
  const { errorMessage: emailError, isError: isEmailError } = useInputValidation(
    email,
    z.string().email({ message: emailErrorMessage })
  )

  useEffect(() => {
    if (currentUser) {
      setEmail(currentUser.email)
    }
  }, [currentUser])

  useEffect(() => {
    if (isError) {
      toast.error('Failed to update email. Please try again later.')
    }
  }, [isError])

  useEffect(() => {
    if (isSuccess) {
      toast.success('User email updated successfully.')
    }
  }, [isSuccess])

  return (
    <div className={twMerge('flex flex-col gap-y-4', props.className)}>
      <div className="w-full flex justify-between gap-x-8">
        <div className="mb-2">
          <Span className="text-sm text-custom-gray-50">Update your password</Span>
          <P className="text-sm text-white/60 mt-4">
            You can update your password below. If you forgot your current password please contact support for
            assistance.
          </P>
        </div>
      </div>
      <EditableProfileInfo
        label="Email"
        value={email}
        setValue={event => setEmail(event.target.value)}
        errorMessage={email.length > 0 ? emailError : undefined}
      />
      <Button
        className="w-full h-10 rounded-md bg-transparent hover:bg-white/5 border border-white/10"
        label="Save"
        disabled={currentUser?.email === email || isEmailError}
        onClick={() => updateUserProfile({ email } as UpdateProfileBody)}
      />
    </div>
  )
}
