// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The base types written by hand (`builtin`), at their limits.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use igs_iec_tase2_model::builtin::{Integer16, Integer32, MmsIdentifier, Unsigned8};

#[test]
fn an_integer_of_16_bits_takes_its_limits_and_no_more() {
    assert_eq!(Integer16::new(-32_768).unwrap().get(), -32_768);
    assert_eq!(Integer16::new(32_767).unwrap().get(), 32_767);
    assert!(Integer16::new(32_768).is_err());
    assert!(Integer16::new(-32_769).is_err());
}

#[test]
fn an_integer_of_32_bits_takes_its_limits_and_no_more() {
    assert_eq!(
        Integer32::new(-2_147_483_648).unwrap().get(),
        -2_147_483_648
    );
    assert_eq!(Integer32::new(2_147_483_647).unwrap().get(), 2_147_483_647);
    assert!(Integer32::new(2_147_483_648).is_err());
}

#[test]
fn an_unsigned_of_8_bits_takes_its_limits_and_no_more() {
    assert_eq!(Unsigned8::new(0).unwrap().get(), 0);
    assert_eq!(Unsigned8::new(255).unwrap().get(), 255);
    assert!(Unsigned8::new(256).is_err());
}

#[test]
fn an_mms_identifier_has_at_most_32_characters() {
    let longest = "x".repeat(32);
    assert_eq!(
        MmsIdentifier::new(longest.as_str()).unwrap().as_str(),
        longest
    );
    assert!(MmsIdentifier::new("x".repeat(33)).is_err());
}
