#!/bin/sh
# SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
#
# SPDX-License-Identifier: Apache-2.0
#
# Builds the device under test (igsmms-server, from the sibling repository igs-iso9506-mms) as a glibc binary in the
# bookworm image of the bench, so that it runs in the bench's Debian container. The binary is written to
# bench/device/bin/, which git ignores. The sibling repository is read, not changed.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
mms=${MMS_REPO:-$here/../../../igs-iso9506-mms}
podman run --rm -v "$mms:/src" -w /src docker.io/library/rust:1.85-bookworm \
    cargo build --release -p igs-iso9506-mms --bin igsmms-server --target-dir /src/target/bench-bookworm
mkdir -p "$here/bin"
cp "$mms/target/bench-bookworm/release/igsmms-server" "$here/bin/igsmms-server"
echo "built $here/bin/igsmms-server"
