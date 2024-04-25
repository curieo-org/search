import {signIn, config, auth} from "@/auth"
import {redirect} from "next/navigation"
import {getProviders} from "next-auth/react";
import {SignInButton} from "@/app/api/auth/sign-in";

export default async function SignInPage() {
    console.log(config.providers);
    const session = await auth();
    //const providers = await getProviders();
    //const url = providers?.google?.signinUrl;

    // The user is already logged in, redirect to homepage.
    // Make sure is not the same URL to avoid an infinite loop!
    if (session) return redirect("/")

    return (
        <div className="flex flex-col gap-4 max-w-80 mx-auto justify-center h-screen">
            <SignInButton/>
            <form action={""} method="POST">
                <button className="flex flex-row gap-2" type="submit">
                    <span>Sign in with Google</span>
                </button>
            </form>
        </div>
    )
}