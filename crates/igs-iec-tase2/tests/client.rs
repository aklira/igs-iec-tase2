// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The TASE.2 client (task T1), against an in-process server stub: the MMS server of `igs-iso9506-mms` with the variables of
//! a TASE.2 server (the Bilateral Table identifier, the TASE.2 version, the supported features, two data values, and a
//! report that the server publishes).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::time::Duration;

use igs_iec_tase2::{Agreement, Error, ReportAccess, TaseClient, TaseEvent, TaseEvents};
use igs_iec_tase2_link::bilateral::{ObjectName, Scope};
use igs_iec_tase2_link::data::{ErrorCode, Item};
use igs_iec_tase2_link::version::{Conflict, Tase2Version, Versions, STANDARD_VERSION};
use igs_iec_tase2_model::builtin::{Integer16, Integer32};
use igs_iec_tase2_model::generated::iso503::{
    DSConditions, DSTransferSet, DSTransferSetAllChangesReported, DSTransferSetBlockData,
    DSTransferSetCritical, DSTransferSetRBE, DSTransferSetStatus, MMSObjectName,
    MMSObjectNameScope, VisibleString32,
};
use igs_iso9506_mms::client::{in_scope_capabilities, ClientConfig, ConnectionState, Reconnect};
use igs_iso9506_mms::server::{Server, ServerConfig};
use igs_iso9506_mms::vmd::{Identity, MemoryVariable, Name, Target, Vmd};
use igs_iso9506_mms::Error as MmsError;
use igs_iso9506_mms_codec::generated::types::{
    AccessResult, Data, Identifier, InformationReport, Integer32 as CodecInteger32,
    ObjectName as MmsObjectName, ObjectNameDomainSpecific, TypeDescription,
    TypeDescriptionStructure, TypeDescriptionStructureComponentsItem, TypeSpecification,
    UnconfirmedService, Unsigned8, VariableAccessSpecification,
    VariableAccessSpecificationListOfVariableItem, VariableSpecification,
};
use igs_iso9506_mms_codec::schema::BitString;
use igs_iso9506_mms_link::config::LinkConfig;
use tokio::time::timeout;

const ANSWER: Duration = Duration::from_secs(10);

fn icc(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

fn agreement(table_id: &str, tase2: Tase2Version) -> Agreement {
    Agreement {
        domain: "ICC1".to_string(),
        versions: Versions {
            table_id: VisibleString32::new(table_id).unwrap(),
            tase2,
        },
    }
}

/// The variables of a TASE.2 server: the Bilateral Table identifier and the supported features, the TASE.2 version (a
/// structure of two integers), and two data values, one of them read-only.
fn stub() -> Vmd {
    let mut vmd = Vmd::new(Identity {
        vendor: "igs".to_string(),
        model: "tase2-stub".to_string(),
        revision: "1".to_string(),
    });
    vmd.add_domain(Name::new("ICC1").unwrap()).unwrap();
    let add = |vmd: &mut Vmd, target: Target, variable: MemoryVariable| {
        vmd.add_variable(&target, Box::new(variable)).unwrap();
    };
    let domain =
        |item: &str| Target::in_domain(Name::new("ICC1").unwrap(), Name::new(item).unwrap());
    let vmd_scope = |item: &str| Target::vmd(Name::new(item).unwrap());
    add(
        &mut vmd,
        domain("Bilateral_Table_ID"),
        MemoryVariable::new(
            // A negative size is a maximum in MMS: the identifier may be shorter than 32 characters (decision D-004).
            TypeDescription::VisibleString(CodecInteger32(-32)),
            Data::VisibleString("BT-2026-001".to_string()),
            false,
        )
        .unwrap(),
    );
    let component = |name: &str| TypeDescriptionStructureComponentsItem {
        component_name: Some(Identifier(name.to_string())),
        component_type: TypeSpecification::TypeDescription(Box::new(TypeDescription::Integer(
            Unsigned8(16),
        ))),
    };
    add(
        &mut vmd,
        vmd_scope("TASE2_Version"),
        MemoryVariable::new(
            TypeDescription::Structure(TypeDescriptionStructure {
                packed: None,
                components: vec![
                    component("MajorVersionNumber"),
                    component("MinorVersionNumber"),
                ],
            }),
            Data::Structure(vec![Data::Integer(2000), Data::Integer(8)]),
            false,
        )
        .unwrap(),
    );
    add(
        &mut vmd,
        vmd_scope("Supported_Features"),
        MemoryVariable::new(
            TypeDescription::BitString(CodecInteger32(12)),
            // Block1 (bit 0) is set, as the standard requires: 12 bits in two octets, four of them unused.
            Data::BitString(BitString {
                unused: 4,
                bytes: vec![0x80, 0x00],
            }),
            false,
        )
        .unwrap(),
    );
    add(
        &mut vmd,
        domain("Flow1"),
        MemoryVariable::new(
            TypeDescription::Integer(Unsigned8(32)),
            Data::Integer(5),
            true,
        )
        .unwrap(),
    );
    add(
        &mut vmd,
        domain("ReadOnly"),
        MemoryVariable::new(
            TypeDescription::Integer(Unsigned8(32)),
            Data::Integer(1),
            false,
        )
        .unwrap(),
    );
    vmd
}

async fn serving() -> Server {
    let config = ServerConfig::new(LinkConfig::default());
    Server::bind("127.0.0.1:0", config, stub()).await.unwrap()
}

fn client_config() -> ClientConfig {
    ClientConfig {
        reconnect: Reconnect::Never,
        ..ClientConfig::new(LinkConfig::default(), in_scope_capabilities())
    }
}

async fn associate(server: &Server, agreed: &Agreement) -> Result<(TaseClient, TaseEvents), Error> {
    let address = server.local_address().to_string();
    timeout(
        ANSWER,
        TaseClient::associate(&address, client_config(), agreed),
    )
    .await
    .expect("the association is answered in time")
}

#[tokio::test]
async fn the_association_is_established_with_a_matching_table_and_version() {
    let server = serving().await;
    let (client, _events) = associate(&server, &agreement("BT-2026-001", STANDARD_VERSION))
        .await
        .unwrap();
    assert_eq!(client.domain(), "ICC1");
    // The first building block of Supported_Features is set by the stub.
    assert_eq!(client.supported_features().0 & 1, 1, "Block1 is set");
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_mismatching_table_identifier_concludes_the_association() {
    let server = serving().await;
    let outcome = associate(&server, &agreement("BT-OTHER", STANDARD_VERSION)).await;
    assert!(
        matches!(outcome, Err(Error::Conflict(Conflict::TableId))),
        "{outcome:?}"
    );
    // The server is still serving: a matching client associates after the refused one.
    let (client, _events) = associate(&server, &agreement("BT-2026-001", STANDARD_VERSION))
        .await
        .unwrap();
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_mismatching_tase2_version_concludes_the_association() {
    let server = serving().await;
    let outcome = associate(
        &server,
        &agreement(
            "BT-2026-001",
            Tase2Version {
                major: 2000,
                minor: 9,
            },
        ),
    )
    .await;
    assert!(
        matches!(outcome, Err(Error::Conflict(Conflict::Tase2Version))),
        "{outcome:?}"
    );
    server.shutdown().await;
}

#[tokio::test]
async fn get_data_value_answers_each_item() {
    let server = serving().await;
    let (client, _events) = associate(&server, &agreement("BT-2026-001", STANDARD_VERSION))
        .await
        .unwrap();
    let items = client
        .get_data_values(&[icc("Flow1"), icc("Missing")])
        .await
        .unwrap();
    assert_eq!(
        items,
        vec![
            Item::Success(Data::Integer(5)),
            Item::Failure(ErrorCode::ObjectNonExistent)
        ]
    );
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn set_data_value_writes_a_writable_value_and_refuses_a_read_only_one() {
    let server = serving().await;
    let (client, _events) = associate(&server, &agreement("BT-2026-001", STANDARD_VERSION))
        .await
        .unwrap();
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
    assert_eq!(
        values[1],
        Item::Success(Data::Integer(1)),
        "the refused write changed nothing"
    );
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_data_set_is_created_read_written_and_deleted() {
    let server = serving().await;
    let (client, _events) = associate(&server, &agreement("BT-2026-001", STANDARD_VERSION))
        .await
        .unwrap();
    client
        .create_data_set(&icc("DS1"), &[icc("Flow1"), icc("ReadOnly")])
        .await
        .unwrap();
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
    assert_eq!(client.delete_data_sets(&[icc("DS1")]).await.unwrap(), 1);
    // Once deleted, the data set does not answer.
    assert!(client.get_data_set(&icc("DS1")).await.is_err());
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn a_server_refusal_of_a_definition_is_the_mms_error() {
    let server = serving().await;
    let (client, _events) = associate(&server, &agreement("BT-2026-001", STANDARD_VERSION))
        .await
        .unwrap();
    client
        .create_data_set(&icc("DS1"), &[icc("Flow1")])
        .await
        .unwrap();
    // A second data set of the same name exists: the server answers with an error.
    let refused = client.create_data_set(&icc("DS1"), &[icc("Flow1")]).await;
    assert!(
        matches!(refused, Err(Error::Mms(MmsError::Service(_)))),
        "{refused:?}"
    );
    client.close().await.unwrap();
    server.shutdown().await;
}

fn transfer_set() -> DSTransferSet {
    DSTransferSet {
        data_set_name: MMSObjectName {
            scope: MMSObjectNameScope(MMSObjectNameScope::DOMAIN_SPECIFIC),
            domain_name: igs_iec_tase2_model::builtin::MmsIdentifier::new("ICC1").unwrap(),
            name: igs_iec_tase2_model::builtin::MmsIdentifier::new("DS1").unwrap(),
        },
        start_time: Integer32::new(0).unwrap(),
        interval: igs_iec_tase2_model::builtin::Integer32::new(10).unwrap(),
        tle: igs_iec_tase2_model::builtin::Integer32::new(0).unwrap(),
        buffer_time: igs_iec_tase2_model::builtin::Integer32::new(0).unwrap(),
        integrity_check: igs_iec_tase2_model::builtin::Integer32::new(0).unwrap(),
        ds_conditions_requested: DSConditions(DSConditions::INTERVAL_TIME_OUT),
        block_data: DSTransferSetBlockData(false),
        critical: DSTransferSetCritical(false),
        rbe: DSTransferSetRBE(false),
        all_changes_reported: DSTransferSetAllChangesReported(false),
        status: DSTransferSetStatus(DSTransferSetStatus::DISABLED),
        event_code_requested: Integer16::new(0).unwrap(),
    }
}

#[tokio::test]
async fn a_transfer_set_that_the_server_does_not_hold_is_refused_per_item() {
    let server = serving().await;
    let (client, _events) = associate(&server, &agreement("BT-2026-001", STANDARD_VERSION))
        .await
        .unwrap();
    let started = client
        .start_transfer(&icc("TS1"), &transfer_set())
        .await
        .unwrap();
    assert!(matches!(started, Item::Failure(_)), "{started:?}");
    client.close().await.unwrap();
    server.shutdown().await;
}

async fn next_report(events: &mut TaseEvents) -> igs_iec_tase2::ReceivedReport {
    loop {
        match timeout(ANSWER, events.next())
            .await
            .expect("an event in time")
        {
            Some(Ok(TaseEvent::Report(report))) => return report,
            Some(Ok(TaseEvent::Connection(_))) => continue,
            other => panic!("an event was expected: {other:?}"),
        }
    }
}

#[tokio::test]
async fn a_transfer_report_of_the_server_is_received_as_an_event() {
    let server = serving().await;
    let (client, mut events) = associate(&server, &agreement("BT-2026-001", STANDARD_VERSION))
        .await
        .unwrap();
    let report = UnconfirmedService::InformationReport(InformationReport {
        variable_access_specification: VariableAccessSpecification::ListOfVariable(vec![
            VariableAccessSpecificationListOfVariableItem {
                variable_specification: VariableSpecification::Name(MmsObjectName::DomainSpecific(
                    ObjectNameDomainSpecific {
                        domain_id: Identifier("ICC1".to_string()),
                        item_id: Identifier("Flow1".to_string()),
                    },
                )),
                alternate_access: None,
            },
        ]),
        list_of_access_result: vec![AccessResult::Success(Data::Integer(12))],
    });
    assert_eq!(server.publish(report), 1);
    let received = next_report(&mut events).await;
    assert_eq!(received.access, ReportAccess::Names(vec![icc("Flow1")]));
    assert_eq!(received.values, vec![Item::Success(Data::Integer(12))]);
    client.close().await.unwrap();
    server.shutdown().await;
}

#[tokio::test]
async fn the_connection_state_is_an_event() {
    let server = serving().await;
    let (client, mut events) = associate(&server, &agreement("BT-2026-001", STANDARD_VERSION))
        .await
        .unwrap();
    let first = timeout(ANSWER, events.next())
        .await
        .expect("an event in time");
    assert!(
        matches!(
            first,
            Some(Ok(TaseEvent::Connection(ConnectionState::Connected)))
        ),
        "the first event is the connection: {first:?}"
    );
    client.close().await.unwrap();
    server.shutdown().await;
}
