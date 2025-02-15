FROM rust:alpine

RUN apk add bash curl mold musl-dev
RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall trunk

# Install bun and add it to the path.
RUN curl -fsSL https://bun.sh/install | bash
ENV BUN_INSTALL="/root/.bun"
ENV PATH="$BUN_INSTALL/bin:$PATH"

# Install tailwindcss cli and add it to a location on the path.
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64-musl
RUN chmod +x tailwindcss-linux-x64-musl
RUN mv tailwindcss-linux-x64-musl /usr/bin/tailwindcss

LABEL authors="cal"
