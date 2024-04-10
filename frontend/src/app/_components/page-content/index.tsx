"use client";

import { Suspense, useState } from "react";
import { SearchResultsGrid } from "../search-results-grid";
import { SearchForm } from "../search-form";
import { Loader } from "../loader";
import useSWR from "swr";

async function tokenFetcher(url: string): Promise<boolean> {
  const c = document.cookie
    .split("; ")
    .filter((row) => row.startsWith("id="))
    .map((c) => c.split("=")[1])[0];
  console.log(c);
  if (c) {
    return true;
  }
  const input: Record<string, string> = {
    username: "test1",
    password: "abcdef",
  };
  const payload = new URLSearchParams(input);

  const res = await fetch(url, {
    method: "POST",
    body: payload,
  });

  return true;
}

export const PageContent = () => {
  const [searchResults, setSearchResults] = useState<string>("");
  const {
    data: token,
    error,
    isLoading,
  } = useSWR("/api/auth/login", tokenFetcher);

  if (error) {
    return <h1>Could not ausdsthenticate</h1>;
  }
  if (isLoading) {
    return <Loader />;
  }
  if (token === undefined) {
    return <h1>Could not authenticate</h1>;
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
