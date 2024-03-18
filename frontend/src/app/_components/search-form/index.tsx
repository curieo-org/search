"use client";

import { Dispatch, SetStateAction, useEffect, useRef, useState } from "react";
import { SubmitButton } from "./submit-button";
import toast from "react-hot-toast";

interface SearchFormProps {
  token: string;
  setSearchResults: Dispatch<SetStateAction<string>>;
}

export function SearchForm({ token, setSearchResults }: SearchFormProps) {
  const submitRef = useRef<React.ElementRef<"button">>(null);

  async function handleSubmit(e) {
    e.preventDefault();
    const form = e.target;
    const formData = new FormData(form);
    const data = {
      query: formData.get("query")?.toString(),
    };

    const params = new URLSearchParams(data);
    const url = "api/search?" + params;
    const res = await fetch(url, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    if (res.status == 200) {
      const json = await res.json();
      setSearchResults(json?.result ?? "");
    } else {
      toast.error(res.statusText);
    }
  }
  return (
    <form
      className="flex h-fit w-full flex-row items-center rounded-xl bg-black px-1 shadow-lg"
      action="#"
      onSubmit={handleSubmit}
    >
      <input
        type="text"
        name="query"
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            e.preventDefault();
            submitRef.current?.click();
          }
        }}
        className="h-10 w-full resize-none bg-transparent px-2 py-2.5 font-mono text-sm text-white outline-none ring-0 transition-all duration-300 placeholder:text-gray-400"
      />
      <input
        aria-hidden
        type="text"
        name="token"
        value={token}
        className="hidden"
        readOnly
      />
      <SubmitButton ref={submitRef} />
    </form>
  );
}
