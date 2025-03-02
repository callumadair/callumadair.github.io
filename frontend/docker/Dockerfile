FROM rust:alpine

# Add alpine packages
RUN apk add bash clang curl mold musl-dev

# Add rust toolchains and components.
RUN rustup target add wasm32-unknown-unknown
RUN rustup install nightly
RUN rustup component add rustc-codegen-cranelift-preview --toolchain nightly

# Install dx.
RUN cargo install dioxus-cli

# Install bun and add it to the path.
RUN curl -fsSL https://bun.sh/install | bash
ENV BUN_INSTALL="/root/.bun"
ENV PATH="$BUN_INSTALL/bin:$PATH"

# Install tailwindcss cli and add it to a location on the path.
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64-musl
RUN chmod +x tailwindcss-linux-x64-musl
RUN mv tailwindcss-linux-x64-musl /usr/bin/tailwindcss

LABEL authors="cal"
