// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The data values and the data sets (task B3): the operations of each, their success, and each of their errors.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::collections::{BTreeMap, BTreeSet};

use igs_iec_tase2_link::bilateral::{BilateralTable, ObjectAccess, ObjectName, Permission, Scope};
use igs_iec_tase2_link::data::{ErrorClass, ErrorCode, Item, Store, Vcc};
use igs_iec_tase2_link::version::{Versions, STANDARD_VERSION};
use igs_iec_tase2_model::generated::iso503::VisibleString32;
use igs_iso9506_mms_codec::generated::types::{Data, TypeDescription, Unsigned8};
use igs_iso9506_mms_codec::schema::BitString;

/// The values of the data values, with the objects whose write fails in the server.
#[derive(Debug, Clone, Default)]
struct MemoryStore {
    values: BTreeMap<ObjectName, Data>,
    failing: BTreeSet<ObjectName>,
}

impl Store for MemoryStore {
    fn read(&self, name: &ObjectName) -> Option<Data> {
        self.values.get(name).cloned()
    }

    fn write(&mut self, name: &ObjectName, value: Data) -> Result<(), ErrorCode> {
        let Some(current) = self.values.get(name) else {
            return Err(ErrorCode::ObjectNonExistent);
        };
        if self.failing.contains(name) {
            return Err(ErrorCode::HardwareFault);
        }
        if std::mem::discriminant(current) != std::mem::discriminant(&value) {
            return Err(ErrorCode::TypeInconsistent);
        }
        self.values.insert(name.clone(), value);
        Ok(())
    }

    fn type_of(&self, name: &ObjectName) -> Option<TypeDescription> {
        self.values
            .get(name)
            .map(|_| TypeDescription::Integer(Unsigned8(32)))
    }
}

fn icc(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

fn vmd(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Vmd,
        name: item.to_string(),
    }
}

fn access(list: &[Permission]) -> ObjectAccess {
    ObjectAccess {
        permitted: list.iter().copied().collect(),
        minimum_interval: None,
    }
}

/// The table: five data values (one of them invisible, one readable only, one writable with a failing store), and three
/// data sets (one deletable, one whose deletion is refused, and one for the special identifiers).
fn table(may_create: bool) -> BilateralTable {
    let versions = Versions {
        table_id: VisibleString32::new("BT-1").unwrap(),
        tase2: STANDARD_VERSION,
    };
    let mut table = BilateralTable::new("remote-cc", versions, "ICC1")
        .with_application_reference("ae-1")
        .with_data_value(
            icc("Flow1"),
            access(&[
                Permission::Visibility,
                Permission::GetDataValue,
                Permission::SetDataValue,
                Permission::GetDataValueType,
            ]),
        )
        .with_data_value(
            icc("ReadOnly"),
            access(&[
                Permission::Visibility,
                Permission::GetDataValue,
                Permission::GetDataValueType,
            ]),
        )
        .with_data_value(
            icc("NoRead"),
            access(&[Permission::Visibility, Permission::SetDataValue]),
        )
        .with_data_value(
            icc("Hidden"),
            access(&[Permission::GetDataValue, Permission::SetDataValue]),
        )
        .with_data_value(
            icc("Failing"),
            access(&[
                Permission::Visibility,
                Permission::GetDataValue,
                Permission::SetDataValue,
            ]),
        )
        .with_data_value(
            icc("Ghost"),
            access(&[Permission::Visibility, Permission::GetDataValue]),
        )
        .with_data_set(
            icc("DS1"),
            access(&[
                Permission::Visibility,
                Permission::DeleteDataSet,
                Permission::GetDataSetElementValues,
                Permission::SetDataSetElementValues,
            ]),
        )
        .with_data_set(
            icc("DS2"),
            access(&[Permission::Visibility, Permission::GetDataSetElementValues]),
        )
        .with_data_set(
            icc("DSFail"),
            access(&[Permission::Visibility, Permission::SetDataSetElementValues]),
        )
        .with_data_set(
            icc("DSSpecial"),
            access(&[
                Permission::Visibility,
                Permission::GetDataSetElementValues,
                Permission::SetDataSetElementValues,
            ]),
        );
    if may_create {
        table = table.with_data_set_creation();
    }
    table
}

fn store() -> MemoryStore {
    let mut store = MemoryStore::default();
    store.values.insert(icc("Flow1"), Data::Integer(5));
    store.values.insert(icc("ReadOnly"), Data::Integer(1));
    store.values.insert(icc("NoRead"), Data::Integer(2));
    store.values.insert(icc("Hidden"), Data::Integer(0));
    store.values.insert(icc("Failing"), Data::Integer(3));
    store.failing.insert(icc("Failing"));
    store
}

/// A server with the store, the data sets DS1 (Flow1, ReadOnly), DS2 (Flow1), DSSpecial (Flow1 and a special identifier)
/// loaded, and room for four data sets.
fn server() -> Vcc<MemoryStore> {
    let mut vcc = Vcc::new(store(), 4);
    vcc.load_data_set(icc("DS1"), vec![icc("Flow1"), icc("ReadOnly")]);
    vcc.load_data_set(icc("DS2"), vec![icc("Flow1")]);
    vcc.load_data_set(
        icc("DSSpecial"),
        vec![
            icc("Flow1"),
            ObjectName {
                scope: Scope::Domain("ICC1".to_string()),
                name: "Transfer_Set_Name".to_string(),
            },
        ],
    );
    vcc
}

#[test]
fn an_error_class_is_the_one_of_the_standard() {
    assert_eq!(ErrorCode::ObjectAccessDenied.class(), ErrorClass::Access);
    assert_eq!(ErrorCode::ObjectExists.class(), ErrorClass::Definition);
    assert_eq!(ErrorCode::ObjectUndefined.class(), ErrorClass::Definition);
    assert_eq!(ErrorCode::MemoryUnavailable.class(), ErrorClass::Resource);
}

#[test]
fn get_data_value_reads_the_values_of_the_readable_objects() {
    let vcc = server();
    let items = vcc.get_data_values(&table(false), &[icc("Flow1"), icc("ReadOnly")]);
    assert_eq!(
        items,
        vec![
            Item::Success(Data::Integer(5)),
            Item::Success(Data::Integer(1))
        ]
    );
}

#[test]
fn get_data_value_fails_per_item_for_an_object_that_does_not_exist_or_is_invisible_or_unreadable() {
    let vcc = server();
    let items = vcc.get_data_values(
        &table(false),
        &[icc("Missing"), icc("Hidden"), icc("NoRead")],
    );
    assert_eq!(
        items,
        vec![
            Item::Failure(ErrorCode::ObjectNonExistent),
            Item::Failure(ErrorCode::ObjectNonExistent),
            Item::Failure(ErrorCode::ObjectAccessDenied),
        ]
    );
}

#[test]
fn set_data_value_writes_a_permitted_object_of_the_same_type() {
    let mut vcc = server();
    let items = vcc.set_data_values(&table(false), vec![(icc("Flow1"), Data::Integer(9))]);
    assert_eq!(items, vec![Item::Success(())]);
    assert_eq!(
        vcc.store().values.get(&icc("Flow1")),
        Some(&Data::Integer(9))
    );
}

#[test]
fn set_data_value_refuses_a_write_the_table_does_not_permit_and_writes_nothing() {
    let mut vcc = server();
    let items = vcc.set_data_values(&table(false), vec![(icc("ReadOnly"), Data::Integer(7))]);
    assert_eq!(items, vec![Item::Failure(ErrorCode::ObjectAccessDenied)]);
    assert_eq!(
        vcc.store().values.get(&icc("ReadOnly")),
        Some(&Data::Integer(1))
    );
}

#[test]
fn set_data_value_refuses_a_missing_object_and_a_value_of_another_type() {
    let mut vcc = server();
    let items = vcc.set_data_values(
        &table(false),
        vec![
            (icc("Missing"), Data::Integer(1)),
            (icc("Flow1"), Data::VisibleString("text".to_string())),
        ],
    );
    assert_eq!(
        items,
        vec![
            Item::Failure(ErrorCode::ObjectNonExistent),
            Item::Failure(ErrorCode::TypeInconsistent),
        ]
    );
    assert_eq!(
        vcc.store().values.get(&icc("Flow1")),
        Some(&Data::Integer(5))
    );
}

#[test]
fn set_data_value_reports_a_failure_of_the_server_as_its_error() {
    let mut vcc = server();
    let items = vcc.set_data_values(&table(false), vec![(icc("Failing"), Data::Integer(4))]);
    assert_eq!(items, vec![Item::Failure(ErrorCode::HardwareFault)]);
}

#[test]
fn get_data_value_type_describes_a_readable_object_only() {
    let vcc = server();
    let table = table(false);
    assert!(vcc.get_data_value_type(&table, &icc("Flow1")).is_ok());
    assert_eq!(
        vcc.get_data_value_type(&table, &icc("NoRead")),
        Err(ErrorCode::ObjectAccessDenied)
    );
    assert_eq!(
        vcc.get_data_value_type(&table, &icc("Missing")),
        Err(ErrorCode::ObjectNonExistent)
    );
}

#[test]
fn create_data_set_defines_a_data_set_of_readable_members() {
    let mut vcc = server();
    let created = vcc.create_data_set(
        &table(true),
        icc("DSNew"),
        vec![icc("Flow1"), icc("ReadOnly")],
    );
    assert_eq!(created, Ok(()));
    assert_eq!(
        vcc.members(&icc("DSNew")),
        Some([icc("Flow1"), icc("ReadOnly")].as_slice())
    );
}

#[test]
fn create_data_set_is_refused_when_the_table_does_not_allow_it() {
    let mut vcc = server();
    assert_eq!(
        vcc.create_data_set(&table(false), icc("DSNew"), vec![icc("Flow1")]),
        Err(ErrorCode::ObjectAccessDenied)
    );
}

#[test]
fn create_data_set_refuses_a_member_that_the_client_cannot_read() {
    let mut vcc = server();
    // A member that is invisible or without read access: the access check comes first.
    assert_eq!(
        vcc.create_data_set(&table(true), icc("DSNew"), vec![icc("Hidden")]),
        Err(ErrorCode::ObjectAccessDenied)
    );
    assert_eq!(
        vcc.create_data_set(&table(true), icc("DSNew"), vec![icc("NoRead")]),
        Err(ErrorCode::ObjectAccessDenied)
    );
}

#[test]
fn create_data_set_refuses_a_name_that_exists() {
    let mut vcc = server();
    assert_eq!(
        vcc.create_data_set(&table(true), icc("DS1"), vec![icc("Flow1")]),
        Err(ErrorCode::ObjectExists)
    );
}

#[test]
fn create_data_set_refuses_a_member_that_is_not_defined() {
    let mut vcc = server();
    // The client may read Ghost, but the server holds no value for it.
    assert_eq!(
        vcc.create_data_set(&table(true), icc("DSNew"), vec![icc("Ghost")]),
        Err(ErrorCode::ObjectUndefined)
    );
}

#[test]
fn create_data_set_refuses_a_vmd_specific_name_as_inconsistent() {
    let mut vcc = server();
    assert_eq!(
        vcc.create_data_set(&table(true), vmd("DSVmd"), vec![icc("Flow1")]),
        Err(ErrorCode::ObjectAttributeInconsistent)
    );
}

#[test]
fn create_data_set_refuses_when_the_server_has_no_room() {
    let mut vcc = Vcc::new(store(), 1);
    vcc.load_data_set(icc("DS1"), vec![icc("Flow1")]);
    assert_eq!(
        vcc.create_data_set(&table(true), icc("DSNew"), vec![icc("Flow1")]),
        Err(ErrorCode::MemoryUnavailable)
    );
}

#[test]
fn delete_data_set_removes_a_deletable_data_set_and_keeps_the_others() {
    let mut vcc = server();
    let outcomes = vcc.delete_data_sets(
        &table(false),
        &[icc("DS1"), icc("Missing")],
        &BTreeSet::new(),
    );
    assert_eq!(outcomes, vec![Ok(()), Err(ErrorCode::ObjectUndefined)]);
    assert!(vcc.members(&icc("DS1")).is_none());
}

#[test]
fn delete_data_set_refuses_a_data_set_the_client_may_not_delete() {
    let mut vcc = server();
    assert_eq!(
        vcc.delete_data_sets(&table(false), &[icc("DS2")], &BTreeSet::new()),
        vec![Err(ErrorCode::ObjectAccessDenied)]
    );
    assert!(vcc.members(&icc("DS2")).is_some());
}

#[test]
fn delete_data_set_refuses_a_data_set_that_an_active_transfer_set_uses() {
    let mut vcc = server();
    let in_use = BTreeSet::from([icc("DS1")]);
    assert_eq!(
        vcc.delete_data_sets(&table(false), &[icc("DS1")], &in_use),
        vec![Err(ErrorCode::ObjectAccessUnsupported)]
    );
    assert!(vcc.members(&icc("DS1")).is_some());
}

#[test]
fn delete_data_set_refuses_a_vmd_specific_data_set_as_inconsistent() {
    let mut vcc = server();
    vcc.load_data_set(vmd("DSVmd"), vec![icc("Flow1")]);
    let table = {
        // The table must let the client delete it: a visible data set, deletable, in the VMD scope.
        BilateralTable::new(
            "remote-cc",
            Versions {
                table_id: VisibleString32::new("BT-1").unwrap(),
                tase2: STANDARD_VERSION,
            },
            "ICC1",
        )
        .with_data_set(
            vmd("DSVmd"),
            access(&[Permission::Visibility, Permission::DeleteDataSet]),
        )
    };
    assert_eq!(
        vcc.delete_data_sets(&table, &[vmd("DSVmd")], &BTreeSet::new()),
        vec![Err(ErrorCode::ObjectAttributeInconsistent)]
    );
}

#[test]
fn get_data_set_element_values_gives_one_item_per_member() {
    let vcc = server();
    let items = vcc
        .get_data_set_element_values(&table(false), &icc("DS1"))
        .unwrap();
    assert_eq!(
        items,
        vec![
            (icc("Flow1"), Item::Success(Data::Integer(5))),
            (icc("ReadOnly"), Item::Success(Data::Integer(1))),
        ]
    );
}

#[test]
fn get_data_set_element_values_refuses_a_special_or_unreadable_member_per_item() {
    let vcc = server();
    let items = vcc
        .get_data_set_element_values(&table(false), &icc("DSSpecial"))
        .unwrap();
    assert_eq!(
        items,
        vec![
            (icc("Flow1"), Item::Success(Data::Integer(5))),
            (
                ObjectName {
                    scope: Scope::Domain("ICC1".to_string()),
                    name: "Transfer_Set_Name".to_string(),
                },
                Item::Failure(ErrorCode::ObjectAccessDenied),
            ),
        ]
    );
}

#[test]
fn get_data_set_element_values_fails_for_a_missing_or_unpermitted_data_set() {
    let vcc = server();
    let table = table(false);
    assert_eq!(
        vcc.get_data_set_element_values(&table, &icc("Missing")),
        Err(ErrorCode::ObjectNonExistent)
    );
    // DS2 is visible and its elements may be read: the call succeeds.
    assert!(vcc.get_data_set_element_values(&table, &icc("DS2")).is_ok());
    let without = BilateralTable::new(
        "remote-cc",
        Versions {
            table_id: VisibleString32::new("BT-1").unwrap(),
            tase2: STANDARD_VERSION,
        },
        "ICC1",
    )
    .with_data_set(icc("DS1"), access(&[Permission::Visibility]));
    assert_eq!(
        vcc.get_data_set_element_values(&without, &icc("DS1")),
        Err(ErrorCode::ObjectAccessDenied)
    );
}

#[test]
fn set_data_set_element_values_writes_each_permitted_member_in_order() {
    let mut vcc = server();
    let items = vcc
        .set_data_set_element_values(
            &table(false),
            &icc("DS1"),
            vec![Data::Integer(9), Data::Integer(2)],
        )
        .unwrap();
    assert_eq!(
        items,
        vec![
            Item::Success(()),
            Item::Failure(ErrorCode::ObjectAccessDenied)
        ]
    );
    assert_eq!(
        vcc.store().values.get(&icc("Flow1")),
        Some(&Data::Integer(9))
    );
    assert_eq!(
        vcc.store().values.get(&icc("ReadOnly")),
        Some(&Data::Integer(1))
    );
}

#[test]
fn set_data_set_element_values_refuses_a_list_of_another_length() {
    let mut vcc = server();
    assert_eq!(
        vcc.set_data_set_element_values(&table(false), &icc("DS1"), vec![Data::Integer(9)]),
        Err(ErrorCode::ObjectAttributeInconsistent)
    );
    assert_eq!(
        vcc.store().values.get(&icc("Flow1")),
        Some(&Data::Integer(5))
    );
}

#[test]
fn set_data_set_element_values_refuses_a_special_member_and_reports_a_server_failure() {
    let mut vcc = server();
    vcc.load_data_set(icc("DSFail"), vec![icc("Failing")]);
    let special = vcc
        .set_data_set_element_values(
            &table(false),
            &icc("DSSpecial"),
            vec![Data::Integer(8), Data::Integer(0)],
        )
        .unwrap();
    assert_eq!(
        special,
        vec![
            Item::Success(()),
            Item::Failure(ErrorCode::ObjectAccessDenied)
        ]
    );
    let failing = vcc
        .set_data_set_element_values(&table(false), &icc("DSFail"), vec![Data::Integer(4)])
        .unwrap();
    assert_eq!(failing, vec![Item::Failure(ErrorCode::HardwareFault)]);
}

#[test]
fn a_bit_string_does_not_replace_an_integer() {
    // The store checks the kind of the value against the object: a bit string is refused for an integer.
    let mut vcc = server();
    let items = vcc.set_data_values(
        &table(false),
        vec![(
            icc("Flow1"),
            Data::BitString(BitString {
                unused: 0,
                bytes: vec![0],
            }),
        )],
    );
    assert_eq!(items, vec![Item::Failure(ErrorCode::TypeInconsistent)]);
}
