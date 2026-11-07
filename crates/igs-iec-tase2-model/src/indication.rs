// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The quality flags and the two-bit codes of the indication points (IEC 60870-6-802 §5.2.2, §6.2.1, §7.1.1, §8.2).
//!
//! The flags of `Data_Flags` and `Data_State` have the same layout: validity on bits 2 and 3 (`Validity_hi` and
//! `Validity_lo`), current source on bits 4 and 5, normal value on bit 6, time stamp quality on bit 7. A two-bit code is
//! the high bit times two plus the low bit: `Validity_hi` is the bit of value 2. The state of a point and the tag of a
//! supplemental state are two-bit codes too; their meaning depends on the device (§8.2) and is not given here (D-017).

use crate::generated::iso802::{DataFlags, DataState, DataStateSupplemental};

/// The validity of a value (§5.2.2, §7.1.1): VALID=0, HELD=1, SUSPECT=2, NOTVALID=3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Validity {
    /// The value is valid (0).
    Valid,
    /// The previous value is held over (1). The interpretation is local.
    Held,
    /// The value is questionable (2). The interpretation is local.
    Suspect,
    /// The value is not valid (3).
    NotValid,
}

impl Validity {
    /// The two-bit code of the validity.
    pub fn code(self) -> TwoBitCode {
        match self {
            Self::Valid => TwoBitCode::Code00,
            Self::Held => TwoBitCode::Code01,
            Self::Suspect => TwoBitCode::Code10,
            Self::NotValid => TwoBitCode::Code11,
        }
    }

    /// The validity of a two-bit code.
    pub fn from_code(code: TwoBitCode) -> Self {
        match code {
            TwoBitCode::Code00 => Self::Valid,
            TwoBitCode::Code01 => Self::Held,
            TwoBitCode::Code10 => Self::Suspect,
            TwoBitCode::Code11 => Self::NotValid,
        }
    }
}

/// The current source of a value (§5.2.2, §7.1.1): TELEMETERED=0, CALCULATED=1, ENTERED=2, ESTIMATED=3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentSource {
    /// Received from a telemetered site (0).
    Telemetered,
    /// Calculated from other values (1).
    Calculated,
    /// Entered manually (2).
    Entered,
    /// Estimated, for instance by a state estimator (3).
    Estimated,
}

impl CurrentSource {
    /// The two-bit code of the current source.
    pub fn code(self) -> TwoBitCode {
        match self {
            Self::Telemetered => TwoBitCode::Code00,
            Self::Calculated => TwoBitCode::Code01,
            Self::Entered => TwoBitCode::Code10,
            Self::Estimated => TwoBitCode::Code11,
        }
    }

    /// The current source of a two-bit code.
    pub fn from_code(code: TwoBitCode) -> Self {
        match code {
            TwoBitCode::Code00 => Self::Telemetered,
            TwoBitCode::Code01 => Self::Calculated,
            TwoBitCode::Code10 => Self::Entered,
            TwoBitCode::Code11 => Self::Estimated,
        }
    }
}

/// Whether the value is normal (§5.2.2): NORMAL=0, ABNORMAL=1 (bit 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalValue {
    /// The value is the configured normal value of the point (0).
    Normal,
    /// The value is not the configured normal value of the point (1).
    Abnormal,
}

/// The quality of the time stamp (§5.2.2, §7.1.1): VALID=0, INVALID=1 (bit 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeStampQuality {
    /// The time stamp is the time of the last change (0).
    Valid,
    /// The time stamp is not the time of the last change (1).
    Invalid,
}

/// A two-bit code: the value of a high bit and a low bit, the high bit counting double.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoBitCode {
    /// The code 0: both bits clear.
    Code00,
    /// The code 1: only the low bit set.
    Code01,
    /// The code 2: only the high bit set.
    Code10,
    /// The code 3: both bits set.
    Code11,
}

impl TwoBitCode {
    /// The code of a high and a low bit.
    pub fn from_bits(high: bool, low: bool) -> Self {
        match (high, low) {
            (false, false) => Self::Code00,
            (false, true) => Self::Code01,
            (true, false) => Self::Code10,
            (true, true) => Self::Code11,
        }
    }

    /// The value of the code, 0 to 3.
    pub fn value(self) -> u8 {
        match self {
            Self::Code00 => 0,
            Self::Code01 => 1,
            Self::Code10 => 2,
            Self::Code11 => 3,
        }
    }
}

/// The bits of a two-bit code at the masks of its high and low bits.
pub(crate) fn put_code(word: u64, code: TwoBitCode, high: u64, low: u64) -> u64 {
    let cleared = word & !(high | low);
    let value = code.value();
    let with_high = if value & 2 != 0 {
        cleared | high
    } else {
        cleared
    };
    if value & 1 != 0 {
        with_high | low
    } else {
        with_high
    }
}

/// The two-bit code at the masks of its high and low bits.
pub(crate) fn get_code(word: u64, high: u64, low: u64) -> TwoBitCode {
    TwoBitCode::from_bits(word & high != 0, word & low != 0)
}

/// Sets or clears one flag bit.
pub(crate) fn put_flag(word: u64, flag: u64, on: bool) -> u64 {
    if on {
        word | flag
    } else {
        word & !flag
    }
}

/// The quality accessors of a type whose layout is the one of the quality flags (`Data_Flags`, `Data_State`).
macro_rules! quality_accessors {
    ($ty:ident) => {
        impl $ty {
            /// The validity of the value: its bits 2 and 3.
            pub fn validity(self) -> Validity {
                Validity::from_code(get_code(self.0, Self::VALIDITY_HI, Self::VALIDITY_LO))
            }

            /// The same flags with another validity; the other bits are kept.
            #[must_use]
            pub fn with_validity(self, validity: Validity) -> Self {
                Self(put_code(
                    self.0,
                    validity.code(),
                    Self::VALIDITY_HI,
                    Self::VALIDITY_LO,
                ))
            }

            /// The current source of the value: its bits 4 and 5.
            pub fn current_source(self) -> CurrentSource {
                CurrentSource::from_code(get_code(
                    self.0,
                    Self::CURRENT_SOURCE_HI,
                    Self::CURRENT_SOURCE_LO,
                ))
            }

            /// The same flags with another current source; the other bits are kept.
            #[must_use]
            pub fn with_current_source(self, source: CurrentSource) -> Self {
                Self(put_code(
                    self.0,
                    source.code(),
                    Self::CURRENT_SOURCE_HI,
                    Self::CURRENT_SOURCE_LO,
                ))
            }

            /// Whether the value is normal: its bit 6.
            pub fn normal_value(self) -> NormalValue {
                if self.0 & Self::NORMAL_VALUE != 0 {
                    NormalValue::Abnormal
                } else {
                    NormalValue::Normal
                }
            }

            /// The same flags with another normal value; the other bits are kept.
            #[must_use]
            pub fn with_normal_value(self, value: NormalValue) -> Self {
                Self(put_flag(
                    self.0,
                    Self::NORMAL_VALUE,
                    value == NormalValue::Abnormal,
                ))
            }

            /// The quality of the time stamp: its bit 7.
            pub fn time_stamp_quality(self) -> TimeStampQuality {
                if self.0 & Self::TIME_STAMP_QUALITY != 0 {
                    TimeStampQuality::Invalid
                } else {
                    TimeStampQuality::Valid
                }
            }

            /// The same flags with another time stamp quality; the other bits are kept.
            #[must_use]
            pub fn with_time_stamp_quality(self, quality: TimeStampQuality) -> Self {
                Self(put_flag(
                    self.0,
                    Self::TIME_STAMP_QUALITY,
                    quality == TimeStampQuality::Invalid,
                ))
            }
        }
    };
}

quality_accessors!(DataFlags);
quality_accessors!(DataState);

impl DataState {
    /// The state of the point: its bits 0 and 1 (`State_hi` and `State_lo`).
    pub fn state(self) -> TwoBitCode {
        get_code(self.0, Self::STATE_HI, Self::STATE_LO)
    }

    /// The same state with another code; the other bits are kept.
    #[must_use]
    pub fn with_state(self, code: TwoBitCode) -> Self {
        Self(put_code(self.0, code, Self::STATE_HI, Self::STATE_LO))
    }
}

impl DataStateSupplemental {
    /// The state of the point: its bits 0 and 1.
    pub fn state(self) -> TwoBitCode {
        get_code(self.0, Self::STATE_HI, Self::STATE_LO)
    }

    /// The tag: its bits 2 and 3 (`Tag_hi` and `Tag_lo`).
    pub fn tag(self) -> TwoBitCode {
        get_code(self.0, Self::TAG_HI, Self::TAG_LO)
    }

    /// The expected state: its bits 4 and 5 (`ExpectedState_hi` and `ExpectedState_lo`). A value different from the state
    /// indicates a potential issue (§5.2.2).
    pub fn expected_state(self) -> TwoBitCode {
        get_code(self.0, Self::EXPECTED_STATE_HI, Self::EXPECTED_STATE_LO)
    }

    /// The same value with another state; the other bits are kept.
    #[must_use]
    pub fn with_state(self, code: TwoBitCode) -> Self {
        Self(put_code(self.0, code, Self::STATE_HI, Self::STATE_LO))
    }

    /// The same value with another tag; the other bits are kept.
    #[must_use]
    pub fn with_tag(self, code: TwoBitCode) -> Self {
        Self(put_code(self.0, code, Self::TAG_HI, Self::TAG_LO))
    }

    /// The same value with another expected state; the other bits are kept.
    #[must_use]
    pub fn with_expected_state(self, code: TwoBitCode) -> Self {
        Self(put_code(
            self.0,
            code,
            Self::EXPECTED_STATE_HI,
            Self::EXPECTED_STATE_LO,
        ))
    }
}
