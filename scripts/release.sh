#!/usr/bin/env bash

TARGET=${1:-patch}

CURRENT_VER=`toml get Cargo.toml package | jq -r '.version'`
NEXT_VER=`semver-cli $CURRENT_VER --increment $TARGET`
CARGO=`toml set Cargo.toml package.version $NEXT_VER`

echo "$CARGO" > Cargo.toml

git add .
git commit -m ":bookmark: release $NEXT_VER"
git tag -a v$NEXT_VER -m "release $NEXT_VER"

git-chglog --output CHANGELOG.md
git add .
git commit -m ":books: update CHANGELOG.md"
