FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo install --locked cargo-watch
COPY . .
RUN cargo build --release
ENV SQLX_OFFLINE true

FROM node:20-slim AS frontend-builder
WORKDIR /client
COPY client/package*.json ./
RUN npm ci
COPY client .
RUN npm run build

FROM debian:bookworm-slim as runtime
WORKDIR /EloStealo
COPY --from=builder /app/target/release/api api
COPY config config
COPY --from=frontend-builder /client/dist client/dist
ENV CONFIG docker

ENTRYPOINT ["./api"]