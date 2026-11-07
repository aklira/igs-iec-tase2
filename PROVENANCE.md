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
- **Test**: `tests/transfer.rs`, `the_main_text_names_prevail_over_the_annexes`.

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
- **Test**: `crates/igs-iec-tase2-model/tests/conversions.rs` (the maximum length and the visible characters of each string).

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

### D-011 — The milliseconds of a time stamp are the part of a second

- **Decision**: a time stamp with milliseconds (IEC 60870-6-503 §8.2.5, `TimeStampExtended`) is the GMT seconds and a
  `TimeIntervalL16` component whose value is the milliseconds of that second: 0 to 999. The type `TimeIntervalL16`
  itself accepts any 16-bit value; the stamp refuses 1000 and above.
- **Why**: a time stamp names an instant; a value above 999 would name the instant of another second, which the seconds
  component already gives. The data files give the component a 16-bit width only.
- **Sources**: IEC 60870-6-503 §8.2.5 (the component), §8.2.6 (the interval types).
- **Confidence**: medium: the standard does not state the range in the text read; the decision is the reading that keeps
  the seconds and the milliseconds from overlapping. To confirm against an observation of the reference peer (F4).
- **Test**: `tests/conversions.rs`, `a_time_stamp_of_thousand_milliseconds_is_refused`.

### D-012 — The GMT seconds, and their conversion to a system time

- **Decision**: a GMT time is a 32-bit integer of seconds since 1 January 1970 at 00:00:00 GMT (IEC 60870-6-503 §8.2.4).
  A system time is converted to it by rounding down to the millisecond and to the second, so that a time before 1970 has a
  negative second: `-0.5 s` is second `-1` with `500` milliseconds. The range is that of 32 bits: 1901-12-13T20:45:52Z to
  2038-01-19T03:14:07Z; a time outside it is refused, not wrapped.
- **Sources**: IEC 60870-6-503 §8.2.4 (the base date and the unit).
- **Confidence**: high for the base and the unit (the text states them); medium for the rounding of the times before 1970,
  which the text does not discuss.
- **Test**: `tests/conversions.rs`, `a_gmt_time_counts_seconds_from_1970_as_the_standard_says`,
  `the_limits_of_the_gmt_seconds_are_the_dates_of_32_bits`, `a_time_before_1970_is_rounded_down_to_the_millisecond`.

### D-013 — A real is single precision on the wire

- **Decision**: every floating-point type of the data files is 32 bits with an 8-bit exponent (the data files give the
  format of each: all eleven). It is converted to the MMS floating-point value of the codec: the exponent width 8, then the
  four octets of the IEEE single precision number. A value in Rust is an `f64`; the conversion rounds it to single
  precision, so only a single-precision value survives a round trip.
- **Why**: the wire format is the one the data files and the codec describe; a double precision value on the wire would be
  a different type.
- **Sources**: the data files (the format and the exponent width of each floating-point definition); the codec's own
  description of the floating-point form (`igs-iso9506-mms-codec`, `floating.rs`, the form of ISO 9506-2 §14.4.2.2).
- **Confidence**: high.
- **Test**: `tests/conversions.rs`, `a_real_is_the_ieee_single_precision_octets_with_exponent_width_eight`.

### D-014 — The characters of a visible string

- **Decision**: a visible string holds the graphic characters and the space: the characters from space (0x20) to tilde
  (0x7E). A string with another character (a tab, an accented letter) is refused, and so is a string longer than the
  width of its type (D-004).
- **Why**: the MMS visible string is the ASN.1 VisibleString, whose characters are the graphic set of ISO 646 and the
  space; the decision keeps the conversions to that set.
- **Sources**: ISO/IEC 8824-1 (the VisibleString type, as the MMS encoding of visible strings uses it); IEC 60870-6-503
  §8.2.2 (the visible strings of the model).
- **Confidence**: medium: the 503 text does not list the characters; the decision follows the ASN.1 type.
- **Test**: `tests/conversions.rs`, `a_character_that_is_not_visible_is_refused`, `a_string_longer_than_its_type_is_refused`.

### D-015 — Every structure, bit string, enumeration, octet string and array converts to MMS

- **Decision**: the generator emits the conversions to and from MMS data for every generated type that is a structure
  (component by component, in order), a bit string, an enumeration, an octet string or an array (element by element), in
  addition to the scalars of M1. The only exception is `TimeStampExtended`, written by hand (D-011): its milliseconds are
  checked there.
- **Why**: the indication variants of 802 §6.2.1 are structures of scalars, flags and time stamps; converting each one by
  its components keeps the types of the data files as the only source of their layout.
- **Sources**: IEC 60870-6-802 §6.2.1 (the structures of the variants), §6.2.1 with the flags (bit strings).
- **Confidence**: high; the layouts are the data files', and the round trips test each variant.
- **Test**: `crates/igs-iec-tase2-model/tests/indication.rs`, `each_variant_of_a_real_point_round_trips` and the two
  others of the same kind (the sixteen variants).

### D-016 — The length of a flag string in MMS

- **Decision**: a bit string converts to a flag type of `BIT_COUNT` bits when it holds at least that many bits, and no
  bit beyond them is set. The first bit of the string is bit 0 of the type (the most significant bit of the first octet).
  Unused bits at the end of the string are not flags. A string shorter than the type is refused.
- **Why**: a later edition could add bits to a flag string; a bit that the type does not know must not be read as a flag,
  but a longer string with those bits clear is the same value. The refusal of a short string keeps a truncated value from
  being read as a complete one.
- **Sources**: IEC 60870-6-802 §6.2.1 (the bits of each flag string, with their positions).
- **Confidence**: medium: the standard does not say how a longer string is treated; the choice keeps the value and refuses
  what the type cannot hold.
- **Test**: `tests/indication.rs`, `a_bit_string_shorter_than_the_flags_or_with_a_bit_beyond_them_is_refused`,
  `the_flags_of_a_mms_bit_string_map_to_the_bits_in_order`.

### D-017 — The two-bit codes: states, tags and expected states

- **Decision**: a state of a point (`State_hi`, `State_lo`), a tag and an expected state are two-bit codes, kept as the
  values 0 to 3 with the type `TwoBitCode`. Their meaning depends on the device: the table of 802 §8.2 gives, for each device
  kind, what code 00, 01, 10 and 11 mean (for instance a breaker and a disconnector do not share their names). The model
  does not name the codes. The tag codes of the control points (NO-TAG = 0, OPEN-AND-CLOSEINHIBIT = 1, CLOSE-ONLY-INHIBIT
  = 2, 802 §7.1.2) are not assumed for the supplemental tag: the bits are kept as codes.
- **Why**: a name that the text does not give for a code would be an invention; the device tables are the authority of the
  meaning, and they belong to the application that knows the device.
- **Sources**: IEC 60870-6-802 §8.2 (the code table of the states), §7.1.2 (the tags of the control points), §5.2.2 (the
  expected state and its use).
- **Confidence**: high for the bit positions; medium for the supplemental tag, which the standard does not map to the
  control point codes in these words.
- **Test**: `tests/indication.rs`, `the_state_of_a_point_is_bits_zero_and_one_and_keeps_its_quality`,
  `a_supplemental_state_has_its_state_its_tag_and_its_expected_state`.

### D-018 — The quality flags and their bit positions

- **Decision**: validity on bits 2 and 3 (VALID 0, HELD 1, SUSPECT 2, NOTVALID 3); current source on bits 4 and 5
  (TELEMETERED 0, CALCULATED 1, ENTERED 2, ESTIMATED 3); normal value on bit 6 (NORMAL 0, ABNORMAL 1); time stamp quality on
  bit 7 (VALID 0, INVALID 1). The same positions in `Data_Flags` and `Data_State`. The accessors write only their bits.
- **Why**: the positions and the values are those of the mapping of 802 §7.1.1, which the data files confirm (the flag
  strings of §6.2.1 name the same positions).
- **Sources**: IEC 60870-6-802 §5.2.2 (the attributes and their values), §6.2.1 (the flag strings), §7.1.1 (the mapping).
- **Confidence**: high.
- **Test**: `tests/indication.rs`, the validity, current source, normal value and time stamp quality tests, and the
  64 combinations.

### D-019 — The ranges of the data narrow the integers

- **Decision**: an integer of the data that has a `constraint: range least .. greatest` is generated with those bounds
  (`MIN` and `MAX` of the newtype, and the refusal of a value outside them), not with the bounds of its width. The
  width still gives the bits of the MMS value. Nine items of the 802 data carry a range (among them the point type,
  the quality class, the normal source, the time stamp class, the COV class, the control point type and the set point
  type); the others keep the bounds of their width.
- **Why**: a control point type of 0 to 2 that accepted 127 would let a malformed command through; the standard
  constrains these values, and the data file records the constraint next to the type. The generator reads the
  constraint, so no value is typed by hand.
- **Sources**: the `constraint: range` entries of the 6-802 data file (`docs/spec/6-802/data/mms_types.yaml`, outside this
  repository); the generator in `xtask/src/codegen.rs` (`parse_range`, `render_integer`).
- **Confidence**: high for the bounds (they are the data's); medium for the reading that the constraint applies to the
  whole type, not to one use: the data gives it on the type, and the clauses that use the type were not re-read here.
- **Test**: `tests/control.rs`, `a_control_point_type_outside_its_range_is_refused_by_the_type_and_by_mms` and
  `a_control_configuration_round_trips_within_its_ranges`; `tests/generated.rs` checks the counts of the generated items.

### D-020 — An enumeration whose base is Boolean is a Boolean

- **Decision**: an enumeration whose base type is `Boolean` is the MMS Boolean. Its names stand for the values: `TRUE`
  (non-zero) and `FALSE` (0) for the block data, the critical, RBE and all-changes-reported flags; `ENABLED` (1) and
  `DISABLED` (0) for the status. The generated type is a `bool` newtype with the names as constants.
- **Why**: the generator had written these as integers (only 0 and 1 accepted), so a Boolean value from the peer was
  refused. The text gives the base as Boolean, and the values as the MMS Boolean.
- **Sources**: IEC 60870-6-503 §8.2.7.1 (DSTransferSet) and §8.2.7.2 (TSTransferSet, TATransferSet): the component types
  `Boolean {TRUE (non-zero), FALSE (0)}` and `Boolean {ENABLED(1), DISABLED(0)}`.
- **Confidence**: high.
- **Test**: `tests/transfer.rs`, `a_boolean_enumeration_is_a_boolean_with_the_names_of_the_standard` and
  `a_status_is_enabled_or_disabled`.

### D-021 — The reserved and unused bits of a bit string have no name, and Block1 is not enforced

- **Decision**: a bit named `reserved` or `unused` has no constant and no accessor: its bit is kept as received, so every
  value of the string round-trips. (The protection flags of M5 and the `Data_Flags` of M2 have such bits.) `SupportedFeatures` has eight reserved bits (2, 5 to 9): the generator had kept one of them
  under the name `RESERVED`, which named only bit 2. `Block1` is always 1 in the text; the type does not refuse a
  value with it cleared, since the peer's value is the one to report (the link decides what to do with it).
- **Why**: a constant for one of several bits called `reserved` suggests a single meaning; the reserved bits have none.
  Refusing a cleared `Block1` would decide a protocol behaviour in the type model.
- **Sources**: IEC 60870-6-503 §8.2.9 (the bit positions of SupportedFeatures, and the note on Block1).
- **Confidence**: high for the names and positions; medium for the choice not to refuse, to be confirmed by the
  interop test of the link (B-tasks).
- **Test**: `tests/transfer.rs`, `the_supported_features_keep_every_bit_of_their_twelve`.

### D-022 — The main text is exported at the root of its file

- **Decision**: a name defined in several clauses, with exactly one definition in the main text (not an annex), is also
  exported at the root of its file: `iso503::TSTransferSet`, `TATransferSet`, `IMTransferSet`, `TSConditions` and
  `SupportedFeatures` are the definitions of the main text. Names defined in several main clauses
  (D-001), keep no root name; a name defined once, in the main text or in an annex, is at the root as before. The
  generator exports only definitions that produce items: a reference to a type of the other file produces none.
- **Why**: D-003 says the main text prevails; a name at the root reads as the current definition, and the annexes stay
  reachable by their clause.
- **Sources**: D-003; IEC 60870-6-503 §8.2 (main text) and annex G (earlier edition).
- **Confidence**: high.
- **Test**: `tests/transfer.rs`, `the_main_text_names_prevail_over_the_annexes`.

### D-023 — The scope of M5: the protection equipment types of Basic Services

- **Decision**: M5 covers the protection equipment types of 6-802 §6.2.3 (`SingleFlags`, `EventFlags`, `PackedFlags`,
  `SingleProtectionEvent`, `PackedProtectionEvent`). They are the data value types of the block Basic Services, whose
  table of data values (702 A.4) carries the object `ProtectionEquipmentEvent` with the optional base `o`. The
  object's behaviour in the link is decided at B-tasks.
  The types of 6-802 §6.4 (InformationBuffer) and of annex B (transfer accounts, power plant, matrix, general data)
  belong to blocks that D-009 leaves out of 1.0 (Information Message, Accounts, Programs, Time Series). They stay
  generated, and their round trip is tested when their block is chosen.
- **Why**: the scope of 1.0 is D-009; the protection types are the only 6-802 types of the data-value part of Basic
  Services that M1 to M4 do not cover.
- **Sources**: IEC 60870-6-702 A.3 (Table A.4, whose scope footnote names block 1), IEC 60870-6-802 §6.2.3, §6.4, annex B.
- **Confidence**: medium for the reading of Table A.4 as the data values of block 1 (the footnote names block 1 for
  the scope of the data values); high for the exclusion of the other blocks (D-009).
- **Test**: `tests/protection.rs`, the bit positions of the three flag types, the round trips of the two events, and the
  refusals of a malformed bit string and structure.

### D-024 — The protection flags and the event state, as the mapping of 802 §7.1.3 gives them

- **Decision**: the event state is the two-bit code of bits 6 and 7 (`EventState_hi`, `EventState_lo`) of
  `SingleFlags`, with the three named values of the mapping: OFF for code 1, ON for code 2, INVALID for code 0 or 3
  (written as 0). The first five flags of `SingleFlags` and `PackedFlags` have accessors named by their condition, and a
  set bit is that condition: `elapsed_time_invalid`, `blocked`, `substituted`, `event_invalid`. The exception is
  `Topical`, whose bit is inverted in the mapping (a set bit is NOTTOPICAL): the accessor is `not_topical`.
  `EventFlags` has one accessor per bit (`general`, `phase1` to `phase3`, `earth`, `reverse`), each meaning the start
  or the trip of its phase by the attribute: the mapping maps both `StartX` and `TripX` on the same bit.
- **Why**: the mapping gives the named values of the event state, which replace the device-dependent codes of the
  state (D-017) for these bits; the inverted flag is named by its condition so that no reader takes its bit for the
  opposite condition.
- **Sources**: IEC 60870-6-802 §6.2.3 (the bit tables), §7.1.3 (the attributes ElapsedTimeValidity, Blocked,
  Substituted, Topical, EventValidity, EventState, StartX, TripX).
- **Confidence**: high for the bit positions and the event state; medium for the mapping of the packed flags' start and
  trip, which the bits share.
- **Test**: `tests/protection.rs`, `an_event_state_is_off_on_or_invalid_as_the_mapping_says`,
  `the_state_is_written_and_the_other_bits_are_kept`, `an_inverted_attribute_is_named_by_its_condition`.
