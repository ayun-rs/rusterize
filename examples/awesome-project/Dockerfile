ARG RUST_VERSION
ARG BIN_NAME

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG BIN_NAME

WORKDIR /app

COPY . .

RUN cargo build --locked --release --bin=$BIN_NAME

RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/

FROM debian:bullseye-slim AS final
ARG BIN_NAME

WORKDIR /home

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /app/target/release/$BIN_NAME ./app

ENTRYPOINT ["/home/app"]