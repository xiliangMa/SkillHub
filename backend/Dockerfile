FROM rustlang/rust:nightly AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools musl-dev libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml ./

RUN cargo fetch || true

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl --bin skillhub-api

FROM alpine:3.19

RUN apk add --no-cache openssl ca-certificates libgcc

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/skillhub-api /skillhub-api

EXPOSE 8080

CMD ["/skillhub-api"]
