#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

DISTRO_DIR=src/distro/decider-spell

mkdir -p "$DISTRO_DIR"

export CC=/opt/homebrew/opt/llvm/bin/clang
export AR=/opt/homebrew/opt/llvm/bin/llvm-ar
fluence dep i
# compile connector
fluence build
cp target/wasm32-wasi/release/fluence_aurora_connector.wasm src/distro/decider-spell/
cp target/wasm32-wasi/release/curl_adapter.wasm src/distro/decider-spell/
cp example/Config.toml src/distro/decider-spell/

# compile aqua file
fluence aqua --import src/aqua --import src/aqua/decider --import src/aqua/chain --import src/aqua/fluence -i src/aqua/decider/deal_spell.aqua -o "$DISTRO_DIR/deal_spell" --air
fluence aqua --import src/aqua --import src/aqua/decider --import src/aqua/chain --import src/aqua/fluence -i src/aqua/decider/poll.aqua -o "$DISTRO_DIR/poll" --air

# compile distro
cd src/distro
cargo build
