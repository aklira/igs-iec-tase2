// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! Types of IEC 60870-6-503, generated from its data file by `cargo xtask codegen`.
//!
//! DO NOT EDIT. The clauses are cited on each type; the text of the standard is not in this file.

/// Every definition of this file: its name, clause, page, kind, components and bits.
pub const DEFINITIONS: &[crate::value::Definition] = &[
    crate::value::Definition {
        name: "Visible-String-32",
        standard: "IEC 60870-6-503",
        clause: "8.2.2",
        page: 92,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "MMS ObjectName",
        standard: "IEC 60870-6-503",
        clause: "8.2.3",
        page: 92,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "GMTBasedS",
        standard: "IEC 60870-6-503",
        clause: "8.2.4",
        page: 93,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "GMTBasedM",
        standard: "IEC 60870-6-503",
        clause: "8.2.4",
        page: 93,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "GMTBasedH",
        standard: "IEC 60870-6-503",
        clause: "8.2.4",
        page: 93,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "GMTBasedD",
        standard: "IEC 60870-6-503",
        clause: "8.2.4",
        page: 93,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TimeStampExtended",
        standard: "IEC 60870-6-503",
        clause: "8.2.5",
        page: 93,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "TimeIntervalD",
        standard: "IEC 60870-6-503",
        clause: "8.2.6",
        page: 94,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TimeIntervalH",
        standard: "IEC 60870-6-503",
        clause: "8.2.6",
        page: 94,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TimeIntervalM",
        standard: "IEC 60870-6-503",
        clause: "8.2.6",
        page: 94,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TimeIntervalS",
        standard: "IEC 60870-6-503",
        clause: "8.2.6",
        page: 94,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TimeIntervalL16",
        standard: "IEC 60870-6-503",
        clause: "8.2.6",
        page: 94,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TimeIntervalL32",
        standard: "IEC 60870-6-503",
        clause: "8.2.6",
        page: 94,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "DSTransferSet",
        standard: "IEC 60870-6-503",
        clause: "8.2.7.1",
        page: 94,
        kind: "structure",
        components: 13,
        bits: 0,
    },
    crate::value::Definition {
        name: "IMTransferSet",
        standard: "IEC 60870-6-503",
        clause: "8.2.7.1",
        page: 95,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TSTransferSet",
        standard: "IEC 60870-6-503",
        clause: "8.2.7.2",
        page: 95,
        kind: "structure",
        components: 8,
        bits: 0,
    },
    crate::value::Definition {
        name: "TATransferSet",
        standard: "IEC 60870-6-503",
        clause: "8.2.7.2",
        page: 96,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "DSConditions",
        standard: "IEC 60870-6-503",
        clause: "8.2.8.1",
        page: 96,
        kind: "bit-string",
        components: 0,
        bits: 5,
    },
    crate::value::Definition {
        name: "TSConditions",
        standard: "IEC 60870-6-503",
        clause: "8.2.8.2",
        page: 97,
        kind: "bit-string",
        components: 0,
        bits: 4,
    },
    crate::value::Definition {
        name: "SupportedFeatures",
        standard: "IEC 60870-6-503",
        clause: "8.2.9",
        page: 97,
        kind: "bit-string",
        components: 0,
        bits: 12,
    },
    crate::value::Definition {
        name: "TASE.2Version",
        standard: "IEC 60870-6-503",
        clause: "8.2.10",
        page: 97,
        kind: "structure",
        components: 2,
        bits: 0,
    },
    crate::value::Definition {
        name: "Supported_Features",
        standard: "IEC 60870-6-503",
        clause: "8.3.2",
        page: 98,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Bilateral_Table_ID",
        standard: "IEC 60870-6-503",
        clause: "8.3.3",
        page: 98,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TASE.2_Version",
        standard: "IEC 60870-6-503",
        clause: "8.3.4",
        page: 98,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Next_DSTransfer_Set",
        standard: "IEC 60870-6-503",
        clause: "8.3.7",
        page: 99,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Transfer_Set_Name",
        standard: "IEC 60870-6-503",
        clause: "8.3.8",
        page: 99,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "IM_Transfer_Set",
        standard: "IEC 60870-6-503",
        clause: "8.3.9",
        page: 99,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "DSConditions_Detected",
        standard: "IEC 60870-6-503",
        clause: "8.3.10",
        page: 99,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Transfer_Set_Time_Stamp",
        standard: "IEC 60870-6-503",
        clause: "8.3.11",
        page: 100,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Transfer_Report_ACK",
        standard: "IEC 60870-6-503",
        clause: "8.3.12",
        page: 100,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Transfer_Report_NACK",
        standard: "IEC 60870-6-503",
        clause: "8.3.13",
        page: 100,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Info_Mess_Header",
        standard: "IEC 60870-6-503",
        clause: "8.5",
        page: 100,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Info_Buff_XX",
        standard: "IEC 60870-6-503",
        clause: "8.5",
        page: 100,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TSTransferSet",
        standard: "IEC 60870-6-503",
        clause: "G.2.2",
        page: 150,
        kind: "structure",
        components: 8,
        bits: 0,
    },
    crate::value::Definition {
        name: "TATransferSet",
        standard: "IEC 60870-6-503",
        clause: "G.2.2",
        page: 151,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "IMTransferSet",
        standard: "IEC 60870-6-503",
        clause: "G.2.2",
        page: 151,
        kind: "type",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "SupportedFeatures",
        standard: "IEC 60870-6-503",
        clause: "G.2.3",
        page: 151,
        kind: "bit-string",
        components: 0,
        bits: 12,
    },
    crate::value::Definition {
        name: "TSConditions",
        standard: "IEC 60870-6-503",
        clause: "G.2.4",
        page: 151,
        kind: "bit-string",
        components: 0,
        bits: 4,
    },
    crate::value::Definition {
        name: "TAConditions",
        standard: "IEC 60870-6-503",
        clause: "G.2.4",
        page: 152,
        kind: "bit-string",
        components: 0,
        bits: 8,
    },
    crate::value::Definition {
        name: "Next_TSTransfer_Set",
        standard: "IEC 60870-6-503",
        clause: "G.3.2",
        page: 152,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TSConditions_Detected",
        standard: "IEC 60870-6-503",
        clause: "G.3.3",
        page: 152,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TA_Transfer_Set",
        standard: "IEC 60870-6-503",
        clause: "G.3.4",
        page: 152,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TAConditions_Detected",
        standard: "IEC 60870-6-503",
        clause: "G.3.5",
        page: 153,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Event_Code_Detected",
        standard: "IEC 60870-6-503",
        clause: "G.3.6",
        page: 153,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Info_Mess_Header",
        standard: "IEC 60870-6-503",
        clause: "G.3.7",
        page: 153,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "Info_Buff_xx",
        standard: "IEC 60870-6-503",
        clause: "G.3.7",
        page: 153,
        kind: "variable",
        components: 0,
        bits: 0,
    },
    crate::value::Definition {
        name: "TSTransferSet",
        standard: "IEC 60870-6-503",
        clause: "G.4",
        page: 154,
        kind: "structure",
        components: 8,
        bits: 0,
    },
    crate::value::Definition {
        name: "TATransferSet",
        standard: "IEC 60870-6-503",
        clause: "G.4",
        page: 155,
        kind: "structure",
        components: 3,
        bits: 0,
    },
    crate::value::Definition {
        name: "IMTransferSet",
        standard: "IEC 60870-6-503",
        clause: "G.4",
        page: 155,
        kind: "type",
        components: 0,
        bits: 0,
    },
];

/// The variables of the bilateral table: name, clause, page, scope, and the type of the variable.
pub const VARIABLES: &[crate::value::VariableDef] = &[
    crate::value::VariableDef {
        name: "Supported_Features",
        clause: "8.3.2",
        page: 98,
        scope: "VMD-specific",
        type_path: "crate::generated::iso503::clause_8_2_9::SupportedFeatures",
    },
    crate::value::VariableDef {
        name: "Bilateral_Table_ID",
        clause: "8.3.3",
        page: 98,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::VisibleString32",
    },
    crate::value::VariableDef {
        name: "TASE.2_Version",
        clause: "8.3.4",
        page: 98,
        scope: "VMD-specific",
        type_path: "crate::generated::iso503::TASE2Version",
    },
    crate::value::VariableDef {
        name: "Next_DSTransfer_Set",
        clause: "8.3.7",
        page: 99,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::MMSObjectName",
    },
    crate::value::VariableDef {
        name: "Transfer_Set_Name",
        clause: "8.3.8",
        page: 99,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::MMSObjectName",
    },
    crate::value::VariableDef {
        name: "IM_Transfer_Set",
        clause: "8.3.9",
        page: 99,
        scope: "AA-specific",
        type_path: "crate::generated::iso503::clause_8_2_7_1::IMTransferSet",
    },
    crate::value::VariableDef {
        name: "DSConditions_Detected",
        clause: "8.3.10",
        page: 99,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::DSConditions",
    },
    crate::value::VariableDef {
        name: "Transfer_Set_Time_Stamp",
        clause: "8.3.11",
        page: 100,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::GMTBasedS",
    },
    crate::value::VariableDef {
        name: "Transfer_Report_ACK",
        clause: "8.3.12",
        page: 100,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::MMSObjectName",
    },
    crate::value::VariableDef {
        name: "Transfer_Report_NACK",
        clause: "8.3.13",
        page: 100,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::MMSObjectName",
    },
    crate::value::VariableDef {
        name: "Info_Mess_Header",
        clause: "8.5",
        page: 100,
        scope: "VMD-specific or Domain-specific",
        type_path: "crate::generated::iso802::InfoMessHeader",
    },
    crate::value::VariableDef {
        name: "Info_Buff_XX",
        clause: "8.5",
        page: 100,
        scope: "VMD-specific or Domain-specific",
        type_path: "crate::generated::iso802::InfoBufXX",
    },
    crate::value::VariableDef {
        name: "Next_TSTransfer_Set",
        clause: "G.3.2",
        page: 152,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::MMSObjectName",
    },
    crate::value::VariableDef {
        name: "TSConditions_Detected",
        clause: "G.3.3",
        page: 152,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::clause_8_2_8_2::TSConditions",
    },
    crate::value::VariableDef {
        name: "TA_Transfer_Set",
        clause: "G.3.4",
        page: 152,
        scope: "AA-specific",
        type_path: "crate::generated::iso503::clause_8_2_7_2::TATransferSet",
    },
    crate::value::VariableDef {
        name: "TAConditions_Detected",
        clause: "G.3.5",
        page: 153,
        scope: "Domain-specific",
        type_path: "crate::generated::iso503::TAConditions",
    },
    crate::value::VariableDef {
        name: "Event_Code_Detected",
        clause: "G.3.6",
        page: 153,
        scope: "Domain-specific",
        type_path: "crate::builtin::Integer16",
    },
    crate::value::VariableDef {
        name: "Info_Mess_Header",
        clause: "G.3.7",
        page: 153,
        scope: "VMD-specific or Domain-specific",
        type_path: "crate::generated::iso802::InfoMessHeader",
    },
    crate::value::VariableDef {
        name: "Info_Buff_xx",
        clause: "G.3.7",
        page: 153,
        scope: "VMD-specific or Domain-specific",
        type_path: "crate::generated::iso802::InfoBufXX",
    },
];
/// IEC 60870-6-503 8.2.2 (page 92). type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibleString32(String);
impl VisibleString32 {
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
impl crate::mms::MmsValue for VisibleString32 {
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
/// IEC 60870-6-503 8.2.3 (page 92). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MMSObjectNameScope(pub i64);
impl MMSObjectNameScope {
    /// The value `domain-specific`.
    pub const DOMAIN_SPECIFIC: i64 = 1;
    /// The value `vmd-specific`.
    pub const VMD_SPECIFIC: i64 = 0;
}
impl crate::mms::MmsValue for MMSObjectNameScope {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Integer(i128::from(self.0))
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Integer(value) => {
                match i64::try_from(*value) {
                    Ok(value) if [1, 0].contains(&value) => Ok(Self(value)),
                    _ => Err(crate::mms::ValueError::Format(
                        "not a value of the enumeration",
                    )),
                }
            }
            other => Err(crate::mms::ValueError::kind_of("integer", other)),
        }
    }
}
/// IEC 60870-6-503 8.2.3 (page 92). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct MMSObjectName {
    /// The component `scope`.
    pub scope: MMSObjectNameScope,
    /// The component `domain_name`.
    pub domain_name: crate::builtin::MmsIdentifier,
    /// The component `name`.
    pub name: crate::builtin::MmsIdentifier,
}
impl crate::mms::MmsValue for MMSObjectName {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.scope),
            crate::mms::MmsValue::to_mms(&self.domain_name),
            crate::mms::MmsValue::to_mms(&self.name),
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
                    scope: <MMSObjectNameScope as crate::mms::MmsValue>::from_mms(item0)?,
                    domain_name: <crate::builtin::MmsIdentifier as crate::mms::MmsValue>::from_mms(
                        item1,
                    )?,
                    name: <crate::builtin::MmsIdentifier as crate::mms::MmsValue>::from_mms(item2)?,
                }),
                _ => Err(crate::mms::ValueError::Format(
                    "a structure of another number of components",
                )),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-503 8.2.4 (page 93). type definition.
pub type GMTBasedS = crate::builtin::Integer32;
/// IEC 60870-6-503 8.2.4 (page 93). type definition.
pub type GMTBasedM = crate::builtin::Integer32;
/// IEC 60870-6-503 8.2.4 (page 93). type definition.
pub type GMTBasedH = crate::builtin::Integer32;
/// IEC 60870-6-503 8.2.4 (page 93). type definition.
pub type GMTBasedD = crate::builtin::Integer32;
/// IEC 60870-6-503 8.2.5 (page 93). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeStampExtended {
    /// The component `gmt_based_s`.
    pub gmt_based_s: crate::generated::iso503::GMTBasedS,
    /// The component `milliseconds`.
    pub milliseconds: crate::generated::iso503::TimeIntervalL16,
}
/// IEC 60870-6-503 8.2.6 (page 94). type definition.
pub type TimeIntervalD = crate::builtin::Integer16;
/// IEC 60870-6-503 8.2.6 (page 94). type definition.
pub type TimeIntervalH = crate::builtin::Integer16;
/// IEC 60870-6-503 8.2.6 (page 94). type definition.
pub type TimeIntervalM = crate::builtin::Integer16;
/// IEC 60870-6-503 8.2.6 (page 94). type definition.
pub type TimeIntervalS = crate::builtin::Integer32;
/// IEC 60870-6-503 8.2.6 (page 94). type definition.
pub type TimeIntervalL16 = crate::builtin::Integer16;
/// IEC 60870-6-503 8.2.6 (page 94). type definition.
pub type TimeIntervalL32 = crate::builtin::Integer32;
/// IEC 60870-6-503 8.2.7.1 (page 94). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DSTransferSetBlockData(pub bool);
impl DSTransferSetBlockData {
    /// The value `FALSE`.
    pub const FALSE: bool = false;
    /// The value `TRUE`.
    pub const TRUE: bool = true;
}
impl crate::mms::MmsValue for DSTransferSetBlockData {
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
/// IEC 60870-6-503 8.2.7.1 (page 94). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DSTransferSetCritical(pub bool);
impl DSTransferSetCritical {
    /// The value `FALSE`.
    pub const FALSE: bool = false;
    /// The value `TRUE`.
    pub const TRUE: bool = true;
}
impl crate::mms::MmsValue for DSTransferSetCritical {
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
/// IEC 60870-6-503 8.2.7.1 (page 94). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DSTransferSetRBE(pub bool);
impl DSTransferSetRBE {
    /// The value `FALSE`.
    pub const FALSE: bool = false;
    /// The value `TRUE`.
    pub const TRUE: bool = true;
}
impl crate::mms::MmsValue for DSTransferSetRBE {
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
/// IEC 60870-6-503 8.2.7.1 (page 94). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DSTransferSetAllChangesReported(pub bool);
impl DSTransferSetAllChangesReported {
    /// The value `FALSE`.
    pub const FALSE: bool = false;
    /// The value `TRUE`.
    pub const TRUE: bool = true;
}
impl crate::mms::MmsValue for DSTransferSetAllChangesReported {
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
/// IEC 60870-6-503 8.2.7.1 (page 94). inline definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DSTransferSetStatus(pub bool);
impl DSTransferSetStatus {
    /// The value `DISABLED`.
    pub const DISABLED: bool = false;
    /// The value `ENABLED`.
    pub const ENABLED: bool = true;
}
impl crate::mms::MmsValue for DSTransferSetStatus {
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
/// IEC 60870-6-503 8.2.7.1 (page 94). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DSTransferSet {
    /// The component `data_set_name`.
    pub data_set_name: crate::generated::iso503::MMSObjectName,
    /// The component `start_time`.
    pub start_time: crate::generated::iso503::GMTBasedS,
    /// The component `interval`.
    pub interval: crate::generated::iso503::TimeIntervalS,
    /// The component `tle`.
    pub tle: crate::generated::iso503::TimeIntervalS,
    /// The component `buffer_time`.
    pub buffer_time: crate::generated::iso503::TimeIntervalS,
    /// The component `integrity_check`.
    pub integrity_check: crate::generated::iso503::TimeIntervalS,
    /// The component `ds_conditions_requested`.
    pub ds_conditions_requested: crate::generated::iso503::DSConditions,
    /// The component `block_data`.
    pub block_data: DSTransferSetBlockData,
    /// The component `critical`.
    pub critical: DSTransferSetCritical,
    /// The component `rbe`.
    pub rbe: DSTransferSetRBE,
    /// The component `all_changes_reported`.
    pub all_changes_reported: DSTransferSetAllChangesReported,
    /// The component `status`.
    pub status: DSTransferSetStatus,
    /// The component `event_code_requested`.
    pub event_code_requested: crate::builtin::Integer16,
}
impl crate::mms::MmsValue for DSTransferSet {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.data_set_name),
            crate::mms::MmsValue::to_mms(&self.start_time),
            crate::mms::MmsValue::to_mms(&self.interval),
            crate::mms::MmsValue::to_mms(&self.tle),
            crate::mms::MmsValue::to_mms(&self.buffer_time),
            crate::mms::MmsValue::to_mms(&self.integrity_check),
            crate::mms::MmsValue::to_mms(&self.ds_conditions_requested),
            crate::mms::MmsValue::to_mms(&self.block_data),
            crate::mms::MmsValue::to_mms(&self.critical),
            crate::mms::MmsValue::to_mms(&self.rbe),
            crate::mms::MmsValue::to_mms(&self.all_changes_reported),
            crate::mms::MmsValue::to_mms(&self.status),
            crate::mms::MmsValue::to_mms(&self.event_code_requested),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7, item8, item9, item10, item11, item12] => Ok(Self { data_set_name: <crate::generated::iso503::MMSObjectName as crate::mms::MmsValue>::from_mms(item0)?, start_time: <crate::generated::iso503::GMTBasedS as crate::mms::MmsValue>::from_mms(item1)?, interval: <crate::generated::iso503::TimeIntervalS as crate::mms::MmsValue>::from_mms(item2)?, tle: <crate::generated::iso503::TimeIntervalS as crate::mms::MmsValue>::from_mms(item3)?, buffer_time: <crate::generated::iso503::TimeIntervalS as crate::mms::MmsValue>::from_mms(item4)?, integrity_check: <crate::generated::iso503::TimeIntervalS as crate::mms::MmsValue>::from_mms(item5)?, ds_conditions_requested: <crate::generated::iso503::DSConditions as crate::mms::MmsValue>::from_mms(item6)?, block_data: <DSTransferSetBlockData as crate::mms::MmsValue>::from_mms(item7)?, critical: <DSTransferSetCritical as crate::mms::MmsValue>::from_mms(item8)?, rbe: <DSTransferSetRBE as crate::mms::MmsValue>::from_mms(item9)?, all_changes_reported: <DSTransferSetAllChangesReported as crate::mms::MmsValue>::from_mms(item10)?, status: <DSTransferSetStatus as crate::mms::MmsValue>::from_mms(item11)?, event_code_requested: <crate::builtin::Integer16 as crate::mms::MmsValue>::from_mms(item12)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
    }
}
/// IEC 60870-6-503 8.2.8.1 (page 96). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DSConditions(pub u64);
impl DSConditions {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 5;
    /// The mask of the flag `IntervalTimeOut`.
    pub const INTERVAL_TIME_OUT: u64 = 0x1;
    /// The mask of the flag `IntegrityTimeOut`.
    pub const INTEGRITY_TIME_OUT: u64 = 0x2;
    /// The mask of the flag `ObjectChange`.
    pub const OBJECT_CHANGE: u64 = 0x4;
    /// The mask of the flag `Operator Request`.
    pub const OPERATOR_REQUEST: u64 = 0x8;
    /// The mask of the flag `OtherExternalEvent`.
    pub const OTHER_EXTERNAL_EVENT: u64 = 0x10;
}
impl DSConditions {
    /// Whether the flag `IntervalTimeOut` is set.
    pub fn is_interval_time_out(self) -> bool {
        self.0 & Self::INTERVAL_TIME_OUT != 0
    }
    /// Whether the flag `IntegrityTimeOut` is set.
    pub fn is_integrity_time_out(self) -> bool {
        self.0 & Self::INTEGRITY_TIME_OUT != 0
    }
    /// Whether the flag `ObjectChange` is set.
    pub fn is_object_change(self) -> bool {
        self.0 & Self::OBJECT_CHANGE != 0
    }
    /// Whether the flag `Operator Request` is set.
    pub fn is_operator_request(self) -> bool {
        self.0 & Self::OPERATOR_REQUEST != 0
    }
    /// Whether the flag `OtherExternalEvent` is set.
    pub fn is_other_external_event(self) -> bool {
        self.0 & Self::OTHER_EXTERNAL_EVENT != 0
    }
}
impl crate::mms::MmsValue for DSConditions {
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
/// IEC 60870-6-503 8.2.10 (page 97). structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TASE2Version {
    /// The component `major_version_number`.
    pub major_version_number: crate::builtin::Integer16,
    /// The component `minor_version_number`.
    pub minor_version_number: crate::builtin::Integer16,
}
impl crate::mms::MmsValue for TASE2Version {
    fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
        igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
            crate::mms::MmsValue::to_mms(&self.major_version_number),
            crate::mms::MmsValue::to_mms(&self.minor_version_number),
        ])
    }
    fn from_mms(
        data: &igs_iso9506_mms_codec::generated::types::Data,
    ) -> Result<Self, crate::mms::ValueError> {
        match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => {
                match items.as_slice() {
                    [item0, item1] => Ok(Self {
                        major_version_number:
                            <crate::builtin::Integer16 as crate::mms::MmsValue>::from_mms(item0)?,
                        minor_version_number:
                            <crate::builtin::Integer16 as crate::mms::MmsValue>::from_mms(item1)?,
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
/// IEC 60870-6-503 G.2.4 (page 152). bit-string definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TAConditions(pub u64);
impl TAConditions {
    /// The number of bits of the string.
    pub const BIT_COUNT: u32 = 8;
    /// The mask of the flag `BeforeTheHour`.
    pub const BEFORE_THE_HOUR: u64 = 0x1;
    /// The mask of the flag `DispatchUpdate`.
    pub const DISPATCH_UPDATE: u64 = 0x2;
    /// The mask of the flag `DuringTheHour`.
    pub const DURING_THE_HOUR: u64 = 0x4;
    /// The mask of the flag `AfterTheHour`.
    pub const AFTER_THE_HOUR: u64 = 0x8;
    /// The mask of the flag `ActualDataUpdate`.
    pub const ACTUAL_DATA_UPDATE: u64 = 0x10;
    /// The mask of the flag `PastHours`.
    pub const PAST_HOURS: u64 = 0x20;
    /// The mask of the flag `ObjectChange`.
    pub const OBJECT_CHANGE: u64 = 0x40;
    /// The mask of the flag `Operator Request`.
    pub const OPERATOR_REQUEST: u64 = 0x80;
}
impl TAConditions {
    /// Whether the flag `BeforeTheHour` is set.
    pub fn is_before_the_hour(self) -> bool {
        self.0 & Self::BEFORE_THE_HOUR != 0
    }
    /// Whether the flag `DispatchUpdate` is set.
    pub fn is_dispatch_update(self) -> bool {
        self.0 & Self::DISPATCH_UPDATE != 0
    }
    /// Whether the flag `DuringTheHour` is set.
    pub fn is_during_the_hour(self) -> bool {
        self.0 & Self::DURING_THE_HOUR != 0
    }
    /// Whether the flag `AfterTheHour` is set.
    pub fn is_after_the_hour(self) -> bool {
        self.0 & Self::AFTER_THE_HOUR != 0
    }
    /// Whether the flag `ActualDataUpdate` is set.
    pub fn is_actual_data_update(self) -> bool {
        self.0 & Self::ACTUAL_DATA_UPDATE != 0
    }
    /// Whether the flag `PastHours` is set.
    pub fn is_past_hours(self) -> bool {
        self.0 & Self::PAST_HOURS != 0
    }
    /// Whether the flag `ObjectChange` is set.
    pub fn is_object_change(self) -> bool {
        self.0 & Self::OBJECT_CHANGE != 0
    }
    /// Whether the flag `Operator Request` is set.
    pub fn is_operator_request(self) -> bool {
        self.0 & Self::OPERATOR_REQUEST != 0
    }
}
impl crate::mms::MmsValue for TAConditions {
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

/// The definitions of the clause 8.2.7.1.
pub mod clause_8_2_7_1 {
    /// IEC 60870-6-503 8.2.7.1 (page 95). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IMTransferSet(pub bool);
    impl crate::mms::MmsValue for IMTransferSet {
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
}

/// The definitions of the clause 8.2.7.2.
pub mod clause_8_2_7_2 {
    /// IEC 60870-6-503 8.2.7.2 (page 95). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TSTransferSetBlockData(pub bool);
    impl TSTransferSetBlockData {
        /// The value `FALSE`.
        pub const FALSE: bool = false;
        /// The value `TRUE`.
        pub const TRUE: bool = true;
    }
    impl crate::mms::MmsValue for TSTransferSetBlockData {
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
    /// IEC 60870-6-503 8.2.7.2 (page 95). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TSTransferSetStatus(pub bool);
    impl TSTransferSetStatus {
        /// The value `DISABLED`.
        pub const DISABLED: bool = false;
        /// The value `ENABLED`.
        pub const ENABLED: bool = true;
    }
    impl crate::mms::MmsValue for TSTransferSetStatus {
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
    /// IEC 60870-6-503 8.2.7.2 (page 95). structure definition.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TSTransferSet {
        /// The component `data_value_name`.
        pub data_value_name: crate::generated::iso503::MMSObjectName,
        /// The component `ts_conditions_requested`.
        pub ts_conditions_requested: crate::generated::iso503::clause_8_2_8_2::TSConditions,
        /// The component `block_data`.
        pub block_data: TSTransferSetBlockData,
        /// The component `begin_time`.
        pub begin_time: crate::generated::iso503::GMTBasedS,
        /// The component `end_time`.
        pub end_time: crate::generated::iso503::GMTBasedS,
        /// The component `sampling_interval`.
        pub sampling_interval: crate::generated::iso503::TimeIntervalL32,
        /// The component `reporting_interval`.
        pub reporting_interval: crate::generated::iso503::TimeIntervalL32,
        /// The component `status`.
        pub status: TSTransferSetStatus,
    }
    impl crate::mms::MmsValue for TSTransferSet {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
                crate::mms::MmsValue::to_mms(&self.data_value_name),
                crate::mms::MmsValue::to_mms(&self.ts_conditions_requested),
                crate::mms::MmsValue::to_mms(&self.block_data),
                crate::mms::MmsValue::to_mms(&self.begin_time),
                crate::mms::MmsValue::to_mms(&self.end_time),
                crate::mms::MmsValue::to_mms(&self.sampling_interval),
                crate::mms::MmsValue::to_mms(&self.reporting_interval),
                crate::mms::MmsValue::to_mms(&self.status),
            ])
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7] => Ok(Self { data_value_name: <crate::generated::iso503::MMSObjectName as crate::mms::MmsValue>::from_mms(item0)?, ts_conditions_requested: <crate::generated::iso503::clause_8_2_8_2::TSConditions as crate::mms::MmsValue>::from_mms(item1)?, block_data: <TSTransferSetBlockData as crate::mms::MmsValue>::from_mms(item2)?, begin_time: <crate::generated::iso503::GMTBasedS as crate::mms::MmsValue>::from_mms(item3)?, end_time: <crate::generated::iso503::GMTBasedS as crate::mms::MmsValue>::from_mms(item4)?, sampling_interval: <crate::generated::iso503::TimeIntervalL32 as crate::mms::MmsValue>::from_mms(item5)?, reporting_interval: <crate::generated::iso503::TimeIntervalL32 as crate::mms::MmsValue>::from_mms(item6)?, status: <TSTransferSetStatus as crate::mms::MmsValue>::from_mms(item7)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
        }
    }
    /// IEC 60870-6-503 8.2.7.2 (page 96). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TATransferSetBlockData(pub bool);
    impl TATransferSetBlockData {
        /// The value `FALSE`.
        pub const FALSE: bool = false;
        /// The value `TRUE`.
        pub const TRUE: bool = true;
    }
    impl crate::mms::MmsValue for TATransferSetBlockData {
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
    /// IEC 60870-6-503 8.2.7.2 (page 96). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TATransferSetStatus(pub bool);
    impl TATransferSetStatus {
        /// The value `DISABLED`.
        pub const DISABLED: bool = false;
        /// The value `ENABLED`.
        pub const ENABLED: bool = true;
    }
    impl crate::mms::MmsValue for TATransferSetStatus {
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
    /// IEC 60870-6-503 8.2.7.2 (page 96). structure definition.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TATransferSet {
        /// The component `ta_conditions_requested`.
        pub ta_conditions_requested: crate::generated::iso503::TAConditions,
        /// The component `block_data`.
        pub block_data: TATransferSetBlockData,
        /// The component `status`.
        pub status: TATransferSetStatus,
    }
    impl crate::mms::MmsValue for TATransferSet {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
                crate::mms::MmsValue::to_mms(&self.ta_conditions_requested),
                crate::mms::MmsValue::to_mms(&self.block_data),
                crate::mms::MmsValue::to_mms(&self.status),
            ])
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { ta_conditions_requested: <crate::generated::iso503::TAConditions as crate::mms::MmsValue>::from_mms(item0)?, block_data: <TATransferSetBlockData as crate::mms::MmsValue>::from_mms(item1)?, status: <TATransferSetStatus as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
        }
    }
}

/// The definitions of the clause 8.2.8.2.
pub mod clause_8_2_8_2 {
    /// IEC 60870-6-503 8.2.8.2 (page 97). bit-string definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TSConditions(pub u64);
    impl TSConditions {
        /// The number of bits of the string.
        pub const BIT_COUNT: u32 = 4;
        /// The mask of the flag `EndTimeArrived`.
        pub const END_TIME_ARRIVED: u64 = 0x1;
        /// The mask of the flag `OperatorRequest`.
        pub const OPERATOR_REQUEST: u64 = 0x2;
        /// The mask of the flag `ReportIntervalTimeOut`.
        pub const REPORT_INTERVAL_TIME_OUT: u64 = 0x4;
        /// The mask of the flag `MaxMMSPDU`.
        pub const MAX_MMSPDU: u64 = 0x8;
    }
    impl TSConditions {
        /// Whether the flag `EndTimeArrived` is set.
        pub fn is_end_time_arrived(self) -> bool {
            self.0 & Self::END_TIME_ARRIVED != 0
        }
        /// Whether the flag `OperatorRequest` is set.
        pub fn is_operator_request(self) -> bool {
            self.0 & Self::OPERATOR_REQUEST != 0
        }
        /// Whether the flag `ReportIntervalTimeOut` is set.
        pub fn is_report_interval_time_out(self) -> bool {
            self.0 & Self::REPORT_INTERVAL_TIME_OUT != 0
        }
        /// Whether the flag `MaxMMSPDU` is set.
        pub fn is_max_mmspdu(self) -> bool {
            self.0 & Self::MAX_MMSPDU != 0
        }
    }
    impl crate::mms::MmsValue for TSConditions {
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
}

/// The definitions of the clause 8.2.9.
pub mod clause_8_2_9 {
    /// IEC 60870-6-503 8.2.9 (page 97). bit-string definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SupportedFeatures(pub u64);
    impl SupportedFeatures {
        /// The number of bits of the string.
        pub const BIT_COUNT: u32 = 12;
        /// The mask of the flag `Block1`.
        pub const BLOCK1: u64 = 0x1;
        /// The mask of the flag `Block2`.
        pub const BLOCK2: u64 = 0x2;
        /// The mask of the flag `Block4`.
        pub const BLOCK4: u64 = 0x8;
        /// The mask of the flag `Block5`.
        pub const BLOCK5: u64 = 0x10;
        /// The mask of the flag `Block11`.
        pub const BLOCK11: u64 = 0x400;
        /// The mask of the flag `Block12`.
        pub const BLOCK12: u64 = 0x800;
    }
    impl SupportedFeatures {
        /// Whether the flag `Block1` is set.
        pub fn is_block1(self) -> bool {
            self.0 & Self::BLOCK1 != 0
        }
        /// Whether the flag `Block2` is set.
        pub fn is_block2(self) -> bool {
            self.0 & Self::BLOCK2 != 0
        }
        /// Whether the flag `Block4` is set.
        pub fn is_block4(self) -> bool {
            self.0 & Self::BLOCK4 != 0
        }
        /// Whether the flag `Block5` is set.
        pub fn is_block5(self) -> bool {
            self.0 & Self::BLOCK5 != 0
        }
        /// Whether the flag `Block11` is set.
        pub fn is_block11(self) -> bool {
            self.0 & Self::BLOCK11 != 0
        }
        /// Whether the flag `Block12` is set.
        pub fn is_block12(self) -> bool {
            self.0 & Self::BLOCK12 != 0
        }
    }
    impl crate::mms::MmsValue for SupportedFeatures {
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
}

/// The definitions of the clause 8.5.
pub mod clause_8_5 {}

/// The definitions of the clause g.2.2.
pub mod clause_g_2_2 {
    /// IEC 60870-6-503 G.2.2 (page 150). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TSTransferSetBlockData(pub bool);
    impl TSTransferSetBlockData {
        /// The value `FALSE`.
        pub const FALSE: bool = false;
        /// The value `TRUE`.
        pub const TRUE: bool = true;
    }
    impl crate::mms::MmsValue for TSTransferSetBlockData {
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
    /// IEC 60870-6-503 G.2.2 (page 150). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TSTransferSetStatus(pub bool);
    impl TSTransferSetStatus {
        /// The value `DISABLED`.
        pub const DISABLED: bool = false;
        /// The value `ENABLED`.
        pub const ENABLED: bool = true;
    }
    impl crate::mms::MmsValue for TSTransferSetStatus {
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
    /// IEC 60870-6-503 G.2.2 (page 150). structure definition.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TSTransferSet {
        /// The component `data_value_name`.
        pub data_value_name: crate::generated::iso503::MMSObjectName,
        /// The component `ts_conditions_requested`.
        pub ts_conditions_requested: crate::generated::iso503::clause_8_2_8_2::TSConditions,
        /// The component `block_data`.
        pub block_data: TSTransferSetBlockData,
        /// The component `begin_time`.
        pub begin_time: crate::generated::iso503::GMTBasedS,
        /// The component `end_time`.
        pub end_time: crate::generated::iso503::GMTBasedS,
        /// The component `sampling_interval`.
        pub sampling_interval: crate::generated::iso503::TimeIntervalL32,
        /// The component `reporting_interval`.
        pub reporting_interval: crate::generated::iso503::TimeIntervalL32,
        /// The component `status`.
        pub status: TSTransferSetStatus,
    }
    impl crate::mms::MmsValue for TSTransferSet {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
                crate::mms::MmsValue::to_mms(&self.data_value_name),
                crate::mms::MmsValue::to_mms(&self.ts_conditions_requested),
                crate::mms::MmsValue::to_mms(&self.block_data),
                crate::mms::MmsValue::to_mms(&self.begin_time),
                crate::mms::MmsValue::to_mms(&self.end_time),
                crate::mms::MmsValue::to_mms(&self.sampling_interval),
                crate::mms::MmsValue::to_mms(&self.reporting_interval),
                crate::mms::MmsValue::to_mms(&self.status),
            ])
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7] => Ok(Self { data_value_name: <crate::generated::iso503::MMSObjectName as crate::mms::MmsValue>::from_mms(item0)?, ts_conditions_requested: <crate::generated::iso503::clause_8_2_8_2::TSConditions as crate::mms::MmsValue>::from_mms(item1)?, block_data: <TSTransferSetBlockData as crate::mms::MmsValue>::from_mms(item2)?, begin_time: <crate::generated::iso503::GMTBasedS as crate::mms::MmsValue>::from_mms(item3)?, end_time: <crate::generated::iso503::GMTBasedS as crate::mms::MmsValue>::from_mms(item4)?, sampling_interval: <crate::generated::iso503::TimeIntervalL32 as crate::mms::MmsValue>::from_mms(item5)?, reporting_interval: <crate::generated::iso503::TimeIntervalL32 as crate::mms::MmsValue>::from_mms(item6)?, status: <TSTransferSetStatus as crate::mms::MmsValue>::from_mms(item7)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
        }
    }
    /// IEC 60870-6-503 G.2.2 (page 151). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TATransferSetBlockData(pub bool);
    impl TATransferSetBlockData {
        /// The value `FALSE`.
        pub const FALSE: bool = false;
        /// The value `TRUE`.
        pub const TRUE: bool = true;
    }
    impl crate::mms::MmsValue for TATransferSetBlockData {
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
    /// IEC 60870-6-503 G.2.2 (page 151). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TATransferSetStatus(pub bool);
    impl TATransferSetStatus {
        /// The value `DISABLED`.
        pub const DISABLED: bool = false;
        /// The value `ENABLED`.
        pub const ENABLED: bool = true;
    }
    impl crate::mms::MmsValue for TATransferSetStatus {
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
    /// IEC 60870-6-503 G.2.2 (page 151). structure definition.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TATransferSet {
        /// The component `ta_conditions_requested`.
        pub ta_conditions_requested: crate::generated::iso503::TAConditions,
        /// The component `block_data`.
        pub block_data: TATransferSetBlockData,
        /// The component `status`.
        pub status: TATransferSetStatus,
    }
    impl crate::mms::MmsValue for TATransferSet {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
                crate::mms::MmsValue::to_mms(&self.ta_conditions_requested),
                crate::mms::MmsValue::to_mms(&self.block_data),
                crate::mms::MmsValue::to_mms(&self.status),
            ])
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { ta_conditions_requested: <crate::generated::iso503::TAConditions as crate::mms::MmsValue>::from_mms(item0)?, block_data: <TATransferSetBlockData as crate::mms::MmsValue>::from_mms(item1)?, status: <TATransferSetStatus as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
        }
    }
    /// IEC 60870-6-503 G.2.2 (page 151). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IMTransferSet(pub bool);
    impl crate::mms::MmsValue for IMTransferSet {
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
}

/// The definitions of the clause g.2.3.
pub mod clause_g_2_3 {
    /// IEC 60870-6-503 G.2.3 (page 151). bit-string definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SupportedFeatures(pub u64);
    impl SupportedFeatures {
        /// The number of bits of the string.
        pub const BIT_COUNT: u32 = 12;
        /// The mask of the flag `Block1`.
        pub const BLOCK1: u64 = 0x1;
        /// The mask of the flag `Block2`.
        pub const BLOCK2: u64 = 0x2;
        /// The mask of the flag `Block3`.
        pub const BLOCK3: u64 = 0x4;
        /// The mask of the flag `Block4`.
        pub const BLOCK4: u64 = 0x8;
        /// The mask of the flag `Block5`.
        pub const BLOCK5: u64 = 0x10;
        /// The mask of the flag `Block6`.
        pub const BLOCK6: u64 = 0x20;
        /// The mask of the flag `Block7`.
        pub const BLOCK7: u64 = 0x40;
        /// The mask of the flag `Block8`.
        pub const BLOCK8: u64 = 0x80;
        /// The mask of the flag `Block9`.
        pub const BLOCK9: u64 = 0x100;
        /// The mask of the flag `Block10`.
        pub const BLOCK10: u64 = 0x200;
        /// The mask of the flag `Block11`.
        pub const BLOCK11: u64 = 0x400;
        /// The mask of the flag `Block12`.
        pub const BLOCK12: u64 = 0x800;
    }
    impl SupportedFeatures {
        /// Whether the flag `Block1` is set.
        pub fn is_block1(self) -> bool {
            self.0 & Self::BLOCK1 != 0
        }
        /// Whether the flag `Block2` is set.
        pub fn is_block2(self) -> bool {
            self.0 & Self::BLOCK2 != 0
        }
        /// Whether the flag `Block3` is set.
        pub fn is_block3(self) -> bool {
            self.0 & Self::BLOCK3 != 0
        }
        /// Whether the flag `Block4` is set.
        pub fn is_block4(self) -> bool {
            self.0 & Self::BLOCK4 != 0
        }
        /// Whether the flag `Block5` is set.
        pub fn is_block5(self) -> bool {
            self.0 & Self::BLOCK5 != 0
        }
        /// Whether the flag `Block6` is set.
        pub fn is_block6(self) -> bool {
            self.0 & Self::BLOCK6 != 0
        }
        /// Whether the flag `Block7` is set.
        pub fn is_block7(self) -> bool {
            self.0 & Self::BLOCK7 != 0
        }
        /// Whether the flag `Block8` is set.
        pub fn is_block8(self) -> bool {
            self.0 & Self::BLOCK8 != 0
        }
        /// Whether the flag `Block9` is set.
        pub fn is_block9(self) -> bool {
            self.0 & Self::BLOCK9 != 0
        }
        /// Whether the flag `Block10` is set.
        pub fn is_block10(self) -> bool {
            self.0 & Self::BLOCK10 != 0
        }
        /// Whether the flag `Block11` is set.
        pub fn is_block11(self) -> bool {
            self.0 & Self::BLOCK11 != 0
        }
        /// Whether the flag `Block12` is set.
        pub fn is_block12(self) -> bool {
            self.0 & Self::BLOCK12 != 0
        }
    }
    impl crate::mms::MmsValue for SupportedFeatures {
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
}

/// The definitions of the clause g.2.4.
pub mod clause_g_2_4 {
    /// IEC 60870-6-503 G.2.4 (page 151). bit-string definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TSConditions(pub u64);
    impl TSConditions {
        /// The number of bits of the string.
        pub const BIT_COUNT: u32 = 4;
        /// The mask of the flag `EndTimeArrived`.
        pub const END_TIME_ARRIVED: u64 = 0x1;
        /// The mask of the flag `OperatorRequest`.
        pub const OPERATOR_REQUEST: u64 = 0x2;
        /// The mask of the flag `ReportIntervalTimeOut`.
        pub const REPORT_INTERVAL_TIME_OUT: u64 = 0x4;
        /// The mask of the flag `MaiMMSPDU`.
        pub const MAI_MMSPDU: u64 = 0x8;
    }
    impl TSConditions {
        /// Whether the flag `EndTimeArrived` is set.
        pub fn is_end_time_arrived(self) -> bool {
            self.0 & Self::END_TIME_ARRIVED != 0
        }
        /// Whether the flag `OperatorRequest` is set.
        pub fn is_operator_request(self) -> bool {
            self.0 & Self::OPERATOR_REQUEST != 0
        }
        /// Whether the flag `ReportIntervalTimeOut` is set.
        pub fn is_report_interval_time_out(self) -> bool {
            self.0 & Self::REPORT_INTERVAL_TIME_OUT != 0
        }
        /// Whether the flag `MaiMMSPDU` is set.
        pub fn is_mai_mmspdu(self) -> bool {
            self.0 & Self::MAI_MMSPDU != 0
        }
    }
    impl crate::mms::MmsValue for TSConditions {
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
}

/// The definitions of the clause g.3.7.
pub mod clause_g_3_7 {}

/// The definitions of the clause g.4.
pub mod clause_g_4 {
    /// IEC 60870-6-503 G.4 (page 154). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TSTransferSetBlockData(pub bool);
    impl TSTransferSetBlockData {
        /// The value `FALSE`.
        pub const FALSE: bool = false;
        /// The value `TRUE`.
        pub const TRUE: bool = true;
    }
    impl crate::mms::MmsValue for TSTransferSetBlockData {
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
    /// IEC 60870-6-503 G.4 (page 154). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TSTransferSetStatus(pub bool);
    impl TSTransferSetStatus {
        /// The value `DISABLED`.
        pub const DISABLED: bool = false;
        /// The value `ENABLED`.
        pub const ENABLED: bool = true;
    }
    impl crate::mms::MmsValue for TSTransferSetStatus {
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
    /// IEC 60870-6-503 G.4 (page 154). structure definition.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TSTransferSet {
        /// The component `data_value_name`.
        pub data_value_name: crate::generated::iso503::MMSObjectName,
        /// The component `ts_conditions_requested`.
        pub ts_conditions_requested: crate::generated::iso503::clause_8_2_8_2::TSConditions,
        /// The component `block_data`.
        pub block_data: TSTransferSetBlockData,
        /// The component `begin_time`.
        pub begin_time: crate::generated::iso503::GMTBasedS,
        /// The component `end_time`.
        pub end_time: crate::generated::iso503::GMTBasedS,
        /// The component `sampling_interval`.
        pub sampling_interval: crate::generated::iso503::TimeIntervalL32,
        /// The component `reporting_interval`.
        pub reporting_interval: crate::generated::iso503::TimeIntervalL32,
        /// The component `status`.
        pub status: TSTransferSetStatus,
    }
    impl crate::mms::MmsValue for TSTransferSet {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
                crate::mms::MmsValue::to_mms(&self.data_value_name),
                crate::mms::MmsValue::to_mms(&self.ts_conditions_requested),
                crate::mms::MmsValue::to_mms(&self.block_data),
                crate::mms::MmsValue::to_mms(&self.begin_time),
                crate::mms::MmsValue::to_mms(&self.end_time),
                crate::mms::MmsValue::to_mms(&self.sampling_interval),
                crate::mms::MmsValue::to_mms(&self.reporting_interval),
                crate::mms::MmsValue::to_mms(&self.status),
            ])
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2, item3, item4, item5, item6, item7] => Ok(Self { data_value_name: <crate::generated::iso503::MMSObjectName as crate::mms::MmsValue>::from_mms(item0)?, ts_conditions_requested: <crate::generated::iso503::clause_8_2_8_2::TSConditions as crate::mms::MmsValue>::from_mms(item1)?, block_data: <TSTransferSetBlockData as crate::mms::MmsValue>::from_mms(item2)?, begin_time: <crate::generated::iso503::GMTBasedS as crate::mms::MmsValue>::from_mms(item3)?, end_time: <crate::generated::iso503::GMTBasedS as crate::mms::MmsValue>::from_mms(item4)?, sampling_interval: <crate::generated::iso503::TimeIntervalL32 as crate::mms::MmsValue>::from_mms(item5)?, reporting_interval: <crate::generated::iso503::TimeIntervalL32 as crate::mms::MmsValue>::from_mms(item6)?, status: <TSTransferSetStatus as crate::mms::MmsValue>::from_mms(item7)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
        }
    }
    /// IEC 60870-6-503 G.4 (page 155). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TATransferSetBlockData(pub bool);
    impl TATransferSetBlockData {
        /// The value `FALSE`.
        pub const FALSE: bool = false;
        /// The value `TRUE`.
        pub const TRUE: bool = true;
    }
    impl crate::mms::MmsValue for TATransferSetBlockData {
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
    /// IEC 60870-6-503 G.4 (page 155). inline definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TATransferSetStatus(pub bool);
    impl TATransferSetStatus {
        /// The value `DISABLED`.
        pub const DISABLED: bool = false;
        /// The value `ENABLED`.
        pub const ENABLED: bool = true;
    }
    impl crate::mms::MmsValue for TATransferSetStatus {
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
    /// IEC 60870-6-503 G.4 (page 155). structure definition.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TATransferSet {
        /// The component `ta_conditions_requested`.
        pub ta_conditions_requested: crate::generated::iso503::TAConditions,
        /// The component `block_data`.
        pub block_data: TATransferSetBlockData,
        /// The component `status`.
        pub status: TATransferSetStatus,
    }
    impl crate::mms::MmsValue for TATransferSet {
        fn to_mms(&self) -> igs_iso9506_mms_codec::generated::types::Data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(vec![
                crate::mms::MmsValue::to_mms(&self.ta_conditions_requested),
                crate::mms::MmsValue::to_mms(&self.block_data),
                crate::mms::MmsValue::to_mms(&self.status),
            ])
        }
        fn from_mms(
            data: &igs_iso9506_mms_codec::generated::types::Data,
        ) -> Result<Self, crate::mms::ValueError> {
            match data {
            igs_iso9506_mms_codec::generated::types::Data::Structure(items) => match items.as_slice() {
                [item0, item1, item2] => Ok(Self { ta_conditions_requested: <crate::generated::iso503::TAConditions as crate::mms::MmsValue>::from_mms(item0)?, block_data: <TATransferSetBlockData as crate::mms::MmsValue>::from_mms(item1)?, status: <TATransferSetStatus as crate::mms::MmsValue>::from_mms(item2)? }),
                _ => Err(crate::mms::ValueError::Format("a structure of another number of components")),
            },
            other => Err(crate::mms::ValueError::kind_of("structure", other)),
        }
        }
    }
    /// IEC 60870-6-503 G.4 (page 155). type definition.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IMTransferSet(pub bool);
    impl crate::mms::MmsValue for IMTransferSet {
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
}
pub use clause_8_2_7_1::IMTransferSet;
pub use clause_8_2_7_2::TATransferSet;
pub use clause_8_2_7_2::TSTransferSet;
pub use clause_8_2_8_2::TSConditions;
pub use clause_8_2_9::SupportedFeatures;
