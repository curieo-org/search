'use client'

import { useRouter } from 'next/navigation'
import { useEffect } from 'react'

export default function Home() {
  const router = useRouter()

  useEffect(() => {
    router.push('/authentication')
  }, [])

  return <div className="w-full p-24"></div>
}
