#!/bin/bash

set -efu

dir="./src/air/"

# example/periodic.json or example/oneshot.json
decider_config="$1"
# example/decider_init_args.json
decider_args="$2"
# example/periodic.json
worker_config="$3"

# compile worker.aqua to worker.main.air
fluence aqua -i src/aqua/worker.aqua -o "$dir" --air
echo "compiled worker"

# compile decider.aqua to decider.main.air
fluence aqua -i src/aqua/decider.aqua -o "$dir" --air
echo "compiled decider"

# create worker_settings.json
jq -c --arg script "$(cat $dir/worker.main.air | awk '{$1=$1};1')" '{"worker_script": $script, "worker_config": .}' "$worker_config" > "$dir"/worker_settings.json
echo "create worker settings"

jq -s '.[0] * .[1]' "$decider_args" "$dir"/worker_settings.json > "$dir"/init.json
echo "create initial data for decider"

# Need json with:
# "script": decider script
# "with_decider": "$FLUENCE_ENV_CONNECTOR_JOIN_ALL_DEALS"
# "cfg": decider cfg
# "dat":
#     "listener_id": aurora listener service
#     "info":
#     	 "net": net from which to poll
#     	 "address": contract address
#      "from_block": "latest"
#      "worker_script": worker.aqua script
#      "worker_config": periodic worker config worker_config.json
#      "worker_ipfs": IPFS API address from which to get apps
jq -s --arg script "$(cat $dir/decider.main.air | awk '{$1=$1};1')" '{ "script": $script, "with_decider": "$FLUENCE_ENV_CONNECTOR_JOIN_ALL_DEALS", "cfg": .[0], "dat": .[1]}' "$decider_config" "$dir"/init.json > decider.json
echo "Compiled to decider.json"
