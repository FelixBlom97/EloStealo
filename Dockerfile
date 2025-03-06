FROM rust:slim-bookworm as builder

WORKDIR /EloStealo

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim as runtime

WORKDIR /EloStealo

COPY --from=builder /EloStealo/target/release/api api
COPY config config
COPY client/dist client/dist

ENV CONFIG production

ENTRYPOINT ["./api"]