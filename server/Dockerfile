FROM lukemathwalker/cargo-chef:latest-rust-1.76-slim-bookworm as chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin server

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/server /usr/local/bin
COPY ./config ./config

RUN useradd -u 1000 server

USER server

EXPOSE 3030

CMD ["server"]