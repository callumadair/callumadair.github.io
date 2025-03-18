#!/usr/bin/env just --justfile

clean:
    cargo clean

compose:
    zellij --layout ./compose.kdl

alias dd := double-docker

# This does not quite work currently due to issues with zellij in the zellij-compose recipe.
double-docker:
    docker build -t double-docker:latest --file ./Dockerfile .
    docker run -it -v "/var/run/docker.sock:/var/run/docker.sock:rw" double-docker:latest

dev:
    zellij --layout ./dev.kdl

fmt:
    cargo +nightly fmt

lint:
    cargo clippy --workspace --all-features

lint-fix:
    cargo clippy --workspace --all-features --fix
