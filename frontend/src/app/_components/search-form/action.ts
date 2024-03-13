"use server"

import { nanoid } from "@/lib/utils"
import { Ratelimit } from "@upstash/ratelimit"
import { kv } from "@vercel/kv"
import { jwtVerify } from "jose"
import { redirect } from "next/navigation"
import { z } from "zod"

const jwtSchema = z.object({
  ip: z.string(),
  isIOS: z.boolean(),
})

const ratelimit = {
  free: new Ratelimit({
    redis: kv,
    limiter: Ratelimit.slidingWindow(500, "1 d"),
  }),
  ios: new Ratelimit({
    redis: kv,
    limiter: Ratelimit.slidingWindow(3, "7 d"),
    prefix: "ratelimit:ios",
  }),
}

interface FormState {
  message: string
}