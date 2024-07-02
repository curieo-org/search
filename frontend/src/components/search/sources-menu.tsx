import { Source } from '@/types/search'
import { HTMLAttributes, useEffect, useRef, useState } from 'react'
import { twMerge } from 'tailwind-merge'
import LayersIcon from '../icons/layers'
import { H2 } from '../lib/typography'
import LinkPreview from '../util/link-preview'

type SourcesMenuProps = HTMLAttributes<HTMLDivElement> & { sources: Source[] }

export default function SourcesMenu(props: SourcesMenuProps) {
  const [updateCount, setUpdateCount] = useState(0)
  const [listHeight, setListHeight] = useState(100)
  const containerRef = useRef<HTMLDivElement>(null)
  const { sources, className, ...rest } = props

  useEffect(() => {
    if (updateCount < 5) {
      const interval = setInterval(() => {
        setUpdateCount(prevCount => prevCount + 1)
        if (containerRef.current) {
          setListHeight(Math.max(containerRef.current.offsetHeight - 70, 100))
        } else {
          clearInterval(interval)
        }
      }, 100)

      return () => clearInterval(interval)
    }
  }, [updateCount])

  return (
    <div
      className={twMerge('w-72 flex-none p-3 transition-all duration-300 bg-white/2 rounded-l-xl', className)}
      {...rest}
      ref={containerRef}
    >
      <div className="flex items-center justify-center gap-x-2 py-2 mb-2 mr-2.5 bg-white/10 border border-white/10 rounded-lg">
        <LayersIcon className="text-typography-light dark:text-typography-dark" size={14} />
        <H2 className="font-medium text-[#DDDDE3] text-sm">Sources</H2>
      </div>
      <div
        className="flex flex-col gap-y-2.5 overflow-y-scroll scrollbar-visible pr-1"
        style={{ maxHeight: listHeight }}
      >
        {sources.map((source, index) => (
          <LinkPreview
            style={{ animation: `fade-in ${Math.min(500 + index * 500, 3000)}ms` }}
            className="h-32 w-auto flex gap-x-2 items-stretch justify-center p-2 xl:p-3 rounded-lg border border-white/10 bg-white/5 hover:bg-background-dark/20 dark:hover:bg-white/15"
            key={`source-preview-${index}`}
            source={source}
          />
        ))}
      </div>
    </div>
  )
}
