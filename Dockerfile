FROM rust:1.70-slim as builder

WORKDIR /usr/src/cli2rest
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/cli2rest/target/release/cli2rest /usr/local/bin/

EXPOSE 8000
ENTRYPOINT ["cli2rest"]
