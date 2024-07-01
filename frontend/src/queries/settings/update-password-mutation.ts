import { updatePassword } from '@/actions/settings'
import { UpdatePasswordBody } from '@/types/settings'
import { useMutation } from '@tanstack/react-query'

export const useUpdatePasswordMutation = () => {
  return useMutation({
    mutationKey: ['update-password'],
    async mutationFn(payload: UpdatePasswordBody) {
      return await updatePassword(payload)
    },
  })
}
