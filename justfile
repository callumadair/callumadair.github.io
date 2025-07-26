#!/usr/bin/env just --justfile

alias c := clean

# Clean the build artifacts.
clean:
    cargo clean

alias C := compose

# Run the backend, frontend and database using the compose configurations.
compose:
    zellij --layout ./.zellij/compose.kdl

alias dd := double-docker

# Run the compose recipe from a base container.
double-docker:
    docker build -t double-docker:latest --file ./Dockerfile .
    docker run -it -v "/var/run/docker.sock:/var/run/docker.sock:rw" double-docker:latest

alias d := dev

# Run the backend and frontend locally, with the database in a container.
dev:
    zellij --layout ./.zellij/dev.kdl

alias A := devtools-arch

# Install devtools for the user on ArchLinux.
devtools-arch:
    sudo pacman -S bacon dioxus-cli just rustup  zellij
    rustup target add wasm32-unknown-unknown
    curl -fsSL https://bun.sh/install | bash
    bun install -D tailwindcss

alias D := devtools

# Install the required devtools assuming a platform independent approach.
devtools:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup target add wasm32-unknown-unknown
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    cargo binstall dioxus-cli just zellij bacon
    curl -fsSL https://bun.sh/install | bash
    bun install -D tailwindcss

alias f := fmt

# Run cargo fmt with the nightly toolchain for access to more rustfmt options.
fmt:
    cargo +nightly fmt

alias l := lint

# Run cargo clippy across the workspace.
lint:
    cargo clippy --workspace --all-features

alias L := lint-fix

# Run cargo clippy fix across the workspace.
lint-fix:
    cargo clippy --workspace --all-features --fix

alias u := update

# Update all cargo dependencies in the local crate.
update:
    cargo update

alias U := update-workspace

# Update just the workspace level dependencies.
update-workspace:
    cargo update --workspace
