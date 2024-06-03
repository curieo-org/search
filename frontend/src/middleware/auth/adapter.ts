import { httpAdapter } from 'next-auth-http-adapter'

export const adapter = httpAdapter({
  baseURL: 'backend-api/',
  headers: {
    Authorization: process.env.AUTH_SECRET!,
    // or set any global headers to be able to authenticate your requests to your backend
  },
  // you can provide any other
  adapterProcedures: {
    createUser(user) {
      console.debug('Creating user')
      return {
        path: 'auth/register/',
        method: 'POST',
        body: user,
      }
    },
    getUserById: id => ({
      path: `auth/get-user/${id}/`,
    }),
    getUserByEmail: email => ({
      path: `auth/get-user-by-email/${encodeURIComponent(email)}/`,
    }),
    getUserByAccount: ({ providerAccountId, provider }) => ({
      path: `auth/get-user-by-account/${encodeURIComponent(provider)}/${encodeURIComponent(providerAccountId)}/`,
    }),
    updateUser: user => ({
      path: 'auth/update-user/',
      method: 'PATCH',
    }),
    deleteUser: id => ({
      path: `auth/delete-user/${id}/`,
      method: 'DELETE',
    }),
    linkAccount: account => ({
      path: 'auth/link-account/',
      method: 'POST',
      body: account,
    }),
    unlinkAccount: ({ provider, providerAccountId }) => ({
      path: `auth/unlink-account/${encodeURIComponent(provider)}/${encodeURIComponent(providerAccountId)}/`,
      method: 'DELETE',
    }),
    createSession: session => ({
      path: 'auth/create-session/',
      method: 'POST',
      body: session,
    }),
    getSessionAndUser: sessionToken => ({
      path: `auth/get-session/${sessionToken}/`,
    }),
    updateSession: session => ({
      path: 'auth/update-session/',
      method: 'PATCH',
      body: session,
    }),
    deleteSession: sessionToken => ({
      path: `auth/delete-session/${sessionToken}/`,
      method: 'DELETE',
    }),
    createVerificationToken: verificationToken => ({
      path: 'auth/create-verification-token/',
      method: 'POST',
      body: verificationToken,
    }),
    useVerificationToken: params => ({
      path: 'auth/use-verification-token/',
      method: 'POST',
      body: params,
    }),
  },
})
