#!/usr/bin/env just --justfile

clean:
    cargo clean

compose:
    zellij --layout ./zellij/compose.kdl

alias dd := double-docker

# This does not quite work currently due to issues with zellij in the zellij-compose recipe.
double-docker:
    docker build -t double-docker:latest --file ./Dockerfile .
    docker run -it -v "/var/run/docker.sock:/var/run/docker.sock:rw" double-docker:latest

dev:
    zellij -n ./zellij/dev.kdl

devtools:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup target add wasm32-unknown-unknown
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    cargo binstall dioxus-cli just zellij bacon
    curl -fsSL https://bun.sh/install | bash
    bun install -D tailwindcss

fmt:
    cargo +nightly fmt

lint:
    cargo clippy --workspace --all-features

lint-fix:
    cargo clippy --workspace --all-features --fix

alias u := update

update:
  cargo update
