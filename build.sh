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
cp src/compiled-aqua/deal_spell.main.air "$DISTRO_DIR/"
cp src/compiled-aqua/poll.main.air "$DISTRO_DIR/"

# compile distro
cd src/distro
cargo build
