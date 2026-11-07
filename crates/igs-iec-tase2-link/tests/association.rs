// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The association (task B2): the client's checks after the MMS Initiate, with a matching and with a mismatching table;
//! the server's acceptance and refusals; the release of the transfer sets and devices.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::collections::BTreeSet;

use igs_iec_tase2_link::association::{
    client_after_initiate, release, server_on_initiate, Active, Decision, Decline, ServerPolicy,
    ServerVariables,
};
use igs_iec_tase2_link::bilateral::{
    Admission, BilateralTable, BilateralTables, ObjectName, Scope,
};
use igs_iec_tase2_link::version::{Conflict, Versions, STANDARD_VERSION};
use igs_iec_tase2_model::builtin::Integer16;
use igs_iec_tase2_model::generated::iso503::{SupportedFeatures, TASE2Version, VisibleString32};

fn agreed() -> Versions {
    Versions {
        table_id: VisibleString32::new("BT-2026-001").unwrap(),
        tase2: STANDARD_VERSION,
    }
}

fn tase2(major: i64, minor: i64) -> TASE2Version {
    TASE2Version {
        major_version_number: Integer16::new(major).unwrap(),
        minor_version_number: Integer16::new(minor).unwrap(),
    }
}

/// What a server answers to the client's reads: the table identifier, the version and the features.
fn read(table_id: &str, version: TASE2Version) -> ServerVariables {
    ServerVariables {
        table_id: VisibleString32::new(table_id).unwrap(),
        tase2: version,
        supported_features: SupportedFeatures(0b1),
    }
}

fn tables() -> BilateralTables {
    let table =
        BilateralTable::new("remote-cc", agreed(), "ICC1").with_application_reference("ae-1");
    let mut tables = BilateralTables::new();
    tables.insert(table);
    tables
}

fn name(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

#[test]
fn a_matching_table_and_version_establish_the_association_with_the_server_features() {
    let established =
        client_after_initiate(&agreed(), &read("BT-2026-001", tase2(2000, 8))).unwrap();
    assert_eq!(established.supported_features, SupportedFeatures(0b1));
}

#[test]
fn a_mismatching_table_identifier_makes_the_client_conclude() {
    let outcome = client_after_initiate(&agreed(), &read("BT-2026-002", tase2(2000, 8)));
    assert_eq!(outcome, Err(Conflict::TableId));
}

#[test]
fn the_table_identifier_is_checked_before_the_version() {
    // Both differ: the conflict is the table identifier, the first check of the standard.
    let outcome = client_after_initiate(&agreed(), &read("BT-2026-002", tase2(1999, 1)));
    assert_eq!(outcome, Err(Conflict::TableId));
}

#[test]
fn a_mismatching_tase2_version_makes_the_client_conclude_when_the_table_matches() {
    let outcome = client_after_initiate(&agreed(), &read("BT-2026-001", tase2(2000, 9)));
    assert_eq!(outcome, Err(Conflict::Tase2Version));
}

#[test]
fn the_version_read_from_mms_is_the_one_compared() {
    // A value of TASE.2_Version is 16-bit integers by construction (the model refuses others on reading), so the
    // conversion keeps both numbers.
    assert_eq!(
        igs_iec_tase2_link::version::Tase2Version::from_mms(&tase2(2000, 8)),
        Some(STANDARD_VERSION)
    );
}

#[test]
fn a_known_client_with_a_listed_reference_is_accepted() {
    assert_eq!(
        server_on_initiate(&tables(), &ServerPolicy::new(), "remote-cc", "ae-1"),
        Decision::Accept
    );
}

#[test]
fn an_unknown_client_is_declined_with_a_negative_response() {
    assert_eq!(
        server_on_initiate(&tables(), &ServerPolicy::new(), "other-cc", "ae-1"),
        Decision::Refuse(Decline::Negative(Admission::UnknownClient))
    );
}

#[test]
fn an_unlisted_application_reference_is_declined_with_a_negative_response() {
    assert_eq!(
        server_on_initiate(&tables(), &ServerPolicy::new(), "remote-cc", "ae-9"),
        Decision::Refuse(Decline::Negative(Admission::UnknownApplicationReference))
    );
}

#[test]
fn a_server_that_is_the_designated_initiator_refuses_with_an_initiate_error() {
    // The table is valid, but this server is the initiator of the associations with this peer: an Initiate-ErrorPDU.
    let policy = ServerPolicy::new().with_initiator_for("remote-cc");
    assert_eq!(
        server_on_initiate(&tables(), &policy, "remote-cc", "ae-1"),
        Decision::Refuse(Decline::InitiateErrorOther)
    );
}

#[test]
fn the_release_disables_the_transfer_sets_and_deselects_the_devices() {
    let mut active = Active {
        transfer_sets: BTreeSet::from([name("TS1"), name("TS2")]),
        selected_devices: BTreeSet::from([name("Breaker12")]),
    };
    let released = release(&mut active);
    assert_eq!(
        released.disabled_transfer_sets,
        BTreeSet::from([name("TS1"), name("TS2")])
    );
    assert_eq!(
        released.deselected_devices,
        BTreeSet::from([name("Breaker12")])
    );
    assert_eq!(active, Active::default());
}

#[test]
fn a_second_release_has_nothing_left_to_release() {
    let mut active = Active {
        transfer_sets: BTreeSet::from([name("TS1")]),
        selected_devices: BTreeSet::new(),
    };
    let first = release(&mut active);
    assert_eq!(first.disabled_transfer_sets.len(), 1);
    let second = release(&mut active);
    assert!(second.disabled_transfer_sets.is_empty());
    assert!(second.deselected_devices.is_empty());
}
