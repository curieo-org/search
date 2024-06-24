import { emailErrorMessage, passwordErrorMessage } from '@/constants/messages'
import { useInputValidation } from '@/hooks/form/use-input-validation'
import { useFetchUserProfile } from '@/queries/settings/fetch-user-profile-query'
import { useUpdateUserProfileMutation } from '@/queries/settings/update-user-profile-mutation'
import { UpdateProfileBody } from '@/types/settings'
import { HTMLAttributes, useEffect, useState } from 'react'
import { toast } from 'react-toastify'
import { twMerge } from 'tailwind-merge'
import { z } from 'zod'
import { Button } from '../lib/button'
import { PasswordInput } from '../lib/form'
import { P, Span } from '../lib/typography'

type SecuritySettingsProps = HTMLAttributes<HTMLDivElement> & {}

export default function SecuritySettings(props: SecuritySettingsProps) {
  const [oldPassword, setOldPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [confirmNewPassword, setConfirmNewPassword] = useState('')

  const { errorMessage: oldPasswordError, isError: isOldPasswordError } = useInputValidation(
    oldPassword,
    z.string().min(6, { message: passwordErrorMessage })
  )
  const { errorMessage: newPasswordError, isError: isNewPasswordError } = useInputValidation(
    newPassword,
    z.string().min(6, { message: passwordErrorMessage })
  )
  const { errorMessage: confirmNewPasswordError, isError: isConfirmNewPasswordError } = useInputValidation(
    confirmNewPassword,
    z.string().min(6, { message: passwordErrorMessage })
  )

  // useEffect(() => {
  //   if (isError) {
  //     toast.error('Failed to update email. Please try again later.')
  //   }
  // }, [isError])

  // useEffect(() => {
  //   if (isSuccess) {
  //     toast.success('User email updated successfully.')
  //   }
  // }, [isSuccess])

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
      <div>
        <Span className="text-sm text-custom-gray-50">Old Password</Span>
        <PasswordInput
          containerClass="mt-2"
          className="bg-transparent text-custom-gray-150 border border-white/20"
          placeholder="Old password"
          name="oldPassword"
          onChange={e => setOldPassword(e.target.value)}
          errorMessage={oldPassword.length > 0 ? oldPasswordError : undefined}
        />
      </div>
      <div>
        <Span className="text-sm text-custom-gray-50">New Password</Span>
        <PasswordInput
          containerClass="mt-2"
          className="bg-transparent text-custom-gray-150 border border-white/20"
          placeholder="New password"
          name="newPassword"
          onChange={e => setNewPassword(e.target.value)}
          errorMessage={newPassword.length > 0 ? newPasswordError : undefined}
        />
      </div>
      <div>
        <Span className="text-sm text-custom-gray-50">Confirm New Password</Span>
        <PasswordInput
          containerClass="mt-2"
          className="bg-transparent text-custom-gray-150 border border-white/20"
          placeholder="Confirm New password"
          name="confirmNewPassword"
          onChange={e => setNewPassword(e.target.value)}
          errorMessage={confirmNewPassword.length > 0 ? confirmNewPasswordError : undefined}
        />
      </div>
      <Button
        className="w-full h-10 rounded-md bg-transparent hover:bg-white/5 border border-white/10"
        label="Save"
        disabled={
          isOldPasswordError || isNewPasswordError || isConfirmNewPasswordError || newPassword !== confirmNewPassword
        }
        //onClick={() => updateUserProfile({ email } as UpdateProfileBody)}
      />
    </div>
  )
}
