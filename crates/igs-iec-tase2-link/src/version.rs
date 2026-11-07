// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The versions that two control centres compare after an association (IEC 60870-6-503 §5.3.3.3, §8.3.3, §8.3.4): the
//! identifier of the Bilateral Table, then the TASE.2 version. A mismatch ends the association.

use igs_iec_tase2_model::generated::iso503::VisibleString32;

/// A version of TASE.2: a major and a minor number (the type `TASE.2Version` of 503 §8.3.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tase2Version {
    /// The major version number.
    pub major: i16,
    /// The minor version number.
    pub minor: i16,
}

/// The version of TASE.2 of this standard: 2000.08 (503 §8.3.4).
pub const STANDARD_VERSION: Tase2Version = Tase2Version {
    major: 2000,
    minor: 8,
};

/// What two parties agree on: the identifier of the Bilateral Table and the TASE.2 version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Versions {
    /// The identifier of the Bilateral Table (`Bilateral_Table_ID`).
    pub table_id: VisibleString32,
    /// The TASE.2 version (`TASE.2_Version`).
    pub tase2: Tase2Version,
}

/// The versions that do not match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conflict {
    /// The identifiers of the Bilateral Table differ: the Conclude is the first check (503 §5.3.3.3).
    TableId,
    /// The identifiers match, and the TASE.2 versions differ.
    Tase2Version,
}

/// Compares the versions read from the peer (`read`) with those of the Bilateral Agreement (`agreed`). The table
/// identifier is compared first; the TASE.2 version only when the identifiers are identical (503 §5.3.3.3).
pub fn check_versions(agreed: &Versions, read: &Versions) -> Result<(), Conflict> {
    if agreed.table_id != read.table_id {
        return Err(Conflict::TableId);
    }
    if agreed.tase2 != read.tase2 {
        return Err(Conflict::Tase2Version);
    }
    Ok(())
}
