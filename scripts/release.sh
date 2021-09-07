#!/usr/bin/env bash

TARGET=${1:-patch}

CURRENT_VER=`toml get Cargo.toml package | jq -r '.version'`
NEXT_VER=`semver-cli $CURRENT_VER --increment $TARGET`
CARGO=`toml set Cargo.toml package.version $NEXT_VER`

echo "$CARGO" > Cargo.toml

cargo c
git-cliff --output CHANGELOG.md --tag v$NEXT_VER

git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m ":bookmark: release v$NEXT_VER"
git tag -a v$NEXT_VER -m "release v$NEXT_VER"

cargo package
cargo publish
