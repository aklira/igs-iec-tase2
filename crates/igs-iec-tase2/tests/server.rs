// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The TASE.2 server (task S1), with the TASE.2 client over TCP: each operation of the server is tested through the client,
//! for a client whose table allows it and for one whose table does not.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::collections::BTreeSet;
use std::time::Duration;

use igs_iec_tase2::{
    Agreement, Error, TaseClient, TaseEvent, TaseEvents, TaseServer, TaseServerConfig,
};
use igs_iec_tase2_link::association::ServerPolicy;
use igs_iec_tase2_link::bilateral::{
    BilateralTable, BilateralTables, DeviceAccess, ObjectAccess, ObjectName, Permission, Scope,
};
use igs_iec_tase2_link::data::{ErrorCode, Item};
use igs_iec_tase2_link::transfer::Policy;
use igs_iec_tase2_link::version::{Conflict, Versions, STANDARD_VERSION};
use igs_iec_tase2_model::builtin::{Integer16, Integer32, MmsIdentifier};
use igs_iec_tase2_model::generated::iso503::{
    DSConditions, DSTransferSet, DSTransferSetAllChangesReported, DSTransferSetBlockData,
    DSTransferSetCritical, DSTransferSetRBE, DSTransferSetStatus, MMSObjectName,
    MMSObjectNameScope, VisibleString32,
};
use igs_iso9506_mms::client::{in_scope_capabilities, ClientConfig, Reconnect};
use igs_iso9506_mms::server::ServerConfig;
use igs_iso9506_mms::vmd::{Identity, MemoryVariable, Name, Target, Vmd};
use igs_iso9506_mms_codec::generated::types::{Data, TypeDescription, Unsigned8};
use igs_iso9506_mms_link::config::LinkConfig;
use tokio::time::timeout;

const ANSWER: Duration = Duration::from_secs(10);
/// The AP-title of the client, and the designation of its table.
const CLIENT: [u64; 4] = [1, 3, 9999, 1];
const DESIGNATION: &str = "1.3.9999.1";

fn icc(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

fn access(list: &[Permission]) -> ObjectAccess {
    ObjectAccess {
        permitted: list.iter().copied().collect(),
        minimum_interval: None,
    }
}

/// The Bilateral Table of the client: the data values, the data set DS1 and DS2, the transfer set TS1 and a device.
fn table(table_id: &str) -> BilateralTable {
    BilateralTable::new(
        DESIGNATION,
        Versions {
            table_id: VisibleString32::new(table_id).unwrap(),
            tase2: STANDARD_VERSION,
        },
        "ICC1",
    )
    .with_application_reference(DESIGNATION)
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
    .with_data_value(icc("Hidden"), access(&[Permission::GetDataValue]))
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
        access(&[Permission::Visibility, Permission::DeleteDataSet]),
    )
    .with_data_set(
        icc("DS3"),
        access(&[Permission::Visibility, Permission::GetDataSetElementValues]),
    )
    .with_transfer_set(
        icc("TS1"),
        access(&[
            Permission::Visibility,
            Permission::StartTransfer,
            Permission::StopTransfer,
        ]),
    )
    .with_device(
        icc("Breaker12"),
        DeviceAccess {
            permitted: BTreeSet::from([Permission::Visibility, Permission::Select]),
            check_back_id: "cb".to_string(),
            timeout: Duration::from_secs(5),
        },
    )
    .with_data_set_creation()
}

/// The server's VMD: the data values, in the domain ICC1.
fn vmd() -> Vmd {
    let mut vmd = Vmd::new(Identity {
        vendor: "igs".to_string(),
        model: "tase2-server".to_string(),
        revision: "1".to_string(),
    });
    vmd.add_domain(Name::new("ICC1").unwrap()).unwrap();
    let add = |vmd: &mut Vmd, item: &str, value: i128, writable: bool| {
        let target = Target::in_domain(Name::new("ICC1").unwrap(), Name::new(item).unwrap());
        let variable = MemoryVariable::new(
            TypeDescription::Integer(Unsigned8(32)),
            Data::Integer(value),
            writable,
        )
        .unwrap();
        vmd.add_variable(&target, Box::new(variable)).unwrap();
    };
    add(&mut vmd, "Flow1", 5, true);
    add(&mut vmd, "ReadOnly", 1, false);
    add(&mut vmd, "Hidden", 0, true);
    vmd
}

fn config(table_id: &str) -> TaseServerConfig {
    TaseServerConfig {
        mms: ServerConfig {
            poll_interval: Duration::from_millis(10),
            ..ServerConfig::new(LinkConfig::default())
        },
        tables: {
            let mut tables = BilateralTables::new();
            tables.insert(table(table_id));
            tables
        },
        policy: ServerPolicy::new(),
        transfer_sets: BTreeSet::from([icc("TS1")]),
        data_sets: vec![
            (icc("DS1"), vec![icc("Flow1"), icc("ReadOnly")]),
            (icc("DS2"), vec![icc("Flow1")]),
        ],
        data_set_capacity: 4,
        transfer: Policy {
            expected_send_delay: Duration::ZERO,
            ack_timeout: Duration::from_secs(30),
        },
    }
}

async fn serving(table_id: &str) -> TaseServer {
    TaseServer::bind("127.0.0.1:0", config(table_id), vmd())
        .await
        .unwrap()
}

fn client_config(ap_title: Vec<u64>) -> ClientConfig {
    let link = LinkConfig {
        ap_title: Some(igs_iso9506_mms_codec::schema::ObjectIdentifier(ap_title)),
        ..LinkConfig::default()
    };
    ClientConfig {
        reconnect: Reconnect::Never,
        ..ClientConfig::new(link, in_scope_capabilities())
    }
}

fn agreement(table_id: &str) -> Agreement {
    Agreement {
        domain: "ICC1".to_string(),
        versions: Versions {
            table_id: VisibleString32::new(table_id).unwrap(),
            tase2: STANDARD_VERSION,
        },
    }
}

async fn associate(
    server: &TaseServer,
    ap_title: Vec<u64>,
    agreed: &Agreement,
) -> Result<(TaseClient, TaseEvents), Error> {
    let address = server.local_address().to_string();
    timeout(
        ANSWER,
        TaseClient::associate(&address, client_config(ap_title), agreed),
    )
    .await
    .expect("the association is answered in time")
}

async fn client(server: &TaseServer) -> (TaseClient, TaseEvents) {
    associate(server, CLIENT.to_vec(), &agreement("BT-2026-001"))
        .await
        .unwrap()
}

fn transfer_set(data_set: &str, conditions: u64, rbe: bool) -> DSTransferSet {
    DSTransferSet {
        data_set_name: MMSObjectName {
            scope: MMSObjectNameScope(MMSObjectNameScope::DOMAIN_SPECIFIC),
            domain_name: MmsIdentifier::new("ICC1").unwrap(),
            name: MmsIdentifier::new(data_set).unwrap(),
        },
        start_time: Integer32::new(0).unwrap(),
        interval: Integer32::new(0).unwrap(),
        tle: Integer32::new(0).unwrap(),
        buffer_time: Integer32::new(0).unwrap(),
        integrity_check: Integer32::new(0).unwrap(),
        ds_conditions_requested: DSConditions(conditions),
        block_data: DSTransferSetBlockData(false),
        critical: DSTransferSetCritical(false),
        rbe: DSTransferSetRBE(rbe),
        all_changes_reported: DSTransferSetAllChangesReported(false),
        status: DSTransferSetStatus(DSTransferSetStatus::ENABLED),
        event_code_requested: Integer16::new(0).unwrap(),
    }
}

/// The next transfer report of the events, within the answer time.
async fn next_report(events: &mut TaseEvents) -> igs_iec_tase2::ReceivedReport {
    loop {
        match timeout(ANSWER, events.next())
            .await
            .expect("an event in time")
        {
            Some(Ok(TaseEvent::Report(report))) => return report,
            Some(Ok(TaseEvent::Connection(_))) => continue,
            other => panic!("a report was expected: {other:?}"),
        }
    }
}

#[tokio::test]
async fn the_association_is_established_with_the_special_objects_of_the_server() {
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    assert_eq!(client.supported_features().0 & 1, 1, "Block1 is served");
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_client_with_another_table_identifier_is_concluded() {
    let server = serving("BT-2026-001").await;
    let outcome = associate(&server, CLIENT.to_vec(), &agreement("BT-2026-002")).await;
    assert!(
        matches!(outcome, Err(Error::Conflict(Conflict::TableId))),
        "{outcome:?}"
    );
    server.shutdown().await;
}

#[tokio::test]
async fn a_client_without_a_table_is_refused_the_association() {
    let server = serving("BT-2026-001").await;
    let outcome = associate(&server, vec![1, 3, 9999, 2], &agreement("BT-2026-001")).await;
    assert!(outcome.is_err(), "the unknown client is refused");
    server.shutdown().await;
}

#[tokio::test]
async fn get_data_value_answers_what_the_table_allows() {
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    let items = client
        .get_data_values(&[icc("Flow1"), icc("Hidden"), icc("Missing")])
        .await
        .unwrap();
    assert_eq!(items[0], Item::Success(Data::Integer(5)));
    assert_eq!(
        items[1],
        Item::Failure(ErrorCode::ObjectNonExistent),
        "Hidden is not visible"
    );
    assert_eq!(items[2], Item::Failure(ErrorCode::ObjectNonExistent));
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn set_data_value_writes_what_the_table_allows_only() {
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    let items = client
        .set_data_values(vec![
            (icc("Flow1"), Data::Integer(9)),
            (icc("ReadOnly"), Data::Integer(7)),
        ])
        .await
        .unwrap();
    assert_eq!(items[0], Item::Success(()));
    assert!(
        matches!(items[1], Item::Failure(_)),
        "the read-only value is refused: {items:?}"
    );
    let values = client
        .get_data_values(&[icc("Flow1"), icc("ReadOnly")])
        .await
        .unwrap();
    assert_eq!(values[0], Item::Success(Data::Integer(9)));
    assert_eq!(values[1], Item::Success(Data::Integer(1)));
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn get_data_value_type_describes_a_readable_value_only() {
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    let ty = client.get_data_value_type(&icc("Flow1")).await.unwrap();
    assert_eq!(ty, TypeDescription::Integer(Unsigned8(32)));
    assert!(
        client.get_data_value_type(&icc("Hidden")).await.is_err(),
        "Hidden is not visible"
    );
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_data_set_is_read_written_and_deleted_through_the_table() {
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    // DS1 was configured: its members are read as the table allows (Flow1 readable and writable, ReadOnly readable).
    assert_eq!(
        client.data_set_members(&icc("DS1")).await.unwrap(),
        vec![icc("Flow1"), icc("ReadOnly")]
    );
    let values = client.get_data_set(&icc("DS1")).await.unwrap();
    assert_eq!(
        values,
        vec![
            Item::Success(Data::Integer(5)),
            Item::Success(Data::Integer(1))
        ]
    );
    let written = client
        .set_data_set(&icc("DS1"), vec![Data::Integer(3), Data::Integer(0)])
        .await
        .unwrap();
    assert_eq!(written[0], Item::Success(()));
    assert!(matches!(written[1], Item::Failure(_)));
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_data_set_is_created_by_the_client_and_refused_when_it_exists() {
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    client
        .create_data_set(&icc("DS3"), &[icc("Flow1")])
        .await
        .unwrap();
    assert_eq!(
        client.data_set_members(&icc("DS3")).await.unwrap(),
        vec![icc("Flow1")]
    );
    // A second definition of the same name is refused by the server.
    assert!(client
        .create_data_set(&icc("DS3"), &[icc("Flow1")])
        .await
        .is_err());
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_data_set_the_client_created_is_visible_to_it_and_deletable_by_it() {
    // DS4 is not in the table: the client that creates it is granted it (decision D-055).
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    client
        .create_data_set(&icc("DS4"), &[icc("Flow1")])
        .await
        .unwrap();
    assert_eq!(
        client.data_set_members(&icc("DS4")).await.unwrap(),
        vec![icc("Flow1")]
    );
    assert_eq!(client.delete_data_sets(&[icc("DS4")]).await.unwrap(), 1);
    assert!(
        client.data_set_members(&icc("DS4")).await.is_err(),
        "DS4 is gone"
    );
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_data_set_is_deleted_when_the_table_allows_it_and_an_unknown_one_is_refused() {
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    assert!(client.delete_data_sets(&[icc("Unknown")]).await.is_err());
    assert_eq!(client.delete_data_sets(&[icc("DS2")]).await.unwrap(), 1);
    assert!(
        client.data_set_members(&icc("DS2")).await.is_err(),
        "DS2 is gone"
    );
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_change_of_a_monitored_value_is_reported_to_its_association() {
    let server = serving("BT-2026-001").await;
    let (client, mut events) = client(&server).await;
    let set = transfer_set("DS1", DSConditions::OBJECT_CHANGE, false);
    assert_eq!(
        client.start_transfer(&icc("TS1"), &set).await.unwrap(),
        Item::Success(())
    );
    server
        .set_value(icc("Flow1"), Data::Integer(42))
        .await
        .unwrap();
    let report = next_report(&mut events).await;
    assert_eq!(
        report.values,
        vec![
            Item::Success(Data::Integer(42)),
            Item::Success(Data::Integer(1))
        ],
        "the entire data set, without RBE"
    );
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_report_reaches_only_the_association_that_started_its_transfer_set() {
    // Two associations at once (the load run): only the first starts TS1, so only the first receives the report.
    let server = serving("BT-2026-001").await;
    let (first, mut first_events) = client(&server).await;
    let (second, mut second_events) = client(&server).await;
    let set = transfer_set("DS1", DSConditions::OBJECT_CHANGE, false);
    assert_eq!(
        first.start_transfer(&icc("TS1"), &set).await.unwrap(),
        Item::Success(())
    );
    server
        .set_value(icc("Flow1"), Data::Integer(43))
        .await
        .unwrap();
    let report = next_report(&mut first_events).await;
    assert_eq!(report.values[0], Item::Success(Data::Integer(43)));
    // The second association sees no report within the answer time (its own events may carry only connection events).
    let quiet = timeout(ANSWER, async {
        loop {
            match second_events.next().await {
                Some(Ok(TaseEvent::Report(report))) => return Some(report),
                Some(Ok(TaseEvent::Connection(_))) => continue,
                _ => return None,
            }
        }
    })
    .await;
    assert!(
        !matches!(quiet, Ok(Some(_))),
        "the second association received a report it did not start"
    );
    first.close().await.unwrap();
    second.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_report_by_exception_carries_the_changed_member_only() {
    let server = serving("BT-2026-001").await;
    let (client, mut events) = client(&server).await;
    let set = transfer_set("DS1", DSConditions::OBJECT_CHANGE, true);
    assert_eq!(
        client.start_transfer(&icc("TS1"), &set).await.unwrap(),
        Item::Success(())
    );
    server
        .set_value(icc("Flow1"), Data::Integer(43))
        .await
        .unwrap();
    let report = next_report(&mut events).await;
    assert_eq!(report.values, vec![Item::Success(Data::Integer(43))]);
    assert_eq!(
        report.access,
        igs_iec_tase2::ReportAccess::Names(vec![icc("Flow1")]),
        "the changed member is named"
    );
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_change_the_store_refuses_is_neither_kept_nor_reported() {
    let server = serving("BT-2026-001").await;
    let (client, mut events) = client(&server).await;
    let set = transfer_set("DS1", DSConditions::OBJECT_CHANGE, false);
    client.start_transfer(&icc("TS1"), &set).await.unwrap();
    // ReadOnly is not writable: the application's change is refused by the store, so no report follows.
    server
        .set_value(icc("ReadOnly"), Data::Integer(8))
        .await
        .unwrap();
    let quiet = timeout(Duration::from_millis(300), async {
        loop {
            match events.next().await {
                Some(Ok(TaseEvent::Report(report))) => return Some(report),
                Some(_) => continue,
                None => return None,
            }
        }
    })
    .await;
    assert!(quiet.is_err(), "no report for a refused change: {quiet:?}");
    assert_eq!(
        client.get_data_values(&[icc("ReadOnly")]).await.unwrap()[0],
        Item::Success(Data::Integer(1))
    );
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_stopped_transfer_set_sends_no_further_report() {
    let server = serving("BT-2026-001").await;
    let (client, mut events) = client(&server).await;
    let set = transfer_set("DS1", DSConditions::OBJECT_CHANGE, false);
    client.start_transfer(&icc("TS1"), &set).await.unwrap();
    server
        .set_value(icc("Flow1"), Data::Integer(44))
        .await
        .unwrap();
    let _ = next_report(&mut events).await;
    let mut stopped = set.clone();
    stopped.status = DSTransferSetStatus(DSTransferSetStatus::DISABLED);
    assert_eq!(
        client.stop_transfer(&icc("TS1"), &stopped).await.unwrap(),
        Item::Success(())
    );
    server
        .set_value(icc("Flow1"), Data::Integer(45))
        .await
        .unwrap();
    let quiet = timeout(Duration::from_millis(400), async {
        loop {
            match events.next().await {
                Some(Ok(TaseEvent::Report(report))) => return Some(report),
                Some(_) => continue,
                None => return None,
            }
        }
    })
    .await;
    assert!(quiet.is_err(), "no report after the stop: {quiet:?}");
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_transfer_set_the_server_does_not_hold_fails_per_item() {
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    let set = transfer_set("DS1", DSConditions::OBJECT_CHANGE, false);
    let started = client.start_transfer(&icc("TS9"), &set).await.unwrap();
    assert_eq!(started, Item::Failure(ErrorCode::ObjectNonExistent));
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_transfer_set_on_a_data_set_the_table_does_not_show_is_refused() {
    let server = serving("BT-2026-001").await;
    let (client, _events) = client(&server).await;
    let set = transfer_set("DSMissing", DSConditions::OBJECT_CHANGE, false);
    let started = client.start_transfer(&icc("TS1"), &set).await.unwrap();
    assert_eq!(started, Item::Failure(ErrorCode::ObjectNonExistent));
    client.close().await.unwrap();
    server.shutdown().await;
}
