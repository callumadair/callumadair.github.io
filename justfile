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
    sudo pacman -S bacon clang dioxus-cli just mold rustup  zellij
    rustup target add wasm32-unknown-unknown
    curl -fsSL https://bun.sh/install | bash
    source ~/.zshrc
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

alias uc := update

# Update all cargo dependencies in the local crate.
update:
    cargo update

alias uw := update-workspace

# Update just the workspace level dependencies.
update-workspace:
    cargo update --workspace

alias U := update-all

update-all:
    cd nuxt-frontend && bun update
    z backend && cargo update --workspace

alias P := git-push
    
git-push:
    git remote | xargs -L1 git push --all

alias bbd := build-backend-docker
build-backend-docker:
    docker build -t callumadair/portfolio-backend:latest --file ./backend/docker/Dockerfile .

alias pbd := push-backend-docker
push-backend-docker:
    docker push callumadair/portfolio-backend:latest

alias bfd := build-frontend-docker
build-frontend-docker:
    docker build -t callumadair/nuxt-portfolio-frontend:latest --file ./nuxt-frontend/Dockerfile .

alias pfd := push-frontend-docker
push-frontend-docker:
    docker build -t callumadair/nuxt-portfolio-frontend:latest

alias ci := ci-jobs
ci-jobs:
    #!/usr/bin/env bash
    cd backend
    cargo run fmt --check
    cargo clippy --workspace
    cargo llvm nextest