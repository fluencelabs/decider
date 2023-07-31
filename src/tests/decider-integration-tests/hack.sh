#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

#    "@fluencelabs/cli": "https://github.com/fluencelabs/cli#matching",

cd tests/sample_project

npx fluence $@