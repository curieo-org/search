import { signOut } from '@/auth'
import { headers as next_headers } from 'next/headers'

function curieoApiUrl(reqInfo?: RequestInfo): URL {
  let envUrl = process.env.NEXT_PUBLIC_API_URL
  let url: URL
  if (envUrl === undefined || envUrl === '') {
    throw new Error('Invalid API URL')
  }
  url = new URL(envUrl)
  let basePath = reqInfo instanceof Request ? reqInfo.url : reqInfo
  if (basePath && basePath !== '/' && url.pathname !== '/') {
    url.pathname = '/'
  }
  // remove trailing slash
  const sanitizedUrl = url.toString().replace(/\/$/, '')

  if (basePath) {
    // remove leading and trailing slash
    const sanitizedBasePath = basePath?.replace(/(^\/|\/$)/g, '') ?? ''
    return new URL(`${sanitizedUrl}/${sanitizedBasePath}`)
  }
  return new URL(`${sanitizedUrl}`)
}

export async function curieoFetch(reqInfo: RequestInfo, init?: RequestInit): Promise<Response> {
  const cookie = next_headers().get('cookie')
  let headers = init ? new Headers(init.headers) : new Headers()
  if (cookie) {
    headers.set('cookie', cookie)
  }
  if (!headers.has('Content-Type')) {
    headers.set('Content-Type', 'application/json')
  }
  if (init) {
    init.headers = headers
  } else {
    init = { headers }
  }

  const url: URL = curieoApiUrl(reqInfo)
  const response = await fetch(url, init)

  if (response.status === 405) {
    await signOut()
  }

  return response.ok ? Promise.resolve(response) : Promise.reject(response)
}
