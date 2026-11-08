#!/bin/sh
# SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
#
# SPDX-License-Identifier: Apache-2.0
#
# Builds the third-party peer of the interop bench (task I1). The peer is a licensed library that is not in this repository:
# THIRD_PARTY_PEER_ARCHIVE gives the path of its source archive, and THIRD_PARTY_PEER_BIN the directory that receives its
# example programs. The archive is extracted in THIRD_PARTY_PEER_WORK (outside this repository, by default in the user's cache),
# and only the built example programs are copied out. Nothing of the peer's source is read or kept by this repository; the peer
# runs as a separate process, as a black box. THIRD_PARTY_PEER_PATCH, optional, is a patch that makes the archive compile; it is kept
# outside this repository, and applied to the extracted sources in the container (see PROVENANCE.md, D-068).
set -eu
: "${THIRD_PARTY_PEER_ARCHIVE:?set THIRD_PARTY_PEER_ARCHIVE to the archive of the peer}"
: "${THIRD_PARTY_PEER_BIN:?set THIRD_PARTY_PEER_BIN to the directory that receives the example programs}"
work="${THIRD_PARTY_PEER_WORK:-$HOME/.cache/igs-bench-third-party-peer}"
rm -rf "$work"
mkdir -p "$work" "$THIRD_PARTY_PEER_BIN"
tar -xzf "$THIRD_PARTY_PEER_ARCHIVE" -C "$work"
source_dir=$(find "$work" -mindepth 1 -maxdepth 1 -type d | head -n 1)
patch_file="${THIRD_PARTY_PEER_PATCH:-/dev/null}"
podman run --rm \
    -v "$work:/work" \
    -v "$THIRD_PARTY_PEER_BIN:/out" \
    -v "$patch_file:/patch:ro" \
    docker.io/library/debian:bookworm-slim \
    sh -eu -c '
        apt-get update -qq >/dev/null
        apt-get install -y -qq cmake gcc g++ make patch >/dev/null
        if [ -s /patch ]; then patch -p1 -d "/work/'"$(basename "$source_dir")"'" < /patch; fi
        cmake -S "/work/'"$(basename "$source_dir")"'" -B /work/build \
            -DBUILD_EXAMPLES=ON -DBUILD_TESTS=OFF -DUSE_CCACHE=OFF -DDEBUG_SYMBOLS=OFF >/dev/null
        cmake --build /work/build -j"$(nproc)"
        find /work/build/examples -type f -perm -u+x ! -name "*.sh" -exec cp {} /out/ \;
        ls -l /out
    '
echo "built the example programs in $THIRD_PARTY_PEER_BIN"
