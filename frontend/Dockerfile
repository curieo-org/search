FROM node:20-alpine

RUN apk add --no-cache g++ make py3-pip libc6-compat

WORKDIR /app

COPY package*.json ./
RUN npm ci

COPY src ./src
COPY public ./public
COPY next.config.mjs .
COPY postcss.config.mjs .
COPY tailwind.config.ts .
COPY tsconfig.json .

ARG POSTHOG_KEY
ARG POSTHOG_API_HOST
ARG POSTHOG_UI_HOST
ARG API_URL

ENV NEXT_TELEMETRY_DISABLED 1
ENV NEXT_PUBLIC_POSTHOG_KEY $POSTHOG_KEY
ENV NEXT_PUBLIC_POSTHOG_API_HOST $POSTHOG_API_HOST
ENV NEXT_PUBLIC_POSTHOG_UI_HOST $POSTHOG_UI_HOST
ENV NEXT_PUBLIC_API_URL $API_URL

RUN npm run build

EXPOSE 3000

CMD npm start