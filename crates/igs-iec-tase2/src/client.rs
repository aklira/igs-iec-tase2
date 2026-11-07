// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The TASE.2 client: an association over `igs-iso9506-mms`, and the operations of the data values, the data sets and the
//! transfer sets, each answered as the standard answers it.
//!
//! The events (the connection states and the transfer reports) come through [`TaseEvents`], from the bounded channel of
//! the MMS client (`ClientConfig::event_capacity`). Back-pressure: a channel that the caller does not drain fills; the
//! connection then waits to deliver the next report, which delays the acknowledgement of a critical report, and the
//! server's acknowledgement timeout (503 §7.2.5.5.3) may then raise a data failure. The caller drains the events.

use igs_iec_tase2_link::association::{client_after_initiate, ServerVariables};
use igs_iec_tase2_link::bilateral::{ObjectName, Scope};
use igs_iec_tase2_link::data::{ErrorCode, Item};
use igs_iec_tase2_link::device::{DeviceState, Tag, TagState};
use igs_iec_tase2_link::information::{InformationMessage, HEADER_NAME};
use igs_iec_tase2_link::version::Versions;
use igs_iec_tase2_model::generated::iso503::{
    DSTransferSet, DSTransferSetStatus, SupportedFeatures, TASE2Version, VisibleString32,
};
use igs_iec_tase2_model::generated::iso802::{
    InfoBufXX, InfoMessHeader, SBOCheckBackName, TagFlags, TagValue, TextString,
};
use igs_iec_tase2_model::mms::MmsValue;
use igs_iso9506_mms::client::{Client, ClientConfig, ConnectionState, Event, Events};
use igs_iso9506_mms_codec::generated::types::{
    AccessResult, Data, DataAccessError, DefineNamedVariableListRequest,
    DefineNamedVariableListRequestListOfVariableItem, DeleteNamedVariableListRequest,
    DeleteNamedVariableListRequestScopeOfDelete, GetNamedVariableListAttributesRequest,
    GetVariableAccessAttributesRequest, InformationReport, ReadRequest, TypeDescription,
    UnconfirmedService, VariableAccessSpecification, VariableSpecification, WriteRequest,
    WriteResponseItemItem,
};
use igs_iso9506_mms_codec::generated::types::{
    Identifier, ObjectName as MmsObjectName, VariableAccessSpecificationListOfVariableItem,
};

use crate::error::Error;
use crate::names::{from_mms, list_of_variables, to_mms, variable};

/// The agreement of the client with the server: the domain of the client (the domain of the Bilateral Table's objects)
/// and the versions of the Bilateral Agreement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Agreement {
    /// The domain that the server uses for the client (`Bilateral_Table_ID` is domain-specific to it).
    pub domain: String,
    /// The Bilateral Table identifier and the TASE.2 version that the agreement holds.
    pub versions: Versions,
}

/// An association with a TASE.2 server, after the checks of the standard.
#[derive(Debug)]
pub struct TaseClient {
    mms: Client,
    domain: String,
    features: SupportedFeatures,
}

/// The stream of the events of an association.
#[derive(Debug)]
pub struct TaseEvents {
    events: Events,
}

/// An event of the association.
#[derive(Debug, Clone, PartialEq)]
pub enum TaseEvent {
    /// The state of the connection changed.
    Connection(ConnectionState),
    /// A transfer report (an information report of a data set transfer set).
    Report(ReceivedReport),
    /// An information message: a report of the Information Message Transfer Set (503 §7.2.5.5.4).
    Information(InformationMessage),
}

/// A transfer report, as received: which objects it carries, and their values in the same order.
#[derive(Debug, Clone, PartialEq)]
pub struct ReceivedReport {
    /// The objects that the report carries: the changed members (RBE), or the data set itself.
    pub access: ReportAccess,
    /// The value of each object, or the error of its item.
    pub values: Vec<Item<Data>>,
}

/// The objects a report names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportAccess {
    /// The named variables (the changed members of a report by exception).
    Names(Vec<ObjectName>),
    /// The data set (the entire data set, without RBE).
    DataSet(ObjectName),
}

fn code_of(error: DataAccessError) -> ErrorCode {
    if error == DataAccessError::OBJECT_NON_EXISTENT {
        ErrorCode::ObjectNonExistent
    } else if error == DataAccessError::OBJECT_ACCESS_DENIED {
        ErrorCode::ObjectAccessDenied
    } else if error == DataAccessError::OBJECT_ACCESS_UNSUPPORTED {
        ErrorCode::ObjectAccessUnsupported
    } else if error == DataAccessError::OBJECT_UNDEFINED {
        ErrorCode::ObjectUndefined
    } else if error == DataAccessError::OBJECT_ATTRIBUTE_INCONSISTENT {
        ErrorCode::ObjectAttributeInconsistent
    } else if error == DataAccessError::TEMPORARILY_UNAVAILABLE {
        ErrorCode::TemporarilyUnavailable
    } else if error == DataAccessError::HARDWARE_FAULT {
        ErrorCode::HardwareFault
    } else if error == DataAccessError::TYPE_INCONSISTENT {
        ErrorCode::TypeInconsistent
    } else {
        ErrorCode::Other(error.0)
    }
}

fn item_of(result: &AccessResult) -> Item<Data> {
    match result {
        AccessResult::Success(value) => Item::Success(value.clone()),
        AccessResult::Failure(error) => Item::Failure(code_of(error.clone())),
    }
}

fn write_item_of(result: &WriteResponseItemItem) -> Item<()> {
    match result {
        WriteResponseItemItem::Success => Item::Success(()),
        WriteResponseItemItem::Failure(error) => Item::Failure(code_of(error.clone())),
    }
}

/// The value of one of the variables read after the association.
fn variable_value<T: MmsValue>(result: &AccessResult) -> Result<T, Error> {
    match result {
        AccessResult::Success(value) => Ok(T::from_mms(value)?),
        AccessResult::Failure(error) => Err(Error::Variable(code_of(error.clone()))),
    }
}

/// The three variables that the client reads after the MMS Initiate (503 §7.2.2.2.2).
async fn read_after_association(mms: &Client, domain: &str) -> Result<ServerVariables, Error> {
    let names = [
        ObjectName {
            scope: Scope::Domain(domain.to_string()),
            name: "Bilateral_Table_ID".to_string(),
        },
        ObjectName {
            scope: Scope::Vmd,
            name: "TASE2_Version".to_string(),
        },
        ObjectName {
            scope: Scope::Vmd,
            name: "Supported_Features".to_string(),
        },
    ];
    let response = mms
        .read(ReadRequest {
            specification_with_result: None,
            variable_access_specification: list_of_variables(&names),
        })
        .await?;
    match response.list_of_access_result.as_slice() {
        [table_id, tase2, features] => Ok(ServerVariables {
            table_id: variable_value::<VisibleString32>(table_id)?,
            tase2: variable_value::<TASE2Version>(tase2)?,
            supported_features: variable_value::<SupportedFeatures>(features)?,
        }),
        _ => Err(Error::Malformed(
            "the three variables read after the association are expected",
        )),
    }
}

impl TaseClient {
    /// Establishes the association and runs the checks of the client after the Initiate (503 §7.2.2.2.2): the Bilateral
    /// Table identifier and the TASE.2 version are compared with the agreement; on a conflict the association is concluded
    /// and the conflict is returned.
    pub async fn associate(
        address: &str,
        config: ClientConfig,
        agreement: &Agreement,
    ) -> Result<(Self, TaseEvents), Error> {
        let (mms, events) = Client::connect(address, config).await?;
        let checked = match read_after_association(&mms, &agreement.domain).await {
            Ok(variables) => {
                client_after_initiate(&agreement.versions, &variables).map_err(Error::Conflict)
            }
            Err(error) => Err(error),
        };
        match checked {
            Ok(established) => Ok((
                Self {
                    mms,
                    domain: agreement.domain.clone(),
                    features: established.supported_features,
                },
                TaseEvents { events },
            )),
            Err(error) => {
                // The association is not used: it is concluded before the error is returned.
                let _ = mms.close().await;
                Err(error)
            }
        }
    }

    /// The domain of the agreement.
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// The building blocks that the server supports (`Supported_Features`).
    pub fn supported_features(&self) -> &SupportedFeatures {
        &self.features
    }

    /// Closes the association.
    pub async fn close(&self) -> Result<(), Error> {
        self.mms.close().await?;
        Ok(())
    }

    /// Get Data Value (503 §7.2.3.2): the value of each data value, or the error of its item.
    pub async fn get_data_values(&self, names: &[ObjectName]) -> Result<Vec<Item<Data>>, Error> {
        let response = self
            .mms
            .read(ReadRequest {
                specification_with_result: None,
                variable_access_specification: list_of_variables(names),
            })
            .await?;
        Ok(response.list_of_access_result.iter().map(item_of).collect())
    }

    /// Set Data Value (503 §7.2.3.3): writes each value, in the order of the names; the result is per item.
    pub async fn set_data_values(
        &self,
        writes: Vec<(ObjectName, Data)>,
    ) -> Result<Vec<Item<()>>, Error> {
        let names: Vec<ObjectName> = writes.iter().map(|(name, _)| name.clone()).collect();
        let values: Vec<Data> = writes.into_iter().map(|(_, value)| value).collect();
        let response = self
            .mms
            .write(WriteRequest {
                variable_access_specification: list_of_variables(&names),
                list_of_data: values,
            })
            .await?;
        Ok(response.0.iter().map(write_item_of).collect())
    }

    /// Get Data Set Element Values (503 §7.2.4.4): the values of the members, in the order of the data set.
    pub async fn get_data_set(&self, name: &ObjectName) -> Result<Vec<Item<Data>>, Error> {
        let response = self
            .mms
            .read(ReadRequest {
                specification_with_result: None,
                variable_access_specification: VariableAccessSpecification::VariableListName(
                    to_mms(name),
                ),
            })
            .await?;
        Ok(response.list_of_access_result.iter().map(item_of).collect())
    }

    /// Set Data Set Element Values (503 §7.2.4.5): writes one value per member, in the order of the data set.
    pub async fn set_data_set(
        &self,
        name: &ObjectName,
        values: Vec<Data>,
    ) -> Result<Vec<Item<()>>, Error> {
        let response = self
            .mms
            .write(WriteRequest {
                variable_access_specification: VariableAccessSpecification::VariableListName(
                    to_mms(name),
                ),
                list_of_data: values,
            })
            .await?;
        Ok(response.0.iter().map(write_item_of).collect())
    }

    /// The members of a data set, as the server holds them (GetNamedVariableListAttributes).
    pub async fn data_set_members(&self, name: &ObjectName) -> Result<Vec<ObjectName>, Error> {
        let response = self
            .mms
            .get_named_variable_list_attributes(GetNamedVariableListAttributesRequest(to_mms(name)))
            .await?;
        response
            .list_of_variable
            .iter()
            .map(|item| match &item.variable_specification {
                VariableSpecification::Name(member) => from_mms(member)
                    .ok_or(Error::Malformed("a member of a data set is a TASE.2 name")),
                _ => Err(Error::Malformed("a member of a data set is named")),
            })
            .collect()
    }

    /// Get Data Value Type (503 §7.2.3.5): the type description of a data value.
    pub async fn get_data_value_type(&self, name: &ObjectName) -> Result<TypeDescription, Error> {
        let response = self
            .mms
            .get_variable_access_attributes(GetVariableAccessAttributesRequest::Name(to_mms(name)))
            .await?;
        Ok(response.type_description)
    }

    /// Create Data Set (503 §7.2.4.2): defines the data set with its members. A refusal is the MMS error of the server.
    pub async fn create_data_set(
        &self,
        name: &ObjectName,
        members: &[ObjectName],
    ) -> Result<(), Error> {
        self.mms
            .define_named_variable_list(DefineNamedVariableListRequest {
                variable_list_name: to_mms(name),
                list_of_variable: members
                    .iter()
                    .map(|member| DefineNamedVariableListRequestListOfVariableItem {
                        variable_specification: variable(member),
                        alternate_access: None,
                    })
                    .collect(),
            })
            .await?;
        Ok(())
    }

    /// Delete Data Set (503 §7.2.4.3): deletes the data sets named; the answer is the number deleted.
    pub async fn delete_data_sets(&self, names: &[ObjectName]) -> Result<u32, Error> {
        let response = self
            .mms
            .delete_named_variable_list(DeleteNamedVariableListRequest {
                scope_of_delete: Some(DeleteNamedVariableListRequestScopeOfDelete::SPECIFIC),
                list_of_variable_list_name: Some(names.iter().map(to_mms).collect()),
                domain_name: None,
            })
            .await?;
        u32::try_from(response.number_deleted.0)
            .map_err(|_| Error::Malformed("a number of data sets deleted"))
    }

    /// Start Transfer (503 §7.2.5.2): writes the transfer set with its status ENABLED.
    pub async fn start_transfer(
        &self,
        name: &ObjectName,
        set: &DSTransferSet,
    ) -> Result<Item<()>, Error> {
        let enabled = DSTransferSet {
            status: DSTransferSetStatus(DSTransferSetStatus::ENABLED),
            ..set.clone()
        };
        self.write_transfer_set(name, &enabled).await
    }

    /// Stop Transfer (503 §7.2.5.3.2): writes the transfer set with its status DISABLED. The transfer set's other
    /// attributes are written as given; the server ends the monitoring.
    pub async fn stop_transfer(
        &self,
        name: &ObjectName,
        set: &DSTransferSet,
    ) -> Result<Item<()>, Error> {
        let disabled = DSTransferSet {
            status: DSTransferSetStatus(DSTransferSetStatus::DISABLED),
            ..set.clone()
        };
        self.write_transfer_set(name, &disabled).await
    }

    /// Start Transfer of the Information Message Transfer Set (503 §7.2.5.2): writes its status ENABLED. The server then reports
    /// each message whose condition occurs, until the transfer set is stopped.
    pub async fn start_information_transfer(&self) -> Result<Item<()>, Error> {
        self.write_information_transfer(true).await
    }

    /// Stop Transfer of the Information Message Transfer Set (503 §7.2.5.3.2): writes its status DISABLED.
    pub async fn stop_information_transfer(&self) -> Result<Item<()>, Error> {
        self.write_information_transfer(false).await
    }

    async fn write_information_transfer(&self, enabled: bool) -> Result<Item<()>, Error> {
        let name = VariableAccessSpecificationListOfVariableItem {
            variable_specification: VariableSpecification::Name(MmsObjectName::AaSpecific(
                Identifier(IM_TRANSFER_SET.to_string()),
            )),
            alternate_access: None,
        };
        let response = self
            .mms
            .write(WriteRequest {
                variable_access_specification: VariableAccessSpecification::ListOfVariable(vec![
                    name,
                ]),
                list_of_data: vec![Data::Boolean(enabled)],
            })
            .await?;
        response
            .0
            .first()
            .map(write_item_of)
            .ok_or(Error::Malformed(
                "one answer for the information transfer set",
            ))
    }

    /// Select (503 §7.2.6.2): reads the CheckBackID of an SBO device, which the server gives when the device is armed for this
    /// client. The reading of the CheckBackID is the client's business: the Operate does not carry it (see D-064).
    pub async fn select_device(
        &self,
        device: &ObjectName,
    ) -> Result<Item<SBOCheckBackName>, Error> {
        let name = device_variable(device, "SBO");
        match self
            .get_data_values(std::slice::from_ref(&name))
            .await?
            .into_iter()
            .next()
        {
            Some(Item::Success(data)) => Ok(Item::Success(SBOCheckBackName::from_mms(&data)?)),
            Some(Item::Failure(code)) => Ok(Item::Failure(code)),
            None => Err(Error::Malformed("one answer for the check-back identifier")),
        }
    }

    /// Operate (503 §7.2.6.3): writes the command of a device. For an SBO device, the client selects the device first, in the
    /// same association; the server refuses the command otherwise. The command is the value the device's control point takes.
    pub async fn operate_device(
        &self,
        device: &ObjectName,
        command: Data,
    ) -> Result<Item<()>, Error> {
        self.set_data_values(vec![(device.clone(), command)])
            .await?
            .into_iter()
            .next()
            .ok_or(Error::Malformed("one answer for the operate"))
    }

    /// Get Tag Value (503 §5.3.12.6, 802 §7.1): the tag of a device, its reason and its state.
    pub async fn get_tag(&self, device: &ObjectName) -> Result<Item<TagState>, Error> {
        let name = device_variable(device, "TAG");
        match self
            .get_data_values(std::slice::from_ref(&name))
            .await?
            .into_iter()
            .next()
        {
            Some(Item::Success(data)) => Ok(Item::Success(tag_state(&TagValue::from_mms(&data)?)?)),
            Some(Item::Failure(code)) => Ok(Item::Failure(code)),
            None => Err(Error::Malformed("one answer for the tag")),
        }
    }

    /// Set Tag Value (503 §5.3.12.5): sets the tag of a device, with its reason. The state bit of the value is not written: the
    /// server gives the state of the device.
    pub async fn set_tag(
        &self,
        device: &ObjectName,
        tag: Tag,
        reason: &str,
    ) -> Result<Item<()>, Error> {
        let value = tag_value(tag, reason)?;
        self.set_data_values(vec![(device_variable(device, "TAG"), value.to_mms())])
            .await?
            .into_iter()
            .next()
            .ok_or(Error::Malformed("one answer for the tag"))
    }

    async fn write_transfer_set(
        &self,
        name: &ObjectName,
        set: &DSTransferSet,
    ) -> Result<Item<()>, Error> {
        let response = self
            .mms
            .write(WriteRequest {
                variable_access_specification: list_of_variables(std::slice::from_ref(name)),
                list_of_data: vec![set.to_mms()],
            })
            .await?;
        response
            .0
            .first()
            .map(write_item_of)
            .ok_or(Error::Malformed("one answer for the transfer set"))
    }
}

impl TaseEvents {
    /// The next event, in order; `None` once the association is closed and every event was taken.
    pub async fn next(&mut self) -> Option<Result<TaseEvent, Error>> {
        loop {
            match self.events.next().await? {
                Event::State(state) => return Some(Ok(TaseEvent::Connection(state))),
                Event::Report(UnconfirmedService::InformationReport(report)) => {
                    return Some(match information_message(&report) {
                        Ok(Some(message)) => Ok(TaseEvent::Information(message)),
                        Ok(None) => received(&report).map(TaseEvent::Report),
                        Err(error) => Err(error),
                    });
                }
                // Other unconfirmed services and the traces of the MMS client are not transfer reports.
                Event::Report(_) | Event::Trace(_) => continue,
            }
        }
    }
}

/// The name of the Information Message Transfer Set object of the server (503 §8.3.9): an AA-specific variable.
const IM_TRANSFER_SET: &str = "IM_Transfer_Set";

/// The variable of a suffix of a device: `<Device_Name>_SBO` holds the CheckBackID, `<Device_Name>_TAG` the Tag_Value (6.15).
fn device_variable(device: &ObjectName, suffix: &str) -> ObjectName {
    ObjectName {
        scope: device.scope.clone(),
        name: format!("{}_{suffix}", device.name),
    }
}

/// The tag of a Tag_Value: its two bits give the code of the tag (802 §7.1), and its state bit whether the device is ARMED.
fn tag_state(value: &TagValue) -> Result<TagState, Error> {
    let hi = if value.flags.is_tag_hi() { 1 } else { 0 };
    let lo = if value.flags.is_tag_lo() { 2 } else { 0 };
    let tag = Tag::from_code(hi | lo).ok_or(Error::Malformed(
        "a tag of the standard: NO-TAG, OPEN-AND-CLOSE-INHIBIT or CLOSE-ONLY-INHIBIT",
    ))?;
    let state = if value.flags.is_tag_state() {
        DeviceState::Armed
    } else {
        DeviceState::Idle
    };
    Ok(TagState {
        tag,
        reason: value.reason.as_str().to_string(),
        state,
    })
}

/// The Tag_Value that sets a tag with its reason (the state bit is 0).
fn tag_value(tag: Tag, reason: &str) -> Result<TagValue, Error> {
    let code = tag.code();
    let mut bits = 0;
    if code & 1 != 0 {
        bits |= TagFlags::TAG_HI;
    }
    if code & 2 != 0 {
        bits |= TagFlags::TAG_LO;
    }
    Ok(TagValue {
        flags: TagFlags(bits),
        reason: TextString::new(reason)
            .map_err(|_| Error::Malformed("a reason of a tag of at most 255 characters"))?,
    })
}

/// The information message of a report whose variables are the header and the buffer of an information message (802 §6.4,
/// 503 §7.2.5.5.4). `None` for any other report, which is a transfer report.
fn information_message(report: &InformationReport) -> Result<Option<InformationMessage>, Error> {
    let VariableAccessSpecification::ListOfVariable(items) = &report.variable_access_specification
    else {
        return Ok(None);
    };
    let [header_item, _] = items.as_slice() else {
        return Ok(None);
    };
    let header_name = match &header_item.variable_specification {
        VariableSpecification::Name(name) => from_mms(name),
        _ => None,
    };
    if header_name
        != Some(ObjectName {
            scope: Scope::Vmd,
            name: HEADER_NAME.to_string(),
        })
    {
        return Ok(None);
    }
    let (header, buffer) = match report.list_of_access_result.as_slice() {
        [AccessResult::Success(header), AccessResult::Success(buffer)] => (header, buffer),
        _ => {
            return Err(Error::Malformed(
                "the header and the buffer of an information message are read",
            ))
        }
    };
    let header = InfoMessHeader::from_mms(header)?;
    let buffer = InfoBufXX::from_mms(buffer)?;
    InformationMessage::from_parts(header, &buffer)
        .map(Some)
        .map_err(|_| Error::Malformed("an information message whose size exceeds its buffer"))
}

fn received(report: &InformationReport) -> Result<ReceivedReport, Error> {
    let access = match &report.variable_access_specification {
        VariableAccessSpecification::ListOfVariable(items) => ReportAccess::Names(
            items
                .iter()
                .map(|item| match &item.variable_specification {
                    VariableSpecification::Name(name) => from_mms(name).ok_or(Error::Malformed(
                        "a named variable of a report is a TASE.2 name",
                    )),
                    _ => Err(Error::Malformed("a named variable of a report is named")),
                })
                .collect::<Result<Vec<_>, _>>()?,
        ),
        VariableAccessSpecification::VariableListName(name) => {
            ReportAccess::DataSet(from_mms(name).ok_or(Error::Malformed(
                "the data set of a report is a TASE.2 name",
            ))?)
        }
    };
    Ok(ReceivedReport {
        access,
        values: report.list_of_access_result.iter().map(item_of).collect(),
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    fn name(scope: Scope, item: &str) -> ObjectName {
        ObjectName {
            scope,
            name: item.to_string(),
        }
    }

    #[test]
    fn a_device_variable_takes_the_suffix_in_the_scope_of_the_device() {
        let device = name(Scope::Domain("ICC1".to_string()), "Breaker12");
        assert_eq!(
            device_variable(&device, "SBO"),
            name(Scope::Domain("ICC1".to_string()), "Breaker12_SBO")
        );
        assert_eq!(
            device_variable(&name(Scope::Vmd, "Breaker12"), "TAG"),
            name(Scope::Vmd, "Breaker12_TAG")
        );
    }

    #[test]
    fn every_tag_is_written_as_its_code_and_read_back() {
        for tag in [Tag::NoTag, Tag::OpenAndCloseInhibit, Tag::CloseOnlyInhibit] {
            let value = tag_value(tag, "crew on site").unwrap();
            let read = tag_state(&value).unwrap();
            assert_eq!(read.tag, tag);
            assert_eq!(read.reason, "crew on site");
            assert_eq!(read.state, DeviceState::Idle);
        }
    }

    #[test]
    fn the_state_bit_of_a_tag_is_the_armed_state() {
        let value = TagValue {
            flags: TagFlags(TagFlags::TAG_STATE),
            reason: TextString::new("").unwrap(),
        };
        assert_eq!(tag_state(&value).unwrap().tag, Tag::NoTag);
        assert_eq!(tag_state(&value).unwrap().state, DeviceState::Armed);
    }

    #[test]
    fn the_tag_code_three_is_refused() {
        let value = TagValue {
            flags: TagFlags(TagFlags::TAG_HI | TagFlags::TAG_LO),
            reason: TextString::new("").unwrap(),
        };
        assert!(matches!(tag_state(&value), Err(Error::Malformed(_))));
    }

    fn information_report(names: [&str; 2], header: Data, buffer: Data) -> InformationReport {
        InformationReport {
            variable_access_specification: VariableAccessSpecification::ListOfVariable(
                names
                    .iter()
                    .map(|item| VariableAccessSpecificationListOfVariableItem {
                        variable_specification: VariableSpecification::Name(
                            MmsObjectName::VmdSpecific(Identifier(item.to_string())),
                        ),
                        alternate_access: None,
                    })
                    .collect(),
            ),
            list_of_access_result: vec![
                AccessResult::Success(header),
                AccessResult::Success(buffer),
            ],
        }
    }

    fn header_data(size: i128) -> Data {
        Data::Structure(vec![
            Data::Integer(1),
            Data::Integer(2),
            Data::Integer(3),
            Data::Integer(size),
        ])
    }

    #[test]
    fn an_information_report_decodes_into_its_message() {
        let report = information_report(
            [HEADER_NAME, "Info_Buff_64"],
            header_data(3),
            Data::OctetString(vec![7, 8, 9, 0]),
        );
        let message = information_message(&report).unwrap().unwrap();
        assert_eq!(message.info_reference(), 1);
        assert_eq!(message.local_reference(), 2);
        assert_eq!(message.message_id(), 3);
        assert_eq!(message.stream(), &[7, 8, 9]);
    }

    #[test]
    fn a_transfer_report_is_not_an_information_message() {
        let report = information_report(["Flow1", "Level"], Data::Integer(1), Data::Integer(2));
        assert_eq!(information_message(&report).unwrap(), None);
    }

    #[test]
    fn a_size_beyond_its_buffer_is_refused() {
        let report = information_report(
            [HEADER_NAME, "Info_Buff_64"],
            header_data(9),
            Data::OctetString(vec![0; 4]),
        );
        assert!(matches!(
            information_message(&report),
            Err(Error::Malformed(_))
        ));
    }
}
