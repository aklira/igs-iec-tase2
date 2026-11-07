# SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
#
# SPDX-License-Identifier: Apache-2.0
#
# The reference peer of the bench (task F4): the harness that uses the public API of rusty-iccp 0.9.2 (MIT OR
# Apache-2.0) as a black box. The crates are compiled here and never read (PROVENANCE.md, D-010). The toolchain is
# newer than the project MSRV: rusty-acse uses let-chains, which need Rust 1.88 or later.

FROM docker.io/library/rust:1.90-bookworm AS build
WORKDIR /src
COPY harness/ ./
RUN cargo build --release --locked

FROM docker.io/library/debian:bookworm-slim
COPY --from=build /src/target/release/ref-peer /usr/local/bin/ref-peer
