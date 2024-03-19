FROM rust:1.76.0 as builder

WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/server ./server

FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/server  /usr/local/bin/server

COPY ./config ./config

RUN useradd -u 1000 server

USER server

EXPOSE 3030

CMD ["server"]
