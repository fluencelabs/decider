#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

DISTRO_DIR=src/distro/decider-spell

mkdir -p "$DISTRO_DIR"

fluence dep i
# compile connector
fluence build
# compile aqua file
fluence aqua -i src/aqua/worker.aqua -o "$DISTRO_DIR" --air
fluence aqua -i src/aqua/decider.aqua -o "$DISTRO_DIR" --air

cp target/wasm32-wasi/release/fluence_aurora_connector.wasm src/distro/decider-spell/
cp target/wasm32-wasi/release/curl_adapter.wasm src/distro/decider-spell/
cp example/Config.toml src/distro/decider-spell/

# compile distro
cd src/distro
cargo build
