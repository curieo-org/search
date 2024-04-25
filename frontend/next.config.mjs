/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: true,
    async rewrites() {
        return {
            beforeFiles: [
                {
                    source: "/custodian/:path*",
                    destination: `${process.env.NEXT_PUBLIC_API_URL}/:path*`,
                },
                {
                    source: "/ingest/static/:path*",
                    destination: "https://us-assets.i.posthog.com/static/:path*",
                },
                {
                    source: "/ingest/:path*",
                    destination: "https://us.i.posthog.com/:path*",
                },
            ],
        };
    },
};

export default nextConfig;
