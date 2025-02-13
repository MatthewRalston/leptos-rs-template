#!/bin/bash

source $HOME/.functions.sh

reset_path


# Use 'trunk' for client-side rendering (CSR) with Leptos and WASM
#trunk serve --port 3000 --open
# cargo build --no-default-features --features=ssr # Just build server

# Use cargo-leptos build to install dependencies
#cargo leptos build

# Start server with cargo leptos
cargo leptos watch

