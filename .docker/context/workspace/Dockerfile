ARG RUST_VERSION
FROM rust:${RUST_VERSION}-slim-bullseye

WORKDIR /root

COPY . .

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

ARG INSTALL_NUSHELL=false
RUN if [ ${INSTALL_NUSHELL} = true ]; then \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev cmake && \
    apt-get clean && \
    cargo install nu starship && mkdir -p ~/.cache/starship && \
    nu && starship init nu | save -f ~/.cache/starship/init.nu &&\
    echo "use ~/.cache/starship/init.nu" >> ~/.config/nushell/config.nu;\
fi;