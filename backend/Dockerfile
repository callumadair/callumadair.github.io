FROM rust:alpine

EXPOSE 8080

# Add alpine dependencies.
RUN apk add clang curl mold musl-dev perl

# Add sea-orm cli
RUN cargo install sea-orm-cli@1.1.0

# Install nightly toolchain and add cranelift.
RUN rustup install nightly
RUN rustup component add rustc-codegen-cranelift-preview --toolchain nightly

# Copy project files over.
COPY . /backend-workspace
WORKDIR /backend-workspace/backend