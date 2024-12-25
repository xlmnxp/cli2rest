FROM rust:latest

WORKDIR /app

COPY . .
RUN cargo build --release

EXPOSE 8000
ENTRYPOINT ["/app/target/release/cli2rest"]
