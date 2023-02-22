#!/usr/bin/env bash
set -o pipefail -o nounset -o errexit

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"
SCRIPT_DIR="$(pwd)"

(
    echo "*** compile spell scripts ***"
    cd ..
    ./scripts/compile.sh example/periodic.json example/decider_init_args.json example/periodic.json
    cp decider.json builtin-package/on_start.json
)

(
    echo "*** build fluence_aurora_connector ***"
    cd ..
    fluence build
)
(
    echo "*** copy wasm files ***"
    cd ../src/services/fluence-aurora-connector/modules/
    cp fluence_aurora_connector/target/wasm32-wasi/release/fluence_aurora_connector.wasm "$SCRIPT_DIR"
    cp curl_adapter/target/wasm32-wasi/release/curl_adapter.wasm "$SCRIPT_DIR"
)

(
    echo "*** create builtin distribution package ***"
    cd ..
    mkdir -p connector
    cp -rf builtin-package/ connector
    tar --exclude="package.sh" -f connector.tar.gz -zcv ./connector
    rm -rf connector
)

echo "*** done ***"
