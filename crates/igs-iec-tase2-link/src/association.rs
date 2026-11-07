// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The association of two control centres (IEC 60870-6-503 §5.3.3, §7.2.2), sans I/O: what the client reads and checks
//! after the MMS Initiate, what the server checks before it accepts, and what is released when the association is
//! concluded or aborted. The MMS exchanges themselves are the caller's (the client and server of task T1 and S1).

use std::collections::BTreeSet;

use igs_iec_tase2_model::generated::iso503::{SupportedFeatures, TASE2Version, VisibleString32};

use crate::bilateral::{Admission, BilateralTables, ObjectName};
use crate::version::{check_versions, Conflict, Tase2Version, Versions};

/// The three named variables that the client reads after the MMS Initiate: `Bilateral_Table_ID`, `TASE.2_Version` and
/// `Supported_Features` (503 §7.2.2.2.2).
#[derive(Debug, Clone, PartialEq)]
pub struct ServerVariables {
    /// The value of `Bilateral_Table_ID`.
    pub table_id: VisibleString32,
    /// The value of `TASE.2_Version`.
    pub tase2: TASE2Version,
    /// The value of `Supported_Features`.
    pub supported_features: SupportedFeatures,
}

/// An association that the client may use: the server's features, which the client records.
#[derive(Debug, Clone, PartialEq)]
pub struct Established {
    /// The building blocks that the server supports.
    pub supported_features: SupportedFeatures,
}

/// The check of the client after a positive MMS Initiate: the table identifier, then the TASE.2 version, against the
/// Bilateral Agreement. A mismatch is a conflict: the client concludes and makes no further Initiate until it is resolved
/// (503 §7.2.2.2.2). A TASE.2 version whose numbers do not fit in 16 bits is a mismatch of the version.
pub fn client_after_initiate(
    agreed: &Versions,
    read: &ServerVariables,
) -> Result<Established, Conflict> {
    let tase2 = Tase2Version::from_mms(&read.tase2).ok_or(Conflict::Tase2Version)?;
    check_versions(
        agreed,
        &Versions {
            table_id: read.table_id.clone(),
            tase2,
        },
    )?;
    Ok(Established {
        supported_features: read.supported_features,
    })
}

/// The reasons of a server's refusal of an association request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decline {
    /// A negative MMS Initiate response, for the client designation or the application reference that the table does not
    /// contain (503 §5.3.2, §7.2.2.2.3).
    Negative(Admission),
    /// An MMS Initiate-ErrorPDU with the error class INITIATE and the error code OTHER: the server is the designated
    /// initiator of the association with this peer (503 §7.2.2.2.1).
    InitiateErrorOther,
}

/// The server's decision on an association request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// The association is accepted: the server returns a positive MMS Initiate response.
    Accept,
    /// The association is refused, and the reason says which response to send.
    Refuse(Decline),
}

/// The agreements of the server that the tables do not hold: the peers for which this server is the designated
/// initiator of the associations (503 §7.2.2.2.1), decided by prior agreement between the control centres.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ServerPolicy {
    initiator_for: BTreeSet<String>,
}

impl ServerPolicy {
    /// A policy in which this server is the designated initiator for none of the peers.
    pub fn new() -> Self {
        Self::default()
    }

    /// Designates this server as the initiator of the associations with the peer `designation`.
    #[must_use]
    pub fn with_initiator_for(mut self, designation: impl Into<String>) -> Self {
        self.initiator_for.insert(designation.into());
        self
    }
}

/// The check of the server on a valid MMS Initiate request: the request is refused when this server is the designated
/// initiator of the peer, and otherwise the Bilateral Table decides: the client designation must have a table, and the
/// application reference must be listed in it (503 §5.3.2, §7.2.2.2.3).
pub fn server_on_initiate(
    tables: &BilateralTables,
    policy: &ServerPolicy,
    designation: &str,
    application_reference: &str,
) -> Decision {
    if policy.initiator_for.contains(designation) {
        return Decision::Refuse(Decline::InitiateErrorOther);
    }
    match tables.admit(designation, application_reference) {
        Admission::Granted => Decision::Accept,
        refused => Decision::Refuse(Decline::Negative(refused)),
    }
}

/// What an association holds in use: the transfer sets that are active and the devices that the client has selected.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Active {
    /// The transfer sets that the client started on this association.
    pub transfer_sets: BTreeSet<ObjectName>,
    /// The devices that the client selected on this association.
    pub selected_devices: BTreeSet<ObjectName>,
}

/// What the server releases when the association is concluded or aborted (503 §7.2.2.3.3, §7.2.2.4.3).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Release {
    /// The transfer sets that become DISABLED; their names return to the pool of available names.
    pub disabled_transfer_sets: BTreeSet<ObjectName>,
    /// The devices that are deselected.
    pub deselected_devices: BTreeSet<ObjectName>,
}

/// The release of an association, by a Conclude or an Abort: every active transfer set is disabled, and every selected
/// device is deselected. The association holds nothing afterwards.
pub fn release(active: &mut Active) -> Release {
    Release {
        disabled_transfer_sets: std::mem::take(&mut active.transfer_sets),
        deselected_devices: std::mem::take(&mut active.selected_devices),
    }
}
