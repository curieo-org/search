"use client";

import { Suspense, useState } from "react";
import { SearchResultsGrid } from "../search-results-grid";
import { SearchForm } from "../search-form";
import { Loader } from "../loader";
import useSWR from "swr";

async function tokenFetcher(url: string): Promise<string> {
  // TODO: Remove once we have real auth working.
  localStorage.clear();

  const token = localStorage.getItem("token");
  if (token) {
    return token;
  }

  const formData = new FormData();
  formData.append("username", "curieo");
  formData.append("password", "whatever");

  const res = await fetch(url, {
    method: "POST",
    body: formData,
  });
  const json = await res.json();
  let tokenValue = json?.access_token;
  localStorage.setItem("token", tokenValue);
  return tokenValue;
}

export const PageContent = () => {
  const [searchResults, setSearchResults] = useState<string>("");
  const { data: token, error, isLoading } = useSWR("/api/token", tokenFetcher);

  if (error) {
    return <h1>Could not authenticate</h1>;
  }
  if (isLoading) {
    return <Loader />;
  }
  return (
    <>
      <div className="flex flex-col items-center justify-center py-[15vh] sm:py-[20vh]">
        <div className="animate-in fade-in slide-in-from-bottom-4 duration-1200 w-full max-w-md space-y-4 ease-in-out">
          <SearchForm token={token} setSearchResults={setSearchResults} />
        </div>
      </div>

      <Suspense>
        <SearchResultsGrid result={searchResults} />
      </Suspense>
    </>
  );
};
