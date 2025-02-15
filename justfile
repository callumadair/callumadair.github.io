#!/usr/bin/env just --justfile

fmt:
    cd ./backend && cargo +nightly fmt
    cd ./frontend && cargo +nightly fmt
    cd ./shared && cargo +nightly fmt

compose:
    zellij run -- docker compose up --build --watch dev-backend
    zellij run -- docker compose up --build --watch dev-frontend
    docker compose up db
