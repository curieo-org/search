import { getHTMLTextWidth } from '@/helpers/css-helpers'
import { useEffect, useState } from 'react'

export function useTextWidth(text: string): number {
  const [textWidth, setTextWidth] = useState(1)

  useEffect(() => {
    setTextWidth(getHTMLTextWidth(text))
  }, [text])

  return textWidth
}
