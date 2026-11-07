// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The generated types (task F2): the counts of the data files, the components and the bits, and the scoping of a
//! name defined in several clauses (decision D-001). The expected counts come from the data files, counted
//! independently (python, from the YAML), and are written here; the test does not read the standards.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::collections::BTreeSet;

use igs_iec_tase2_model::generated::{iso503, iso802};
use igs_iec_tase2_model::value::Definition;

const STANDARD_802: &str = "IEC 60870-6-802";
const STANDARD_503: &str = "IEC 60870-6-503";

fn of(standard: &str, all: &'static [Definition]) -> Vec<&'static Definition> {
    all.iter().filter(|d| d.standard == standard).collect()
}

fn unique_names(definitions: &[&Definition]) -> usize {
    definitions
        .iter()
        .map(|d| d.name)
        .collect::<BTreeSet<_>>()
        .len()
}

#[test]
fn the_definitions_of_802_are_123_with_111_names() {
    let all = of(STANDARD_802, iso802::DEFINITIONS);
    assert_eq!(all.len(), 123);
    assert_eq!(unique_names(&all), 111);
}

#[test]
fn the_definitions_of_503_are_49_with_40_names() {
    let all = of(STANDARD_503, iso503::DEFINITIONS);
    assert_eq!(all.len(), 49);
    assert_eq!(unique_names(&all), 40);
}

#[test]
fn the_components_and_the_bits_are_counted_as_in_the_data() {
    // Structures: 285 components in 802, 53 in 503. Bit strings: 68 bits in 802, 45 in 503.
    let components = |all: &[&Definition]| all.iter().map(|d| d.components).sum::<usize>();
    let bits = |all: &[&Definition]| all.iter().map(|d| d.bits).sum::<usize>();
    let s802 = of(STANDARD_802, iso802::DEFINITIONS);
    let s503 = of(STANDARD_503, iso503::DEFINITIONS);
    assert_eq!(components(&s802), 285);
    assert_eq!(components(&s503), 53);
    assert_eq!(bits(&s802), 68);
    assert_eq!(bits(&s503), 45);
}

#[test]
fn the_variables_of_the_bilateral_table_are_19() {
    assert_eq!(iso503::VARIABLES.len(), 19);
}

#[test]
fn a_name_defined_in_several_clauses_keeps_each_definition_in_its_clause() {
    // Number: 16 bits in clause 6.3, 32 bits in clause 6.4 (decision D-001).
    assert_eq!(iso802::clause_6_3::Number::MAX, 32_767);
    assert_eq!(iso802::clause_6_4::Number::MAX, 2_147_483_647);
    assert_eq!(iso802::clause_6_3::Number::WIDTH, 16);
    assert_eq!(iso802::clause_6_4::Number::WIDTH, 32);
}

#[test]
fn a_bit_string_names_each_flag_with_its_mask() {
    // DSConditions: IntervalTimeOut is bit 0, IntegrityTimeOut is bit 1, five bits in all.
    assert_eq!(iso503::DSConditions::BIT_COUNT, 5);
    assert_eq!(iso503::DSConditions::INTERVAL_TIME_OUT, 0x1);
    assert_eq!(iso503::DSConditions::INTEGRITY_TIME_OUT, 0x2);
    let set = iso503::DSConditions(0x2);
    assert!(set.is_integrity_time_out());
    assert!(!set.is_interval_time_out());
}

#[test]
fn the_spelling_of_the_data_file_is_kept_in_the_type_names() {
    // D-002: COVCounter, the spelling of the definition in the data file (802 §6.2).
    assert_eq!(iso802::COVCounter::MAX, 65_535);
}

#[test]
fn a_plain_integer_of_the_data_file_is_a_32_bit_integer() {
    // D-005: NumPeriods of Forecast is a plain INTEGER, typed as the 32-bit integer of the base types.
    fn num_periods(forecast: &iso802::Forecast) -> &igs_iec_tase2_model::builtin::Integer32 {
        &forecast.num_periods
    }
    let _ = num_periods;
}
