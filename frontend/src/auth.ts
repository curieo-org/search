import NextAuth, {NextAuthConfig} from "next-auth"
import Google from "next-auth/providers/google"

export const config = {
    debug: true,
    providers: [Google],
    pages: {
        signIn: "/signin",
    },
} satisfies NextAuthConfig;

export const {handlers, auth, signIn, signOut} = NextAuth(config);