import { P } from '@/components/lib/typography'
import SearchInput from '@/components/search/search-input'
import { HTMLAttributes } from 'react'
import { twMerge } from 'tailwind-merge'

type NewSearchProps = HTMLAttributes<HTMLDivElement> & {
  handleSearch: () => void
  searchQuery: string
  setSearchQuery: (query: string) => void
}

export default function NewSearch(props: NewSearchProps) {
  return (
    <div className={twMerge('w-full h-[90vh] flex justify-center items-center', props.className)}>
      <div className="w-full max-w-[720px] flex flex-col items-center px-4">
        <P className="mb-10 text-2xl xl:text-3xl transition-all duration-300">How can I help you today?</P>
        <SearchInput
          handleSearch={props.handleSearch}
          searchQuery={props.searchQuery}
          setSearchQuery={props.setSearchQuery}
        />
      </div>
    </div>
  )
}
