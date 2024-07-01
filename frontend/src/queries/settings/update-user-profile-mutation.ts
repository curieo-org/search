import { updateUserProfile } from '@/actions/settings'
import { useSettingsStore } from '@/stores/settings/settings-store'
import { UpdateProfileBody, UserProfile, emptyUser } from '@/types/settings'
import { useMutation, useQueryClient } from '@tanstack/react-query'

export const useUpdateUserProfileMutation = () => {
  const queryClient = useQueryClient()
  const {
    state: { currentUser },
  } = useSettingsStore()
  return useMutation({
    mutationKey: ['update-profile'],
    async mutationFn(payload: UpdateProfileBody) {
      const refinedPayload = {} as UpdateProfileBody
      Object.keys(emptyUser).forEach(key => {
        const key_ = key as keyof UserProfile
        if (payload[key_] !== currentUser[key_]) {
          refinedPayload[key_] = payload[key_]
        }
      })

      const updatedProfile = await updateUserProfile(refinedPayload)
      queryClient.setQueryData(['user-profile'], updatedProfile)
      return updatedProfile
    },
  })
}
