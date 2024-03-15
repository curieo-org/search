import { SearchCheck } from "lucide-react";
import React from "react";
import { Loader } from "../loader";

export const SubmitButton = React.forwardRef<React.ElementRef<"button">>(
  (_, ref) => {
    return (
      <button
        ref={ref}
        type="submit"
        //disabled={pending}
        //aria-disabled={pending}
        className="flex aspect-square h-8 w-8 items-center justify-center rounded-lg text-white outline-0 ring-0 hover:bg-white/25 focus:bg-white/25"
      >
        <SearchCheck size={16} className="-ml-px" />
      </button>
    );
  },
);
SubmitButton.displayName = "SubmitButton";
