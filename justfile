#!/usr/bin/env just --justfile

compose:
    zellij run -- docker compose up --build --watch dev-backend
    zellij run -- docker compose up --build --watch dev-frontend
    zellij run -- docker compose up db
    clear

alias dd := double-docker

# This does not quite work currently due to issues with zellij in the zellij-compose recipe.
double-docker:
    docker build -t double-docker:latest --file ./Dockerfile .
    docker run -it -v "/var/run/docker.sock:/var/run/docker.sock:rw" double-docker:latest

fmt:
    cargo +nightly fmt

lint:
    cargo clippy --workspace --all-features

lint-fix:
    cargo clippy --workspace --all-features --fix

alias zc := zellij-compose

# This does not work due to some of the panes not starting correctly, so that when you attach to the session, only the second two panes work.
zellij-compose:
    zellij attach --create-background container
    zellij --session container action new-pane -- just -f {{ justfile() }} compose
    zellij attach container
