#Stage 1: Build binary with cargo-chelf
# Using the `rust-musl-builder` as base image, instead of
# the official Rust toolchain
# doc: https://github.com/LukeMathWalker/cargo-chef
FROM clux/muslrust:stable AS chef
#install cargo-chef to do cache dependencies
RUN cargo install cargo-chef
WORKDIR /app

#Stage 2: Cache dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

#Stage 3: Build binary with cargo-chelf
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release
RUN mv target/${CARGO_BUILD_TARGET}/release /out

#Stage 4: Run with small image
FROM scratch
WORKDIR /user

COPY --from=builder /out/rust-api-service ./

CMD ["/user/rust-api-service"]
