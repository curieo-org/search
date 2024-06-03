import { NextResponse } from 'next/server'
import { AuthParams, AuthResponse } from '@/types/auth'
import { BackendAPIClient } from '@/utils/backend-api-client'
import { encodeAsUrlSearchParams } from '@/utils'

export async function GET(request: Request): Promise<NextResponse> {
  const requestUrl = new URL(request.url)
  return NextResponse.redirect(requestUrl.origin)
}

export async function POST(request: Request): Promise<NextResponse> {
  console.debug('request', request)
  const requestUrl = new URL(request.url)

  // function login(p: AuthParams): Promise<AuthResponse> {
  //   return new Promise(async function (resolve, reject) {
  //     BackendAPIClient.post(
  //       '/auth/schmogin',
  //       encodeAsUrlSearchParams({
  //         username: p.username.trim(),
  //         password: p.password,
  //       })
  //     )
  //       .then(res => {
  //         resolve(res.data as AuthResponse)
  //       })
  //       .catch(err => reject(err))
  //   })
  // }

  //await login(await request.json())
  return NextResponse.redirect(requestUrl.origin)
}
