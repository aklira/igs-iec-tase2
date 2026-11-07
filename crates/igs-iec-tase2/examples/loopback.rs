// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! A self-contained demonstration: a TASE.2 server and its client in one process, over the loopback interface. The client
//! reads and writes a point, creates a data set, and receives a transfer report when the server changes the point.
//!
//! Run with `cargo run -p igs-iec-tase2 --example loopback`.

use std::collections::BTreeSet;
use std::time::Duration;

use igs_iec_tase2::{Agreement, TaseClient, TaseEvent, TaseServer, TaseServerConfig};
use igs_iec_tase2_link::association::ServerPolicy;
use igs_iec_tase2_link::bilateral::{
    BilateralTable, BilateralTables, ObjectAccess, ObjectName, Permission, Scope,
};
use igs_iec_tase2_link::transfer::Policy;
use igs_iec_tase2_link::version::{Versions, STANDARD_VERSION};
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
use igs_iso9506_mms_codec::schema::ObjectIdentifier;
use igs_iso9506_mms_link::config::LinkConfig;

/// The errors of the MMS stack do not implement `std::error::Error`: they are shown with their debug form.
trait Ctx<T> {
    fn ctx(self) -> Result<T, String>;
}

impl<T, E: std::fmt::Debug> Ctx<T> for Result<T, E> {
    fn ctx(self) -> Result<T, String> {
        self.map_err(|e| format!("{e:?}"))
    }
}

fn icc(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

fn access(list: &[Permission]) -> ObjectAccess {
    ObjectAccess {
        permitted: list.iter().copied().collect::<BTreeSet<_>>(),
        minimum_interval: None,
    }
}

/// The server: one point (Flow1, writable), and the table of one client (AP-title 1.3.9999.1).
fn server_config() -> Result<(TaseServerConfig, Vmd), Box<dyn std::error::Error>> {
    let mut vmd = Vmd::new(Identity {
        vendor: "igs".to_string(),
        model: "loopback".to_string(),
        revision: "1".to_string(),
    });
    vmd.add_domain(Name::new("ICC1").ctx()?).ctx()?;
    let target = Target::in_domain(Name::new("ICC1").ctx()?, Name::new("Flow1").ctx()?);
    vmd.add_variable(
        &target,
        Box::new(
            MemoryVariable::new(
                TypeDescription::Integer(Unsigned8(32)),
                Data::Integer(5),
                true,
            )
            .ctx()?,
        ),
    )
    .ctx()?;
    let table = BilateralTable::new(
        "1.3.9999.1",
        Versions {
            table_id: VisibleString32::new("BT-2026-001").ctx()?,
            tase2: STANDARD_VERSION,
        },
        "ICC1",
    )
    .with_application_reference("1.3.9999.1")
    .with_data_value(
        icc("Flow1"),
        access(&[
            Permission::Visibility,
            Permission::GetDataValue,
            Permission::SetDataValue,
        ]),
    )
    .with_transfer_set(
        icc("TS1"),
        access(&[Permission::Visibility, Permission::StartTransfer]),
    )
    .with_data_set_creation();
    let mut tables = BilateralTables::new();
    tables.insert(table);
    let config = TaseServerConfig {
        mms: ServerConfig {
            poll_interval: Duration::from_millis(10),
            ..ServerConfig::new(LinkConfig::default())
        },
        tables,
        policy: ServerPolicy::new(),
        transfer_sets: BTreeSet::from([icc("TS1")]),
        data_sets: Vec::new(),
        data_set_capacity: 8,
        devices: Vec::new(),
        transfer: Policy {
            expected_send_delay: Duration::ZERO,
            ack_timeout: Duration::from_secs(30),
        },
    };
    Ok((config, vmd))
}

/// The data set transfer set that the client writes to start the transfer.
fn transfer_set(data_set: &str) -> Result<DSTransferSet, Box<dyn std::error::Error>> {
    Ok(DSTransferSet {
        data_set_name: MMSObjectName {
            scope: MMSObjectNameScope(MMSObjectNameScope::DOMAIN_SPECIFIC),
            domain_name: MmsIdentifier::new("ICC1").ctx()?,
            name: MmsIdentifier::new(data_set).ctx()?,
        },
        start_time: Integer32::new(0).ctx()?,
        interval: Integer32::new(0).ctx()?,
        tle: Integer32::new(0).ctx()?,
        buffer_time: Integer32::new(0).ctx()?,
        integrity_check: Integer32::new(0).ctx()?,
        ds_conditions_requested: DSConditions(DSConditions::OBJECT_CHANGE),
        block_data: DSTransferSetBlockData(false),
        critical: DSTransferSetCritical(false),
        rbe: DSTransferSetRBE(false),
        all_changes_reported: DSTransferSetAllChangesReported(false),
        status: DSTransferSetStatus(DSTransferSetStatus::ENABLED),
        event_code_requested: Integer16::new(0).ctx()?,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config, vmd) = server_config().ctx()?;
    let server = TaseServer::bind("127.0.0.1:0", config, vmd).await?;
    let address = server.local_address().to_string();
    println!("server listening on {address}");

    let client_config = ClientConfig {
        reconnect: Reconnect::Never,
        ..ClientConfig::new(
            LinkConfig {
                ap_title: Some(ObjectIdentifier(vec![1, 3, 9999, 1])),
                ..LinkConfig::default()
            },
            in_scope_capabilities(),
        )
    };
    let agreement = Agreement {
        domain: "ICC1".to_string(),
        versions: Versions {
            table_id: VisibleString32::new("BT-2026-001").ctx()?,
            tase2: STANDARD_VERSION,
        },
    };
    let (client, mut events) = TaseClient::associate(&address, client_config, &agreement).await?;
    println!(
        "associated; the server's features are {:#x}",
        client.supported_features().0
    );

    println!(
        "read Flow1: {:?}",
        client.get_data_values(&[icc("Flow1")]).await?
    );
    println!(
        "write Flow1=9: {:?}",
        client
            .set_data_values(vec![(icc("Flow1"), Data::Integer(9))])
            .await?
    );

    client.create_data_set(&icc("DS1"), &[icc("Flow1")]).await?;
    println!(
        "data set DS1 members: {:?}",
        client.data_set_members(&icc("DS1")).await?
    );

    println!(
        "start TS1: {:?}",
        client
            .start_transfer(&icc("TS1"), &transfer_set("DS1").ctx()?)
            .await?
    );
    // The application changes the point; the transfer set reports the change to this association.
    server.set_value(icc("Flow1"), Data::Integer(12)).await?;
    loop {
        match tokio::time::timeout(Duration::from_secs(5), events.next()).await? {
            Some(Ok(TaseEvent::Report(report))) => {
                println!("report: {:?}", report.values);
                break;
            }
            Some(Ok(TaseEvent::Connection(_))) => continue,
            other => return Err(format!("no report: {other:?}").into()),
        }
    }

    client.close().await?;
    server.shutdown().await;
    Ok(())
}
