// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The protection equipment events (task M5): the flags of a single event, of a packed event and of the start and trip
//! of the phases, each bit at its position; the event state, with its named values; and a round trip through MMS for
//! each of the two event structures.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use igs_iec_tase2_model::builtin::{Integer16, Integer32};
use igs_iec_tase2_model::generated::iso503::TimeStampExtended;
use igs_iec_tase2_model::generated::iso802::{
    EventFlags, PackedFlags, PackedProtectionEvent, SingleFlags, SingleProtectionEvent,
};
use igs_iec_tase2_model::indication::TwoBitCode;
use igs_iec_tase2_model::mms::{MmsValue, ValueError};
use igs_iec_tase2_model::protection::ProtectionEventState;
use igs_iso9506_mms_codec::generated::types::Data;
use igs_iso9506_mms_codec::schema::BitString;

fn round_trip<T: MmsValue + PartialEq + std::fmt::Debug>(value: &T) {
    let data = value.to_mms();
    assert_eq!(&T::from_mms(&data).unwrap(), value, "{data:?}");
}

fn event_time() -> TimeStampExtended {
    TimeStampExtended::new(
        Integer32::new(1_700_000_000).unwrap(),
        Integer16::new(250).unwrap(),
    )
    .unwrap()
}

#[test]
fn the_bits_of_a_single_event_are_at_the_positions_of_the_standard() {
    assert_eq!(SingleFlags::BIT_COUNT, 8);
    assert_eq!(SingleFlags::ELAPSED_TIME_VALIDITY, 1 << 0);
    assert_eq!(SingleFlags::BLOCKED, 1 << 1);
    assert_eq!(SingleFlags::SUBSTITUTED, 1 << 2);
    assert_eq!(SingleFlags::TOPICAL, 1 << 3);
    assert_eq!(SingleFlags::EVENT_VALIDITY, 1 << 4);
    assert_eq!(SingleFlags::EVENT_STATE_HI, 1 << 6);
    assert_eq!(SingleFlags::EVENT_STATE_LO, 1 << 7);
    for bits in 0..256 {
        round_trip(&SingleFlags(bits));
    }
}

#[test]
fn the_bits_of_a_packed_event_are_at_the_positions_of_the_standard() {
    assert_eq!(PackedFlags::BIT_COUNT, 8);
    assert_eq!(PackedFlags::ELAPSED_TIME_VALIDITY, 1 << 0);
    assert_eq!(PackedFlags::BLOCKED, 1 << 1);
    assert_eq!(PackedFlags::SUBSTITUTED, 1 << 2);
    assert_eq!(PackedFlags::TOPICAL, 1 << 3);
    assert_eq!(PackedFlags::EVENT_VALIDITY, 1 << 4);
    for bits in 0..256 {
        round_trip(&PackedFlags(bits));
    }
}

#[test]
fn the_bits_of_the_start_and_trip_of_the_phases_are_at_the_positions_of_the_standard() {
    assert_eq!(EventFlags::BIT_COUNT, 8);
    assert_eq!(EventFlags::GENERAL, 1 << 0);
    assert_eq!(EventFlags::PHASE1, 1 << 1);
    assert_eq!(EventFlags::PHASE2, 1 << 2);
    assert_eq!(EventFlags::PHASE3, 1 << 3);
    assert_eq!(EventFlags::EARTH, 1 << 4);
    assert_eq!(EventFlags::REVERSE, 1 << 5);
    for bits in 0..256 {
        round_trip(&EventFlags(bits));
    }
    let flags = EventFlags(EventFlags::PHASE2 | EventFlags::EARTH);
    assert!(!flags.general());
    assert!(flags.phase2());
    assert!(flags.earth());
    assert!(!flags.reverse());
}

#[test]
fn an_event_state_is_off_on_or_invalid_as_the_mapping_says() {
    assert_eq!(
        ProtectionEventState::from_code(TwoBitCode::Code01),
        ProtectionEventState::Off
    );
    assert_eq!(
        ProtectionEventState::from_code(TwoBitCode::Code10),
        ProtectionEventState::On
    );
    assert_eq!(
        ProtectionEventState::from_code(TwoBitCode::Code00),
        ProtectionEventState::Invalid
    );
    assert_eq!(
        ProtectionEventState::from_code(TwoBitCode::Code11),
        ProtectionEventState::Invalid
    );
    // The low bit is EventState_lo (bit 7): an event with only that bit set is OFF.
    assert_eq!(
        SingleFlags(SingleFlags::EVENT_STATE_LO).event_state(),
        ProtectionEventState::Off
    );
    assert_eq!(
        SingleFlags(SingleFlags::EVENT_STATE_HI).event_state(),
        ProtectionEventState::On
    );
}

#[test]
fn the_state_is_written_and_the_other_bits_are_kept() {
    let blocked = SingleFlags(SingleFlags::BLOCKED | SingleFlags::TOPICAL);
    for state in [
        ProtectionEventState::Off,
        ProtectionEventState::On,
        ProtectionEventState::Invalid,
    ] {
        let written = blocked.with_event_state(state);
        assert_eq!(written.event_state(), state);
        assert!(written.blocked());
        // The bit 3 is set in the value: the event is NOTTOPICAL.
        assert!(written.not_topical());
    }
}

#[test]
fn an_inverted_attribute_is_named_by_its_condition() {
    // A set bit 3 is NOTTOPICAL; a clear bit is TOPICAL, the inverse of the bit.
    assert!(!SingleFlags(0).not_topical());
    assert!(SingleFlags(SingleFlags::TOPICAL).not_topical());
    assert!(PackedFlags(PackedFlags::TOPICAL).not_topical());
    assert!(SingleFlags(SingleFlags::ELAPSED_TIME_VALIDITY).elapsed_time_invalid());
    assert!(SingleFlags(SingleFlags::SUBSTITUTED).substituted());
    assert!(PackedFlags(PackedFlags::EVENT_VALIDITY).event_invalid());
}

#[test]
fn a_single_protection_event_round_trips_with_its_time_and_its_flags() {
    round_trip(&SingleProtectionEvent {
        single_event_flags: SingleFlags(SingleFlags::BLOCKED | SingleFlags::EVENT_STATE_LO),
        operating_time: Integer16::new(120).unwrap(),
        event_time: event_time(),
    });
}

#[test]
fn a_packed_protection_event_round_trips_with_its_phases() {
    round_trip(&PackedProtectionEvent {
        packed_event: EventFlags(EventFlags::GENERAL | EventFlags::PHASE3),
        packed_event_flags: PackedFlags(PackedFlags::SUBSTITUTED),
        operating_time: Integer16::new(35).unwrap(),
        event_time: event_time(),
    });
}

#[test]
fn a_bit_beyond_the_flags_of_the_event_is_refused() {
    // Eight bits: a bit at position 8, the first bit of the second octet, is beyond them.
    let beyond = Data::BitString(BitString {
        unused: 7,
        bytes: vec![0x00, 0x80],
    });
    assert!(matches!(
        SingleFlags::from_mms(&beyond),
        Err(ValueError::Format(_))
    ));
}

#[test]
fn a_structure_of_the_wrong_length_is_refused() {
    let data = Data::Structure(vec![Data::BitString(BitString {
        unused: 0,
        bytes: vec![0],
    })]);
    assert!(matches!(
        SingleProtectionEvent::from_mms(&data),
        Err(ValueError::Format(_))
    ));
    assert!(matches!(
        PackedProtectionEvent::from_mms(&data),
        Err(ValueError::Format(_))
    ));
}
