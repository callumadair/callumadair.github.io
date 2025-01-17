FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

RUN curl -fsSL https://bun.sh/install | bash
RUN ~/.bun/bin/bun install -D tailwindcss
RUN ~/.bun/bin/bun i -D daisyui@latest

EXPOSE 12000

COPY . /portfolio-web-app

WORKDIR /portfolio-web-app

LABEL authors="cal"

