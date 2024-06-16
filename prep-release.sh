#!/usr/bin/env sh
set -ex

VERSION=$1

if [ -z "$VERSION" ]
then
  echo "USAGE: ./prep-release.sh <version>"
  exit 1
fi

cargo set-version -p rusql-rt "$VERSION"
cargo set-version -p rusql-core "$VERSION"
cargo set-version -p rusql-macros "$VERSION"
cargo set-version -p rusql "$VERSION"
cargo set-version -p rusql-cli "$VERSION"