#!/bin/bash

source $HOME/.functions.sh

reset_path


# Use 'trunk' for client-side rendering (CSR) with Leptos and WASM
#trunk serve --port 3000 --open

cargo leptos watch

