# `igstase2-server`: the demonstration TASE.2 server

`igstase2-server` serves the points, data sets and transfer sets of a TOML file to the clients of its Bilateral Tables
(IEC 60870-6-503), and makes by itself the changes that the file describes. It is built on the TASE.2 server library
(`igs_iec_tase2::TaseServer`, task S1) and the MMS server of `igs-iso9506-mms`. It is the device under test of the bench
(`bench/`, service `device-tase2`).

## Build and run

```bash
cargo run -p igstase2-server -- --listen 127.0.0.1:10103 --config bench/device/tase2-demo.toml
```

`igstase2-server --help` gives the options:

- `--listen` (default `127.0.0.1:102`): the address to listen on. TASE.2 uses port 102.
- `--config` (required): the configuration file, described below.
- `--max-associations` (default 64, from 1 to 10 000): the associations at once; a connection beyond the limit is closed.

The server runs until it is stopped with Ctrl-C, and then exits with status 0. The status is 2 when the file is refused, 1
when the server cannot listen.

To see it from a client, the probe of the library runs the client of the demonstration table (AP-title `1.3.9999.1`), reads
the points, starts the transfer set `TS1`, and prints three reports:

```bash
cargo run -p igs-iec-tase2 --example probe -- 127.0.0.1:10103
```

## The configuration file

The file is TOML. Unknown keys are refused, with the line of the error, and every reference is checked when the file is read:
a point, data set or transfer set that another section names must be declared, and the refusal names the object. The sections,
in the order of the example `bench/device/tase2-demo.toml`:

| Section | Keys | Meaning |
| --- | --- | --- |
| `[server]` | `poll_ms` (50), `ack_timeout_s` (30), `expected_send_delay_ms` (0), `data_set_capacity` (64) | The model's poll period; the wait for the acknowledgement of a critical report; the expected send time (a report that would take longer than the TLE is discarded); the number of data sets. |
| `[identity]` | `vendor`, `model`, `revision` | The answer to Identify. |
| `[[domain]]` | `name` | A domain. The empty domain (`domain = ""` in a point, a device, a data set or an access) is the VMD: its names are VMD-specific, not named in a domain. |
| `[[point]]` | `domain`, `name`, `type`, `bits`, `value`, `writable` | A data value. `writable` is false by default. |
| `[[data-set]]` | `domain`, `name`, `members` | A data set, configured at the start (the clients may also create some). `members` are points of the same domain. |
| `[[transfer-set]]` | `domain`, `name` | A transfer set object. A client starts or stops it by writing it. |
| `[[device]]` | `domain`, `name`, `class`, `closing_commands` | A device (503 §5.3.12): `class` is `sbo` or `non-sbo`; `closing_commands` are the commands that close or raise, which CLOSE-ONLY-INHIBIT refuses. Its command is simulated: an accepted Operate completes at once. |
| `[[table]]` | `designation`, `table_id`, `domain`, `application_references`, `data_set_creation`, `initiator` | The Bilateral Table of one client (below). |
| `[[table.access]]` | `kind`, `name`, `domain`, `permissions`, `interval_ms`, `check_back_id`, `timeout_s` | What the client may do with one object of its table. `domain` is the domain of the object when it is not that of the table. A device needs `check_back_id` (the value a Select returns, an integer of 16 bits) and `timeout_s`, and the `visibility` permission to be seen at all. |
| `[[simulate]]` | `domain`, `name`, `kind`, `period_ms`, `min`, `max`, `step` | A change that the server makes by itself, at each period. |
| `[[information]]` | `info_reference`, `local_reference`, `stream`, `period_ms` | An information message that the server sends at each period, with a MessageId counted from 1 (decision D-061). Its stream is the text, as UTF-8. |

Types of a point (`type`):

| `type` | Value | Notes |
| --- | --- | --- |
| `integer` | a signed integer that fits `bits` | `bits` 1 to 127; 32 by default. |
| `unsigned` | an integer of zero or more that fits `bits` | `bits` 1 to 127; 32 by default. |
| `float` | a finite number, single precision | |
| `boolean` | `true` or `false` | |

Permissions of a table access (`permissions`): `visibility` (without it the object does not exist for the client),
`get-data-value`, `set-data-value`, `get-data-value-type`, `delete-data-set`, `get-data-set-element-values`,
`set-data-set-element-values`, `start-transfer`, `stop-transfer`, `select`, `operate`, `set-tag`.

Kinds of an access (`kind`): `data-value`, `data-set`, `transfer-set`, `device`. `interval_ms` is the shortest interval the client may ask
for in a transfer set on that data value (the maximum exchange rate of the agreement).

Kinds of a simulation (`kind`):

| `kind` | Point | Behaviour |
| --- | --- | --- |
| `ramp` | integer, unsigned, float | From `min`, up by `step` at each period; after `max`, it starts again at `min`. `step` above zero, `min` below `max`. |
| `toggle` | boolean | Changes at each period. |
| `sine` | float | Follows a sine between `min` and `max`, over 60 periods. |

A simulated point must be `writable = true`: the change is made through the store, as the application's own write. The table
still decides what a client may do with it.

## The Bilateral Table of a client

A `[[table]]` is the Bilateral Table of the client whose designation is its AP-title, written in dotted form (`1.3.9999.1`).
The client is admitted when its application reference is listed: the designation, followed by `/` and the AE qualifier when the
client's AARQ gives one. `initiator = true` makes this server the designated initiator of the associations with that client: it
then refuses them with an Initiate-ErrorPDU (IEC 60870-6-503 §7.2.2.2.1).

The client reads `Bilateral_Table_ID` (the `table_id`, at most 32 characters) and `TASE2_Version` (2000.08) after the association,
and compares them with its agreement. `data_set_creation = true` lets the client create data sets.

## Behaviour

- **Requests** are checked against the table of the client that makes them. An object that is not visible (no `visibility`) is
  answered as non-existent; a permission that is missing is answered as access denied (IEC 60870-6-503 §7.2.3, §7.2.4).
- **Transfer sets** are started and stopped by the client, by a write of the whole transfer set. Their reports go to the
  association that started them: each change of a monitored point (`OBJECT_CHANGE`), the interval, the integrity check, the
  operator request and the other events, with the rules of IEC 60870-6-503 §7.2.5.
- **Devices** are served under the table of the client: a Select reads `<name>_SBO` (the CheckBackID of the table, and the device is
  armed for the client for `timeout_s`); an Operate writes the command of the device (it completes at once, the device simulated);
  a Set Tag and a Get Tag use `<name>_TAG`. The tag refuses the commands as IEC 60870-6-503 §5.3.12 gives them (decision D-064).
- **Information messages**: the client starts and stops its Information Message Transfer Set (`IM_Transfer_Set`), and the server
  reports the messages that the application sends (decision D-061).
- **Application changes** come from the simulations. A change that the store refuses (a point that is not writable) is neither
  kept nor reported.
- **Transfer sets** are configured whole, or component by component (alternate access): a Status of ENABLED starts the monitoring with
  the attributes written so far, and a Status of DISABLED stops it. `Next_DSTransfer_Set` names the first transfer set of the domain
  that no association monitors.
- **Not served**: GetNameList (the listing of the names a table shows), the blocked transfer reports (`block_data`), the
  Data_failure event, the device actions (Timeout, Local Reset, Success, Failure) as MMS event conditions (the stack serves no event
  conditions), the Local Reset as an operation of the application, and the dual-use mode of IEC 60870-6-503 §5.3.3.3: an association has one role, the server's, and the
  server cannot send requests to its client on it (decision D-054). A critical report's acknowledgement is not sent by the server's client side (the MMS client has no
  unconfirmed send): the server signals the data failure after `ack_timeout_s`, internally.

## Bench run

The device runs in the bench's Debian container, from a binary built in the bookworm image:

```bash
cd bench
sh device/build-tase2.sh                 # writes device/bin/igstase2-server (the sibling igs-iso9506-mms is read, not changed)
podman-compose up -d device-tase2        # listens on 127.0.0.1:10103 (container 172.31.110.11, port 102)
cd .. && cargo run -p igs-iec-tase2 --example probe -- 127.0.0.1:10103
podman-compose down
```

`bench/device/tase2-demo.toml` is the file of the bench.
