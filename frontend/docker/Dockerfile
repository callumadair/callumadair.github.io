FROM rust:alpine

RUN apk add bash clang curl mold musl-dev libressl-dev

# Add rust toolchains and components.
RUN rustup target add wasm32-unknown-unknown
RUN rustup install nightly
RUN rustup component add rustc-codegen-cranelift-preview --toolchain nightly

# Install dx.
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli

# Install bun and add it to the path.
ENV BUN_INSTALL="/"
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="$BUN_INSTALL/bin:$PATH"

LABEL authors="cal"
