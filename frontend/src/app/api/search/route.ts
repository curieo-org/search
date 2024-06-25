import { curieoFetch } from '@/actions/fetch'
import { NextRequest, NextResponse } from 'next/server'

export const GET = async (req: NextRequest) => {
  const { searchParams } = new URL(req.url)
  const searchQuery = searchParams.get('searchQuery')
  const threadId = searchParams.get('threadId')

  if (!searchQuery) {
    return NextResponse.json({ error: 'searchQuery is required' }, { status: 400 })
  }

  async function pump(reader: ReadableStreamDefaultReader<Uint8Array>, writer: WritableStreamDefaultWriter<any>) {
    while (true) {
      const { done, value } = await reader.read()
      if (done) break
      writer.write(value)
    }
    writer.close()
  }

  try {
    const response = await curieoFetch(`/search?query=${searchQuery}${threadId ? `&thread_id=${threadId}` : ``}`)

    if (!response.ok) {
      return NextResponse.json({ error: 'Failed to fetch data from external API' }, { status: response.status })
    }

    const reader = response.body?.getReader()
    if (!reader) {
      return NextResponse.json({ error: 'Failed to read from external API stream' }, { status: 500 })
    }

    const headers = new Headers(response.headers)
    headers.delete('content-encoding')
    headers.set('content-encoding', 'none')

    const { readable, writable } = new TransformStream()
    const writer = writable.getWriter()

    pump(reader!, writer).catch(err => err)

    return new NextResponse(readable, { headers })
  } catch (error) {
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 })
  }
}
