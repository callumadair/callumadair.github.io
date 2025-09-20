FROM rust:alpine
LABEL authors="Callum Adair"

RUN apk add alsa-lib-dev
RUN apk add bash 
RUN apk add clang 
RUN apk add curl 
RUN apk add file 
RUN apk add glib
RUN apk add mold 
RUN apk add musl-dev 
RUN apk add openssl-dev

# Add additional rustup components as needed
RUN rustup component add clippy
RUN rustup +nightly component add rustfmt
RUN rustup target add wasm32-unknown-unknown

# Add binary tools for CI/CD
RUN cargo install cargo-binstall
RUN cargo binstall cargo-llvm-cov
RUN cargo binstall cargo-nextest
RUN cargo binstall dioxus-cli

# This enables us to skip the build script, and not use tailwind etc.
ENV SKIP_BUILD_SCRIPT="1"
