import { Suspense } from "react"
import { SearchResultsGrid } from "../search-results-grid"
import { SearchResultsCount } from "../search-results-count"
import { SearchForm } from "../search-form"

interface PageContentProps extends React.PropsWithChildren {
  prompt?: string
}

export const PageContent = ({ children, prompt }: PageContentProps) => {
  return (
    <>
      <div className="py-[15vh] sm:py-[20vh] flex flex-col items-center justify-center">
        <h1 className="font-medium text-4xl text-black mb-3 animate-in fade-in slide-in-from-bottom-3 duration-1000 ease-in-out">
          Curieo
        </h1>

        <div className="max-w-md space-y-4 w-full animate-in fade-in slide-in-from-bottom-4 duration-1200 ease-in-out">
          <SearchForm initialPrompt={prompt} />
          {children}
        </div>
      </div>

      <Suspense>
        <SearchResultsGrid prompt={prompt} />
      </Suspense>
    </>
  )
}
