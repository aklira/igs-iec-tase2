// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! Support types of the generated code: the metadata of the definitions, the range and length errors.

/// One definition of a data file: its name, the standard, the clause and the page where it is defined, its kind,
/// the number of components of a structure, and the number of bits of a bit string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Definition {
    /// The name in the data file.
    pub name: &'static str,
    /// The standard: `IEC 60870-6-802` or `IEC 60870-6-503`.
    pub standard: &'static str,
    /// The clause where it is defined.
    pub clause: &'static str,
    /// The page of the standard.
    pub page: u32,
    /// The kind of type.
    pub kind: &'static str,
    /// The components, for a structure; zero otherwise.
    pub components: usize,
    /// The bits, for a bit string: one more than the greatest bit position; zero otherwise.
    pub bits: usize,
}

/// One variable of the bilateral table of IEC 60870-6-503: an object with a type and a scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableDef {
    /// The name in the data file.
    pub name: &'static str,
    /// The clause where it is defined.
    pub clause: &'static str,
    /// The page of the standard.
    pub page: u32,
    /// The scope of the variable, as the data file writes it.
    pub scope: &'static str,
    /// The Rust path of its type.
    pub type_path: &'static str,
}

/// A value outside the range of its type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfRange {
    /// The value offered.
    pub value: i128,
    /// The least value of the type.
    pub min: i128,
    /// The greatest value of the type.
    pub max: i128,
}

impl std::fmt::Display for OutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is outside [{}, {}]", self.value, self.min, self.max)
    }
}

impl std::error::Error for OutOfRange {}

/// A string longer than the greatest number of characters of its type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooLong {
    /// The number of characters offered.
    pub length: usize,
    /// The greatest number of characters.
    pub max: usize,
}

impl std::fmt::Display for TooLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} characters, at most {} allowed",
            self.length, self.max
        )
    }
}

impl std::error::Error for TooLong {}
