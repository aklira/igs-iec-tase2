// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The binary `igstase2-server` (task S2), run as a process: its help, its refusal of a bad file, and a client of its table
//! that reads a value which the simulation changes.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use igs_iec_tase2::{Agreement, TaseClient};
use igs_iec_tase2_link::bilateral::{ObjectName, Scope};
use igs_iec_tase2_link::data::Item;
use igs_iec_tase2_link::version::{Versions, STANDARD_VERSION};
use igs_iec_tase2_model::generated::iso503::VisibleString32;
use igs_iso9506_mms::client::{in_scope_capabilities, ClientConfig, Reconnect};
use igs_iso9506_mms_codec::generated::types::Data;
use igs_iso9506_mms_codec::schema::ObjectIdentifier;
use igs_iso9506_mms_link::config::LinkConfig;

const WAIT: Duration = Duration::from_secs(10);

/// A child process that is stopped when the test ends.
struct Running(Child);

impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn free_address() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap().to_string();
    drop(listener);
    address
}

fn write_config(name: &str, text: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    std::fs::write(&path, text).unwrap();
    path
}

fn start(config: &Path, address: &str) -> Running {
    let child = Command::new(env!("CARGO_BIN_EXE_igstase2-server"))
        .args(["--listen", address, "--config"])
        .arg(config)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    Running(child)
}

/// A configuration: one client (AP-title 1.3.9999.1) whose table reads Flow1, and a ramp that moves Flow1 from 0 to 3.
const DEMO: &str = r#"
[[domain]]
name = "ICC1"

[[point]]
domain = "ICC1"
name = "Flow1"
type = "integer"
bits = 16
value = 0
writable = true

[[table]]
designation = "1.3.9999.1"
table_id = "BT-1"
domain = "ICC1"
application_references = ["1.3.9999.1"]

[[table.access]]
kind = "data-value"
name = "Flow1"
permissions = ["visibility", "get-data-value"]

[[simulate]]
domain = "ICC1"
name = "Flow1"
kind = "ramp"
period_ms = 100
min = 0
max = 3
step = 1
"#;

fn agreement() -> Agreement {
    Agreement {
        domain: "ICC1".to_string(),
        versions: Versions {
            table_id: VisibleString32::new("BT-1").unwrap(),
            tase2: STANDARD_VERSION,
        },
    }
}

/// Associates with the server, retrying while it starts.
async fn associate(address: &str) -> (TaseClient, igs_iec_tase2::TaseEvents) {
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
    let started = Instant::now();
    loop {
        match TaseClient::associate(address, config.clone(), &agreement()).await {
            Ok(client) => return client,
            Err(error) if started.elapsed() < WAIT => {
                let _ = error;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Err(error) => panic!("the server does not answer: {error:?}"),
        }
    }
}

#[test]
fn the_help_names_the_options() {
    let output = Command::new(env!("CARGO_BIN_EXE_igstase2-server"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        text.contains("--config") && text.contains("--listen"),
        "{text}"
    );
}

#[test]
fn a_bad_configuration_exits_with_status_2_and_names_the_object() {
    let config = write_config(
        "bad.toml",
        r#"
[[domain]]
name = "ICC1"

[[point]]
domain = "ICC1"
name = "Big"
type = "integer"
bits = 8
value = 200
"#,
    );
    let output = Command::new(env!("CARGO_BIN_EXE_igstase2-server"))
        .args(["--listen", "127.0.0.1:0", "--config"])
        .arg(&config)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let text = String::from_utf8_lossy(&output.stderr);
    assert!(text.contains("ICC1/Big"), "{text}");
}

#[tokio::test]
async fn a_client_of_its_table_reads_a_value_that_the_simulation_changes() {
    let config = write_config("demo.toml", DEMO);
    let address = free_address();
    let _server = start(&config, &address);
    let (client, _events) = associate(&address).await;
    let name = ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: "Flow1".to_string(),
    };
    let first = client
        .get_data_values(std::slice::from_ref(&name))
        .await
        .unwrap();
    let started = Instant::now();
    let mut changed = false;
    while started.elapsed() < WAIT {
        let now = client
            .get_data_values(std::slice::from_ref(&name))
            .await
            .unwrap();
        if let (Item::Success(Data::Integer(a)), Item::Success(Data::Integer(b))) =
            (&first[0], &now[0])
        {
            if a != b {
                assert!(
                    (0..=3).contains(b),
                    "the ramp stays between its bounds: {b}"
                );
                changed = true;
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    assert!(changed, "the simulation changed Flow1 in time");
    client.close().await.unwrap();
}
