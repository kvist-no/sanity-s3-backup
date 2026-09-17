FROM rust:1.98 AS builder
WORKDIR /usr/src/sanity-s3-backup
COPY . .
# --locked builds the committed Cargo.lock instead of resolving the newest
# crates, some of which no longer parse on older toolchains.
RUN cargo install --path . --locked

FROM debian:bookworm-slim

RUN apt update && apt install -y openssl ca-certificates

COPY --from=builder /usr/local/cargo/bin/sanity-s3-backup /usr/local/bin/sanity-s3-backup
CMD ["sanity-s3-backup"]