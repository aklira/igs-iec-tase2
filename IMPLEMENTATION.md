# Implementation plan

This plan drives the implementation of `igs-iec-tase2` up to a ready-to-use 1.0 release. It is
written for an AI coding agent working task by task. It follows the method of `igs-iec104`, with
one change: **the agent decides and records; it does not stop to ask** (rule 10).

Scope of 1.0: TASE.2 (ICCP) client and server on `igs-iso9506-mms`: bilateral tables, the data
types of IEC 60870-6-802, data sets, transfer sets, and the conformance blocks chosen in task F3.
The MMS stack itself lives in `igs-iso9506-mms`.

---

## 1. Rules for the agent (read before every task)

1. **Clean room.** Never open, quote, reconstruct or describe the source code of rusty-iccp or any
   other TASE.2, ICCP or MMS implementation. Reference implementations are only ever run as
   separate processes in the interop bench.
2. **No standard text in this repository.** IEC standards are licensed. Never paste text from
   `docs/spec/` into code, comments, docs, tests or commit messages. Never commit a file of
   `docs/spec/`, `mms_types.yaml` included. Cite the clause (`// IEC 60870-6-503 §8.2`) and write in
   your own words. Facts such as type names, widths and bit positions may be used as data.
3. **English only** for everything committed: code, comments, docs, commit messages.
4. **Every protocol behaviour needs a source.** Take values from the data files of section 3, not
   from memory. If the standard is silent or ambiguous, choose the reading best supported by the
   clauses, record it as a `D-nnn` entry of `PROVENANCE.md` with its confidence and a test, and go
   on. Do not wait for an answer.
5. **One task at a time.** Read only the inputs listed for the task. Keep each change small (aim for
   under 400 changed lines per commit). Do not start a task whose dependencies are not checked.
6. **Done means green.** A task is done only when all its done criteria pass, plus the standard gate:
   ```
   cargo fmt --all --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace --all-features
   cargo deny check
   ```
7. **Commit per task, never push.** Nothing leaves this machine before task R1. Message:
   `<crate or area>: <what>` then a blank line and `Task: <ID>`. Tick the task's box in section 6
   in the same commit.
8. **Safety.** `#![forbid(unsafe_code)]` in `igs-iec-tase2-model` and `igs-iec-tase2-link`. No
   `unwrap`, `expect`, indexing that can panic, or unchecked arithmetic on data from the network:
   return `Result`. Bound every length read from the wire.
9. **Dependencies** must be MIT, Apache-2.0, BSD or ISC and pass `cargo deny`. `igs-iso9506-mms` is
   a path dependency (`../igs-iso9506-mms`) until it is published. A missing MMS feature is added in
   that repository, under its own plan, never worked around here.
10. **Decide and record.** The only human gate is task R1 (⛔). Choices with written criteria (the
    conformance blocks, the reference peer, the exit of a phase) are made by the agent against those
    criteria and recorded in `PROVENANCE.md` and the ai-log. A failing criterion is fixed, or
    recorded as a known gap in `docs/capabilities.md`; it does not stop the plan.
11. **Log prompts.** Archive the prompt and a short summary of each task in
    `docs/ai-log/<task-id>.md` (no standard text).
12. **Reading the licensed text.** The agent may read `docs/spec/` and the PDFs, local or online,
    under the standing authorization recorded in `.claude/CONTEXT.md`. It reads only what the task
    needs (rule 5). Rule 2 still holds: nothing it reads goes into this repository.

---

## 2. Architecture (summary)

Cargo workspace:

| Crate | Role | I/O | async |
| --- | --- | --- | --- |
| `igs-iec-tase2-model` | TASE.2 types of 802 and 503, generated; conversion to and from MMS `Data` | none | no |
| `igs-iec-tase2-link` | Sans-I/O logic: bilateral tables, association checks, data sets, transfer sets | none | no |
| `igs-iec-tase2` | Public API: `Client`, `Server`, binaries `igstase2-client` and `igstase2-server` | through `igs-iso9506-mms` | yes |
| `xtask` | Code generation and maintenance commands (not published) | files | no |

Design rules:

- **Sans-I/O logic**: transfer sets and their conditions take events (data changes, current instant,
  MMS indications) and return actions (reports to send, next wake-up time).
- **Strong types**: one TASE.2 type = one Rust type; quality and state bit fields have named flags.
- **Two-level validation**: structural conversion from MMS `Data`, then a check against the bilateral
  table and the conformance blocks. This is the role that the 104 profile plays in `igs-iec104`.
- **Server objects through the VMD trait** of `igs-iso9506-mms`, without changing that library.

---

## 3. Sources of truth

`docs/spec/` is a local symlink to the private standards repository; it is ignored by git and not
available in CI. Read `docs/spec/<standard>/INDEX.md` first, then only the section you need.

**Data files (use these for any value that becomes code):**

| File | Content |
| --- | --- |
| `docs/spec/6-802/data/mms_types.yaml` | 123 type definitions of 802 (structures, bit strings, scalars) |
| `docs/spec/6-503/data/mms_types.yaml` | 49 types and variables of 503 |
| `docs/spec/6-702/data/conformance_702.yaml` | Conformance blocks, MMS services, stack (task P4 of the MMS plan) |
| `PROVENANCE.md` | Decisions on ambiguities |

**Known defects of the published text** (details in the private `CONTEXT.md` §14): the type
`Number` of 802 is defined twice, with two widths; annex G of 503 (informative, edition 2002) differs
from the current text, which prevails.

**Sections of 503, 802 and the guide:**

| Topic | Sections |
| --- | --- |
| Model of TASE.2 | 6-503 §5 |
| Mapping of the objects to MMS | 6-503 §6 |
| Use of MMS services by operation | 6-503 §7 (the largest section, with 9 missing figures) |
| Standard named types and variables | 6-503 §8 |
| Bilateral table model | 6-503 §10 |
| Conformance | 6-503 §11, 6-702 §6, annex A |
| 2002 edition compatibility | 6-503 annexes F, G |
| Object models, mapping to types | 6-802 §5, §7, §8 |
| Types | 6-802 §6, annex B |
| Conformance blocks of the object models | 6-802 §9 |
| Concepts, blocks (secondary source) | 6-505 §6, §8 |

**Do not use `6-503/sections/` for tables on pages 61–64, 76–78, 88–90, 105–108 and 143–145**: the
model added table headers that do not exist. Use `raw/` or the PDF. Figures are missing from the
prepared text: read them in the PDF. **The PDF is the authority** when text and data disagree.

---

## 4. Phases and tasks

Each task lists: **Depends**, **Read**, **Produce**, **Done when**.

### Phase 0 — Foundations

**F1 — Workspace skeleton**
- Depends: none.
- Produce: root `Cargo.toml`, `rust-toolchain.toml`, the four crates, workspace lints as in
  `igs-iec104`, `deny.toml`, Apache-2.0 SPDX header in each source file, CI and
  `docs/ai-log/README.md` as in the MMS repository.
- Done when: standard gate passes on the empty crates.

**F2 — Code generation**
- Depends: F1.
- Produce: `cargo xtask codegen` reading both `mms_types.yaml` and writing
  `crates/igs-iec-tase2-model/src/generated/`, with a "generated, do not edit" banner citing clauses.
  Where a name is defined in several clauses, the generated type is scoped by clause. Generated
  files are committed; `--check` fails when they are stale.
- Done when: the code compiles and a test asserts 111 + 40 names and the component and bit counts.

**F3 — Scope of 1.0**
- Depends: conformance data (MMS plan, P4).
- Produce: the list of conformance blocks of 1.0. Criteria: every block mandatory in 702; every
  other block that the reference peer implements; the rest later. Record it in `PROVENANCE.md`.
- Done when: the list is recorded and section 6 is adjusted to it.

**F4 — Interop bench skeleton**
- Depends: F1, MMS plan T3.
- Produce: `bench/` as in `igs-iec104`. Reference peer: rusty-iccp built in a container, as a black
  box, its source never read; licence and version recorded in `PROVENANCE.md`. Any other peer made
  available by a maintainer is added as a second one.
- Done when: one scenario associates with the peer and the capture decodes in `tshark`.

### Phase 1 — Model (`igs-iec-tase2-model`)

**M1 — Scalars and time**
- Depends: F2.
- Read: 6-503 §8.2, 6-802 §6.
- Produce: time types, floating point and strings, conversions to and from MMS `Data`.
- Done when: round trips; hand-written vectors; range checks.

**M2 — Indication points**
- Depends: M1.
- Read: 6-802 §5.2, §6.2, §7.1.
- Produce: the state, real and discrete values, with their quality flags, in every variant.
- Done when: each variant round-trips; each flag is named and tested.

**M3 — Control points and device outage**
- Depends: M1.
- Read: 6-802 §5.2, §5.3, §6.3, §7.2, §8.3.
- Done when: each type round-trips.

**M4 — Transfer sets, conditions, data set types**
- Depends: M1.
- Read: 6-503 §6.9 to §6.14, §8.2.
- Produce: `DSTransferSet` and the other transfer set types, conditions, features. A missing brace
  in the published `TSTransferSet` does not change its components.
- Done when: each type round-trips; bit positions checked against the generated tables.

**M5 — Other types of the chosen blocks**
- Depends: F3.
- Read: 6-802 §6.4, annex B.
- Done when: each type round-trips.

**M6 — Property tests and fuzzing**
- Depends: M2 to M5.
- Produce: `proptest` strategies; random `Data` never panics; `fuzz/` target run by the agent for at
  least 8 hours in the background.
- Done when: 10 000 cases per property pass; no crash.

### Phase 2 — Link (`igs-iec-tase2-link`)

**B1 — Bilateral table**
- Depends: M1.
- Read: 6-503 §5, §6.5, §10.
- Produce: bilateral table model; access checks; `Bilateral_Table_ID` and version checks.
- Done when: tests for each access result.

**B2 — Association**
- Depends: B1.
- Read: 6-503 §6, §7.
- Produce: sans-I/O association logic: what is read and checked after MMS Initiate, in both roles.
- Done when: tests for a matching and a mismatching table.

**B3 — Data values and data sets**
- Depends: B2.
- Read: 6-503 §6.6, §6.7, §7.
- Produce: read, write, data set creation, deletion, and their errors.
- Done when: tests per operation.

**B4 — Transfer sets**
- Depends: B3, M4.
- Read: 6-503 §6.9 to §6.14, §7.
- Produce: state machine: start, stop, conditions, interval, integrity, change reports, critical
  acknowledgement, all with a fake clock.
- Done when: a deterministic test per condition and per expiry.

**B5 — Controls and messages of the chosen blocks**
- Depends: B3, F3.
- Read: 6-503 §7, 6-802 §8.
- Produce: device control and information messages, as far as F3 includes them.
- Done when: tests per operation.

### Phase 3 — Client (`igs-iec-tase2`)

**T1 — Client API**
- Depends: B2 to B5, MMS plan T2.
- Produce: `Client` on `igs-iso9506-mms`: association, data sets, transfer sets, an event stream of
  reports, bounded channels with documented back-pressure.
- Done when: integration tests against an in-process server stub.

**T2 — `igstase2-client` binary**
- Depends: T1.
- Done when: `--help` documented; a run against the bench peer recorded.

**T3 — Client interop matrix**
- Depends: T1, F4.
- Done when: bench scenarios per operation pass; gaps recorded (rule 10).

### Phase 4 — Server

**S1 — Server**
- Depends: B2 to B5, MMS plan S2.
- Produce: `Server`: bilateral tables, objects through the VMD trait, transfer sets reporting to
  each association.
- Done when: integration tests per operation with the `Client`.

**S2 — `igstase2-server` binary**
- Depends: S1.
- Produce: demo server with points from a TOML file, simulated changes.
- Done when: documented; usable as the bench device under test.

**S3 — Server interop and load**
- Depends: S2, F4.
- Produce: bench scenarios; a load test with 10 000 points and several associations.
- Done when: the matrix passes; latency measured; gaps recorded (rule 10).

### Phase 5 — Documentation and release

**X1 — Documentation and capabilities**
- Depends: T1, S1.
- Produce: rustdoc on every public item, `examples/`, README quick start, `CHANGELOG.md`,
  `docs/capabilities.md` (blocks, operations, the test covering each row, known gaps).
- Done when: `cargo doc --no-deps` has no warnings; every row names a test that exists.

**R1 — Legal review and release** ⛔
- Depends: all previous tasks, and task R1 of `igs-iso9506-mms`.
- Gate (the only human gate): legal review, similarity check, crates.io names, git push and tag.
- Done when: a maintainer approves and publishes 1.0.0.

---

## 5. Definition of "ready to use" (1.0)

- Client and server interoperate with the reference peer on the bench matrix.
- Every type of the chosen blocks round-trips; no panic on any input.
- Public API documented with examples; binaries usable.
- Every non-trivial behaviour traceable to a clause or an entry of `PROVENANCE.md`.

---

## 6. Progress

| Task | Status |
| --- | --- |
| F1 Workspace skeleton | [x] |
| F2 Code generation | [x] |
| F3 Scope of 1.0 | [x] |
| F4 Interop bench skeleton | [x] |
| M1 Scalars and time | [x] |
| M2 Indication points | [x] |
| M3 Control points and device outage | [x] |
| M4 Transfer sets, conditions, data set types | [x] |
| M5 Other types of the chosen blocks | [x] |
| M6 Property tests and fuzzing | [ ] |
| B1 Bilateral table | [x] |
| B2 Association | [ ] |
| B3 Data values and data sets | [ ] |
| B4 Transfer sets | [ ] |
| B5 Controls and messages of the chosen blocks | [ ] |
| T1 Client API | [ ] |
| T2 `igstase2-client` binary | [ ] |
| T3 Client interop matrix | [ ] |
| S1 Server | [ ] |
| S2 `igstase2-server` binary | [ ] |
| S3 Server interop and load | [ ] |
| X1 Documentation and capabilities | [ ] |
| R1 Legal review and release ⛔ | [ ] |

Suggested order: F1, F2, F3, M1–M6 (in parallel with the MMS plan), B1–B5, F4, T1–T3, S1–S3, X1, R1.
