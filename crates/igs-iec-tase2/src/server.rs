// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The TASE.2 server (task S1): the MMS server of `igs-iso9506-mms` with a model that serves the TASE.2 services. Each
//! request is checked against the Bilateral Table of the client that makes it, the data values live in the VMD of the
//! MMS stack, and each data set transfer set reports to the association that started it.
//!
//! The model is the only owner of the state: the tables, the data sets, the data values and the transfer sets. The
//! application changes a data value through [`TaseServer::set_value`]; the change is sent by the model at its next poll,
//! and the transfer sets that monitor the value report it.
//!
//! Not served by S1: the GetNameList service (503 §7.2.3.4, §7.2.4.6), which needs a per-table listing; the blocked
//! transfer reports (BlockData); and the acknowledgement of a critical report, which the MMS stack does not send from the
//! server side as a request (the server signals the data failure after the ack timeout, and no Data_failure event).

use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use igs_iec_tase2_link::association::{server_on_initiate, Decision, ServerPolicy};
use igs_iec_tase2_link::bilateral::{
    Access, BilateralTable, BilateralTables, Object, ObjectName, Permission, Scope,
};
use igs_iec_tase2_link::data::{ErrorCode, Item, Store, Vcc};
use igs_iec_tase2_link::transfer::{DataSetTransferSet, Parameters, Payload, Policy, Report};
use igs_iec_tase2_link::version::STANDARD_VERSION;
use igs_iec_tase2_model::builtin::Integer16;
use igs_iec_tase2_model::generated::iso503::{DSConditions, DSTransferSet, SupportedFeatures};
use igs_iec_tase2_model::mms::MmsValue;
use igs_iso9506_mms::server::{Model, Outbound, Peer, PeerId, Server, ServerConfig};
use igs_iso9506_mms::vmd::{Name, Target, Vmd};
use igs_iso9506_mms_codec::generated::types::ObjectName as MmsObjectName;
use igs_iso9506_mms_codec::generated::types::{
    AEQualifier, AEQualifierForm2, APTitle, APTitleForm2, AccessResult, ConfirmedServiceRequest,
    ConfirmedServiceResponse, Data, DataAccessError, DefineNamedVariableListResponse,
    DeleteNamedVariableListResponse, GetNamedVariableListAttributesResponse,
    GetNamedVariableListAttributesResponseListOfVariableItem, GetVariableAccessAttributesRequest,
    GetVariableAccessAttributesResponse, InformationReport, ReadResponse, ServiceError,
    ServiceErrorErrorClass, ServiceErrorErrorClassAccess, ServiceErrorErrorClassDefinition,
    ServiceErrorErrorClassResource, ServiceErrorErrorClassService, TypeDescription,
    UnconfirmedService, Unsigned32, VariableAccessSpecification,
    VariableAccessSpecificationListOfVariableItem, VariableSpecification, WriteResponse,
    WriteResponseItemItem,
};
use igs_iso9506_mms_link::association::Calling;
use tokio::sync::mpsc;

use crate::error::Error;
use crate::names::{from_mms, to_mms, variable};

/// The number of changes of data values that wait for the model.
const UPDATE_CAPACITY: usize = 64;

/// The configuration of a TASE.2 server.
#[derive(Debug, Clone)]
pub struct TaseServerConfig {
    /// The MMS configuration: the link parameters, the capabilities, the limit of associations, the poll interval.
    pub mms: ServerConfig,
    /// The Bilateral Tables, one per remote control centre.
    pub tables: BilateralTables,
    /// The agreements of the server that the tables do not hold (the designated initiators, 503 §7.2.2.2.1).
    pub policy: ServerPolicy,
    /// The transfer set objects of the server: the names a Start or Stop Transfer may write.
    pub transfer_sets: BTreeSet<ObjectName>,
    /// The data sets that the server holds at its start, with their members.
    pub data_sets: Vec<(ObjectName, Vec<ObjectName>)>,
    /// The number of data sets the server can hold.
    pub data_set_capacity: usize,
    /// The expected time to send a report, and the time to wait for the acknowledgement of a critical one.
    pub transfer: Policy,
}

/// A running TASE.2 server.
pub struct TaseServer {
    mms: Server,
    updates: mpsc::Sender<(ObjectName, Data)>,
}

impl TaseServer {
    /// Listens on `address` and serves the TASE.2 services, with the data values held by `vmd`. Each data value is a
    /// variable of `vmd`: a writable one can be written by a client that the Bilateral Table allows.
    pub async fn bind(address: &str, config: TaseServerConfig, vmd: Vmd) -> Result<Self, Error> {
        let (updates, received) = mpsc::channel(UPDATE_CAPACITY);
        let model = TaseModel::new(config.clone(), vmd, received);
        let mms = Server::bind_model(address, config.mms, model).await?;
        Ok(Self { mms, updates })
    }

    /// The address the server listens on.
    pub fn local_address(&self) -> SocketAddr {
        self.mms.local_address()
    }

    /// Changes the value of a data value of the server, as the application observes it. The transfer sets that monitor
    /// it report the change at the next poll of the model.
    pub async fn set_value(&self, name: ObjectName, value: Data) -> Result<(), Error> {
        self.updates
            .send((name, value))
            .await
            .map_err(|_| Error::Malformed("the server has stopped"))
    }

    /// Stops listening and ends the associations.
    pub async fn shutdown(self) {
        self.mms.shutdown().await;
    }
}

/// The MMS error of a TASE.2 error code.
fn service_error(code: ErrorCode) -> ServiceError {
    let class = match code {
        ErrorCode::ObjectNonExistent => {
            ServiceErrorErrorClass::Access(ServiceErrorErrorClassAccess(2))
        }
        ErrorCode::ObjectAccessDenied => {
            ServiceErrorErrorClass::Access(ServiceErrorErrorClassAccess(3))
        }
        ErrorCode::ObjectAccessUnsupported => {
            ServiceErrorErrorClass::Access(ServiceErrorErrorClassAccess(1))
        }
        ErrorCode::ObjectExists => {
            ServiceErrorErrorClass::Definition(ServiceErrorErrorClassDefinition(5))
        }
        ErrorCode::ObjectUndefined => {
            ServiceErrorErrorClass::Definition(ServiceErrorErrorClassDefinition(1))
        }
        ErrorCode::ObjectAttributeInconsistent => {
            ServiceErrorErrorClass::Definition(ServiceErrorErrorClassDefinition(6))
        }
        ErrorCode::MemoryUnavailable => {
            ServiceErrorErrorClass::Resource(ServiceErrorErrorClassResource(1))
        }
        _ => ServiceErrorErrorClass::Access(ServiceErrorErrorClassAccess(0)),
    };
    ServiceError {
        error_class: class,
        additional_code: None,
        additional_description: None,
        service_specific_info: None,
    }
}

/// The MMS error of a service that this server does not serve.
fn unsupported() -> ServiceError {
    ServiceError {
        error_class: ServiceErrorErrorClass::Service(ServiceErrorErrorClassService(0)),
        additional_code: None,
        additional_description: None,
        service_specific_info: None,
    }
}

/// The MMS data access error of a TASE.2 error code, for an item of a response.
fn data_error(code: ErrorCode) -> DataAccessError {
    match code {
        ErrorCode::ObjectNonExistent => DataAccessError::OBJECT_NON_EXISTENT,
        ErrorCode::ObjectAccessDenied => DataAccessError::OBJECT_ACCESS_DENIED,
        ErrorCode::ObjectAccessUnsupported => DataAccessError::OBJECT_ACCESS_UNSUPPORTED,
        ErrorCode::ObjectUndefined => DataAccessError::OBJECT_UNDEFINED,
        ErrorCode::ObjectAttributeInconsistent | ErrorCode::ObjectAttributeUnsupported => {
            DataAccessError::OBJECT_ATTRIBUTE_INCONSISTENT
        }
        ErrorCode::HardwareFault => DataAccessError::HARDWARE_FAULT,
        ErrorCode::TypeInconsistent => DataAccessError::TYPE_INCONSISTENT,
        ErrorCode::ObjectExists | ErrorCode::MemoryUnavailable => {
            DataAccessError::OBJECT_VALUE_INVALID
        }
        ErrorCode::Other(value) => DataAccessError(value),
    }
}

/// The MMS error of a data access error, as the stack's store gives it.
fn code_of_data_error(error: DataAccessError) -> ErrorCode {
    if error == DataAccessError::OBJECT_NON_EXISTENT {
        ErrorCode::ObjectNonExistent
    } else if error == DataAccessError::OBJECT_ACCESS_DENIED {
        ErrorCode::ObjectAccessDenied
    } else if error == DataAccessError::TYPE_INCONSISTENT {
        ErrorCode::TypeInconsistent
    } else if error == DataAccessError::HARDWARE_FAULT {
        ErrorCode::HardwareFault
    } else {
        ErrorCode::Other(error.0)
    }
}

/// The MMS object name of a TASE.2 name, as the VMD of the stack names it.
fn target(name: &ObjectName) -> Option<Target> {
    match &name.scope {
        Scope::Vmd => Some(Target::vmd(Name::new(&name.name).ok()?)),
        Scope::Domain(domain) => Some(Target::in_domain(
            Name::new(domain).ok()?,
            Name::new(&name.name).ok()?,
        )),
    }
}

/// The data values of the server, held by the VMD of the MMS stack.
pub struct VmdStore {
    vmd: Vmd,
}

impl Store for VmdStore {
    fn read(&self, name: &ObjectName) -> Option<Data> {
        target(name).and_then(|target| self.vmd.read(&target).ok())
    }

    fn write(&mut self, name: &ObjectName, value: Data) -> Result<(), ErrorCode> {
        let target = target(name).ok_or(ErrorCode::ObjectNonExistent)?;
        self.vmd.write(&target, value).map_err(code_of_data_error)
    }

    fn type_of(&self, name: &ObjectName) -> Option<TypeDescription> {
        target(name).and_then(|target| self.vmd.type_description(&target).ok().cloned())
    }
}

/// The TASE.2 name of an MMS object name of a request.
fn name_of(name: &MmsObjectName) -> Option<ObjectName> {
    from_mms(name)
}

/// The designation of a client: the dotted form of its AP-title.
fn designation_of(calling: &Calling) -> Option<String> {
    match calling.ap_title.as_ref()? {
        APTitle::ApTitleForm2(APTitleForm2(oid)) => Some(
            oid.0
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join("."),
        ),
        APTitle::ApTitleForm1(_) => None,
    }
}

/// The application reference of a client: its AE-title, written as its designation followed by `/` and the AE qualifier
/// when the AARQ gives one (503 §6.3.4). An AE-title without a qualifier is the AP-title alone.
fn application_reference_of(calling: &Calling, designation: &str) -> Option<String> {
    match calling.ae_qualifier.as_ref() {
        None => Some(designation.to_string()),
        Some(AEQualifier::AeQualifierForm2(AEQualifierForm2(value))) => {
            Some(format!("{designation}/{value}"))
        }
        Some(AEQualifier::AeQualifierForm1(_)) => None,
    }
}

/// The time now, as the seconds and milliseconds since the epoch of the GMT time (503 §8.2.4).
fn clock() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
}

/// A data set transfer set in monitoring, with what its reports need.
struct Monitored {
    set: DataSetTransferSet,
    data_set: ObjectName,
    rbe: bool,
}

/// One association: the designation of its client, and its transfer sets.
struct Session {
    designation: String,
    transfer_sets: BTreeMap<ObjectName, Monitored>,
}

/// The TASE.2 model: the state of the server and the services that it serves.
pub struct TaseModel {
    vcc: Vcc<VmdStore>,
    tables: BilateralTables,
    policy: ServerPolicy,
    transfer_sets: BTreeSet<ObjectName>,
    transfer: Policy,
    sessions: BTreeMap<PeerId, Session>,
    updates: mpsc::Receiver<(ObjectName, Data)>,
}

impl TaseModel {
    fn new(
        config: TaseServerConfig,
        vmd: Vmd,
        updates: mpsc::Receiver<(ObjectName, Data)>,
    ) -> Self {
        let mut vcc = Vcc::new(VmdStore { vmd }, config.data_set_capacity);
        for (name, members) in config.data_sets {
            vcc.load_data_set(name, members);
        }
        Self {
            vcc,
            tables: config.tables,
            policy: config.policy,
            transfer_sets: config.transfer_sets,
            transfer: config.transfer,
            sessions: BTreeMap::new(),
            updates,
        }
    }

    /// The Bilateral Table of an association, and the session of it.
    fn table_of(&self, id: PeerId) -> Option<&BilateralTable> {
        let session = self.sessions.get(&id)?;
        self.tables.table(&session.designation)
    }

    /// The names of the data sets that an active transfer set references.
    fn data_sets_in_use(&self) -> BTreeSet<ObjectName> {
        self.sessions
            .values()
            .flat_map(|session| {
                session
                    .transfer_sets
                    .values()
                    .map(|monitored| monitored.data_set.clone())
            })
            .collect()
    }

    /// The value of a special object of the server: the Bilateral Table identifier of the client's domain, the TASE.2
    /// version, and the supported features (503 §8.3.2 to §8.3.4).
    fn special(&self, name: &ObjectName, table: &BilateralTable) -> Option<Data> {
        match (&name.scope, name.name.as_str()) {
            (Scope::Domain(domain), "Bilateral_Table_ID") if domain.as_str() == table.domain() => {
                Some(Data::VisibleString(
                    table.versions().table_id.as_str().to_string(),
                ))
            }
            (Scope::Vmd, "TASE2_Version") => Some(Data::Structure(vec![
                Data::Integer(i128::from(STANDARD_VERSION.major)),
                Data::Integer(i128::from(STANDARD_VERSION.minor)),
            ])),
            (Scope::Vmd, "Supported_Features") => Some(SupportedFeatures(1).to_mms()),
            _ => None,
        }
    }

    /// The read of the names of a list (Read with a list of variables).
    fn read_list(&self, table: &BilateralTable, names: &[ObjectName]) -> Vec<AccessResult> {
        names
            .iter()
            .map(|name| match self.special(name, table) {
                Some(value) => AccessResult::Success(value),
                None => match self
                    .vcc
                    .get_data_values(table, std::slice::from_ref(name))
                    .into_iter()
                    .next()
                {
                    Some(Item::Success(value)) => AccessResult::Success(value),
                    Some(Item::Failure(code)) => AccessResult::Failure(data_error(code)),
                    None => AccessResult::Failure(DataAccessError::OBJECT_NON_EXISTENT),
                },
            })
            .collect()
    }

    /// Starts or stops a transfer set that the client writes (503 §7.2.5.2, §7.2.5.3).
    fn write_transfer_set(
        &mut self,
        id: PeerId,
        table: &BilateralTable,
        name: &ObjectName,
        value: Data,
    ) -> Item<()> {
        if !self.transfer_sets.contains(name) {
            return Item::Failure(ErrorCode::ObjectNonExistent);
        }
        let Ok(set) = DSTransferSet::from_mms(&value) else {
            return Item::Failure(ErrorCode::TypeInconsistent);
        };
        if !set.status.0 {
            // Stop Transfer: the monitoring of this set ends, and its name returns to the pool.
            let stopped = self
                .sessions
                .get_mut(&id)
                .and_then(|session| session.transfer_sets.remove(name));
            return match stopped {
                Some(mut monitored) => {
                    monitored.set.stop();
                    Item::Success(())
                }
                None => Item::Failure(ErrorCode::ObjectNonExistent),
            };
        }
        match self.start_monitoring(table, name, &set) {
            Ok(monitored) => {
                if let Some(session) = self.sessions.get_mut(&id) {
                    session.transfer_sets.insert(name.clone(), monitored);
                }
                Item::Success(())
            }
            Err(code) => Item::Failure(code),
        }
    }

    /// The checks of the Start Transfer, then the monitoring of the transfer set (503 §7.2.5.2.3).
    fn start_monitoring(
        &self,
        table: &BilateralTable,
        name: &ObjectName,
        set: &DSTransferSet,
    ) -> Result<Monitored, ErrorCode> {
        let object = Object::TransferSet(name.clone());
        if table.check(&object, Permission::StartTransfer) != Access::Granted {
            return Err(ErrorCode::ObjectAccessDenied);
        }
        let data_set = name_of(&MmsObjectName::DomainSpecific(
            igs_iso9506_mms_codec::generated::types::ObjectNameDomainSpecific {
                domain_id: igs_iso9506_mms_codec::generated::types::Identifier(
                    set.data_set_name.domain_name.as_str().to_string(),
                ),
                item_id: igs_iso9506_mms_codec::generated::types::Identifier(
                    set.data_set_name.name.as_str().to_string(),
                ),
            },
        ))
        .ok_or(ErrorCode::ObjectNonExistent)?;
        if table.check(&Object::DataSet(data_set.clone()), Permission::Visibility)
            != Access::Granted
        {
            return Err(ErrorCode::ObjectNonExistent);
        }
        let members = self
            .vcc
            .members(&data_set)
            .ok_or(ErrorCode::ObjectNonExistent)?
            .to_vec();
        let conditions = set.ds_conditions_requested;
        let interval = Duration::from_secs(u64::try_from(set.interval.get()).unwrap_or(0));
        let mut snapshot = BTreeMap::new();
        for member in &members {
            if igs_iec_tase2_link::data::SPECIAL_MEMBERS.contains(&member.name.as_str()) {
                continue;
            }
            if table.check(&Object::DataValue(member.clone()), Permission::GetDataValue)
                != Access::Granted
            {
                return Err(ErrorCode::ObjectAccessDenied);
            }
            if conditions.0 & DSConditions::INTERVAL_TIME_OUT != 0
                && table.check_interval(&Object::DataValue(member.clone()), interval)
                    != Access::Granted
            {
                return Err(ErrorCode::ObjectAttributeInconsistent);
            }
            if let Some(value) = self.vcc.store().read(member) {
                snapshot.insert(member.clone(), value);
            }
        }
        if set.block_data.0 {
            // The blocked report is not served by S1.
            return Err(ErrorCode::ObjectAttributeUnsupported);
        }
        let parameters = Parameters {
            start_time: Duration::from_secs(u64::try_from(set.start_time.get()).unwrap_or(0)),
            interval,
            tle: Duration::from_secs(u64::try_from(set.tle.get()).unwrap_or(0)),
            buffer_time: Duration::from_secs(u64::try_from(set.buffer_time.get()).unwrap_or(0)),
            integrity_check: Duration::from_secs(
                u64::try_from(set.integrity_check.get()).unwrap_or(0),
            ),
            conditions,
            block_data: false,
            critical: set.critical.0,
            rbe: set.rbe.0,
            all_changes_reported: set.all_changes_reported.0,
            event_code_requested: Integer16::new(set.event_code_requested.get())
                .map_err(|_| ErrorCode::ObjectAttributeInconsistent)?,
        };
        let started =
            DataSetTransferSet::start(parameters, self.transfer, members, snapshot, clock())?;
        Ok(Monitored {
            set: started,
            data_set,
            rbe: set.rbe.0,
        })
    }

    /// The report of a monitored transfer set, as an MMS Information Report (503 §7.2.5.5.3).
    fn report_pdu(monitored: &Monitored, report: &Report) -> Option<UnconfirmedService> {
        let Payload::Named(values) = &report.payload else {
            return None;
        };
        let access = if monitored.rbe {
            VariableAccessSpecification::ListOfVariable(
                values
                    .iter()
                    .map(|(name, _)| VariableAccessSpecificationListOfVariableItem {
                        variable_specification: variable(name),
                        alternate_access: None,
                    })
                    .collect(),
            )
        } else {
            VariableAccessSpecification::VariableListName(to_mms(&monitored.data_set))
        };
        Some(UnconfirmedService::InformationReport(InformationReport {
            variable_access_specification: access,
            list_of_access_result: values
                .iter()
                .map(|(_, value)| AccessResult::Success(value.clone()))
                .collect(),
        }))
    }
}

impl Model for TaseModel {
    fn admit(&mut self, peer: &Peer) -> bool {
        let Some(designation) = designation_of(&peer.calling) else {
            return false;
        };
        let Some(reference) = application_reference_of(&peer.calling, &designation) else {
            return false;
        };
        match server_on_initiate(&self.tables, &self.policy, &designation, &reference) {
            Decision::Accept => {
                self.sessions.insert(
                    peer.id,
                    Session {
                        designation,
                        transfer_sets: BTreeMap::new(),
                    },
                );
                true
            }
            Decision::Refuse(_) => false,
        }
    }

    fn dispatch(
        &mut self,
        peer: &Peer,
        service: ConfirmedServiceRequest,
    ) -> Result<ConfirmedServiceResponse, ServiceError> {
        if let ConfirmedServiceRequest::Identify(_) = service {
            // The identity of the VMD: the stack answers it.
            return <Vmd as Model>::dispatch(&mut self.vcc.store_mut().vmd, peer, service);
        }
        let Some(table) = self.table_of(peer.id) else {
            return Err(service_error(ErrorCode::ObjectAccessDenied));
        };
        let table = table.clone();
        match service {
            ConfirmedServiceRequest::Read(request) => match request.variable_access_specification {
                VariableAccessSpecification::ListOfVariable(items) => {
                    let names: Vec<ObjectName> = items
                        .iter()
                        .filter_map(|item| match &item.variable_specification {
                            VariableSpecification::Name(name) => name_of(name),
                            _ => None,
                        })
                        .collect();
                    Ok(ConfirmedServiceResponse::Read(ReadResponse {
                        variable_access_specification: None,
                        list_of_access_result: self.read_list(&table, &names),
                    }))
                }
                VariableAccessSpecification::VariableListName(name) => {
                    let name = name_of(&name)
                        .ok_or_else(|| service_error(ErrorCode::ObjectNonExistent))?;
                    let items = self
                        .vcc
                        .get_data_set_element_values(&table, &name)
                        .map_err(service_error)?;
                    Ok(ConfirmedServiceResponse::Read(ReadResponse {
                        variable_access_specification: None,
                        list_of_access_result: items
                            .into_iter()
                            .map(|(_, item)| match item {
                                Item::Success(value) => AccessResult::Success(value),
                                Item::Failure(code) => AccessResult::Failure(data_error(code)),
                            })
                            .collect(),
                    }))
                }
            },
            ConfirmedServiceRequest::Write(request) => {
                let items = match request.variable_access_specification {
                    VariableAccessSpecification::ListOfVariable(items) => {
                        let names: Vec<ObjectName> = items
                            .iter()
                            .filter_map(|item| match &item.variable_specification {
                                VariableSpecification::Name(name) => name_of(name),
                                _ => None,
                            })
                            .collect();
                        let mut answers = Vec::with_capacity(names.len());
                        for (name, value) in names.into_iter().zip(request.list_of_data) {
                            let answer = if self.transfer_sets.contains(&name) {
                                self.write_transfer_set(peer.id, &table, &name, value)
                            } else {
                                self.vcc
                                    .set_data_values(&table, vec![(name, value)])
                                    .into_iter()
                                    .next()
                                    .unwrap_or(Item::Failure(ErrorCode::ObjectNonExistent))
                            };
                            answers.push(answer);
                        }
                        answers
                    }
                    VariableAccessSpecification::VariableListName(name) => {
                        let name = name_of(&name)
                            .ok_or_else(|| service_error(ErrorCode::ObjectNonExistent))?;
                        self.vcc
                            .set_data_set_element_values(&table, &name, request.list_of_data)
                            .map_err(service_error)?
                    }
                };
                Ok(ConfirmedServiceResponse::Write(WriteResponse(
                    items
                        .into_iter()
                        .map(|item| match item {
                            Item::Success(()) => WriteResponseItemItem::Success,
                            Item::Failure(code) => WriteResponseItemItem::Failure(data_error(code)),
                        })
                        .collect(),
                )))
            }
            ConfirmedServiceRequest::GetVariableAccessAttributes(request) => {
                let GetVariableAccessAttributesRequest::Name(object) = request else {
                    return Err(unsupported());
                };
                let name =
                    name_of(&object).ok_or_else(|| service_error(ErrorCode::ObjectNonExistent))?;
                let ty = self
                    .vcc
                    .get_data_value_type(&table, &name)
                    .map_err(service_error)?;
                Ok(ConfirmedServiceResponse::GetVariableAccessAttributes(
                    GetVariableAccessAttributesResponse {
                        mms_deletable: false,
                        address: None,
                        type_description: ty,
                        access_control_list: None,
                        meaning: None,
                    },
                ))
            }
            ConfirmedServiceRequest::DefineNamedVariableList(request) => {
                let name = name_of(&request.variable_list_name)
                    .ok_or_else(|| service_error(ErrorCode::ObjectAttributeInconsistent))?;
                let mut members = Vec::with_capacity(request.list_of_variable.len());
                for item in &request.list_of_variable {
                    let VariableSpecification::Name(member) = &item.variable_specification else {
                        return Err(service_error(ErrorCode::ObjectAttributeInconsistent));
                    };
                    members
                        .push(name_of(member).ok_or_else(|| {
                            service_error(ErrorCode::ObjectAttributeInconsistent)
                        })?);
                }
                self.vcc
                    .create_data_set(&table, name, members)
                    .map_err(service_error)?;
                Ok(ConfirmedServiceResponse::DefineNamedVariableList(
                    DefineNamedVariableListResponse(()),
                ))
            }
            ConfirmedServiceRequest::GetNamedVariableListAttributes(request) => {
                let name = name_of(&request.0)
                    .ok_or_else(|| service_error(ErrorCode::ObjectNonExistent))?;
                if table.check(&Object::DataSet(name.clone()), Permission::Visibility)
                    != Access::Granted
                {
                    return Err(service_error(ErrorCode::ObjectNonExistent));
                }
                let members = self
                    .vcc
                    .members(&name)
                    .ok_or_else(|| service_error(ErrorCode::ObjectNonExistent))?;
                Ok(ConfirmedServiceResponse::GetNamedVariableListAttributes(
                    GetNamedVariableListAttributesResponse {
                        mms_deletable: false,
                        list_of_variable: members
                            .iter()
                            .map(|member| {
                                GetNamedVariableListAttributesResponseListOfVariableItem {
                                    variable_specification: variable(member),
                                    alternate_access: None,
                                }
                            })
                            .collect(),
                        access_control_list: None,
                    },
                ))
            }
            ConfirmedServiceRequest::DeleteNamedVariableList(request) => {
                let names: Vec<ObjectName> = request
                    .list_of_variable_list_name
                    .unwrap_or_default()
                    .iter()
                    .filter_map(name_of)
                    .collect();
                let in_use = self.data_sets_in_use();
                let outcomes = self.vcc.delete_data_sets(&table, &names, &in_use);
                let matched = outcomes.len();
                let mut deleted = 0usize;
                for outcome in outcomes {
                    match outcome {
                        Ok(()) => deleted = deleted.saturating_add(1),
                        Err(code) => return Err(service_error(code)),
                    }
                }
                Ok(ConfirmedServiceResponse::DeleteNamedVariableList(
                    DeleteNamedVariableListResponse {
                        number_matched: Unsigned32(i64::try_from(matched).unwrap_or(i64::MAX)),
                        number_deleted: Unsigned32(i64::try_from(deleted).unwrap_or(i64::MAX)),
                    },
                ))
            }
            // GetNameList is not served by S1 (a per-table listing): refused as not supported.
            _ => Err(unsupported()),
        }
    }

    fn released(&mut self, peer: &Peer) {
        // The association releases what it held: its transfer sets are disabled, and their names return to the pool
        // (503 §7.2.2.3.3); dropping the session stops their monitoring.
        self.sessions.remove(&peer.id);
    }

    fn poll(&mut self, _now: Duration) -> Vec<igs_iso9506_mms::server::Outbound> {
        let now = clock();
        let mut outbound = Vec::new();
        // The changes of the application reach the transfer sets that monitor them.
        while let Ok((name, value)) = self.updates.try_recv() {
            // The application's own write: the table does not apply to it. A change the store refuses (a read-only
            // variable, a value of another type) is neither kept nor reported.
            if self.vcc.store_mut().write(&name, value.clone()).is_err() {
                continue;
            }
            for (id, session) in self.sessions.iter_mut() {
                for monitored in session.transfer_sets.values_mut() {
                    for report in monitored.set.change(&name, value.clone(), now) {
                        if let Some(pdu) = Self::report_pdu(monitored, &report) {
                            outbound.push(Outbound {
                                peer: Some(*id),
                                report: pdu,
                            });
                        }
                    }
                }
            }
        }
        for (id, session) in self.sessions.iter_mut() {
            for monitored in session.transfer_sets.values_mut() {
                for report in monitored.set.advance(now) {
                    if let Some(pdu) = Self::report_pdu(monitored, &report) {
                        outbound.push(Outbound {
                            peer: Some(*id),
                            report: pdu,
                        });
                    }
                }
            }
        }
        outbound
    }
}
