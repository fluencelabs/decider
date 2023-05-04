#!/usr/bin/env bash
set -o pipefail -o nounset -o errexit

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"
PACKAGE_DIR="$(pwd)/../connector"

(
    rm -rf $PACKAGE_DIR
    mkdir -p $PACKAGE_DIR
)

(

    echo "*** build fluence_aurora_connector ***"
    cd ..
    fluence build
)

(
    echo "*** copy wasm files ***"
    cd ..
    cp target/wasm32-wasi/release/fluence_aurora_connector.wasm "$PACKAGE_DIR"
    cp target/wasm32-wasi/release/curl_adapter.wasm "$PACKAGE_DIR"
)

(
    echo "*** compile spell scripts ***"
    cd ..
    ./scripts/compile.sh example/periodic.json example/decider_init_args.json example/periodic.json
    cp decider.json "$PACKAGE_DIR"/on_start.json
)

(
    echo "*** copy on_start script ***"
    cp on_start.air "$PACKAGE_DIR"
)

CONNECTOR_CID=$(ipfs add -q --only-hash --cid-version=1 --chunker=size-262144 $PACKAGE_DIR/fluence_aurora_connector.wasm)
CURL_CID=$(ipfs add -q --only-hash --cid-version=1 --chunker=size-262144 $PACKAGE_DIR/curl_adapter.wasm)
mv $PACKAGE_DIR/fluence_aurora_connector.wasm "$PACKAGE_DIR"/"$CONNECTOR_CID".wasm
mv $PACKAGE_DIR/curl_adapter.wasm "$PACKAGE_DIR"/"$CURL_CID".wasm
cp fluence_aurora_connector_config.json "$PACKAGE_DIR"/"$CONNECTOR_CID"_config.json
cp curl_adapter_config.json "$PACKAGE_DIR"/"$CURL_CID"_config.json

# write blueprint.json
echo "{}" | jq --arg trust_graph_cid "$CONNECTOR_CID" --arg sqlite_cid "$CURL_CID" '{"name": "connector", "dependencies":[{"/":$sqlite_cid},{"/":$trust_graph_cid}]}' > "$PACKAGE_DIR/blueprint.json"

(
    echo "*** create builtin distribution package ***"
    cd ..

    tar -f connector.tar.gz -zcv ./connector

)

echo "*** done ***"
