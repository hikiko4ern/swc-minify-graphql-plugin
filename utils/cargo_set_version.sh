#!/usr/bin/env bash

set -xeo pipefail

if [ -z "$npm_package_version" ]; then
	echo "npm_package_version env is not set"
	exit 1
fi

sed -i -e "0,/version/{s/\(version\s*=\s*\"\)[^\"]*\(\"\)/\1${npm_package_version}\2/}" Cargo.toml
cargo tree -i swc-plugin-minify-graphql # this will update crate's version in Cargo.lock
