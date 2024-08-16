ARG RUST_VERSION
FROM rust:${RUST_VERSION}-slim-bullseye

WORKDIR /home

ARG INSTALL_WATCH=false
RUN if [ ${INSTALL_WATCH} = true ]; then \
    cargo install cargo-watch;\
fi;

ARG INSTALL_SEA_ORM=false
RUN if [ ${INSTALL_SEA_ORM} = true ]; then \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    apt-get clean && \
    cargo install sea-orm-cli;\
fi;

EXPOSE 8000