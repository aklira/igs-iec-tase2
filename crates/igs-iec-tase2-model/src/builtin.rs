// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The base types that the data files name without defining them: the MMS scalars and the identifier of MMS.
//! Written by hand, cited here; the generated code refers to them (`crate::builtin`).

use crate::value::{OutOfRange, TooLong};

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

    /// The identifier, if it has at most 32 characters.
    pub fn new(text: impl Into<String>) -> Result<Self, TooLong> {
        let text = text.into();
        let length = text.chars().count();
        if length <= Self::MAX_LEN {
            Ok(Self(text))
        } else {
            Err(TooLong {
                length,
                max: Self::MAX_LEN,
            })
        }
    }

    /// The identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
