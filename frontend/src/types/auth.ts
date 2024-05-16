export type AuthParams = {
  email: string
  password: string
}

export type AuthResponse = {
  user_id: string
  email: string
}

export type LogoutResponse = {
  message: string
}
