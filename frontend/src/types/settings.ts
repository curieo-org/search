export type UserProfile = {
  user_id: string
  email: string
  username: string
  fullname: string | null
  title: string | null
  company: string | null
}

export const emptyUser = {
  user_id: '',
  email: '',
  username: '',
  fullname: null,
  title: null,
  company: null,
}

export type UpdateProfileBody = {
  user_id: string | undefined
  email: string | undefined
  username: string | undefined
  fullname: string | undefined
  title: string | undefined
  company: string | undefined
}

export type UpdatePasswordBody = {
  old_password: string
  new_password: string
}
