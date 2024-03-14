"use client"

import { useEffect, useRef, useState } from "react"
import { SubmitButton } from "./submit-button"
import toast from "react-hot-toast"
import useSWR from "swr"

interface SearchFormProps {
  initialPrompt?: string
}

export function SearchForm({ initialPrompt }: SearchFormProps) {
  const submitRef = useRef<React.ElementRef<"button">>(null)
  const [token, setToken] = useState("")

  //useEffect(() => {
  //  if (!formState) return
  //  toast.error(formState.message)
  //}, [formState])
  let formData = new FormData();
  formData.append('username', 'curieo');
  formData.append('password', 'whatever');
  useSWR(
    "/api/token",
    async (url: string) => {
      const res = await fetch(
        url,
        {
            method: 'POST',
            body: formData
        });
      const json = await res.json()
      return json?.token ?? ""
    },
    {
      onSuccess: (token) => setToken(token),
    }
  )

  return (
    <form className="bg-black rounded-xl shadow-lg h-fit flex flex-row px-1 items-center w-full">
      <input
        defaultValue={initialPrompt}
        type="text"
        name="prompt"
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            e.preventDefault()
            submitRef.current?.click()
          }
        }}
        placeholder="..."
        className="bg-transparent text-white placeholder:text-gray-400 ring-0 outline-none resize-none py-2.5 px-2 font-mono text-sm h-10 w-full transition-all duration-300"
      />
      <input aria-hidden type="text" name="token" value={token} className="hidden" readOnly />
      <SubmitButton ref={submitRef} />
    </form>
  )
}
