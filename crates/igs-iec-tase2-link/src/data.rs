// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The data values and the data sets of a server (IEC 60870-6-503 §5.2.4, §5.2.5, §7.2.3, §7.2.4), sans I/O: each
//! operation checks the Bilateral Table, then the existence of the objects, then the operation itself, and returns the
//! result per item that the MMS response carries. The values are kept by the caller, behind [`Store`].

use std::collections::{BTreeMap, BTreeSet};

use igs_iso9506_mms_codec::generated::types::{Data, TypeDescription};

use crate::bilateral::{Access, BilateralTable, Object, ObjectName, Permission, Scope};

/// The identifiers that a data set may reference for the Transfer Report (503 §7.2.4.2.3). They are not data values of the
/// VCC: a data set may hold them, and their values are not read or written through the data set.
pub const SPECIAL_MEMBERS: [&str; 4] = [
    "Transfer_Set_Name",
    "DSConditions_Detected",
    "Event_Code_Detected",
    "Transfer_Set_Time_Stamp",
];

/// The error classes of MMS that the operations use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    /// The access to the object is refused, or the object is not visible.
    Access,
    /// The definition of an object is refused: it exists, is undefined, or has an inconsistent attribute.
    Definition,
    /// The server has no resource for the operation.
    Resource,
}

/// The error codes of the operations of data values and data sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// The object does not exist for the client (also when it is not visible, 503 §7.2.3.1).
    ObjectNonExistent,
    /// The access control specification refuses the operation.
    ObjectAccessDenied,
    /// The object cannot be deleted now: an active transfer set references it.
    ObjectAccessUnsupported,
    /// A data set of this name exists.
    ObjectExists,
    /// A member, or the object itself, is not defined.
    ObjectUndefined,
    /// The scope or another attribute is inconsistent with the operation.
    ObjectAttributeInconsistent,
    /// The server has no memory left for the data set.
    MemoryUnavailable,
    /// The write failed in the server (the DataAccessError codes of 503 §7.2.3.3.3).
    HardwareFault,
    /// The value does not have the type of the object.
    TypeInconsistent,
    /// The object does not support the attribute of the write.
    ObjectAttributeUnsupported,
}

impl ErrorCode {
    /// The class of the error, as the standard gives it for the operations.
    pub fn class(self) -> ErrorClass {
        match self {
            Self::ObjectExists | Self::ObjectUndefined | Self::ObjectAttributeInconsistent => {
                ErrorClass::Definition
            }
            Self::MemoryUnavailable => ErrorClass::Resource,
            Self::ObjectNonExistent
            | Self::ObjectAccessDenied
            | Self::ObjectAccessUnsupported
            | Self::HardwareFault
            | Self::TypeInconsistent
            | Self::ObjectAttributeUnsupported => ErrorClass::Access,
        }
    }
}

/// The result of one item of a read or a write: the value, or the error of that item.
#[derive(Debug, Clone, PartialEq)]
pub enum Item<T> {
    /// The item succeeded, with its value (or nothing, for a write).
    Success(T),
    /// The item failed with this error.
    Failure(ErrorCode),
}

/// The values of the data values of the VCC. The caller keeps them; `None` means that the object does not exist.
pub trait Store {
    /// The value of a data value object, or `None` when the object does not exist.
    fn read(&self, name: &ObjectName) -> Option<Data>;

    /// Writes the value of a data value object. The object exists (checked by the caller).
    fn write(&mut self, name: &ObjectName, value: Data) -> Result<(), ErrorCode>;

    /// The type description of a data value object, or `None` when the object does not exist.
    fn type_of(&self, name: &ObjectName) -> Option<TypeDescription>;
}

/// The data sets of the server with their members, and the store of the data values.
#[derive(Debug, Clone)]
pub struct Vcc<S: Store> {
    store: S,
    data_sets: BTreeMap<ObjectName, Vec<ObjectName>>,
    capacity: usize,
}

fn is_special(name: &ObjectName) -> bool {
    SPECIAL_MEMBERS.contains(&name.name.as_str())
}

fn is_icc(name: &ObjectName) -> bool {
    matches!(name.scope, Scope::Domain(_))
}

fn object_access(access: Access) -> Result<(), ErrorCode> {
    match access {
        Access::Granted => Ok(()),
        Access::UnknownObject => Err(ErrorCode::ObjectNonExistent),
        Access::NotPermitted(_) | Access::IntervalOutOfAgreement | Access::OutOfTime => {
            Err(ErrorCode::ObjectAccessDenied)
        }
    }
}

impl<S: Store> Vcc<S> {
    /// A server with the values of `store`, no data set, and room for `capacity` data sets.
    pub fn new(store: S, capacity: usize) -> Self {
        Self {
            store,
            data_sets: BTreeMap::new(),
            capacity,
        }
    }

    /// The store of the values.
    pub fn store(&self) -> &S {
        &self.store
    }

    /// The members of a data set, or `None` when the data set does not exist.
    pub fn members(&self, name: &ObjectName) -> Option<&[ObjectName]> {
        self.data_sets.get(name).map(Vec::as_slice)
    }

    /// Adds a data set of the configuration, without the checks of a client's creation. The data sets of the VCC are
    /// configured by the operator as well as created by the clients.
    pub fn load_data_set(&mut self, name: ObjectName, members: Vec<ObjectName>) {
        self.data_sets.insert(name, members);
    }

    /// Get Data Value (503 §7.2.3.2): the value of each data value, or the error of each item. The object must be
    /// visible and readable for the client; an object that does not exist is an error of its item.
    pub fn get_data_values(&self, table: &BilateralTable, names: &[ObjectName]) -> Vec<Item<Data>> {
        names
            .iter()
            .map(|name| {
                let object = Object::DataValue(name.clone());
                match object_access(table.check(&object, Permission::GetDataValue)) {
                    Err(code) => Item::Failure(code),
                    Ok(()) => match self.store.read(name) {
                        Some(value) => Item::Success(value),
                        None => Item::Failure(ErrorCode::ObjectNonExistent),
                    },
                }
            })
            .collect()
    }

    /// Set Data Value (503 §7.2.3.3): writes each value, in the order of the names. An item that the table refuses, or that
    /// the server cannot write, is an error of that item and writes nothing.
    pub fn set_data_values(
        &mut self,
        table: &BilateralTable,
        writes: Vec<(ObjectName, Data)>,
    ) -> Vec<Item<()>> {
        writes
            .into_iter()
            .map(|(name, value)| {
                let object = Object::DataValue(name.clone());
                if let Err(code) = object_access(table.check(&object, Permission::SetDataValue)) {
                    return Item::Failure(code);
                }
                if self.store.read(&name).is_none() {
                    return Item::Failure(ErrorCode::ObjectNonExistent);
                }
                match self.store.write(&name, value) {
                    Ok(()) => Item::Success(()),
                    Err(code) => Item::Failure(code),
                }
            })
            .collect()
    }

    /// Get Data Value Type (503 §7.2.3.5): the type description of a data value, for the client that may read it.
    pub fn get_data_value_type(
        &self,
        table: &BilateralTable,
        name: &ObjectName,
    ) -> Result<TypeDescription, ErrorCode> {
        let object = Object::DataValue(name.clone());
        object_access(table.check(&object, Permission::GetDataValueType))?;
        self.store.type_of(name).ok_or(ErrorCode::ObjectNonExistent)
    }

    /// Create Data Set (503 §7.2.4.2.3). The checks come in the order of the standard, and the first failure is the
    /// answer: the access (the client may create data sets, and reads each member), the uniqueness of the name, the
    /// existence of each member, the ICC-specific scope of the name, and the resources.
    pub fn create_data_set(
        &mut self,
        table: &BilateralTable,
        name: ObjectName,
        members: Vec<ObjectName>,
    ) -> Result<(), ErrorCode> {
        if !table.may_create_data_sets() {
            return Err(ErrorCode::ObjectAccessDenied);
        }
        // The client must have at least Read access to every member: any refusal is an access error (503 §7.2.4.2.3).
        for member in members.iter().filter(|member| !is_special(member)) {
            let object = Object::DataValue(member.clone());
            if table.check(&object, Permission::GetDataValue) != Access::Granted {
                return Err(ErrorCode::ObjectAccessDenied);
            }
        }
        if self.data_sets.contains_key(&name) {
            return Err(ErrorCode::ObjectExists);
        }
        for member in members.iter().filter(|member| !is_special(member)) {
            if self.store.read(member).is_none() {
                return Err(ErrorCode::ObjectUndefined);
            }
        }
        if !is_icc(&name) {
            return Err(ErrorCode::ObjectAttributeInconsistent);
        }
        if self.data_sets.len() >= self.capacity {
            return Err(ErrorCode::MemoryUnavailable);
        }
        self.data_sets.insert(name, members);
        Ok(())
    }

    /// Delete Data Set (503 §7.2.4.3.3), for each name: the existence, the access, the absence of an active transfer set
    /// that references it (`in_use`), and the ICC-specific scope. A data set that fails is not deleted; the others are.
    pub fn delete_data_sets(
        &mut self,
        table: &BilateralTable,
        names: &[ObjectName],
        in_use: &BTreeSet<ObjectName>,
    ) -> Vec<Result<(), ErrorCode>> {
        names
            .iter()
            .map(|name| {
                if !self.data_sets.contains_key(name) {
                    return Err(ErrorCode::ObjectUndefined);
                }
                if table.check(&Object::DataSet(name.clone()), Permission::DeleteDataSet)
                    != Access::Granted
                {
                    return Err(ErrorCode::ObjectAccessDenied);
                }
                if in_use.contains(name) {
                    return Err(ErrorCode::ObjectAccessUnsupported);
                }
                if !is_icc(name) {
                    return Err(ErrorCode::ObjectAttributeInconsistent);
                }
                self.data_sets.remove(name);
                Ok(())
            })
            .collect()
    }

    /// Get Data Set Element Values (503 §7.2.4.4.3): the values of the members of a data set, one item per member. A
    /// member that is a special identifier, or that the table does not let the client read, is an error of its item.
    pub fn get_data_set_element_values(
        &self,
        table: &BilateralTable,
        name: &ObjectName,
    ) -> Result<Vec<(ObjectName, Item<Data>)>, ErrorCode> {
        let members = self.data_set_for(table, name, Permission::GetDataSetElementValues)?;
        Ok(members
            .into_iter()
            .map(|member| {
                let item = self.member_read(table, &member);
                (member, item)
            })
            .collect())
    }

    /// Set Data Set Element Values (503 §7.2.4.5.3): writes the values in the order of the members. A value that is not
    /// one per member is refused as an attribute inconsistency of the data set; a member that is special, or that the table
    /// does not let the client write, is an error of its item.
    pub fn set_data_set_element_values(
        &mut self,
        table: &BilateralTable,
        name: &ObjectName,
        values: Vec<Data>,
    ) -> Result<Vec<Item<()>>, ErrorCode> {
        let members = self.data_set_for(table, name, Permission::SetDataSetElementValues)?;
        if members.len() != values.len() {
            return Err(ErrorCode::ObjectAttributeInconsistent);
        }
        Ok(members
            .iter()
            .zip(values)
            .map(|(member, value)| self.member_write(table, member, value))
            .collect())
    }

    /// The read of one member of a data set. Without read access to the data value (an invisible one included), the
    /// item fails with access denied (503 §7.2.4.4.3); so does a special identifier.
    fn member_read(&self, table: &BilateralTable, member: &ObjectName) -> Item<Data> {
        if is_special(member)
            || table.check(&Object::DataValue(member.clone()), Permission::GetDataValue)
                != Access::Granted
        {
            return Item::Failure(ErrorCode::ObjectAccessDenied);
        }
        match self.store.read(member) {
            Some(value) => Item::Success(value),
            None => Item::Failure(ErrorCode::ObjectNonExistent),
        }
    }

    /// The write of one member of a data set, with the same refusals as the read (503 §7.2.4.5.3).
    fn member_write(
        &mut self,
        table: &BilateralTable,
        member: &ObjectName,
        value: Data,
    ) -> Item<()> {
        if is_special(member)
            || table.check(&Object::DataValue(member.clone()), Permission::SetDataValue)
                != Access::Granted
        {
            return Item::Failure(ErrorCode::ObjectAccessDenied);
        }
        if self.store.read(member).is_none() {
            return Item::Failure(ErrorCode::ObjectNonExistent);
        }
        match self.store.write(member, value) {
            Ok(()) => Item::Success(()),
            Err(code) => Item::Failure(code),
        }
    }

    /// The members of a data set for an operation of the client: the data set must exist, and be visible and permitted.
    fn data_set_for(
        &self,
        table: &BilateralTable,
        name: &ObjectName,
        permission: Permission,
    ) -> Result<Vec<ObjectName>, ErrorCode> {
        object_access(table.check(&Object::DataSet(name.clone()), permission))?;
        self.data_sets
            .get(name)
            .cloned()
            .ok_or(ErrorCode::ObjectNonExistent)
    }
}
