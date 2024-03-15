type SearchResultCardProps = any;

export async function SearchResultCard({ key, result }: SearchResultCardProps) {
  return (
    <div
      id={key}
      className="borders group relative flex w-full flex-row flex-nowrap items-center gap-1.5 rounded-xl bg-white px-1.5 py-1 shadow-sm ring-1 ring-gray-200"
    >
      <p className="font-mono text-sm">{result}</p>
    </div>
  );
}
