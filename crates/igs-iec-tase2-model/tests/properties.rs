// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! Property tests of the types (task M6): random MMS data is read without a panic, a value that is read is read back the
//! same once written, a valid value round-trips through MMS data and through its BER encoding, and data close to a valid
//! value (one component replaced, or dropped) is read without a panic. Each property runs 10 000 cases.

#![cfg(feature = "strategy")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::fmt::Debug;

use igs_iec_tase2_model::builtin::{Integer16, Integer32, MmsIdentifier, Unsigned8};
use igs_iec_tase2_model::generated::iso503::{
    DSConditions, DSTransferSet, DSTransferSetBlockData, IMTransferSet, MMSObjectName,
    SupportedFeatures, TAConditions, TATransferSet, TSConditions, TSTransferSet, TimeStampExtended,
};
use igs_iec_tase2_model::generated::iso802::{
    COVCounter, ControlCommand, ControlConfig, ControlSetpointDiscrete, ControlSetpointReal,
    DOActual, DOCancel, DONewRevSched, DataFlags, DataState, DataStateSupplemental, EventFlags,
    PackedFlags, PackedProtectionEvent, SBOCheckBackName, SelectState, SingleFlags,
    SingleProtectionEvent, TagFlags, TagValue, TextString, SBO,
};
use igs_iec_tase2_model::mms::MmsValue;
use igs_iec_tase2_model::strategy as strat;
use igs_iso9506_mms_codec::asn;
use igs_iso9506_mms_codec::generated::types::Data;
use igs_iso9506_mms_codec::strategy::values;
use proptest::prelude::*;
use proptest::test_runner::TestCaseError;

const CASES: u32 = 10_000;

fn config() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

/// Reading any data is an answer, accepted or refused: it never panics.
fn reads<T: MmsValue>(data: &Data) -> Result<(), TestCaseError> {
    let _ = T::from_mms(data);
    Ok(())
}

/// A value that is read is read back the same once it is written.
fn writes_back<T: MmsValue + PartialEq + Debug>(data: &Data) -> Result<(), TestCaseError> {
    if let Ok(value) = T::from_mms(data) {
        let written = value.to_mms();
        let again = T::from_mms(&written)
            .map_err(|error| TestCaseError::fail(format!("refused after writing: {error:?}")))?;
        prop_assert_eq!(&again, &value);
    }
    Ok(())
}

/// A valid value round-trips through its MMS data, and through the BER encoding of that data.
fn round_trips<T: MmsValue + PartialEq + Debug>(value: &T) -> Result<(), TestCaseError> {
    let data = value.to_mms();
    let back = T::from_mms(&data)
        .map_err(|error| TestCaseError::fail(format!("refused after writing: {error:?}")))?;
    prop_assert_eq!(&back, value);
    let bytes = asn::encode_value::<Data>(&data)
        .map_err(|error| TestCaseError::fail(format!("not encoded: {error:?}")))?;
    let decoded = asn::decode_value::<Data>(&bytes)
        .map_err(|error| TestCaseError::fail(format!("not decoded: {error:?}")))?;
    prop_assert_eq!(&decoded, &data);
    let back = T::from_mms(&decoded)
        .map_err(|error| TestCaseError::fail(format!("refused after the wire: {error:?}")))?;
    prop_assert_eq!(&back, value);
    Ok(())
}

/// Applies `check` to every type of the model that the properties cover.
macro_rules! for_every_type {
    ($check:ident, $data:expr) => {{
        $check::<Integer16>($data)?;
        $check::<Integer32>($data)?;
        $check::<Unsigned8>($data)?;
        $check::<COVCounter>($data)?;
        $check::<MmsIdentifier>($data)?;
        $check::<TimeStampExtended>($data)?;
        $check::<TextString>($data)?;
        $check::<DataFlags>($data)?;
        $check::<DataState>($data)?;
        $check::<DataStateSupplemental>($data)?;
        $check::<SingleFlags>($data)?;
        $check::<EventFlags>($data)?;
        $check::<PackedFlags>($data)?;
        $check::<TagFlags>($data)?;
        $check::<SelectState>($data)?;
        $check::<SBO>($data)?;
        $check::<TagValue>($data)?;
        $check::<ControlConfig>($data)?;
        $check::<ControlCommand>($data)?;
        $check::<ControlSetpointReal>($data)?;
        $check::<ControlSetpointDiscrete>($data)?;
        $check::<SBOCheckBackName>($data)?;
        $check::<SingleProtectionEvent>($data)?;
        $check::<PackedProtectionEvent>($data)?;
        $check::<DONewRevSched>($data)?;
        $check::<DOCancel>($data)?;
        $check::<DOActual>($data)?;
        $check::<DSConditions>($data)?;
        $check::<TSConditions>($data)?;
        $check::<TAConditions>($data)?;
        $check::<SupportedFeatures>($data)?;
        $check::<DSTransferSetBlockData>($data)?;
        $check::<MMSObjectName>($data)?;
        $check::<DSTransferSet>($data)?;
        $check::<TSTransferSet>($data)?;
        $check::<TATransferSet>($data)?;
        $check::<IMTransferSet>($data)?;
    }};
}

proptest! {
    #![proptest_config(config())]

    #[test]
    fn random_data_is_read_by_every_type_without_a_panic(data in values::<Data>(3)) {
        for_every_type!(reads, &data);
    }

    #[test]
    fn every_value_that_is_read_is_read_back_after_writing(data in values::<Data>(3)) {
        for_every_type!(writes_back, &data);
    }

    #[test]
    fn an_integer_is_accepted_exactly_within_its_range(value in any::<i64>()) {
        let data = Data::Integer(i128::from(value));
        prop_assert_eq!(
            Integer16::from_mms(&data).is_ok(),
            (i64::from(i16::MIN)..=i64::from(i16::MAX)).contains(&value)
        );
        prop_assert_eq!(
            Integer32::from_mms(&data).is_ok(),
            (i64::from(i32::MIN)..=i64::from(i32::MAX)).contains(&value)
        );
    }
}

/// A typed value of any type of the model, as MMS data: the data a peer would send.
fn valid_data() -> BoxedStrategy<Data> {
    prop_oneof![
        strat::ds_transfer_set().prop_map(|value| value.to_mms()),
        strat::single_protection_event().prop_map(|value| value.to_mms()),
        strat::packed_protection_event().prop_map(|value| value.to_mms()),
        strat::control_config().prop_map(|value| value.to_mms()),
        strat::sbo().prop_map(|value| value.to_mms()),
        strat::tag_value().prop_map(|value| value.to_mms()),
        strat::object_name().prop_map(|value| value.to_mms()),
    ]
    .boxed()
}

/// The valid data with one of its components replaced by `replacement`, or without its last component: a structure of
/// the wrong shape, close to the valid one. Data that is not a structure is replaced as a whole.
fn perturbed(data: &Data, index: usize, replacement: Data) -> Data {
    match data {
        Data::Structure(items) if !items.is_empty() => {
            let mut items = items.clone();
            if index % 3 == 2 {
                items.pop();
            } else {
                let slot = index % items.len();
                items[slot] = replacement;
            }
            Data::Structure(items)
        }
        _ => replacement,
    }
}

proptest! {
    #![proptest_config(config())]

    #[test]
    fn data_close_to_a_valid_value_is_read_by_every_type_without_a_panic(
        valid in valid_data(),
        index in 0usize..16,
        replacement in values::<Data>(2),
    ) {
        let data = perturbed(&valid, index, replacement);
        for_every_type!(reads, &data);
    }

    #[test]
    fn the_valid_data_of_a_value_is_read_back_as_the_value(valid in valid_data()) {
        for_every_type!(writes_back, &valid);
    }
}

macro_rules! round_trip_property {
    ($name:ident, $strategy:expr) => {
        proptest! {
            #![proptest_config(config())]

            #[test]
            fn $name(value in $strategy) {
                round_trips(&value)?;
            }
        }
    };
}

round_trip_property!(integer16_round_trips, strat::integer16());
round_trip_property!(integer32_round_trips, strat::integer32());
round_trip_property!(unsigned8_round_trips, strat::unsigned8());
round_trip_property!(mms_identifier_round_trips, strat::mms_identifier());
round_trip_property!(text_string_round_trips, strat::text_string());
round_trip_property!(time_stamp_round_trips, strat::time_stamp_extended());
round_trip_property!(object_name_round_trips, strat::object_name());
round_trip_property!(data_flags_round_trip, strat::data_flags());
round_trip_property!(ds_conditions_round_trip, strat::ds_conditions());
round_trip_property!(single_flags_round_trip, strat::single_flags());
round_trip_property!(event_flags_round_trip, strat::event_flags());
round_trip_property!(packed_flags_round_trip, strat::packed_flags());
round_trip_property!(tag_flags_round_trip, strat::tag_flags());
round_trip_property!(select_state_round_trips, strat::select_state());
round_trip_property!(info_mess_header_round_trips, strat::info_mess_header());
round_trip_property!(info_buf_xx_round_trips, strat::info_buf_xx());
round_trip_property!(tag_value_round_trips, strat::tag_value());
round_trip_property!(sbo_round_trips, strat::sbo());
round_trip_property!(control_config_round_trips, strat::control_config());
round_trip_property!(ds_transfer_set_round_trips, strat::ds_transfer_set());
round_trip_property!(
    single_protection_event_round_trips,
    strat::single_protection_event()
);
round_trip_property!(
    packed_protection_event_round_trips,
    strat::packed_protection_event()
);
