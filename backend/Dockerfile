FROM rust:alpine

EXPOSE 8080

# Add alpine dependencies.
RUN apk add clang curl mold musl-dev perl

# Install nightly toolchain and add cranelift.
RUN rustup install nightly
RUN rustup component add rustc-codegen-cranelift-preview --toolchain nightly

# Copy project files over.
COPY . /backend-workspace
WORKDIR /backend-workspace/backend