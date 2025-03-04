FROM rust:slim-bookworm as builder

WORKDIR /ip

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim as runtime

WORKDIR /ip

COPY --from=builder /ip/target/release/api api
COPY config config
COPY client/dist client/dist

ENV CONFIG production

ENTRYPOINT ["./api"]