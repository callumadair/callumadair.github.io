FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN curl -fsSL https://bun.sh/install | bash

LABEL authors="cal"
