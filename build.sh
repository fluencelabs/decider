#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

DISTRO_DIR=src/distro/decider-spell

mkdir -p "$DISTRO_DIR"

# compile aqua file
fluence aqua -i src/aqua/worker.aqua -o "$DISTRO_DIR" --air
fluence aqua -i src/aqua/decider.aqua -o "$DISTRO_DIR" --air

# compile connector
fluence build

# compile distro
cd src/distro
cargo build
