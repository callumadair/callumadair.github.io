# Just my personal site for testing out the feasibility of tech stacks

## Running this locally

In order to run this site, you will need `Rustup`, a `Rust`, the rust `wasm32-unknown-unknown` target, `trunk`, `bun` (
or equivalent), `tailwindcss` and `daisyUI`.

These can be installed by running `just devtools` and a local server run with `trunk serve`.

## Running in a docker container

This is a simple setup using docker and docker-compose.

```shell
docker compose up
```

## Accessing a development site

Simply go to [localhost:12000](http://localhost:12000)