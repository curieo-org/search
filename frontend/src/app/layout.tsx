import type { Metadata } from "next";
import { Inter } from "next/font/google";
import Link from "next/link"
import { cn } from "@/lib/utils"
import { Providers } from "./_components/providers"
import Image from 'next/image';
import "./globals.css";

const BODY_PADDING = "px-4 sm:px-6"

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Curieo",
  description: "Next generation healthcare search engine",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={cn(inter.className, "antialiased bg-gray-100")}>
        <header
          className={cn(
            "top-0 sticky z-20 w-full py-3 bg-gray-100 flex flex-row flex-nowrap justify-between max-w-5xl mx-auto h-14 items-stretch animate-in fade-in slide-in-from-top-4 duration-1000 ease-in-out",
            BODY_PADDING
          )}
        >

          <Link
            className="text-black text-lg font-medium flex flex-row flex-nowrap items-center justify-center gap-x-1.5 pr-1.5 leading-none rounded-lg"
            href="/"
          >
            <Image
              src="/icon.png"
              width={24}
              height={24}
              alt="Picture of the author"
            />
            <span>Curieo</span>
          </Link>
        </header>
        <main className={cn("min-h-screen flex items-stretch flex-col pb-28 max-w-5xl mx-auto", BODY_PADDING)}>
          {children}
        </main>
        <Providers />
      </body>
    </html>
  )
}
