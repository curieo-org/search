import { emailErrorMessage } from '@/constants/messages'
import { useInputValidation } from '@/hooks/form/use-input-validation'
import { useUpdateUserProfileMutation } from '@/queries/settings/update-user-profile-mutation'
import { useSettingsStore } from '@/stores/settings/settings-store'
import { UpdateProfileBody } from '@/types/settings'
import _ from 'lodash'
import { useSession } from 'next-auth/react'
import { HTMLAttributes, useEffect } from 'react'
import { toast } from 'react-toastify'
import { twMerge } from 'tailwind-merge'
import { z } from 'zod'
import { Button } from '../lib/button'
import { P, Span } from '../lib/typography'
import EditableProfileInfo from './editable-profile-info'

type ProfileSettingsProps = HTMLAttributes<HTMLDivElement> & {}

export default function ProfileSettings(props: ProfileSettingsProps) {
  const { data: session } = useSession()
  const { mutate: updateUserProfile, isError, isSuccess } = useUpdateUserProfileMutation()
  const {
    state: { editedUser, currentUser, isEdited },
    setCurrentUser,
    setEditedUserInfo,
  } = useSettingsStore()
  const { errorMessage: emailError, isError: isEmailError } = useInputValidation(
    editedUser.email,
    z.string().email({ message: emailErrorMessage })
  )

  useEffect(() => {
    if (isError) {
      setCurrentUser(currentUser)
      toast.error('Failed to update profile. Please try again later.')
    }
  }, [isError])

  useEffect(() => {
    if (isSuccess) {
      toast.success('User profile updated successfully.')
    }
  }, [isSuccess])

  return (
    <div className={twMerge('', props.className)}>
      <div className="w-full flex justify-between gap-x-8">
        <div>
          <Span className="text-sm text-custom-gray-50">Avatar</Span>
          <P className="text-sm text-white/60 mt-4">
            Update your avatar by clicking the image below. 288x288 px size recommended in PNG or JPG format only with a
            maximum file size of 1 mb.
          </P>
        </div>
        <img
          src={session?.user?.image ? session?.user?.image : '/images/placeholder-user.png'}
          className="h-32 w-auto rounded-2xl"
          alt="user image"
        />
      </div>
      <div className="flex flex-col gap-y-3">
        <EditableProfileInfo
          label="Email"
          value={editedUser.email ?? ''}
          setValue={event => setEditedUserInfo('email', event.target.value)}
          errorMessage={editedUser.email.length > 0 ? emailError : undefined}
        />
        <EditableProfileInfo
          label="User name"
          value={editedUser.username ?? ''}
          setValue={event => setEditedUserInfo('username', event.target.value)}
        />
        <EditableProfileInfo
          label="Full name"
          value={editedUser.fullname ?? ''}
          setValue={event => setEditedUserInfo('fullname', event.target.value)}
        />
        <EditableProfileInfo
          label="Title"
          value={editedUser.title ?? ''}
          setValue={event => setEditedUserInfo('title', event.target.value)}
        />
        <EditableProfileInfo
          label="Company"
          value={editedUser.company ?? ''}
          setValue={event => setEditedUserInfo('company', event.target.value)}
        />
        <Button
          className="w-full h-8 text-xs xl:text-sm rounded-md bg-transparent hover:bg-white/5 border border-white/10 mt-2"
          label="Save"
          disabled={!isEdited || isEmailError}
          onClick={() => updateUserProfile(_.omitBy(editedUser, _.isNil) as UpdateProfileBody)}
        />
      </div>
    </div>
  )
}
