# syntax=docker/dockerfile:1.7

FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --locked --release

FROM docker:29-cli AS docker-cli

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=docker-cli /usr/local/bin/docker /usr/local/bin/docker
COPY --from=builder /app/target/release/sandbox-rs /usr/local/bin/sandbox-rs

ENV SANDBOX_BIND_ADDR=0.0.0.0:8080

EXPOSE 8080

ENTRYPOINT ["sandbox-rs"]
