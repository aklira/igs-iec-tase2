// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The control points and the device outages (task M3): the select-before-operate types, the tag flags, the
//! control configuration whose ranges come from the data, and the outage messages. Each type round-trips
//! through MMS, and a malformed value is refused.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use igs_iec_tase2_model::builtin::Integer32;
use igs_iec_tase2_model::generated::iso802::{
    clause_6_3::{DataReal, Number, ReferenceNum, UtilityId},
    ActionId, CommentString, ControlCommand, ControlConfig, ControlConfigControlPointType,
    ControlConfigDeviceClass, ControlConfigSetPointType, ControlConfigTagClass,
    ControlSetpointDiscrete, ControlSetpointReal, DOActual, DOCancel, DONewRevSched, DeviceId,
    DeviceName, OutageAmountTypeId, OutagePeriodId, OutageTypeId, PlanTypeId, SBOCheckBackName,
    SelectState, StationNameString, TagFlags, TagValue, TextString, SBO,
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

#[test]
fn the_select_state_is_a_boolean() {
    round_trip(&SelectState(true));
    round_trip(&SelectState(false));
    assert_eq!(SelectState(true).to_mms(), Data::Boolean(true));
    assert!(matches!(
        SelectState::from_mms(&Data::Integer(1)),
        Err(ValueError::Kind { .. })
    ));
}

#[test]
fn the_control_commands_round_trip_at_their_limits() {
    round_trip(&ControlCommand::new(ControlCommand::MIN).unwrap());
    round_trip(&ControlCommand::new(ControlCommand::MAX).unwrap());
    round_trip(&ControlSetpointDiscrete::new(ControlSetpointDiscrete::MIN).unwrap());
    round_trip(&ControlSetpointDiscrete::new(ControlSetpointDiscrete::MAX).unwrap());
    round_trip(&SBOCheckBackName::new(SBOCheckBackName::MIN).unwrap());
    round_trip(&SBOCheckBackName::new(SBOCheckBackName::MAX).unwrap());
    assert!(ControlCommand::new(ControlCommand::MAX + 1).is_err());
}

#[test]
fn a_control_setpoint_real_round_trips_as_a_single_precision_real() {
    round_trip(&ControlSetpointReal(2.5));
    round_trip(&ControlSetpointReal(-0.125));
}

#[test]
fn the_tag_flags_name_their_bits_and_round_trip() {
    let flags = TagFlags(0b101);
    assert!(flags.is_tag_hi());
    assert!(!flags.is_tag_lo());
    assert!(flags.is_tag_state());
    round_trip(&flags);
    // The first and the third bit of the three-bit string: 1 0 1 with five unused bits.
    let data = Data::BitString(BitString {
        unused: 5,
        bytes: vec![0b1010_0000],
    });
    assert_eq!(TagFlags::from_mms(&data).unwrap(), flags);
}

#[test]
fn a_tag_value_round_trips_with_its_reason() {
    round_trip(&TagValue {
        flags: TagFlags(0b011),
        reason: TextString::new("local work").unwrap(),
    });
}

#[test]
fn a_select_before_operate_round_trips_with_its_time_out() {
    round_trip(&SBO {
        time_out: gmt(60),
        select: SelectState(true),
    });
}

#[test]
fn a_control_configuration_round_trips_within_its_ranges() {
    round_trip(&ControlConfig {
        control_point_type: ControlConfigControlPointType::new(2).unwrap(),
        set_point_type: ControlConfigSetPointType::new(1).unwrap(),
        device_class: ControlConfigDeviceClass::new(1).unwrap(),
        tag_class: ControlConfigTagClass::new(1).unwrap(),
    });
}

#[test]
fn a_control_point_type_outside_its_range_is_refused_by_the_type_and_by_mms() {
    assert!(ControlConfigControlPointType::new(3).is_err());
    let data = Data::Structure(vec![
        Data::Integer(3),
        Data::Integer(1),
        Data::Integer(1),
        Data::Integer(1),
    ]);
    assert!(matches!(
        ControlConfig::from_mms(&data),
        Err(ValueError::Range(_))
    ));
}

#[test]
fn a_structure_with_the_wrong_number_of_components_is_refused() {
    let data = Data::Structure(vec![Data::Boolean(true)]);
    assert!(matches!(SBO::from_mms(&data), Err(ValueError::Format(_))));
}

fn device() -> (DeviceName, DeviceId, Number, DataReal, Integer32) {
    (
        DeviceName::new("breaker 12").unwrap(),
        DeviceId::new(4).unwrap(),
        Number::new(7).unwrap(),
        DataReal(110.0),
        gmt(1_700_000_000),
    )
}

#[test]
fn a_device_outage_plan_round_trips_with_its_periods_and_limits() {
    let (device_name, device_type, device_number, device_rating, activity) = device();
    round_trip(&DONewRevSched {
        outage_reference_id: ReferenceNum::new(12).unwrap(),
        owning_utility_id: UtilityId::new(3).unwrap(),
        time_stamp: gmt(1_700_000_100),
        station_name: StationNameString::new("substation A").unwrap(),
        device_name: device_name.clone(),
        device_type,
        device_number,
        device_rating,
        activity_date_and_time: activity,
        plan_type: PlanTypeId::new(1).unwrap(),
        planned_open_or_out_of_service_date_and_time: gmt(1_700_010_000),
        planned_close_or_in_service_date_and_time: gmt(1_700_020_000),
        outage_period: OutagePeriodId::new(2).unwrap(),
        outage_type: OutageTypeId::new(1).unwrap(),
        outage_type_amount: OutageAmountTypeId::new(1).unwrap(),
        amount: DataReal(25.5),
        upper_operating_limit: DataReal(120.0),
        lower_operating_limit: DataReal(80.0),
        class: igs_iec_tase2_model::generated::iso802::ClassId::new(1).unwrap(),
        comments: CommentString::new("planned maintenance").unwrap(),
        outage_effect: CommentString::new("line de-energised").unwrap(),
    });
}

#[test]
fn a_device_outage_cancellation_round_trips() {
    let (device_name, device_type, device_number, device_rating, activity) = device();
    round_trip(&DOCancel {
        outage_reference_id: ReferenceNum::new(12).unwrap(),
        owning_utility_id: UtilityId::new(3).unwrap(),
        time_stamp: gmt(1_700_000_200),
        station_name: StationNameString::new("substation A").unwrap(),
        device_name,
        device_type,
        device_number,
        device_rating,
        activity_date_and_time: activity,
        comments: CommentString::new("postponed").unwrap(),
        outage_effect: CommentString::new("none").unwrap(),
    });
}

#[test]
fn a_device_outage_actual_round_trips_with_its_action_and_amount() {
    let (device_name, device_type, device_number, device_rating, activity) = device();
    round_trip(&DOActual {
        outage_reference_id: ReferenceNum::new(12).unwrap(),
        owning_utility_id: UtilityId::new(3).unwrap(),
        time_stamp: gmt(1_700_000_300),
        station_name: StationNameString::new("substation A").unwrap(),
        device_name,
        device_type,
        device_number,
        device_rating,
        activity_date_and_time: activity,
        action: ActionId::new(2).unwrap(),
        affected_amount: DataReal(3.25),
        comments: CommentString::new("opened").unwrap(),
        outage_effect: CommentString::new("load shed").unwrap(),
    });
}
