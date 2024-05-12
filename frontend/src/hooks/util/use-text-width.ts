import { getHTMLTextWidth } from '@/helpers/browser'
import { useEffect, useState } from 'react'

export function useTextWidth(text: string): number {
  const [textWidth, setTextWidth] = useState(1)

  useEffect(() => {
    setTextWidth(getHTMLTextWidth(text))
  }, [text])

  return textWidth
}
