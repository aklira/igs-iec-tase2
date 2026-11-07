// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The information message types of IEC 60870-6-802 §6.4 (task C2, decision D-059): the header of an information message
//! and its buffer. Each round-trips through MMS, and a malformed value is refused.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use igs_iec_tase2_model::generated::iso802::clause_6_4::{Number, ReferenceNum};
use igs_iec_tase2_model::generated::iso802::{InfoBufXX, InfoMessHeader};
use igs_iec_tase2_model::mms::{MmsValue, ValueError};
use igs_iso9506_mms_codec::generated::types::Data;

fn round_trip<T: MmsValue + PartialEq + std::fmt::Debug>(value: &T) {
    let data = value.to_mms();
    assert_eq!(&T::from_mms(&data).unwrap(), value, "{data:?}");
}

fn header(reference: i64, local: i64, message: i64, size: i64) -> InfoMessHeader {
    InfoMessHeader {
        info_reference: ReferenceNum::new(reference).unwrap(),
        localreference: ReferenceNum::new(local).unwrap(),
        message_id: ReferenceNum::new(message).unwrap(),
        size: Number::new(size).unwrap(),
    }
}

#[test]
fn the_header_round_trips_at_the_limits_of_its_numbers() {
    round_trip(&header(0, 0, 0, 0));
    round_trip(&header(
        ReferenceNum::MIN,
        ReferenceNum::MAX,
        ReferenceNum::MIN,
        Number::MAX,
    ));
    round_trip(&header(1_234_567, 42, 9, 1024));
}

#[test]
fn the_header_is_a_structure_of_four_components_in_order() {
    let data = header(7, 8, 9, 10).to_mms();
    assert_eq!(
        data,
        Data::Structure(vec![
            Data::Integer(7),
            Data::Integer(8),
            Data::Integer(9),
            Data::Integer(10),
        ])
    );
}

#[test]
fn a_header_with_another_number_of_components_is_refused() {
    let data = Data::Structure(vec![Data::Integer(1), Data::Integer(2), Data::Integer(3)]);
    assert!(matches!(
        InfoMessHeader::from_mms(&data),
        Err(ValueError::Format(_))
    ));
}

#[test]
fn a_header_that_is_not_a_structure_is_refused() {
    assert!(matches!(
        InfoMessHeader::from_mms(&Data::Integer(1)),
        Err(ValueError::Kind { .. })
    ));
}

#[test]
fn a_reference_out_of_its_range_is_refused() {
    // The first reference is 32 bits: a value beyond its greatest is refused, not truncated.
    let data = Data::Structure(vec![
        Data::Integer(i128::from(ReferenceNum::MAX) + 1),
        Data::Integer(0),
        Data::Integer(0),
        Data::Integer(0),
    ]);
    assert!(matches!(
        InfoMessHeader::from_mms(&data),
        Err(ValueError::Range(_))
    ));
    // The size is 32 bits as well (Number of 6.4): the same refusal.
    let data = Data::Structure(vec![
        Data::Integer(0),
        Data::Integer(0),
        Data::Integer(0),
        Data::Integer(i128::from(Number::MAX) + 1),
    ]);
    assert!(matches!(
        InfoMessHeader::from_mms(&data),
        Err(ValueError::Range(_))
    ));
}

#[test]
fn the_buffer_round_trips_whatever_its_octets() {
    round_trip(&InfoBufXX(Vec::new()));
    round_trip(&InfoBufXX(vec![0x00]));
    round_trip(&InfoBufXX((0..=255u8).collect()));
    let long = InfoBufXX(vec![0xA5; 4096]);
    round_trip(&long);
}

#[test]
fn the_buffer_is_an_octet_string_and_refuses_another_kind() {
    assert_eq!(
        InfoBufXX(vec![1, 2, 3]).to_mms(),
        Data::OctetString(vec![1, 2, 3])
    );
    assert!(matches!(
        InfoBufXX::from_mms(&Data::Integer(3)),
        Err(ValueError::Kind { .. })
    ));
}
