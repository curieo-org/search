"use client"

import { track } from "@vercel/analytics"
import Image from "next/image"
import { useEffect, useState } from "react"
import toast from "react-hot-toast"
import { Loader } from "../loader"
import { cn } from "@/lib/utils"
import useSWR from "swr"

interface ButtonCard {
  id: string
  name: string
  src: string | null
  createdAt: Date
}

async function fetcher(url: string) {
  return fetch(url)
    .then((res) => res.json())
}

export function ButtonCard({ id, name, src: _src, createdAt }: ButtonCard) {
  const { data, isLoading } = useSWR<Awaited<ReturnType<typeof fetcher>>>(
    !_src ? `/api/search/${id}` : null,
    {
      fetcher,
      refreshInterval: (data) => (!!data?.recentSrc ? 0 : 1000), // 1 second
    }
  )
  const src = data?.recentSrc || _src
  const showImageTag = !!src // don't render image tag if no src
  const showImagePlaceholder = isLoading || !showImageTag

  useEffect(() => {
    if (isLoading || !data?.error) return
    toast.error(data.error)
  }, [isLoading, data?.error])

  return (
    <div
      id={id}
      className="borders ring-1 ring-gray-200 flex flex-row flex-nowrap py-1 px-1.5 items-center shadow-sm rounded-xl gap-1.5 bg-white w-full relative group"
    >

      {showImagePlaceholder && (
        <div
          aria-hidden
          className={cn("w-8 h-8 aspect-square bg-white", showImageTag ? "absolute left-1.5" : "relative")}
        >
          <div className="w-full h-full skeleton bg-gray-200 rounded-lg" />
        </div>
      )}

      <p className="font-mono text-sm truncate" title={name}>
        :{name}:
      </p>
    </div>
  )
}
