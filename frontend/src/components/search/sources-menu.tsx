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
      <div className="flex items-center gap-x-2 pl-2 mb-4">
        <LayersIcon className="text-typography-light dark:text-typography-dark" size={20} />
        <H2 className="font-medium">Sources</H2>
      </div>
      <div className="flex flex-col gap-y-2.5">
        {props.sources.map((source, index) => (
          <LinkPreview
            style={{ animation: `fade-in ${Math.min(500 + index * 500, 3000)}ms` }}
            className="h-32 w-auto flex flex-col items-stretch justify-center p-2 xl:p-3 rounded-2xl border border-background-dark/20 dark:border-background-light/20 bg-gradient-source-card hover:bg-background-dark/20 dark:hover:bg-background-light/20"
            key={`source-preview-${index}`}
            url={source.url}
            metadata={source.metadata}
          />
        ))}
      </div>
    </div>
  )
}
