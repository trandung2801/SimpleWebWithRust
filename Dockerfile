FROM rust:latest AS builder
USER root

RUN cargo install bunyan

WORKDIR /app
COPY . .


RUN cargo build

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/release/rust-api-service ./

CMD ["/app/rust-api-service"]