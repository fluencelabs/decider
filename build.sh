#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

DISTRO_DIR=src/distro/decider-spell

mkdir -p "$DISTRO_DIR"

fluence dep i
# compile connector
fluence build
cp target/wasm32-wasi/release/chain_connector.wasm "$DISTRO_DIR"
cp target/wasm32-wasi/release/curl_adapter.wasm "$DISTRO_DIR"
cp example/Config.toml "$DISTRO_DIR"

# compile aqua file
cp src/compiled-aqua/deal_spell.main.air "$DISTRO_DIR/"
cp src/compiled-aqua/poll.main.air "$DISTRO_DIR/"

# compile distro
cd src/distro
cargo build
