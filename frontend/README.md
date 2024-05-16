# Curieo Search Frontend

## Prerequisites
- Node.js
- NPM
- Docker (for production image creation)

## Configuration
Copy the `.env.template` file to `.env.local` and update the values as needed.
```bash
cp .env.template .env.local
```

## Installation
```bash
npm install
npm run build
```

## Running
```bash
npm run dev
```

## Production Docker Image Creation
```bash
# Use a new tag for the image
TAG = <TAG>

# Update the following lines in the `Makefile` with the correct values
POSTHOG_KEY = <POSTHOG_API_KEY>
POSTHOG_API_HOST = <FRONTEND_URL>/ingest
API_URL = <BACKEND_URL>

# Build and push the image
make
```
