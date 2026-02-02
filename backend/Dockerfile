FROM rustlang/rust:nightly AS builder

WORKDIR /app

COPY Cargo.toml ./

RUN cargo fetch || true

COPY . .

RUN cargo build --release --bin skillhub-api

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/skillhub-api /skillhub-api

EXPOSE 8080

CMD ["/skillhub-api"]
