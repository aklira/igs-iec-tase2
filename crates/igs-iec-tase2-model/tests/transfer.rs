// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The transfer sets, their conditions and the supported features (task M4): the Boolean enumerations, the
//! components of the transfer sets in each of their copies (the main text and the annexes), the bit positions of the
//! condition and feature types, and the names of the main text, which prevail over the annexes (decision D-003).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::any::TypeId;

use igs_iec_tase2_model::builtin::{Integer16, Integer32, MmsIdentifier};
use igs_iec_tase2_model::generated::iso503::{
    self, clause_8_2_7_1, clause_8_2_7_2, clause_8_2_8_2, clause_8_2_9, clause_g_2_2, clause_g_2_3,
    clause_g_2_4, clause_g_4, DSConditions, DSTransferSet, DSTransferSetAllChangesReported,
    DSTransferSetBlockData, DSTransferSetCritical, DSTransferSetRBE, DSTransferSetStatus,
    MMSObjectName, MMSObjectNameScope, TAConditions, TSConditions, TSTransferSet,
};
use igs_iec_tase2_model::mms::{MmsValue, ValueError};
use igs_iso9506_mms_codec::generated::types::Data;
use igs_iso9506_mms_codec::schema::BitString;

fn round_trip<T: MmsValue + PartialEq + std::fmt::Debug>(value: &T) {
    let data = value.to_mms();
    assert_eq!(&T::from_mms(&data).unwrap(), value, "{data:?}");
}

fn gmt(seconds: i64) -> Integer32 {
    Integer32::new(seconds).unwrap()
}

fn object_name(domain: &str, item: &str) -> MMSObjectName {
    MMSObjectName {
        scope: MMSObjectNameScope(MMSObjectNameScope::DOMAIN_SPECIFIC),
        domain_name: MmsIdentifier::new(domain).unwrap(),
        name: MmsIdentifier::new(item).unwrap(),
    }
}

#[test]
fn a_boolean_enumeration_is_a_boolean_with_the_names_of_the_standard() {
    assert_eq!(
        DSTransferSetBlockData(DSTransferSetBlockData::TRUE).to_mms(),
        Data::Boolean(true)
    );
    assert_eq!(
        DSTransferSetBlockData(DSTransferSetBlockData::FALSE).to_mms(),
        Data::Boolean(false)
    );
    round_trip(&DSTransferSetBlockData(true));
    round_trip(&DSTransferSetBlockData(false));
    assert_eq!(DSTransferSetBlockData(true).to_mms(), Data::Boolean(true));
    assert!(matches!(
        DSTransferSetBlockData::from_mms(&Data::Integer(1)),
        Err(ValueError::Kind { .. })
    ));
}

#[test]
fn a_status_is_enabled_or_disabled() {
    assert_eq!(
        DSTransferSetStatus(DSTransferSetStatus::ENABLED).to_mms(),
        Data::Boolean(true)
    );
    assert_eq!(
        DSTransferSetStatus(DSTransferSetStatus::DISABLED).to_mms(),
        Data::Boolean(false)
    );
    round_trip(&DSTransferSetStatus(DSTransferSetStatus::ENABLED));
    round_trip(&clause_8_2_7_2::TSTransferSetStatus(
        clause_8_2_7_2::TSTransferSetStatus::DISABLED,
    ));
    assert!(matches!(
        clause_8_2_7_2::TATransferSetStatus::from_mms(&Data::Integer(2)),
        Err(ValueError::Kind { .. })
    ));
}

#[test]
fn a_data_set_transfer_set_round_trips_with_its_components_in_order() {
    let value = DSTransferSet {
        data_set_name: object_name("ICC1", "DataSet1"),
        start_time: gmt(1_700_000_000),
        interval: gmt(60),
        tle: gmt(600),
        buffer_time: gmt(5),
        integrity_check: gmt(900),
        ds_conditions_requested: DSConditions(
            DSConditions::INTERVAL_TIME_OUT | DSConditions::OBJECT_CHANGE,
        ),
        block_data: DSTransferSetBlockData(true),
        critical: DSTransferSetCritical(false),
        rbe: DSTransferSetRBE(true),
        all_changes_reported: DSTransferSetAllChangesReported(false),
        status: DSTransferSetStatus(DSTransferSetStatus::ENABLED),
        event_code_requested: Integer16::new(3).unwrap(),
    };
    round_trip(&value);
    match value.to_mms() {
        Data::Structure(items) => {
            assert_eq!(items.len(), 13);
            // The block data is the eighth component and the status the twelfth: the order of the standard.
            assert_eq!(items[7], Data::Boolean(true));
            assert_eq!(items[11], Data::Boolean(true));
            assert_eq!(items[12], Data::Integer(3));
        }
        other => panic!("a structure is expected: {other:?}"),
    }
}

#[test]
fn the_scopes_of_an_object_name_are_those_of_the_standard() {
    assert_eq!(MMSObjectNameScope::VMD_SPECIFIC, 0);
    assert_eq!(MMSObjectNameScope::DOMAIN_SPECIFIC, 1);
    round_trip(&object_name("ICC1", "Next_DSTransfer_Set"));
}

#[test]
fn the_three_copies_of_the_ts_transfer_set_have_the_same_components() {
    // The main text (8.2.7.2) and the two annexes (G.2.2 and G.4) describe the same structure: each copy is built with
    // the same values, so a copy that lost a component would not compile.
    let main = TSTransferSet {
        data_value_name: object_name("ICC1", "Flow1"),
        ts_conditions_requested: TSConditions(TSConditions::MAX_MMSPDU),
        block_data: clause_8_2_7_2::TSTransferSetBlockData(false),
        begin_time: gmt(1_700_000_000),
        end_time: gmt(1_700_003_600),
        sampling_interval: gmt(10),
        reporting_interval: gmt(300),
        status: clause_8_2_7_2::TSTransferSetStatus(clause_8_2_7_2::TSTransferSetStatus::ENABLED),
    };
    round_trip(&main);
    let annex_g2 = clause_g_2_2::TSTransferSet {
        data_value_name: object_name("ICC1", "Flow1"),
        ts_conditions_requested: TSConditions(TSConditions::MAX_MMSPDU),
        block_data: clause_g_2_2::TSTransferSetBlockData(false),
        begin_time: gmt(1_700_000_000),
        end_time: gmt(1_700_003_600),
        sampling_interval: gmt(10),
        reporting_interval: gmt(300),
        status: clause_g_2_2::TSTransferSetStatus(clause_g_2_2::TSTransferSetStatus::ENABLED),
    };
    round_trip(&annex_g2);
    let annex_g4 = clause_g_4::TSTransferSet {
        data_value_name: object_name("ICC1", "Flow1"),
        ts_conditions_requested: TSConditions(TSConditions::MAX_MMSPDU),
        block_data: clause_g_4::TSTransferSetBlockData(false),
        begin_time: gmt(1_700_000_000),
        end_time: gmt(1_700_003_600),
        sampling_interval: gmt(10),
        reporting_interval: gmt(300),
        status: clause_g_4::TSTransferSetStatus(clause_g_4::TSTransferSetStatus::ENABLED),
    };
    round_trip(&annex_g4);
}

#[test]
fn the_ta_and_im_transfer_sets_round_trip_in_each_copy() {
    let ta = clause_8_2_7_2::TATransferSetBlockData(true);
    round_trip(&ta);
    round_trip(&clause_8_2_7_2::TATransferSet {
        ta_conditions_requested: TAConditions(TAConditions::AFTER_THE_HOUR),
        block_data: clause_8_2_7_2::TATransferSetBlockData(true),
        status: clause_8_2_7_2::TATransferSetStatus(clause_8_2_7_2::TATransferSetStatus::DISABLED),
    });
    round_trip(&clause_g_2_2::TATransferSet {
        ta_conditions_requested: TAConditions(TAConditions::PAST_HOURS),
        block_data: clause_g_2_2::TATransferSetBlockData(false),
        status: clause_g_2_2::TATransferSetStatus(clause_g_2_2::TATransferSetStatus::ENABLED),
    });
    round_trip(&clause_g_4::TATransferSet {
        ta_conditions_requested: TAConditions(TAConditions::OBJECT_CHANGE),
        block_data: clause_g_4::TATransferSetBlockData(true),
        status: clause_g_4::TATransferSetStatus(clause_g_4::TATransferSetStatus::ENABLED),
    });
    // The interval-moving transfer set is a Boolean in each of its copies.
    round_trip(&clause_8_2_7_1::IMTransferSet(true));
    round_trip(&clause_g_2_2::IMTransferSet(false));
    round_trip(&clause_g_4::IMTransferSet(true));
}

#[test]
fn the_bits_of_the_ds_conditions_are_at_the_positions_of_the_standard() {
    assert_eq!(DSConditions::BIT_COUNT, 5);
    assert_eq!(DSConditions::INTERVAL_TIME_OUT, 1 << 0);
    assert_eq!(DSConditions::INTEGRITY_TIME_OUT, 1 << 1);
    assert_eq!(DSConditions::OBJECT_CHANGE, 1 << 2);
    assert_eq!(DSConditions::OPERATOR_REQUEST, 1 << 3);
    assert_eq!(DSConditions::OTHER_EXTERNAL_EVENT, 1 << 4);
    for bits in 0..32 {
        round_trip(&DSConditions(bits));
    }
    // A bit beyond the five of the string is refused: the fifth bit of the first octet is the last one allowed.
    let beyond = Data::BitString(BitString {
        unused: 2,
        bytes: vec![0b0000_0100],
    });
    assert!(matches!(
        DSConditions::from_mms(&beyond),
        Err(ValueError::Format(_))
    ));
}

#[test]
fn the_bits_of_the_ts_conditions_are_at_the_positions_of_the_standard() {
    assert_eq!(TSConditions::BIT_COUNT, 4);
    assert_eq!(TSConditions::END_TIME_ARRIVED, 1 << 0);
    assert_eq!(TSConditions::OPERATOR_REQUEST, 1 << 1);
    assert_eq!(TSConditions::REPORT_INTERVAL_TIME_OUT, 1 << 2);
    assert_eq!(TSConditions::MAX_MMSPDU, 1 << 3);
    for bits in 0..16 {
        round_trip(&TSConditions(bits));
    }
}

#[test]
fn the_bits_of_the_ta_conditions_are_at_the_positions_of_the_standard() {
    assert_eq!(TAConditions::BIT_COUNT, 8);
    assert_eq!(TAConditions::BEFORE_THE_HOUR, 1 << 0);
    assert_eq!(TAConditions::DISPATCH_UPDATE, 1 << 1);
    assert_eq!(TAConditions::DURING_THE_HOUR, 1 << 2);
    assert_eq!(TAConditions::AFTER_THE_HOUR, 1 << 3);
    assert_eq!(TAConditions::ACTUAL_DATA_UPDATE, 1 << 4);
    assert_eq!(TAConditions::PAST_HOURS, 1 << 5);
    assert_eq!(TAConditions::OBJECT_CHANGE, 1 << 6);
    assert_eq!(TAConditions::OPERATOR_REQUEST, 1 << 7);
    for bits in 0..=255 {
        round_trip(&TAConditions(bits));
    }
}

#[test]
fn the_supported_features_keep_every_bit_of_their_twelve() {
    assert_eq!(clause_8_2_9::SupportedFeatures::BIT_COUNT, 12);
    assert_eq!(clause_8_2_9::SupportedFeatures::BLOCK1, 1 << 0);
    assert_eq!(clause_8_2_9::SupportedFeatures::BLOCK2, 1 << 1);
    assert_eq!(clause_8_2_9::SupportedFeatures::BLOCK4, 1 << 3);
    assert_eq!(clause_8_2_9::SupportedFeatures::BLOCK5, 1 << 4);
    assert_eq!(clause_8_2_9::SupportedFeatures::BLOCK11, 1 << 10);
    assert_eq!(clause_8_2_9::SupportedFeatures::BLOCK12, 1 << 11);
    // The bits named reserved have no constant: every value of the twelve bits still round-trips.
    for bits in 0..4096 {
        round_trip(&clause_8_2_9::SupportedFeatures(bits));
    }
}

#[test]
fn the_main_text_names_prevail_over_the_annexes() {
    // The root names are the definitions of the main text (decision D-003).
    assert_eq!(
        TypeId::of::<iso503::TSTransferSet>(),
        TypeId::of::<clause_8_2_7_2::TSTransferSet>()
    );
    assert_eq!(
        TypeId::of::<iso503::TSConditions>(),
        TypeId::of::<clause_8_2_8_2::TSConditions>()
    );
    assert_eq!(
        TypeId::of::<iso503::SupportedFeatures>(),
        TypeId::of::<clause_8_2_9::SupportedFeatures>()
    );
    assert_eq!(
        TypeId::of::<iso503::IMTransferSet>(),
        TypeId::of::<clause_8_2_7_1::IMTransferSet>()
    );
    // The annex describes the earlier edition: its conditions and features differ from the main text in their
    // names, and the annex keeps them.
    assert_eq!(clause_g_2_4::TSConditions::MAI_MMSPDU, 1 << 3);
    assert_eq!(clause_g_2_3::SupportedFeatures::BLOCK3, 1 << 2);
    assert_ne!(
        TypeId::of::<iso503::SupportedFeatures>(),
        TypeId::of::<clause_g_2_3::SupportedFeatures>()
    );
}
