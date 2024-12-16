FROM rust:1.83 AS builder

WORKDIR /app
COPY ../Cargo.lock ../Cargo.toml ./
COPY ../cancellations-api ./cancellations-api
COPY ../common ./common
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release .
EXPOSE 3000
CMD ["./cancellations-api"]
