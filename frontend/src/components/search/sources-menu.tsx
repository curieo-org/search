import { Source } from '@/types/search'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import LayersIcon from '../icons/layers'
import { H2 } from '../lib/typography'
import LinkPreview from '../util/link-preview'

type SourcesMenuProps = HTMLAttributes<HTMLDivElement> & { sources: Source[] }

export default function SourcesMenu(props: SourcesMenuProps) {
  return (
    <div className={twMerge('w-full', props.className)}>
      <div className="flex items-center justify-center gap-x-2 py-2 mb-2 mr-2.5 bg-white/10 border border-white/10 rounded-lg">
        <LayersIcon className="text-typography-light dark:text-typography-dark" size={14} />
        <H2 className="font-medium text-[#DDDDE3] text-sm">Sources</H2>
      </div>
      <div className="flex flex-col gap-y-2.5 overflow-y-scroll scrollbar-visible h-5/6 pr-1">
        {props.sources.map((source, index) => (
          <LinkPreview
            style={{ animation: `fade-in ${Math.min(500 + index * 500, 3000)}ms` }}
            className="h-32 w-auto flex flex-col items-stretch justify-center p-2 xl:p-3 rounded-lg border border-white/10 bg-white/5 hover:bg-background-dark/20 dark:hover:bg-white/15"
            key={`source-preview-${index}`}
            source={source}
          />
        ))}
      </div>
    </div>
  )
}
