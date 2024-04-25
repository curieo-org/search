"use client"

import {signIn} from "next-auth/react"

export function SignInButton() {
    return <button onClick={() => signIn("google")}>Sign in</button>
}