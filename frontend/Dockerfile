FROM rust:latest

COPY . /frontend-workspace

EXPOSE 3000

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /frontend-workspace/frontend

RUN curl -fsSL https://bun.sh/install | bash
RUN ~/.bun/bin/bun install -D tailwindcss
RUN ~/.bun/bin/bun i -D daisyui@latest
RUN trunk build

LABEL authors="cal"
