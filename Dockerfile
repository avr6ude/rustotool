FROM rust:alpine AS builder

WORKDIR /usr/src/app

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY config.yaml ./
COPY migrations ./migrations
COPY src src

ENV OPENSSL_STATIC=1
ENV OPENSSL_DIR=/usr

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/utotool-rust /usr/local/bin/utotool-rust

CMD ["utotool-rust"]
