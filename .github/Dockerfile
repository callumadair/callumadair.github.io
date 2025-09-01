FROM rust:alpine
LABEL authors="Callum Adair"

# RUN apk add alsa-lib-dev
# RUN apk add build-base
# RUN apk add curl 
# RUN apk add file 
# # RUN apk add gtk+3.0-dev 
# # RUN apk add glib-dev 
# # RUN apk add libayatana-appindicator-dev 
# RUN apk add librsvg 
# RUN apk add libsoup3-dev 
# RUN apk add pango 
# RUN apk add perl about:blank#blocked
# RUN apk add wget 
# RUN apk add webkit2gtk-4.1 
# RUN apk add webkit2gtk-4.1-dev

RUN apk add alsa-lib-dev
RUN apk add bash 
RUN apk add clang 
RUN apk add curl 
RUN apk add file 
# RUN apk add glib-dev 
RUN apk add glib
RUN apk add mold 
RUN apk add musl-dev 
RUN apk add libressl-dev

# Add additional rustup components as needed
RUN rustup component add clippy
RUN rustup +nightly component add rustfmt
RUN rustup target add wasm32-unknown-unknown

# Add binary tools for CI/CD
RUN cargo install cargo-binstall
RUN cargo binstall cargo-llvm-cov
RUN cargo binstall cargo-nextest
RUN cargo binstall dioxus-cli

# This enables us to skip the build script, and not use tailwind etc.
ENV SKIP_BUILD_SCRIPT="1"
