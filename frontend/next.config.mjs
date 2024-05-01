/** @type {import('next').NextConfig} */
const nextConfig = {
  env: {
    NEXTAUTH_SECRET: process.env.NEXTAUTH_SECRET,
    NEXTAUTH_URL: process.env.NEXTAUTH_URL,
    API_BASE_URL: process.env.API_BASE_URL,
  },
  output: 'standalone',
}

export default nextConfig
