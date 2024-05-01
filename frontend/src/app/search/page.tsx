import { P } from '@/components/lib/typography'
import SearchInput from '@/components/search/search-input'

export default function Search() {
  return (
    <div className="w-full h-[90vh] flex justify-center items-center">
      <div className="w-full flex flex-col items-center">
        <P className="mb-10 text-3xl">Fast-Track Your Research</P>
        <SearchInput />
      </div>
    </div>
  )
}
