FROM rust:alpine

RUN apk add curl bash
RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall trunk
RUN curl -fsSL https://bun.sh/install | bash

LABEL authors="cal"
