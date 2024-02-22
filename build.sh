#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

DISTRO_DIR=src/distro/decider-spell

mkdir -p "$DISTRO_DIR"

fluence dep i
# compile connector
fluence build
cp target/wasm32-wasi/release/chain_connector.wasm src/distro/decider-spell/
cp target/wasm32-wasi/release/curl_adapter.wasm src/distro/decider-spell/
cp example/Config.toml src/distro/decider-spell/

cp -r .fluence/aqua-dependencies/node_modules/@fluencelabs/aqua-lib .fluence/aqua-dependencies/node_modules/@fluencelabs/installation-spell/node_modules/@fluencelabs
cp -r .fluence/aqua-dependencies/node_modules/@fluencelabs/aqua-ipfs .fluence/aqua-dependencies/node_modules/@fluencelabs/installation-spell/node_modules/@fluencelabs

# compile aqua file
fluence aqua -i src/aqua/decider/deal_spell.aqua -o "$DISTRO_DIR/deal_spell" --air
fluence aqua -i src/aqua/decider/poll.aqua -o "$DISTRO_DIR/poll" --air

# compile distro
cd src/distro
cargo build
