# Load test (task S3)

The load run drives `igstase2-server` with many associations at once, on a table of 10 000 points, and measures the
latency of each operation as the client sees it. It is the `igstase2-load` binary (`crates/igstase2-load/`), run against the
bench's device of 10 000 points (service `device-load`) and, for development, in process (`--local`).

## What is measured

- **Workload**: `--clients 8` associations run at once, each with the table of the bench (AP-title `1.3.9999.1`, table
  `BT-2026-001`) over the 10 000 points `ICC1/P00000` … `ICC1/P09999`. Each association reads every point in chunks of 100 and
  writes 10 points. `--rounds` repeats the reads.
- **Counters**: `errors` (an operation that failed, or an item that is not a success) and `stray_reports` (a report that reaches
  an association that did not start a transfer set). The driver exits 1 when either is not 0.
- **Latencies**: per operation (`associate`, `read`, `write`), the p50, p95, p99 and maximum, in milliseconds, measured by the
  client around each request.
- **Report latency**: measured only in the local run (`--local`), where the driver starts a transfer set and times the report
  of a change. The bench run starts no transfer set, so it measures no report latency (see the gaps).

## Configuration of the device

`bench/device/gen_load_config.py [POINTS]` writes `bench/device/load-10k.toml` (git ignores it: the file is 2.3 MB and
generated). It holds the points, the data set `DS1` (its first point), the transfer set `TS1`, and the Bilateral Table with
the access of each point. The same server is built in process by `igstase2-load --local`
(`crates/igstase2-load/src/local.rs`), so both runs test the same table.

## Run

```bash
bench/device/build-tase2.sh                       # builds igstase2-server, igstase2-client and igstase2-load (bookworm)
cd bench && python3 -m pytest -v -s tests/test_s3_load.py
```

The test generates the configuration, starts `device-load` and `client-tase2`, runs the driver in the client container
against `172.31.110.12:102`, and checks the counters and the read count (8 clients × 100 chunks = 800 reads). The transcript is
`bench/transcripts/S3-run.txt`.

## Result

Release build (`rust:1.90-bookworm`), podman bench, one run of `--points 10000 --clients 8 --rounds 1 --writes 10`:

| Operation | n | p50 | p95 | p99 | max |
|-----------|--:|----:|----:|----:|----:|
| associate | 8 | 8.7 ms | 12.8 ms | 12.8 ms | 12.8 ms |
| read (100 points per request) | 800 | 8.8 ms | 9.7 ms | 11.4 ms | 12.6 ms |
| write | 80 | 8.4 ms | 8.8 ms | 9.8 ms | 9.8 ms |

Outcome: `errors=0 stray_reports=0`; 1.10 s elapsed for the whole run; exit 0. The bench test passes, and a second test checks
that the driver exits with a failure when no server answers.

These figures come from one run on one machine (WSL2, podman bridge network). They show the order of magnitude of the
operations, not a benchmark: the run is short, and a single sample gives no variance. Repeat it before comparing two versions.

## Gaps (rule 10): recorded, not tested

- **Report latency and report isolation over the bench.** The report latency is measured only in process. Over the bench no
  transfer set is started, so `stray_reports=0` shows that the server sends no report to a silent association, and nothing
  more. The isolation itself is tested in the server's own tests, with two associations at once, one of which holds the
  transfer set (`a_report_reaches_only_the_association_that_started_its_transfer_set`, `crates/igs-iec-tase2/tests/server.rs`).
  Reports to several associations that all hold a transfer set are not tested under load.
- **Interop with a third-party TASE.2 peer.** None is available: the reference peer of the bench does not serve TASE.2 (D-010).
  The load run therefore checks our server against our client, not against a foreign implementation. This is the same gap as in
  the client matrix (`docs/client-matrix.md`).
- **Sustained load.** One run of a few seconds. No soak test, and no measurement of memory over time.
- **Critical acknowledgement, blocked reports, dual use.** Not exercised by the load run (D-046, D-054).
- **Latency under a slow network.** Not simulated: the bench runs on a local bridge network.
- **Debug build.** The development runs used the debug build, which is slower (associate 18 ms, read p50 12 ms with 2 clients).
  Those figures are not reported.

## Decisions

The choices taken for this run are recorded in `PROVENANCE.md` as D-058.
