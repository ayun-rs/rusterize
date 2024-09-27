ARG RUST_VERSION
FROM rust:${RUST_VERSION}-slim-bullseye

ARG INSTALL_ZSH=false
RUN if [ ${INSTALL_ZSH} = true ]; then \
    apt-get update && \
    apt-get install -y git curl zsh && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    git config --global init.defaultBranch master && \
    chsh -s $(which zsh) &&\
    sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"; \
    fi;

ARG INSTALL_WATCH=false
RUN if [ ${INSTALL_WATCH} = true ]; then \
    cargo install cargo-watch;\
    fi;

ARG INSTALL_SEA_ORM=false
RUN if [ ${INSTALL_SEA_ORM} = true ]; then \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev curl zsh && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    cargo install sea-orm-cli;\
    fi;

WORKDIR /home