#!/usr/bin/env bash

set -xeo pipefail

cur_version="$(jq -r '.version' package.json)"
new_version_tag="$(pnpm git-cliff --bumped-version)"
new_version="${new_version_tag#v}"

if [ "$cur_version" = "$new_version" ]; then
	echo "version ${cur_version} is already actual"
	exit 127
fi

echo "bumping from ${cur_version} to ${new_version}"

pnpm version --sign-git-tag -m "chore(release): ${new_version_tag}" "$new_version"
