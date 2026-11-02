#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors
#
# Builds the five Guitar Pro plugins' release files for the system this runs
# on:
#
#   dist/gp3-VERSION-SYSTEM-ARCH.oxp, gp4-..., gp5-..., gpx-..., gp-...
#
# such as dist/gp5-0.1.0-linux-x86_64.oxp, named the way OxiGuitar's plugins
# are (docs/plugins/template in github.com/mateusrdp/oxiguitar). On macOS it
# builds both processors and joins them into one universal file (needs both
# Rust targets: rustup target add aarch64-apple-darwin x86_64-apple-darwin).
# On Windows, run it from Git Bash. Each plugin is built on its own: built
# together, Cargo would give every plugin the readers of all of them (each
# plugin carries only its own code). .github/workflows/release.yml runs it
# on each system.

set -euo pipefail

# The plugin files' names; the package is oxiguitar-NAME-plugin and the
# library oxiguitar_NAME_plugin.
NAMES=(gp3 gp4 gp5 gpx gp)

cd "$(dirname "$0")"
version="$(cargo pkgid -p oxiguitar-gp3-plugin)"
version="${version##*[#@]}"
host="$(rustc -vV | sed -n 's/^host: //p')"
rm -rf dist
mkdir -p dist

for name in "${NAMES[@]}"; do
    package="oxiguitar-$name-plugin"
    library="oxiguitar_${name}_plugin"
    build() {
        cargo rustc --locked --release -p "$package" --features oxp --crate-type cdylib "$@"
    }
    case "$host" in
        *apple*)
            out="dist/$name-$version-macos-universal.oxp"
            for target in aarch64-apple-darwin x86_64-apple-darwin; do
                build --target "$target"
            done
            lipo -create -output "$out" \
                "target/aarch64-apple-darwin/release/lib$library.dylib" \
                "target/x86_64-apple-darwin/release/lib$library.dylib"
            ;;
        *windows*)
            arch="${host%%-*}"
            [[ "$arch" == aarch64 ]] && arch=arm64
            out="dist/$name-$version-windows-$arch.oxp"
            build
            cp "target/release/$library.dll" "$out"
            ;;
        *)
            out="dist/$name-$version-linux-${host%%-*}.oxp"
            build
            cp "target/release/lib$library.so" "$out"
            ;;
    esac
    echo "$out"
done
