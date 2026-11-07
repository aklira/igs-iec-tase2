// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The indication points (task M2): the quality flags, named and tested one by one; the two-bit codes; and a round trip
//! through MMS for each variant of the 802 types (6.2.1), with the refusals of a malformed value.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use igs_iec_tase2_model::builtin::{Integer16, Integer32};
use igs_iec_tase2_model::generated::iso503::TimeStampExtended;
use igs_iec_tase2_model::generated::iso802::{
    clause_6_2_1::{DataDiscrete, DataReal},
    COVCounter, DataDiscreteExtended, DataDiscreteQ, DataDiscreteQTimeTag,
    DataDiscreteQTimeTagExtended, DataFlags, DataRealExtended, DataRealQ, DataRealQTimeTag,
    DataRealQTimeTagExtended, DataState, DataStateExtended, DataStateQ, DataStateQTimeTag,
    DataStateQTimeTagExtended, DataStateSupplemental, DataStateSupplementalExtended,
    DataStateSupplementalQ, DataStateSupplementalQTimeTag, DataStateSupplementalQTimeTagExtended,
};
use igs_iec_tase2_model::indication::{
    CurrentSource, NormalValue, TimeStampQuality, TwoBitCode, Validity,
};
use igs_iec_tase2_model::mms::{MmsValue, ValueError};
use igs_iso9506_mms_codec::generated::types::Data;
use igs_iso9506_mms_codec::schema::BitString;

const VALIDITY: [Validity; 4] = [
    Validity::Valid,
    Validity::Held,
    Validity::Suspect,
    Validity::NotValid,
];
const SOURCES: [CurrentSource; 4] = [
    CurrentSource::Telemetered,
    CurrentSource::Calculated,
    CurrentSource::Entered,
    CurrentSource::Estimated,
];

fn round_trip<T: MmsValue + PartialEq + std::fmt::Debug>(value: &T) {
    let data = value.to_mms();
    assert_eq!(&T::from_mms(&data).unwrap(), value, "{data:?}");
}

#[test]
fn the_validity_is_bits_two_and_three_as_the_standard_maps_it() {
    // VALID=0, HELD=1 (bit 3), SUSPECT=2 (bit 2), NOTVALID=3 (both): clause 7.1.1.
    let base = DataFlags(0);
    assert_eq!(base.with_validity(Validity::Valid).0, 0x00);
    assert_eq!(base.with_validity(Validity::Held).0, 0x08);
    assert_eq!(base.with_validity(Validity::Suspect).0, 0x04);
    assert_eq!(base.with_validity(Validity::NotValid).0, 0x0C);
    assert_eq!(DataFlags(0x08).validity(), Validity::Held);
    assert_eq!(DataFlags(0x04).validity(), Validity::Suspect);
}

#[test]
fn the_current_source_is_bits_four_and_five() {
    // TELEMETERED=0, CALCULATED=1 (bit 5), ENTERED=2 (bit 4), ESTIMATED=3 (both).
    let base = DataFlags(0);
    assert_eq!(base.with_current_source(CurrentSource::Telemetered).0, 0x00);
    assert_eq!(base.with_current_source(CurrentSource::Calculated).0, 0x20);
    assert_eq!(base.with_current_source(CurrentSource::Entered).0, 0x10);
    assert_eq!(base.with_current_source(CurrentSource::Estimated).0, 0x30);
    assert_eq!(DataFlags(0x30).current_source(), CurrentSource::Estimated);
}

#[test]
fn the_normal_value_is_bit_six() {
    assert_eq!(DataFlags(0).with_normal_value(NormalValue::Normal).0, 0x00);
    assert_eq!(
        DataFlags(0).with_normal_value(NormalValue::Abnormal).0,
        0x40
    );
    assert_eq!(DataFlags(0x40).normal_value(), NormalValue::Abnormal);
    assert!(DataFlags(0x40).is_normal_value());
}

#[test]
fn the_time_stamp_quality_is_bit_seven() {
    assert_eq!(
        DataFlags(0)
            .with_time_stamp_quality(TimeStampQuality::Valid)
            .0,
        0x00
    );
    assert_eq!(
        DataFlags(0)
            .with_time_stamp_quality(TimeStampQuality::Invalid)
            .0,
        0x80
    );
    assert_eq!(
        DataFlags(0x80).time_stamp_quality(),
        TimeStampQuality::Invalid
    );
}

#[test]
fn every_combination_of_the_quality_flags_is_kept_and_the_unused_bit_too() {
    // Bit 0 of Data_Flags is unused: whatever it holds, the accessors keep it.
    for validity in VALIDITY {
        for source in SOURCES {
            for abnormal in [NormalValue::Normal, NormalValue::Abnormal] {
                for stamp in [TimeStampQuality::Valid, TimeStampQuality::Invalid] {
                    let flags = DataFlags(0x01)
                        .with_validity(validity)
                        .with_current_source(source)
                        .with_normal_value(abnormal)
                        .with_time_stamp_quality(stamp);
                    assert_eq!(flags.validity(), validity);
                    assert_eq!(flags.current_source(), source);
                    assert_eq!(flags.normal_value(), abnormal);
                    assert_eq!(flags.time_stamp_quality(), stamp);
                    assert!(flags.is_unused(), "the unused bit is kept");
                    round_trip(&flags);
                }
            }
        }
    }
}

#[test]
fn the_state_of_a_point_is_bits_zero_and_one_and_keeps_its_quality() {
    let quality = DataState(0)
        .with_validity(Validity::Suspect)
        .with_normal_value(NormalValue::Abnormal);
    for code in [
        TwoBitCode::Code00,
        TwoBitCode::Code01,
        TwoBitCode::Code10,
        TwoBitCode::Code11,
    ] {
        let state = quality.with_state(code);
        assert_eq!(state.state(), code);
        assert_eq!(state.validity(), Validity::Suspect, "the validity is kept");
        assert_eq!(
            state.normal_value(),
            NormalValue::Abnormal,
            "the normal value is kept"
        );
    }
    // State_hi is bit 0 (value 2 of the code), State_lo is bit 1 (value 1 of the code).
    assert_eq!(DataState(0).with_state(TwoBitCode::Code01).0, 0x02);
    assert_eq!(DataState(0).with_state(TwoBitCode::Code10).0, 0x01);
    assert_eq!(DataState(0).with_state(TwoBitCode::Code11).0, 0x03);
}

#[test]
fn a_supplemental_state_has_its_state_its_tag_and_its_expected_state() {
    // State: bits 0 and 1; Tag: bits 2 and 3; ExpectedState: bits 4 and 5.
    let value = DataStateSupplemental(0)
        .with_state(TwoBitCode::Code10)
        .with_tag(TwoBitCode::Code01)
        .with_expected_state(TwoBitCode::Code11);
    // State 10 is State_hi (0x01); Tag 01 is Tag_lo (0x08); ExpectedState 11 is 0x30.
    assert_eq!(value.0, 0x01 | 0x08 | 0x30);
    assert_eq!(value.state(), TwoBitCode::Code10);
    assert_eq!(value.tag(), TwoBitCode::Code01);
    assert_eq!(value.expected_state(), TwoBitCode::Code11);
    round_trip(&value);
}

#[test]
fn a_two_bit_code_is_its_value() {
    assert_eq!(TwoBitCode::from_bits(true, false).value(), 2);
    assert_eq!(TwoBitCode::from_bits(false, true).value(), 1);
    assert_eq!(TwoBitCode::from_bits(true, true).value(), 3);
}

#[test]
fn each_variant_of_a_real_point_round_trips() {
    let flags = DataFlags(0)
        .with_validity(Validity::Held)
        .with_current_source(CurrentSource::Entered);
    round_trip(&DataRealQ {
        value: DataReal(21.5),
        flags,
    });
    let time_stamp = Integer32::new(946_684_800).unwrap();
    round_trip(&DataRealQTimeTag {
        value: DataReal(21.5),
        time_stamp,
        flags,
    });
    let stamp = TimeStampExtended::new(
        Integer32::new(946_684_800).unwrap(),
        Integer16::new(250).unwrap(),
    )
    .unwrap();
    round_trip(&DataRealQTimeTagExtended {
        value: DataReal(21.5),
        time_stamp: stamp,
        flags,
    });
    round_trip(&DataRealExtended {
        value: DataReal(21.5),
        time_stamp,
        flags,
        cov: COVCounter::new(7).unwrap(),
    });
}

#[test]
fn each_variant_of_a_discrete_point_round_trips() {
    let flags = DataFlags(0).with_normal_value(NormalValue::Abnormal);
    let value = DataDiscrete::new(-12).unwrap();
    let time_stamp = Integer32::new(946_684_800).unwrap();
    let stamp = TimeStampExtended::new(
        Integer32::new(946_684_800).unwrap(),
        Integer16::new(1).unwrap(),
    )
    .unwrap();
    round_trip(&DataDiscreteQ { value, flags });
    round_trip(&DataDiscreteQTimeTag {
        value,
        time_stamp,
        flags,
    });
    round_trip(&DataDiscreteQTimeTagExtended {
        value,
        time_stamp: stamp,
        flags,
    });
    round_trip(&DataDiscreteExtended {
        value,
        time_stamp,
        flags,
        cov: COVCounter::new(0).unwrap(),
    });
}

#[test]
fn each_variant_of_a_state_point_round_trips() {
    let flags = DataState(0)
        .with_validity(Validity::NotValid)
        .with_state(TwoBitCode::Code11);
    let time_stamp = Integer32::new(946_684_800).unwrap();
    let stamp = TimeStampExtended::new(
        Integer32::new(946_684_800).unwrap(),
        Integer16::new(999).unwrap(),
    )
    .unwrap();
    round_trip(&DataStateQ {
        value: DataState(0).with_state(TwoBitCode::Code10),
        flags: DataFlags(0).with_validity(Validity::Suspect),
    });
    round_trip(&DataStateQTimeTag { time_stamp, flags });
    round_trip(&DataStateQTimeTagExtended {
        time_stamp: stamp,
        flags,
    });
    round_trip(&DataStateExtended {
        time_stamp,
        flags,
        cov: COVCounter::new(65_535).unwrap(),
    });
}

#[test]
fn each_variant_of_a_supplemental_state_point_round_trips() {
    let value = DataStateSupplemental(0)
        .with_state(TwoBitCode::Code01)
        .with_expected_state(TwoBitCode::Code10);
    let flags = DataFlags(0).with_time_stamp_quality(TimeStampQuality::Invalid);
    let time_stamp = Integer32::new(946_684_800).unwrap();
    let stamp = TimeStampExtended::new(
        Integer32::new(946_684_800).unwrap(),
        Integer16::new(5).unwrap(),
    )
    .unwrap();
    round_trip(&DataStateSupplementalQ { value, flags });
    round_trip(&DataStateSupplementalQTimeTag {
        value,
        time_stamp,
        flags,
    });
    round_trip(&DataStateSupplementalQTimeTagExtended {
        value,
        time_stamp: stamp,
        flags,
    });
    round_trip(&DataStateSupplementalExtended {
        value,
        time_stamp,
        flags,
        cov: COVCounter::new(1).unwrap(),
    });
}

#[test]
fn a_flag_structure_of_the_wrong_size_is_refused() {
    let short = Data::Structure(vec![Data::Integer(1)]);
    assert!(matches!(
        DataRealQ::from_mms(&short),
        Err(ValueError::Format(_))
    ));
    assert!(matches!(
        DataRealQ::from_mms(&Data::Integer(1)),
        Err(ValueError::Kind {
            expected: "structure",
            ..
        })
    ));
}

#[test]
fn a_bit_string_shorter_than_the_flags_or_with_a_bit_beyond_them_is_refused() {
    // Data_Flags has 8 bits: a string of 4 bits is too short.
    let short = Data::BitString(BitString {
        unused: 4,
        bytes: vec![0xF0],
    });
    assert!(matches!(
        DataFlags::from_mms(&short),
        Err(ValueError::Format(_))
    ));
    // A ninth bit set is beyond the flags: refused.
    let beyond = Data::BitString(BitString {
        unused: 7,
        bytes: vec![0x00, 0x80],
    });
    assert!(matches!(
        DataFlags::from_mms(&beyond),
        Err(ValueError::Format(_))
    ));
    // The same string with the ninth bit clear is accepted: unused trailing bits are not flags.
    let clear = Data::BitString(BitString {
        unused: 7,
        bytes: vec![0x00, 0x00],
    });
    assert_eq!(DataFlags::from_mms(&clear).unwrap(), DataFlags(0));
}

#[test]
fn the_flags_of_a_mms_bit_string_map_to_the_bits_in_order() {
    // Bit 2 (Validity_hi) is the third bit of the first octet, 0x20.
    let data = DataFlags(0x04).to_mms();
    assert_eq!(
        data,
        Data::BitString(BitString {
            unused: 0,
            bytes: vec![0x20]
        })
    );
}
