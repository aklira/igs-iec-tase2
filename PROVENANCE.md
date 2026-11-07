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

### D-025 — A real value must be finite

- **Decision**: a single-precision value that is NaN or an infinity is refused when it is read from MMS, for every
  floating-point type of the model (the measurements, the real set points, the real quality types). The generator
  refuses it with the format error "a real value must be finite".
- **Why**: a NaN reads back as a value that is not equal to itself, so a value read from a peer could not be written
  and read back the same. A point value or a set point that is not a number has no meaning in TASE.2.
- **Sources**: IEC 60870-6-802 §6.2 (the real types), and the single-precision reading of D-013. The text read here
  does not allow a non-finite value for a data point; this is a reading of the type, to be confirmed by the interop
  tests of the link (B-tasks).
- **Confidence**: medium.
- **Test**: `tests/properties.rs`, `every_value_that_is_read_is_read_back_after_writing` (the failing case found by
  the property, kept in `tests/properties.proptest-regressions`), and `tests/conversions.rs`, `a_real_that_is_not_single_precision_is_refused`.

### D-026 — The property tests and the fuzz target of M6

- **Decision**: the strategies of the valid values are in `src/strategy.rs`, behind the feature `strategy` (off by
  default: the library does not depend on the test framework). The properties are in `tests/properties.rs`, 25 properties
  of 10 000 cases each: random data is read by every type without a panic; an accepted value is read back after
  writing; a valid value round-trips through its MMS data and its BER encoding; data close to a valid value (one
  component replaced, or the last one dropped) is read without a panic. The fuzz target `fuzz/fuzz_targets/read_types.rs`
  reads random bytes as BER data, then as each type, and asserts that an accepted value is read back after writing. It
  is a separate workspace, so the libfuzzer runtime is not a dependency of the library.
- **Known limit**: writing a real rounds it to single precision (`to_mms` casts to `f32`); a value that is not
  representable is rounded, not refused. A value read from MMS is always representable, so the round trip from MMS is
  exact. The strategies do not generate reals that are not single precision.
- **Why**: a property states what holds for every valid value; a generator of random data finds the values that the
  refusals miss (here a NaN). The fuzz target runs the same checks on the bytes a peer could send.
- **Sources**: the design of the tasks (M6 of `IMPLEMENTATION.md`); the codec's strategies of the MMS stack
  (`igs_iso9506_mms_codec::strategy`) for the random data.
- **Confidence**: high for the design; the fuzzing result is recorded in `docs/ai-log/M6.md`.
- **Test**: `tests/properties.rs` (25 properties). The fuzz target builds and runs; the long run required by M6 (at least
  eight hours) is set aside at the maintainer's request, see `docs/ai-log/M6.md`.

### D-027 — The access results of the bilateral table

- **Decision**: the table holds, per remote control centre, the data values, data sets, transfer sets and devices it may
  access, each with its permitted operations (the Permitted Access lists of 503 §5.3.6.2, §5.3.7.2, §5.3.10.2.1, §5.3.12.2), and for
  the data values and transfer sets the shortest interval agreed (the maximum exchange rate); each device carries its
  CheckBackID and its Timeout. The checks give these results:
  - `Granted`: the operation is in the permitted access of a visible object;
  - `UnknownObject`: the object is not in the table, or is in it without the Visibility permission (for the client it
    does not exist, so it is covered by no agreement);
  - `NotPermitted(operation)`: the object is visible, but the operation is not in its list;
  - `IntervalOutOfAgreement`: the requested interval is shorter than the agreed one;
  - `OutOfTime`: the operation on a device is later than its Timeout (the time at the timeout itself is within it);
  - the admission of a client (`UnknownClient`, `UnknownApplicationReference`, `Granted`) by the designation and the list of
    application references.
  The CheckBackID is returned only for a device the client may Select.
- **Why**: the general validity check and the access checks of 503 §5.2.3 say what is refused, not how; the table is local
  (503 §5.2.3.1). The result names follow the grounds of the refusal, so that each can be tested.
- **Sources**: IEC 60870-6-503 §5.2.3.1 to §5.2.3.8, §5.3.2, §5.3.4.2, §5.3.6.2, §5.3.7.2, §5.3.10.2.1, §5.3.12.2 (the Permitted
  Access lists), §6.5 (the table and its mapping).
- **Confidence**: high for the lists and the grounds; medium for treating a missing Visibility as `UnknownObject` (the
  standard says the object is or is not visible, not which answer the server gives).
- **Test**: `crates/igs-iec-tase2-link/tests/bilateral.rs`: one test per result, and the scope of a name.

### D-028 — The versions are compared in the order of the standard

- **Decision**: after the association, the Bilateral Table identifier is compared first (`Bilateral_Table_ID`, 8.3.3); only
  when it is identical, the TASE.2 version (`TASE.2_Version`, 8.3.4) is compared. A mismatch of either ends the association
  (a Conclude). The version of this standard is 2000.08 (major 2000, minor 8).
- **Why**: the standard gives this order in the text of the association: the second check happens only when the first
  passes, so a table identifier mismatch is reported whatever the TASE.2 versions.
- **Sources**: IEC 60870-6-503 §5.3.3.3 (the Associate operation: the read of the variables, then the two comparisons), §8.3.3,
  §8.3.4.
- **Confidence**: high.
- **Test**: `tests/bilateral.rs`, `identical_versions_are_compatible`, `a_different_table_identifier_is_the_first_conflict`,
  `a_different_tase2_version_is_a_conflict_when_the_table_identifiers_match`,
  `the_table_identifier_is_checked_before_the_tase2_version`.

### D-029 — What the bilateral table of B1 leaves to the later tasks

- **Decision**: the list of association objects (503 §6.5), the Access_violation event condition (503 §5.3.4.2), the
  security parameters of the data values, and the reading of the table over the MMS variables `Bilateral_Table_ID` and
  `TASE.2_Version` are not in B1. The association objects are B2; the reading of the variables is B2 (the client reads
  them after the association); the device operations are B5; the security parameters are local, as the standard leaves
  them (503 §6.5, the access control list is a local matter).
- **Why**: B1 is the table and its checks; the MMS exchanges belong to the association and the services.
- **Sources**: IEC 60870-6-503 §5.3.4.2, §6.5, §5.3.5 (the access control specification is a local matter).
- **Confidence**: high.
- **Test**: none in B1; B2 and B5 test these parts.

### D-030 — The client checks the server's variables after the MMS Initiate, in the order of the standard

- **Decision**: after a positive MMS Initiate, the client reads `Bilateral_Table_ID`, `TASE.2_Version` and
  `Supported_Features`. It compares the table identifier with the one of the Bilateral Agreement; only when they are
  identical, it compares the TASE.2 version. A mismatch of either is a conflict: the client concludes and makes no further
  Initiate until it is resolved. The features read are recorded with the association. The order of the reads, and whether
  they are interleaved with the checks, is outside the standard (503 §7.2.2.2.2).
- **Why**: the standard gives the checks in this order, and the second only when the first passes.
- **Sources**: IEC 60870-6-503 §7.2.2.2.2 (the client role), §5.3.3.3, §8.3.2 to §8.3.4.
- **Confidence**: high.
- **Test**: `crates/igs-iec-tase2-link/tests/association.rs`: the matching table and version, a mismatching table identifier
  (also when the version differs too), and a mismatching TASE.2 version.

### D-031 — The server accepts or declines an association request, and refuses a peer it is the initiator with

- **Decision**: a server that is the designated initiator of the associations with a peer refuses the Initiate request
  with an MMS Initiate-ErrorPDU (error class INITIATE, error code OTHER), whatever the table says. Otherwise the Bilateral
  Table decides: a client designation without a table, or an application reference not listed in its table, is declined
  with a negative MMS Initiate response. The designation of the initiator is a prior agreement between the control
  centres; the policy holds it, the tables do not (503 §5.3.2 and §7.2.2.2.1 leave it outside the table).
- **Why**: the two refusals are two different responses on the wire; the table cannot hold the initiator role, which is
  not an attribute of the Bilateral Table object model (503 §5.3.4.2).
- **Sources**: IEC 60870-6-503 §5.3.2, §7.2.2.2.1 (the initiator and the Initiate-ErrorPDU), §7.2.2.2.3 (the server role).
- **Confidence**: high for the two responses; medium for keeping the initiator agreement outside the table, which the
  standard does not state.
- **Test**: `tests/association.rs`: acceptance, an unknown client, an unlisted reference, and the initiator refusal.

### D-032 — The release on Conclude or Abort disables the transfer sets and deselects the devices

- **Decision**: when an association is concluded or aborted by the peer, the server releases what it holds on that
  association: every active transfer set becomes DISABLED and its name returns to the pool, and every device the client
  selected is deselected. The association holds nothing afterwards. The client, after a Conclude or Abort, attempts to
  re-establish the association (503 §7.2.2.2.2, §7.2.2.3.2, §7.2.2.4.2); the retry is the caller's.
- **Why**: the standard gives these as the outcome of the release on the server, for both the Conclude and the Abort.
- **Sources**: IEC 60870-6-503 §7.2.2.3.3 (the server role of the Conclude), §7.2.2.4.3 (the server role of the Abort).
- **Confidence**: high.
- **Test**: `tests/association.rs`: the release of the transfer sets and the devices, and a second release, which has
  nothing left.

### D-033 — The reads and writes of data values, item by item

- **Decision**: Get Data Value and Set Data Value answer per item. An object that the table does not contain, or that is not
  visible to the client, is `OBJECT-NON-EXISTENT` (class ACCESS); a visible object whose operation is not permitted is
  `OBJECT-ACCESS-DENIED`. A write to an object the server does not hold is `OBJECT-NON-EXISTENT`; a write that the server
  cannot make is the error the store gives (`HARDWARE-FAULT`, `TYPE-INCONSISTENT`, `OBJECT-ATTRIBUTE-UNSUPPORTED`). A failed
  item writes nothing. The type of a data value is read with the Permitted Access `Get Data Value Type`.
- **Why**: the standard gives these codes per operation (503 §7.2.3.2.3, §7.2.3.3.3, §7.2.3.5.3), and the visibility rule
  of §7.2.3.1 gives the non-existent answer for an invisible object.
- **Sources**: IEC 60870-6-503 §7.2.3.1 (the visibility rule), §7.2.3.2.3, §7.2.3.3.3, §7.2.3.5.3.
- **Confidence**: high.
- **Test**: `crates/igs-iec-tase2-link/tests/data.rs`, the `get_data_value…`, `set_data_value…` and `get_data_value_type…`
  tests.

### D-034 — The element reads and writes of a data set, item by item, with access denied for every refused member

- **Decision**: for Get and Set Data Set Element Values, a data set that the table does not contain, or does not show, is
  `OBJECT-NON-EXISTENT` for the operation as a whole; a data set without the operation's permission is
  `OBJECT-ACCESS-DENIED`. Each member is then answered on its own: a member that the table does not let the client read (or
  write), an invisible one included, is `FAILURE` with `OBJECT-ACCESS-DENIED`; so is a special identifier
  (`Transfer_Set_Name`, `DSConditions_Detected`, `Event_Code_Detected`, `Transfer_Set_Time_Stamp`). The other members are
  read or written.
- **Why**: the standard answers `OBJECT-ACCESS-DENIED` for a member without read or write access (§7.2.4.4.3, §7.2.4.5.3),
  and the visibility rule (§7.2.4.1) applies to the data set itself, not to its members.
- **Sources**: IEC 60870-6-503 §7.2.4.1, §7.2.4.4.3, §7.2.4.5.3 (the special identifiers of §7.2.4.2.3).
- **Confidence**: high for the members; medium for the data-set-level answer, which the standard gives only for the
  visibility of the data set.
- **Test**: `tests/data.rs`, the `get_data_set_element_values…` and `set_data_set_element_values…` tests.

### D-035 — Creating a data set: the checks in the order of the standard, and the create permission of the table

- **Decision**: a client creates a data set only if the table grants it (`with_data_set_creation`, the Create Data Set
  permission, which is not in the list of an object because the object does not exist yet). Then the checks come in the
  order of the standard, and the first failure is the answer: the access to each member (at least Read: any refusal,
  invisible included, is `OBJECT-ACCESS-DENIED`), the uniqueness of the name (`OBJECT-EXISTS`), the existence of each
  member except the special identifiers (`OBJECT-UNDEFINED`), the ICC-specific scope of the name
  (`OBJECT-ATTRIBUTE-INCONSISTENT`), and the room for the data set (`MEMORY-UNAVAILABLE`).
- **Why**: the standard lists the checks and their codes in this order (§7.2.4.2.3); the table flag keeps the permission
  visible in the model.
- **Sources**: IEC 60870-6-503 §7.2.4.2.1, §7.2.4.2.3.
- **Confidence**: high for the order and the codes; medium for the table flag, which is the model's, not the standard's.
- **Test**: `tests/data.rs`, the `create_data_set…` tests.

### D-036 — Deleting data sets: each name on its own

- **Decision**: each name of a Delete Data Set request is checked on its own, in the order of the standard: its existence
  (`OBJECT-UNDEFINED`), the access of the client (`OBJECT-ACCESS-DENIED`), the absence of an active transfer set that
  references it (`OBJECT-ACCESS-UNSUPPORTED`, the caller supplies the set of names in use), and the ICC-specific scope
  (`OBJECT-ATTRIBUTE-INCONSISTENT`). A name that fails is not deleted; the others are.
- **Why**: the standard says the failing object is not deleted (§7.2.4.3.3); it does not say the others are not, so they
  are deleted as the request asks.
- **Sources**: IEC 60870-6-503 §7.2.4.3.2, §7.2.4.3.3.
- **Confidence**: medium for the other names: a reading that deletes all or none would also fit the sentence.
- **Test**: `tests/data.rs`, the `delete_data_set…` tests.

### D-037 — A list of values of another length than the members is refused

- **Decision**: Set Data Set Element Values with a list of values that is not one value per member is refused as an
  attribute inconsistency of the data set (`OBJECT-ATTRIBUTE-INCONSISTENT`, class DEFINITION), before any write.
- **Why**: the standard gives the list of data in the order of the variables (§7.2.4.5.2) and does not give the case of a
  different length; refusing it is the only answer that writes nothing.
- **Sources**: IEC 60870-6-503 §7.2.4.5.2. The code is the model's choice, not the standard's.
- **Confidence**: low: the standard does not say; to be confirmed by the interop tests (T3).
- **Test**: `tests/data.rs`, `set_data_set_element_values_refuses_a_list_of_another_length`.

### D-038 — The timing of a Data Set Transfer Set: the start time, the intervals and the buffer

- **Decision**: a zero start time means now; otherwise the monitoring begins at the start time, and the interval (and the
  integrity check) is counted from it. A change before the start time is applied to the values but is neither reported
  nor kept for the first report. The timers that expired by the time of an input run first, in time order, so that a late
  clock gives the reports of every interval that expired (each with its own time). The buffer of the ObjectChange condition
  starts at the first change in it and ends `Buffer Time` later. Stop Transfer ends the monitoring at once: no further report,
  and the buffered changes are dropped (the name returned to the pool is the caller's).
- **Why**: the standard gives the start, the interval counted from the start time, the buffer that starts with the first
  change, and the end of monitoring by Stop Transfer (503 §5.3.10.2.2, §7.2.5.2.3, §7.2.5.3.3, §7.2.5.4.1, §7.2.5.4.2).
  The catch-up reports are the consequence of the periodic interval, not a choice.
- **Confidence**: high for the start and the interval; medium for dropping the buffered changes at Stop Transfer, which the
  standard does not say.
- **Test**: `crates/igs-iec-tase2-link/tests/transfer.rs`: the interval, the late clock, the future and the zero start time,
  the stop, the change before the start time, and the buffer.

### D-039 — The contents of a report, by the condition that triggers it

- **Decision**: IntervalTimeOut with RBE: the members changed since the last report, and no report when none changed; without
  RBE: the entire data set. ObjectChange without a buffer: the changed member with RBE, the entire data set without. ObjectChange
  with a buffer: the last value of each changed member, or every change in order when All Changes Reported is set; the
  entire data set without RBE. IntegrityTimeOut (with RBE): the entire data set. OperatorRequest and OtherExternalEvent (the
  code requested): the entire data set. A report clears the changes it carries.
- **Why**: the standard gives the content for the interval (current state of all, or of the changed objects), for the buffer,
  and for the integrity check (503 §5.3.10.2.2, §5.3.10.2.3, §7.2.5.5.3). For the two requests it gives no content; the
  entire data set is the one that answers a request for the state.
- **Confidence**: high for the interval, the buffer and the integrity check; medium for "no report when none changed" and for
  the content of the two requests, which the standard does not state.
- **Test**: `tests/transfer.rs`: the interval with RBE and no change, the object change with and without RBE, the buffer with
  the last value and with all changes, the integrity, the operator request, and the other event with and without the
  requested code.

### D-040 — The blocked report: its rule, and its values not yet encoded as octets

- **Decision**: a blocked report (BlockData true) is the rule and its values: rule 0 without RBE (the values of all the members,
  in order); with RBE, rule 1 when every position of the changed members fits in one octet, else rule 2 (the positions of the
  members in the data set). The values are kept as MMS data. The octet string of the standard, in which each value is encoded
  by its storage type of IEC 60870-6-802, is not built here.
- **Why**: the rules and the positions are the part of the blocking that the transfer set decides; the encoding of a value by
  its storage type belongs to the model of 802 and the link that sends the report (S1). The special transfer identifiers are
  not included in the report yet, for the same reason.
- **Sources**: IEC 60870-6-503 §7.2.5.5.3 (the blocked report, the header, rules 0, 1 and 2).
- **Confidence**: high for the rules; the encoding is open.
- **Test**: `tests/transfer.rs`, `a_blocked_report_without_rbe_carries_the_values_in_rule_0` and
  `a_blocked_report_with_rbe_carries_the_positions_of_the_changed_members_in_rule_1`.

### D-041 — The critical report: the acknowledgement, the negative acknowledgement, and the data failure

- **Decision**: a critical report waits for an acknowledgement of its sequence number; an acknowledgement of another number is
  refused. A negative acknowledgement (Transfer_Report_NACK), or the absence of the acknowledgement for the ack timeout (a
  policy of the server), raises a data failure (`Data_failure`) for the report, once.
- **Why**: the standard gives the acknowledgement that the server expects, the Transfer_Report_NACK, and the data failure
  (503 §7.2.5.5.2, §7.2.5.5.3). The time of the wait is local: the standard leaves it to the server.
- **Confidence**: high for the acknowledgement and the data failure; the timeout value is the policy's.
- **Test**: `tests/transfer.rs`, the critical acknowledgement, the unexpected acknowledgement, the timeout, and the negative
  acknowledgement.

### D-042 — The TLE: a report that would take longer is discarded

- **Decision**: when the TLE is not zero and the server expects to take longer than it to send a report, the report and its data
  are discarded and counted; a zero TLE is ignored. The expected send delay is a policy of the server.
- **Why**: the standard gives the rule: the server does not generate a report it cannot send within the TLE, and discards its
  data (503 §5.3.10.2.2, §7.2.5.5.3).
- **Confidence**: high. The rule that a report due after an earlier one exceeded the TLE is always generated is not
  implemented: it needs the time of the previous report, which the caller gives at the send.
- **Test**: `tests/transfer.rs`, `a_report_that_would_take_longer_than_its_tle_is_discarded` and
  `a_report_within_its_tle_is_sent`.

### D-043 — The parameters refused at the start

- **Decision**: the Start Transfer is refused with `OBJECT-ATTRIBUTE-INCONSISTENT` when the IntervalTimeOut condition is
  requested with a zero interval, or when the IntegrityTimeOut condition is requested with RBE and a zero integrity check. A
  refusal is the answer of the write of the transfer set.
- **Why**: the standard gives the error code for a Reporting Interval that the checks refuse (503 §7.2.5.2.3); a zero interval
  cannot expire, and the integrity check applies only with RBE.
- **Confidence**: medium: the standard names the check and the code, not the value that fails it.
- **Test**: `tests/transfer.rs`, `a_zero_interval_is_refused_at_the_start_of_the_interval_condition` and
  `a_zero_integrity_check_is_refused_with_rbe`.

### D-044 — B5 has no operation in 1.0: the device control and the information messages are blocks left out by D-009

- **Decision**: task B5 ("controls and messages of the chosen blocks, as far as F3 includes them") adds no operation to 1.0.
  The device operations (Select, Operate, Set Tag, with their CheckBackID and Timeout) belong to the block SBO Device Control
  (base `o`), and the information messages to the block Information Message (base `o`); D-009 leaves both out of 1.0, and no
  later decision adds them (criterion 2 of F3 waits for the reference peer, and its observation is recorded only as D-010).
  What these blocks need is already in place: the control types of M3 round-trip, the device entries of the bilateral table
  and their CheckBackID and Timeout (B1), and the transfer report of the transfer sets (B4).
- **Why**: the plan gives B5 the scope that F3 gives it; the scope of F3 is the Basic Services block.
- **Sources**: D-009; IEC 60870-6-702 (the blocks and their base); IEC 60870-6-503 §5.3.12.2 and §7.2.6 (the device operations,
  which are the content of the block).
- **Confidence**: high for the exclusion (D-009); the blocks come in a later decision, after the observation of the peer.
- **Test**: none in B5. The control types are tested in `igs-iec-tase2-model/tests/control.rs` (M3).

### D-045 — The name of the TASE.2 version variable: `TASE2_Version`

- **Decision**: the client reads the variable `TASE2_Version`, in the VMD scope, where 503 §8.3.4 prints `TASE.2_Version`.
- **Why**: an identifier of MMS, without the `char` building block, is drawn from letters, digits, `$` and `_` (ISO 9506-2,
  the production of `Identifier`). The dot of the printed name is not in that set, so the codec refuses to encode or decode it.
  The Basic Services parameter CBBs of 702 (Table A.19) do not list `char`. The maintainer chose the spelling without the dot
  (question of the T1 session).
- **Sources**: IEC 60870-6-503 §8.3.4 (the printed name); ISO 9506-2 (the production of `Identifier`, and the `char` CBB);
  IEC 60870-6-702 Table A.19.
- **Confidence**: medium. A real TASE.2 peer may print or name the variable differently; the interop tests (T3) decide. The
  printed name remains the one of the standard; the choice is a deviation, to be revisited with `char` (a later decision).
- **Test**: `crates/igs-iec-tase2/tests/client.rs`, all association tests (the stub names the variable so).

### D-046 — The client's operations on MMS, and what it does not do yet

- **Decision**: each operation of the data values and the data sets is one MMS service: Read and Write for the values (per
  item: `AccessResult` and `WriteResponseItemItem`), with the data access error codes mapped to the link's `ErrorCode`; an
  unknown code is kept as `ErrorCode::Other`. The members of a data set are read with GetNamedVariableListAttributes; Create
  and Delete Data Set are DefineNamedVariableList and DeleteNamedVariableList, and their negative responses are returned as
  the MMS error. Start Transfer and Stop Transfer write the whole DSTransferSet structure with its Status set to ENABLED or
  DISABLED. The client does not send the acknowledgement of a critical transfer report (503 §7.2.5.5.2): the MMS client has no
  unconfirmed send.
- **Why**: the standard maps the operations on these services (503 §7.2.3 to §7.2.5). The whole-structure write is a
  simplification: the standard writes the Status alone (§7.2.5.3.2), which needs a component address in the Write, not in the
  client yet.
- **Confidence**: high for the mapping; the whole-structure write and the missing acknowledgement are open (see below).
- **Open**: (1) the Stop Transfer writes the whole structure, not the Status component (alternate access); (2) the
  acknowledgement of a critical report, which needs an unconfirmed send in the MMS client (a gap of the sibling stack, to be
  recorded there); (3) the negative responses of Create and Delete are not yet mapped to the TASE.2 error codes.
- **Test**: `crates/igs-iec-tase2/tests/client.rs`: the reads and the writes per item, the data sets, the server's definition
  refusal, the transfer set the server does not hold, and the report received.

### D-047 — The events and their back-pressure

- **Decision**: the events of an association are a stream of connection states and transfer reports, from the bounded event
  channel of the MMS client (`ClientConfig::event_capacity`). A caller that does not drain the stream fills the channel; the
  connection then waits to deliver the next report. A critical report whose acknowledgement waits is then delayed, and the
  server's acknowledgement timeout may raise a data failure. This is documented in `client.rs`.
- **Why**: the MMS client bounds its channels (back-pressure of the stack); the TASE.2 client keeps it, and says so.
- **Confidence**: high.
- **Test**: `tests/client.rs`, `a_transfer_report_of_the_server_is_received_as_an_event` and
  `the_connection_state_is_an_event`.

### D-048 — The designation and the application reference of a client

- **Decision**: the Client Control Centre Designation is the dotted form of the calling AP-title of the AARQ (an object
  identifier). The application reference is the AE-title: the designation, followed by `/` and the AE qualifier when the
  AARQ gives one, else the designation alone. A table lists the references of its client; the server admits a client whose
  designation has a table and whose reference is listed (503 §5.3.2, §7.2.2.2.3). An AP-title or AE qualifier that is not in
  object-identifier or integer form is refused.
- **Why**: the standard gives the designation as an AP-title (503 §6.5) and each application reference as an AE-title (§6.3.4),
  without the text of the AE-title's form; the dotted form is the object identifier's own written form, and the qualifier is
  the least change that keeps two AE-titles of one AP-title apart.
- **Sources**: IEC 60870-6-503 §5.3.2, §6.3.4, §6.5; ISO 8650-1 (the AARQ fields `calling-AP-title`, `calling-AE-qualifier`).
- **Confidence**: medium: the written form of the designation is a choice; the interop tests (T3) and a real peer decide it.
- **Test**: `crates/igs-iec-tase2/tests/server.rs`, `the_association_is_established_with_the_special_objects_of_the_server`
  and `a_client_without_a_table_is_refused_the_association`.

### D-049 — The services of the TASE.2 server, and what S1 does not serve

- **Decision**: the server serves through a model of the MMS server (D-031 of the MMS stack, in the sibling repository on its
  branch `tase2-association-context`), which names the association of each call. Each request is checked against the table of
  the association's client and answered as the standard gives it per item: Read and Write of the data values (503 §7.2.3.2,
  §7.2.3.3), of the data sets (§7.2.4.4, §7.2.4.5), Get Data Value Type (§7.2.3.5), Create and Delete Data Set (§7.2.4.2,
  §7.2.4.3), GetNamedVariableListAttributes (the members of a visible data set), and Identify (the stack's answer). The
  special objects (`Bilateral_Table_ID` of the client's domain, `TASE2_Version`, `Supported_Features`) are answered from the
  table and the standard's version (503 §8.3.2 to §8.3.4). A Write to a configured transfer set starts it (ENABLED) or stops
  it (DISABLED), as the client writes the whole structure. The errors of a negative response are the MMS service errors of the
  standard's classes (ACCESS, DEFINITION, RESOURCE); an item error is the MMS data access error.
  Not served by S1: GetNameList (a listing of the names visible to a table, 503 §7.2.3.4, §7.2.4.6), the blocked transfer
  reports (BlockData), the Data_failure event condition, and the sending of an acknowledgement (which the MMS client cannot yet
  send, D-046 of the client). A transfer set whose BlockData is set is refused per item (`OBJECT-ATTRIBUTE-UNSUPPORTED`, which
  the MMS data access codes do not name: `OBJECT-ATTRIBUTE-INCONSISTENT` is sent on the wire).
- **Why**: the access checks of the standard need the identity of the association (MMS D-031); the services are the ones the
  client of T1 uses, so that each operation is tested through it.
- **Sources**: IEC 60870-6-503 §5.2.3.2, §7.2.2.2.2, §7.2.2.3.3, §7.2.3, §7.2.4, §7.2.5.2 to §7.2.5.5.
- **Confidence**: high for the mapping and the checks; medium for the error classes of the negative responses, which the
  standard names by codes the MMS stack gives in its own tables.
- **Test**: `crates/igs-iec-tase2/tests/server.rs` (15 tests, one or more per operation, through the client over TCP).

### D-050 — Application changes of a data value reach the transfer sets that monitor it

- **Decision**: the application changes a data value with `TaseServer::set_value`. The change is queued, and the model applies
  it at its next poll (`ServerConfig::poll_interval`): it is written to the store (the variable of the VMD, which must be
  writable), and each transfer set of each association that monitors the value receives it. A change that the store refuses is
  neither kept nor reported. The reports of the transfer sets, time-driven and change-driven, go to the association that
  started them. The clock is the wall clock (GMT seconds since the epoch, 503 §8.2.4).
- **Why**: the MMS stack's model task is the only owner of the state; a change made outside it passes through its poll, which
  keeps the order of the reports. A store that refuses a write holds no new value, so a report of it would be false.
- **Confidence**: high for the order and the refusal; the latency is one poll interval (50 ms by default).
- **Test**: `tests/server.rs`: `a_change_of_a_monitored_value_is_reported_to_its_association`,
  `a_report_by_exception_carries_the_changed_member_only`, `a_stopped_transfer_set_sends_no_further_report`,
  `a_change_the_store_refuses_is_neither_kept_nor_reported`.

### D-052 — The configuration of the demonstration server, and what it refuses

- **Decision**: the server's file is TOML, with the sections of `docs/igstase2-server.md`. Unknown keys are refused. Every
  reference is checked when the file is read: a domain, a point, a data set member, a table access and a simulation must name
  a declared object, and the refusal names it. A point's value must fit its type and its bits (1 to 127). A point is not
  writable unless its file says so. A simulation needs a writable point of a type that suits its kind (ramp: integer,
  unsigned or float with a step above zero and min below max; toggle: boolean; sine: float with min below max). The table's
  designation is the dotted AP-title; its identifier is at most 32 characters.
- **Why**: a demonstration server that starts with an invalid object would answer the clients of the bench with false values;
  the refusal names the object so that the file is corrected, not guessed. The format follows the file of the MMS server
  (`docs/igsmms-server.md`), so that the two benches read alike.
- **Sources**: the types of the points, from the MMS value types of the codec; the table and the access rules of IEC 60870-6-503
  §5.2.3, §5.3.4 (through D-027 and D-048). No standard text beyond these is used.
- **Confidence**: high for the checks (they are the ones of the types); the simulation kinds are a demonstration's choice.
- **Test**: `crates/igstase2-server/src/config.rs` (the refusals, one per check, and a valid file) and
  `tests/binary.rs` (a bad file exits with status 2 and names the object).

### D-053 — The bench device under test: the server built in the bookworm image, and the probe that reads it

- **Decision**: the device under test of the TASE.2 bench is `igstase2-server`, built by `bench/device/build-tase2.sh` in
  `rust:1.90-bookworm` (the workspace states a minimum of 1.85, which the local toolchain 1.85 also builds), and run as the
  compose service `device-tase2` (container `172.31.110.11`, host `127.0.0.1:10103`), with `bench/device/tase2-demo.toml`.
  The probe `crates/igs-iec-tase2/examples/probe.rs` associates with it as the client of the demonstration table, reads its
  points, starts its transfer set, and prints reports.
- **Result, recorded on 2026-11-07 in the bench**: the container starts and listens; the probe associates (supported features
  `0x1`), reads the four points (`Flow1` 6, `Level` 110, `Temperature` 21.5 as a single-precision real, `Breaker` true), and
  receives three transfer reports driven by the simulations, with the values of the points at each report.
- **Why**: the bench of the MMS stack runs its device the same way (the bench run of `docs/igsmms-server.md` in the sibling
  repository); the probe is the client
  of the bench for the TASE.2 device until the client binary (T2) exists.
- **Sources**: the bench design of F4 (`bench/README.md`); D-010 (the reference peer does not serve TASE.2 yet, so the device
  is tested with our own client).
- **Confidence**: high for the run; the probe is a demonstration, not the interop matrix (T3).
- **Test**: the probe, run against the device (above); `tests/binary.rs` runs the same kind of exchange against a process.

### D-054 — The dual-use mode is not supported

- **Decision**: the implementation does not support the dual-use mode of IEC 60870-6-503 §5.3.3.3, in which one association
  carries the client and the server directions of two TASE.2 users. Each association has one role: `TaseClient` (T1) on the
  initiating side, `TaseServer` (S1) on the responding side. The client and the server can run in one process, each on its own
  association, but this multiple-association mode is not tested together. The dual-use mode is outside 1.0.
- **What the standard requires, and what is met**: a TASE.2 implementation shall be able to act as client or as server over a
  single association; dual use is optional, and a dual-use site falls back to single-use associations with a peer that does not
  support it (503 §5.3.3.3). The single-role associations of 1.0 meet the mandatory part.
- **What is missing**:
  - the server side of an association cannot send a confirmed request and receive its answer: `igs_iso9506_mms::Server` has no
    API for it, although the link layer does not restrict requests by role;
  - the client side of an association cannot answer the requests of its peer: `igs_iso9506_mms::Client` handles only the
    unconfirmed reports of the peer;
  - no type unites `TaseClient` and `TaseServer` on one association, and the initiator rule of 503 §5.3.3.3 (the side designated
    to initiate, by prior agreement) is applied only on the server side (D-031).
- **Why outside 1.0**: the standard makes it optional, and the plan (`IMPLEMENTATION.md`) did not include it. Adding it means
  changes in the MMS stack (sibling repository) on both sides of the association.
- **Confidence**: high: the code has no path for it, and the MMS API has none.
- **Test**: none; the absence is checked by the review of the API (`crates/igs-iec-tase2/src/client.rs`,
  `crates/igs-iec-tase2/src/server.rs`).

### D-055 — A client may use the data set it creates

- **Decision**: when a client creates a data set, the server grants that client the access to it: visibility, deletion, and
  the reads and writes of its elements. The table of the client is updated for the rest of the association and of the
  server's life.
- **Why**: the agreement allows the client to create data sets (`data_set_creation`), and a data set that the client cannot
  see or delete after its creation is not usable. The found fault: the bench client created `DS9` and then could neither read
  its members nor delete it.
- **Sources**: IEC 60870-6-503 §7.2.4.2 (the creation and its checks), §5.2.3.4 (the access control of data sets). The standard
  does not say which access the creator receives; this is a choice of the implementation.
- **Confidence**: medium: the grant is the natural reading, but another server may keep the created data set private.
- **Test**: `crates/igs-iec-tase2/tests/server.rs`, `a_data_set_the_client_created_is_visible_to_it_and_deletable_by_it`.

### D-056 — The command-line client: one association per command, and the exit status of each answer

- **Decision**: `igstase2-client` runs one operation per association, then concludes it. A per-item answer (a point, a member)
  is printed with its error and does not change the exit status; a refusal of the whole request, of the association, or of a
  command that cannot start is an error with status 1. A Stop Transfer is refused when the transfer set is not active on the
  association of the command (the server releases it when the association of the Start ended).
- **Why**: the standard answers item by item (503 §7.2.3, §7.2.4), so the exit status follows the request, not the items; a
  Start and its reports need one association, which the command keeps for its duration.
- **Sources**: IEC 60870-6-503 §7.2.2.3.3 (the release of the transfer sets at the end of an association), §7.2.5.3.3.
- **Confidence**: high for the status rule; the limit of a stop in another association follows from D-032.
- **Test**: `crates/igstase2-client/tests/cli.rs` (the help, the usage status, the per-item answer with status 0, the refused
  association with status 1), and the transcript `bench/transcripts/T2-run.txt`.

