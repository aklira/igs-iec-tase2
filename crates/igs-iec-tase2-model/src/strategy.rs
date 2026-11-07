// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! Strategies for property tests (task M6): values of the TASE.2 types that are valid by construction, so that a
//! property can state what holds for every valid value. The strategies of the generated types of `iso802` and `iso503`
//! are written here, by hand, for the types of the blocks of 1.0; the random data of the codec is in
//! `igs_iso9506_mms_codec::strategy`.
//!
//! Enabled by the feature `strategy`, which is off by default: the library does not depend on the test framework.

use proptest::prelude::*;

use crate::builtin::{Integer16, Integer32, MmsIdentifier, Unsigned8};
use crate::generated::iso503::{
    DSConditions, DSTransferSet, DSTransferSetAllChangesReported, DSTransferSetBlockData,
    DSTransferSetCritical, DSTransferSetRBE, DSTransferSetStatus, MMSObjectName,
    MMSObjectNameScope, TimeStampExtended,
};
use crate::generated::iso802::{
    ControlConfig, ControlConfigControlPointType, ControlConfigDeviceClass,
    ControlConfigSetPointType, ControlConfigTagClass, EventFlags, PackedFlags,
    PackedProtectionEvent, SelectState, SingleFlags, SingleProtectionEvent, TagFlags, TagValue,
    TextString, SBO,
};

/// Integers of 16 bits, in their range.
pub fn integer16() -> BoxedStrategy<Integer16> {
    any::<i16>()
        .prop_filter_map("a 16-bit integer", |v| Integer16::new(i64::from(v)).ok())
        .boxed()
}

/// Integers of 32 bits, in their range.
pub fn integer32() -> BoxedStrategy<Integer32> {
    any::<i32>()
        .prop_filter_map("a 32-bit integer", |v| Integer32::new(i64::from(v)).ok())
        .boxed()
}

/// Unsigned integers of 8 bits, in their range.
pub fn unsigned8() -> BoxedStrategy<Unsigned8> {
    any::<u8>()
        .prop_filter_map("an unsigned 8-bit integer", |v| {
            Unsigned8::new(u64::from(v)).ok()
        })
        .boxed()
}

/// Identifiers of MMS: letters, digits and underscores, of 1 to 32 characters.
pub fn mms_identifier() -> BoxedStrategy<MmsIdentifier> {
    "[A-Za-z0-9_]{1,32}"
        .prop_filter_map("an identifier", |s| MmsIdentifier::new(s).ok())
        .boxed()
}

/// Visible strings of at most 255 characters: printable ASCII.
pub fn text_string() -> BoxedStrategy<TextString> {
    "[ -~]{0,255}"
        .prop_filter_map("a visible string", |s| TextString::new(s).ok())
        .boxed()
}

/// Object names of the domain-specific scope, or of the VMD-specific scope.
pub fn object_name() -> BoxedStrategy<MMSObjectName> {
    (any::<bool>(), mms_identifier(), mms_identifier())
        .prop_map(|(domain_specific, domain_name, name)| MMSObjectName {
            scope: MMSObjectNameScope(if domain_specific {
                MMSObjectNameScope::DOMAIN_SPECIFIC
            } else {
                MMSObjectNameScope::VMD_SPECIFIC
            }),
            domain_name,
            name,
        })
        .boxed()
}

/// Time stamps: any second of 32 bits, and a millisecond of 0 to 999.
pub fn time_stamp_extended() -> BoxedStrategy<TimeStampExtended> {
    (integer32(), 0i64..=999)
        .prop_filter_map("a time stamp", |(seconds, milliseconds)| {
            TimeStampExtended::new(seconds, Integer16::new(milliseconds).ok()?).ok()
        })
        .boxed()
}

/// A bit string of `count` bits: every value of the bits.
fn bits(count: u32) -> BoxedStrategy<u64> {
    (0u64..(1u64 << count)).boxed()
}

/// The flags of a data value: its eight bits.
pub fn data_flags() -> BoxedStrategy<crate::generated::iso802::DataFlags> {
    bits(8)
        .prop_map(crate::generated::iso802::DataFlags)
        .boxed()
}

/// The conditions of a data set transfer set: its five bits.
pub fn ds_conditions() -> BoxedStrategy<DSConditions> {
    bits(DSConditions::BIT_COUNT).prop_map(DSConditions).boxed()
}

/// The flags of a single protection event: its eight bits.
pub fn single_flags() -> BoxedStrategy<SingleFlags> {
    bits(SingleFlags::BIT_COUNT).prop_map(SingleFlags).boxed()
}

/// The flags of the start and trip of the phases: its eight bits.
pub fn event_flags() -> BoxedStrategy<EventFlags> {
    bits(EventFlags::BIT_COUNT).prop_map(EventFlags).boxed()
}

/// The flags of a packed protection event: its eight bits.
pub fn packed_flags() -> BoxedStrategy<PackedFlags> {
    bits(PackedFlags::BIT_COUNT).prop_map(PackedFlags).boxed()
}

/// The flags of a tag: its three bits.
pub fn tag_flags() -> BoxedStrategy<TagFlags> {
    bits(TagFlags::BIT_COUNT).prop_map(TagFlags).boxed()
}

/// A select state: true or false.
pub fn select_state() -> BoxedStrategy<SelectState> {
    any::<bool>().prop_map(SelectState).boxed()
}

/// A tag value: its flags and its reason.
pub fn tag_value() -> BoxedStrategy<TagValue> {
    (tag_flags(), text_string())
        .prop_map(|(flags, reason)| TagValue { flags, reason })
        .boxed()
}

/// A select before operate: its time out and its select state.
pub fn sbo() -> BoxedStrategy<SBO> {
    (integer32(), select_state())
        .prop_map(|(time_out, select)| SBO { time_out, select })
        .boxed()
}

/// A control configuration: each component within its range.
pub fn control_config() -> BoxedStrategy<ControlConfig> {
    (
        ControlConfigControlPointType::MIN..=ControlConfigControlPointType::MAX,
        ControlConfigSetPointType::MIN..=ControlConfigSetPointType::MAX,
        ControlConfigDeviceClass::MIN..=ControlConfigDeviceClass::MAX,
        ControlConfigTagClass::MIN..=ControlConfigTagClass::MAX,
    )
        .prop_filter_map(
            "a control configuration",
            |(point, set_point, device, tag)| {
                Some(ControlConfig {
                    control_point_type: ControlConfigControlPointType::new(point).ok()?,
                    set_point_type: ControlConfigSetPointType::new(set_point).ok()?,
                    device_class: ControlConfigDeviceClass::new(device).ok()?,
                    tag_class: ControlConfigTagClass::new(tag).ok()?,
                })
            },
        )
        .boxed()
}

/// A data set transfer set: each component of the standard, with its Boolean components.
pub fn ds_transfer_set() -> BoxedStrategy<DSTransferSet> {
    (
        object_name(),
        (
            integer32(),
            integer32(),
            integer32(),
            integer32(),
            integer32(),
        ),
        ds_conditions(),
        (
            any::<bool>(),
            any::<bool>(),
            any::<bool>(),
            any::<bool>(),
            any::<bool>(),
        ),
        integer16(),
    )
        .prop_map(
            |(
                data_set_name,
                (start, interval, tle, buffer, check),
                ds_conditions_requested,
                flags,
                event_code,
            )| {
                let (block, critical, rbe, all, status) = flags;
                DSTransferSet {
                    data_set_name,
                    start_time: start,
                    interval,
                    tle,
                    buffer_time: buffer,
                    integrity_check: check,
                    ds_conditions_requested,
                    block_data: DSTransferSetBlockData(block),
                    critical: DSTransferSetCritical(critical),
                    rbe: DSTransferSetRBE(rbe),
                    all_changes_reported: DSTransferSetAllChangesReported(all),
                    status: DSTransferSetStatus(if status {
                        DSTransferSetStatus::ENABLED
                    } else {
                        DSTransferSetStatus::DISABLED
                    }),
                    event_code_requested: event_code,
                }
            },
        )
        .boxed()
}

/// A single protection event: its flags, its operating time and its event time.
pub fn single_protection_event() -> BoxedStrategy<SingleProtectionEvent> {
    (single_flags(), integer16(), time_stamp_extended())
        .prop_map(
            |(single_event_flags, operating_time, event_time)| SingleProtectionEvent {
                single_event_flags,
                operating_time,
                event_time,
            },
        )
        .boxed()
}

/// A packed protection event: its phases, its flags, its operating time and its event time.
pub fn packed_protection_event() -> BoxedStrategy<PackedProtectionEvent> {
    (
        event_flags(),
        packed_flags(),
        integer16(),
        time_stamp_extended(),
    )
        .prop_map(
            |(packed_event, packed_event_flags, operating_time, event_time)| {
                PackedProtectionEvent {
                    packed_event,
                    packed_event_flags,
                    operating_time,
                    event_time,
                }
            },
        )
        .boxed()
}
