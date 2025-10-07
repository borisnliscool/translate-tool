FROM rust:1.89-alpine AS builder

RUN apk add --no-cache gcc musl-dev libc-dev

WORKDIR /app

COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --all --release && strip target/release/translate-tool

FROM debian:bookworm-slim

WORKDIR /

COPY --from=builder /app/target/release/translate-tool /bin/translate-tool
RUN chmod +x /bin/translate-tool

LABEL org.opencontainers.image.source=https://github.com/borisnliscool/translate-tool

COPY ./motd /root/motd
RUN echo "alias tt=translate-tool" >> "/root/.bashrc"
RUN echo "echo -e \"\$(cat $HOME/motd)\"" >> "/root/.bashrc"
RUN echo "echo" >> "/root/.bashrc"

CMD ["bash"]