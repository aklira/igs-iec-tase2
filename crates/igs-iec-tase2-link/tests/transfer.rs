// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The Data Set Transfer Set (task B4), with a fake clock: one deterministic test per condition and per expiry, the
//! report contents (RBE, buffering, blocking), the critical acknowledgement, the TLE, and the refusals at the start.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::collections::BTreeMap;
use std::time::Duration;

use igs_iec_tase2_link::bilateral::{ObjectName, Scope};
use igs_iec_tase2_link::data::ErrorCode;
use igs_iec_tase2_link::transfer::{
    Alert, Block, DataSetTransferSet, Parameters, Payload, Policy, Reason, Report, UnexpectedAck,
};
use igs_iec_tase2_model::builtin::Integer16;
use igs_iec_tase2_model::generated::iso503::DSConditions;
use igs_iso9506_mms_codec::generated::types::Data;

fn secs(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

fn name(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

fn members() -> Vec<ObjectName> {
    vec![name("Flow1"), name("ReadOnly")]
}

fn snapshot() -> BTreeMap<ObjectName, Data> {
    BTreeMap::from([
        (name("Flow1"), Data::Integer(5)),
        (name("ReadOnly"), Data::Integer(1)),
    ])
}

fn parameters(conditions: u64) -> Parameters {
    Parameters {
        start_time: Duration::ZERO,
        interval: secs(10),
        tle: Duration::ZERO,
        buffer_time: Duration::ZERO,
        integrity_check: Duration::ZERO,
        conditions: DSConditions(conditions),
        block_data: false,
        critical: false,
        rbe: false,
        all_changes_reported: false,
        event_code_requested: Integer16::new(7).unwrap(),
    }
}

fn policy() -> Policy {
    Policy {
        expected_send_delay: secs(1),
        ack_timeout: secs(30),
    }
}

/// A transfer set started at `now`, with the parameters given.
fn start(parameters: Parameters, now: Duration) -> DataSetTransferSet {
    DataSetTransferSet::start(parameters, policy(), members(), snapshot(), now).unwrap()
}

fn named(report: &Report) -> Vec<(ObjectName, Data)> {
    match &report.payload {
        Payload::Named(values) => values.clone(),
        other => panic!("named variables are expected: {other:?}"),
    }
}

const INTERVAL: u64 = DSConditions::INTERVAL_TIME_OUT;
const INTEGRITY: u64 = DSConditions::INTEGRITY_TIME_OUT;
const OBJECT_CHANGE: u64 = DSConditions::OBJECT_CHANGE;
const OPERATOR: u64 = DSConditions::OPERATOR_REQUEST;
const OTHER: u64 = DSConditions::OTHER_EXTERNAL_EVENT;

#[test]
fn the_interval_expiry_reports_the_entire_data_set_at_each_interval() {
    let mut set = start(parameters(INTERVAL), Duration::ZERO);
    let first = set.advance(secs(10));
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].reason, Reason::Interval);
    assert_eq!(first[0].at, secs(10));
    assert_eq!(first[0].sequence, 1);
    assert_eq!(named(&first[0]).len(), 2);
    assert!(
        set.advance(secs(15)).is_empty(),
        "no interval expires before 20 s"
    );
    let second = set.advance(secs(20));
    assert_eq!(second.len(), 1);
    assert_eq!(second[0].sequence, 2);
}

#[test]
fn a_late_clock_reports_every_interval_that_expired_in_time_order() {
    let mut set = start(parameters(INTERVAL), Duration::ZERO);
    let reports = set.advance(secs(35));
    assert_eq!(
        reports.iter().map(|report| report.at).collect::<Vec<_>>(),
        vec![secs(10), secs(20), secs(30)]
    );
}

#[test]
fn a_start_time_in_the_future_begins_the_interval_at_that_time() {
    let mut parameters = parameters(INTERVAL);
    parameters.start_time = secs(100);
    let mut set = start(parameters, secs(0));
    assert!(
        set.advance(secs(50)).is_empty(),
        "nothing before the start time"
    );
    let reports = set.advance(secs(110));
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].at, secs(110));
}

#[test]
fn a_zero_start_time_begins_now() {
    let mut set = start(parameters(INTERVAL), secs(1_000));
    assert!(set.advance(secs(1_005)).is_empty());
    assert_eq!(set.advance(secs(1_010)).len(), 1);
}

#[test]
fn a_stop_transfer_ends_the_monitoring() {
    let mut set = start(parameters(INTERVAL), Duration::ZERO);
    set.stop();
    assert!(!set.is_enabled());
    assert!(set.advance(secs(100)).is_empty());
    assert_eq!(set.next_deadline(), None);
}

#[test]
fn with_rbe_the_interval_reports_only_the_members_that_changed() {
    let mut parameters = parameters(INTERVAL);
    parameters.rbe = true;
    let mut set = start(parameters, Duration::ZERO);
    assert!(set
        .change(&name("ReadOnly"), Data::Integer(2), secs(3))
        .is_empty());
    let reports = set.advance(secs(10));
    assert_eq!(reports.len(), 1);
    assert_eq!(
        named(&reports[0]),
        vec![(name("ReadOnly"), Data::Integer(2))]
    );
    // No change since the report: the next interval has nothing to report.
    assert!(set.advance(secs(20)).is_empty());
}

#[test]
fn an_object_change_without_a_buffer_is_reported_at_once() {
    let mut set = start(parameters(OBJECT_CHANGE), Duration::ZERO);
    let reports = set.change(&name("Flow1"), Data::Integer(7), secs(2));
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].reason, Reason::ObjectChange);
    assert_eq!(reports[0].at, secs(2));
    // Without RBE the report is the entire data set, with the new value.
    assert_eq!(named(&reports[0]).len(), 2);
    assert!(named(&reports[0]).contains(&(name("Flow1"), Data::Integer(7))));
}

#[test]
fn with_rbe_an_object_change_reports_only_the_changed_member() {
    let mut parameters = parameters(OBJECT_CHANGE);
    parameters.rbe = true;
    let mut set = start(parameters, Duration::ZERO);
    let reports = set.change(&name("Flow1"), Data::Integer(7), secs(2));
    assert_eq!(named(&reports[0]), vec![(name("Flow1"), Data::Integer(7))]);
}

#[test]
fn a_change_before_the_start_time_is_not_monitored_but_updates_the_values() {
    let mut parameters = parameters(OBJECT_CHANGE);
    parameters.start_time = secs(100);
    let mut set = start(parameters, Duration::ZERO);
    assert!(set
        .change(&name("Flow1"), Data::Integer(7), secs(50))
        .is_empty());
    let reports = set.operator_request(secs(60));
    assert!(
        reports.is_empty(),
        "the operator request needs the OperatorRequest condition"
    );
}

#[test]
fn a_buffered_change_is_reported_at_the_end_of_the_buffer_with_the_last_value() {
    let mut parameters = parameters(OBJECT_CHANGE);
    parameters.rbe = true;
    parameters.buffer_time = secs(5);
    let mut set = start(parameters, Duration::ZERO);
    assert!(set
        .change(&name("Flow1"), Data::Integer(7), secs(1))
        .is_empty());
    assert!(set
        .change(&name("Flow1"), Data::Integer(8), secs(2))
        .is_empty());
    assert!(
        set.advance(secs(5)).is_empty(),
        "the buffer ends 5 s after the first change"
    );
    let reports = set.advance(secs(6));
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].at, secs(6));
    assert_eq!(named(&reports[0]), vec![(name("Flow1"), Data::Integer(8))]);
}

#[test]
fn with_all_changes_reported_every_value_of_the_buffer_is_reported_in_order() {
    let mut parameters = parameters(OBJECT_CHANGE);
    parameters.rbe = true;
    parameters.buffer_time = secs(5);
    parameters.all_changes_reported = true;
    let mut set = start(parameters, Duration::ZERO);
    set.change(&name("Flow1"), Data::Integer(7), secs(1));
    set.change(&name("Flow1"), Data::Integer(8), secs(2));
    let reports = set.advance(secs(6));
    assert_eq!(
        named(&reports[0]),
        vec![
            (name("Flow1"), Data::Integer(7)),
            (name("Flow1"), Data::Integer(8))
        ]
    );
}

#[test]
fn an_integrity_expiry_reports_the_entire_data_set_with_rbe() {
    let mut parameters = parameters(INTEGRITY);
    parameters.rbe = true;
    parameters.integrity_check = secs(30);
    let mut set = start(parameters, Duration::ZERO);
    let reports = set.advance(secs(30));
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].reason, Reason::Integrity);
    assert_eq!(named(&reports[0]).len(), 2);
}

#[test]
fn the_integrity_check_applies_only_with_rbe() {
    let mut parameters = parameters(INTEGRITY);
    parameters.integrity_check = secs(30);
    let set = start(parameters, Duration::ZERO);
    assert_eq!(set.next_deadline(), None);
}

#[test]
fn an_operator_request_reports_the_entire_data_set_when_requested() {
    let mut set = start(parameters(OPERATOR), Duration::ZERO);
    let reports = set.operator_request(secs(4));
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].reason, Reason::OperatorRequest);
    assert_eq!(named(&reports[0]).len(), 2);
}

#[test]
fn an_operator_request_is_ignored_without_the_condition() {
    let mut set = start(parameters(INTERVAL), Duration::ZERO);
    assert!(set.operator_request(secs(4)).is_empty());
}

#[test]
fn another_external_event_reports_only_the_requested_code() {
    let mut set = start(parameters(OTHER), Duration::ZERO);
    assert!(
        set.other_event(6, secs(4)).is_empty(),
        "another code is not the requested one"
    );
    let reports = set.other_event(7, secs(5));
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].reason, Reason::OtherEvent(7));
}

#[test]
fn a_blocked_report_without_rbe_carries_the_values_in_rule_0() {
    let mut parameters = parameters(OPERATOR);
    parameters.block_data = true;
    let mut set = start(parameters, Duration::ZERO);
    let reports = set.operator_request(secs(4));
    assert_eq!(
        reports[0].payload,
        Payload::Blocked(Block::Rule0(vec![Data::Integer(5), Data::Integer(1)]))
    );
}

#[test]
fn a_blocked_report_with_rbe_carries_the_positions_of_the_changed_members_in_rule_1() {
    let mut parameters = parameters(OBJECT_CHANGE);
    parameters.block_data = true;
    parameters.rbe = true;
    let mut set = start(parameters, Duration::ZERO);
    let reports = set.change(&name("ReadOnly"), Data::Integer(2), secs(2));
    assert_eq!(
        reports[0].payload,
        Payload::Blocked(Block::Rule1(vec![(1, Data::Integer(2))]))
    );
}

#[test]
fn a_critical_report_is_acknowledged_by_its_sequence_number() {
    let mut parameters = parameters(OPERATOR);
    parameters.critical = true;
    let mut set = start(parameters, Duration::ZERO);
    let reports = set.operator_request(secs(4));
    assert!(reports[0].critical);
    assert_eq!(set.acknowledge(reports[0].sequence), Ok(()));
    assert!(set.advance(secs(100)).is_empty());
    assert!(set.take_alerts().is_empty());
}

#[test]
fn an_acknowledgement_that_is_not_awaited_is_refused() {
    let mut parameters = parameters(OPERATOR);
    parameters.critical = true;
    let mut set = start(parameters, Duration::ZERO);
    set.operator_request(secs(4));
    assert_eq!(set.acknowledge(9), Err(UnexpectedAck { sequence: 9 }));
}

#[test]
fn a_critical_report_not_acknowledged_in_time_is_a_data_failure() {
    let mut parameters = parameters(OPERATOR);
    parameters.critical = true;
    let mut set = start(parameters, Duration::ZERO);
    set.operator_request(secs(4));
    assert!(set.advance(secs(33)).is_empty());
    set.advance(secs(34));
    assert_eq!(set.take_alerts(), vec![Alert::DataFailure { sequence: 1 }]);
    assert!(set.take_alerts().is_empty(), "the alert is raised once");
}

#[test]
fn a_negative_acknowledgement_is_a_data_failure() {
    let mut parameters = parameters(OPERATOR);
    parameters.critical = true;
    let mut set = start(parameters, Duration::ZERO);
    let reports = set.operator_request(secs(4));
    set.negative_acknowledge(reports[0].sequence);
    assert_eq!(set.take_alerts(), vec![Alert::DataFailure { sequence: 1 }]);
}

#[test]
fn a_report_that_would_take_longer_than_its_tle_is_discarded() {
    let mut parameters = parameters(OPERATOR);
    parameters.tle = secs(2);
    let mut set = DataSetTransferSet::start(
        parameters,
        Policy {
            expected_send_delay: secs(5),
            ack_timeout: secs(30),
        },
        members(),
        snapshot(),
        Duration::ZERO,
    )
    .unwrap();
    assert!(set.operator_request(secs(4)).is_empty());
    assert_eq!(set.discarded(), 1);
}

#[test]
fn a_report_within_its_tle_is_sent() {
    let mut parameters = parameters(OPERATOR);
    parameters.tle = secs(10);
    let mut set = start(parameters, Duration::ZERO);
    assert_eq!(set.operator_request(secs(4)).len(), 1);
    assert_eq!(set.discarded(), 0);
}

#[test]
fn a_zero_interval_is_refused_at_the_start_of_the_interval_condition() {
    let mut parameters = parameters(INTERVAL);
    parameters.interval = Duration::ZERO;
    let refused =
        DataSetTransferSet::start(parameters, policy(), members(), snapshot(), Duration::ZERO);
    assert_eq!(refused.err(), Some(ErrorCode::ObjectAttributeInconsistent));
}

#[test]
fn a_zero_integrity_check_is_refused_with_rbe() {
    let mut parameters = parameters(INTEGRITY);
    parameters.rbe = true;
    parameters.integrity_check = Duration::ZERO;
    let refused =
        DataSetTransferSet::start(parameters, policy(), members(), snapshot(), Duration::ZERO);
    assert_eq!(refused.err(), Some(ErrorCode::ObjectAttributeInconsistent));
}
