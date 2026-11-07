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
use igs_iec_tase2_link::version::Versions;
use igs_iec_tase2_model::generated::iso503::{
    DSTransferSet, DSTransferSetStatus, SupportedFeatures, TASE2Version, VisibleString32,
};
use igs_iec_tase2_model::mms::MmsValue;
use igs_iso9506_mms::client::{Client, ClientConfig, ConnectionState, Event, Events};
use igs_iso9506_mms_codec::generated::types::{
    AccessResult, Data, DataAccessError, DefineNamedVariableListRequest,
    DefineNamedVariableListRequestListOfVariableItem, DeleteNamedVariableListRequest,
    DeleteNamedVariableListRequestScopeOfDelete, GetNamedVariableListAttributesRequest,
    InformationReport, ReadRequest, UnconfirmedService, VariableAccessSpecification,
    VariableSpecification, WriteRequest, WriteResponseItemItem,
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
                    return Some(received(&report).map(TaseEvent::Report));
                }
                // Other unconfirmed services and the traces of the MMS client are not transfer reports.
                Event::Report(_) | Event::Trace(_) => continue,
            }
        }
    }
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
