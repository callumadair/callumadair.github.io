FROM rust:alpine
LABEL authors="Callum Adair"

RUN apk add \
    alsa-lib-dev \
    build-base \
    clang \
    curl \
    file \
    gtk+3.0-dev \
    glib-dev \
    libayatana-appindicator-dev \
    librsvg \
    libsoup3-dev \
    mold \
    musl-dev \
    openssl \
    openssl-dev \
    pango \
    perl \
    wget \
    webkit2gtk-4.1 \
    webkit2gtk-4.1-dev


# Add additional rustup components as needed
RUN rustup component add clippy
RUN rustup +nightly component add rustfmt

# Add binary tools for CI/CD
RUN cargo install cargo-binstall
RUN cargo binstall cargo-llvm-cov
RUN cargo binstall cargo-nextest

# This enables us to skip the build script, and not use tailwind etc.
ENV SKIP_BUILD_SCRIPT="1"
