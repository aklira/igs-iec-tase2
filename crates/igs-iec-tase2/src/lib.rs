// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The TASE.2 client (task T1): an association with a TASE.2 server over `igs-iso9506-mms`, the reads and writes of data
//! values and data sets, the start and the stop of data set transfer sets, and the stream of the transfer reports.
//!
//! The checks of the standard are the link's (`igs-iec-tase2-link`): the client's association uses its association check,
//! and the model's types convert the values. Nothing here is a second implementation of them.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod client;
pub mod error;
mod names;
pub mod server;

pub use client::{Agreement, ReceivedReport, ReportAccess, TaseClient, TaseEvent, TaseEvents};
pub use error::Error;
pub use server::{mms_capabilities, DeviceConfig, TaseServer, TaseServerConfig};
