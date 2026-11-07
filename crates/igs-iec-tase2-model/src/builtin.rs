// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The base types that the data files name without defining them: the MMS scalars and the identifier of MMS.
//! Written by hand, cited here; the generated code refers to them (`crate::builtin`).

use igs_iso9506_mms_codec::generated::types::Data;

use crate::mms::{MmsValue, ValueError};
use crate::value::{check_visible, OutOfRange, StringError};

/// A signed integer of 16 bits (the two's complement range of 16 bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Integer16(i64);

impl Integer16 {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32_768;
    /// The greatest value.
    pub const MAX: i64 = 32_767;

    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(OutOfRange {
                value: i128::from(value),
                min: i128::from(Self::MIN),
                max: i128::from(Self::MAX),
            })
        }
    }

    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}

/// A signed integer of 32 bits (the two's complement range of 32 bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Integer32(i64);

impl Integer32 {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2_147_483_648;
    /// The greatest value.
    pub const MAX: i64 = 2_147_483_647;

    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(OutOfRange {
                value: i128::from(value),
                min: i128::from(Self::MIN),
                max: i128::from(Self::MAX),
            })
        }
    }

    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}

/// An unsigned integer of 8 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unsigned8(u64);

impl Unsigned8 {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: u64 = 0;
    /// The greatest value.
    pub const MAX: u64 = 255;

    /// The value, if it is in range.
    pub fn new(value: u64) -> Result<Self, OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(OutOfRange {
                value: i128::from(value),
                min: 0,
                max: i128::from(Self::MAX),
            })
        }
    }

    /// The value.
    pub fn get(self) -> u64 {
        self.0
    }
}

/// The identifier of MMS: a visible string of at most 32 characters (ISO 9506-2, identifiers of 32 characters).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmsIdentifier(String);

impl MmsIdentifier {
    /// The greatest number of characters.
    pub const MAX_LEN: usize = 32;

    /// The identifier, if it has at most 32 visible characters.
    pub fn new(text: impl Into<String>) -> Result<Self, StringError> {
        let text = text.into();
        check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }

    /// The identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl MmsValue for Integer16 {
    fn to_mms(&self) -> Data {
        Data::Integer(i128::from(self.0))
    }

    fn from_mms(data: &Data) -> Result<Self, ValueError> {
        match data {
            Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(ValueError::kind_of("integer", other)),
        }
    }
}

impl MmsValue for Integer32 {
    fn to_mms(&self) -> Data {
        Data::Integer(i128::from(self.0))
    }

    fn from_mms(data: &Data) -> Result<Self, ValueError> {
        match data {
            Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(ValueError::kind_of("integer", other)),
        }
    }
}

impl MmsValue for Unsigned8 {
    fn to_mms(&self) -> Data {
        Data::Unsigned(u128::from(self.0))
    }

    fn from_mms(data: &Data) -> Result<Self, ValueError> {
        match data {
            Data::Unsigned(value) => {
                let value = u64::try_from(*value).map_err(|_| OutOfRange {
                    value: i128::try_from(*value).unwrap_or(i128::MAX),
                    min: 0,
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(ValueError::kind_of("unsigned", other)),
        }
    }
}

impl MmsValue for MmsIdentifier {
    fn to_mms(&self) -> Data {
        Data::VisibleString(self.as_str().to_string())
    }

    fn from_mms(data: &Data) -> Result<Self, ValueError> {
        match data {
            Data::VisibleString(text) => Ok(Self::new(text.clone())?),
            other => Err(ValueError::kind_of("visible string", other)),
        }
    }
}
