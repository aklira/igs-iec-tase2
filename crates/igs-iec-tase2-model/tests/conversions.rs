// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The conversions of the TASE.2 scalars and time types to and from MMS data (task M1): round trips, vectors written by
//! hand from the encodings of the standards, the range checks, the string checks, and the refusal of another kind.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::time::{Duration, UNIX_EPOCH};

use igs_iec_tase2_model::builtin::{Integer16, Integer32, MmsIdentifier, Unsigned8};
use igs_iec_tase2_model::generated::iso503::{TimeStampExtended, VisibleString32};
use igs_iec_tase2_model::generated::iso802::ControlSetpointReal;
use igs_iec_tase2_model::generated::iso802::{
    self, clause_6_2_1, clause_6_3, clause_6_4, COVCounter, TextString,
};
use igs_iec_tase2_model::mms::{MmsValue, ValueError};
use igs_iec_tase2_model::value::{OutOfRange, StringError};
use igs_iso9506_mms_codec::generated::types::{Data, FloatingPoint};

/// The MMS value of a value, then the value of that MMS value: it must give the value back.
fn round_trip<T: MmsValue + PartialEq + std::fmt::Debug>(value: &T) {
    let data = value.to_mms();
    assert_eq!(&T::from_mms(&data).unwrap(), value, "{data:?}");
}

#[test]
fn integers_round_trip_at_their_limits() {
    for value in [i64::from(i16::MIN), 0, i64::from(i16::MAX)] {
        round_trip(&clause_6_3::Number::new(value).unwrap());
    }
    for value in [i64::from(i32::MIN), -1, 0, i64::from(i32::MAX)] {
        round_trip(&clause_6_2_1::DataDiscrete::new(value).unwrap());
        round_trip(&Integer32::new(value).unwrap());
    }
    round_trip(&Integer16::new(-32_768).unwrap());
    round_trip(&Integer16::new(32_767).unwrap());
}

#[test]
fn unsigned_integers_round_trip_at_their_limits() {
    round_trip(&COVCounter::new(0).unwrap());
    round_trip(&COVCounter::new(65_535).unwrap());
    round_trip(&Unsigned8::new(255).unwrap());
}

#[test]
fn floating_points_round_trip_when_they_are_single_precision() {
    // Values that a single precision number holds exactly: the wire has single precision (decision D-013).
    for value in [21.5, -0.0, 1.5e-40, f64::from(f32::MAX), -1.0 / 3.0] {
        let value = f64::from(value as f32);
        round_trip(&clause_6_2_1::DataReal(value));
    }
}

#[test]
fn a_boolean_and_strings_round_trip() {
    round_trip(&iso802::SelectState(true));
    round_trip(&iso802::SelectState(false));
    round_trip(&TextString::new("igs").unwrap());
    round_trip(&VisibleString32::new("Bilateral table 1").unwrap());
    round_trip(&MmsIdentifier::new("IdentifierOf32Characters1234567").unwrap());
}

#[test]
fn time_types_round_trip() {
    round_trip(
        &TimeStampExtended::new(
            Integer32::new(946_684_800).unwrap(),
            Integer16::new(999).unwrap(),
        )
        .unwrap(),
    );
    round_trip(
        &TimeStampExtended::new(Integer32::new(0).unwrap(), Integer16::new(0).unwrap()).unwrap(),
    );
}

#[test]
fn a_real_is_the_ieee_single_precision_octets_with_exponent_width_eight() {
    // 21.5 is 0x41AC0000 in single precision: the first octet is the exponent width (8).
    let data = clause_6_2_1::DataReal(21.5).to_mms();
    assert_eq!(
        data,
        Data::FloatingPoint(FloatingPoint(vec![8, 0x41, 0xAC, 0x00, 0x00]))
    );
}

#[test]
fn integers_and_counters_map_to_mms_integers_and_unsigneds() {
    assert_eq!(
        clause_6_2_1::DataDiscrete::new(-1).unwrap().to_mms(),
        Data::Integer(-1)
    );
    assert_eq!(COVCounter::new(300).unwrap().to_mms(), Data::Unsigned(300));
    assert_eq!(
        TextString::new("igs").unwrap().to_mms(),
        Data::VisibleString("igs".to_string())
    );
}

#[test]
fn a_gmt_time_counts_seconds_from_1970_as_the_standard_says() {
    // 2000-01-01T00:00:00Z is 946 684 800 seconds after 1970-01-01T00:00:00Z.
    let stamp =
        TimeStampExtended::from_system_time(UNIX_EPOCH + Duration::from_secs(946_684_800)).unwrap();
    assert_eq!(stamp.gmt_based_s.get(), 946_684_800);
    assert_eq!(stamp.milliseconds.get(), 0);
    assert_eq!(
        stamp.to_mms(),
        Data::Structure(vec![Data::Integer(946_684_800), Data::Integer(0)])
    );
}

#[test]
fn the_limits_of_the_gmt_seconds_are_the_dates_of_32_bits() {
    // -2^31 seconds is 1901-12-13T20:45:52Z; 2^31 - 1 seconds is 2038-01-19T03:14:07Z.
    let least =
        TimeStampExtended::from_system_time(UNIX_EPOCH - Duration::from_secs(2_147_483_648))
            .unwrap();
    assert_eq!(least.gmt_based_s.get(), i64::from(i32::MIN));
    let greatest =
        TimeStampExtended::from_system_time(UNIX_EPOCH + Duration::from_secs(2_147_483_647))
            .unwrap();
    assert_eq!(greatest.gmt_based_s.get(), i64::from(i32::MAX));
    // One second after the greatest, and one second before the least, are out of range.
    assert!(matches!(
        TimeStampExtended::from_system_time(UNIX_EPOCH + Duration::from_secs(2_147_483_648)),
        Err(ValueError::Range(_))
    ));
    assert!(matches!(
        TimeStampExtended::from_system_time(UNIX_EPOCH - Duration::from_secs(2_147_483_649)),
        Err(ValueError::Range(_))
    ));
}

#[test]
fn a_time_before_1970_is_rounded_down_to_the_millisecond() {
    // Half a second before the epoch: GMT second -1, milliseconds 500.
    let stamp =
        TimeStampExtended::from_system_time(UNIX_EPOCH - Duration::from_millis(500)).unwrap();
    assert_eq!(stamp.gmt_based_s.get(), -1);
    assert_eq!(stamp.milliseconds.get(), 500);
    assert_eq!(
        stamp.to_system_time(),
        UNIX_EPOCH - Duration::from_millis(500)
    );
}

#[test]
fn a_time_stamp_of_thousand_milliseconds_is_refused() {
    assert!(matches!(
        TimeStampExtended::new(Integer32::new(0).unwrap(), Integer16::new(1000).unwrap()),
        Err(ValueError::Range(OutOfRange {
            value: 1000,
            min: 0,
            max: 999
        }))
    ));
    let mms = Data::Structure(vec![Data::Integer(0), Data::Integer(1000)]);
    assert!(TimeStampExtended::from_mms(&mms).is_err());
}

#[test]
fn a_time_stamp_round_trips_to_system_time_with_its_milliseconds() {
    let time = UNIX_EPOCH + Duration::from_millis(946_684_800_999);
    let stamp = TimeStampExtended::from_system_time(time).unwrap();
    assert_eq!(stamp.milliseconds.get(), 999);
    assert_eq!(stamp.to_system_time(), time);
}

#[test]
fn an_integer_outside_its_range_is_refused_from_mms() {
    assert!(matches!(
        Integer16::from_mms(&Data::Integer(32_768)),
        Err(ValueError::Range(OutOfRange {
            value: 32_768,
            min: -32_768,
            max: 32_767
        }))
    ));
    assert!(matches!(
        Integer32::from_mms(&Data::Integer(i128::from(i32::MAX) + 1)),
        Err(ValueError::Range(_))
    ));
    assert!(matches!(
        Unsigned8::from_mms(&Data::Unsigned(256)),
        Err(ValueError::Range(_))
    ));
    assert!(matches!(
        COVCounter::from_mms(&Data::Unsigned(65_536)),
        Err(ValueError::Range(_))
    ));
    assert!(matches!(Integer16::new(32_768), Err(OutOfRange { .. })));
}

#[test]
fn a_value_of_another_kind_is_refused() {
    assert_eq!(
        Integer32::from_mms(&Data::Boolean(true)),
        Err(ValueError::Kind {
            expected: "integer",
            found: "boolean"
        })
    );
    assert!(matches!(
        clause_6_2_1::DataReal::from_mms(&Data::Integer(1)),
        Err(ValueError::Kind {
            expected: "floating point",
            ..
        })
    ));
    assert!(matches!(
        TextString::from_mms(&Data::Integer(1)),
        Err(ValueError::Kind {
            expected: "visible string",
            ..
        })
    ));
    assert!(matches!(
        TimeStampExtended::from_mms(&Data::Integer(1)),
        Err(ValueError::Kind {
            expected: "structure",
            ..
        })
    ));
}

#[test]
fn a_real_that_is_not_single_precision_is_refused() {
    let double = Data::FloatingPoint(FloatingPoint(vec![11, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert!(matches!(
        clause_6_2_1::DataReal::from_mms(&double),
        Err(ValueError::Format(_))
    ));
    let short = Data::FloatingPoint(FloatingPoint(vec![8, 0, 0, 0]));
    assert!(matches!(
        clause_6_2_1::DataReal::from_mms(&short),
        Err(ValueError::Format(_))
    ));
}

#[test]
fn a_string_longer_than_its_type_is_refused() {
    assert_eq!(
        VisibleString32::new("x".repeat(33)),
        Err(StringError::TooLong {
            length: 33,
            max: 32
        })
    );
    assert!(VisibleString32::new("x".repeat(32)).is_ok());
    assert!(TextString::new("x".repeat(255)).is_ok());
    assert!(TextString::new("x".repeat(256)).is_err());
    assert!(MmsIdentifier::new("x".repeat(33)).is_err());
    assert!(matches!(
        VisibleString32::from_mms(&Data::VisibleString("x".repeat(40))),
        Err(ValueError::Text(StringError::TooLong { .. }))
    ));
}

#[test]
fn a_character_that_is_not_visible_is_refused() {
    assert_eq!(
        VisibleString32::new("a\tb"),
        Err(StringError::NotVisible('\t'))
    );
    assert_eq!(MmsIdentifier::new("é"), Err(StringError::NotVisible('é')));
    assert!(VisibleString32::new(" ~!").is_ok());
}

#[test]
fn the_generated_types_name_their_clause_and_the_base_types_are_by_hand() {
    // The generated code keeps the definition of each clause (decision D-001): both Number definitions exist.
    assert_eq!(clause_6_3::Number::WIDTH, 16);
    assert_eq!(clause_6_4::Number::WIDTH, 32);
}

#[test]
fn a_real_that_is_not_a_number_or_infinite_is_refused() {
    // Single precision: exponent width 8, then the octets. All ones in the exponent: an infinity, or a NaN.
    let infinity = Data::FloatingPoint(FloatingPoint(vec![8, 0x7f, 0x80, 0, 0]));
    let nan = Data::FloatingPoint(FloatingPoint(vec![8, 0x7f, 0xc0, 0, 0]));
    let negative_infinity = Data::FloatingPoint(FloatingPoint(vec![8, 0xff, 0x80, 0, 0]));
    for data in [infinity, nan, negative_infinity] {
        assert!(matches!(
            ControlSetpointReal::from_mms(&data),
            Err(ValueError::Format(_))
        ));
    }
    // The largest finite single-precision value is read.
    let largest = Data::FloatingPoint(FloatingPoint(vec![8, 0x7f, 0x7f, 0xff, 0xff]));
    assert!(ControlSetpointReal::from_mms(&largest).is_ok());
}
