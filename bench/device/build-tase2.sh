#!/bin/sh
# SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
#
# SPDX-License-Identifier: Apache-2.0
#
# Builds the TASE.2 device under test (igstase2-server, task S2) as a glibc binary in the bookworm image of the bench, so
# that it runs in the bench's Debian container. The binary is written to bench/device/bin/, which git ignores. The path
# dependencies are read from the sibling repository igs-iso9506-mms, which is not changed.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/../.." && pwd)
parent=$(cd "$repo/.." && pwd)
# The repositories are mounted side by side, as the path dependencies name them: /work/igs-iec-tase2 and /work/igs-iso9506-mms.
podman run --rm -v "$parent:/work" -w /work/igs-iec-tase2 docker.io/library/rust:1.90-bookworm \
    cargo build --release --locked -p igstase2-server --bin igstase2-server --target-dir /work/igs-iec-tase2/target/bench-bookworm
mkdir -p "$here/bin"
cp "$repo/target/bench-bookworm/release/igstase2-server" "$here/bin/igstase2-server"
echo "built $here/bin/igstase2-server"
