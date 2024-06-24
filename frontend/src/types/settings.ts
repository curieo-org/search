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
