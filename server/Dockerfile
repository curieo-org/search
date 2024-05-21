FROM rust:1.77 AS planner

WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.77 AS cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.77 AS builder

RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin server

FROM debian:bookworm-slim AS runtime
WORKDIR app
COPY config config
COPY --from=builder /app/target/release/server /usr/local/bin
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

RUN useradd -u 1000 server

USER server

EXPOSE 3030

ENTRYPOINT ["/usr/local/bin/server"]
