// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The TASE.2 link layer, sans I/O: the bilateral table, its access checks, and the version checks of an association.
//! Nothing here opens a socket or reads a clock: a caller gives the elapsed times and the requested intervals.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod association;
pub mod bilateral;
pub mod data;
pub mod transfer;
pub mod version;
