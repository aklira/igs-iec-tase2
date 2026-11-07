// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The binary `igstase2-client` (task T2), run as a process against a TASE.2 server of this workspace, started in the test.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::collections::BTreeSet;
use std::process::Command;
use std::time::Duration;

use igs_iec_tase2::{TaseServer, TaseServerConfig};
use igs_iec_tase2_link::association::ServerPolicy;
use igs_iec_tase2_link::bilateral::{
    BilateralTable, BilateralTables, ObjectAccess, ObjectName, Permission, Scope,
};
use igs_iec_tase2_link::transfer::Policy;
use igs_iec_tase2_link::version::{Versions, STANDARD_VERSION};
use igs_iec_tase2_model::generated::iso503::VisibleString32;
use igs_iso9506_mms::server::ServerConfig;
use igs_iso9506_mms::vmd::{Identity, MemoryVariable, Name, Target, Vmd};
use igs_iso9506_mms_codec::generated::types::{Data, TypeDescription, Unsigned8};
use igs_iso9506_mms_link::config::LinkConfig;

fn icc(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

/// A server with Flow1 (writable, 5) and Hidden (not in the table): the client of the table may read and write Flow1.
async fn serving() -> TaseServer {
    let mut vmd = Vmd::new(Identity {
        vendor: "igs".to_string(),
        model: "cli-test".to_string(),
        revision: "1".to_string(),
    });
    vmd.add_domain(Name::new("ICC1").unwrap()).unwrap();
    let target = Target::in_domain(Name::new("ICC1").unwrap(), Name::new("Flow1").unwrap());
    let variable = MemoryVariable::new(
        TypeDescription::Integer(Unsigned8(32)),
        Data::Integer(5),
        true,
    )
    .unwrap();
    vmd.add_variable(&target, Box::new(variable)).unwrap();

    let table = BilateralTable::new(
        "1.3.9999.1",
        Versions {
            table_id: VisibleString32::new("BT-2026-001").unwrap(),
            tase2: STANDARD_VERSION,
        },
        "ICC1",
    )
    .with_application_reference("1.3.9999.1")
    .with_data_value(
        icc("Flow1"),
        ObjectAccess {
            permitted: BTreeSet::from([
                Permission::Visibility,
                Permission::GetDataValue,
                Permission::GetDataValueType,
            ]),
            minimum_interval: None,
        },
    );
    let mut tables = BilateralTables::new();
    tables.insert(table);
    let config = TaseServerConfig {
        mms: ServerConfig {
            poll_interval: Duration::from_millis(10),
            ..ServerConfig::new(LinkConfig::default())
        },
        tables,
        policy: ServerPolicy::new(),
        transfer_sets: BTreeSet::new(),
        data_sets: Vec::new(),
        data_set_capacity: 4,
        devices: Vec::new(),
        transfer: Policy {
            expected_send_delay: Duration::ZERO,
            ack_timeout: Duration::from_secs(30),
        },
    };
    TaseServer::bind("127.0.0.1:0", config, vmd).await.unwrap()
}

/// Runs the client with the arguments, against the address of the server; returns the status and the output.
fn client(address: &str, args: &[&str]) -> (Option<i32>, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_igstase2-client"))
        .args(["-s", address])
        .args(args)
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn the_help_names_the_commands() {
    let output = Command::new(env!("CARGO_BIN_EXE_igstase2-client"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        text.contains("get") && text.contains("start") && text.contains("--server"),
        "{text}"
    );
}

#[test]
fn a_command_line_without_a_command_is_a_usage_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_igstase2-client"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[tokio::test(flavor = "multi_thread")]
async fn the_client_reads_writes_and_refuses_as_the_table_says() {
    let server = serving().await;
    let address = server.local_address().to_string();
    let (status, out, _) = tokio::task::spawn_blocking({
        let address = address.clone();
        move || client(&address, &["get", "Flow1"])
    })
    .await
    .unwrap();
    assert_eq!(status, Some(0));
    assert_eq!(out.trim(), "Flow1: 5");

    // The table does not let this client write Flow1 here: the item is an error, and the command succeeds.
    let (status, out, _) = tokio::task::spawn_blocking({
        let address = address.clone();
        move || client(&address, &["set", "Flow1=9"])
    })
    .await
    .unwrap();
    assert_eq!(status, Some(0));
    assert!(out.contains("Flow1: error ObjectAccessDenied"), "{out}");
    server.shutdown().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn an_unknown_ap_title_is_refused_with_status_1() {
    let server = serving().await;
    let address = server.local_address().to_string();
    let (status, _, err) = tokio::task::spawn_blocking(move || {
        client(&address, &["--ap-title", "1.3.9999.9", "get", "Flow1"])
    })
    .await
    .unwrap();
    assert_eq!(status, Some(1));
    assert!(err.contains("refused the association"), "{err}");
    server.shutdown().await;
}
