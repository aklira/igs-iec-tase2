// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The work of one load client: it associates, reads every point in chunks, writes a few points, and (in the local
//! run) starts a transfer set and measures the latency of its report. It also checks that no report reaches it that
//! it did not ask for.

use std::sync::Arc;
use std::time::{Duration, Instant};

use igs_iec_tase2::{Agreement, TaseClient, TaseEvent, TaseServer};
use igs_iec_tase2_link::bilateral::{ObjectName, Scope};
use igs_iec_tase2_link::data::Item;
use igs_iec_tase2_link::version::{Versions, STANDARD_VERSION};
use igs_iec_tase2_model::builtin::{Integer16, Integer32, MmsIdentifier};
use igs_iec_tase2_model::generated::iso503::{
    DSConditions, DSTransferSet, DSTransferSetAllChangesReported, DSTransferSetBlockData,
    DSTransferSetCritical, DSTransferSetRBE, DSTransferSetStatus, MMSObjectName,
    MMSObjectNameScope, VisibleString32,
};
use igs_iso9506_mms::client::{in_scope_capabilities, ClientConfig, Reconnect};
use igs_iso9506_mms_codec::generated::types::Data;
use igs_iso9506_mms_codec::schema::ObjectIdentifier;
use igs_iso9506_mms_link::config::LinkConfig;

use crate::local::{DESIGNATION, TABLE_ID};
use crate::stats::Recorder;

/// The name of point `index`: `P00000`, `P00001`, ... in the domain ICC1.
pub fn point_name(index: usize) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: format!("P{index:05}"),
    }
}

/// The options of the load.
#[derive(Debug, Clone)]
pub struct Options {
    /// The number of points.
    pub points: usize,
    /// The number of points in one read.
    pub chunk: usize,
    /// The number of times each client reads every point.
    pub rounds: usize,
    /// The number of writes of each client.
    pub writes: usize,
    /// Whether the client starts the transfer set and measures its report (the local run only).
    pub report: bool,
    /// How long to wait for a report.
    pub timeout: Duration,
}

fn agreement() -> Result<Agreement, String> {
    Ok(Agreement {
        domain: "ICC1".to_string(),
        versions: Versions {
            table_id: VisibleString32::new(TABLE_ID).map_err(|e| format!("{e:?}"))?,
            tase2: STANDARD_VERSION,
        },
    })
}

fn client_config() -> ClientConfig {
    ClientConfig {
        reconnect: Reconnect::Never,
        ..ClientConfig::new(
            LinkConfig {
                ap_title: Some(ObjectIdentifier(
                    DESIGNATION
                        .split('.')
                        .filter_map(|part| part.parse().ok())
                        .collect(),
                )),
                ..LinkConfig::default()
            },
            in_scope_capabilities(),
        )
    }
}

fn transfer_set() -> Result<DSTransferSet, String> {
    let id = |text: &str| MmsIdentifier::new(text).map_err(|e| format!("{e:?}"));
    let int = |v: i64| Integer32::new(v).map_err(|e| format!("{e:?}"));
    Ok(DSTransferSet {
        data_set_name: MMSObjectName {
            scope: MMSObjectNameScope(MMSObjectNameScope::DOMAIN_SPECIFIC),
            domain_name: id("ICC1")?,
            name: id("DS1")?,
        },
        start_time: int(0)?,
        interval: int(0)?,
        tle: int(0)?,
        buffer_time: int(0)?,
        integrity_check: int(0)?,
        ds_conditions_requested: DSConditions(DSConditions::OBJECT_CHANGE),
        block_data: DSTransferSetBlockData(false),
        critical: DSTransferSetCritical(false),
        rbe: DSTransferSetRBE(true),
        all_changes_reported: DSTransferSetAllChangesReported(false),
        status: DSTransferSetStatus(DSTransferSetStatus::ENABLED),
        event_code_requested: Integer16::new(0).map_err(|e| format!("{e:?}"))?,
    })
}

/// Runs the work of client `index`, and returns what it measured.
pub async fn run_client(
    address: String,
    index: usize,
    options: Options,
    server: Option<Arc<TaseServer>>,
) -> Recorder {
    let mut recorder = Recorder::default();
    let started = Instant::now();
    let agreed = match agreement() {
        Ok(agreed) => agreed,
        Err(_) => {
            recorder.errors = recorder.errors.saturating_add(1);
            return recorder;
        }
    };
    let (client, mut events) = match TaseClient::associate(&address, client_config(), &agreed).await
    {
        Ok(pair) => pair,
        Err(_) => {
            recorder.errors = recorder.errors.saturating_add(1);
            return recorder;
        }
    };
    recorder.record("associate", started.elapsed());

    let names: Vec<ObjectName> = (0..options.points).map(point_name).collect();
    for _ in 0..options.rounds {
        for chunk in names.chunks(options.chunk.max(1)) {
            let begin = Instant::now();
            match client.get_data_values(chunk).await {
                Ok(items)
                    if items.len() == chunk.len()
                        && items.iter().all(|i| matches!(i, Item::Success(_))) =>
                {
                    recorder.record("read", begin.elapsed());
                }
                _ => recorder.errors = recorder.errors.saturating_add(1),
            }
        }
    }

    // The writes go to the points after the first, which the report uses.
    for write in 0..options.writes {
        let span = options.points.saturating_sub(1).max(1);
        let spread = index.saturating_mul(options.writes).saturating_add(write);
        let index_of_point = spread.checked_rem(span).unwrap_or(0).saturating_add(1);
        let value = i128::try_from(write).unwrap_or(0);
        let begin = Instant::now();
        match client
            .set_data_values(vec![(point_name(index_of_point), Data::Integer(value))])
            .await
        {
            Ok(items) if items.iter().all(|i| matches!(i, Item::Success(()))) => {
                recorder.record("write", begin.elapsed());
            }
            _ => recorder.errors = recorder.errors.saturating_add(1),
        }
    }

    if options.report && index == 0 {
        if let Some(server) = server.as_ref() {
            let started_ok = match transfer_set() {
                Ok(set) => matches!(
                    client
                        .start_transfer(
                            &ObjectName {
                                scope: Scope::Domain("ICC1".to_string()),
                                name: "TS1".to_string()
                            },
                            &set
                        )
                        .await,
                    Ok(Item::Success(()))
                ),
                Err(_) => false,
            };
            if started_ok {
                let begin = Instant::now();
                if server
                    .set_value(point_name(0), Data::Integer(1_000_000))
                    .await
                    .is_err()
                {
                    recorder.errors = recorder.errors.saturating_add(1);
                }
                loop {
                    match tokio::time::timeout(options.timeout, events.next()).await {
                        Ok(Some(Ok(TaseEvent::Report(_)))) => {
                            recorder.record("report", begin.elapsed());
                            break;
                        }
                        Ok(Some(Ok(TaseEvent::Connection(_)))) => continue,
                        _ => {
                            recorder.errors = recorder.errors.saturating_add(1);
                            break;
                        }
                    }
                }
            } else {
                recorder.errors = recorder.errors.saturating_add(1);
            }
        }
    }

    // A report that reaches a client that did not start a transfer set is a stray one.
    let drain_until = Instant::now()
        .checked_add(Duration::from_millis(100))
        .unwrap_or_else(Instant::now);
    while let Ok(Some(event)) = tokio::time::timeout(
        drain_until
            .saturating_duration_since(Instant::now())
            .max(Duration::from_millis(1)),
        events.next(),
    )
    .await
    {
        if let Ok(TaseEvent::Report(_)) = event {
            if !(options.report && index == 0) {
                recorder.stray_reports = recorder.stray_reports.saturating_add(1);
            }
        }
        if Instant::now() >= drain_until {
            break;
        }
    }
    let _ = client.close().await;
    recorder
}
