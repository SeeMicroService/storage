FROM rust:1.83 AS builder
WORKDIR /storage
COPY . .
RUN cargo build --release --bin storage

FROM debian:bookworm-slim
WORKDIR /storage
RUN apt-get update && apt-get install -y libssl3 && apt-get clean
COPY --from=builder /storage/target/release/storage /usr/local/bin
CMD ["storage"]
