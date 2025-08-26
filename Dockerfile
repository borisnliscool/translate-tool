FROM rust:1.89-alpine AS builder

RUN apk add --no-cache gcc musl-dev libc-dev

WORKDIR /app

COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --all --release && strip target/release/translate-tool

FROM alpine:latest

WORKDIR /

COPY --from=builder /app/target/release/translate-tool /translate-tool
RUN chmod +x /translate-tool

RUN echo '{"translations_directory": "locales"}' > tt.config.json

LABEL org.opencontainers.image.source=https://github.com/borisnliscool/translate-tool