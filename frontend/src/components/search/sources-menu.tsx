import { Source } from '@/develop/dummy-data/search-result'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'
import LinkPreview from '../util/link-preview'
import LayersIcon from '../icons/layers'
import { H2 } from '../lib/typography'

type SourcesMenuProps = HTMLAttributes<HTMLDivElement> & { sources: Source[] }

export default function SourcesMenu(props: SourcesMenuProps) {
  return (
    <div className={twMerge('w-full', props.className)}>
      <div className="flex items-center gap-x-2 pl-2 mb-4">
        <LayersIcon className="text-background-light" size={20} />
        <H2 className="font-medium">Sources</H2>
      </div>
      <div className="flex flex-col gap-y-2">
        {props.sources.map((source, index) => (
          <LinkPreview
            className="p-3 hover:bg-background-light/20 hover:rounded-xl"
            key={`source-preview-${index}`}
            url={source.url}
          />
        ))}
      </div>
    </div>
  )
}
