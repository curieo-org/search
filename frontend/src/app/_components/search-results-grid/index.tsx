import { SearchResultCard } from "../search-result-card";

export function SearchResultsGrid({ result }: any) {
  return (
    <div className="animate-in fade-in slide-in-from-bottom-4 duration-1200 ease-in-out">
      <h2 className="text-md mb-3 w-full text-left font-semibold">Results</h2>
      <div className="grid w-full grid-cols-1 justify-items-stretch gap-2">
        <SearchResultCard result={result} />
      </div>
    </div>
  );
}
