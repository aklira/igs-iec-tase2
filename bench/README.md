# Interop bench

Black-box tests of `igs-iec-tase2` (and its MMS stack) against reference peers, with the captures decoded by `tshark`.

## Reference peer

rusty-iccp 0.9.2 (MIT OR Apache-2.0), built in `peer/` with its MMS service crate, and a small harness
(`peer/harness`) that calls the public API. The peer's source is never read: the crates are compiled in the container,
and the harness uses only their public signatures (`PROVENANCE.md`, D-010).

The scenario of task F4 (`tests/test_f4_peer_association.py`): the peer associates with the device, and identifies it.

## Device under test

`igsmms-server` of the sibling repository `igs-iso9506-mms`, serving `device/demo.toml`. It is built in the bookworm
image so that it runs in the bench's Debian container:

```bash
cd bench
sh device/build.sh      # writes device/bin/igsmms-server; the sibling repository is read, not changed
```

## Needs

- Podman with `podman-compose` (or Docker with the compose plugin, set `BENCH_COMPOSE`).
- Python 3.10 or later, with `pytest`: `python3 -m venv .venv && .venv/bin/pip install pytest`.
- The sibling repository `igs-iso9506-mms` next to this one (path dependency, rule 9 of `IMPLEMENTATION.md`).

## Run

```bash
cd bench
podman-compose build peer
sh device/build.sh
.venv/bin/pytest
```

Captures stay in `captures/`, which git ignores.

## Layout

| Path | Content |
| --- | --- |
| `docker-compose.yml` | the peer, the device, and the capture that shares the device's network namespace |
| `peer/Dockerfile`, `peer/harness/` | the reference peer: the harness, built with rusty-iccp 0.9.2 |
| `device/build.sh`, `device/demo.toml` | the device under test: its build and its configuration |
| `igs_bench/` | helpers: compose wrapper, capture, tshark decoding (copied from `igs-iso9506-mms/bench`) |
| `tests/` | the scenarios, one file per task |
