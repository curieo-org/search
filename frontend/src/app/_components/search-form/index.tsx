"use client";

import { useEffect, useRef, useState } from "react";
import { SubmitButton } from "./submit-button";
import toast from "react-hot-toast";
import useSWR from "swr";

interface SearchFormProps {
  initialPrompt?: string;
}

export function SearchForm({ initialPrompt }: SearchFormProps) {
  const submitRef = useRef<React.ElementRef<"button">>(null);
  const [token, setToken] = useState("");

  //useEffect(() => {
  //  if (!formState) return
  //  toast.error(formState.message)
  //}, [formState])
  let formData = new FormData();
  formData.append("username", "curieo");
  formData.append("password", "whatever");
  useSWR(
    "/api/token",
    async (url: string) => {
      const res = await fetch(url, {
        method: "POST",
        body: formData,
      });
      const json = await res.json();
      return json?.access_token ?? "";
    },
    {
      onSuccess: (token) => {
        setToken(token);
        localStorage.setItem("token", token);
      },
    },
  );

  return (
    <form className="flex h-fit w-full flex-row items-center rounded-xl bg-black px-1 shadow-lg">
      <input
        defaultValue={initialPrompt}
        type="text"
        name="prompt"
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
