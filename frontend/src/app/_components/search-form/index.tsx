"use client";

import { Dispatch, FormEvent, SetStateAction, useRef } from "react";
import toast from "react-hot-toast";
import { SubmitButton } from "./submit-button";

interface SearchFormProps {
  setSearchResults: Dispatch<SetStateAction<string>>;
}

export function SearchForm({ setSearchResults }: SearchFormProps) {
  const submitRef = useRef<React.ElementRef<"button">>(null);

  async function handleSubmit(e: FormEvent<HTMLFormElement>): Promise<void> {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const formData = new FormData(form);
    const data: Record<string, string> = {
      query: formData.get("query")?.toString() || "",
    };

    const params = new URLSearchParams(data);
    const url = "api/search?" + params;
    const res = await fetch(url);
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
      <SubmitButton ref={submitRef} />
    </form>
  );
}
