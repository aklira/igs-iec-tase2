// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! Support types of the generated code: the metadata of the definitions, the range and string errors.

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

/// A string that its type refuses: longer than the greatest number of characters, or with a character that a
/// visible string does not have (decision D-014).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum StringError {
    /// More characters than the greatest number of the type.
    TooLong {
        /// The number of characters offered.
        length: usize,
        /// The greatest number of characters.
        max: usize,
    },
    /// A character outside the visible string characters (space to tilde).
    NotVisible(char),
}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { length, max } => {
                write!(f, "{length} characters, at most {max} allowed")
            }
            Self::NotVisible(c) => {
                write!(f, "the character {c:?} is not a visible string character")
            }
        }
    }
}

impl std::error::Error for StringError {}

/// Whether a character is a visible string character: the graphic characters and the space (decision D-014).
pub fn is_visible(c: char) -> bool {
    matches!(c, ' '..='~')
}

/// Checks a string against the greatest number of characters of its type, and against the visible characters.
pub fn check_visible(text: &str, max: usize) -> Result<(), StringError> {
    let length = text.chars().count();
    if length > max {
        return Err(StringError::TooLong { length, max });
    }
    match text.chars().find(|c| !is_visible(*c)) {
        Some(c) => Err(StringError::NotVisible(c)),
        None => Ok(()),
    }
}
