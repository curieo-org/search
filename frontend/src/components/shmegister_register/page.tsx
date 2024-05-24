import AuthForm from '@/components/auth/auth-form'

export default function Register() {
  return (
    <div className="w-full h-screen flex justify-center items-center">
      <AuthForm authPurpose="register" />
    </div>
  )
}
