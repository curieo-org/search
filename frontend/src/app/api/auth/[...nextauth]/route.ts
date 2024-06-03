import { GET as G, POST as P } from '@/auth'
import type { NextRequest } from 'next/server'

export async function GET(req: NextRequest): Promise<Response> {
  console.debug(req)
  return G(req)
}

export async function POST(req: NextRequest): Promise<Response> {
  console.debug(req)
  return P(req)
}
