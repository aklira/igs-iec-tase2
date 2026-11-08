# Changelog

All notable changes to this project are recorded in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Fixed

- `igs-iec-tase2`: an MMS ObjectName is read with an integer or an unsigned scope: the peers write either.
- `igs-iec-tase2`: the two bits of a tag are read and written in the order of the standard: `Tag_hi` is the high bit of the
  code, so that CLOSE-ONLY-INHIBIT is the string `1 0`. The earlier order read and wrote the two codes swapped; an interoperability
  test with a third-party implementation found it.
- `igstase2-server`: the configuration may name VMD-specific objects with the empty domain, and gives an access its own domain.

### Added

- `igs-iec-tase2`: `Next_DSTransfer_Set` (503 §8.3.7): the first transfer set of the domain that no association monitors.
- `igs-iec-tase2`: a transfer set is configured component by component (alternate access, `valt`), and started by its Status;
  `mms_capabilities` announces alternate access for a TASE.2 server.
- `igs-iec-tase2`: `Client::names_in_domain`, the GetNameList of the variables or the data sets of the client's domain, every
  page of the list. `igstase2-client names [--data-sets]` prints them.

## [1.0.0] - 2026-11-07

### Added

- `igs-iec-tase2-model`: the types of IEC 60870-6-802 and 60870-6-503, generated from their data files, with their conversions
  to and from MMS values: scalars and times, the quality flags and the two-bit codes, the indication and control points, the
  device outages, the protection events, the data set transfer sets and their conditions. The reals are single precision, and a
  real that is not finite is refused. The strategies and the property tests are behind the `strategy` feature.
- `igs-iec-tase2-link`: sans-I/O bilateral tables with their access checks and the version checks; the association (the client's
  check after the Initiate, the server's admission, the release); the data values and the data sets, operation by operation; the
  data set transfer set, with its conditions, its reports, its blocking, its acknowledgement and its TLE.
- `igs-iec-tase2`: `TaseClient` (association, data values, data sets, transfer sets, the event stream of reports) and
  `TaseServer` (the same services under the table of each client, the reports to the association that started the transfer set).
- `igs-iec-tase2`: the devices of block 5 (Select, Operate, Set Tag, Get Tag, with the CheckBackID, the timeout and the tags), the
  Information Message Transfer Set of block 4 (start, stop, and the reports of the server), and the block-2 conditions tested end to end
  (the integrity check, the buffer time, the TLE).
- `igstase2-server`: the demonstration server, with its configuration file and simulated changes (the device under test of the bench).
  Its configuration gains the devices (`[[device]]`), the table access of a device, and periodic information messages (`[[information]]`).
- `igstase2-client`: the command-line client (get, set, type, members, read, write, create, delete, start, stop), and the commands of
  the devices (select, operate, get-tag, set-tag), of the information messages (start-info, stop-info), and the block-2 options of
  start (`--integrity-s`, `--buffer-s`, `--tle-s`).
- The interop bench (`bench/`): the device and the client in their containers, and the client interop matrix, with the scenarios
  of the blocks 2, 4 and 5 (`bench/tests/test_c8_blocks.py`).
- `igstase2-load`: the load driver (many associations, 10 000 points, latencies measured, the counters of errors and stray
  reports), and the bench's load run (`bench/tests/test_s3_load.py`, `docs/load-test.md`).
- The documentation: `docs/capabilities.md` (the blocks, the operations, the test of each row, the known gaps), the client
  and server documents, and the decisions of `PROVENANCE.md`.

### Fixed

- The CheckBackID of a device is an integer of 16 bits, as its MMS type gives it; it was stored as a string (decision D-063).
- A request's names are each answered: a name that is not a TASE.2 name no longer shifts the answers of the others.

### Known limits

- The dual-use mode (one association for both directions) is not supported (decision D-054).
- The scope of 1.0 is Basic Services, Extended Conditions, Information Message and SBO Device Control (decision D-059). Blocked
  Transfers (block 3), and the Programs, Events, Accounts and Time Series blocks, are not implemented (decisions D-009 and D-059).
- The device commands are simulated on the server, and the device actions are not sent as MMS events (decision D-066).
- The information messages are sent by the server only (decision D-061).
- GetNameList, the blocked transfer reports and the Data_failure event are not served; a critical report's acknowledgement is not
  sent by the client (decisions D-046 and D-049).
- The name of the TASE.2 version variable is `TASE2_Version`, without the dot of the printed name (decision D-045).
- The interop with a third-party TASE.2 peer is not verified: the reference peer does not serve TASE.2 yet (decision D-010).
