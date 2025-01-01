devtools:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup target add wasm32-unknown-unknown
    cargo install --locked trunk
    curl -fsSL https://bun.sh/install | bash
    bun install -D tailwindcss

serve:
    trunk serve