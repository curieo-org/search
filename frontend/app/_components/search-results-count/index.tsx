import { getSearchResultsCount } from "@/server/get-search-results-count"
import { Suspense } from "react"

interface CountDisplayProps {
  count?: number
}

function CountDisplay({ count }: CountDisplayProps) {
  return (
    <p className="text-gray-500 mb-12 text-base animate-in fade-in slide-in-from-bottom-4 duration-1200 ease-in-out">
      {count || "–––"}
    </p>
  )
}

async function AsyncSearchResultsCount() {
  const count = await getSearchResultsCount()

  return <CountDisplay count={count} />
}

export function SearchResultsCount() {
  return (
    <Suspense fallback={<CountDisplay />}>
      <AsyncSearchResultsCount />
    </Suspense>
  )
}
