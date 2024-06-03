import { signOut } from '@/auth'

export default function SignOut() {
  return (
    <div className="w-full h-screen flex justify-center items-center">
      <form
        action={async () => {
          'use server'
          await signOut()
        }}
      >
        <button type="submit">Sign out</button>
      </form>
    </div>
  )
}
