import { NextResponse } from 'next/server'

export async function GET(request: Request): Promise<NextResponse> {
  const requestUrl = new URL(request.url)
  return NextResponse.redirect(requestUrl.origin)
}

export async function POST(request: Request): Promise<NextResponse> {
  const requestUrl = new URL(request.url)
  return NextResponse.redirect(requestUrl.origin)
}
