FROM rust:alpine
LABEL authors="Callum Adair"

EXPOSE 8080
ENV RUSTFLAGS="-C target-feature=-crt-static"
# Add alpine dependencies.
RUN apk add alsa-lib-dev 
RUN apk add clang 
RUN apk add curl 
RUN apk add mold 
RUN apk add musl-dev 
RUN apk add openssl-dev 
RUN apk add perl

# Add sea-orm cli
RUN cargo install --locked sea-orm-cli@1.1.0
RUN cargo install --locked bacon --all-features

# Install nightly toolchain and add cranelift.
RUN rustup install nightly
RUN rustup component add rustc-codegen-cranelift-preview --toolchain nightly
