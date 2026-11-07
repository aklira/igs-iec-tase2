// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The TASE.2 types of IEC 60870-6-802 and IEC 60870-6-503, and their conversion to and from MMS data (no I/O).
//!
//! The types of the standards are generated from their data files (`generated`, by `cargo xtask codegen`); the
//! base types and the support types are written by hand (`builtin`, `value`).

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod builtin;
pub mod generated;
pub mod indication;
pub mod mms;
pub mod protection;
#[cfg(feature = "strategy")]
pub mod strategy;
pub mod time;
pub mod value;
