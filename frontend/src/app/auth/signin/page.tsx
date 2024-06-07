import AuthForm from '@/components/auth/auth-form'

export default async function SignIn() {
  return (
    <div className="w-full h-screen flex justify-center items-center">
      <AuthForm authPurpose="login" />
    </div>
  )
}
