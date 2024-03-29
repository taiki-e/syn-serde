#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
set -eEuo pipefail
IFS=$'\n\t'
cd "$(dirname "$0")"/..

# shellcheck disable=SC2154
trap 's=$?; echo >&2 "$0: error on line "${LINENO}": ${BASH_COMMAND}"; exit ${s}' ERR

# Run code generators.
#
# USAGE:
#    ./tools/gen.sh

set -x

cargo run --manifest-path tools/codegen/Cargo.toml

cargo run --manifest-path examples/rust2json/Cargo.toml -- examples/rust2json/src/main.rs examples/rust2json/main.json
cargo run --manifest-path examples/rust2json/Cargo.toml -- examples/json2rust/src/main.rs examples/json2rust/main.json
cargo run --manifest-path examples/rust2json/Cargo.toml -- examples/rust2pickle/src/main.rs examples/rust2pickle/main.json
