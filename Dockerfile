FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY . .

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/install-nothing /usr/local/bin/install-nothing

RUN useradd -m -u 1000 -U appuser
USER appuser

ENTRYPOINT ["install-nothing"]
