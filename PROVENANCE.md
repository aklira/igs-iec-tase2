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
- **Test**: to be written (task F2).

### D-002 — Spelling of type names

- **Decision**: the names used by the code follow the data files: `COVCounter` (the definition of
  802 §6.2 is spelled `COV_Counter`), `OutageAmountTypeId`, `ReferenceNum`, `InfoBufXX`.
- **Sources**: count of each spelling in the PDF of 802 and 503.
- **Confidence**: medium, deduced from the names.
- **Test**: to be written (task F2).

### D-003 — The current text of 503 prevails over its annex G

- **Decision**: where `SupportedFeatures` and `TSConditions` differ, the current text is used;
  annex G describes the 2002 edition and is informative.
- **Sources**: IEC 60870-6-503 §8.2, annex G.
- **Confidence**: high.
- **Test**: to be written (task M4).

## Third-party code and sources

| Source | Licence | Use | Provenance |
| --- | --- | --- | --- |
| rusty-iccp | MIT, to be confirmed at task F4 | Reference peer of the bench, black box only | Source never read |
