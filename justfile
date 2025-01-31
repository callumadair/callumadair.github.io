#!/usr/bin/env just --justfile

fmt:
    cd ./backend && cargo +nightly fmt
    cd ./frontend && cargo +nightly fmt
    cd ./shared && cargo +nightly fmt
