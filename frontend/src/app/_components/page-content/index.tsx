"use client";

import {Suspense, useState} from "react";
import {SearchResultsGrid} from "../search-results-grid";
import {SearchForm} from "../search-form";
import {Loader} from "../loader";
import useSWR from "swr";

async function tokenFetcher(url: string): Promise<boolean> {
    const input: Record<string, string> = {
        username: "test1",
        password: "abcdef",
    };
    await fetch(url, {
        method: "POST",
        body: new URLSearchParams(input),
    });

    return true;
}

export const PageContent = () => {
    const {data} = useSWR("/custodian/auth/login", tokenFetcher);
    const [searchResults, setSearchResults] = useState<string>("");
    return (
        <>
            <div className="flex flex-col items-center justify-center py-[15vh] sm:py-[20vh]">
                <div
                    className="animate-in fade-in slide-in-from-bottom-4 duration-1200 w-full max-w-md space-y-4 ease-in-out">
                    <SearchForm setSearchResults={setSearchResults}/>
                </div>
            </div>

            <Suspense>
                <SearchResultsGrid result={searchResults}/>
            </Suspense>
        </>
    );
};
