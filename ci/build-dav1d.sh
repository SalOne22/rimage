#!/usr/bin/env bash
# Builds a static dav1d and installs it into a prefix, for targets that have
# no system dav1d package (musl and cross-compiled GNU targets).
#
# Usage: build-dav1d.sh <install-prefix> [meson-cross-file]
#
# The following environment variables select the toolchain when cross file is
# omitted and the target differs from the host: CC, AR.
set -euo pipefail

prefix=$1
version=1.5.1

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

git clone --depth 1 --branch "$version" \
    https://code.videolan.org/videolan/dav1d.git "$work/dav1d"

# dav1d 1.5.x spells these options enable_tools/enable_tests; the shorter
# tools/build_tests names are rejected as unknown project options
args=(-Ddefault_library=static -Denable_tools=false -Denable_tests=false --prefix="$prefix")
if [[ $# -gt 1 ]]; then
    args+=(--cross-file "$2")
fi

# options must not sit between meson setup's two positional arguments:
# argparse then fails to bind the trailing sourcedir ("unrecognized
# arguments"), so run from inside the source tree and pass only the builddir
(cd "$work/dav1d" && meson setup "$work/build" "${args[@]}")
ninja -C "$work/build" install
