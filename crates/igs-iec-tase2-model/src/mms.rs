// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The conversion of the TASE.2 types to and from the data of MMS (ISO 9506-2, the `Data` type of the codec).
//!
//! The conversions check the kind of the MMS value, its range and its length, and refuse a value that the type does
//! not hold: nothing is converted by guessing (rule 8 of `IMPLEMENTATION.md`).

use igs_iso9506_mms_codec::generated::types::Data;

use crate::value::{OutOfRange, StringError};

/// A TASE.2 type that converts to and from an MMS value.
pub trait MmsValue: Sized {
    /// The MMS value of this value.
    fn to_mms(&self) -> Data;

    /// The value of an MMS value, when the MMS value is of the kind of this type and in its range.
    fn from_mms(data: &Data) -> Result<Self, ValueError>;
}

/// A conversion from an MMS value that failed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValueError {
    /// The MMS value is of another kind.
    Kind {
        /// The kind that the type holds.
        expected: &'static str,
        /// The kind that the MMS value has.
        found: &'static str,
    },
    /// The number is outside the range of the type.
    Range(OutOfRange),
    /// The string is refused by the type.
    Text(StringError),
    /// The MMS value has the right kind, but not the form of the type (for example a floating-point value that is not
    /// of the single precision format).
    Format(&'static str),
}

impl ValueError {
    /// A kind mismatch, from the value found.
    pub fn kind_of(expected: &'static str, found: &Data) -> Self {
        Self::Kind {
            expected,
            found: kind(found),
        }
    }
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kind { expected, found } => write!(f, "expected {expected}, found {found}"),
            Self::Range(range) => write!(f, "{range}"),
            Self::Text(error) => write!(f, "{error}"),
            Self::Format(why) => write!(f, "{why}"),
        }
    }
}

impl std::error::Error for ValueError {}

impl From<OutOfRange> for ValueError {
    fn from(range: OutOfRange) -> Self {
        Self::Range(range)
    }
}

impl From<StringError> for ValueError {
    fn from(error: StringError) -> Self {
        Self::Text(error)
    }
}

/// The name of the kind of an MMS value, for the messages of the conversions.
pub fn kind(data: &Data) -> &'static str {
    match data {
        Data::Array(_) => "array",
        Data::Structure(_) => "structure",
        Data::Boolean(_) => "boolean",
        Data::BitString(_) => "bit string",
        Data::Integer(_) => "integer",
        Data::Unsigned(_) => "unsigned",
        Data::FloatingPoint(_) => "floating point",
        Data::OctetString(_) => "octet string",
        Data::VisibleString(_) => "visible string",
        Data::GeneralizedTime(_) => "generalized time",
        Data::BinaryTime(_) => "binary time",
        Data::Bcd(_) => "binary coded decimal",
        Data::BooleanArray(_) => "boolean array",
        Data::ObjId(_) => "object identifier",
        Data::MMSString(_) => "MMS string",
        Data::Unsupported { .. } => "unsupported value",
    }
}
