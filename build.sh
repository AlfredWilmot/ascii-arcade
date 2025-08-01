#!/usr/bin/env bash

PKG="$(sed -n '/\[package\]/,/^$/{s/^name = "\(.*\)"$/\1/p}' Cargo.toml)"
VER="$(sed -n '/\[package\]/,/^$/{s/^version = "\(.*\)"$/\1/p}' Cargo.toml)"

set -x

# forward any build args to docker
docker build -t "${PKG}:${VER}" --build-arg PROJECT="${PKG}" "${@}" .
