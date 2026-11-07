// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! Types of IEC 60870-6-802, generated from its data file by `cargo xtask codegen`.
//!
//! DO NOT EDIT. The clauses are cited on each type; the text of the standard is not in this file.

/// Every definition of this file: its name, clause, page, kind, components and bits.
pub const DEFINITIONS: &[crate::value::Definition] = &[
    crate::value::Definition {
        name: "Data_Real",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 24,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_State",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 24,
        kind: "bit-string",
        components: 0,
        bits: 8,
    },
    crate::value::Definition {
        name: "Data_Discrete",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 24,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_StateSupplemental",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 24,
        kind: "bit-string",
        components: 0,
        bits: 8,
    },
    crate::value::Definition {
        name: "Data_Flags",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 24,
        kind: "bit-string",
        components: 0,
        bits: 8,
    },
    crate::value::Definition {
        name: "Data_TimeStamp",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 24,
        kind: "reference",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_TimeStampExtended",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 24,
        kind: "reference",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "COVCounter",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 24,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_RealQ",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 24,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_StateQ",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 25,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_DiscreteQ",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 25,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_StateSupplementalQ",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 25,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_RealQTimeTag",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 25,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_StateQTimeTag",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 25,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_DiscreteQTimeTag",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 25,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_ StateSupplementalQTimeTag",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 25,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_RealExtended",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 25,
        kind: "structure",
        components: 4,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_StateExtended",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 25,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_DiscreteExtended",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 26,
        kind: "structure",
        components: 4,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_StateSupplementalExtended",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 26,
        kind: "structure",
        components: 4,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_RealQTimeTagExtended",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 26,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_StateQTimeTagExtended",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 26,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_DiscreteQTimeTagExtended",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 26,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_State_SupplementalQTimeTagExtended",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 26,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "IndicationPointConfig",
        standard: "IEC 60870-6-802",
        clause: "6.2.1",
        page: 26,
        kind: "structure",
        components: 5,
        bits: 0,
    },
    crate::value::Definition {
        name: "Control_Command",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Control_Setpoint_Real",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Control_Setpoint_Discrete",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "SBO_CheckBackName",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "SelectState",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TagFlags",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "bit-string",
        components: 0,
        bits: 3,
    },
    crate::value::Definition {
        name: "TextString",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "SBO",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "Tag_Value",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "ControlConfig",
        standard: "IEC 60870-6-802",
        clause: "6.2.2",
        page: 27,
        kind: "structure",
        components: 4,
        bits: 0,
    },
    crate::value::Definition {
        name: "SingleFlags",
        standard: "IEC 60870-6-802",
        clause: "6.2.3",
        page: 27,
        kind: "bit-string",
        components: 0,
        bits: 8,
    },
    crate::value::Definition {
        name: "EventFlags",
        standard: "IEC 60870-6-802",
        clause: "6.2.3",
        page: 27,
        kind: "bit-string",
        components: 0,
        bits: 8,
    },
    crate::value::Definition {
        name: "PackedFlags",
        standard: "IEC 60870-6-802",
        clause: "6.2.3",
        page: 28,
        kind: "bit-string",
        components: 0,
        bits: 8,
    },
    crate::value::Definition {
        name: "SingleProtectionEvent",
        standard: "IEC 60870-6-802",
        clause: "6.2.3",
        page: 28,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "PackedProtectionEvent",
        standard: "IEC 60870-6-802",
        clause: "6.2.3",
        page: 28,
        kind: "structure",
        components: 4,
        bits: 0,
    },
    crate::value::Definition {
        name: "ActionId",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "ClassId",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "CommentString",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_Real",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "DeviceId",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "DeviceName",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "DeviceNumber",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Number",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "OutageAmountTypeId",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "OutagePeriodId",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "OutageTypeId",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "PlanTypeId",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "ReferenceNum",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 28,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "ScheduleTime",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 29,
        kind: "reference",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "StationNameString",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 29,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TimeStampS",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 29,
        kind: "reference",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "UtilityId",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 29,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "DONewRevSched",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 29,
        kind: "structure",
        components: 21,
        bits: 0,
    },
    crate::value::Definition {
        name: "DOCancel",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 29,
        kind: "structure",
        components: 11,
        bits: 0,
    },
    crate::value::Definition {
        name: "DOActual",
        standard: "IEC 60870-6-802",
        clause: "6.3",
        page: 29,
        kind: "structure",
        components: 13,
        bits: 0,
    },
    crate::value::Definition {
        name: "ReferenceNum",
        standard: "IEC 60870-6-802",
        clause: "6.4",
        page: 30,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Number",
        standard: "IEC 60870-6-802",
        clause: "6.4",
        page: 30,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "InfoMessHeader",
        standard: "IEC 60870-6-802",
        clause: "6.4",
        page: 30,
        kind: "structure",
        components: 4,
        bits: 0,
    },
    crate::value::Definition {
        name: "InfoBufXX",
        standard: "IEC 60870-6-802",
        clause: "6.4",
        page: 30,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "CommodityClass",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_Discrete",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_Real",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Data_TimeStamp",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "reference",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Duration",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "reference",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "IntegerId",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "InterchangeId",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "NameString",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "NumIntegers",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "NumFloats",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "NumPeriods",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "NumProfiles",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "NumSegs",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Price",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "ReferenceNum",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "ScheduleTime",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "reference",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TransactionAmount",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TransactionCode",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TransmissionSegCode",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "UtilityId",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TASegmentsPeriodic",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 65,
        kind: "structure",
        components: 15,
        bits: 0,
    },
    crate::value::Definition {
        name: "TANoSegmentsPeriodic",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 66,
        kind: "structure",
        components: 14,
        bits: 0,
    },
    crate::value::Definition {
        name: "TASegmentsProfile",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 66,
        kind: "structure",
        components: 11,
        bits: 0,
    },
    crate::value::Definition {
        name: "TANoSegmentsProfile",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 66,
        kind: "structure",
        components: 10,
        bits: 0,
    },
    crate::value::Definition {
        name: "TATransmissionSegment",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 66,
        kind: "structure",
        components: 11,
        bits: 0,
    },
    crate::value::Definition {
        name: "TATransmissionSegmentProfile",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 67,
        kind: "structure",
        components: 10,
        bits: 0,
    },
    crate::value::Definition {
        name: "TAProfileValue",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 67,
        kind: "structure",
        components: 5,
        bits: 0,
    },
    crate::value::Definition {
        name: "AccountRequest",
        standard: "IEC 60870-6-802",
        clause: "B.2",
        page: 67,
        kind: "structure",
        components: 5,
        bits: 0,
    },
    crate::value::Definition {
        name: "AvailReasonCode",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 67,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Capacity",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 67,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "DateAndTime",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 67,
        kind: "reference",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "LFCReasonCode",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 67,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "PlantId",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 67,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Price",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 68,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "RampRate",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 68,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "ReferenceNum",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 68,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "ReportStatus",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 68,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "UnitId",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 68,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "UnitCapacity",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 68,
        kind: "structure",
        components: 5,
        bits: 0,
    },
    crate::value::Definition {
        name: "RampRates",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 68,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "AvailabilityClass",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 68,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "Available",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 68,
        kind: "structure",
        components: 15,
        bits: 0,
    },
    crate::value::Definition {
        name: "UnAvailable",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 69,
        kind: "structure",
        components: 11,
        bits: 0,
    },
    crate::value::Definition {
        name: "StatusAvailable",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 69,
        kind: "structure",
        components: 8,
        bits: 0,
    },
    crate::value::Definition {
        name: "StatusUnAvailable",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 69,
        kind: "structure",
        components: 6,
        bits: 0,
    },
    crate::value::Definition {
        name: "Forecast",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 69,
        kind: "structure",
        components: 7,
        bits: 0,
    },
    crate::value::Definition {
        name: "CurveLimit",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 70,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "CurveType",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 70,
        kind: "bit-string",
        components: 0,
        bits: 17,
    },
    crate::value::Definition {
        name: "Coefficient",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 70,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "CurveSegmentDescription",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 70,
        kind: "structure",
        components: 4,
        bits: 0,
    },
    crate::value::Definition {
        name: "Curve",
        standard: "IEC 60870-6-802",
        clause: "B.3",
        page: 70,
        kind: "structure",
        components: 5,
        bits: 0,
    },
    crate::value::Definition {
        name: "MatrixId",
        standard: "IEC 60870-6-802",
        clause: "B.4.2",
        page: 71,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "IntegerArrayXX",
        standard: "IEC 60870-6-802",
        clause: "B.4.2",
        page: 72,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "FloatArrayXX",
        standard: "IEC 60870-6-802",
        clause: "B.4.2",
        page: 72,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Text32ArrayXX",
        standard: "IEC 60870-6-802",
        clause: "B.4.2",
        page: 72,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TransactionCode",
        standard: "IEC 60870-6-802",
        clause: "B.5",
        page: 72,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "NumRows",
        standard: "IEC 60870-6-802",
        clause: "B.5",
        page: 72,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "GeneralDataReport",
        standard: "IEC 60870-6-802",
        clause: "B.5",
        page: 72,
        kind: "structure",
        components: 17,
        bits: 0,
    },
    crate::value::Definition {
        name: "GeneralDataResponse",
        standard: "IEC 60870-6-802",
        clause: "B.6",
        page: 72,
        kind: "structure",
        components: 7,
        bits: 0,
    },
];
/// IEC 60870-6-802 6.2.1 (page 24). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataState(pub u64);
impl DataState {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 8;
    /// The mask of the flag `State_hi`.
    pub const STATE_HI: u64 = 0x1;
    /// The mask of the flag `State_lo`.
    pub const STATE_LO: u64 = 0x2;
    /// The mask of the flag `Validity_hi`.
    pub const VALIDITY_HI: u64 = 0x4;
    /// The mask of the flag `Validity_lo`.
    pub const VALIDITY_LO: u64 = 0x8;
    /// The mask of the flag `CurrentSource_hi`.
    pub const CURRENT_SOURCE_HI: u64 = 0x10;
    /// The mask of the flag `CurrentSource_lo`.
    pub const CURRENT_SOURCE_LO: u64 = 0x20;
    /// The mask of the flag `NormalValue`.
    pub const NORMAL_VALUE: u64 = 0x40;
    /// The mask of the flag `TimeStampQuality`.
    pub const TIME_STAMP_QUALITY: u64 = 0x80;
}
impl DataState {
    /// Whether the flag `State_hi` is set.
    pub fn is_state_hi(self) -> bool {
        self.0 & Self::STATE_HI != 0
    }
    /// Whether the flag `State_lo` is set.
    pub fn is_state_lo(self) -> bool {
        self.0 & Self::STATE_LO != 0
    }
    /// Whether the flag `Validity_hi` is set.
    pub fn is_validity_hi(self) -> bool {
        self.0 & Self::VALIDITY_HI != 0
    }
    /// Whether the flag `Validity_lo` is set.
    pub fn is_validity_lo(self) -> bool {
        self.0 & Self::VALIDITY_LO != 0
    }
    /// Whether the flag `CurrentSource_hi` is set.
    pub fn is_current_source_hi(self) -> bool {
        self.0 & Self::CURRENT_SOURCE_HI != 0
    }
    /// Whether the flag `CurrentSource_lo` is set.
    pub fn is_current_source_lo(self) -> bool {
        self.0 & Self::CURRENT_SOURCE_LO != 0
    }
    /// Whether the flag `NormalValue` is set.
    pub fn is_normal_value(self) -> bool {
        self.0 & Self::NORMAL_VALUE != 0
    }
    /// Whether the flag `TimeStampQuality` is set.
    pub fn is_time_stamp_quality(self) -> bool {
        self.0 & Self::TIME_STAMP_QUALITY != 0
    }
}
impl crate::mms::MmsValue for DataState {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 24). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataStateSupplemental(pub u64);
impl DataStateSupplemental {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 8;
    /// The mask of the flag `State_hi`.
    pub const STATE_HI: u64 = 0x1;
    /// The mask of the flag `State_lo`.
    pub const STATE_LO: u64 = 0x2;
    /// The mask of the flag `Tag_hi`.
    pub const TAG_HI: u64 = 0x4;
    /// The mask of the flag `Tag_lo`.
    pub const TAG_LO: u64 = 0x8;
    /// The mask of the flag `ExpectedState_hi`.
    pub const EXPECTED_STATE_HI: u64 = 0x10;
    /// The mask of the flag `ExpectedState_lo`.
    pub const EXPECTED_STATE_LO: u64 = 0x20;
    /// The mask of the flag `Reserved`.
    pub const RESERVED: u64 = 0x40;
}
impl DataStateSupplemental {
    /// Whether the flag `State_hi` is set.
    pub fn is_state_hi(self) -> bool {
        self.0 & Self::STATE_HI != 0
    }
    /// Whether the flag `State_lo` is set.
    pub fn is_state_lo(self) -> bool {
        self.0 & Self::STATE_LO != 0
    }
    /// Whether the flag `Tag_hi` is set.
    pub fn is_tag_hi(self) -> bool {
        self.0 & Self::TAG_HI != 0
    }
    /// Whether the flag `Tag_lo` is set.
    pub fn is_tag_lo(self) -> bool {
        self.0 & Self::TAG_LO != 0
    }
    /// Whether the flag `ExpectedState_hi` is set.
    pub fn is_expected_state_hi(self) -> bool {
        self.0 & Self::EXPECTED_STATE_HI != 0
    }
    /// Whether the flag `ExpectedState_lo` is set.
    pub fn is_expected_state_lo(self) -> bool {
        self.0 & Self::EXPECTED_STATE_LO != 0
    }
    /// Whether the flag `Reserved` is set.
    pub fn is_reserved(self) -> bool {
        self.0 & Self::RESERVED != 0
    }
}
impl crate::mms::MmsValue for DataStateSupplemental {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 24). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataFlags(pub u64);
impl DataFlags {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 8;
    /// The mask of the flag `Validity_hi`.
    pub const VALIDITY_HI: u64 = 0x4;
    /// The mask of the flag `Validity_lo`.
    pub const VALIDITY_LO: u64 = 0x8;
    /// The mask of the flag `CurrentSource_hi`.
    pub const CURRENT_SOURCE_HI: u64 = 0x10;
    /// The mask of the flag `CurrentSource_lo`.
    pub const CURRENT_SOURCE_LO: u64 = 0x20;
    /// The mask of the flag `NormalValue`.
    pub const NORMAL_VALUE: u64 = 0x40;
    /// The mask of the flag `TimeStampQuality`.
    pub const TIME_STAMP_QUALITY: u64 = 0x80;
}
impl DataFlags {
    /// Whether the flag `Validity_hi` is set.
    pub fn is_validity_hi(self) -> bool {
        self.0 & Self::VALIDITY_HI != 0
    }
    /// Whether the flag `Validity_lo` is set.
    pub fn is_validity_lo(self) -> bool {
        self.0 & Self::VALIDITY_LO != 0
    }
    /// Whether the flag `CurrentSource_hi` is set.
    pub fn is_current_source_hi(self) -> bool {
        self.0 & Self::CURRENT_SOURCE_HI != 0
    }
    /// Whether the flag `CurrentSource_lo` is set.
    pub fn is_current_source_lo(self) -> bool {
        self.0 & Self::CURRENT_SOURCE_LO != 0
    }
    /// Whether the flag `NormalValue` is set.
    pub fn is_normal_value(self) -> bool {
        self.0 & Self::NORMAL_VALUE != 0
    }
    /// Whether the flag `TimeStampQuality` is set.
    pub fn is_time_stamp_quality(self) -> bool {
        self.0 & Self::TIME_STAMP_QUALITY != 0
    }
}
impl crate::mms::MmsValue for DataFlags {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 24). reference definition.
pub type DataTimeStampExtended = crate::generated::iso503::TimeStampExtended;
/// IEC 60870-6-802 6.2.1 (page 24). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct COVCounter(u64);
impl COVCounter {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: u64 = 0;
    /// The greatest value.
    pub const MAX: u64 = 65535;
    /// The value, if it is in range.
    pub fn new(value: u64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 65535,
            })
        }
    }
    /// The value.
    pub fn get(self) -> u64 {
        self.0
    }
}
impl crate::mms::MmsValue for COVCounter {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Unsigned(u128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Unsigned(value) => {
                let value = u64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: i128::try_from(*value).unwrap_or(i128::MAX),
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("unsigned", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 24). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRealQ {
    /// The component `value`.
    pub value: crate::generated::iso802::clause_6_2_1::DataReal,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataRealQ {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1] => Ok(Self { value: <crate::generated::iso802::clause_6_2_1::DataReal as crate::mms::MmsValue>::from_mms(item0)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item1)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 25). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataStateQ {
    /// The component `value`.
    pub value: crate::generated::iso802::DataState,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataStateQ {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items
                .as_slice()
            {
                [item0, item1] => Ok(Self {
                    value: <crate::generated::iso802::DataState as crate::mms::MmsValue>::from_mms(
                        item0,
                    )?,
                    flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(
                        item1,
                    )?,
                }),
                _ => Err(crate::mms::ValueError::Format(
                    "a structure of another number of components",
                )),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 25). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataDiscreteQ {
    /// The component `value`.
    pub value: crate::generated::iso802::clause_6_2_1::DataDiscrete,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataDiscreteQ {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1] => Ok(Self { value: <crate::generated::iso802::clause_6_2_1::DataDiscrete as crate::mms::MmsValue>::from_mms(item0)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item1)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 25). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataStateSupplementalQ {
    /// The component `value`.
    pub value: crate::generated::iso802::DataStateSupplemental,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataStateSupplementalQ {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1] => Ok(Self { value: <crate::generated::iso802::DataStateSupplemental as crate::mms::MmsValue>::from_mms(item0)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item1)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 25). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRealQTimeTag {
    /// The component `value`.
    pub value: crate::generated::iso802::clause_6_2_1::DataReal,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataRealQTimeTag {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { value: <crate::generated::iso802::clause_6_2_1::DataReal as crate::mms::MmsValue>::from_mms(item0)?, time_stamp: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item1)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 25). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataStateQTimeTag {
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataState,
}
impl crate::mms::MmsValue for DataStateQTimeTag {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1] => Ok(Self { time_stamp: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item0)?, flags: <crate::generated::iso802::DataState as crate::mms::MmsValue>::from_mms(item1)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 25). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataDiscreteQTimeTag {
    /// The component `value`.
    pub value: crate::generated::iso802::clause_6_2_1::DataDiscrete,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataDiscreteQTimeTag {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { value: <crate::generated::iso802::clause_6_2_1::DataDiscrete as crate::mms::MmsValue>::from_mms(item0)?, time_stamp: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item1)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 25). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataStateSupplementalQTimeTag {
    /// The component `value`.
    pub value: crate::generated::iso802::DataStateSupplemental,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataStateSupplementalQTimeTag {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { value: <crate::generated::iso802::DataStateSupplemental as crate::mms::MmsValue>::from_mms(item0)?, time_stamp: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item1)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 25). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRealExtended {
    /// The component `value`.
    pub value: crate::generated::iso802::clause_6_2_1::DataReal,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
    /// The component `cov`.
    pub cov: crate::generated::iso802::COVCounter,
}
impl crate::mms::MmsValue for DataRealExtended {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
            crate::mms::MmsValue::to_mms(&self.cov),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3] => Ok(Self { value: <crate::generated::iso802::clause_6_2_1::DataReal as crate::mms::MmsValue>::from_mms(item0)?, time_stamp: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item1)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item2)?, cov: <crate::generated::iso802::COVCounter as crate::mms::MmsValue>::from_mms(item3)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 25). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataStateExtended {
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataState,
    /// The component `cov`.
    pub cov: crate::generated::iso802::COVCounter,
}
impl crate::mms::MmsValue for DataStateExtended {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
            crate::mms::MmsValue::to_mms(&self.cov),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { time_stamp: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item0)?, flags: <crate::generated::iso802::DataState as crate::mms::MmsValue>::from_mms(item1)?, cov: <crate::generated::iso802::COVCounter as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataDiscreteExtended {
    /// The component `value`.
    pub value: crate::generated::iso802::clause_6_2_1::DataDiscrete,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
    /// The component `cov`.
    pub cov: crate::generated::iso802::COVCounter,
}
impl crate::mms::MmsValue for DataDiscreteExtended {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
            crate::mms::MmsValue::to_mms(&self.cov),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3] => Ok(Self { value: <crate::generated::iso802::clause_6_2_1::DataDiscrete as crate::mms::MmsValue>::from_mms(item0)?, time_stamp: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item1)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item2)?, cov: <crate::generated::iso802::COVCounter as crate::mms::MmsValue>::from_mms(item3)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataStateSupplementalExtended {
    /// The component `value`.
    pub value: crate::generated::iso802::DataStateSupplemental,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
    /// The component `cov`.
    pub cov: crate::generated::iso802::COVCounter,
}
impl crate::mms::MmsValue for DataStateSupplementalExtended {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
            crate::mms::MmsValue::to_mms(&self.cov),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3] => Ok(Self { value: <crate::generated::iso802::DataStateSupplemental as crate::mms::MmsValue>::from_mms(item0)?, time_stamp: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item1)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item2)?, cov: <crate::generated::iso802::COVCounter as crate::mms::MmsValue>::from_mms(item3)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRealQTimeTagExtended {
    /// The component `value`.
    pub value: crate::generated::iso802::clause_6_2_1::DataReal,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::DataTimeStampExtended,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataRealQTimeTagExtended {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { value: <crate::generated::iso802::clause_6_2_1::DataReal as crate::mms::MmsValue>::from_mms(item0)?, time_stamp: <crate::generated::iso802::DataTimeStampExtended as crate::mms::MmsValue>::from_mms(item1)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataStateQTimeTagExtended {
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::DataTimeStampExtended,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataState,
}
impl crate::mms::MmsValue for DataStateQTimeTagExtended {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1] => Ok(Self { time_stamp: <crate::generated::iso802::DataTimeStampExtended as crate::mms::MmsValue>::from_mms(item0)?, flags: <crate::generated::iso802::DataState as crate::mms::MmsValue>::from_mms(item1)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataDiscreteQTimeTagExtended {
    /// The component `value`.
    pub value: crate::generated::iso802::clause_6_2_1::DataDiscrete,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::DataTimeStampExtended,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataDiscreteQTimeTagExtended {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { value: <crate::generated::iso802::clause_6_2_1::DataDiscrete as crate::mms::MmsValue>::from_mms(item0)?, time_stamp: <crate::generated::iso802::DataTimeStampExtended as crate::mms::MmsValue>::from_mms(item1)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DataStateSupplementalQTimeTagExtended {
    /// The component `value`.
    pub value: crate::generated::iso802::DataStateSupplemental,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::DataTimeStampExtended,
    /// The component `flags`.
    pub flags: crate::generated::iso802::DataFlags,
}
impl crate::mms::MmsValue for DataStateSupplementalQTimeTagExtended {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.value),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.flags),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { value: <crate::generated::iso802::DataStateSupplemental as crate::mms::MmsValue>::from_mms(item0)?, time_stamp: <crate::generated::iso802::DataTimeStampExtended as crate::mms::MmsValue>::from_mms(item1)?, flags: <crate::generated::iso802::DataFlags as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndicationPointConfigPointType(i64);
impl IndicationPointConfigPointType {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: i64 = 0;
    /// The greatest value.
    pub const MAX: i64 = 2;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 2,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for IndicationPointConfigPointType {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndicationPointConfigQualityClass(i64);
impl IndicationPointConfigQualityClass {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: i64 = 0;
    /// The greatest value.
    pub const MAX: i64 = 1;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 1,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for IndicationPointConfigQualityClass {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndicationPointConfigNormalSource(i64);
impl IndicationPointConfigNormalSource {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: i64 = 0;
    /// The greatest value.
    pub const MAX: i64 = 3;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 3,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for IndicationPointConfigNormalSource {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndicationPointConfigTimeStampClass(i64);
impl IndicationPointConfigTimeStampClass {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: i64 = 0;
    /// The greatest value.
    pub const MAX: i64 = 1;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 1,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for IndicationPointConfigTimeStampClass {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndicationPointConfigCOVClass(i64);
impl IndicationPointConfigCOVClass {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: i64 = 0;
    /// The greatest value.
    pub const MAX: i64 = 1;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 1,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for IndicationPointConfigCOVClass {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.1 (page 26). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct IndicationPointConfig {
    /// The component `point_type`.
    pub point_type: IndicationPointConfigPointType,
    /// The component `quality_class`.
    pub quality_class: IndicationPointConfigQualityClass,
    /// The component `normal_source`.
    pub normal_source: IndicationPointConfigNormalSource,
    /// The component `time_stamp_class`.
    pub time_stamp_class: IndicationPointConfigTimeStampClass,
    /// The component `cov_class`.
    pub cov_class: IndicationPointConfigCOVClass,
}
impl crate::mms::MmsValue for IndicationPointConfig {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.point_type),
            crate::mms::MmsValue::to_mms(&self.quality_class),
            crate::mms::MmsValue::to_mms(&self.normal_source),
            crate::mms::MmsValue::to_mms(&self.time_stamp_class),
            crate::mms::MmsValue::to_mms(&self.cov_class),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items
                .as_slice()
            {
                [item0, item1, item2, item3, item4] => Ok(Self {
                    point_type: <IndicationPointConfigPointType as crate::mms::MmsValue>::from_mms(
                        item0,
                    )?,
                    quality_class:
                        <IndicationPointConfigQualityClass as crate::mms::MmsValue>::from_mms(
                            item1,
                        )?,
                    normal_source:
                        <IndicationPointConfigNormalSource as crate::mms::MmsValue>::from_mms(
                            item2,
                        )?,
                    time_stamp_class:
                        <IndicationPointConfigTimeStampClass as crate::mms::MmsValue>::from_mms(
                            item3,
                        )?,
                    cov_class: <IndicationPointConfigCOVClass as crate::mms::MmsValue>::from_mms(
                        item4,
                    )?,
                }),
                _ => Err(crate::mms::ValueError::Format(
                    "a structure of another number of components",
                )),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlCommand(i64);
impl ControlCommand {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for ControlCommand {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). type definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlSetpointReal(pub f64);
impl ControlSetpointReal {
    /// The format width, in bits.
    pub const FORMAT_WIDTH: u32 = 32;
    /// The exponent width, in bits.
    pub const EXPONENT_WIDTH: u32 = 8;
}
impl crate::mms::MmsValue for ControlSetpointReal {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let [a, b, c, d] = (self.0 as f32).to_be_bytes();
        igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
            igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlSetpointDiscrete(i64);
impl ControlSetpointDiscrete {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for ControlSetpointDiscrete {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SBOCheckBackName(i64);
impl SBOCheckBackName {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for SBOCheckBackName {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectState(pub bool);
impl crate::mms::MmsValue for SelectState {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Boolean(self.0)
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Boolean(value) => Ok(Self(*value)),
            other => Err(crate::mms::ValueError::kind_of("boolean", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagFlags(pub u64);
impl TagFlags {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 3;
    /// The mask of the flag `tag_hi`.
    pub const TAG_HI: u64 = 0x1;
    /// The mask of the flag `tag_lo`.
    pub const TAG_LO: u64 = 0x2;
    /// The mask of the flag `tag_state`.
    pub const TAG_STATE: u64 = 0x4;
}
impl TagFlags {
    /// Whether the flag `tag_hi` is set.
    pub fn is_tag_hi(self) -> bool {
        self.0 & Self::TAG_HI != 0
    }
    /// Whether the flag `tag_lo` is set.
    pub fn is_tag_lo(self) -> bool {
        self.0 & Self::TAG_LO != 0
    }
    /// Whether the flag `tag_state` is set.
    pub fn is_tag_state(self) -> bool {
        self.0 & Self::TAG_STATE != 0
    }
}
impl crate::mms::MmsValue for TagFlags {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextString(String);
impl TextString {
    /// The greatest number of characters (a maximum: decision D-004).
    pub const MAX_LEN: usize = 255;
    /// Whether the length is variable (the data file says so).
    pub const VARYING: bool = false;
    /// The string, if it has at most the greatest number of visible characters (decision D-014).
    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {
        let text = text.into();
        crate::value::check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }
    /// The string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl crate::mms::MmsValue for TextString {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::VisibleString(self.as_str().to_string())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::VisibleString(text) => {
                Ok(Self::new(text.clone())?)
            }
            other => Err(crate::mms::ValueError::kind_of("visible string", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct SBO {
    /// The component `time_out`.
    pub time_out: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `select`.
    pub select: crate::generated::iso802::SelectState,
}
impl crate::mms::MmsValue for SBO {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.time_out),
            crate::mms::MmsValue::to_mms(&self.select),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1] => Ok(Self { time_out: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item0)?, select: <crate::generated::iso802::SelectState as crate::mms::MmsValue>::from_mms(item1)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TagValue {
    /// The component `flags`.
    pub flags: crate::generated::iso802::TagFlags,
    /// The component `reason`.
    pub reason: crate::generated::iso802::TextString,
}
impl crate::mms::MmsValue for TagValue {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.flags),
            crate::mms::MmsValue::to_mms(&self.reason),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items
                .as_slice()
            {
                [item0, item1] => Ok(Self {
                    flags: <crate::generated::iso802::TagFlags as crate::mms::MmsValue>::from_mms(
                        item0,
                    )?,
                    reason:
                        <crate::generated::iso802::TextString as crate::mms::MmsValue>::from_mms(
                            item1,
                        )?,
                }),
                _ => Err(crate::mms::ValueError::Format(
                    "a structure of another number of components",
                )),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlConfigControlPointType(i64);
impl ControlConfigControlPointType {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: i64 = 0;
    /// The greatest value.
    pub const MAX: i64 = 2;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 2,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for ControlConfigControlPointType {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlConfigSetPointType(i64);
impl ControlConfigSetPointType {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: i64 = 0;
    /// The greatest value.
    pub const MAX: i64 = 2;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 2,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for ControlConfigSetPointType {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlConfigDeviceClass(i64);
impl ControlConfigDeviceClass {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: i64 = 0;
    /// The greatest value.
    pub const MAX: i64 = 1;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 1,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for ControlConfigDeviceClass {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlConfigTagClass(i64);
impl ControlConfigTagClass {
    /// The width in bits.
    pub const WIDTH: u32 = 8;
    /// The least value.
    pub const MIN: i64 = 0;
    /// The greatest value.
    pub const MAX: i64 = 1;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: 0,
                max: 1,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for ControlConfigTagClass {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.2 (page 27). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ControlConfig {
    /// The component `control_point_type`.
    pub control_point_type: ControlConfigControlPointType,
    /// The component `set_point_type`.
    pub set_point_type: ControlConfigSetPointType,
    /// The component `device_class`.
    pub device_class: ControlConfigDeviceClass,
    /// The component `tag_class`.
    pub tag_class: ControlConfigTagClass,
}
impl crate::mms::MmsValue for ControlConfig {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.control_point_type),
            crate::mms::MmsValue::to_mms(&self.set_point_type),
            crate::mms::MmsValue::to_mms(&self.device_class),
            crate::mms::MmsValue::to_mms(&self.tag_class),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items
                .as_slice()
            {
                [item0, item1, item2, item3] => Ok(Self {
                    control_point_type:
                        <ControlConfigControlPointType as crate::mms::MmsValue>::from_mms(item0)?,
                    set_point_type: <ControlConfigSetPointType as crate::mms::MmsValue>::from_mms(
                        item1,
                    )?,
                    device_class: <ControlConfigDeviceClass as crate::mms::MmsValue>::from_mms(
                        item2,
                    )?,
                    tag_class: <ControlConfigTagClass as crate::mms::MmsValue>::from_mms(item3)?,
                }),
                _ => Err(crate::mms::ValueError::Format(
                    "a structure of another number of components",
                )),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.3 (page 27). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleFlags(pub u64);
impl SingleFlags {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 8;
    /// The mask of the flag `ElapsedTimeValidity`.
    pub const ELAPSED_TIME_VALIDITY: u64 = 0x1;
    /// The mask of the flag `Blocked`.
    pub const BLOCKED: u64 = 0x2;
    /// The mask of the flag `Substituted`.
    pub const SUBSTITUTED: u64 = 0x4;
    /// The mask of the flag `Topical`.
    pub const TOPICAL: u64 = 0x8;
    /// The mask of the flag `EventValidity`.
    pub const EVENT_VALIDITY: u64 = 0x10;
    /// The mask of the flag `EventState_hi`.
    pub const EVENT_STATE_HI: u64 = 0x40;
    /// The mask of the flag `EventState_lo`.
    pub const EVENT_STATE_LO: u64 = 0x80;
}
impl SingleFlags {
    /// Whether the flag `ElapsedTimeValidity` is set.
    pub fn is_elapsed_time_validity(self) -> bool {
        self.0 & Self::ELAPSED_TIME_VALIDITY != 0
    }
    /// Whether the flag `Blocked` is set.
    pub fn is_blocked(self) -> bool {
        self.0 & Self::BLOCKED != 0
    }
    /// Whether the flag `Substituted` is set.
    pub fn is_substituted(self) -> bool {
        self.0 & Self::SUBSTITUTED != 0
    }
    /// Whether the flag `Topical` is set.
    pub fn is_topical(self) -> bool {
        self.0 & Self::TOPICAL != 0
    }
    /// Whether the flag `EventValidity` is set.
    pub fn is_event_validity(self) -> bool {
        self.0 & Self::EVENT_VALIDITY != 0
    }
    /// Whether the flag `EventState_hi` is set.
    pub fn is_event_state_hi(self) -> bool {
        self.0 & Self::EVENT_STATE_HI != 0
    }
    /// Whether the flag `EventState_lo` is set.
    pub fn is_event_state_lo(self) -> bool {
        self.0 & Self::EVENT_STATE_LO != 0
    }
}
impl crate::mms::MmsValue for SingleFlags {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.3 (page 27). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventFlags(pub u64);
impl EventFlags {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 8;
    /// The mask of the flag `General`.
    pub const GENERAL: u64 = 0x1;
    /// The mask of the flag `Phase1`.
    pub const PHASE1: u64 = 0x2;
    /// The mask of the flag `Phase2`.
    pub const PHASE2: u64 = 0x4;
    /// The mask of the flag `Phase3`.
    pub const PHASE3: u64 = 0x8;
    /// The mask of the flag `Earth`.
    pub const EARTH: u64 = 0x10;
    /// The mask of the flag `Reverse`.
    pub const REVERSE: u64 = 0x20;
}
impl EventFlags {
    /// Whether the flag `General` is set.
    pub fn is_general(self) -> bool {
        self.0 & Self::GENERAL != 0
    }
    /// Whether the flag `Phase1` is set.
    pub fn is_phase1(self) -> bool {
        self.0 & Self::PHASE1 != 0
    }
    /// Whether the flag `Phase2` is set.
    pub fn is_phase2(self) -> bool {
        self.0 & Self::PHASE2 != 0
    }
    /// Whether the flag `Phase3` is set.
    pub fn is_phase3(self) -> bool {
        self.0 & Self::PHASE3 != 0
    }
    /// Whether the flag `Earth` is set.
    pub fn is_earth(self) -> bool {
        self.0 & Self::EARTH != 0
    }
    /// Whether the flag `Reverse` is set.
    pub fn is_reverse(self) -> bool {
        self.0 & Self::REVERSE != 0
    }
}
impl crate::mms::MmsValue for EventFlags {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.3 (page 28). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedFlags(pub u64);
impl PackedFlags {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 8;
    /// The mask of the flag `ElapsedTimeValidity`.
    pub const ELAPSED_TIME_VALIDITY: u64 = 0x1;
    /// The mask of the flag `Blocked`.
    pub const BLOCKED: u64 = 0x2;
    /// The mask of the flag `Substituted`.
    pub const SUBSTITUTED: u64 = 0x4;
    /// The mask of the flag `Topical`.
    pub const TOPICAL: u64 = 0x8;
    /// The mask of the flag `EventValidity`.
    pub const EVENT_VALIDITY: u64 = 0x10;
}
impl PackedFlags {
    /// Whether the flag `ElapsedTimeValidity` is set.
    pub fn is_elapsed_time_validity(self) -> bool {
        self.0 & Self::ELAPSED_TIME_VALIDITY != 0
    }
    /// Whether the flag `Blocked` is set.
    pub fn is_blocked(self) -> bool {
        self.0 & Self::BLOCKED != 0
    }
    /// Whether the flag `Substituted` is set.
    pub fn is_substituted(self) -> bool {
        self.0 & Self::SUBSTITUTED != 0
    }
    /// Whether the flag `Topical` is set.
    pub fn is_topical(self) -> bool {
        self.0 & Self::TOPICAL != 0
    }
    /// Whether the flag `EventValidity` is set.
    pub fn is_event_validity(self) -> bool {
        self.0 & Self::EVENT_VALIDITY != 0
    }
}
impl crate::mms::MmsValue for PackedFlags {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.3 (page 28). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct SingleProtectionEvent {
    /// The component `single_event_flags`.
    pub single_event_flags: crate::generated::iso802::SingleFlags,
    /// The component `operating_time`.
    pub operating_time: crate::generated::iso503::TimeIntervalL16,
    /// The component `event_time`.
    pub event_time: crate::generated::iso503::TimeStampExtended,
}
impl crate::mms::MmsValue for SingleProtectionEvent {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.single_event_flags),
            crate::mms::MmsValue::to_mms(&self.operating_time),
            crate::mms::MmsValue::to_mms(&self.event_time),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { single_event_flags: <crate::generated::iso802::SingleFlags as crate::mms::MmsValue>::from_mms(item0)?, operating_time: <crate::generated::iso503::TimeIntervalL16 as crate::mms::MmsValue>::from_mms(item1)?, event_time: <crate::generated::iso503::TimeStampExtended as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.2.3 (page 28). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct PackedProtectionEvent {
    /// The component `packed_event`.
    pub packed_event: crate::generated::iso802::EventFlags,
    /// The component `packed_event_flags`.
    pub packed_event_flags: crate::generated::iso802::PackedFlags,
    /// The component `operating_time`.
    pub operating_time: crate::generated::iso503::TimeIntervalL16,
    /// The component `event_time`.
    pub event_time: crate::generated::iso503::TimeStampExtended,
}
impl crate::mms::MmsValue for PackedProtectionEvent {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.packed_event),
            crate::mms::MmsValue::to_mms(&self.packed_event_flags),
            crate::mms::MmsValue::to_mms(&self.operating_time),
            crate::mms::MmsValue::to_mms(&self.event_time),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3] => Ok(Self { packed_event: <crate::generated::iso802::EventFlags as crate::mms::MmsValue>::from_mms(item0)?, packed_event_flags: <crate::generated::iso802::PackedFlags as crate::mms::MmsValue>::from_mms(item1)?, operating_time: <crate::generated::iso503::TimeIntervalL16 as crate::mms::MmsValue>::from_mms(item2)?, event_time: <crate::generated::iso503::TimeStampExtended as crate::mms::MmsValue>::from_mms(item3)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionId(i64);
impl ActionId {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for ActionId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassId(i64);
impl ClassId {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for ClassId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentString(String);
impl CommentString {
    /// The greatest number of characters (a maximum: decision D-004).
    pub const MAX_LEN: usize = 128;
    /// Whether the length is variable (the data file says so).
    pub const VARYING: bool = false;
    /// The string, if it has at most the greatest number of visible characters (decision D-014).
    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {
        let text = text.into();
        crate::value::check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }
    /// The string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl crate::mms::MmsValue for CommentString {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::VisibleString(self.as_str().to_string())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::VisibleString(text) => {
                Ok(Self::new(text.clone())?)
            }
            other => Err(crate::mms::ValueError::kind_of("visible string", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceId(i64);
impl DeviceId {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for DeviceId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceName(String);
impl DeviceName {
    /// The greatest number of characters (a maximum: decision D-004).
    pub const MAX_LEN: usize = 32;
    /// Whether the length is variable (the data file says so).
    pub const VARYING: bool = true;
    /// The string, if it has at most the greatest number of visible characters (decision D-014).
    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {
        let text = text.into();
        crate::value::check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }
    /// The string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl crate::mms::MmsValue for DeviceName {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::VisibleString(self.as_str().to_string())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::VisibleString(text) => {
                Ok(Self::new(text.clone())?)
            }
            other => Err(crate::mms::ValueError::kind_of("visible string", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceNumber(i64);
impl DeviceNumber {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for DeviceNumber {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutageAmountTypeId(i64);
impl OutageAmountTypeId {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for OutageAmountTypeId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutagePeriodId(i64);
impl OutagePeriodId {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for OutagePeriodId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutageTypeId(i64);
impl OutageTypeId {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for OutageTypeId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 28). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanTypeId(i64);
impl PlanTypeId {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for PlanTypeId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 29). type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StationNameString(String);
impl StationNameString {
    /// The greatest number of characters (a maximum: decision D-004).
    pub const MAX_LEN: usize = 32;
    /// Whether the length is variable (the data file says so).
    pub const VARYING: bool = false;
    /// The string, if it has at most the greatest number of visible characters (decision D-014).
    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {
        let text = text.into();
        crate::value::check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }
    /// The string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl crate::mms::MmsValue for StationNameString {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::VisibleString(self.as_str().to_string())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::VisibleString(text) => {
                Ok(Self::new(text.clone())?)
            }
            other => Err(crate::mms::ValueError::kind_of("visible string", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 29). reference definition.
pub type TimeStampS = crate::generated::iso503::GMTBasedS;
/// IEC 60870-6-802 6.3 (page 29). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DONewRevSched {
    /// The component `outage_reference_id`.
    pub outage_reference_id: crate::generated::iso802::clause_6_3::ReferenceNum,
    /// The component `owning_utility_id`.
    pub owning_utility_id: crate::generated::iso802::clause_6_3::UtilityId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::TimeStampS,
    /// The component `station_name`.
    pub station_name: crate::generated::iso802::StationNameString,
    /// The component `device_name`.
    pub device_name: crate::generated::iso802::DeviceName,
    /// The component `device_type`.
    pub device_type: crate::generated::iso802::DeviceId,
    /// The component `device_number`.
    pub device_number: crate::generated::iso802::clause_6_3::Number,
    /// The component `device_rating`.
    pub device_rating: crate::generated::iso802::clause_6_3::DataReal,
    /// The component `activity_date_and_time`.
    pub activity_date_and_time: crate::generated::iso802::clause_6_3::ScheduleTime,
    /// The component `plan_type`.
    pub plan_type: crate::generated::iso802::PlanTypeId,
    /// The component `planned_open_or_out_of_service_date_and_time`.
    pub planned_open_or_out_of_service_date_and_time:
        crate::generated::iso802::clause_6_3::ScheduleTime,
    /// The component `planned_close_or_in_service_date_and_time`.
    pub planned_close_or_in_service_date_and_time:
        crate::generated::iso802::clause_6_3::ScheduleTime,
    /// The component `outage_period`.
    pub outage_period: crate::generated::iso802::OutagePeriodId,
    /// The component `outage_type`.
    pub outage_type: crate::generated::iso802::OutageTypeId,
    /// The component `outage_type_amount`.
    pub outage_type_amount: crate::generated::iso802::OutageAmountTypeId,
    /// The component `amount`.
    pub amount: crate::generated::iso802::clause_6_3::DataReal,
    /// The component `upper_operating_limit`.
    pub upper_operating_limit: crate::generated::iso802::clause_6_3::DataReal,
    /// The component `lower_operating_limit`.
    pub lower_operating_limit: crate::generated::iso802::clause_6_3::DataReal,
    /// The component `class`.
    pub class: crate::generated::iso802::ClassId,
    /// The component `comments`.
    pub comments: crate::generated::iso802::CommentString,
    /// The component `outage_effect`.
    pub outage_effect: crate::generated::iso802::CommentString,
}
impl crate::mms::MmsValue for DONewRevSched {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.outage_reference_id),
            crate::mms::MmsValue::to_mms(&self.owning_utility_id),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.station_name),
            crate::mms::MmsValue::to_mms(&self.device_name),
            crate::mms::MmsValue::to_mms(&self.device_type),
            crate::mms::MmsValue::to_mms(&self.device_number),
            crate::mms::MmsValue::to_mms(&self.device_rating),
            crate::mms::MmsValue::to_mms(&self.activity_date_and_time),
            crate::mms::MmsValue::to_mms(&self.plan_type),
            crate::mms::MmsValue::to_mms(&self.planned_open_or_out_of_service_date_and_time),
            crate::mms::MmsValue::to_mms(&self.planned_close_or_in_service_date_and_time),
            crate::mms::MmsValue::to_mms(&self.outage_period),
            crate::mms::MmsValue::to_mms(&self.outage_type),
            crate::mms::MmsValue::to_mms(&self.outage_type_amount),
            crate::mms::MmsValue::to_mms(&self.amount),
            crate::mms::MmsValue::to_mms(&self.upper_operating_limit),
            crate::mms::MmsValue::to_mms(&self.lower_operating_limit),
            crate::mms::MmsValue::to_mms(&self.class),
            crate::mms::MmsValue::to_mms(&self.comments),
            crate::mms::MmsValue::to_mms(&self.outage_effect),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10, item11, item12, item13, item14, item15, item16, item17, item18, item19, item20] => Ok(Self { outage_reference_id: <crate::generated::iso802::clause_6_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, owning_utility_id: <crate::generated::iso802::clause_6_3::UtilityId as crate::mms::MmsValue>::from_mms(item1)?, time_stamp: <crate::generated::iso802::TimeStampS as crate::mms::MmsValue>::from_mms(item2)?, station_name: <crate::generated::iso802::StationNameString as crate::mms::MmsValue>::from_mms(item3)?, device_name: <crate::generated::iso802::DeviceName as crate::mms::MmsValue>::from_mms(item4)?, device_type: <crate::generated::iso802::DeviceId as crate::mms::MmsValue>::from_mms(item5)?, device_number: <crate::generated::iso802::clause_6_3::Number as crate::mms::MmsValue>::from_mms(item6)?, device_rating: <crate::generated::iso802::clause_6_3::DataReal as crate::mms::MmsValue>::from_mms(item7)?, activity_date_and_time: <crate::generated::iso802::clause_6_3::ScheduleTime as crate::mms::MmsValue>::from_mms(item8)?, plan_type: <crate::generated::iso802::PlanTypeId as crate::mms::MmsValue>::from_mms(item9)?, planned_open_or_out_of_service_date_and_time: <crate::generated::iso802::clause_6_3::ScheduleTime as crate::mms::MmsValue>::from_mms(item10)?, planned_close_or_in_service_date_and_time: <crate::generated::iso802::clause_6_3::ScheduleTime as crate::mms::MmsValue>::from_mms(item11)?, outage_period: <crate::generated::iso802::OutagePeriodId as crate::mms::MmsValue>::from_mms(item12)?, outage_type: <crate::generated::iso802::OutageTypeId as crate::mms::MmsValue>::from_mms(item13)?, outage_type_amount: <crate::generated::iso802::OutageAmountTypeId as crate::mms::MmsValue>::from_mms(item14)?, amount: <crate::generated::iso802::clause_6_3::DataReal as crate::mms::MmsValue>::from_mms(item15)?, upper_operating_limit: <crate::generated::iso802::clause_6_3::DataReal as crate::mms::MmsValue>::from_mms(item16)?, lower_operating_limit: <crate::generated::iso802::clause_6_3::DataReal as crate::mms::MmsValue>::from_mms(item17)?, class: <crate::generated::iso802::ClassId as crate::mms::MmsValue>::from_mms(item18)?, comments: <crate::generated::iso802::CommentString as crate::mms::MmsValue>::from_mms(item19)?, outage_effect: <crate::generated::iso802::CommentString as crate::mms::MmsValue>::from_mms(item20)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 29). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DOCancel {
    /// The component `outage_reference_id`.
    pub outage_reference_id: crate::generated::iso802::clause_6_3::ReferenceNum,
    /// The component `owning_utility_id`.
    pub owning_utility_id: crate::generated::iso802::clause_6_3::UtilityId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::TimeStampS,
    /// The component `station_name`.
    pub station_name: crate::generated::iso802::StationNameString,
    /// The component `device_name`.
    pub device_name: crate::generated::iso802::DeviceName,
    /// The component `device_type`.
    pub device_type: crate::generated::iso802::DeviceId,
    /// The component `device_number`.
    pub device_number: crate::generated::iso802::clause_6_3::Number,
    /// The component `device_rating`.
    pub device_rating: crate::generated::iso802::clause_6_3::DataReal,
    /// The component `activity_date_and_time`.
    pub activity_date_and_time: crate::generated::iso802::clause_6_3::ScheduleTime,
    /// The component `comments`.
    pub comments: crate::generated::iso802::CommentString,
    /// The component `outage_effect`.
    pub outage_effect: crate::generated::iso802::CommentString,
}
impl crate::mms::MmsValue for DOCancel {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.outage_reference_id),
            crate::mms::MmsValue::to_mms(&self.owning_utility_id),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.station_name),
            crate::mms::MmsValue::to_mms(&self.device_name),
            crate::mms::MmsValue::to_mms(&self.device_type),
            crate::mms::MmsValue::to_mms(&self.device_number),
            crate::mms::MmsValue::to_mms(&self.device_rating),
            crate::mms::MmsValue::to_mms(&self.activity_date_and_time),
            crate::mms::MmsValue::to_mms(&self.comments),
            crate::mms::MmsValue::to_mms(&self.outage_effect),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10] => Ok(Self { outage_reference_id: <crate::generated::iso802::clause_6_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, owning_utility_id: <crate::generated::iso802::clause_6_3::UtilityId as crate::mms::MmsValue>::from_mms(item1)?, time_stamp: <crate::generated::iso802::TimeStampS as crate::mms::MmsValue>::from_mms(item2)?, station_name: <crate::generated::iso802::StationNameString as crate::mms::MmsValue>::from_mms(item3)?, device_name: <crate::generated::iso802::DeviceName as crate::mms::MmsValue>::from_mms(item4)?, device_type: <crate::generated::iso802::DeviceId as crate::mms::MmsValue>::from_mms(item5)?, device_number: <crate::generated::iso802::clause_6_3::Number as crate::mms::MmsValue>::from_mms(item6)?, device_rating: <crate::generated::iso802::clause_6_3::DataReal as crate::mms::MmsValue>::from_mms(item7)?, activity_date_and_time: <crate::generated::iso802::clause_6_3::ScheduleTime as crate::mms::MmsValue>::from_mms(item8)?, comments: <crate::generated::iso802::CommentString as crate::mms::MmsValue>::from_mms(item9)?, outage_effect: <crate::generated::iso802::CommentString as crate::mms::MmsValue>::from_mms(item10)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.3 (page 29). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DOActual {
    /// The component `outage_reference_id`.
    pub outage_reference_id: crate::generated::iso802::clause_6_3::ReferenceNum,
    /// The component `owning_utility_id`.
    pub owning_utility_id: crate::generated::iso802::clause_6_3::UtilityId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::TimeStampS,
    /// The component `station_name`.
    pub station_name: crate::generated::iso802::StationNameString,
    /// The component `device_name`.
    pub device_name: crate::generated::iso802::DeviceName,
    /// The component `device_type`.
    pub device_type: crate::generated::iso802::DeviceId,
    /// The component `device_number`.
    pub device_number: crate::generated::iso802::clause_6_3::Number,
    /// The component `device_rating`.
    pub device_rating: crate::generated::iso802::clause_6_3::DataReal,
    /// The component `activity_date_and_time`.
    pub activity_date_and_time: crate::generated::iso802::clause_6_3::ScheduleTime,
    /// The component `action`.
    pub action: crate::generated::iso802::ActionId,
    /// The component `affected_amount`.
    pub affected_amount: crate::generated::iso802::clause_6_3::DataReal,
    /// The component `comments`.
    pub comments: crate::generated::iso802::CommentString,
    /// The component `outage_effect`.
    pub outage_effect: crate::generated::iso802::CommentString,
}
impl crate::mms::MmsValue for DOActual {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.outage_reference_id),
            crate::mms::MmsValue::to_mms(&self.owning_utility_id),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.station_name),
            crate::mms::MmsValue::to_mms(&self.device_name),
            crate::mms::MmsValue::to_mms(&self.device_type),
            crate::mms::MmsValue::to_mms(&self.device_number),
            crate::mms::MmsValue::to_mms(&self.device_rating),
            crate::mms::MmsValue::to_mms(&self.activity_date_and_time),
            crate::mms::MmsValue::to_mms(&self.action),
            crate::mms::MmsValue::to_mms(&self.affected_amount),
            crate::mms::MmsValue::to_mms(&self.comments),
            crate::mms::MmsValue::to_mms(&self.outage_effect),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10, item11, item12] => Ok(Self { outage_reference_id: <crate::generated::iso802::clause_6_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, owning_utility_id: <crate::generated::iso802::clause_6_3::UtilityId as crate::mms::MmsValue>::from_mms(item1)?, time_stamp: <crate::generated::iso802::TimeStampS as crate::mms::MmsValue>::from_mms(item2)?, station_name: <crate::generated::iso802::StationNameString as crate::mms::MmsValue>::from_mms(item3)?, device_name: <crate::generated::iso802::DeviceName as crate::mms::MmsValue>::from_mms(item4)?, device_type: <crate::generated::iso802::DeviceId as crate::mms::MmsValue>::from_mms(item5)?, device_number: <crate::generated::iso802::clause_6_3::Number as crate::mms::MmsValue>::from_mms(item6)?, device_rating: <crate::generated::iso802::clause_6_3::DataReal as crate::mms::MmsValue>::from_mms(item7)?, activity_date_and_time: <crate::generated::iso802::clause_6_3::ScheduleTime as crate::mms::MmsValue>::from_mms(item8)?, action: <crate::generated::iso802::ActionId as crate::mms::MmsValue>::from_mms(item9)?, affected_amount: <crate::generated::iso802::clause_6_3::DataReal as crate::mms::MmsValue>::from_mms(item10)?, comments: <crate::generated::iso802::CommentString as crate::mms::MmsValue>::from_mms(item11)?, outage_effect: <crate::generated::iso802::CommentString as crate::mms::MmsValue>::from_mms(item12)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.4 (page 30). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct InfoMessHeader {
    /// The component `info_reference`.
    pub info_reference: crate::generated::iso802::clause_6_4::ReferenceNum,
    /// The component `localreference`.
    pub localreference: crate::generated::iso802::clause_6_4::ReferenceNum,
    /// The component `message_id`.
    pub message_id: crate::generated::iso802::clause_6_4::ReferenceNum,
    /// The component `size`.
    pub size: crate::generated::iso802::clause_6_4::Number,
}
impl crate::mms::MmsValue for InfoMessHeader {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.info_reference),
            crate::mms::MmsValue::to_mms(&self.localreference),
            crate::mms::MmsValue::to_mms(&self.message_id),
            crate::mms::MmsValue::to_mms(&self.size),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3] => Ok(Self { info_reference: <crate::generated::iso802::clause_6_4::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, localreference: <crate::generated::iso802::clause_6_4::ReferenceNum as crate::mms::MmsValue>::from_mms(item1)?, message_id: <crate::generated::iso802::clause_6_4::ReferenceNum as crate::mms::MmsValue>::from_mms(item2)?, size: <crate::generated::iso802::clause_6_4::Number as crate::mms::MmsValue>::from_mms(item3)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 6.4 (page 30). type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoBufXX(pub Vec<u8>);
impl InfoBufXX {
    /// The greatest number of octets, when the data file gives one.
    pub const MAX_LEN: Option<usize> = None;
}
impl crate::mms::MmsValue for InfoBufXX {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::OctetString(self.0.clone())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::OctetString(octets) => {
                if Self::MAX_LEN.is_some_and(|max| octets.len() > max) {
                    return Err(crate::mms::ValueError::Format(
                        "an octet string longer than its type",
                    ));
                }
                Ok(Self(octets.clone()))
            }
            other => Err(crate::mms::ValueError::kind_of("octet string", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommodityClass(i64);
impl CommodityClass {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for CommodityClass {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). reference definition.
pub type Duration = crate::generated::iso503::TimeIntervalS;
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerId(i64);
impl IntegerId {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for IntegerId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterchangeId(i64);
impl InterchangeId {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for InterchangeId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameString(String);
impl NameString {
    /// The greatest number of characters (a maximum: decision D-004).
    pub const MAX_LEN: usize = 32;
    /// Whether the length is variable (the data file says so).
    pub const VARYING: bool = true;
    /// The string, if it has at most the greatest number of visible characters (decision D-014).
    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {
        let text = text.into();
        crate::value::check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }
    /// The string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl crate::mms::MmsValue for NameString {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::VisibleString(self.as_str().to_string())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::VisibleString(text) => {
                Ok(Self::new(text.clone())?)
            }
            other => Err(crate::mms::ValueError::kind_of("visible string", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumIntegers(i64);
impl NumIntegers {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for NumIntegers {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumFloats(i64);
impl NumFloats {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for NumFloats {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumPeriods(i64);
impl NumPeriods {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for NumPeriods {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumProfiles(i64);
impl NumProfiles {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for NumProfiles {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumSegs(i64);
impl NumSegs {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for NumSegs {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransactionAmount(pub f64);
impl TransactionAmount {
    /// The format width, in bits.
    pub const FORMAT_WIDTH: u32 = 32;
    /// The exponent width, in bits.
    pub const EXPONENT_WIDTH: u32 = 8;
}
impl crate::mms::MmsValue for TransactionAmount {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let [a, b, c, d] = (self.0 as f32).to_be_bytes();
        igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
            igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransmissionSegCode(i64);
impl TransmissionSegCode {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for TransmissionSegCode {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 65). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TASegmentsPeriodic {
    /// The component `transfer_account_ref`.
    pub transfer_account_ref: crate::generated::iso802::clause_b_2::ReferenceNum,
    /// The component `send_utility`.
    pub send_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `recv_utility`.
    pub recv_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `selling_utility`.
    pub selling_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `buying_utility`.
    pub buying_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_b_2::DataTimeStamp,
    /// The component `transaction_code`.
    pub transaction_code: crate::generated::iso802::clause_b_2::TransactionCode,
    /// The component `name`.
    pub name: crate::generated::iso802::NameString,
    /// The component `start_time`.
    pub start_time: crate::generated::iso802::clause_b_2::ScheduleTime,
    /// The component `period_resolution`.
    pub period_resolution: crate::generated::iso503::TimeIntervalS,
    /// The component `number_local_references`.
    pub number_local_references: crate::generated::iso802::NumIntegers,
    /// The component `number_segments`.
    pub number_segments: crate::generated::iso802::NumSegs,
    /// The component `number_float_ids`.
    pub number_float_ids: crate::generated::iso802::NumFloats,
    /// The component `number_integer_ids`.
    pub number_integer_ids: crate::generated::iso802::NumIntegers,
    /// The component `number_periods`.
    pub number_periods: crate::generated::iso802::NumPeriods,
}
impl crate::mms::MmsValue for TASegmentsPeriodic {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.transfer_account_ref),
            crate::mms::MmsValue::to_mms(&self.send_utility),
            crate::mms::MmsValue::to_mms(&self.recv_utility),
            crate::mms::MmsValue::to_mms(&self.selling_utility),
            crate::mms::MmsValue::to_mms(&self.buying_utility),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.transaction_code),
            crate::mms::MmsValue::to_mms(&self.name),
            crate::mms::MmsValue::to_mms(&self.start_time),
            crate::mms::MmsValue::to_mms(&self.period_resolution),
            crate::mms::MmsValue::to_mms(&self.number_local_references),
            crate::mms::MmsValue::to_mms(&self.number_segments),
            crate::mms::MmsValue::to_mms(&self.number_float_ids),
            crate::mms::MmsValue::to_mms(&self.number_integer_ids),
            crate::mms::MmsValue::to_mms(&self.number_periods),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10, item11, item12, item13, item14] => Ok(Self { transfer_account_ref: <crate::generated::iso802::clause_b_2::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, send_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item1)?, recv_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item2)?, selling_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item3)?, buying_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item4)?, time_stamp: <crate::generated::iso802::clause_b_2::DataTimeStamp as crate::mms::MmsValue>::from_mms(item5)?, transaction_code: <crate::generated::iso802::clause_b_2::TransactionCode as crate::mms::MmsValue>::from_mms(item6)?, name: <crate::generated::iso802::NameString as crate::mms::MmsValue>::from_mms(item7)?, start_time: <crate::generated::iso802::clause_b_2::ScheduleTime as crate::mms::MmsValue>::from_mms(item8)?, period_resolution: <crate::generated::iso503::TimeIntervalS as crate::mms::MmsValue>::from_mms(item9)?, number_local_references: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item10)?, number_segments: <crate::generated::iso802::NumSegs as crate::mms::MmsValue>::from_mms(item11)?, number_float_ids: <crate::generated::iso802::NumFloats as crate::mms::MmsValue>::from_mms(item12)?, number_integer_ids: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item13)?, number_periods: <crate::generated::iso802::NumPeriods as crate::mms::MmsValue>::from_mms(item14)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 66). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TANoSegmentsPeriodic {
    /// The component `transfer_account_ref`.
    pub transfer_account_ref: crate::generated::iso802::clause_b_2::ReferenceNum,
    /// The component `send_utility`.
    pub send_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `recv_utility`.
    pub recv_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `selling_utility`.
    pub selling_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `buying_utility`.
    pub buying_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_b_2::DataTimeStamp,
    /// The component `transaction_code`.
    pub transaction_code: crate::generated::iso802::clause_b_2::TransactionCode,
    /// The component `name`.
    pub name: crate::generated::iso802::NameString,
    /// The component `start_time`.
    pub start_time: crate::generated::iso802::clause_b_2::ScheduleTime,
    /// The component `period_resolution`.
    pub period_resolution: crate::generated::iso503::TimeIntervalS,
    /// The component `number_local_references`.
    pub number_local_references: crate::generated::iso802::NumIntegers,
    /// The component `number_float_ids`.
    pub number_float_ids: crate::generated::iso802::NumFloats,
    /// The component `number_integer_ids`.
    pub number_integer_ids: crate::generated::iso802::NumIntegers,
    /// The component `number_periods`.
    pub number_periods: crate::generated::iso802::NumPeriods,
}
impl crate::mms::MmsValue for TANoSegmentsPeriodic {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.transfer_account_ref),
            crate::mms::MmsValue::to_mms(&self.send_utility),
            crate::mms::MmsValue::to_mms(&self.recv_utility),
            crate::mms::MmsValue::to_mms(&self.selling_utility),
            crate::mms::MmsValue::to_mms(&self.buying_utility),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.transaction_code),
            crate::mms::MmsValue::to_mms(&self.name),
            crate::mms::MmsValue::to_mms(&self.start_time),
            crate::mms::MmsValue::to_mms(&self.period_resolution),
            crate::mms::MmsValue::to_mms(&self.number_local_references),
            crate::mms::MmsValue::to_mms(&self.number_float_ids),
            crate::mms::MmsValue::to_mms(&self.number_integer_ids),
            crate::mms::MmsValue::to_mms(&self.number_periods),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10, item11, item12, item13] => Ok(Self { transfer_account_ref: <crate::generated::iso802::clause_b_2::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, send_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item1)?, recv_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item2)?, selling_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item3)?, buying_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item4)?, time_stamp: <crate::generated::iso802::clause_b_2::DataTimeStamp as crate::mms::MmsValue>::from_mms(item5)?, transaction_code: <crate::generated::iso802::clause_b_2::TransactionCode as crate::mms::MmsValue>::from_mms(item6)?, name: <crate::generated::iso802::NameString as crate::mms::MmsValue>::from_mms(item7)?, start_time: <crate::generated::iso802::clause_b_2::ScheduleTime as crate::mms::MmsValue>::from_mms(item8)?, period_resolution: <crate::generated::iso503::TimeIntervalS as crate::mms::MmsValue>::from_mms(item9)?, number_local_references: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item10)?, number_float_ids: <crate::generated::iso802::NumFloats as crate::mms::MmsValue>::from_mms(item11)?, number_integer_ids: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item12)?, number_periods: <crate::generated::iso802::NumPeriods as crate::mms::MmsValue>::from_mms(item13)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 66). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TASegmentsProfile {
    /// The component `transfer_account_ref`.
    pub transfer_account_ref: crate::generated::iso802::clause_b_2::ReferenceNum,
    /// The component `send_utility`.
    pub send_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `recv_utility`.
    pub recv_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `selling_utility`.
    pub selling_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `buying_utility`.
    pub buying_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_b_2::DataTimeStamp,
    /// The component `transaction_code`.
    pub transaction_code: crate::generated::iso802::clause_b_2::TransactionCode,
    /// The component `name`.
    pub name: crate::generated::iso802::NameString,
    /// The component `number_local_references`.
    pub number_local_references: crate::generated::iso802::NumIntegers,
    /// The component `number_segments`.
    pub number_segments: crate::generated::iso802::NumSegs,
    /// The component `number_profile_values`.
    pub number_profile_values: crate::generated::iso802::NumProfiles,
}
impl crate::mms::MmsValue for TASegmentsProfile {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.transfer_account_ref),
            crate::mms::MmsValue::to_mms(&self.send_utility),
            crate::mms::MmsValue::to_mms(&self.recv_utility),
            crate::mms::MmsValue::to_mms(&self.selling_utility),
            crate::mms::MmsValue::to_mms(&self.buying_utility),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.transaction_code),
            crate::mms::MmsValue::to_mms(&self.name),
            crate::mms::MmsValue::to_mms(&self.number_local_references),
            crate::mms::MmsValue::to_mms(&self.number_segments),
            crate::mms::MmsValue::to_mms(&self.number_profile_values),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10] => Ok(Self { transfer_account_ref: <crate::generated::iso802::clause_b_2::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, send_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item1)?, recv_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item2)?, selling_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item3)?, buying_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item4)?, time_stamp: <crate::generated::iso802::clause_b_2::DataTimeStamp as crate::mms::MmsValue>::from_mms(item5)?, transaction_code: <crate::generated::iso802::clause_b_2::TransactionCode as crate::mms::MmsValue>::from_mms(item6)?, name: <crate::generated::iso802::NameString as crate::mms::MmsValue>::from_mms(item7)?, number_local_references: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item8)?, number_segments: <crate::generated::iso802::NumSegs as crate::mms::MmsValue>::from_mms(item9)?, number_profile_values: <crate::generated::iso802::NumProfiles as crate::mms::MmsValue>::from_mms(item10)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 66). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TANoSegmentsProfile {
    /// The component `transfer_account_ref`.
    pub transfer_account_ref: crate::generated::iso802::clause_b_2::ReferenceNum,
    /// The component `send_utility`.
    pub send_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `recv_utility`.
    pub recv_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `selling_utility`.
    pub selling_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `buying_utility`.
    pub buying_utility: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::clause_b_2::DataTimeStamp,
    /// The component `transaction_code`.
    pub transaction_code: crate::generated::iso802::clause_b_2::TransactionCode,
    /// The component `name`.
    pub name: crate::generated::iso802::NameString,
    /// The component `number_local_references`.
    pub number_local_references: crate::generated::iso802::NumIntegers,
    /// The component `number_profile_values`.
    pub number_profile_values: crate::generated::iso802::NumProfiles,
}
impl crate::mms::MmsValue for TANoSegmentsProfile {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.transfer_account_ref),
            crate::mms::MmsValue::to_mms(&self.send_utility),
            crate::mms::MmsValue::to_mms(&self.recv_utility),
            crate::mms::MmsValue::to_mms(&self.selling_utility),
            crate::mms::MmsValue::to_mms(&self.buying_utility),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.transaction_code),
            crate::mms::MmsValue::to_mms(&self.name),
            crate::mms::MmsValue::to_mms(&self.number_local_references),
            crate::mms::MmsValue::to_mms(&self.number_profile_values),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9] => Ok(Self { transfer_account_ref: <crate::generated::iso802::clause_b_2::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, send_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item1)?, recv_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item2)?, selling_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item3)?, buying_utility: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item4)?, time_stamp: <crate::generated::iso802::clause_b_2::DataTimeStamp as crate::mms::MmsValue>::from_mms(item5)?, transaction_code: <crate::generated::iso802::clause_b_2::TransactionCode as crate::mms::MmsValue>::from_mms(item6)?, name: <crate::generated::iso802::NameString as crate::mms::MmsValue>::from_mms(item7)?, number_local_references: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item8)?, number_profile_values: <crate::generated::iso802::NumProfiles as crate::mms::MmsValue>::from_mms(item9)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 66). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TATransmissionSegment {
    /// The component `transmission_reference`.
    pub transmission_reference: crate::generated::iso802::clause_b_2::ReferenceNum,
    /// The component `util_wheeling`.
    pub util_wheeling: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `util_paying`.
    pub util_paying: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `transmission_seg_type`.
    pub transmission_seg_type: crate::generated::iso802::TransmissionSegCode,
    /// The component `util_in`.
    pub util_in: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `util_out`.
    pub util_out: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `interchange_pt_in`.
    pub interchange_pt_in: crate::generated::iso802::InterchangeId,
    /// The component `interchange_pt_out`.
    pub interchange_pt_out: crate::generated::iso802::InterchangeId,
    /// The component `interchange_pt`.
    pub interchange_pt: crate::generated::iso802::InterchangeId,
    /// The component `number_float_ids`.
    pub number_float_ids: crate::generated::iso802::NumFloats,
    /// The component `number_integer_ids`.
    pub number_integer_ids: crate::generated::iso802::NumIntegers,
}
impl crate::mms::MmsValue for TATransmissionSegment {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.transmission_reference),
            crate::mms::MmsValue::to_mms(&self.util_wheeling),
            crate::mms::MmsValue::to_mms(&self.util_paying),
            crate::mms::MmsValue::to_mms(&self.transmission_seg_type),
            crate::mms::MmsValue::to_mms(&self.util_in),
            crate::mms::MmsValue::to_mms(&self.util_out),
            crate::mms::MmsValue::to_mms(&self.interchange_pt_in),
            crate::mms::MmsValue::to_mms(&self.interchange_pt_out),
            crate::mms::MmsValue::to_mms(&self.interchange_pt),
            crate::mms::MmsValue::to_mms(&self.number_float_ids),
            crate::mms::MmsValue::to_mms(&self.number_integer_ids),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10] => Ok(Self { transmission_reference: <crate::generated::iso802::clause_b_2::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, util_wheeling: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item1)?, util_paying: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item2)?, transmission_seg_type: <crate::generated::iso802::TransmissionSegCode as crate::mms::MmsValue>::from_mms(item3)?, util_in: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item4)?, util_out: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item5)?, interchange_pt_in: <crate::generated::iso802::InterchangeId as crate::mms::MmsValue>::from_mms(item6)?, interchange_pt_out: <crate::generated::iso802::InterchangeId as crate::mms::MmsValue>::from_mms(item7)?, interchange_pt: <crate::generated::iso802::InterchangeId as crate::mms::MmsValue>::from_mms(item8)?, number_float_ids: <crate::generated::iso802::NumFloats as crate::mms::MmsValue>::from_mms(item9)?, number_integer_ids: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item10)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 67). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TATransmissionSegmentProfile {
    /// The component `transmission_reference`.
    pub transmission_reference: crate::generated::iso802::clause_b_2::ReferenceNum,
    /// The component `util_wheeling`.
    pub util_wheeling: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `util_paying`.
    pub util_paying: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `transmission_seg_type`.
    pub transmission_seg_type: crate::generated::iso802::TransmissionSegCode,
    /// The component `util_in`.
    pub util_in: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `util_out`.
    pub util_out: crate::generated::iso802::clause_b_2::UtilityId,
    /// The component `interchange_pt_in`.
    pub interchange_pt_in: crate::generated::iso802::InterchangeId,
    /// The component `interchange_pt_out`.
    pub interchange_pt_out: crate::generated::iso802::InterchangeId,
    /// The component `interchange_pt`.
    pub interchange_pt: crate::generated::iso802::InterchangeId,
    /// The component `number_profile_values`.
    pub number_profile_values: crate::generated::iso802::NumProfiles,
}
impl crate::mms::MmsValue for TATransmissionSegmentProfile {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.transmission_reference),
            crate::mms::MmsValue::to_mms(&self.util_wheeling),
            crate::mms::MmsValue::to_mms(&self.util_paying),
            crate::mms::MmsValue::to_mms(&self.transmission_seg_type),
            crate::mms::MmsValue::to_mms(&self.util_in),
            crate::mms::MmsValue::to_mms(&self.util_out),
            crate::mms::MmsValue::to_mms(&self.interchange_pt_in),
            crate::mms::MmsValue::to_mms(&self.interchange_pt_out),
            crate::mms::MmsValue::to_mms(&self.interchange_pt),
            crate::mms::MmsValue::to_mms(&self.number_profile_values),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9] => Ok(Self { transmission_reference: <crate::generated::iso802::clause_b_2::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, util_wheeling: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item1)?, util_paying: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item2)?, transmission_seg_type: <crate::generated::iso802::TransmissionSegCode as crate::mms::MmsValue>::from_mms(item3)?, util_in: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item4)?, util_out: <crate::generated::iso802::clause_b_2::UtilityId as crate::mms::MmsValue>::from_mms(item5)?, interchange_pt_in: <crate::generated::iso802::InterchangeId as crate::mms::MmsValue>::from_mms(item6)?, interchange_pt_out: <crate::generated::iso802::InterchangeId as crate::mms::MmsValue>::from_mms(item7)?, interchange_pt: <crate::generated::iso802::InterchangeId as crate::mms::MmsValue>::from_mms(item8)?, number_profile_values: <crate::generated::iso802::NumProfiles as crate::mms::MmsValue>::from_mms(item9)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 67). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TAProfileValue {
    /// The component `ramp_start_time`.
    pub ramp_start_time: crate::generated::iso802::clause_b_2::ScheduleTime,
    /// The component `ramp_duration`.
    pub ramp_duration: crate::generated::iso802::Duration,
    /// The component `profile_price`.
    pub profile_price: crate::generated::iso802::clause_b_2::Price,
    /// The component `profile_target_class`.
    pub profile_target_class: crate::generated::iso802::CommodityClass,
    /// The component `profile_target`.
    pub profile_target: crate::generated::iso802::TransactionAmount,
}
impl crate::mms::MmsValue for TAProfileValue {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.ramp_start_time),
            crate::mms::MmsValue::to_mms(&self.ramp_duration),
            crate::mms::MmsValue::to_mms(&self.profile_price),
            crate::mms::MmsValue::to_mms(&self.profile_target_class),
            crate::mms::MmsValue::to_mms(&self.profile_target),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4] => Ok(Self { ramp_start_time: <crate::generated::iso802::clause_b_2::ScheduleTime as crate::mms::MmsValue>::from_mms(item0)?, ramp_duration: <crate::generated::iso802::Duration as crate::mms::MmsValue>::from_mms(item1)?, profile_price: <crate::generated::iso802::clause_b_2::Price as crate::mms::MmsValue>::from_mms(item2)?, profile_target_class: <crate::generated::iso802::CommodityClass as crate::mms::MmsValue>::from_mms(item3)?, profile_target: <crate::generated::iso802::TransactionAmount as crate::mms::MmsValue>::from_mms(item4)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.2 (page 67). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct AccountRequest {
    /// The component `reference_tar`.
    pub reference_tar: crate::generated::iso802::clause_b_2::ReferenceNum,
    /// The component `start_time`.
    pub start_time: crate::generated::iso802::clause_b_2::ScheduleTime,
    /// The component `duration`.
    pub duration: crate::generated::iso802::Duration,
    /// The component `request_id`.
    pub request_id: crate::generated::iso802::clause_b_2::ReferenceNum,
    /// The component `ta_conditions_requested`.
    pub ta_conditions_requested: crate::generated::iso503::TAConditions,
}
impl crate::mms::MmsValue for AccountRequest {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.reference_tar),
            crate::mms::MmsValue::to_mms(&self.start_time),
            crate::mms::MmsValue::to_mms(&self.duration),
            crate::mms::MmsValue::to_mms(&self.request_id),
            crate::mms::MmsValue::to_mms(&self.ta_conditions_requested),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4] => Ok(Self { reference_tar: <crate::generated::iso802::clause_b_2::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, start_time: <crate::generated::iso802::clause_b_2::ScheduleTime as crate::mms::MmsValue>::from_mms(item1)?, duration: <crate::generated::iso802::Duration as crate::mms::MmsValue>::from_mms(item2)?, request_id: <crate::generated::iso802::clause_b_2::ReferenceNum as crate::mms::MmsValue>::from_mms(item3)?, ta_conditions_requested: <crate::generated::iso503::TAConditions as crate::mms::MmsValue>::from_mms(item4)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 67). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvailReasonCode(i64);
impl AvailReasonCode {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for AvailReasonCode {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 67). type definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capacity(pub f64);
impl Capacity {
    /// The format width, in bits.
    pub const FORMAT_WIDTH: u32 = 32;
    /// The exponent width, in bits.
    pub const EXPONENT_WIDTH: u32 = 8;
}
impl crate::mms::MmsValue for Capacity {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let [a, b, c, d] = (self.0 as f32).to_be_bytes();
        igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
            igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 67). reference definition.
pub type DateAndTime = crate::generated::iso503::GMTBasedS;
/// IEC 60870-6-802 B.3 (page 67). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LFCReasonCode(i64);
impl LFCReasonCode {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for LFCReasonCode {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 67). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlantId(i64);
impl PlantId {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for PlantId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). type definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RampRate(pub f64);
impl RampRate {
    /// The format width, in bits.
    pub const FORMAT_WIDTH: u32 = 32;
    /// The exponent width, in bits.
    pub const EXPONENT_WIDTH: u32 = 8;
}
impl crate::mms::MmsValue for RampRate {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let [a, b, c, d] = (self.0 as f32).to_be_bytes();
        igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
            igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportStatus(i64);
impl ReportStatus {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for ReportStatus {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitId(i64);
impl UnitId {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for UnitId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitCapacityCapacityClass(pub u64);
impl UnitCapacityCapacityClass {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 2;
    /// The mask of the flag `gross`.
    pub const GROSS: u64 = 0x1;
    /// The mask of the flag `net`.
    pub const NET: u64 = 0x2;
}
impl UnitCapacityCapacityClass {
    /// Whether the flag `gross` is set.
    pub fn is_gross(self) -> bool {
        self.0 & Self::GROSS != 0
    }
    /// Whether the flag `net` is set.
    pub fn is_net(self) -> bool {
        self.0 & Self::NET != 0
    }
}
impl crate::mms::MmsValue for UnitCapacityCapacityClass {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct UnitCapacity {
    /// The component `capacity_class`.
    pub capacity_class: UnitCapacityCapacityClass,
    /// The component `gross_max_capacity`.
    pub gross_max_capacity: crate::generated::iso802::Capacity,
    /// The component `gross_min_capacity`.
    pub gross_min_capacity: crate::generated::iso802::Capacity,
    /// The component `net_max_capacity`.
    pub net_max_capacity: crate::generated::iso802::Capacity,
    /// The component `net_min_capacity`.
    pub net_min_capacity: crate::generated::iso802::Capacity,
}
impl crate::mms::MmsValue for UnitCapacity {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.capacity_class),
            crate::mms::MmsValue::to_mms(&self.gross_max_capacity),
            crate::mms::MmsValue::to_mms(&self.gross_min_capacity),
            crate::mms::MmsValue::to_mms(&self.net_max_capacity),
            crate::mms::MmsValue::to_mms(&self.net_min_capacity),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => {
                match items.as_slice() {
                    [item0, item1, item2, item3, item4] => Ok(Self {
                        capacity_class:
                            <UnitCapacityCapacityClass as crate::mms::MmsValue>::from_mms(item0)?,
                        gross_max_capacity:
                            <crate::generated::iso802::Capacity as crate::mms::MmsValue>::from_mms(
                                item1,
                            )?,
                        gross_min_capacity:
                            <crate::generated::iso802::Capacity as crate::mms::MmsValue>::from_mms(
                                item2,
                            )?,
                        net_max_capacity:
                            <crate::generated::iso802::Capacity as crate::mms::MmsValue>::from_mms(
                                item3,
                            )?,
                        net_min_capacity:
                            <crate::generated::iso802::Capacity as crate::mms::MmsValue>::from_mms(
                                item4,
                            )?,
                    }),
                    _ => Err(crate::mms::ValueError::Format(
                        "a structure of another number of components",
                    )),
                }
            }
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct RampRates {
    /// The component `max_ramp_rate_up`.
    pub max_ramp_rate_up: crate::generated::iso802::RampRate,
    /// The component `max_ramp_rate_down`.
    pub max_ramp_rate_down: crate::generated::iso802::RampRate,
}
impl crate::mms::MmsValue for RampRates {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.max_ramp_rate_up),
            crate::mms::MmsValue::to_mms(&self.max_ramp_rate_down),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => {
                match items.as_slice() {
                    [item0, item1] => Ok(Self {
                        max_ramp_rate_up:
                            <crate::generated::iso802::RampRate as crate::mms::MmsValue>::from_mms(
                                item0,
                            )?,
                        max_ramp_rate_down:
                            <crate::generated::iso802::RampRate as crate::mms::MmsValue>::from_mms(
                                item1,
                            )?,
                    }),
                    _ => Err(crate::mms::ValueError::Format(
                        "a structure of another number of components",
                    )),
                }
            }
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvailabilityClassAvailFlags(pub u64);
impl AvailabilityClassAvailFlags {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 8;
    /// The mask of the flag `online`.
    pub const ONLINE: u64 = 0x1;
    /// The mask of the flag `LFC`.
    pub const LFC: u64 = 0x2;
    /// The mask of the flag `dispatch`.
    pub const DISPATCH: u64 = 0x4;
    /// The mask of the flag `regulating`.
    pub const REGULATING: u64 = 0x8;
    /// The mask of the flag `manual`.
    pub const MANUAL: u64 = 0x10;
    /// The mask of the flag `reserve`.
    pub const RESERVE: u64 = 0x20;
    /// The mask of the flag `blocked_hi`.
    pub const BLOCKED_HI: u64 = 0x40;
    /// The mask of the flag `blocked_lo`.
    pub const BLOCKED_LO: u64 = 0x80;
}
impl AvailabilityClassAvailFlags {
    /// Whether the flag `online` is set.
    pub fn is_online(self) -> bool {
        self.0 & Self::ONLINE != 0
    }
    /// Whether the flag `LFC` is set.
    pub fn is_lfc(self) -> bool {
        self.0 & Self::LFC != 0
    }
    /// Whether the flag `dispatch` is set.
    pub fn is_dispatch(self) -> bool {
        self.0 & Self::DISPATCH != 0
    }
    /// Whether the flag `regulating` is set.
    pub fn is_regulating(self) -> bool {
        self.0 & Self::REGULATING != 0
    }
    /// Whether the flag `manual` is set.
    pub fn is_manual(self) -> bool {
        self.0 & Self::MANUAL != 0
    }
    /// Whether the flag `reserve` is set.
    pub fn is_reserve(self) -> bool {
        self.0 & Self::RESERVE != 0
    }
    /// Whether the flag `blocked_hi` is set.
    pub fn is_blocked_hi(self) -> bool {
        self.0 & Self::BLOCKED_HI != 0
    }
    /// Whether the flag `blocked_lo` is set.
    pub fn is_blocked_lo(self) -> bool {
        self.0 & Self::BLOCKED_LO != 0
    }
}
impl crate::mms::MmsValue for AvailabilityClassAvailFlags {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct AvailabilityClass {
    /// The component `avail_flags`.
    pub avail_flags: AvailabilityClassAvailFlags,
    /// The component `time_to_online`.
    pub time_to_online: crate::generated::iso503::GMTBasedS,
    /// The component `reason_for_no_lfc`.
    pub reason_for_no_lfc: crate::generated::iso802::LFCReasonCode,
}
impl crate::mms::MmsValue for AvailabilityClass {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.avail_flags),
            crate::mms::MmsValue::to_mms(&self.time_to_online),
            crate::mms::MmsValue::to_mms(&self.reason_for_no_lfc),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items
                .as_slice()
            {
                [item0, item1, item2] => Ok(Self {
                    avail_flags: <AvailabilityClassAvailFlags as crate::mms::MmsValue>::from_mms(
                        item0,
                    )?,
                    time_to_online:
                        <crate::generated::iso503::GMTBasedS as crate::mms::MmsValue>::from_mms(
                            item1,
                        )?,
                    reason_for_no_lfc:
                        <crate::generated::iso802::LFCReasonCode as crate::mms::MmsValue>::from_mms(
                            item2,
                        )?,
                }),
                _ => Err(crate::mms::ValueError::Format(
                    "a structure of another number of components",
                )),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvailableProvidingReserve(pub bool);
impl crate::mms::MmsValue for AvailableProvidingReserve {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Boolean(self.0)
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Boolean(value) => Ok(Self(*value)),
            other => Err(crate::mms::ValueError::kind_of("boolean", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvailableImpact(pub u64);
impl AvailableImpact {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 3;
    /// The mask of the flag `price`.
    pub const PRICE: u64 = 0x1;
    /// The mask of the flag `ramp`.
    pub const RAMP: u64 = 0x2;
    /// The mask of the flag `capacity`.
    pub const CAPACITY: u64 = 0x4;
}
impl AvailableImpact {
    /// Whether the flag `price` is set.
    pub fn is_price(self) -> bool {
        self.0 & Self::PRICE != 0
    }
    /// Whether the flag `ramp` is set.
    pub fn is_ramp(self) -> bool {
        self.0 & Self::RAMP != 0
    }
    /// Whether the flag `capacity` is set.
    pub fn is_capacity(self) -> bool {
        self.0 & Self::CAPACITY != 0
    }
}
impl crate::mms::MmsValue for AvailableImpact {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). inline definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailableComment(String);
impl AvailableComment {
    /// The greatest number of characters (a maximum: decision D-004).
    pub const MAX_LEN: usize = 255;
    /// Whether the length is variable (the data file says so).
    pub const VARYING: bool = false;
    /// The string, if it has at most the greatest number of visible characters (decision D-014).
    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {
        let text = text.into();
        crate::value::check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }
    /// The string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl crate::mms::MmsValue for AvailableComment {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::VisibleString(self.as_str().to_string())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::VisibleString(text) => {
                Ok(Self::new(text.clone())?)
            }
            other => Err(crate::mms::ValueError::kind_of("visible string", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 68). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Available {
    /// The component `availability_reference_id`.
    pub availability_reference_id: crate::generated::iso802::clause_b_3::ReferenceNum,
    /// The component `plant_reference_id`.
    pub plant_reference_id: crate::generated::iso802::PlantId,
    /// The component `unit_id`.
    pub unit_id: crate::generated::iso802::UnitId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::TimeStampS,
    /// The component `start_date_and_time`.
    pub start_date_and_time: crate::generated::iso802::DateAndTime,
    /// The component `end_date_and_time`.
    pub end_date_and_time: crate::generated::iso802::DateAndTime,
    /// The component `duration`.
    pub duration: crate::generated::iso503::TimeIntervalM,
    /// The component `providing_reserve`.
    pub providing_reserve: AvailableProvidingReserve,
    /// The component `report_status`.
    pub report_status: crate::generated::iso802::ReportStatus,
    /// The component `impact`.
    pub impact: AvailableImpact,
    /// The component `price_impact`.
    pub price_impact: crate::generated::iso802::clause_b_3::Price,
    /// The component `ramp_impact`.
    pub ramp_impact: crate::generated::iso802::RampRates,
    /// The component `capacity_impact`.
    pub capacity_impact: crate::generated::iso802::UnitCapacity,
    /// The component `availability`.
    pub availability: crate::generated::iso802::AvailabilityClass,
    /// The component `comment`.
    pub comment: AvailableComment,
}
impl crate::mms::MmsValue for Available {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.availability_reference_id),
            crate::mms::MmsValue::to_mms(&self.plant_reference_id),
            crate::mms::MmsValue::to_mms(&self.unit_id),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.start_date_and_time),
            crate::mms::MmsValue::to_mms(&self.end_date_and_time),
            crate::mms::MmsValue::to_mms(&self.duration),
            crate::mms::MmsValue::to_mms(&self.providing_reserve),
            crate::mms::MmsValue::to_mms(&self.report_status),
            crate::mms::MmsValue::to_mms(&self.impact),
            crate::mms::MmsValue::to_mms(&self.price_impact),
            crate::mms::MmsValue::to_mms(&self.ramp_impact),
            crate::mms::MmsValue::to_mms(&self.capacity_impact),
            crate::mms::MmsValue::to_mms(&self.availability),
            crate::mms::MmsValue::to_mms(&self.comment),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10, item11, item12, item13, item14] => Ok(Self { availability_reference_id: <crate::generated::iso802::clause_b_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, plant_reference_id: <crate::generated::iso802::PlantId as crate::mms::MmsValue>::from_mms(item1)?, unit_id: <crate::generated::iso802::UnitId as crate::mms::MmsValue>::from_mms(item2)?, time_stamp: <crate::generated::iso802::TimeStampS as crate::mms::MmsValue>::from_mms(item3)?, start_date_and_time: <crate::generated::iso802::DateAndTime as crate::mms::MmsValue>::from_mms(item4)?, end_date_and_time: <crate::generated::iso802::DateAndTime as crate::mms::MmsValue>::from_mms(item5)?, duration: <crate::generated::iso503::TimeIntervalM as crate::mms::MmsValue>::from_mms(item6)?, providing_reserve: <AvailableProvidingReserve as crate::mms::MmsValue>::from_mms(item7)?, report_status: <crate::generated::iso802::ReportStatus as crate::mms::MmsValue>::from_mms(item8)?, impact: <AvailableImpact as crate::mms::MmsValue>::from_mms(item9)?, price_impact: <crate::generated::iso802::clause_b_3::Price as crate::mms::MmsValue>::from_mms(item10)?, ramp_impact: <crate::generated::iso802::RampRates as crate::mms::MmsValue>::from_mms(item11)?, capacity_impact: <crate::generated::iso802::UnitCapacity as crate::mms::MmsValue>::from_mms(item12)?, availability: <crate::generated::iso802::AvailabilityClass as crate::mms::MmsValue>::from_mms(item13)?, comment: <AvailableComment as crate::mms::MmsValue>::from_mms(item14)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 69). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnAvailableProvidingReserve(pub bool);
impl crate::mms::MmsValue for UnAvailableProvidingReserve {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Boolean(self.0)
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Boolean(value) => Ok(Self(*value)),
            other => Err(crate::mms::ValueError::kind_of("boolean", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 69). inline definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnAvailableComment(String);
impl UnAvailableComment {
    /// The greatest number of characters (a maximum: decision D-004).
    pub const MAX_LEN: usize = 255;
    /// Whether the length is variable (the data file says so).
    pub const VARYING: bool = false;
    /// The string, if it has at most the greatest number of visible characters (decision D-014).
    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {
        let text = text.into();
        crate::value::check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }
    /// The string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl crate::mms::MmsValue for UnAvailableComment {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::VisibleString(self.as_str().to_string())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::VisibleString(text) => {
                Ok(Self::new(text.clone())?)
            }
            other => Err(crate::mms::ValueError::kind_of("visible string", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 69). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct UnAvailable {
    /// The component `availability_reference_id`.
    pub availability_reference_id: crate::generated::iso802::clause_b_3::ReferenceNum,
    /// The component `plant_reference_id`.
    pub plant_reference_id: crate::generated::iso802::PlantId,
    /// The component `unit_id`.
    pub unit_id: crate::generated::iso802::UnitId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::TimeStampS,
    /// The component `start_date_and_time`.
    pub start_date_and_time: crate::generated::iso802::DateAndTime,
    /// The component `end_date_and_time`.
    pub end_date_and_time: crate::generated::iso802::DateAndTime,
    /// The component `duration`.
    pub duration: crate::generated::iso503::TimeIntervalM,
    /// The component `providing_reserve`.
    pub providing_reserve: UnAvailableProvidingReserve,
    /// The component `report_status`.
    pub report_status: crate::generated::iso802::ReportStatus,
    /// The component `reason_for_unavailable`.
    pub reason_for_unavailable: crate::generated::iso802::AvailReasonCode,
    /// The component `comment`.
    pub comment: UnAvailableComment,
}
impl crate::mms::MmsValue for UnAvailable {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.availability_reference_id),
            crate::mms::MmsValue::to_mms(&self.plant_reference_id),
            crate::mms::MmsValue::to_mms(&self.unit_id),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.start_date_and_time),
            crate::mms::MmsValue::to_mms(&self.end_date_and_time),
            crate::mms::MmsValue::to_mms(&self.duration),
            crate::mms::MmsValue::to_mms(&self.providing_reserve),
            crate::mms::MmsValue::to_mms(&self.report_status),
            crate::mms::MmsValue::to_mms(&self.reason_for_unavailable),
            crate::mms::MmsValue::to_mms(&self.comment),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10] => Ok(Self { availability_reference_id: <crate::generated::iso802::clause_b_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, plant_reference_id: <crate::generated::iso802::PlantId as crate::mms::MmsValue>::from_mms(item1)?, unit_id: <crate::generated::iso802::UnitId as crate::mms::MmsValue>::from_mms(item2)?, time_stamp: <crate::generated::iso802::TimeStampS as crate::mms::MmsValue>::from_mms(item3)?, start_date_and_time: <crate::generated::iso802::DateAndTime as crate::mms::MmsValue>::from_mms(item4)?, end_date_and_time: <crate::generated::iso802::DateAndTime as crate::mms::MmsValue>::from_mms(item5)?, duration: <crate::generated::iso503::TimeIntervalM as crate::mms::MmsValue>::from_mms(item6)?, providing_reserve: <UnAvailableProvidingReserve as crate::mms::MmsValue>::from_mms(item7)?, report_status: <crate::generated::iso802::ReportStatus as crate::mms::MmsValue>::from_mms(item8)?, reason_for_unavailable: <crate::generated::iso802::AvailReasonCode as crate::mms::MmsValue>::from_mms(item9)?, comment: <UnAvailableComment as crate::mms::MmsValue>::from_mms(item10)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 69). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusAvailableProvidingReserve(pub bool);
impl crate::mms::MmsValue for StatusAvailableProvidingReserve {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Boolean(self.0)
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Boolean(value) => Ok(Self(*value)),
            other => Err(crate::mms::ValueError::kind_of("boolean", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 69). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct StatusAvailable {
    /// The component `availability_reference_id`.
    pub availability_reference_id: crate::generated::iso802::clause_b_3::ReferenceNum,
    /// The component `plant_reference_id`.
    pub plant_reference_id: crate::generated::iso802::PlantId,
    /// The component `unit_id`.
    pub unit_id: crate::generated::iso802::UnitId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::TimeStampS,
    /// The component `ramp_status`.
    pub ramp_status: crate::generated::iso802::RampRates,
    /// The component `capacity_status`.
    pub capacity_status: crate::generated::iso802::UnitCapacity,
    /// The component `availability_status`.
    pub availability_status: crate::generated::iso802::AvailabilityClass,
    /// The component `providing_reserve`.
    pub providing_reserve: StatusAvailableProvidingReserve,
}
impl crate::mms::MmsValue for StatusAvailable {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.availability_reference_id),
            crate::mms::MmsValue::to_mms(&self.plant_reference_id),
            crate::mms::MmsValue::to_mms(&self.unit_id),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.ramp_status),
            crate::mms::MmsValue::to_mms(&self.capacity_status),
            crate::mms::MmsValue::to_mms(&self.availability_status),
            crate::mms::MmsValue::to_mms(&self.providing_reserve),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7] => Ok(Self { availability_reference_id: <crate::generated::iso802::clause_b_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, plant_reference_id: <crate::generated::iso802::PlantId as crate::mms::MmsValue>::from_mms(item1)?, unit_id: <crate::generated::iso802::UnitId as crate::mms::MmsValue>::from_mms(item2)?, time_stamp: <crate::generated::iso802::TimeStampS as crate::mms::MmsValue>::from_mms(item3)?, ramp_status: <crate::generated::iso802::RampRates as crate::mms::MmsValue>::from_mms(item4)?, capacity_status: <crate::generated::iso802::UnitCapacity as crate::mms::MmsValue>::from_mms(item5)?, availability_status: <crate::generated::iso802::AvailabilityClass as crate::mms::MmsValue>::from_mms(item6)?, providing_reserve: <StatusAvailableProvidingReserve as crate::mms::MmsValue>::from_mms(item7)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 69). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusUnAvailableProvidingReserve(pub bool);
impl crate::mms::MmsValue for StatusUnAvailableProvidingReserve {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Boolean(self.0)
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Boolean(value) => Ok(Self(*value)),
            other => Err(crate::mms::ValueError::kind_of("boolean", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 69). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct StatusUnAvailable {
    /// The component `availability_reference_id`.
    pub availability_reference_id: crate::generated::iso802::clause_b_3::ReferenceNum,
    /// The component `plant_reference_id`.
    pub plant_reference_id: crate::generated::iso802::PlantId,
    /// The component `unit_id`.
    pub unit_id: crate::generated::iso802::UnitId,
    /// The component `time_stamp`.
    pub time_stamp: crate::generated::iso802::TimeStampS,
    /// The component `reason_for_unavailable`.
    pub reason_for_unavailable: crate::generated::iso802::AvailReasonCode,
    /// The component `providing_reserve`.
    pub providing_reserve: StatusUnAvailableProvidingReserve,
}
impl crate::mms::MmsValue for StatusUnAvailable {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.availability_reference_id),
            crate::mms::MmsValue::to_mms(&self.plant_reference_id),
            crate::mms::MmsValue::to_mms(&self.unit_id),
            crate::mms::MmsValue::to_mms(&self.time_stamp),
            crate::mms::MmsValue::to_mms(&self.reason_for_unavailable),
            crate::mms::MmsValue::to_mms(&self.providing_reserve),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5] => Ok(Self { availability_reference_id: <crate::generated::iso802::clause_b_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, plant_reference_id: <crate::generated::iso802::PlantId as crate::mms::MmsValue>::from_mms(item1)?, unit_id: <crate::generated::iso802::UnitId as crate::mms::MmsValue>::from_mms(item2)?, time_stamp: <crate::generated::iso802::TimeStampS as crate::mms::MmsValue>::from_mms(item3)?, reason_for_unavailable: <crate::generated::iso802::AvailReasonCode as crate::mms::MmsValue>::from_mms(item4)?, providing_reserve: <StatusUnAvailableProvidingReserve as crate::mms::MmsValue>::from_mms(item5)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 69). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForecastForecastType(pub u64);
impl ForecastForecastType {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 2;
    /// The mask of the flag `generation`.
    pub const GENERATION: u64 = 0x1;
    /// The mask of the flag `reserve`.
    pub const RESERVE: u64 = 0x2;
}
impl ForecastForecastType {
    /// Whether the flag `generation` is set.
    pub fn is_generation(self) -> bool {
        self.0 & Self::GENERATION != 0
    }
    /// Whether the flag `reserve` is set.
    pub fn is_reserve(self) -> bool {
        self.0 & Self::RESERVE != 0
    }
}
impl crate::mms::MmsValue for ForecastForecastType {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 69). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Forecast {
    /// The component `forecast_schedule_reference_id`.
    pub forecast_schedule_reference_id: crate::generated::iso802::clause_b_3::ReferenceNum,
    /// The component `plant_reference_id`.
    pub plant_reference_id: crate::generated::iso802::PlantId,
    /// The component `unit_id`.
    pub unit_id: crate::generated::iso802::UnitId,
    /// The component `start_time`.
    pub start_time: crate::generated::iso503::GMTBasedS,
    /// The component `period_resolution`.
    pub period_resolution: crate::generated::iso503::TimeIntervalS,
    /// The component `num_periods`.
    pub num_periods: crate::builtin::Integer32,
    /// The component `forecast_type`.
    pub forecast_type: ForecastForecastType,
}
impl crate::mms::MmsValue for Forecast {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.forecast_schedule_reference_id),
            crate::mms::MmsValue::to_mms(&self.plant_reference_id),
            crate::mms::MmsValue::to_mms(&self.unit_id),
            crate::mms::MmsValue::to_mms(&self.start_time),
            crate::mms::MmsValue::to_mms(&self.period_resolution),
            crate::mms::MmsValue::to_mms(&self.num_periods),
            crate::mms::MmsValue::to_mms(&self.forecast_type),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6] => Ok(Self { forecast_schedule_reference_id: <crate::generated::iso802::clause_b_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, plant_reference_id: <crate::generated::iso802::PlantId as crate::mms::MmsValue>::from_mms(item1)?, unit_id: <crate::generated::iso802::UnitId as crate::mms::MmsValue>::from_mms(item2)?, start_time: <crate::generated::iso503::GMTBasedS as crate::mms::MmsValue>::from_mms(item3)?, period_resolution: <crate::generated::iso503::TimeIntervalS as crate::mms::MmsValue>::from_mms(item4)?, num_periods: <crate::builtin::Integer32 as crate::mms::MmsValue>::from_mms(item5)?, forecast_type: <ForecastForecastType as crate::mms::MmsValue>::from_mms(item6)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 70). type definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveLimit(pub f64);
impl CurveLimit {
    /// The format width, in bits.
    pub const FORMAT_WIDTH: u32 = 32;
    /// The exponent width, in bits.
    pub const EXPONENT_WIDTH: u32 = 8;
}
impl crate::mms::MmsValue for CurveLimit {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let [a, b, c, d] = (self.0 as f32).to_be_bytes();
        igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
            igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 70). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurveType(pub u64);
impl CurveType {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 17;
    /// The mask of the flag `HEAT_RATE`.
    pub const HEAT_RATE: u64 = 0x1;
    /// The mask of the flag `IO`.
    pub const IO: u64 = 0x2;
    /// The mask of the flag `IHR`.
    pub const IHR: u64 = 0x4;
    /// The mask of the flag `MVAR_CAP`.
    pub const MVAR_CAP: u64 = 0x8;
    /// The mask of the flag `COST`.
    pub const COST: u64 = 0x10;
    /// The mask of the flag `OPACITY`.
    pub const OPACITY: u64 = 0x20;
    /// The mask of the flag `SOX`.
    pub const SOX: u64 = 0x40;
    /// The mask of the flag `NOX`.
    pub const NOX: u64 = 0x80;
    /// The mask of the flag `CO2`.
    pub const CO2: u64 = 0x100;
    /// The mask of the flag `Unused`.
    pub const UNUSED: u64 = 0x200;
    /// The mask of the flag `User defined`.
    pub const USER_DEFINED: u64 = 0x10000;
}
impl CurveType {
    /// Whether the flag `HEAT_RATE` is set.
    pub fn is_heat_rate(self) -> bool {
        self.0 & Self::HEAT_RATE != 0
    }
    /// Whether the flag `IO` is set.
    pub fn is_io(self) -> bool {
        self.0 & Self::IO != 0
    }
    /// Whether the flag `IHR` is set.
    pub fn is_ihr(self) -> bool {
        self.0 & Self::IHR != 0
    }
    /// Whether the flag `MVAR_CAP` is set.
    pub fn is_mvar_cap(self) -> bool {
        self.0 & Self::MVAR_CAP != 0
    }
    /// Whether the flag `COST` is set.
    pub fn is_cost(self) -> bool {
        self.0 & Self::COST != 0
    }
    /// Whether the flag `OPACITY` is set.
    pub fn is_opacity(self) -> bool {
        self.0 & Self::OPACITY != 0
    }
    /// Whether the flag `SOX` is set.
    pub fn is_sox(self) -> bool {
        self.0 & Self::SOX != 0
    }
    /// Whether the flag `NOX` is set.
    pub fn is_nox(self) -> bool {
        self.0 & Self::NOX != 0
    }
    /// Whether the flag `CO2` is set.
    pub fn is_co2(self) -> bool {
        self.0 & Self::CO2 != 0
    }
    /// Whether the flag `Unused` is set.
    pub fn is_unused(self) -> bool {
        self.0 & Self::UNUSED != 0
    }
    /// Whether the flag `User defined` is set.
    pub fn is_user_defined(self) -> bool {
        self.0 & Self::USER_DEFINED != 0
    }
}
impl crate::mms::MmsValue for CurveType {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let octets = (Self::BIT_COUNT as usize).div_ceil(8);
        let mut bytes = vec![0u8; octets];
        for position in 0..Self::BIT_COUNT {
            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {
                if let Some(octet) = bytes.get_mut((position / 8) as usize) {
                    *octet |= 0x80u8 >> (position % 8);
                }
            }
        }
        let unused = u8::try_from(
            octets
                .saturating_mul(8)
                .saturating_sub(Self::BIT_COUNT as usize),
        )
        .unwrap_or(0);
        igs_iso9506_mms_codec::generated::types::Data::BitString(
            igs_iso9506_mms_codec::schema::BitString { unused, bytes },
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::BitString(bits) => {
                let length = bits
                    .bytes
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(usize::from(bits.unused));
                if length < Self::BIT_COUNT as usize {
                    return Err(crate::mms::ValueError::Format(
                        "a bit string shorter than the flags of its type",
                    ));
                }
                let mut value = 0u64;
                for position in 0..length {
                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);
                    if octet & (0x80u8 >> (position % 8)) != 0 {
                        if position >= Self::BIT_COUNT as usize {
                            return Err(crate::mms::ValueError::Format(
                                "a bit beyond the flags of its type is set",
                            ));
                        }
                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);
                    }
                }
                Ok(Self(value))
            }
            other => Err(crate::mms::ValueError::kind_of("bit string", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 70). type definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coefficient(pub f64);
impl Coefficient {
    /// The format width, in bits.
    pub const FORMAT_WIDTH: u32 = 32;
    /// The exponent width, in bits.
    pub const EXPONENT_WIDTH: u32 = 8;
}
impl crate::mms::MmsValue for Coefficient {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let [a, b, c, d] = (self.0 as f32).to_be_bytes();
        igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
            igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 70). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurveSegmentDescriptionOrder(i64);
impl CurveSegmentDescriptionOrder {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for CurveSegmentDescriptionOrder {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 70). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct CurveSegmentDescription {
    /// The component `order`.
    pub order: CurveSegmentDescriptionOrder,
    /// The component `low_range`.
    pub low_range: crate::generated::iso802::CurveLimit,
    /// The component `high_range`.
    pub high_range: crate::generated::iso802::CurveLimit,
    /// The component `number_of_segments`.
    pub number_of_segments: crate::generated::iso802::NumSegs,
}
impl crate::mms::MmsValue for CurveSegmentDescription {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.order),
            crate::mms::MmsValue::to_mms(&self.low_range),
            crate::mms::MmsValue::to_mms(&self.high_range),
            crate::mms::MmsValue::to_mms(&self.number_of_segments),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items
                .as_slice()
            {
                [item0, item1, item2, item3] => Ok(Self {
                    order: <CurveSegmentDescriptionOrder as crate::mms::MmsValue>::from_mms(item0)?,
                    low_range:
                        <crate::generated::iso802::CurveLimit as crate::mms::MmsValue>::from_mms(
                            item1,
                        )?,
                    high_range:
                        <crate::generated::iso802::CurveLimit as crate::mms::MmsValue>::from_mms(
                            item2,
                        )?,
                    number_of_segments:
                        <crate::generated::iso802::NumSegs as crate::mms::MmsValue>::from_mms(
                            item3,
                        )?,
                }),
                _ => Err(crate::mms::ValueError::Format(
                    "a structure of another number of components",
                )),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 70). inline definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurveCurveName(String);
impl CurveCurveName {
    /// The greatest number of characters (a maximum: decision D-004).
    pub const MAX_LEN: usize = 32;
    /// Whether the length is variable (the data file says so).
    pub const VARYING: bool = true;
    /// The string, if it has at most the greatest number of visible characters (decision D-014).
    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {
        let text = text.into();
        crate::value::check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }
    /// The string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl crate::mms::MmsValue for CurveCurveName {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::VisibleString(self.as_str().to_string())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::VisibleString(text) => {
                Ok(Self::new(text.clone())?)
            }
            other => Err(crate::mms::ValueError::kind_of("visible string", other)),
        }
    }
}
/// IEC 60870-6-802 B.3 (page 70). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Curve {
    /// The component `curve_name`.
    pub curve_name: CurveCurveName,
    /// The component `plant_reference_id`.
    pub plant_reference_id: crate::generated::iso802::PlantId,
    /// The component `unit_id`.
    pub unit_id: crate::generated::iso802::UnitId,
    /// The component `curve_type`.
    pub curve_type: crate::generated::iso802::CurveType,
    /// The component `number_of_segments`.
    pub number_of_segments: crate::generated::iso802::NumSegs,
}
impl crate::mms::MmsValue for Curve {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.curve_name),
            crate::mms::MmsValue::to_mms(&self.plant_reference_id),
            crate::mms::MmsValue::to_mms(&self.unit_id),
            crate::mms::MmsValue::to_mms(&self.curve_type),
            crate::mms::MmsValue::to_mms(&self.number_of_segments),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items
                .as_slice()
            {
                [item0, item1, item2, item3, item4] => Ok(Self {
                    curve_name: <CurveCurveName as crate::mms::MmsValue>::from_mms(item0)?,
                    plant_reference_id:
                        <crate::generated::iso802::PlantId as crate::mms::MmsValue>::from_mms(
                            item1,
                        )?,
                    unit_id: <crate::generated::iso802::UnitId as crate::mms::MmsValue>::from_mms(
                        item2,
                    )?,
                    curve_type:
                        <crate::generated::iso802::CurveType as crate::mms::MmsValue>::from_mms(
                            item3,
                        )?,
                    number_of_segments:
                        <crate::generated::iso802::NumSegs as crate::mms::MmsValue>::from_mms(
                            item4,
                        )?,
                }),
                _ => Err(crate::mms::ValueError::Format(
                    "a structure of another number of components",
                )),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.4.2 (page 71). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatrixId(i64);
impl MatrixId {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for MatrixId {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.4.2 (page 72). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerArrayXXElement(i64);
impl IntegerArrayXXElement {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for IntegerArrayXXElement {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.4.2 (page 72). type definition.
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerArrayXX(pub Vec<IntegerArrayXXElement>);
impl IntegerArrayXX {
    /// The number of elements, when the data file gives one.
    pub const SIZE: Option<usize> = None;
}
impl crate::mms::MmsValue for IntegerArrayXX {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Array(
            self.0.iter().map(crate::mms::MmsValue::to_mms).collect(),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Array(items) => {
                if Self::SIZE.is_some_and(|size| items.len() != size) {
                    return Err(crate::mms::ValueError::Format(
                        "an array of another number of elements",
                    ));
                }
                let elements = items
                    .iter()
                    .map(<IntegerArrayXXElement as crate::mms::MmsValue>::from_mms)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self(elements))
            }
            other => Err(crate::mms::ValueError::kind_of("array", other)),
        }
    }
}
/// IEC 60870-6-802 B.4.2 (page 72). inline definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatArrayXXElement(pub f64);
impl FloatArrayXXElement {
    /// The format width, in bits.
    pub const FORMAT_WIDTH: u32 = 32;
    /// The exponent width, in bits.
    pub const EXPONENT_WIDTH: u32 = 8;
}
impl crate::mms::MmsValue for FloatArrayXXElement {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        let [a, b, c, d] = (self.0 as f32).to_be_bytes();
        igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
            igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
    }
}
/// IEC 60870-6-802 B.4.2 (page 72). type definition.
#[derive(Debug, Clone, PartialEq)]
pub struct FloatArrayXX(pub Vec<FloatArrayXXElement>);
impl FloatArrayXX {
    /// The number of elements, when the data file gives one.
    pub const SIZE: Option<usize> = None;
}
impl crate::mms::MmsValue for FloatArrayXX {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Array(
            self.0.iter().map(crate::mms::MmsValue::to_mms).collect(),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Array(items) => {
                if Self::SIZE.is_some_and(|size| items.len() != size) {
                    return Err(crate::mms::ValueError::Format(
                        "an array of another number of elements",
                    ));
                }
                let elements = items
                    .iter()
                    .map(<FloatArrayXXElement as crate::mms::MmsValue>::from_mms)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self(elements))
            }
            other => Err(crate::mms::ValueError::kind_of("array", other)),
        }
    }
}
/// IEC 60870-6-802 B.4.2 (page 72). inline definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text32ArrayXXElement(String);
impl Text32ArrayXXElement {
    /// The greatest number of characters (a maximum: decision D-004).
    pub const MAX_LEN: usize = 32;
    /// Whether the length is variable (the data file says so).
    pub const VARYING: bool = false;
    /// The string, if it has at most the greatest number of visible characters (decision D-014).
    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {
        let text = text.into();
        crate::value::check_visible(&text, Self::MAX_LEN)?;
        Ok(Self(text))
    }
    /// The string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl crate::mms::MmsValue for Text32ArrayXXElement {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::VisibleString(self.as_str().to_string())
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::VisibleString(text) => {
                Ok(Self::new(text.clone())?)
            }
            other => Err(crate::mms::ValueError::kind_of("visible string", other)),
        }
    }
}
/// IEC 60870-6-802 B.4.2 (page 72). type definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Text32ArrayXX(pub Vec<Text32ArrayXXElement>);
impl Text32ArrayXX {
    /// The number of elements, when the data file gives one.
    pub const SIZE: Option<usize> = None;
}
impl crate::mms::MmsValue for Text32ArrayXX {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Array(
            self.0.iter().map(crate::mms::MmsValue::to_mms).collect(),
        )
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Array(items) => {
                if Self::SIZE.is_some_and(|size| items.len() != size) {
                    return Err(crate::mms::ValueError::Format(
                        "an array of another number of elements",
                    ));
                }
                let elements = items
                    .iter()
                    .map(<Text32ArrayXXElement as crate::mms::MmsValue>::from_mms)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self(elements))
            }
            other => Err(crate::mms::ValueError::kind_of("array", other)),
        }
    }
}
/// IEC 60870-6-802 B.5 (page 72). type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumRows(i64);
impl NumRows {
    /// The width in bits.
    pub const WIDTH: u32 = 16;
    /// The least value.
    pub const MIN: i64 = -32768;
    /// The greatest value.
    pub const MAX: i64 = 32767;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -32768,
                max: 32767,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for NumRows {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.5 (page 72). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralDataReport {
    /// The component `general_data_report_reference_number`.
    pub general_data_report_reference_number: crate::generated::iso802::clause_6_3::ReferenceNum,
    /// The component `report_name`.
    pub report_name: crate::generated::iso802::NameString,
    /// The component `report_date_and_time`.
    pub report_date_and_time: crate::generated::iso802::TimeStampS,
    /// The component `transaction_code`.
    pub transaction_code: crate::generated::iso802::clause_b_5::TransactionCode,
    /// The component `number_of_local_reference`.
    pub number_of_local_reference: crate::generated::iso802::NumIntegers,
    /// The component `number_of_floating_point1`.
    pub number_of_floating_point1: crate::generated::iso802::NumFloats,
    /// The component `number_of_floating_point1_rows`.
    pub number_of_floating_point1_rows: crate::generated::iso802::NumRows,
    /// The component `number_of_floating_point2`.
    pub number_of_floating_point2: crate::generated::iso802::NumFloats,
    /// The component `number_of_floating_point2_rows`.
    pub number_of_floating_point2_rows: crate::generated::iso802::NumRows,
    /// The component `number_of_integer1`.
    pub number_of_integer1: crate::generated::iso802::NumIntegers,
    /// The component `number_of_integer1_rows`.
    pub number_of_integer1_rows: crate::generated::iso802::NumRows,
    /// The component `number_of_integer2`.
    pub number_of_integer2: crate::generated::iso802::NumIntegers,
    /// The component `number_of_integer2_rows`.
    pub number_of_integer2_rows: crate::generated::iso802::NumRows,
    /// The component `number_of_text1`.
    pub number_of_text1: crate::generated::iso802::NumIntegers,
    /// The component `number_of_text1_rows`.
    pub number_of_text1_rows: crate::generated::iso802::NumRows,
    /// The component `number_of_text2`.
    pub number_of_text2: crate::generated::iso802::NumIntegers,
    /// The component `number_of_text2_rows`.
    pub number_of_text2_rows: crate::generated::iso802::NumRows,
}
impl crate::mms::MmsValue for GeneralDataReport {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.general_data_report_reference_number),
            crate::mms::MmsValue::to_mms(&self.report_name),
            crate::mms::MmsValue::to_mms(&self.report_date_and_time),
            crate::mms::MmsValue::to_mms(&self.transaction_code),
            crate::mms::MmsValue::to_mms(&self.number_of_local_reference),
            crate::mms::MmsValue::to_mms(&self.number_of_floating_point1),
            crate::mms::MmsValue::to_mms(&self.number_of_floating_point1_rows),
            crate::mms::MmsValue::to_mms(&self.number_of_floating_point2),
            crate::mms::MmsValue::to_mms(&self.number_of_floating_point2_rows),
            crate::mms::MmsValue::to_mms(&self.number_of_integer1),
            crate::mms::MmsValue::to_mms(&self.number_of_integer1_rows),
            crate::mms::MmsValue::to_mms(&self.number_of_integer2),
            crate::mms::MmsValue::to_mms(&self.number_of_integer2_rows),
            crate::mms::MmsValue::to_mms(&self.number_of_text1),
            crate::mms::MmsValue::to_mms(&self.number_of_text1_rows),
            crate::mms::MmsValue::to_mms(&self.number_of_text2),
            crate::mms::MmsValue::to_mms(&self.number_of_text2_rows),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10, item11, item12, item13, item14, item15, item16] => Ok(Self { general_data_report_reference_number: <crate::generated::iso802::clause_6_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, report_name: <crate::generated::iso802::NameString as crate::mms::MmsValue>::from_mms(item1)?, report_date_and_time: <crate::generated::iso802::TimeStampS as crate::mms::MmsValue>::from_mms(item2)?, transaction_code: <crate::generated::iso802::clause_b_5::TransactionCode as crate::mms::MmsValue>::from_mms(item3)?, number_of_local_reference: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item4)?, number_of_floating_point1: <crate::generated::iso802::NumFloats as crate::mms::MmsValue>::from_mms(item5)?, number_of_floating_point1_rows: <crate::generated::iso802::NumRows as crate::mms::MmsValue>::from_mms(item6)?, number_of_floating_point2: <crate::generated::iso802::NumFloats as crate::mms::MmsValue>::from_mms(item7)?, number_of_floating_point2_rows: <crate::generated::iso802::NumRows as crate::mms::MmsValue>::from_mms(item8)?, number_of_integer1: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item9)?, number_of_integer1_rows: <crate::generated::iso802::NumRows as crate::mms::MmsValue>::from_mms(item10)?, number_of_integer2: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item11)?, number_of_integer2_rows: <crate::generated::iso802::NumRows as crate::mms::MmsValue>::from_mms(item12)?, number_of_text1: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item13)?, number_of_text1_rows: <crate::generated::iso802::NumRows as crate::mms::MmsValue>::from_mms(item14)?, number_of_text2: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item15)?, number_of_text2_rows: <crate::generated::iso802::NumRows as crate::mms::MmsValue>::from_mms(item16)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-802 B.6 (page 72). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneralDataResponseResponseData(i64);
impl GeneralDataResponseResponseData {
    /// The width in bits.
    pub const WIDTH: u32 = 32;
    /// The least value.
    pub const MIN: i64 = -2147483648;
    /// The greatest value.
    pub const MAX: i64 = 2147483647;
    /// The value, if it is in range.
    pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::value::OutOfRange {
                value: i128::from(value),
                min: -2147483648,
                max: 2147483647,
            })
        }
    }
    /// The value.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl crate::mms::MmsValue for GeneralDataResponseResponseData {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                    value: *value,
                    min: i128::from(Self::MIN),
                    max: i128::from(Self::MAX),
                })?;
                Ok(Self::new(value)?)
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-802 B.6 (page 72). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralDataResponse {
    /// The component `report_reference_number`.
    pub report_reference_number: crate::generated::iso802::clause_6_3::ReferenceNum,
    /// The component `report_name`.
    pub report_name: crate::generated::iso802::NameString,
    /// The component `report_time_stamp`.
    pub report_time_stamp: crate::generated::iso802::clause_6_2_1::DataTimeStamp,
    /// The component `number_of_local_references`.
    pub number_of_local_references: crate::generated::iso802::NumIntegers,
    /// The component `response_data`.
    pub response_data: GeneralDataResponseResponseData,
    /// The component `response_code`.
    pub response_code: crate::generated::iso802::clause_b_2::TransactionCode,
    /// The component `response_text`.
    pub response_text: crate::generated::iso802::CommentString,
}
impl crate::mms::MmsValue for GeneralDataResponse {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.report_reference_number),
            crate::mms::MmsValue::to_mms(&self.report_name),
            crate::mms::MmsValue::to_mms(&self.report_time_stamp),
            crate::mms::MmsValue::to_mms(&self.number_of_local_references),
            crate::mms::MmsValue::to_mms(&self.response_data),
            crate::mms::MmsValue::to_mms(&self.response_code),
            crate::mms::MmsValue::to_mms(&self.response_text),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6] => Ok(Self { report_reference_number: <crate::generated::iso802::clause_6_3::ReferenceNum as crate::mms::MmsValue>::from_mms(item0)?, report_name: <crate::generated::iso802::NameString as crate::mms::MmsValue>::from_mms(item1)?, report_time_stamp: <crate::generated::iso802::clause_6_2_1::DataTimeStamp as crate::mms::MmsValue>::from_mms(item2)?, number_of_local_references: <crate::generated::iso802::NumIntegers as crate::mms::MmsValue>::from_mms(item3)?, response_data: <GeneralDataResponseResponseData as crate::mms::MmsValue>::from_mms(item4)?, response_code: <crate::generated::iso802::clause_b_2::TransactionCode as crate::mms::MmsValue>::from_mms(item5)?, response_text: <crate::generated::iso802::CommentString as crate::mms::MmsValue>::from_mms(item6)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}

/// The definitions of the clause 6.2.1.
pub mod clause_6_2_1 {
    /// IEC 60870-6-802 6.2.1 (page 24). type definition.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct DataReal(pub f64);
    impl DataReal {
        /// The format width, in bits.
        pub const FORMAT_WIDTH: u32 = 32;
        /// The exponent width, in bits.
        pub const EXPONENT_WIDTH: u32 = 8;
    }
    impl crate::mms::MmsValue for DataReal {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            let [a, b, c, d] = (self.0 as f32).to_be_bytes();
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
                igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
            )
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
        }
    }
    /// IEC 60870-6-802 6.2.1 (page 24). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DataDiscrete(i64);
    impl DataDiscrete {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for DataDiscrete {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
    /// IEC 60870-6-802 6.2.1 (page 24). reference definition.
    pub type DataTimeStamp = crate::generated::iso503::GMTBasedS;
}

/// The definitions of the clause 6.3.
pub mod clause_6_3 {
    /// IEC 60870-6-802 6.3 (page 28). type definition.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct DataReal(pub f64);
    impl DataReal {
        /// The format width, in bits.
        pub const FORMAT_WIDTH: u32 = 32;
        /// The exponent width, in bits.
        pub const EXPONENT_WIDTH: u32 = 8;
    }
    impl crate::mms::MmsValue for DataReal {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            let [a, b, c, d] = (self.0 as f32).to_be_bytes();
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
                igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
            )
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
        }
    }
    /// IEC 60870-6-802 6.3 (page 28). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Number(i64);
    impl Number {
        /// The width in bits.
        pub const WIDTH: u32 = 16;
        /// The least value.
        pub const MIN: i64 = -32768;
        /// The greatest value.
        pub const MAX: i64 = 32767;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -32768,
                    max: 32767,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for Number {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
    /// IEC 60870-6-802 6.3 (page 28). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ReferenceNum(i64);
    impl ReferenceNum {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for ReferenceNum {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
    /// IEC 60870-6-802 6.3 (page 29). reference definition.
    pub type ScheduleTime = crate::generated::iso503::GMTBasedS;
    /// IEC 60870-6-802 6.3 (page 29). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UtilityId(i64);
    impl UtilityId {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for UtilityId {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
}

/// The definitions of the clause 6.4.
pub mod clause_6_4 {
    /// IEC 60870-6-802 6.4 (page 30). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ReferenceNum(i64);
    impl ReferenceNum {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for ReferenceNum {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
    /// IEC 60870-6-802 6.4 (page 30). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Number(i64);
    impl Number {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for Number {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
}

/// The definitions of the clause b.2.
pub mod clause_b_2 {
    /// IEC 60870-6-802 B.2 (page 65). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DataDiscrete(i64);
    impl DataDiscrete {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for DataDiscrete {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
    /// IEC 60870-6-802 B.2 (page 65). type definition.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct DataReal(pub f64);
    impl DataReal {
        /// The format width, in bits.
        pub const FORMAT_WIDTH: u32 = 32;
        /// The exponent width, in bits.
        pub const EXPONENT_WIDTH: u32 = 8;
    }
    impl crate::mms::MmsValue for DataReal {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            let [a, b, c, d] = (self.0 as f32).to_be_bytes();
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
                igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
            )
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
        }
    }
    /// IEC 60870-6-802 B.2 (page 65). reference definition.
    pub type DataTimeStamp = crate::generated::iso503::GMTBasedS;
    /// IEC 60870-6-802 B.2 (page 65). type definition.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Price(pub f64);
    impl Price {
        /// The format width, in bits.
        pub const FORMAT_WIDTH: u32 = 32;
        /// The exponent width, in bits.
        pub const EXPONENT_WIDTH: u32 = 8;
    }
    impl crate::mms::MmsValue for Price {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            let [a, b, c, d] = (self.0 as f32).to_be_bytes();
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
                igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
            )
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
        }
    }
    /// IEC 60870-6-802 B.2 (page 65). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ReferenceNum(i64);
    impl ReferenceNum {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for ReferenceNum {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
    /// IEC 60870-6-802 B.2 (page 65). reference definition.
    pub type ScheduleTime = crate::generated::iso503::GMTBasedS;
    /// IEC 60870-6-802 B.2 (page 65). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TransactionCode(i64);
    impl TransactionCode {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for TransactionCode {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
    /// IEC 60870-6-802 B.2 (page 65). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UtilityId(i64);
    impl UtilityId {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for UtilityId {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
}

/// The definitions of the clause b.3.
pub mod clause_b_3 {
    /// IEC 60870-6-802 B.3 (page 68). type definition.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Price(pub f64);
    impl Price {
        /// The format width, in bits.
        pub const FORMAT_WIDTH: u32 = 32;
        /// The exponent width, in bits.
        pub const EXPONENT_WIDTH: u32 = 8;
    }
    impl crate::mms::MmsValue for Price {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            let [a, b, c, d] = (self.0 as f32).to_be_bytes();
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(
                igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]),
            )
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {
                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),
                _ => Err(crate::mms::ValueError::Format("a floating point value must be single precision: exponent width 8 and four octets")),
            },
            other => Err(crate::mms::ValueError::kind_of("floating point", other)),
        }
        }
    }
    /// IEC 60870-6-802 B.3 (page 68). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ReferenceNum(i64);
    impl ReferenceNum {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for ReferenceNum {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
}

/// The definitions of the clause b.5.
pub mod clause_b_5 {
    /// IEC 60870-6-802 B.5 (page 72). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TransactionCode(i64);
    impl TransactionCode {
        /// The width in bits.
        pub const WIDTH: u32 = 32;
        /// The least value.
        pub const MIN: i64 = -2147483648;
        /// The greatest value.
        pub const MAX: i64 = 2147483647;
        /// The value, if it is in range.
        pub fn new(value: i64) -> Result<Self, crate::value::OutOfRange> {
            if (Self::MIN..=Self::MAX).contains(&value) {
                Ok(Self(value))
            } else {
                Err(crate::value::OutOfRange {
                    value: i128::from(value),
                    min: -2147483648,
                    max: 2147483647,
                })
            }
        }
        /// The value.
        pub fn get(self) -> i64 {
            self.0
        }
    }
    impl crate::mms::MmsValue for TransactionCode {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
                igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                    let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {
                        value: *value,
                        min: i128::from(Self::MIN),
                        max: i128::from(Self::MAX),
                    })?;
                    Ok(Self::new(value)?)
                }
                other => Err(crate::mms::ValueError::kind_of("integer", other)),
            }
        }
    }
}
pub use clause_6_2_1::DataDiscrete;
pub use clause_6_2_1::DataTimeStamp;
pub use clause_6_3::ScheduleTime;
pub use clause_6_3::UtilityId;
