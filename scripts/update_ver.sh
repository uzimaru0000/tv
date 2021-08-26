#!/usr/bin/env bash

NEXT_VER=`toml get Cargo.toml package | jq -r '.version' | xargs semver-cli --increment {}`
CARGO=`toml set Cargo.toml package.version $NEXT_VER`

echo "$CARGO" > Cargo.toml
