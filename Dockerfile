FROM rust:1.70 AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev && apt-get clean
COPY --from=builder /usr/src/app/target/release/HolaMundo /usr/local/bin/HolaMundo

# Cambiamos a puerto 8000 para mantener consistencia
EXPOSE 8000

CMD ["/usr/local/bin/HolaMundo"]