// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The server of the load test, in the process: `points` writable points in the domain ICC1, the Bilateral Table of the
//! load clients (AP-title 1.3.9999.1), a data set DS1 holding the first point, and the transfer set TS1 on it.

use std::collections::BTreeSet;
use std::time::Duration;

use igs_iec_tase2::TaseServerConfig;
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

use crate::workload::point_name;

/// The table identifier of the load clients.
pub const TABLE_ID: &str = "BT-2026-001";

/// The designation (AP-title) of the load clients.
pub const DESIGNATION: &str = "1.3.9999.1";

fn icc(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

/// The errors of the MMS stack do not implement `std::error::Error`: they are shown with their debug form.
trait Ctx<T> {
    fn ctx(self) -> Result<T, String>;
}

impl<T, E: std::fmt::Debug> Ctx<T> for Result<T, E> {
    fn ctx(self) -> Result<T, String> {
        self.map_err(|e| format!("{e:?}"))
    }
}

/// The server configuration and its VMD, for `points` points.
pub fn server(points: usize) -> Result<(TaseServerConfig, Vmd), String> {
    let mut vmd = Vmd::new(Identity {
        vendor: "igs".to_string(),
        model: "igstase2-load".to_string(),
        revision: "1".to_string(),
    });
    vmd.add_domain(Name::new("ICC1").ctx()?).ctx()?;
    let mut table = BilateralTable::new(
        DESIGNATION,
        Versions {
            table_id: VisibleString32::new(TABLE_ID).ctx()?,
            tase2: STANDARD_VERSION,
        },
        "ICC1",
    )
    .with_application_reference(DESIGNATION)
    .with_data_set_creation();
    let point_access = ObjectAccess {
        permitted: BTreeSet::from([
            Permission::Visibility,
            Permission::GetDataValue,
            Permission::SetDataValue,
            Permission::GetDataValueType,
        ]),
        minimum_interval: None,
    };
    for index in 0..points {
        let name = point_name(index);
        let target = Target::in_domain(Name::new("ICC1").ctx()?, Name::new(&name.name).ctx()?);
        vmd.add_variable(
            &target,
            Box::new(
                MemoryVariable::new(
                    TypeDescription::Integer(Unsigned8(32)),
                    Data::Integer(0),
                    true,
                )
                .ctx()?,
            ),
        )
        .ctx()?;
        table = table.with_data_value(name, point_access.clone());
    }
    let data_set_access = ObjectAccess {
        permitted: BTreeSet::from([
            Permission::Visibility,
            Permission::DeleteDataSet,
            Permission::GetDataSetElementValues,
            Permission::SetDataSetElementValues,
        ]),
        minimum_interval: None,
    };
    let transfer_access = ObjectAccess {
        permitted: BTreeSet::from([
            Permission::Visibility,
            Permission::StartTransfer,
            Permission::StopTransfer,
        ]),
        minimum_interval: None,
    };
    table = table
        .with_data_set(icc("DS1"), data_set_access)
        .with_transfer_set(icc("TS1"), transfer_access);
    let mut tables = BilateralTables::new();
    tables.insert(table);
    let config = TaseServerConfig {
        mms: ServerConfig {
            poll_interval: Duration::from_millis(50),
            max_associations: 1024,
            ..ServerConfig::new(LinkConfig::default())
        },
        tables,
        policy: ServerPolicy::new(),
        transfer_sets: BTreeSet::from([icc("TS1")]),
        data_sets: vec![(icc("DS1"), vec![point_name(0)])],
        data_set_capacity: 16,
        devices: Vec::new(),
        transfer: Policy {
            expected_send_delay: Duration::ZERO,
            ack_timeout: Duration::from_secs(30),
        },
    };
    Ok((config, vmd))
}
