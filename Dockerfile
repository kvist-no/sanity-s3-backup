FROM rust:1.88.0 as builder
WORKDIR /usr/src/sanity-s3-backup
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt update && apt install -y openssl ca-certificates

COPY --from=builder /usr/local/cargo/bin/sanity-s3-backup /usr/local/bin/sanity-s3-backup
CMD ["sanity-s3-backup"]