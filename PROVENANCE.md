# Provenance

This file links every non-trivial behaviour of `igs-iec-tase2` to its source: a clause of a standard, an
observation test, or third-party code under a permissive licence. The project is a clean-room
implementation: no protocol behaviour without a source recorded here or in a
`// IEC 60870-6-503 §8.2` comment.

The ISO and IEC standards are licensed and are not distributed with this repository. The references
below point to clauses; they do not reproduce the text of the standards.

## Reference standards

| Standard | Edition | Use |
| --- | --- | --- |
| IEC 60870-6-503 | 2014 | Services and mapping to MMS |
| IEC 60870-6-802 | 2014 | Object models and types |
| IEC 60870-6-702 | 2014 | Application profile, conformance |
| IEC TR 60870-6-505 | 2002 | User guide (secondary source) |

## Decisions on ambiguities in the standard

Confidence: **high** when the sources agree; **medium** when they contradict each other or are
silent, to be confirmed by an observation test against other implementations.

### D-001 — A name defined in several clauses is scoped by clause

- **Decision**: `Number` is defined with a width of 16 bits in IEC 60870-6-802 §6.3 and 32 bits in
  §6.4. Each definition applies within its clause.
- **Sources**: IEC 60870-6-802 §6.3, §6.4.
- **Confidence**: medium, to be confirmed by reading the use of each type.
- **Test**: `crates/igs-iec-tase2-model/tests/generated.rs`, `a_name_defined_in_several_clauses_keeps_each_definition_in_its_clause`
  (the two widths of `Number`, 16 and 32 bits).

### D-002 — Spelling of type names

- **Decision**: the names used by the code follow the data files: `COVCounter` (the definition of
  802 §6.2 is spelled `COV_Counter`), `OutageAmountTypeId`, `ReferenceNum`, `InfoBufXX`.
- **Sources**: count of each spelling in the PDF of 802 and 503.
- **Confidence**: medium, deduced from the names.
- **Test**: `crates/igs-iec-tase2-model/tests/generated.rs`, `the_spelling_of_the_data_file_is_kept_in_the_type_names`.

### D-003 — The current text of 503 prevails over its annex G

- **Decision**: where `SupportedFeatures` and `TSConditions` differ, the current text is used;
  annex G describes the 2002 edition and is informative.
- **Sources**: IEC 60870-6-503 §8.2, annex G.
- **Confidence**: high.
- **Test**: to be written (task M4).

## Third-party code and sources

| Source | Licence | Use | Provenance |
| --- | --- | --- | --- |
| rusty-iccp 0.9.2 (and rusty-mms-service 0.9.2) | MIT OR Apache-2.0, read on crates.io (`license` of the version) | Reference peer of the bench, as a black box: built in a container, its public API used by the harness `bench/peer/harness` (D-010) | Source never read: the crates.io metadata, the public README of the repository, and the signatures of the public API on docs.rs only |

### D-004 — A visible string width is a maximum

- **Decision**: the width of a visible string (`visiblestring` in the data files, with or without `varying`) is the
  greatest number of characters. The generated `new` refuses a longer string; a shorter one is accepted, also when the
  data file gives no variable length.
- **Why**: the data files give one width per string, and do not say in every case whether the length is fixed. The
  maximum is the constraint that every reading satisfies.
- **Sources**: IEC 60870-6-802 §6.2, §6.3 (the definitions of the strings), and the data files.
- **Confidence**: medium. A fixed-length reading would refuse shorter strings; to be confirmed at M1 against the
  encoding rules of the MMS string types.
- **Test**: to be written with the conversions (task M1); the generated `MAX_LEN` constants are in the model.

### D-005 — A plain INTEGER is a 32-bit integer

- **Decision**: the plain `integer` that the data files name as a type (for example `NumPeriods` of the Forecast type,
  802 annex B) is the 32-bit integer of the base types. Every integer of the files has that width when it has one.
- **Why**: the data files name the type `integer` without a width; the width of the integers of the standard is 32
  bits, and the decoder of the stack reads 32-bit integers in the same places.
- **Sources**: the data files (the widths of the integer definitions); IEC 60870-6-802 annex B.3.
- **Confidence**: medium: a 16-bit reading would also fit the data.
- **Test**: `crates/igs-iec-tase2-model/tests/generated.rs`, `a_plain_integer_of_the_data_file_is_a_32_bit_integer`.

### D-006 — References across the two files, and a name defined several times

- **Decision**: a reference is resolved in the names of both data files (802 and 503), since the two standards share
  their types (for example `TimeIntervalS` of 503 is used by 802 annex B). When a name is defined several times, a
  reference takes the definition of its own clause; otherwise the definition of the main text, when there is exactly
  one (an annex is not preferred to the main text: decision D-003 for annex G of 503); otherwise the first of several
  definitions with the same structure; otherwise the generation fails, and the case is decided and recorded here.
- **Why**: the rule resolves every reference of the two files without a choice by hand; the generation refuses any
  case it cannot resolve, so that no reference is guessed.
- **Sources**: the data files; IEC 60870-6-503 §8.2, annex G (D-003).
- **Confidence**: high for the cross-file resolution (every reference resolves); medium for the choice of the first of
  several identical definitions, which does not change the type.
- **Test**: `crates/igs-iec-tase2-model/tests/generated.rs`, the counts (123 and 111, 49 and 40), the components (285 and
  53) and the bits (68 and 45), counted independently from the YAML.

### D-007 — The base types, written by hand

- **Decision**: the base types that the data files name without defining them are written by hand in
  `crates/igs-iec-tase2-model/src/builtin.rs`: `MMS Identifier` (at most 32 characters), `Integer16`, `Integer32`
  and `Unsigned8` (two's complement and unsigned ranges of their widths).
- **Why**: the generator must not invent the base types; they are the same in every use, and few.
- **Sources**: IEC 60870-6-802 §6 (the base types as used), the identifier of 32 characters of ISO 9506-2 §7.5.2
  (the same rule as the MMS plan, G9 of `igs-iso9506-mms`).
- **Confidence**: high.
- **Test**: `crates/igs-iec-tase2-model/tests/builtin.rs` (the limits of each type).

### D-008 — The generator: the data files, the formatting, the check

- **Decision**: `cargo xtask codegen` reads the two data files with `serde-saphyr` (MIT or Apache-2.0, the parser of
  `igs-iso9506-mms`'s xtask), writes the generated files, and formats them with `rustfmt`. `--check` regenerates in
  memory and fails if a committed file differs: the staleness is the difference of contents, not a hash.
- **Why**: the committed files are the reviewed code; their difference is the only check the reviewer needs.
- **Confidence**: high (`--check` was tested on a stale file: it fails, and passes once restored).
- **Test**: the generated files are checked by `cargo xtask codegen --check` in the gate.

### D-009 — The scope of 1.0 (task F3)

- **Decision**: 1.0 implements the block **Basic Services** of IEC 60870-6-702 (mandatory, base `m`), with the MMS
  services, PDUs and parameter CBBs that the conformance tables mark as required for it:
  - services (9): GetNameList, Identify, Read, Write, InformationReport (unconfirmed), GetVariableAccessAttributes,
    DefineNamedVariableList, GetNamedVariableListAttributes, DeleteNamedVariableList;
  - PDUs (7): Initiate request, response and error, Conclude request, response and error, Reject;
  - parameter CBBs (4): `str1`, `str2`, `vnam`, `vlis` (base `m`).
  These are the services and CBBs that the MMS stack of `igs-iso9506-mms` already serves; no MMS feature is added here.
- **Criterion 2 is open**: the other blocks (Extended Conditions, Blocked Transfers, Information Message, SBO Device
  Control: base `o`; Programs, Events, Accounts, Time Series: base `i`) are added to 1.0 only if the reference peer
  implements them, observed as a black box at task F4 (rule 1: no reading of its source). F4 records the result as a
  new decision, and section 6 follows it. Until then the blocks other than Basic Services are later.
- **Why**: the criteria of the plan (F3) are applied to the conformance data of 702, which is exact. The second one
  needs the reference peer, which F4 sets up.
- **Sources**: IEC 60870-6-702 §6 and annex A (the conformance tables), read through the data file
  `conformance_702.yaml` produced by `extract_conformance_702.py`, whose disagreements with the printed text are decided
  in `conformance_702_decisions.yaml` (the typographic errors of the table A.28 and A.29, which do not change the
  scope of the blocks).
- **Confidence**: high for the mandatory set (the data give the scope of each service, PDU and CBB directly); medium
  for the choice of the other blocks, which waits for F4.
- **Test**: to be written with the blocks themselves (task B5 and the tests of the services in the link and client
  tasks); the scope itself is checked by `docs/capabilities.md` in X1.

### D-010 — The reference peer: rusty-iccp 0.9.2, used as a black box (task F4)

- **Decision**: the reference peer is rusty-iccp 0.9.2 (MIT OR Apache-2.0), with its MMS service crate
  rusty-mms-service 0.9.2 (the same licence). The bench builds a harness (`bench/peer/harness`) that calls the public API
  of the MMS service crate: `create_mms_service_client`, then `identify`. The harness is the reference peer's initiator.
  The ICCP client of rusty-iccp is not yet exercised: its API is not documented (docs.rs: 0 % of the crate documented),
  and the association is the first scenario, as the plan requires.
- **Why this peer**: it is the peer named in the plan (F4), it is public and permissively licensed, and it builds from
  crates.io. No other peer is available (the plan allows a second one, given by a maintainer).
- **How it was read**: the crates.io metadata (licence, versions), the README of the repository (the status of each
  layer), and the signatures of the public API on docs.rs. The source of the crates was never opened or extracted. The
  harness uses only those signatures.
- **Status of the layers** (from the README of the repository, as its authors state it): TPKT and COTP complete; the
  session, presentation, ACSE and MMS layers pre-release; ICCP/TASE.2 in progress. The scenario therefore tests the
  association and one confirmed service, not TASE.2.
- **Confidence**: high for the licence and the version (crates.io); medium for the description of the layers, which is
  the authors' own statement.
- **Test**: `bench/tests/test_f4_peer_association.py`: the peer associates with `igsmms-server` and identifies it; the
  capture decodes with no error in tshark.

