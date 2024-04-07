import type { Metadata } from "next";
import { Inter } from "next/font/google";
import Link from "next/link";
import { cn } from "@/lib/utils";
import { Providers, PosthogProvider } from "./_components/providers";
import Image from "next/image";
import "./globals.css";
import dynamic from "next/dynamic";

const BODY_PADDING = "px-4 sm:px-6";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Curieo",
  description: "Next generation healthcare search engine",
};

const PostHogPageView = dynamic(() => import("./PostHogPageView"), {
  ssr: false,
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <PosthogProvider>
        <body className={cn(inter.className, "bg-gray-100 antialiased")}>
          <header
            className={cn(
              "animate-in fade-in slide-in-from-top-4 sticky top-0 z-20 mx-auto flex h-14 w-full max-w-5xl flex-row flex-nowrap items-stretch justify-between bg-gray-100 py-3 duration-1000 ease-in-out",
              BODY_PADDING,
            )}
          >
            <Link
              className="flex flex-row flex-nowrap items-center justify-center gap-x-1.5 rounded-lg pr-1.5 text-lg font-medium leading-none text-black"
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
          <main
            className={cn(
              "mx-auto flex min-h-screen max-w-5xl flex-col items-stretch pb-28",
              BODY_PADDING,
            )}
          >
            <PostHogPageView />
            {children}
          </main>
          <Providers />
        </body>
      </PosthogProvider>
    </html>
  );
}
