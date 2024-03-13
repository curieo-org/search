import { NextResponse } from "next/server"
import "server-only"
import { ZodError, z } from "zod"

export class Response {
  static success<JsonBody>(props?: { body?: JsonBody; headers?: HeadersInit }) {
    const { headers } = props ?? {}
    const body = props?.body ?? { ok: true }
    return NextResponse.json(body, { status: 200, headers })
  }

  static internalServerError() {
    return NextResponse.json({ error: { message: "Internal server error" } }, { status: 500 })
  }

  static unauthorized() {
    return NextResponse.json({ error: { message: "Unauthorized" } }, { status: 403 })
  }

  static badRequest(message: string) {
    return NextResponse.json({ error: { message } }, { status: 400 })
  }

  static invalidRequest(zodError: ZodError) {
    return NextResponse.json({ error: { message: "Invalid request", details: zodError } }, { status: 400 })
  }

  static tooManyRequests({ headers }: { headers?: HeadersInit }) {
    return NextResponse.json({ error: { message: "Too many requests" } }, { status: 429, headers })
  }
}

export const webhookSchema = z.object({
  id: z.string().length(7).or(z.string().length(10)),
  secret: z.string().refine((data) => data === process.env.API_SECRET, {
    message: "Invalid secret",
  }),
})

export const searchContextSchema = z.object({
  params: z.object({
    id: z.string(),
  }),
})
export type SearchContextProps = z.infer<typeof searchContextSchema>
