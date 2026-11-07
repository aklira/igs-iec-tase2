# igs-iec-tase2

An industrial-grade IEC 60870-6 ICCP TASE.2 protocol stack for SCADA and telecontrol, written in Rust, on top of the MMS stack
`igs-iso9506-mms` (IEC 60870-6-503 over ISO 9506).

- **Model** (`igs-iec-tase2-model`): the types of IEC 60870-6-802 and 60870-6-503, generated from their data files, with their
  conversions to and from MMS values.
- **Link** (`igs-iec-tase2-link`): sans-I/O bilateral tables, association, data values and data sets, transfer sets, with a
  fake clock in the tests.
- **Client and server** (`igs-iec-tase2`): `TaseClient` and `TaseServer` over `igs-iso9506-mms`.
- **Binaries**: `igstase2-server` (a demonstration server, the bench's device, with a simulated control point and information
  messages), `igstase2-client` (a command-line client: reads and writes, transfer sets, devices, information messages) and
  `igstase2-load` (a load driver: many associations, latencies measured; see [`docs/load-test.md`](docs/load-test.md)).

The standards are licensed and are not distributed with this repository. The references point to their clauses; no text of the
standards is in the code or the documents.

## Architecture

![Architecture of igs-iec-tase2: the crates by layer, and their dependencies](docs/diagram.png)

The diagram groups the crates by layer:

- **Standards model** (`igs-iec-tase2-model`): the types of IEC 60870-6-802 and 60870-6-503, and their conversions to and from MMS values.
- **TASE.2 protocol** (`igs-iec-tase2-link`, `igs-iec-tase2`): the sans-I/O bilateral tables, association, data values and data sets,
  transfer sets, devices and information messages; `TaseClient` and `TaseServer` run them over the MMS stack.
- **Applications** (`igstase2-server`, `igstase2-client`): the demonstration server, with its configuration and simulated changes, and the
  command-line client.
- **Load driver** (`igstase2-load`): many associations at once, with their latencies measured.

The MMS stack is the sibling repository `igs-iso9506-mms`, which carries the MMS services.

## Limits

- **Dual use is not supported.** An association has one role, client or server, as IEC 60870-6-503 §5.3.3.3 allows. The dual-use
  mode is outside 1.0 (decision D-054 in `PROVENANCE.md`).
- The scope of 1.0 is Basic Services (block 1), Extended Conditions (block 2), Information Message (block 4) and SBO Device
  Control (block 5): decisions D-009 and D-059. Block 3 (Blocked Transfers) and blocks 6 to 9 are not implemented.
- The device commands of the server are simulated: an accepted Operate completes at once, and no real device executes it. The
  device actions (timeout, local reset, success, failure) are not sent as MMS event conditions (decision D-066).
- A critical report's acknowledgement is not available to the client, and the server does not receive it (decision D-046).
- The full list of what is and is not served, with the test of each row: [`docs/capabilities.md`](docs/capabilities.md).

## Quick start

You need Rust 1.85 or later, and the sibling repository `igs-iso9506-mms` checked out next to this one (the path dependency).

```bash
cargo test --workspace --all-features       # the whole test suite
cargo run -p igs-iec-tase2 --example loopback   # a server and its client in one process
```

### A demonstration server

```bash
cargo run -p igstase2-server -- --listen 127.0.0.1:10103 --config bench/device/tase2-demo.toml
```

Its file describes points, data sets, transfer sets and the Bilateral Table of a client (AP-title `1.3.9999.1`). Format and
options: [`docs/igstase2-server.md`](docs/igstase2-server.md).

### A command-line client

In another terminal, against the server above:

```bash
cargo run -p igstase2-client -- -s 127.0.0.1:10103 get Flow1 Level Temperature Breaker
cargo run -p igstase2-client -- -s 127.0.0.1:10103 start TS1 DS1 --object-change --reports 3
```

Commands and exit status: [`docs/igstase2-client.md`](docs/igstase2-client.md).

### As a library

Illustrative: `config`, `agreement` and `point` are built as in `crates/igs-iec-tase2/examples/loopback.rs`.

```rust
use igs_iec_tase2::{Agreement, TaseClient};

let (client, mut events) = TaseClient::associate("192.0.2.10:102", config, &agreement).await?;
let values = client.get_data_values(&[point]).await?;
```

The API is documented with rustdoc: `cargo doc --no-deps --open`.

## Bench

The interop bench (`bench/`, podman or docker compose) runs the device and the client of this repository in their containers,
and the client interop matrix (`docs/client-matrix.md`). See [`bench/README.md`](bench/README.md).

## Documents

- [`IMPLEMENTATION.md`](IMPLEMENTATION.md): the plan, its rules and its tasks.
- [`PROVENANCE.md`](PROVENANCE.md): the decisions taken on the standards' ambiguities, with their sources and confidence.
- [`docs/capabilities.md`](docs/capabilities.md): the capabilities of 1.0, the matrix, and the known gaps.
- [`CHANGELOG.md`](CHANGELOG.md): the changes, release by release.
- [`docs/ai-log/`](docs/ai-log/): the log of each task.
