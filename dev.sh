#!/usr/bin/env bash

PKG="$(sed -n '/\[package\]/,/^$/{s/^name = "\(.*\)"$/\1/p}' Cargo.toml)"
VER="$(sed -n '/\[package\]/,/^$/{s/^version = "\(.*\)"$/\1/p}' Cargo.toml)"

# display final command, forward any args to docker
set -x
./build.sh --target dev
docker run --rm -it -v "$(pwd):/home/build" "${@}" "${PKG}:${VER}"
