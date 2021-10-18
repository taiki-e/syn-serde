#!/bin/bash

# Run code generators.
#
# Usage:
#    ./tools/gen.sh

set -euo pipefail
IFS=$'\n\t'

cd "$(cd "$(dirname "$0")" && pwd)"/..

cargo run --manifest-path tools/codegen/Cargo.toml

cargo run --manifest-path examples/rust2json/Cargo.toml -- examples/rust2json/src/main.rs examples/rust2json/main.json
cargo run --manifest-path examples/rust2json/Cargo.toml -- examples/json2rust/src/main.rs examples/json2rust/main.json
cargo run --manifest-path examples/rust2json/Cargo.toml -- examples/rust2pickle/src/main.rs examples/rust2pickle/main.json
