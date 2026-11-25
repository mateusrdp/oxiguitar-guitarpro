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

# pack LIBRARY OUT [MANIFEST]: the plugin file OUT (mateusrdp/oxiguitar#347),
# an uncompressed tar archive holding MANIFEST (default manifest.json) as
# manifest.json first, the library second as plugin.so, plugin.dll or
# plugin.dylib, then the ui and assets folders beside the manifest, if
# there are any. COPYFILE_DISABLE keeps macOS's tar from adding ._ files.
pack() {
    local ext="${1##*.}" manifest="${3:-manifest.json}" stage folder
    local more=()
    stage="$(mktemp -d)"
    cp "$manifest" "$stage/manifest.json"
    cp "$1" "$stage/plugin.$ext"
    for folder in ui assets; do
        if [[ -d "$(dirname "$manifest")/$folder" ]]; then
            cp -R "$(dirname "$manifest")/$folder" "$stage/"
            more+=("$folder")
        fi
    done
    (cd "$stage" && COPYFILE_DISABLE=1 tar --format=ustar -cf - manifest.json "plugin.$ext" ${more[@]+"${more[@]}"}) > "$2"
    rm -rf "$stage"
}

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
            lipo -create -output "target/$(basename "${out%.oxp}").dylib" \
                "target/aarch64-apple-darwin/release/lib$library.dylib" \
                "target/x86_64-apple-darwin/release/lib$library.dylib"
            pack "target/$(basename "${out%.oxp}").dylib" "$out" "plugins/$name/manifest.json"
            ;;
        *windows*)
            arch="${host%%-*}"
            [[ "$arch" == aarch64 ]] && arch=arm64
            out="dist/$name-$version-windows-$arch.oxp"
            build
            pack "target/release/$library.dll" "$out" "plugins/$name/manifest.json"
            ;;
        *)
            out="dist/$name-$version-linux-${host%%-*}.oxp"
            build
            pack "target/release/lib$library.so" "$out" "plugins/$name/manifest.json"
            ;;
    esac
    echo "$out"
done
