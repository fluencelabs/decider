#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

DISTRO_DIR=src/distro/decider-spell

mkdir -p "$DISTRO_DIR"

fluence dep i
# compile connector
fluence build
cp example/Config.toml "$DISTRO_DIR"

# compile aqua file
cp src/compiled-aqua/worker_spell.main.air "$DISTRO_DIR/"
cp src/compiled-aqua/main.main.air "$DISTRO_DIR/"

# compile distro
cd src/distro
cargo build
