ARG RUST_VERSION
FROM rust:${RUST_VERSION}-slim-bullseye

# Set working directory
WORKDIR /home

# Install nu
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    apt-get clean && \
    cargo install nu

# Install cargo-watch
RUN cargo install cargo-watch sea-orm-cli