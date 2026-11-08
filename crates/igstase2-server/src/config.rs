// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The configuration file of `igstase2-server` (format: `docs/igstase2-server.md`): its parsing, and its checks, which name
//! the object that is wrong. The file describes the points, the data sets and the transfer sets of the server, the Bilateral
//! Tables of its clients, and the changes that the demonstration makes by itself.

use std::collections::BTreeSet;
use std::time::Duration;

use igs_iec_tase2::{DeviceConfig, TaseServerConfig};
use igs_iec_tase2_link::association::ServerPolicy;
use igs_iec_tase2_link::bilateral::{
    BilateralTable, BilateralTables, DeviceAccess, ObjectAccess, ObjectName, Permission, Scope,
};
use igs_iec_tase2_link::device::DeviceClass;
use igs_iec_tase2_link::transfer::Policy;
use igs_iec_tase2_link::version::{Versions, STANDARD_VERSION};
use igs_iec_tase2_model::generated::iso503::VisibleString32;
use igs_iec_tase2_model::generated::iso802::SBOCheckBackName;
use igs_iec_tase2_model::mms::MmsValue;
use igs_iso9506_mms::server::ServerConfig;
use igs_iso9506_mms::vmd::{Identity, MemoryVariable, Name, Target, Vmd};
use igs_iso9506_mms_codec::generated::types::{
    Data, TypeDescription, TypeDescriptionFloatingPoint, Unsigned8,
};
use igs_iso9506_mms_link::config::LinkConfig;
use serde::Deserialize;

/// The configuration file, as it is written.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The server's timing and limits.
    #[serde(default)]
    pub server: ServerSection,
    /// The identity that the server answers to Identify.
    #[serde(default)]
    pub identity: IdentitySection,
    /// The domains of the server.
    #[serde(default, rename = "domain")]
    pub domains: Vec<DomainSection>,
    /// The data values (points) of the server.
    #[serde(default, rename = "point")]
    pub points: Vec<PointSection>,
    /// The data sets of the server, configured at its start.
    #[serde(default, rename = "data-set")]
    pub data_sets: Vec<DataSetSection>,
    /// The transfer set objects of the server.
    #[serde(default, rename = "transfer-set")]
    pub transfer_sets: Vec<TransferSetSection>,
    /// The devices of the server (503 §5.3.12): their commands are simulated.
    #[serde(default, rename = "device")]
    pub devices: Vec<DeviceSection>,
    /// The Bilateral Tables, one per client.
    #[serde(default, rename = "table")]
    pub tables: Vec<TableSection>,
    /// The changes that the server makes by itself, to demonstrate the transfer sets.
    #[serde(default, rename = "simulate")]
    pub simulations: Vec<SimulationSection>,
    /// The information messages that the server sends by itself, to demonstrate the Information Message Transfer Set.
    #[serde(default, rename = "information")]
    pub information: Vec<InformationSection>,
}

/// The server's timing and limits.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct ServerSection {
    /// The period of the model's poll, in milliseconds: the delay of an application change before its report.
    pub poll_ms: u64,
    /// The time the server waits for the acknowledgement of a critical report, in seconds.
    pub ack_timeout_s: u64,
    /// The time the server expects to take to send a report, in milliseconds: a report that would take longer than the
    /// TLE of its transfer set is discarded.
    pub expected_send_delay_ms: u64,
    /// The number of data sets the server can hold.
    pub data_set_capacity: usize,
}

impl Default for ServerSection {
    fn default() -> Self {
        Self {
            poll_ms: 50,
            ack_timeout_s: 30,
            expected_send_delay_ms: 0,
            data_set_capacity: 64,
        }
    }
}

/// The identity of the server.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct IdentitySection {
    /// The vendor name.
    pub vendor: String,
    /// The model name.
    pub model: String,
    /// The revision.
    pub revision: String,
}

impl Default for IdentitySection {
    fn default() -> Self {
        Self {
            vendor: "igs".to_string(),
            model: "igstase2 demo".to_string(),
            revision: "0.1".to_string(),
        }
    }
}

/// A domain of the server.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainSection {
    /// The name of the domain.
    pub name: String,
}

/// The kind of a point: the MMS type of its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PointKind {
    /// A signed integer of `bits` bits (the default is 32).
    Integer,
    /// An unsigned integer of `bits` bits (the default is 32).
    Unsigned,
    /// A single-precision real.
    Float,
    /// A boolean.
    Boolean,
}

/// A data value (point) of the server.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PointSection {
    /// The domain of the point.
    pub domain: String,
    /// The name of the point.
    pub name: String,
    /// The kind of the point.
    #[serde(rename = "type")]
    pub kind: PointKind,
    /// The width of an integer, in bits (1 to 127; 32 by default).
    #[serde(default)]
    pub bits: Option<u32>,
    /// The initial value.
    pub value: toml::Value,
    /// Whether a client may write the point (the Bilateral Table still decides for each client).
    #[serde(default)]
    pub writable: bool,
}

/// A data set of the server, configured at its start.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataSetSection {
    /// The domain of the data set.
    pub domain: String,
    /// The name of the data set.
    pub name: String,
    /// The members: names of points of the same domain.
    pub members: Vec<String>,
}

/// The class of a device, as the file writes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceClassKind {
    /// Select-Before-Operate: the device must be selected before it is operated.
    Sbo,
    /// Direct-Control: the device is operated directly.
    NonSbo,
}

/// A device of the server (503 §5.3.12). Its command is simulated: an accepted Operate completes at once.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeviceSection {
    /// The domain of the device.
    pub domain: String,
    /// The name of the device: its command is the variable `<name>`, its CheckBackID `<name>_SBO`, its tag `<name>_TAG`.
    pub name: String,
    /// The class of the device.
    pub class: DeviceClassKind,
    /// The commands that close or raise, which CLOSE-ONLY-INHIBIT refuses. Their meaning is local (503 §5.3.12.4).
    #[serde(default)]
    pub closing_commands: Vec<i64>,
}

/// A transfer set object of the server.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransferSetSection {
    /// The domain of the transfer set.
    pub domain: String,
    /// The name of the transfer set.
    pub name: String,
}

/// The kind of an access of a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccessKind {
    /// A data value.
    DataValue,
    /// A data set.
    DataSet,
    /// A transfer set.
    TransferSet,
    /// A device: its permitted operations need `check-back-id` and `timeout-s`.
    Device,
}

/// The access of a client to one object of its table.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessSection {
    /// The kind of the object.
    pub kind: AccessKind,
    /// The name of the object, in the domain of the table.
    pub name: String,
    /// The domain of the object when it is not the domain of the table; the empty domain is the VMD (a VMD-specific name).
    #[serde(default)]
    pub domain: Option<String>,
    /// The permitted operations: `visibility`, `get-data-value`, `set-data-value`, `get-data-value-type`, `delete-data-set`,
    /// `get-data-set-element-values`, `set-data-set-element-values`, `start-transfer`, `stop-transfer`.
    pub permissions: Vec<String>,
    /// The shortest interval that the client may ask for, in milliseconds (the maximum exchange rate of the agreement).
    #[serde(default)]
    pub interval_ms: Option<u64>,
    /// The CheckBackID that a Select returns, for a device: an integer of 16 bits (802 §6.2.2).
    #[serde(default)]
    pub check_back_id: Option<i64>,
    /// The time within which an operation on a device must be performed, in seconds (the Timeout of the table).
    #[serde(default)]
    pub timeout_s: Option<u64>,
}

/// The Bilateral Table of one client.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableSection {
    /// The designation of the client: the dotted form of its AP-title, for example `1.3.9999.1`.
    pub designation: String,
    /// The identifier of the table (`Bilateral_Table_ID`), at most 32 characters.
    pub table_id: String,
    /// The domain of the client, which holds the objects of the table.
    pub domain: String,
    /// The application references that may establish an association: the AP-title, or the AP-title and `/` and the AE
    /// qualifier.
    pub application_references: Vec<String>,
    /// Whether the client may create data sets.
    #[serde(default)]
    pub data_set_creation: bool,
    /// Whether this server is the designated initiator of the associations with the client (503 §7.2.2.2.1).
    #[serde(default)]
    pub initiator: bool,
    /// The objects of the table, with their permitted operations.
    #[serde(default)]
    pub access: Vec<AccessSection>,
}

/// The kind of a simulated change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SimulationKind {
    /// An integer or a real that goes from `min` up by `step`, and starts again at `min` after `max`.
    Ramp,
    /// A boolean that changes at each period.
    Toggle,
    /// A real that follows a sine wave between `min` and `max`, over 60 periods.
    Sine,
}

/// A change that the server makes by itself, at a period.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationSection {
    /// The domain of the point.
    pub domain: String,
    /// The name of the point.
    pub name: String,
    /// The kind of change.
    pub kind: SimulationKind,
    /// The period, in milliseconds.
    pub period_ms: u64,
    /// The lowest value (ramp and sine).
    #[serde(default)]
    pub min: f64,
    /// The greatest value (ramp and sine).
    #[serde(default)]
    pub max: f64,
    /// The step of a ramp.
    #[serde(default)]
    pub step: f64,
}

/// A configuration that is refused, with the object that is wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError(pub String);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ConfigError {}

fn invalid(context: &str, message: impl std::fmt::Display) -> ConfigError {
    ConfigError(format!("{context}: {message}"))
}

/// An information message that the server sends at each period (503 §7.2.5.5.4).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InformationSection {
    /// The Info Reference of the messages (an integer of 32 bits).
    pub info_reference: i64,
    /// The Local Reference of the messages (an integer of 32 bits).
    pub local_reference: i64,
    /// The text of the stream, as its octets (UTF-8).
    pub stream: String,
    /// The period between two messages, in milliseconds.
    pub period_ms: u64,
}

/// An information message to send at each period, ready to run: its MessageId is counted from 1.
#[derive(Debug, Clone, PartialEq)]
pub struct InformationSource {
    /// The Info Reference.
    pub info_reference: i64,
    /// The Local Reference.
    pub local_reference: i64,
    /// The stream.
    pub stream: Vec<u8>,
    /// The period between two messages.
    pub period: Duration,
}

/// A simulated change, ready to run.
#[derive(Debug, Clone, PartialEq)]
pub struct Simulation {
    /// The point that changes.
    pub name: ObjectName,
    /// The kind of change.
    pub kind: SimulationKind,
    /// The period.
    pub period: Duration,
    /// The lowest value.
    pub min: f64,
    /// The greatest value.
    pub max: f64,
    /// The step of a ramp.
    pub step: f64,
    /// The kind of the point.
    pub point: PointKind,
}

/// The configuration, built and checked: the VMD of the points, the TASE.2 server configuration, and the simulations.
pub struct Built {
    /// The VMD that holds the points.
    pub vmd: Vmd,
    /// The configuration of the TASE.2 server.
    pub server: TaseServerConfig,
    /// The changes to make by themselves.
    pub simulations: Vec<Simulation>,
    /// The information messages to send by themselves.
    pub information: Vec<InformationSource>,
}

/// The scope of an object: the domain it names, or the VMD for the empty domain (a VMD-specific name).
fn scope_of(domain: &str) -> Scope {
    if domain.is_empty() {
        Scope::Vmd
    } else {
        Scope::Domain(domain.to_string())
    }
}

/// The object of the VMD that a point of the given domain and name is: in its domain, or VMD-specific for the empty domain.
fn target_of(domain: &str, name: &str, context: &str) -> Result<Target, ConfigError> {
    let name = Name::new(name).map_err(|e| invalid(context, e))?;
    if domain.is_empty() {
        Ok(Target::vmd(name))
    } else {
        Ok(Target::in_domain(
            Name::new(domain).map_err(|e| invalid(context, e))?,
            name,
        ))
    }
}

fn object(domain: &str, name: &str) -> ObjectName {
    ObjectName {
        scope: scope_of(domain),
        name: name.to_string(),
    }
}

fn permission(text: &str, context: &str) -> Result<Permission, ConfigError> {
    Ok(match text {
        "visibility" => Permission::Visibility,
        "get-data-value" => Permission::GetDataValue,
        "set-data-value" => Permission::SetDataValue,
        "get-data-value-type" => Permission::GetDataValueType,
        "delete-data-set" => Permission::DeleteDataSet,
        "get-data-set-element-values" => Permission::GetDataSetElementValues,
        "set-data-set-element-values" => Permission::SetDataSetElementValues,
        "start-transfer" => Permission::StartTransfer,
        "stop-transfer" => Permission::StopTransfer,
        "select" => Permission::Select,
        "operate" => Permission::Operate,
        "set-tag" => Permission::SetTag,
        other => {
            return Err(invalid(
                context,
                format!("the permission {other:?} is not known"),
            ))
        }
    })
}

/// The value of a point, as the MMS type of the point and the data it holds.
fn typed(point: &PointSection) -> Result<(TypeDescription, Data), ConfigError> {
    let context = format!("point {}/{}", point.domain, point.name);
    if point.kind != PointKind::Integer && point.kind != PointKind::Unsigned && point.bits.is_some()
    {
        return Err(invalid(&context, "bits is given for an integer only"));
    }
    match point.kind {
        PointKind::Integer | PointKind::Unsigned => {
            let bits = point.bits.unwrap_or(32);
            if !(1..=127).contains(&bits) {
                return Err(invalid(
                    &context,
                    format!("bits is {bits}; it must be 1 to 127"),
                ));
            }
            let value = point
                .value
                .as_integer()
                .ok_or_else(|| invalid(&context, "the value is not an integer"))?;
            let value = i128::from(value);
            // The bounds of an integer of `bits` bits: half of 2^bits below zero and above it, or 0 to 2^bits - 1.
            let half = 1i128
                .checked_shl(bits.saturating_sub(1))
                .unwrap_or(i128::MAX);
            let span = 1i128.checked_shl(bits).unwrap_or(i128::MAX);
            let (low, high) = if point.kind == PointKind::Integer {
                (0i128.saturating_sub(half), half.saturating_sub(1))
            } else {
                (0, span.saturating_sub(1))
            };
            if value < low || value > high {
                return Err(invalid(
                    &context,
                    format!("the value {value} does not fit {bits} bits"),
                ));
            }
            let width = Unsigned8(i64::from(bits));
            if point.kind == PointKind::Integer {
                Ok((TypeDescription::Integer(width), Data::Integer(value)))
            } else {
                let unsigned =
                    u128::try_from(value).map_err(|_| invalid(&context, "a negative unsigned"))?;
                Ok((TypeDescription::Unsigned(width), Data::Unsigned(unsigned)))
            }
        }
        PointKind::Float => {
            let value = match &point.value {
                toml::Value::Float(value) => *value,
                toml::Value::Integer(value) => *value as f64,
                _ => return Err(invalid(&context, "the value is not a number")),
            };
            if !value.is_finite() {
                return Err(invalid(&context, "the value is not finite"));
            }
            let ty = TypeDescription::FloatingPoint(TypeDescriptionFloatingPoint {
                format_width: Unsigned8(32),
                exponent_width: Unsigned8(8),
            });
            let data =
                igs_iec_tase2_model::generated::iso802::clause_6_2_1::DataReal(value).to_mms();
            Ok((ty, data))
        }
        PointKind::Boolean => {
            let value = point
                .value
                .as_bool()
                .ok_or_else(|| invalid(&context, "the value is not a boolean"))?;
            Ok((TypeDescription::Boolean, Data::Boolean(value)))
        }
    }
}

/// Builds the configuration, and checks every reference: a point, a data set or a transfer set that another section names
/// must be declared. `max_associations` is the limit of the server, from the command line.
pub fn build(config: &Config, max_associations: usize) -> Result<Built, ConfigError> {
    let domains: BTreeSet<&str> = config
        .domains
        .iter()
        .map(|domain| domain.name.as_str())
        .collect();
    let domain_known = |domain: &str, context: &str| {
        if domain.is_empty() || domains.contains(domain) {
            Ok(())
        } else {
            Err(invalid(
                context,
                format!("the domain {domain:?} is not declared"),
            ))
        }
    };

    let identity = Identity {
        vendor: config.identity.vendor.clone(),
        model: config.identity.model.clone(),
        revision: config.identity.revision.clone(),
    };
    let mut vmd = Vmd::new(identity);
    for domain in &config.domains {
        let name = Name::new(&domain.name)
            .map_err(|e| invalid(&format!("domain {:?}", domain.name), e))?;
        vmd.add_domain(name)
            .map_err(|e| invalid(&format!("domain {:?}", domain.name), format!("{e:?}")))?;
    }

    let mut points = BTreeSet::new();
    for point in &config.points {
        let context = format!("point {}/{}", point.domain, point.name);
        domain_known(&point.domain, &context)?;
        if !points.insert(object(&point.domain, &point.name)) {
            return Err(invalid(&context, "the point is declared twice"));
        }
        let target = target_of(&point.domain, &point.name, &context)?;
        let (ty, value) = typed(point)?;
        let variable = MemoryVariable::new(ty, value, point.writable)
            .map_err(|e| invalid(&context, format!("{e:?}")))?;
        vmd.add_variable(&target, Box::new(variable))
            .map_err(|e| invalid(&context, format!("{e:?}")))?;
    }

    let mut device_configs = Vec::new();
    let mut device_names = BTreeSet::new();
    for section in &config.devices {
        let context = format!("device {}/{}", section.domain, section.name);
        domain_known(&section.domain, &context)?;
        let name = object(&section.domain, &section.name);
        if !device_names.insert(name.clone()) {
            return Err(invalid(&context, "the device is declared twice"));
        }
        device_configs.push(DeviceConfig {
            name,
            class: match section.class {
                DeviceClassKind::Sbo => DeviceClass::Sbo,
                DeviceClassKind::NonSbo => DeviceClass::NonSbo,
            },
            closing_commands: section.closing_commands.clone(),
        });
    }

    let mut data_sets = Vec::new();
    let mut data_set_names = BTreeSet::new();
    for set in &config.data_sets {
        let context = format!("data set {}/{}", set.domain, set.name);
        domain_known(&set.domain, &context)?;
        let name = object(&set.domain, &set.name);
        if !data_set_names.insert(name.clone()) {
            return Err(invalid(&context, "the data set is declared twice"));
        }
        let mut members = Vec::new();
        for member in &set.members {
            let member = object(&set.domain, member);
            if !points.contains(&member) {
                return Err(invalid(
                    &context,
                    format!("the member {} is not a point", member.name),
                ));
            }
            members.push(member);
        }
        data_sets.push((name, members));
    }

    let mut transfer_sets = BTreeSet::new();
    for transfer in &config.transfer_sets {
        let context = format!("transfer set {}/{}", transfer.domain, transfer.name);
        domain_known(&transfer.domain, &context)?;
        transfer_sets.insert(object(&transfer.domain, &transfer.name));
    }

    let mut tables = BilateralTables::new();
    let mut policy = ServerPolicy::new();
    for table in &config.tables {
        let context = format!("table {}", table.designation);
        domain_known(&table.domain, &context)?;
        let table_id = VisibleString32::new(&table.table_id)
            .map_err(|e| invalid(&context, format!("the table identifier: {e:?}")))?;
        let mut built = BilateralTable::new(
            table.designation.as_str(),
            Versions {
                table_id,
                tase2: STANDARD_VERSION,
            },
            table.domain.as_str(),
        );
        for reference in &table.application_references {
            built = built.with_application_reference(reference.as_str());
        }
        if table.data_set_creation {
            built = built.with_data_set_creation();
        }
        for access in &table.access {
            let domain = access.domain.as_deref().unwrap_or(&table.domain);
            let object_name = object(domain, &access.name);
            let context = format!("{context}: access to {}", access.name);
            let mut permitted = BTreeSet::new();
            for text in &access.permissions {
                permitted.insert(permission(text, &context)?);
            }
            let objects = ObjectAccess {
                permitted,
                minimum_interval: access.interval_ms.map(Duration::from_millis),
            };
            built = match access.kind {
                AccessKind::DataValue => {
                    if !points.contains(&object_name) {
                        return Err(invalid(&context, "it is not a point"));
                    }
                    built.with_data_value(object_name, objects)
                }
                AccessKind::DataSet => {
                    if !data_set_names.contains(&object_name) {
                        return Err(invalid(&context, "it is not a data set"));
                    }
                    built.with_data_set(object_name, objects)
                }
                AccessKind::TransferSet => {
                    if !transfer_sets.contains(&object_name) {
                        return Err(invalid(&context, "it is not a transfer set"));
                    }
                    built.with_transfer_set(object_name, objects)
                }
                AccessKind::Device => {
                    if !device_names.contains(&object_name) {
                        return Err(invalid(&context, "it is not a device"));
                    }
                    let check_back_id = access
                        .check_back_id
                        .ok_or_else(|| invalid(&context, "a device needs its check-back-id"))?;
                    let timeout_s = access
                        .timeout_s
                        .ok_or_else(|| invalid(&context, "a device needs its timeout-s"))?;
                    let check_back_id = SBOCheckBackName::new(check_back_id)
                        .map_err(|e| invalid(&context, format!("the check-back-id: {e:?}")))?;
                    built.with_device(
                        object_name,
                        DeviceAccess {
                            permitted: objects.permitted,
                            check_back_id,
                            timeout: Duration::from_secs(timeout_s),
                        },
                    )
                }
            };
        }
        if table.initiator {
            policy = policy.with_initiator_for(table.designation.as_str());
        }
        tables.insert(built);
    }

    let mut simulations = Vec::new();
    for section in &config.simulations {
        let context = format!("simulation of {}/{}", section.domain, section.name);
        domain_known(&section.domain, &context)?;
        let name = object(&section.domain, &section.name);
        let point = config
            .points
            .iter()
            .find(|point| point.domain == section.domain && point.name == section.name)
            .ok_or_else(|| invalid(&context, "it is not a point"))?;
        if section.period_ms == 0 {
            return Err(invalid(&context, "the period is zero"));
        }
        if !point.writable {
            // The change is made through the store, as the application's own write: a read-only point refuses it.
            return Err(invalid(
                &context,
                "the point is not writable, so the simulation cannot change it",
            ));
        }
        match (section.kind, point.kind) {
            (SimulationKind::Toggle, PointKind::Boolean) => {}
            (SimulationKind::Ramp, PointKind::Integer | PointKind::Unsigned | PointKind::Float) => {
                if section.step.is_nan() || section.step <= 0.0 {
                    return Err(invalid(&context, "a ramp needs a step above zero"));
                }
                if section.min.is_nan() || section.max.is_nan() || section.min >= section.max {
                    return Err(invalid(&context, "a ramp needs min below max"));
                }
            }
            (SimulationKind::Sine, PointKind::Float) => {
                if section.min.is_nan() || section.max.is_nan() || section.min >= section.max {
                    return Err(invalid(&context, "a sine needs min below max"));
                }
            }
            _ => {
                return Err(invalid(
                    &context,
                    "the change does not suit the type of the point",
                ))
            }
        }
        simulations.push(Simulation {
            name,
            kind: section.kind,
            period: Duration::from_millis(section.period_ms),
            min: section.min,
            max: section.max,
            step: section.step,
            point: point.kind,
        });
    }

    let server = TaseServerConfig {
        mms: ServerConfig {
            poll_interval: Duration::from_millis(config.server.poll_ms.max(1)),
            max_associations,
            capabilities: igs_iec_tase2::mms_capabilities(),
            ..ServerConfig::new(LinkConfig::default())
        },
        tables,
        policy,
        transfer_sets,
        data_sets,
        data_set_capacity: config.server.data_set_capacity,
        devices: device_configs,
        transfer: Policy {
            expected_send_delay: Duration::from_millis(config.server.expected_send_delay_ms),
            ack_timeout: Duration::from_secs(config.server.ack_timeout_s),
        },
    };
    let mut information = Vec::new();
    for section in &config.information {
        let context = format!("information {}", section.info_reference);
        if section.period_ms == 0 {
            return Err(invalid(&context, "the period is zero"));
        }
        // The message is built once to check its references and its stream (the MessageId is only a value here).
        igs_iec_tase2_link::information::InformationMessage::new(
            section.info_reference,
            section.local_reference,
            1,
            section.stream.clone().into_bytes(),
        )
        .map_err(|e| invalid(&context, format!("{e:?}")))?;
        information.push(InformationSource {
            info_reference: section.info_reference,
            local_reference: section.local_reference,
            stream: section.stream.clone().into_bytes(),
            period: Duration::from_millis(section.period_ms),
        });
    }

    Ok(Built {
        vmd,
        server,
        simulations,
        information,
    })
}

/// Parses the text of a configuration file.
pub fn parse(text: &str) -> Result<Config, ConfigError> {
    toml::from_str(text).map_err(|e| ConfigError(e.to_string()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    /// A configuration with one domain and the given sections appended.
    fn text(sections: &str) -> String {
        format!("[[domain]]\nname = \"ICC1\"\n{sections}")
    }

    fn build_text(sections: &str) -> Result<Built, ConfigError> {
        let config = parse(&text(sections))?;
        build(&config, 64)
    }

    const POINT: &str = r#"[[point]]
domain = "ICC1"
name = "Flow1"
type = "integer"
bits = 16
value = 5
writable = true
"#;

    #[test]
    fn an_unknown_key_is_refused_with_its_line() {
        let error = parse(
            r#"[[point]]
domain = "ICC1"
name = "A"
type = "integer"
value = 1
colour = 2
"#,
        )
        .unwrap_err();
        assert!(error.0.contains("colour"), "{error}");
    }

    #[test]
    fn an_integer_outside_its_bits_is_refused_with_the_point_named() {
        let error = build_text(
            r#"[[point]]
domain = "ICC1"
name = "Big"
type = "integer"
bits = 8
value = 200
"#,
        )
        .err()
        .unwrap();
        assert!(
            error.0.contains("ICC1/Big") && error.0.contains("does not fit"),
            "{error}"
        );
    }

    #[test]
    fn a_negative_unsigned_is_refused() {
        let error = build_text(
            r#"[[point]]
domain = "ICC1"
name = "U"
type = "unsigned"
value = -1
"#,
        )
        .err()
        .unwrap();
        assert!(error.0.contains("does not fit"), "{error}");
    }

    #[test]
    fn an_information_message_source_builds_with_its_references_and_stream() {
        let built = build_text(
            r#"[[information]]
info_reference = 7
local_reference = 1
stream = "demo"
period_ms = 3000
"#,
        )
        .unwrap();
        assert_eq!(built.information.len(), 1);
        assert_eq!(built.information[0].info_reference, 7);
        assert_eq!(built.information[0].stream, b"demo".to_vec());
    }

    #[test]
    fn an_information_message_source_with_a_zero_period_is_refused() {
        let error = build_text(
            r#"[[information]]
info_reference = 7
local_reference = 1
stream = "demo"
period_ms = 0
"#,
        )
        .err()
        .unwrap();
        assert!(error.0.contains("the period is zero"), "{error}");
    }

    #[test]
    fn a_device_with_its_check_back_id_and_timeout_builds() {
        let built = build_text(
            r#"[[device]]
domain = "ICC1"
name = "Breaker12"
class = "sbo"
closing_commands = [2]

[[table]]
designation = "1.3.9999.1"
table_id = "BT-1"
domain = "ICC1"
application_references = ["1.3.9999.1"]

[[table.access]]
kind = "device"
name = "Breaker12"
permissions = ["visibility", "select", "operate", "set-tag"]
check_back_id = 12
timeout_s = 5
"#,
        )
        .unwrap();
        assert_eq!(built.server.devices.len(), 1);
        assert_eq!(built.server.devices[0].class, DeviceClass::Sbo);
        assert_eq!(built.server.devices[0].closing_commands, vec![2]);
    }

    #[test]
    fn a_device_access_without_its_check_back_id_is_refused() {
        let error = build_text(
            r#"[[device]]
domain = "ICC1"
name = "Valve9"
class = "non-sbo"

[[table]]
designation = "1.3.9999.1"
table_id = "BT-1"
domain = "ICC1"
application_references = ["1.3.9999.1"]

[[table.access]]
kind = "device"
name = "Valve9"
permissions = ["visibility", "operate"]
timeout_s = 5
"#,
        )
        .err()
        .unwrap();
        assert!(error.0.contains("needs its check-back-id"), "{error}");
    }

    #[test]
    fn a_point_in_an_undeclared_domain_is_refused() {
        let config = parse(
            r#"[[point]]
domain = "Other"
name = "A"
type = "boolean"
value = true
"#,
        )
        .unwrap();
        let error = build(&config, 64).err().unwrap();
        assert!(error.0.contains("\"Other\" is not declared"), "{error}");
    }

    #[test]
    fn a_data_set_member_that_is_not_a_point_is_refused() {
        let error = build_text(&format!(
            "{POINT}[[data-set]]\ndomain = \"ICC1\"\nname = \"DS\"\nmembers = [\"Nope\"]\n"
        ))
        .err()
        .unwrap();
        assert!(error.0.contains("Nope"), "{error}");
    }

    #[test]
    fn a_table_access_to_an_object_that_is_not_declared_is_refused() {
        let error = build_text(&format!(
            r#"{POINT}[[table]]
designation = "1.3.9999.1"
table_id = "BT-1"
domain = "ICC1"
application_references = ["1.3.9999.1"]
[[table.access]]
kind = "data-set"
name = "Flow1"
permissions = ["visibility"]
"#
        ))
        .err()
        .unwrap();
        assert!(error.0.contains("it is not a data set"), "{error}");
    }

    #[test]
    fn an_unknown_permission_is_refused() {
        let error = build_text(&format!(
            r#"{POINT}[[table]]
designation = "1.3.9999.1"
table_id = "BT-1"
domain = "ICC1"
application_references = []
[[table.access]]
kind = "data-value"
name = "Flow1"
permissions = ["fly"]
"#
        ))
        .err()
        .unwrap();
        assert!(error.0.contains("\"fly\" is not known"), "{error}");
    }

    #[test]
    fn a_table_identifier_longer_than_32_characters_is_refused() {
        let error = build_text(&format!(
            "[[table]]\ndesignation = \"1.3.9999.1\"\ntable_id = \"{}\"\ndomain = \"ICC1\"\napplication_references = []\n",
            "x".repeat(33)
        ))
        .err()
        .unwrap();
        assert!(error.0.contains("table identifier"), "{error}");
    }

    #[test]
    fn a_simulation_of_a_read_only_point_is_refused() {
        let error = build_text(
            r#"[[point]]
domain = "ICC1"
name = "Fixed"
type = "boolean"
value = true
[[simulate]]
domain = "ICC1"
name = "Fixed"
kind = "toggle"
period_ms = 100
"#,
        )
        .err()
        .unwrap();
        assert!(error.0.contains("not writable"), "{error}");
    }

    #[test]
    fn a_change_that_does_not_suit_the_type_is_refused() {
        let error = build_text(
            r#"[[point]]
domain = "ICC1"
name = "Flag"
type = "boolean"
value = true
writable = true
[[simulate]]
domain = "ICC1"
name = "Flag"
kind = "ramp"
period_ms = 100
min = 0
max = 3
step = 1
"#,
        )
        .err()
        .unwrap();
        assert!(error.0.contains("does not suit the type"), "{error}");
    }

    #[test]
    fn a_valid_file_builds_the_points_the_tables_and_the_simulations() {
        let built = build_text(&format!(
            r#"{POINT}[[point]]
domain = "ICC1"
name = "Ratio"
type = "float"
value = 0.5
writable = true
[[data-set]]
domain = "ICC1"
name = "DS1"
members = ["Flow1", "Ratio"]
[[table]]
designation = "1.3.9999.1"
table_id = "BT-1"
domain = "ICC1"
application_references = ["1.3.9999.1"]
[[table.access]]
kind = "data-value"
name = "Flow1"
permissions = ["visibility", "get-data-value"]
interval_ms = 500
[[simulate]]
domain = "ICC1"
name = "Flow1"
kind = "ramp"
period_ms = 1000
min = 0
max = 10
step = 1
"#
        ))
        .unwrap();
        assert_eq!(built.server.data_sets.len(), 1);
        assert_eq!(built.simulations.len(), 1);
        assert_eq!(built.simulations[0].point, PointKind::Integer);
    }
}
