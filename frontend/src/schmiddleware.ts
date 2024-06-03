import { auth } from '@/auth'
import { authPaths } from '@/constants/route'
import { NextRequest, NextResponse } from 'next/server'

export default auth(req => {
  console.debug(req)
  console.debug('How can I tell if this is working?')

  if (!req.auth || !authPaths.includes(req.nextUrl.pathname)) {
    const newUrl = new URL('/auth/signin', req.nextUrl.origin)
    return NextResponse.redirect(newUrl)
  }
})

// Optionally, don't invoke Middleware on some paths
export const config = {
  matcher: ['/((?!api|_next/static|_next/image|favicon.ico).*)|/images/curieo-logo.svg'],
}
