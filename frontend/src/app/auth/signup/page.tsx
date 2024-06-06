import AuthForm from '@/components/auth/auth-form'

export default async function SignUp() {
  return (
    <div className="w-full h-screen flex justify-center items-center">
      <AuthForm authPurpose="register"></AuthForm>
    </div>
  )
}
