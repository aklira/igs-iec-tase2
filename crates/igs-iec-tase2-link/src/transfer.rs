// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The Data Set Transfer Set (IEC 60870-6-503 §5.3.10.2, §7.2.5), sans I/O and with a clock that the caller gives: the
//! monitoring of the conditions of §7.2.5.4 and the Transfer Report of §7.2.5.5.3.
//!
//! Every input takes the time `now` (the time since an epoch chosen by the caller, the GMT time of the standard in
//! seconds). The inputs first run the timers that are due before the input, so that the outputs come in time order.
//! Nothing here reads a clock or sends a message.

use std::collections::BTreeMap;
use std::time::Duration;

use igs_iec_tase2_model::builtin::Integer16;
use igs_iec_tase2_model::generated::iso503::DSConditions;
use igs_iso9506_mms_codec::generated::types::Data;

use crate::bilateral::ObjectName;
use crate::data::ErrorCode;

/// The parameters of the transfer that the client asks for: the DSTransmissionPars of 503 §5.3.10.2.2 and the event code
/// of §5.3.10.2.1.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    /// The time at which the monitoring begins. Zero means now.
    pub start_time: Duration,
    /// The interval between reports (the IntervalTimeOut condition). Zero when the condition is not requested.
    pub interval: Duration,
    /// The time limit for execution: a report that the server expects to take longer is discarded. Zero is ignored.
    pub tle: Duration,
    /// The time that buffers the ObjectChange condition before the report. Zero reports each change.
    pub buffer_time: Duration,
    /// The interval of the integrity check (the IntegrityTimeOut condition), which applies with RBE only.
    pub integrity_check: Duration,
    /// The conditions to monitor (the DSConditions Requested attribute).
    pub conditions: DSConditions,
    /// Whether the report is blocked: one octet string with its header, instead of the named variables.
    pub block_data: bool,
    /// Whether the report is critical: the client acknowledges it.
    pub critical: bool,
    /// Report by exception: only the members that changed are reported (the entire set otherwise).
    pub rbe: bool,
    /// Whether every change in the buffer time is reported, not only the last one.
    pub all_changes_reported: bool,
    /// The event code that triggers the OtherExternalEvent condition, when it is requested.
    pub event_code_requested: Integer16,
}

/// The server's own agreements, which are not in the parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Policy {
    /// The time the server expects to take to send a report: a report that would take longer than the TLE is discarded.
    pub expected_send_delay: Duration,
    /// The time the server waits for the acknowledgement of a critical report before it signals a data failure.
    pub ack_timeout: Duration,
}

/// The reason of a report: the condition that triggered it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason {
    /// The IntervalTimeOut condition.
    Interval,
    /// The IntegrityTimeOut condition.
    Integrity,
    /// The ObjectChange condition.
    ObjectChange,
    /// The OperatorRequest condition.
    OperatorRequest,
    /// The OtherExternalEvent condition, with the event code.
    OtherEvent(i64),
}

/// The rule of the blocked report (503 §7.2.5.5.3): the values appended in order, or with their positions.
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// Rule 0: no RBE; the values of all the members, in order.
    Rule0(Vec<Data>),
    /// Rule 1: RBE; each value with its position in the member list, in one octet.
    Rule1(Vec<(u8, Data)>),
    /// Rule 2: RBE; each value with its position in the member list, in two octets.
    Rule2(Vec<(u16, Data)>),
}

/// The contents of a report.
#[derive(Debug, Clone, PartialEq)]
pub enum Payload {
    /// The named variables: each member with its value.
    Named(Vec<(ObjectName, Data)>),
    /// The blocked octet string.
    Blocked(Block),
}

/// A Transfer Report, as the server sends it in an MMS Information Report.
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    /// The number of the report on this transfer set, from 1.
    pub sequence: u32,
    /// The time of the report.
    pub at: Duration,
    /// The condition that triggered it.
    pub reason: Reason,
    /// The contents.
    pub payload: Payload,
    /// Whether the client must acknowledge the report.
    pub critical: bool,
}

/// What the server signals, besides the reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alert {
    /// The Data_failure event: a critical report was not acknowledged, or the client sent a negative acknowledgement.
    DataFailure {
        /// The report concerned.
        sequence: u32,
    },
}

/// The acknowledgement of a report that is not the one awaited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnexpectedAck {
    /// The sequence number that was received.
    pub sequence: u32,
}

/// The status of the transfer set (503 §5.3.10.2.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Enabled,
    Disabled,
}

/// The timers of the monitoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Timer {
    Interval,
    Integrity,
    Buffer,
}

/// A Data Set Transfer Set in monitoring.
#[derive(Debug, Clone)]
pub struct DataSetTransferSet {
    parameters: Parameters,
    policy: Policy,
    members: Vec<ObjectName>,
    snapshot: BTreeMap<ObjectName, Data>,
    status: Status,
    anchor: Option<Duration>,
    next_interval: Option<Duration>,
    next_integrity: Option<Duration>,
    buffer_deadline: Option<Duration>,
    changed_since_report: Vec<ObjectName>,
    pending_changes: Vec<(ObjectName, Data)>,
    sequence: u32,
    awaiting_ack: Option<(u32, Duration)>,
    alerts: Vec<Alert>,
    discarded: u32,
}

impl DataSetTransferSet {
    /// The Start Transfer of the transfer set: the parameters are checked, and the monitoring starts at the start time (or
    /// now, when the start time is zero or past). `members` is the data set, `snapshot` the values of its members now.
    pub fn start(
        parameters: Parameters,
        policy: Policy,
        members: Vec<ObjectName>,
        snapshot: BTreeMap<ObjectName, Data>,
        now: Duration,
    ) -> Result<Self, ErrorCode> {
        // The reporting interval must be a time for the condition that uses it (503 §7.2.5.2.3).
        if has(parameters.conditions, DSConditions::INTERVAL_TIME_OUT)
            && parameters.interval.is_zero()
        {
            return Err(ErrorCode::ObjectAttributeInconsistent);
        }
        if has(parameters.conditions, DSConditions::INTEGRITY_TIME_OUT)
            && parameters.rbe
            && parameters.integrity_check.is_zero()
        {
            return Err(ErrorCode::ObjectAttributeInconsistent);
        }
        // The interval starts when the Start Time occurs; a zero Start Time is now (503 §5.3.10.2.2).
        let anchor = if parameters.start_time.is_zero() {
            now
        } else {
            parameters.start_time
        };
        let mut set = Self {
            parameters,
            policy,
            members,
            snapshot,
            status: Status::Enabled,
            anchor: Some(anchor),
            next_interval: None,
            next_integrity: None,
            buffer_deadline: None,
            changed_since_report: Vec::new(),
            pending_changes: Vec::new(),
            sequence: 0,
            awaiting_ack: None,
            alerts: Vec::new(),
            discarded: 0,
        };
        set.schedule_periodic();
        Ok(set)
    }

    /// Whether the transfer set is enabled (its status is ENABLED).
    pub fn is_enabled(&self) -> bool {
        self.status == Status::Enabled
    }

    /// The reports discarded because they would take longer than the TLE.
    pub fn discarded(&self) -> u32 {
        self.discarded
    }

    /// Takes the alerts raised since the last call.
    pub fn take_alerts(&mut self) -> Vec<Alert> {
        std::mem::take(&mut self.alerts)
    }

    /// The Stop Transfer: the monitoring ends; no further report is sent, and the buffered changes are dropped.
    pub fn stop(&mut self) {
        self.status = Status::Disabled;
        self.next_interval = None;
        self.next_integrity = None;
        self.buffer_deadline = None;
        self.pending_changes.clear();
        self.changed_since_report.clear();
    }

    /// The time of the next timer, or `None` when no timer runs: the caller calls `advance` at that time.
    pub fn next_deadline(&self) -> Option<Duration> {
        if self.status != Status::Enabled {
            return None;
        }
        [
            self.next_interval,
            self.next_integrity,
            self.buffer_deadline,
        ]
        .into_iter()
        .flatten()
        .min()
    }

    /// Runs the timers due at `now` and returns the reports they produce, in time order. An acknowledgement that is
    /// overdue is signalled as a data failure.
    pub fn advance(&mut self, now: Duration) -> Vec<Report> {
        let mut out = Vec::new();
        self.check_ack_timeout(now);
        if self.status != Status::Enabled {
            return out;
        }
        while let Some((at, timer)) = self.earliest_due(now) {
            match timer {
                Timer::Interval => {
                    self.next_interval = Some(at.saturating_add(self.parameters.interval));
                    let payload = if self.parameters.rbe {
                        self.changed_payload()
                    } else {
                        self.full_payload()
                    };
                    self.emit_if_any(Reason::Interval, payload, at, &mut out);
                }
                Timer::Integrity => {
                    self.next_integrity = Some(at.saturating_add(self.parameters.integrity_check));
                    let payload = self.full_payload();
                    self.emit_if_any(Reason::Integrity, payload, at, &mut out);
                }
                Timer::Buffer => {
                    self.buffer_deadline = None;
                    let payload = self.buffered_payload();
                    self.emit_if_any(Reason::ObjectChange, payload, at, &mut out);
                }
            }
        }
        out
    }

    /// A change of a member's value at `now`. The change is applied to the snapshot; it is monitored when the
    /// ObjectChange condition is requested.
    pub fn change(&mut self, name: &ObjectName, value: Data, now: Duration) -> Vec<Report> {
        let mut out = self.advance(now);
        if self.status != Status::Enabled || !self.members.contains(name) {
            self.snapshot.insert(name.clone(), value);
            return out;
        }
        self.snapshot.insert(name.clone(), value.clone());
        // Monitoring begins at the start time: a change before it is not reported, nor kept for the first report.
        if !self.is_started(now) {
            return out;
        }
        // The changed members are kept for the RBE interval report, whatever the conditions.
        if !self.changed_since_report.contains(name) {
            self.changed_since_report.push(name.clone());
        }
        if !has(self.parameters.conditions, DSConditions::OBJECT_CHANGE) {
            return out;
        }
        if self.parameters.buffer_time.is_zero() {
            let payload = if self.parameters.rbe {
                Payload::Named(vec![(name.clone(), value)])
            } else {
                self.full_payload()
            };
            self.emit(Reason::ObjectChange, payload, now, &mut out);
        } else {
            self.pending_changes.push((name.clone(), value));
            if self.buffer_deadline.is_none() {
                self.buffer_deadline = Some(now.saturating_add(self.parameters.buffer_time));
            }
        }
        out
    }

    /// The OperatorRequest condition at `now`: a report of the entire data set.
    pub fn operator_request(&mut self, now: Duration) -> Vec<Report> {
        let mut out = self.advance(now);
        if self.status == Status::Enabled
            && has(self.parameters.conditions, DSConditions::OPERATOR_REQUEST)
        {
            let payload = self.full_payload();
            self.emit(Reason::OperatorRequest, payload, now, &mut out);
        }
        out
    }

    /// The OtherExternalEvent condition with `code` at `now`: a report of the entire data set, when the code is the one
    /// requested.
    pub fn other_event(&mut self, code: i64, now: Duration) -> Vec<Report> {
        let mut out = self.advance(now);
        let requested = self.parameters.event_code_requested.get();
        if self.status == Status::Enabled
            && has(
                self.parameters.conditions,
                DSConditions::OTHER_EXTERNAL_EVENT,
            )
            && code == requested
        {
            let payload = self.full_payload();
            self.emit(Reason::OtherEvent(code), payload, now, &mut out);
        }
        out
    }

    /// The client's acknowledgement of a critical report: the one awaited ends the wait.
    pub fn acknowledge(&mut self, sequence: u32) -> Result<(), UnexpectedAck> {
        match self.awaiting_ack {
            Some((awaited, _)) if awaited == sequence => {
                self.awaiting_ack = None;
                Ok(())
            }
            _ => Err(UnexpectedAck { sequence }),
        }
    }

    /// The client's negative acknowledgement (Transfer_Report_NACK) of a report: a data failure.
    pub fn negative_acknowledge(&mut self, sequence: u32) {
        if self.awaiting_ack.map(|(awaited, _)| awaited) == Some(sequence) {
            self.awaiting_ack = None;
        }
        self.alerts.push(Alert::DataFailure { sequence });
    }

    fn is_started(&self, now: Duration) -> bool {
        self.anchor.is_some_and(|anchor| anchor <= now)
    }

    fn schedule_periodic(&mut self) {
        let Some(anchor) = self.anchor else {
            return;
        };
        if has(self.parameters.conditions, DSConditions::INTERVAL_TIME_OUT) {
            self.next_interval = Some(anchor.saturating_add(self.parameters.interval));
        }
        if has(self.parameters.conditions, DSConditions::INTEGRITY_TIME_OUT) && self.parameters.rbe
        {
            self.next_integrity = Some(anchor.saturating_add(self.parameters.integrity_check));
        }
    }

    fn earliest_due(&self, now: Duration) -> Option<(Duration, Timer)> {
        [
            (self.next_interval, Timer::Interval),
            (self.next_integrity, Timer::Integrity),
            (self.buffer_deadline, Timer::Buffer),
        ]
        .into_iter()
        .filter_map(|(at, timer)| at.filter(|at| *at <= now).map(|at| (at, timer)))
        .min()
    }

    fn check_ack_timeout(&mut self, now: Duration) {
        if let Some((sequence, sent)) = self.awaiting_ack {
            if now >= sent.saturating_add(self.policy.ack_timeout) {
                self.awaiting_ack = None;
                self.alerts.push(Alert::DataFailure { sequence });
            }
        }
    }

    /// The members of the data set and their values now.
    fn full_payload(&self) -> Payload {
        let values = self
            .members
            .iter()
            .filter_map(|member| {
                self.snapshot
                    .get(member)
                    .map(|value| (member.clone(), value.clone()))
            })
            .collect();
        Payload::Named(values)
    }

    /// The members changed since the last report, with their values now.
    fn changed_payload(&self) -> Payload {
        let values = self
            .changed_since_report
            .iter()
            .filter_map(|member| {
                self.snapshot
                    .get(member)
                    .map(|value| (member.clone(), value.clone()))
            })
            .collect();
        Payload::Named(values)
    }

    /// The changes of the buffer: every change in order when All Changes Reported is set, else the last value of each
    /// member; with RBE the members only, otherwise the entire data set.
    fn buffered_payload(&self) -> Payload {
        if !self.parameters.rbe {
            return self.full_payload();
        }
        if self.parameters.all_changes_reported {
            return Payload::Named(self.pending_changes.clone());
        }
        let mut last: BTreeMap<ObjectName, Data> = BTreeMap::new();
        for (member, value) in &self.pending_changes {
            last.insert(member.clone(), value.clone());
        }
        Payload::Named(last.into_iter().collect())
    }

    fn emit_if_any(
        &mut self,
        reason: Reason,
        payload: Payload,
        at: Duration,
        out: &mut Vec<Report>,
    ) {
        let empty = matches!(&payload, Payload::Named(values) if values.is_empty());
        if !empty {
            self.emit(reason, payload, at, out);
        } else {
            // Nothing to report: the changes that were pending are none, and the timers stay as they are.
            self.changed_since_report.clear();
            self.pending_changes.clear();
        }
    }

    /// Makes a report, unless the TLE forbids it: the report and its data are then discarded.
    fn emit(&mut self, reason: Reason, payload: Payload, at: Duration, out: &mut Vec<Report>) {
        self.changed_since_report.clear();
        self.pending_changes.clear();
        let tle = self.parameters.tle;
        if !tle.is_zero() && self.policy.expected_send_delay > tle {
            self.discarded = self.discarded.saturating_add(1);
            return;
        }
        self.sequence = self.sequence.saturating_add(1);
        let payload = if self.parameters.block_data {
            Payload::Blocked(self.block(payload))
        } else {
            payload
        };
        if self.parameters.critical {
            self.awaiting_ack = Some((self.sequence, at));
        }
        out.push(Report {
            sequence: self.sequence,
            at,
            reason,
            payload,
            critical: self.parameters.critical,
        });
    }

    /// The blocked form of a payload: rule 0 without RBE, else the positions of the members in the data set (rule 1 when
    /// they fit in one octet, rule 2 otherwise).
    fn block(&self, payload: Payload) -> Block {
        let values = match payload {
            Payload::Named(values) => values,
            Payload::Blocked(block) => return block,
        };
        if !self.parameters.rbe {
            return Block::Rule0(values.into_iter().map(|(_, value)| value).collect());
        }
        let position = |name: &ObjectName| {
            self.members
                .iter()
                .position(|member| member == name)
                .unwrap_or(0)
        };
        let positions: Vec<(usize, Data)> = values
            .into_iter()
            .map(|(name, value)| (position(&name), value))
            .collect();
        if positions
            .iter()
            .all(|(index, _)| *index <= usize::from(u8::MAX))
        {
            Block::Rule1(
                positions
                    .into_iter()
                    .filter_map(|(index, value)| {
                        u8::try_from(index).ok().map(|index| (index, value))
                    })
                    .collect(),
            )
        } else {
            Block::Rule2(
                positions
                    .into_iter()
                    .filter_map(|(index, value)| {
                        u16::try_from(index).ok().map(|index| (index, value))
                    })
                    .collect(),
            )
        }
    }
}

fn has(conditions: DSConditions, flag: u64) -> bool {
    conditions.0 & flag != 0
}
