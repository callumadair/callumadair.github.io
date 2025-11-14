FROM rust:alpine
LABEL authors="Callum Adair"

EXPOSE 8080
ENV RUSTFLAGS="-C target-feature=-crt-static"
# Add alpine dependencies.
RUN apk add alsa-lib-dev 
RUN apk add bash
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

# Add rustfmt, clippy, and rust components.
RUN rustup component add rustfmt --toolchain nightly
RUN rustup component add --toolchain stable-x86_64-unknown-linux-musl cargo clippy rust-std rustc

# Add tools for ci
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-llvm-cov
RUN cargo binstall cargo-nextest
RUN cargo binstall gitlab_clippy