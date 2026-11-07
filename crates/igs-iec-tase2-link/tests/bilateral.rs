// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The bilateral table (task B1): each access result, the admission of a client, and the checks of the versions.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::collections::BTreeSet;
use std::time::Duration;

use igs_iec_tase2_link::bilateral::{
    Access, Admission, BilateralTable, BilateralTables, DeviceAccess, Object, ObjectAccess,
    ObjectName, Permission, Scope,
};
use igs_iec_tase2_link::version::{
    check_versions, Conflict, Tase2Version, Versions, STANDARD_VERSION,
};
use igs_iec_tase2_model::generated::iso503::VisibleString32;
use igs_iec_tase2_model::generated::iso802::SBOCheckBackName;

fn permissions(list: &[Permission]) -> BTreeSet<Permission> {
    list.iter().copied().collect()
}

fn domain_name(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

fn versions(table_id: &str, tase2: Tase2Version) -> Versions {
    Versions {
        table_id: VisibleString32::new(table_id).unwrap(),
        tase2,
    }
}

/// The table of one remote control centre: a visible data value, a hidden one, a data set, a transfer set and two
/// devices.
fn table() -> BilateralTable {
    BilateralTable::new(
        "remote-cc",
        versions("BT-2026-001", STANDARD_VERSION),
        "ICC1",
    )
    .with_application_reference("ae-1")
    .with_data_value(
        domain_name("Flow1"),
        ObjectAccess {
            permitted: permissions(&[
                Permission::Visibility,
                Permission::GetDataValue,
                Permission::GetDataValueType,
            ]),
            minimum_interval: Some(Duration::from_secs(10)),
        },
    )
    .with_data_value(
        domain_name("Hidden"),
        ObjectAccess {
            permitted: permissions(&[Permission::GetDataValue]),
            minimum_interval: None,
        },
    )
    .with_data_set(
        domain_name("DS1"),
        ObjectAccess {
            permitted: permissions(&[Permission::Visibility, Permission::GetDataSetElementValues]),
            minimum_interval: None,
        },
    )
    .with_transfer_set(
        domain_name("TS1"),
        ObjectAccess {
            permitted: permissions(&[
                Permission::Visibility,
                Permission::StartTransfer,
                Permission::StopTransfer,
            ]),
            minimum_interval: Some(Duration::from_secs(60)),
        },
    )
    .with_device(
        domain_name("Breaker12"),
        DeviceAccess {
            permitted: permissions(&[
                Permission::Visibility,
                Permission::Select,
                Permission::Operate,
            ]),
            check_back_id: SBOCheckBackName::new(12).unwrap(),
            timeout: Duration::from_secs(5),
        },
    )
    .with_device(
        domain_name("Meter3"),
        DeviceAccess {
            permitted: permissions(&[Permission::Visibility]),
            check_back_id: SBOCheckBackName::new(99).unwrap(),
            timeout: Duration::from_secs(5),
        },
    )
}

fn data_value(item: &str) -> Object {
    Object::DataValue(domain_name(item))
}

#[test]
fn a_permitted_operation_on_a_visible_object_is_granted() {
    assert_eq!(
        table().check(&data_value("Flow1"), Permission::GetDataValue),
        Access::Granted
    );
    assert_eq!(
        table().check(&data_value("Flow1"), Permission::Visibility),
        Access::Granted
    );
}

#[test]
fn an_object_that_is_not_in_the_table_is_unknown() {
    assert_eq!(
        table().check(&data_value("Missing"), Permission::GetDataValue),
        Access::UnknownObject
    );
}

#[test]
fn an_object_that_is_not_visible_is_unknown_to_the_client() {
    // The table holds "Hidden" with the operation of reading, but without visibility: for the client it does not exist.
    assert_eq!(
        table().check(&data_value("Hidden"), Permission::GetDataValue),
        Access::UnknownObject
    );
}

#[test]
fn an_operation_outside_the_permitted_access_is_not_permitted() {
    assert_eq!(
        table().check(&data_value("Flow1"), Permission::SetDataValue),
        Access::NotPermitted(Permission::SetDataValue)
    );
}

#[test]
fn the_scope_of_a_name_is_part_of_it() {
    // The same identifier in the VMD-specific scope is another object: it is not in the table.
    let vmd = Object::DataValue(ObjectName {
        scope: Scope::Vmd,
        name: "Flow1".to_string(),
    });
    assert_eq!(
        table().check(&vmd, Permission::GetDataValue),
        Access::UnknownObject
    );
}

#[test]
fn an_interval_at_the_agreed_maximum_exchange_rate_is_granted() {
    let object = data_value("Flow1");
    assert_eq!(
        table().check_interval(&object, Duration::from_secs(10)),
        Access::Granted
    );
    assert_eq!(
        table().check_interval(&object, Duration::from_secs(30)),
        Access::Granted
    );
}

#[test]
fn an_interval_shorter_than_the_agreed_maximum_exchange_rate_is_out_of_agreement() {
    assert_eq!(
        table().check_interval(&data_value("Flow1"), Duration::from_millis(9_999)),
        Access::IntervalOutOfAgreement
    );
    let transfer = Object::TransferSet(domain_name("TS1"));
    assert_eq!(
        table().check_interval(&transfer, Duration::from_secs(59)),
        Access::IntervalOutOfAgreement
    );
}

#[test]
fn an_object_without_an_agreed_interval_accepts_any_interval() {
    // A data set has no agreed maximum exchange rate: even a zero interval is granted, once the data set is visible.
    let data_set = Object::DataSet(domain_name("DS1"));
    assert_eq!(
        table().check_interval(&data_set, Duration::ZERO),
        Access::Granted
    );
    // The data value "Hidden" is not visible: the interval is not checked for it.
    let hidden = Object::DataValue(domain_name("Hidden"));
    assert_eq!(
        table().check_interval(&hidden, Duration::ZERO),
        Access::UnknownObject
    );
}

#[test]
fn a_device_operation_within_its_timeout_is_granted() {
    let breaker = domain_name("Breaker12");
    assert_eq!(
        table().check_device_time(&breaker, Duration::from_secs(1)),
        Access::Granted
    );
    // At the timeout itself the operation is still within it.
    assert_eq!(
        table().check_device_time(&breaker, Duration::from_secs(5)),
        Access::Granted
    );
}

#[test]
fn a_device_operation_after_its_timeout_is_out_of_time() {
    assert_eq!(
        table().check_device_time(&domain_name("Breaker12"), Duration::from_millis(5_001)),
        Access::OutOfTime
    );
}

#[test]
fn a_device_that_is_not_in_the_table_is_unknown() {
    assert_eq!(
        table().check_device_time(&domain_name("Nowhere"), Duration::from_secs(1)),
        Access::UnknownObject
    );
}

#[test]
fn the_check_back_id_is_returned_for_a_selectable_device_only() {
    assert_eq!(
        table().check_back_id(&domain_name("Breaker12")),
        Some(SBOCheckBackName::new(12).unwrap())
    );
    // Meter3 is visible but has no Select: the CheckBackID is not returned to the client.
    assert_eq!(table().check_back_id(&domain_name("Meter3")), None);
    assert_eq!(table().check_back_id(&domain_name("Nowhere")), None);
}

#[test]
fn a_data_set_and_a_transfer_set_are_checked_by_their_own_permissions() {
    assert_eq!(
        table().check(
            &Object::DataSet(domain_name("DS1")),
            Permission::GetDataSetElementValues
        ),
        Access::Granted
    );
    assert_eq!(
        table().check(
            &Object::TransferSet(domain_name("TS1")),
            Permission::StartTransfer
        ),
        Access::Granted
    );
    assert_eq!(
        table().check(&Object::TransferSet(domain_name("TS1")), Permission::Select),
        Access::NotPermitted(Permission::Select)
    );
}

#[test]
fn a_client_with_a_table_and_a_listed_reference_is_admitted() {
    let mut tables = BilateralTables::new();
    tables.insert(table());
    assert_eq!(tables.admit("remote-cc", "ae-1"), Admission::Granted);
}

#[test]
fn a_client_without_a_table_is_refused_as_an_unknown_client() {
    let mut tables = BilateralTables::new();
    tables.insert(table());
    assert_eq!(tables.admit("other-cc", "ae-1"), Admission::UnknownClient);
    assert_eq!(
        BilateralTables::new().admit("remote-cc", "ae-1"),
        Admission::UnknownClient
    );
}

#[test]
fn a_reference_that_is_not_listed_is_refused() {
    let mut tables = BilateralTables::new();
    tables.insert(table());
    assert_eq!(
        tables.admit("remote-cc", "ae-unknown"),
        Admission::UnknownApplicationReference
    );
}

#[test]
fn a_table_for_the_same_designation_replaces_the_previous_one() {
    let mut tables = BilateralTables::new();
    tables.insert(table());
    tables.insert(
        BilateralTable::new(
            "remote-cc",
            versions("BT-2026-002", STANDARD_VERSION),
            "ICC1",
        )
        .with_application_reference("ae-2"),
    );
    assert_eq!(
        tables.admit("remote-cc", "ae-1"),
        Admission::UnknownApplicationReference
    );
    assert_eq!(tables.admit("remote-cc", "ae-2"), Admission::Granted);
}

#[test]
fn identical_versions_are_compatible() {
    let agreed = versions("BT-2026-001", STANDARD_VERSION);
    let read = versions("BT-2026-001", STANDARD_VERSION);
    assert_eq!(check_versions(&agreed, &read), Ok(()));
}

#[test]
fn a_different_table_identifier_is_the_first_conflict() {
    let agreed = versions("BT-2026-001", STANDARD_VERSION);
    let read = versions("BT-2026-002", STANDARD_VERSION);
    assert_eq!(check_versions(&agreed, &read), Err(Conflict::TableId));
}

#[test]
fn a_different_tase2_version_is_a_conflict_when_the_table_identifiers_match() {
    let agreed = versions("BT-2026-001", STANDARD_VERSION);
    let read = versions(
        "BT-2026-001",
        Tase2Version {
            major: 2000,
            minor: 9,
        },
    );
    assert_eq!(check_versions(&agreed, &read), Err(Conflict::Tase2Version));
}

#[test]
fn the_table_identifier_is_checked_before_the_tase2_version() {
    // Both differ: the table identifier is reported, as the first check of the standard.
    let agreed = versions("BT-2026-001", STANDARD_VERSION);
    let read = versions(
        "BT-2026-002",
        Tase2Version {
            major: 1999,
            minor: 1,
        },
    );
    assert_eq!(check_versions(&agreed, &read), Err(Conflict::TableId));
}

#[test]
fn the_standard_version_is_2000_08_and_the_table_knows_its_identifier() {
    assert_eq!(
        STANDARD_VERSION,
        Tase2Version {
            major: 2000,
            minor: 8
        }
    );
    let table = table();
    assert_eq!(table.designation(), "remote-cc");
    assert_eq!(table.domain(), "ICC1");
    assert!(table.has_table_id(&VisibleString32::new("BT-2026-001").unwrap()));
    assert!(!table.has_table_id(&VisibleString32::new("BT-2026-002").unwrap()));
}
