FROM rust:1.80-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim as final

WORKDIR /usr/app

COPY --from=builder /usr/src/target/release/app ./run

CMD ["./run"]