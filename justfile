#!/usr/bin/env just --justfile

compose:
    zellij run -- docker compose up --build --watch dev-backend
    zellij run -- docker compose up --build --watch dev-frontend
    zellij run -- docker compose up db

alias dd := double-docker

double-docker:
    docker build -t double-docker:latest --file ./Dockerfile .
    docker run -it -v "/var/run/docker.sock:/var/run/docker.sock:rw" double-docker:latest /bin/sh

fmt:
    cargo +nightly fmt

lint:
    cargo clippy --workspace --all-features

lint-fix:
    cargo clippy --workspace --all-features --fix

alias zc := zellij-compose

zellij-compose:
    zellij attach --create-background container
    zellij --session container action new-pane -- just -f {{ justfile() }} compose
    zellij attach container
