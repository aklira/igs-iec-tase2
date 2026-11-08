// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! `igstase2-client`: a command-line TASE.2 client (task T2). It associates with a TASE.2 server as the client of its
//! Bilateral Table, and runs one operation: it reads, writes and creates the points and data sets, starts and stops a
//! data set transfer set, and prints its reports. Usage: `igstase2-client --help`, and `docs/igstase2-client.md`.
//!
//! Exit status: 0 when the operation succeeds; 1 when the association is refused or concluded, or when the server
//! answers with an error; 2 for a command line that is not valid (clap's status).

use std::process::ExitCode;
use std::time::Duration;

use clap::{Parser, Subcommand};
use igs_iec_tase2::{
    Agreement, Error, ReceivedReport, ReportAccess, TaseClient, TaseEvent, TaseEvents,
};
use igs_iec_tase2_link::bilateral::{ObjectName, Scope};
use igs_iec_tase2_link::data::Item;
use igs_iec_tase2_link::device::{DeviceState, Tag, TagState};
use igs_iec_tase2_link::information::InformationMessage;
use igs_iec_tase2_link::version::{Versions, STANDARD_VERSION};
use igs_iec_tase2_model::builtin::{Integer16, Integer32, MmsIdentifier};
use igs_iec_tase2_model::generated::iso503::{
    DSConditions, DSTransferSet, DSTransferSetAllChangesReported, DSTransferSetBlockData,
    DSTransferSetCritical, DSTransferSetRBE, DSTransferSetStatus, MMSObjectName,
    MMSObjectNameScope, VisibleString32,
};
use igs_iec_tase2_model::generated::iso802::clause_6_2_1::DataReal as DataRealValue;
use igs_iec_tase2_model::mms::MmsValue;
use igs_iso9506_mms::client::{in_scope_capabilities, ClientConfig, Reconnect};
use igs_iso9506_mms_codec::generated::types::{Data, ServiceError, ServiceErrorErrorClass};
use igs_iso9506_mms_codec::schema::ObjectIdentifier;
use igs_iso9506_mms_link::config::LinkConfig;

/// The command line.
#[derive(Debug, Parser)]
#[command(
    name = "igstase2-client",
    version,
    about = "A command-line TASE.2 client: reads, writes and monitors the points and data sets of a TASE.2 server."
)]
struct Cli {
    /// The address of the server (`IP:PORT`; TASE.2 uses port 102).
    #[arg(long, short = 's', default_value = "127.0.0.1:102")]
    server: String,
    /// The AP-title of this client, in dotted form: the designation of its Bilateral Table.
    #[arg(long, default_value = "1.3.9999.1")]
    ap_title: String,
    /// The domain of this client: the domain that the server gives its objects.
    #[arg(long, default_value = "ICC1")]
    domain: String,
    /// The identifier of the Bilateral Table, as the agreement holds it (`Bilateral_Table_ID`).
    #[arg(long, default_value = "BT-2026-001")]
    table_id: String,
    /// How long to wait for the server's answers, in seconds.
    #[arg(long, default_value_t = 10, value_parser = clap::value_parser!(u64).range(1..=600))]
    timeout_s: u64,
    /// The calling transport selector, in hexadecimal (for example `0001`); absent by default. Some servers require it.
    #[arg(long)]
    calling_tsap: Option<String>,
    /// The called transport selector, in hexadecimal (for example `0001`); absent by default. Some servers require it.
    #[arg(long)]
    called_tsap: Option<String>,
    /// Proposes the application context of the 2002 edition (1.0.9506.2.3) instead of the current one, for the servers of that edition.
    #[arg(long)]
    legacy_application_context: bool,
    #[command(subcommand)]
    command: Command,
}

/// The operations.
#[derive(Debug, Subcommand)]
enum Command {
    /// Reads the values of points. Each point is a name in the domain of the client.
    Get {
        /// The names of the points.
        points: Vec<String>,
    },
    /// Writes the values of points: `name=value`, where the value is `true`, `false`, an integer, or a real.
    Set {
        /// The assignments, for example `Flow1=7`.
        assignments: Vec<String>,
    },
    /// Shows the type of a point.
    Type {
        /// The name of the point.
        point: String,
    },
    /// Lists the members of a data set.
    Members {
        /// The name of the data set.
        data_set: String,
    },
    /// Reads the values of the members of a data set.
    Read {
        /// The name of the data set.
        data_set: String,
    },
    /// Writes the values of the members of a data set, one per member, in member order.
    Write {
        /// The name of the data set.
        data_set: String,
        /// The values, in the order of the members (`true`, `false`, an integer, or a real).
        values: Vec<String>,
    },
    /// Creates a data set with the given members.
    Create {
        /// The name of the new data set.
        data_set: String,
        /// The points that it holds.
        members: Vec<String>,
    },
    /// Deletes data sets.
    Delete {
        /// The names of the data sets.
        data_sets: Vec<String>,
    },
    /// Starts a data set transfer set on a data set, and prints the reports it receives.
    Start {
        /// The name of the transfer set object of the server.
        transfer_set: String,
        /// The name of the data set to report.
        data_set: String,
        /// Reports the changed members only (report by exception).
        #[arg(long)]
        rbe: bool,
        /// Reports at this interval, in seconds (the IntervalTimeOut condition); zero means no interval.
        #[arg(long, default_value_t = 0)]
        interval_s: u32,
        /// Reports each change of a member (the ObjectChange condition). Set when no other condition is given.
        #[arg(long)]
        object_change: bool,
        /// Checks the data set for integrity at this interval, in seconds, with RBE (the IntegrityTimeOut condition, block 2).
        #[arg(long, default_value_t = 0)]
        integrity_s: u32,
        /// Buffers the changes for this time, in seconds, before reporting them (the BufferTime, block 2).
        #[arg(long, default_value_t = 0)]
        buffer_s: u32,
        /// The time within which a report must be sent, in seconds (the TLE, block 2); zero means no limit.
        #[arg(long, default_value_t = 0)]
        tle_s: u32,
        /// How many reports to print before stopping.
        #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=1000))]
        reports: u32,
    },
    /// Stops a data set transfer set.
    Stop {
        /// The name of the transfer set object of the server.
        transfer_set: String,
        /// The name of the data set that it reports.
        data_set: String,
    },
    /// Selects an SBO device: prints the CheckBackID that the server returns, and leaves the device armed for this association.
    Select {
        /// The name of the device.
        device: String,
    },
    /// Operates a device with a command. An SBO device is selected first, in the same association; `--direct` skips the select
    /// (a non-SBO device).
    Operate {
        /// The name of the device.
        device: String,
        /// The command: `true`, `false`, an integer, or a real.
        command: String,
        /// Operates without selecting first: for a non-SBO device.
        #[arg(long)]
        direct: bool,
    },
    /// Shows the tag of a device, its reason and its state.
    GetTag {
        /// The name of the device.
        device: String,
    },
    /// Sets the tag of a device: `no-tag`, `open-and-close-inhibit` or `close-only-inhibit`.
    SetTag {
        /// The name of the device.
        device: String,
        /// The tag.
        tag: String,
        /// The reason of the tag (at most 255 characters).
        #[arg(long, default_value = "")]
        reason: String,
    },
    /// Starts the Information Message Transfer Set, and prints the messages it receives.
    StartInfo {
        /// How many messages to print before stopping.
        #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=1000))]
        messages: u32,
    },
    /// Stops the Information Message Transfer Set.
    StopInfo,
}

/// A failure of the operation, with the message to print.
#[derive(Debug)]
struct Failure(String);

impl From<Error> for Failure {
    fn from(error: Error) -> Self {
        match error {
            Error::Mms(igs_iso9506_mms::Error::Service(service)) => {
                Self(describe_service(&service))
            }
            Error::Mms(igs_iso9506_mms::Error::Refused(_)) => Self(
                "the server refused the association: no Bilateral Table for this AP-title, or the application reference is not listed"
                    .to_string(),
            ),
            other => Self(other.to_string()),
        }
    }
}

/// A negative answer of the server, with the class and the code of its MMS error, named as the standard names them.
fn describe_service(error: &ServiceError) -> String {
    let (class, code, names): (&str, i64, &[(i64, &str)]) = match &error.error_class {
        ServiceErrorErrorClass::Access(code) => (
            "access",
            code.0,
            &[
                (1, "OBJECT-ACCESS-UNSUPPORTED"),
                (2, "OBJECT-NON-EXISTENT"),
                (3, "OBJECT-ACCESS-DENIED"),
            ],
        ),
        ServiceErrorErrorClass::Definition(code) => (
            "definition",
            code.0,
            &[
                (1, "OBJECT-UNDEFINED"),
                (5, "OBJECT-EXISTS"),
                (6, "OBJECT-ATTRIBUTE-INCONSISTENT"),
            ],
        ),
        ServiceErrorErrorClass::Resource(code) => {
            ("resource", code.0, &[(1, "MEMORY-UNAVAILABLE")])
        }
        _ => return format!("the server refused the request: {error:?}"),
    };
    let name = names
        .iter()
        .find(|(value, _)| *value == code)
        .map_or("unnamed", |(_, name)| name);
    format!("the server refused the request: {class} class, {name} (code {code})")
}

fn plain(domain: &str, name: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain(domain.to_string()),
        name: name.to_string(),
    }
}

/// A value from the command line: `true`, `false`, an integer, or a real.
fn parse_value(text: &str) -> Result<Data, String> {
    match text {
        "true" => return Ok(Data::Boolean(true)),
        "false" => return Ok(Data::Boolean(false)),
        _ => {}
    }
    if let Ok(integer) = text.parse::<i128>() {
        return Ok(Data::Integer(integer));
    }
    match text.parse::<f64>() {
        Ok(real) if real.is_finite() => Ok(DataRealValue(real).to_mms()),
        _ => Err(format!(
            "{text:?} is not a value: true, false, an integer, or a finite real"
        )),
    }
}

/// A value, as the command line prints it.
fn show(data: &Data) -> String {
    match data {
        Data::Integer(value) => value.to_string(),
        Data::Unsigned(value) => value.to_string(),
        Data::Boolean(value) => value.to_string(),
        Data::VisibleString(text) => format!("{text:?}"),
        Data::FloatingPoint(_) => match DataRealValue::from_mms(data) {
            Ok(real) => real.0.to_string(),
            Err(_) => format!("{data:?}"),
        },
        other => format!("{other:?}"),
    }
}

/// An item, as the command line prints it: its value, or its error.
fn show_item(item: &Item<Data>) -> String {
    match item {
        Item::Success(value) => show(value),
        Item::Failure(code) => format!("error {code:?}"),
    }
}

/// A transport selector written in hexadecimal: an even number of digits, one or two octets long (the length the standard allows).
fn parse_selector(text: &str) -> Result<Vec<u8>, String> {
    if text.len() % 2 != 0 || text.is_empty() || text.len() > 4 {
        return Err(format!(
            "{text:?} is not a selector: one or two octets in hexadecimal"
        ));
    }
    (0..text.len())
        .step_by(2)
        .map(|start| {
            text.get(start..start.saturating_add(2))
                .and_then(|pair| u8::from_str_radix(pair, 16).ok())
                .ok_or_else(|| format!("{text:?} is not hexadecimal"))
        })
        .collect()
}

fn parse_designation(text: &str) -> Result<Vec<u64>, String> {
    text.split('.')
        .map(|part| {
            part.parse::<u64>()
                .map_err(|_| format!("{text:?} is not an AP-title (dotted numbers)"))
        })
        .collect()
}

/// The transfer set that a Start or a Stop writes, on the data set named.
/// The transmission parameters of a transfer set: the interval, and the block-2 parameters.
#[derive(Debug, Clone, Copy, Default)]
struct Timing {
    interval_s: u32,
    integrity_s: u32,
    buffer_s: u32,
    tle_s: u32,
}

fn transfer_set(
    domain: &str,
    data_set: &str,
    conditions: u64,
    rbe: bool,
    timing: Timing,
    enabled: bool,
) -> Result<DSTransferSet, Failure> {
    let parts =
        |text: &str| MmsIdentifier::new(text).map_err(|e| Failure(format!("{text:?}: {e:?}")));
    Ok(DSTransferSet {
        data_set_name: MMSObjectName {
            scope: MMSObjectNameScope(MMSObjectNameScope::DOMAIN_SPECIFIC),
            domain_name: parts(domain)?,
            name: parts(data_set)?,
        },
        start_time: Integer32::new(0).map_err(|e| Failure(format!("{e:?}")))?,
        interval: Integer32::new(i64::from(timing.interval_s))
            .map_err(|e| Failure(format!("{e:?}")))?,
        tle: Integer32::new(i64::from(timing.tle_s)).map_err(|e| Failure(format!("{e:?}")))?,
        buffer_time: Integer32::new(i64::from(timing.buffer_s))
            .map_err(|e| Failure(format!("{e:?}")))?,
        integrity_check: Integer32::new(i64::from(timing.integrity_s))
            .map_err(|e| Failure(format!("{e:?}")))?,
        ds_conditions_requested: DSConditions(conditions),
        block_data: DSTransferSetBlockData(false),
        critical: DSTransferSetCritical(false),
        rbe: DSTransferSetRBE(rbe),
        all_changes_reported: DSTransferSetAllChangesReported(false),
        status: DSTransferSetStatus(enabled),
        event_code_requested: Integer16::new(0).map_err(|e| Failure(format!("{e:?}")))?,
    })
}

fn print_report(number: u32, report: &ReceivedReport) {
    let access = match &report.access {
        ReportAccess::Names(names) => names
            .iter()
            .map(|n| n.name.clone())
            .collect::<Vec<_>>()
            .join(","),
        ReportAccess::DataSet(name) => format!("data set {}", name.name),
    };
    let values: Vec<String> = report.values.iter().map(show_item).collect();
    println!("report {number}: {access}: {}", values.join(", "));
}

/// The next report of the events, skipping the connection events, within the timeout.
async fn next_report(
    events: &mut TaseEvents,
    timeout: Duration,
) -> Result<ReceivedReport, Failure> {
    loop {
        match tokio::time::timeout(timeout, events.next()).await {
            Err(_) => return Err(Failure("no report within the timeout".to_string())),
            Ok(None) => return Err(Failure("the association ended before a report".to_string())),
            Ok(Some(Err(error))) => return Err(Failure(error.to_string())),
            Ok(Some(Ok(TaseEvent::Report(report)))) => return Ok(report),
            Ok(Some(Ok(TaseEvent::Connection(_) | TaseEvent::Information(_)))) => continue,
        }
    }
}

async fn run(cli: &Cli) -> Result<(), Failure> {
    let designation = parse_designation(&cli.ap_title).map_err(Failure)?;
    let table_id = VisibleString32::new(&cli.table_id)
        .map_err(|e| Failure(format!("the table identifier: {e:?}")))?;
    let config = ClientConfig {
        reconnect: Reconnect::Never,
        ..ClientConfig::new(
            LinkConfig {
                ap_title: Some(ObjectIdentifier(designation)),
                calling_tsap: cli
                    .calling_tsap
                    .as_deref()
                    .map(parse_selector)
                    .transpose()
                    .map_err(Failure)?,
                called_tsap: cli
                    .called_tsap
                    .as_deref()
                    .map(parse_selector)
                    .transpose()
                    .map_err(Failure)?,
                application_context: if cli.legacy_application_context {
                    igs_iso9506_mms_link::config::ApplicationContext::Legacy
                } else {
                    igs_iso9506_mms_link::config::ApplicationContext::Current
                },
                ..LinkConfig::default()
            },
            in_scope_capabilities(),
        )
    };
    let agreement = Agreement {
        domain: cli.domain.clone(),
        versions: Versions {
            table_id,
            tase2: STANDARD_VERSION,
        },
    };
    let (client, mut events) = TaseClient::associate(&cli.server, config, &agreement).await?;
    let timeout = Duration::from_secs(cli.timeout_s);
    let result = execute(cli, &client, &mut events, timeout).await;
    // The association is concluded however the operation ended.
    let _ = client.close().await;
    result
}

async fn execute(
    cli: &Cli,
    client: &TaseClient,
    events: &mut TaseEvents,
    timeout: Duration,
) -> Result<(), Failure> {
    let domain = cli.domain.as_str();
    let object = |name: &str| plain(domain, name);
    match &cli.command {
        Command::Get { points } => {
            let names: Vec<ObjectName> = points.iter().map(|p| object(p)).collect();
            let items = client.get_data_values(&names).await?;
            for (name, item) in names.iter().zip(&items) {
                println!("{}: {}", name.name, show_item(item));
            }
        }
        Command::Set { assignments } => {
            let mut writes = Vec::new();
            for assignment in assignments {
                let (name, value) = assignment
                    .split_once('=')
                    .ok_or_else(|| Failure(format!("{assignment:?} is not name=value")))?;
                writes.push((object(name), parse_value(value).map_err(Failure)?));
            }
            let names: Vec<String> = writes.iter().map(|(name, _)| name.name.clone()).collect();
            for (name, item) in names.iter().zip(client.set_data_values(writes).await?) {
                match item {
                    Item::Success(()) => println!("{name}: written"),
                    Item::Failure(code) => println!("{name}: error {code:?}"),
                }
            }
        }
        Command::Type { point } => {
            let ty = client.get_data_value_type(&object(point)).await?;
            println!("{point}: {ty:?}");
        }
        Command::Members { data_set } => {
            for member in client.data_set_members(&object(data_set)).await? {
                println!("{}", member.name);
            }
        }
        Command::Read { data_set } => {
            for item in client.get_data_set(&object(data_set)).await? {
                println!("{}", show_item(&item));
            }
        }
        Command::Write { data_set, values } => {
            let data = values
                .iter()
                .map(|v| parse_value(v).map_err(Failure))
                .collect::<Result<Vec<_>, _>>()?;
            let members = client.data_set_members(&object(data_set)).await?;
            for (member, item) in members
                .iter()
                .zip(client.set_data_set(&object(data_set), data).await?)
            {
                match item {
                    Item::Success(()) => println!("{}: written", member.name),
                    Item::Failure(code) => println!("{}: error {code:?}", member.name),
                }
            }
        }
        Command::Create { data_set, members } => {
            let members: Vec<ObjectName> = members.iter().map(|m| object(m)).collect();
            client.create_data_set(&object(data_set), &members).await?;
            println!("created {data_set}");
        }
        Command::Delete { data_sets } => {
            let names: Vec<ObjectName> = data_sets.iter().map(|d| object(d)).collect();
            let deleted = client.delete_data_sets(&names).await?;
            println!("deleted {deleted}");
        }
        Command::Start {
            transfer_set: ts,
            data_set,
            rbe,
            interval_s,
            object_change,
            integrity_s,
            buffer_s,
            tle_s,
            reports,
        } => {
            let mut conditions = 0u64;
            if *interval_s > 0 {
                conditions |= DSConditions::INTERVAL_TIME_OUT;
            }
            if *integrity_s > 0 {
                conditions |= DSConditions::INTEGRITY_TIME_OUT;
            }
            if *object_change || conditions == 0 {
                conditions |= DSConditions::OBJECT_CHANGE;
            }
            let timing = Timing {
                interval_s: *interval_s,
                integrity_s: *integrity_s,
                buffer_s: *buffer_s,
                tle_s: *tle_s,
            };
            let set = transfer_set(domain, data_set, conditions, *rbe, timing, true)?;
            match client.start_transfer(&object(ts), &set).await? {
                Item::Success(()) => println!("started {ts} on {data_set}"),
                Item::Failure(code) => {
                    return Err(Failure(format!(
                        "the server refused the transfer set: {code:?}"
                    )))
                }
            }
            for number in 1..=*reports {
                let report = next_report(events, timeout).await?;
                print_report(number, &report);
            }
        }
        Command::Stop {
            transfer_set: ts,
            data_set,
        } => {
            let set = transfer_set(
                domain,
                data_set,
                DSConditions::OBJECT_CHANGE,
                false,
                Timing::default(),
                false,
            )?;
            match client.stop_transfer(&object(ts), &set).await? {
                Item::Success(()) => println!("stopped {ts}"),
                Item::Failure(code) => {
                    return Err(Failure(format!("the server refused the stop: {code:?}")))
                }
            }
        }
        Command::Select { device } => match client.select_device(&object(device)).await? {
            Item::Success(check_back) => {
                println!("selected {device}: CheckBackID {}", check_back.get());
            }
            Item::Failure(code) => {
                return Err(Failure(format!("the server refused the select: {code:?}")))
            }
        },
        Command::Operate {
            device,
            command,
            direct,
        } => {
            let value = parse_value(command).map_err(Failure)?;
            if !direct {
                match client.select_device(&object(device)).await? {
                    Item::Success(_) => {}
                    Item::Failure(code) => {
                        return Err(Failure(format!("the server refused the select: {code:?}")))
                    }
                }
            }
            match client.operate_device(&object(device), value).await? {
                Item::Success(()) => println!("operated {device}"),
                Item::Failure(code) => {
                    return Err(Failure(format!("the server refused the operate: {code:?}")))
                }
            }
        }
        Command::GetTag { device } => match client.get_tag(&object(device)).await? {
            Item::Success(state) => println!("{device}: {}", show_tag_state(&state)),
            Item::Failure(code) => {
                return Err(Failure(format!("the server refused the tag: {code:?}")))
            }
        },
        Command::SetTag {
            device,
            tag,
            reason,
        } => {
            let tag = parse_tag(tag).map_err(Failure)?;
            match client.set_tag(&object(device), tag, reason).await? {
                Item::Success(()) => println!("{device}: tag set to {}", tag_name(tag)),
                Item::Failure(code) => {
                    return Err(Failure(format!("the server refused the tag: {code:?}")))
                }
            }
        }
        Command::StartInfo { messages } => {
            match client.start_information_transfer().await? {
                Item::Success(()) => println!("started the information messages"),
                Item::Failure(code) => {
                    return Err(Failure(format!(
                        "the server refused the information transfer set: {code:?}"
                    )))
                }
            }
            for number in 1..=*messages {
                let message = next_information(events, timeout).await?;
                print_information(number, &message);
            }
        }
        Command::StopInfo => match client.stop_information_transfer().await? {
            Item::Success(()) => println!("stopped the information messages"),
            Item::Failure(code) => {
                return Err(Failure(format!(
                    "the server refused the stop of the information transfer set: {code:?}"
                )))
            }
        },
    }
    Ok(())
}

/// The name of a tag, as the command line writes it.
fn tag_name(tag: Tag) -> &'static str {
    match tag {
        Tag::NoTag => "no-tag",
        Tag::OpenAndCloseInhibit => "open-and-close-inhibit",
        Tag::CloseOnlyInhibit => "close-only-inhibit",
    }
}

/// The tag of a name of the command line, in any case.
fn parse_tag(text: &str) -> Result<Tag, String> {
    match text.to_ascii_lowercase().as_str() {
        "no-tag" => Ok(Tag::NoTag),
        "open-and-close-inhibit" => Ok(Tag::OpenAndCloseInhibit),
        "close-only-inhibit" => Ok(Tag::CloseOnlyInhibit),
        other => Err(format!(
            "{other:?} is not a tag: no-tag, open-and-close-inhibit or close-only-inhibit"
        )),
    }
}

fn show_tag_state(state: &TagState) -> String {
    let device_state = match state.state {
        DeviceState::Idle => "IDLE",
        DeviceState::Armed => "ARMED",
    };
    format!(
        "tag {}, reason {:?}, state {device_state}",
        tag_name(state.tag),
        state.reason
    )
}

/// The next information message of the events, skipping the other events, within the timeout.
async fn next_information(
    events: &mut TaseEvents,
    timeout: Duration,
) -> Result<InformationMessage, Failure> {
    loop {
        match tokio::time::timeout(timeout, events.next()).await {
            Err(_) => {
                return Err(Failure(
                    "no information message within the timeout".to_string(),
                ))
            }
            Ok(None) => {
                return Err(Failure(
                    "the association ended before an information message".to_string(),
                ))
            }
            Ok(Some(Err(error))) => return Err(Failure(error.to_string())),
            Ok(Some(Ok(TaseEvent::Information(message)))) => return Ok(message),
            Ok(Some(Ok(TaseEvent::Report(_) | TaseEvent::Connection(_)))) => continue,
        }
    }
}

fn print_information(number: u32, message: &InformationMessage) {
    let stream: String = message
        .stream()
        .iter()
        .map(|octet| format!("{octet:02x}"))
        .collect();
    println!(
        "message {number}: info reference {}, local reference {}, message id {}, {} octets: {stream}",
        message.info_reference(),
        message.local_reference(),
        message.message_id(),
        message.stream().len(),
    );
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(Failure(message)) => {
            eprintln!("igstase2-client: {message}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn a_value_is_a_boolean_an_integer_or_a_real() {
        assert_eq!(parse_value("true"), Ok(Data::Boolean(true)));
        assert_eq!(parse_value("-12"), Ok(Data::Integer(-12)));
        assert_eq!(show(&parse_value("2.5").unwrap()), "2.5");
    }

    #[test]
    fn a_value_that_is_none_of_them_is_refused_with_the_text() {
        let error = parse_value("nan").unwrap_err();
        assert!(error.contains("\"nan\""), "{error}");
        assert!(parse_value("1e400").is_err(), "an infinite real is refused");
    }

    #[test]
    fn an_ap_title_is_dotted_numbers() {
        assert_eq!(parse_designation("1.3.9999.1"), Ok(vec![1, 3, 9999, 1]));
        assert!(parse_designation("1.x").is_err());
    }

    #[test]
    fn a_tag_is_named_in_any_case_and_refused_otherwise() {
        assert_eq!(parse_tag("NO-TAG"), Ok(Tag::NoTag));
        assert_eq!(
            parse_tag("Open-And-Close-Inhibit"),
            Ok(Tag::OpenAndCloseInhibit)
        );
        assert_eq!(parse_tag("close-only-inhibit"), Ok(Tag::CloseOnlyInhibit));
        assert!(parse_tag("inhibit").is_err());
        for tag in [Tag::NoTag, Tag::OpenAndCloseInhibit, Tag::CloseOnlyInhibit] {
            assert_eq!(parse_tag(tag_name(tag)), Ok(tag));
        }
    }

    #[test]
    fn a_tag_state_shows_its_tag_reason_and_state() {
        let state = TagState {
            tag: Tag::CloseOnlyInhibit,
            reason: "crew on site".to_string(),
            state: DeviceState::Armed,
        };
        assert_eq!(
            show_tag_state(&state),
            "tag close-only-inhibit, reason \"crew on site\", state ARMED"
        );
    }

    #[test]
    fn the_commands_of_the_devices_and_of_the_information_messages_parse() {
        use clap::Parser as _;
        let parse = |args: &[&str]| {
            let mut all = vec!["igstase2-client", "-s", "127.0.0.1:102"];
            all.extend_from_slice(args);
            Cli::try_parse_from(all)
        };
        assert!(parse(&["select", "Breaker12"]).is_ok());
        assert!(parse(&["operate", "Breaker12", "1"]).is_ok());
        assert!(parse(&["operate", "Valve", "true", "--direct"]).is_ok());
        assert!(parse(&["get-tag", "Breaker12"]).is_ok());
        assert!(parse(&[
            "set-tag",
            "Breaker12",
            "close-only-inhibit",
            "--reason",
            "work"
        ])
        .is_ok());
        assert!(parse(&["start-info", "--messages", "3"]).is_ok());
        assert!(parse(&["stop-info"]).is_ok());
        assert!(parse(&[
            "start",
            "TS1",
            "DS1",
            "--integrity-s",
            "5",
            "--buffer-s",
            "1",
            "--tle-s",
            "10"
        ])
        .is_ok());
        assert!(
            parse(&["start-info", "--messages", "0"]).is_err(),
            "at least one message"
        );
    }

    #[test]
    fn a_selector_is_one_or_two_hexadecimal_octets() {
        assert_eq!(parse_selector("0001"), Ok(vec![0x00, 0x01]));
        assert_eq!(parse_selector("01"), Ok(vec![0x01]));
        assert!(parse_selector("001").is_err(), "an odd number of digits");
        assert!(parse_selector("00010203").is_err(), "more than two octets");
        assert!(parse_selector("zz").is_err(), "not hexadecimal");
        assert!(parse_selector("").is_err(), "empty");
    }
}
