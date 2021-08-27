#!/usr/bin/env bash

NEXT_VER=`toml get Cargo.toml package | jq -r '.version' | xargs semver-cli --increment {}`
CARGO=`toml set Cargo.toml package.version $NEXT_VER`

echo "$CARGO" > Cargo.toml

git-chglog --next-tag $NEXT_VER --output CHANGELOG.md
git add .
git commit -m ":tada: release $NEXT_VER"
git tag -a $NEXT_VER -m "release $NEXT_VER"
