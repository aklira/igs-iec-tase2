// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The bilateral table (IEC 60870-6-503 §5.2.3, §5.3.2, §5.3.4, §6.5, §8.3): the data values, data sets, transfer sets
//! and devices that a remote control centre may access, with the operations it may perform on each, and the checks of
//! each access.
//!
//! The table is local to the server: the standard does not fix its format (503 §5.2.3.1). The checks are the general
//! validity check of every operation (§5.2.3.2), the access check of each object (§5.2.3.3 to §5.2.3.8) and the admission
//! of the client (§5.3.2). The operations of each object type are those of the standard's "Permitted Access" lists (in the object models of 5.3).

use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use igs_iec_tase2_model::generated::iso503::VisibleString32;
use igs_iec_tase2_model::generated::iso802::SBOCheckBackName;

use crate::version::Versions;

/// The scope of an MMS object name: VMD-specific, or specific to a domain.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
    /// The VMD-specific scope.
    Vmd,
    /// The domain-specific scope, with the name of the domain.
    Domain(String),
}

/// The name of an object of the table: its scope and its identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectName {
    /// The scope of the name.
    pub scope: Scope,
    /// The identifier, as the object is named in the table.
    pub name: String,
}

/// An operation that the access control specification permits, named as in the standard's Permitted Access lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Permission {
    /// The object is visible to the client. Without it the object does not exist for the client.
    Visibility,
    /// Get Data Value (503 §5.3.6.2).
    GetDataValue,
    /// Set Data Value (503 §5.3.6.2).
    SetDataValue,
    /// Get Data Value Type (503 §5.3.6.2).
    GetDataValueType,
    /// Delete Data Set (503 §5.3.7.2).
    DeleteDataSet,
    /// Get Data Set Element Values (503 §5.3.7.2).
    GetDataSetElementValues,
    /// Set Data Set Element Values (503 §5.3.7.2).
    SetDataSetElementValues,
    /// Get Data Set Element Names (503 §5.3.7.2).
    GetDataSetElementNames,
    /// Start Transfer (503 §5.3.10.2.1).
    StartTransfer,
    /// Stop Transfer (503 §5.3.10.2.1).
    StopTransfer,
    /// Get Next DSTransfer Set Name (503 §5.3.10.2.1).
    GetNextDsTransferSetName,
    /// Select (503 §5.3.12.2).
    Select,
    /// Operate (503 §5.3.12.2).
    Operate,
    /// Set Tag (503 §5.3.12.2).
    SetTag,
}

/// The access control specification of a data value, a data set or a transfer set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectAccess {
    /// The operations the client may perform, the visibility included.
    pub permitted: BTreeSet<Permission>,
    /// The shortest interval that the client may ask for in a transfer set or a data value (the maximum exchange rate of
    /// the agreement, 503 §5.2.3.3). `None` when the agreement sets none.
    pub minimum_interval: Option<Duration>,
}

/// The access control specification of a device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceAccess {
    /// The operations the client may perform, the visibility included.
    pub permitted: BTreeSet<Permission>,
    /// The value returned in response to Select (the CheckBackID attribute, 503 §5.2.3.8). Its MMS type is SBO_CheckBackName, an
    /// integer of 16 bits (IEC 60870-6-802 §6.2.2, and page 27 of the standard).
    pub check_back_id: SBOCheckBackName,
    /// The time within which an operation on the device must be performed (the Timeout attribute, 503 §5.2.3.8).
    pub timeout: Duration,
}

/// An object of the table, by its kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    /// A data value (503 §5.3.6.2).
    DataValue(ObjectName),
    /// A data set (503 §5.3.7.2).
    DataSet(ObjectName),
    /// A transfer set (503 §5.3.10.2.1).
    TransferSet(ObjectName),
    /// A device (503 §5.3.12.2).
    Device(ObjectName),
}

/// The result of an access check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    /// The operation is covered by the agreement.
    Granted,
    /// The object is not in the table, or is not visible to the client (503 §5.2.3.2).
    UnknownObject,
    /// The object is visible, but the operation is not in its permitted access.
    NotPermitted(Permission),
    /// The interval asked for is shorter than the agreed maximum exchange rate (503 §5.2.3.3).
    IntervalOutOfAgreement,
    /// The operation on a device is not performed within its timeout (503 §5.2.3.8).
    OutOfTime,
}

/// The result of the admission of a client after the association request (503 §5.3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    /// The client has a Bilateral Table and its application reference is listed.
    Granted,
    /// No Bilateral Table exists for the designation of the client: the association is refused.
    UnknownClient,
    /// The table exists, but the application reference of the request is not in its list: refused.
    UnknownApplicationReference,
}

/// The Bilateral Table of one remote control centre.
#[derive(Debug, Clone, PartialEq)]
pub struct BilateralTable {
    designation: String,
    versions: Versions,
    domain: String,
    application_references: BTreeSet<String>,
    data_set_creation: bool,
    data_values: BTreeMap<ObjectName, ObjectAccess>,
    data_sets: BTreeMap<ObjectName, ObjectAccess>,
    transfer_sets: BTreeMap<ObjectName, ObjectAccess>,
    devices: BTreeMap<ObjectName, DeviceAccess>,
}

impl BilateralTable {
    /// An empty table for the remote control centre `designation`, with its versions and the domain of the client.
    pub fn new(
        designation: impl Into<String>,
        versions: Versions,
        domain: impl Into<String>,
    ) -> Self {
        Self {
            designation: designation.into(),
            versions,
            domain: domain.into(),
            application_references: BTreeSet::new(),
            data_set_creation: false,
            data_values: BTreeMap::new(),
            data_sets: BTreeMap::new(),
            transfer_sets: BTreeMap::new(),
            devices: BTreeMap::new(),
        }
    }

    /// Lists an application reference that may establish an association.
    #[must_use]
    pub fn with_application_reference(mut self, reference: impl Into<String>) -> Self {
        self.application_references.insert(reference.into());
        self
    }

    /// Allows the client to create data sets (the Create Data Set access permission, 503 §7.2.4.2). The permission is not in
    /// the list of an object, since the object does not exist yet: the table grants it for all.
    #[must_use]
    pub fn with_data_set_creation(mut self) -> Self {
        self.data_set_creation = true;
        self
    }

    /// Whether the client may create data sets.
    pub fn may_create_data_sets(&self) -> bool {
        self.data_set_creation
    }

    /// Adds a data value and its access control specification.
    #[must_use]
    pub fn with_data_value(mut self, name: ObjectName, access: ObjectAccess) -> Self {
        self.data_values.insert(name, access);
        self
    }

    /// Adds a data set and its access control specification.
    #[must_use]
    pub fn with_data_set(mut self, name: ObjectName, access: ObjectAccess) -> Self {
        self.data_sets.insert(name, access);
        self
    }

    /// Grants a data set to the client: the access of a data set that the client has just created (decision D-055).
    pub fn grant_data_set(&mut self, name: ObjectName, access: ObjectAccess) {
        self.data_sets.insert(name, access);
    }

    /// Adds a transfer set and its access control specification.
    #[must_use]
    pub fn with_transfer_set(mut self, name: ObjectName, access: ObjectAccess) -> Self {
        self.transfer_sets.insert(name, access);
        self
    }

    /// Adds a device and its access control specification.
    #[must_use]
    pub fn with_device(mut self, name: ObjectName, access: DeviceAccess) -> Self {
        self.devices.insert(name, access);
        self
    }

    /// The remote control centre of the table (the Client Control Centre Designation).
    pub fn designation(&self) -> &str {
        &self.designation
    }

    /// The versions of the table: its identifier (`Bilateral_Table_ID`) and the TASE.2 version.
    pub fn versions(&self) -> &Versions {
        &self.versions
    }

    /// The domain of the client, which the table is for (not mapped to MMS, 503 §6.5).
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// The access control specification of an object, or `None` when the table does not contain it.
    fn permitted(&self, object: &Object) -> Option<(&BTreeSet<Permission>, Option<Duration>)> {
        match object {
            Object::DataValue(name) => self
                .data_values
                .get(name)
                .map(|access| (&access.permitted, access.minimum_interval)),
            Object::DataSet(name) => self
                .data_sets
                .get(name)
                .map(|access| (&access.permitted, access.minimum_interval)),
            Object::TransferSet(name) => self
                .transfer_sets
                .get(name)
                .map(|access| (&access.permitted, access.minimum_interval)),
            Object::Device(name) => self
                .devices
                .get(name)
                .map(|access| (&access.permitted, None)),
        }
    }

    /// The general validity check and the permitted access of one operation on an object (503 §5.2.3.2 to §5.2.3.7).
    pub fn check(&self, object: &Object, permission: Permission) -> Access {
        let Some((permitted, _)) = self.permitted(object) else {
            return Access::UnknownObject;
        };
        // An object without visibility does not exist for the client.
        if !permitted.contains(&Permission::Visibility) {
            return Access::UnknownObject;
        }
        if permitted.contains(&permission) {
            Access::Granted
        } else {
            Access::NotPermitted(permission)
        }
    }

    /// The check of a requested interval, for a data value or a transfer set: the interval must not be shorter than the
    /// agreed maximum exchange rate (503 §5.2.3.3).
    pub fn check_interval(&self, object: &Object, requested: Duration) -> Access {
        match self.check(object, Permission::Visibility) {
            Access::Granted => {}
            other => return other,
        }
        match self.permitted(object).and_then(|(_, minimum)| minimum) {
            Some(minimum) if requested < minimum => Access::IntervalOutOfAgreement,
            _ => Access::Granted,
        }
    }

    /// The check of the time taken by an operation on a device: it must be within the timeout of the device (503
    /// §5.2.3.8). `elapsed` is the time between the request and the operation.
    pub fn check_device_time(&self, name: &ObjectName, elapsed: Duration) -> Access {
        let object = Object::Device(name.clone());
        match self.check(&object, Permission::Visibility) {
            Access::Granted => {}
            other => return other,
        }
        match self.devices.get(name) {
            Some(device) if elapsed > device.timeout => Access::OutOfTime,
            _ => Access::Granted,
        }
    }

    /// The Timeout of a device in the table: the time within which an operation on it must be performed. `None` when the
    /// device is not in the table.
    pub fn device_timeout(&self, name: &ObjectName) -> Option<Duration> {
        self.devices.get(name).map(|device| device.timeout)
    }

    /// The CheckBackID of a device, returned in response to Select: `None` when the device is not in the table or not
    /// visible to the client.
    pub fn check_back_id(&self, name: &ObjectName) -> Option<SBOCheckBackName> {
        let object = Object::Device(name.clone());
        if self.check(&object, Permission::Select) != Access::Granted {
            return None;
        }
        self.devices.get(name).map(|device| device.check_back_id)
    }

    /// Whether a value of the table identifier (`Bilateral_Table_ID`) is the one of this table.
    pub fn has_table_id(&self, id: &VisibleString32) -> bool {
        &self.versions.table_id == id
    }
}

/// The Bilateral Tables of a server: one per remote control centre that it serves (503 §5.3.2).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BilateralTables {
    tables: BTreeMap<String, BilateralTable>,
}

impl BilateralTables {
    /// No table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the table of a remote control centre; a table for the same designation replaces the previous one.
    pub fn insert(&mut self, table: BilateralTable) {
        self.tables.insert(table.designation.clone(), table);
    }

    /// The table of a remote control centre, by its designation.
    pub fn table(&self, designation: &str) -> Option<&BilateralTable> {
        self.tables.get(designation)
    }

    /// The table of a remote control centre, for its changes.
    pub fn table_mut(&mut self, designation: &str) -> Option<&mut BilateralTable> {
        self.tables.get_mut(designation)
    }

    /// The admission of a client: its designation must have a table, and the application reference of the request must
    /// be listed in that table (503 §5.3.2).
    pub fn admit(&self, designation: &str, application_reference: &str) -> Admission {
        match self.table(designation) {
            None => Admission::UnknownClient,
            Some(table) if table.application_references.contains(application_reference) => {
                Admission::Granted
            }
            Some(_) => Admission::UnknownApplicationReference,
        }
    }
}
