"use client";

import useSWR from "swr";
import { Loader } from "../loader";
import { SearchResultCard } from "../search-result-card";

interface SearchResultsGridProps {
  prompt?: string;
}

export function SearchResultsGrid({ prompt }: SearchResultsGridProps) {
  const token = localStorage.getItem("token");

  const { data, error, isLoading } = useSWR(
    "/api/search",
    async (url: string) => {
      url = url + "?" + new URLSearchParams({ query: "hello" });
      const res = await fetch(url, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      const json = await res.json();
      return json?.result ?? "";
    },
  );

  if (error) {
    return <h1>Something went wrong</h1>;
  }
  if (isLoading) {
    return <Loader />;
  }

  return (
    <div className="animate-in fade-in slide-in-from-bottom-4 duration-1200 ease-in-out">
      <h2 className="text-md mb-3 w-full text-left font-semibold">Results</h2>
      <div className="grid w-full grid-cols-1 justify-items-stretch gap-2">
        <SearchResultCard result={data} />
      </div>
    </div>
  );
}
