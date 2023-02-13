#!/bin/bash

set -efu

dir=$(mktemp -d)

trap 'rm -rf -- "$dir"' EXIT

# example/periodic.json or example/oneshot.json
decider_config="$1"
# example/decider_init_args.json
decider_args="$2"
# example/periodic.json
worker_config="$3"

# compile worker.aqua to worker.main.air
fluence aqua -i src/aqua/worker.aqua -o "$dir" --air --no-relay
echo "compiled worker"

# compile decider.aqua to decider.main.air
fluence aqua -i src/aqua/decider.aqua -o "$dir" --air --no-relay
echo "compiled decider"

# create worker_settings.json
jq --arg script "$(cat $dir/worker.main.air)" '{"worker_script": $script, "worker_config": .}' "$worker_config" > "$dir"/worker_settings.json
echo "create worker settings"

jq -s '.[0] * .[1]' "$decider_args" "$dir"/worker_settings.json > "$dir"/init.json
echo "create initial data for decider"

# Need json with:
# "script": decider script
# "cfg": decider cfg
# "dat":
#     "listener_id": aurora listener service
#     "info":
#     	 "net": net from which to poll
#     	 "address": contract address
#     	 "topics": created deal topic
#      "from_block": "latest"
#      "worker_script": worker.aqua script
#      "worker_config": periodic worker config worker_config.json
jq -s --arg script "$(cat $dir/decider.main.air )" '{ "script": $script, "cfg": .[0], "dat": .[1]}' "$decider_config" "$dir"/init.json > decider.json
echo "Compiled to decider.json"
