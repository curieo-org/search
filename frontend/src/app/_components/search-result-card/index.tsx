import { formatPrompt } from "@/lib/utils"
import { ButtonCard } from "./button-card"

type SearchResultCardProps = any;

export async function SearchResultCard({key, result}: SearchResultCardProps) {
  return (
        <div
          id={key}
          className="borders ring-1 ring-gray-200 flex flex-row flex-nowrap py-1 px-1.5 items-center shadow-sm rounded-xl gap-1.5 bg-white w-full relative group"
        >
            <p className="font-mono text-sm">{result}</p>
        </div>
    );
}
