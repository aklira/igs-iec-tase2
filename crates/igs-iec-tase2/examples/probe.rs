// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! A probe of a TASE.2 server, for the bench and by hand: it associates as the client of the demonstration table
//! (AP-title 1.3.9999.1), reads the points of the demonstration data set, starts its transfer set, and prints the reports
//! it receives. Usage: `cargo run -p igs-iec-tase2 --example probe -- 127.0.0.1:10103`.

use std::time::Duration;

use igs_iec_tase2::{Agreement, TaseClient, TaseEvent};
use igs_iec_tase2_link::bilateral::{ObjectName, Scope};
use igs_iec_tase2_link::version::{Versions, STANDARD_VERSION};
use igs_iec_tase2_model::builtin::MmsIdentifier;
use igs_iec_tase2_model::builtin::{Integer16, Integer32};
use igs_iec_tase2_model::generated::iso503::{
    DSConditions, DSTransferSet, DSTransferSetAllChangesReported, DSTransferSetBlockData,
    DSTransferSetCritical, DSTransferSetRBE, DSTransferSetStatus, MMSObjectName,
    MMSObjectNameScope, VisibleString32,
};
use igs_iso9506_mms::client::{in_scope_capabilities, ClientConfig, Reconnect};
use igs_iso9506_mms_codec::schema::ObjectIdentifier;
use igs_iso9506_mms_link::config::LinkConfig;

fn icc(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:10103".to_string());
    let config = ClientConfig {
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
            table_id: VisibleString32::new("BT-2026-001")?,
            tase2: STANDARD_VERSION,
        },
    };
    let (client, mut events) = TaseClient::associate(&address, config, &agreement).await?;
    println!(
        "associated with {address}; supported features: {:#x}",
        client.supported_features().0
    );

    let names = [
        icc("Flow1"),
        icc("Level"),
        icc("Temperature"),
        icc("Breaker"),
    ];
    for (name, item) in names.iter().zip(client.get_data_values(&names).await?) {
        println!("{}: {item:?}", name.name);
    }

    let set = DSTransferSet {
        data_set_name: MMSObjectName {
            scope: MMSObjectNameScope(MMSObjectNameScope::DOMAIN_SPECIFIC),
            domain_name: MmsIdentifier::new("ICC1")?,
            name: MmsIdentifier::new("DS1")?,
        },
        start_time: Integer32::new(0)?,
        interval: Integer32::new(0)?,
        tle: Integer32::new(0)?,
        buffer_time: Integer32::new(0)?,
        integrity_check: Integer32::new(0)?,
        ds_conditions_requested: DSConditions(DSConditions::OBJECT_CHANGE),
        block_data: DSTransferSetBlockData(false),
        critical: DSTransferSetCritical(false),
        rbe: DSTransferSetRBE(false),
        all_changes_reported: DSTransferSetAllChangesReported(false),
        status: DSTransferSetStatus(DSTransferSetStatus::ENABLED),
        event_code_requested: Integer16::new(0)?,
    };
    println!(
        "start transfer TS1: {:?}",
        client.start_transfer(&icc("TS1"), &set).await?
    );

    let mut reports: u32 = 0;
    let deadline = tokio::time::Instant::now()
        .checked_add(Duration::from_secs(10))
        .unwrap_or_else(tokio::time::Instant::now);
    while reports < 3 && tokio::time::Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_secs(10), events.next()).await {
            Ok(Some(Ok(TaseEvent::Report(report)))) => {
                reports = reports.saturating_add(1);
                println!("report {reports}: {:?}", report.values);
            }
            Ok(Some(Ok(TaseEvent::Connection(state)))) => println!("connection: {state:?}"),
            Ok(Some(Err(error))) => println!("event error: {error:?}"),
            _ => break,
        }
    }
    client.close().await?;
    Ok(())
}
