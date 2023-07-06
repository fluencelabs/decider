#!/usr/bin/env bash

set -o pipefail -o nounset -o errexit

dir="./src/air"

# example/periodic.json or example/oneshot.json
decider_config="$1"
# example/decider_init_args.json
decider_args="$2"
# example/periodic.json
worker_config="$3"

# compile deal_spell.aqua to deal_spell/deal_spell.main.air
fluence aqua -i src/aqua/decider/deal_spell.aqua -o "$dir/deal_spell" --air
# fluence aqua -i src/aqua/worker.aqua -o "$dir" --air
echo "compiled deal spell"

# compile decider/poll.aqua to decider/poll.main.air
fluence aqua --import src/aqua/decider -i src/aqua/decider/poll.aqua -o "$dir/poll" --air
# fluence aqua -i src/aqua/decider.aqua -o "$dir" --air
echo "compiled decider's poll"

# create worker_settings.json
jq -c --arg script "$(cat $dir/deal_spell/deal_spell.main.air | awk '{$1=$1};1')" '{"worker_script": $script, "worker_config": .}' "$worker_config" > "$dir"/worker_settings.json
echo "create worker settings"

jq -s '.[0] * .[1]' "$decider_args" "$dir"/worker_settings.json > "$dir"/init.json
echo "create initial data for decider"

# Need json with:
# "script": decider script
# "join_all_deals": "$FLUENCE_ENV_CONNECTOR_JOIN_ALL_DEALS"
# "cfg": decider cfg
# "dat":
#     "listener_id": aurora listener service
#     "info":
#     	 "api_endpoint": api endpoint from which to poll
#     	 "address": contract address
#      "from_block": "$FLUENCE_ENV_CONNECTOR_FROM_BLOCK"
#      "worker_script": worker.aqua script
#      "worker_config": periodic worker config worker_config.json
#      "worker_ipfs": IPFS API address from which to get apps
jq -s --arg script "$(cat $dir/poll/poll.main.air | awk '{$1=$1};1')" '{ "script": $script, "join_all_deals": "$FLUENCE_ENV_CONNECTOR_JOIN_ALL_DEALS", "cfg": .[0], "dat": .[1]}' "$decider_config" "$dir"/init.json > decider.json
echo "Compiled to decider.json"
