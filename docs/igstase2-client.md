# `igstase2-client`: the command-line TASE.2 client

`igstase2-client` associates with a TASE.2 server as the client of its Bilateral Table, runs one operation, and concludes the
association. It is built on the TASE.2 client library (`igs_iec_tase2::TaseClient`, task T1).

## Build and run

```bash
cargo run -p igstase2-client -- --help
cargo run -p igstase2-client -- -s 127.0.0.1:10103 get Flow1 Level Temperature Breaker
```

Global options (before the command):

- `-s, --server` (default `127.0.0.1:102`): the address of the server.
- `--ap-title` (default `1.3.9999.1`): this client's AP-title, in dotted form. It is the designation of its Bilateral Table.
- `--domain` (default `ICC1`): the domain that the server gives this client's objects.
- `--table-id` (default `BT-2026-001`): the identifier of the Bilateral Table, as the agreement holds it. The server's
  `Bilateral_Table_ID` must match it.
- `--timeout-s` (default 10): how long to wait for a report.

## Commands

| Command | Meaning |
| --- | --- |
| `get POINT...` | Reads the values of points. |
| `set NAME=VALUE...` | Writes values. A value is `true`, `false`, an integer, or a finite real. |
| `type POINT` | Shows the MMS type of a point. |
| `members DATASET` | Lists the members of a data set. |
| `read DATASET` | Reads the values of the members, in member order. |
| `write DATASET VALUE...` | Writes one value per member, in member order. Each member's answer is printed with its name. |
| `create DATASET MEMBER...` | Creates a data set. The client may then use it (decision D-055). |
| `delete DATASET...` | Deletes data sets. |
| `start TRANSFERSET DATASET [--rbe] [--interval-s N] [--object-change] [--reports N]` | Starts a transfer set on a data set and prints N reports (default 1). With no condition given, `--object-change` is used. |
| `stop TRANSFERSET DATASET` | Stops a transfer set. |

Points and data sets are named in the domain of the client (`--domain`).

## Exit status

- **0**: the operation was answered. A per-item answer is printed with its error (for example `Temperature: error
  ObjectAccessDenied`), and the command still exits with 0: each item has its own answer.
- **1**: the association was refused or concluded (the table identifier, the TASE.2 version, or the AP-title does not match),
  or the server refused the whole request (for example a `stop` of a transfer set that is not active on this association).
- **2**: the command line is not valid (clap's status).

## Limits of 1.0

- **Values are sent as the command line gives them.** A value is an integer, a real, or a boolean. For a point of an unsigned
  type, the server refuses an integer as `TypeInconsistent`; the client does not yet read the type first.
- **A transfer set is active only on the association that started it.** The server releases it when the association ends, so
  a `stop` in a new command is refused as not active.
- **No dual use.** The client is one side of one association (decision D-054).

## Bench run

The client was run against the bench device (`docs/igstase2-server.md`, service `device-tase2`). The transcript is in
`bench/transcripts/T2-run.txt`, with the exit status of each command.
