// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The errors of the client.

use igs_iec_tase2_link::data::ErrorCode;
use igs_iec_tase2_link::version::Conflict;
use igs_iec_tase2_model::mms::ValueError;

/// An error of the client.
#[derive(Debug)]
pub enum Error {
    /// The MMS connection or request failed, or the peer answered with an error (the MMS error is kept as it is).
    Mms(igs_iso9506_mms::Error),
    /// The Bilateral Table identifier or the TASE.2 version of the server does not match the agreement: the association is
    /// concluded (503 §7.2.2.2.2).
    Conflict(Conflict),
    /// One of the variables read after the association failed, with its error.
    Variable(ErrorCode),
    /// The value of a variable read after the association is not a value of its type.
    Value(ValueError),
    /// The response of the server does not have the shape of its request.
    Malformed(&'static str),
}

impl From<igs_iso9506_mms::Error> for Error {
    fn from(error: igs_iso9506_mms::Error) -> Self {
        Self::Mms(error)
    }
}

impl From<ValueError> for Error {
    fn from(error: ValueError) -> Self {
        Self::Value(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mms(error) => write!(f, "the MMS connection or request failed: {error:?}"),
            Self::Conflict(conflict) => write!(
                f,
                "the server's versions do not match the agreement: {conflict:?}"
            ),
            Self::Variable(code) => {
                write!(f, "a variable read after the association failed: {code:?}")
            }
            Self::Value(error) => write!(f, "a value does not have its type: {error:?}"),
            Self::Malformed(text) => write!(f, "the response is malformed: {text}"),
        }
    }
}

impl std::error::Error for Error {}
