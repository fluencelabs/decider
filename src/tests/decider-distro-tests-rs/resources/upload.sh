#!/usr/bin/env bash
set -o errexit -o nounset

cd $(dirname "$0")

for proj in $(find . -mindepth 1 -type d)
do
    cd "$proj"
    for fl in $(find . -mindepth 1 -type f)
    do
        echo "uploading file $fl"
    	ipfs add --cid-version 1 --hash sha2-256 -r "$fl" --api /ip4/127.0.0.1/tcp/5001 -Q
    done
    cd -
done
