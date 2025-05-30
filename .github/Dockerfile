FROM rust:alpine
LABEL authors="Callum Adair"

RUN apk add alsa-lib-dev bash clang curl gtk+3.0 glib glib-dev mold musl-dev openssl-dev pango perl
RUN rustup component add clippy
RUN rustup +nightly component add rustfmt

# This enables us to skip the build script, and not use tailwind etc.
ENV SKIP_BUILD_SCRIPT="1"
