#!/usr/bin/env just --justfile

compose:
    zellij run -- docker compose up --build --watch dev-backend
    zellij run -- docker compose up --build --watch dev-frontend
    zellij run -- docker compose up db

fmt:
    cd ./backend && cargo +nightly fmt
    cd ./frontend && cargo +nightly fmt
    cd ./shared && cargo +nightly fmt

lint:
    cargo clippy --workspace --all-features

lint-fix:
    cargo clippy --workspace --all-features --fix
