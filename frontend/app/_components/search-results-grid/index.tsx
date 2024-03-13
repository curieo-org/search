import { search } from "@/server/get-search-results"
import { SearchResultCard } from "../search-result-card"

interface SearchResultsGridProps {
  prompt?: string
}

export async function SearchResultsGrid({ prompt }: SearchResultsGridProps) {
  const results = await search({
    take: 100,
    orderBy: prompt
      ? {
          _relevance: {
            fields: ["prompt"],
            sort: "desc",
            search: prompt,
          },
        }
      : undefined,
    cacheStrategy: prompt
      ? {
          swr: 86_400, // 1 day
          ttl: 7_200, // 2 hours
        }
      : undefined,
  })

  return (
    <div className="animate-in fade-in slide-in-from-bottom-4 duration-1200 ease-in-out">
      <h2 className="font-semibold text-md text-left w-full mb-3">Results</h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2 justify-items-stretch w-full">
        {results.map((result) => (
          <SearchResultCard key={result} result={result} />
        ))}
      </div>
    </div>
  )
}
