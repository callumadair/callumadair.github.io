# Just my personal site for testing out the feasibility of tech stacks

## Running this locally

In order to run this site, you will need `Rustup`, a `Rust`, the rust `wasm32-unknown-unknown` target, `dioxus-cli`,
`bun` (
or equivalent), and some others.

These can be installed by running `just devtools` and a local server run with `just dev`.

## Running in a docker container

Alternatively you can run everything in docker containers using compose. The recommended way to do this is with the just
recipe `just dd`. This only requires docker and just to be installed.

```shell
just dd
```

## Accessing a development site

Simply go to [localhost:12000](http://localhost:12000)