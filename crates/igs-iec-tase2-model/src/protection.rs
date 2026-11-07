// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The protection equipment events (IEC 60870-6-802 §6.2.3, §7.1.3): the flags of a single event and of a packed event,
//! and the event state of a single event.
//!
//! The flags are named by what their bit means in the mapping of the attributes: a bit that is set is the named
//! condition, except where the mapping gives the opposite (`Topical`: a set bit is NOTTOPICAL). The bits named `unused`
//! carry no accessor and are kept as received (decision D-021).

use crate::generated::iso802::{EventFlags, PackedFlags, SingleFlags};
use crate::indication::{get_code, put_code, TwoBitCode};

/// The state of a protection event (the mapping of `EventState`): OFF is code 1, ON is code 2, and code 0 or 3 is
/// INVALID (802 §7.1.3, the mapping of the protection events).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectionEventState {
    /// The equipment has not operated: code 1.
    Off,
    /// The equipment has operated: code 2.
    On,
    /// The state is not known: code 0 or 3. Writing it uses code 0.
    Invalid,
}

impl ProtectionEventState {
    /// The state of a two-bit code.
    pub fn from_code(code: TwoBitCode) -> Self {
        match code {
            TwoBitCode::Code01 => Self::Off,
            TwoBitCode::Code10 => Self::On,
            TwoBitCode::Code00 | TwoBitCode::Code11 => Self::Invalid,
        }
    }

    /// The two-bit code of the state.
    pub fn to_code(self) -> TwoBitCode {
        match self {
            Self::Off => TwoBitCode::Code01,
            Self::On => TwoBitCode::Code10,
            Self::Invalid => TwoBitCode::Code00,
        }
    }
}

/// The accessors that `SingleFlags` and `PackedFlags` share: the first five bits have the same meaning in both.
macro_rules! common_protection_flags {
    ($ty:ident) => {
        impl $ty {
            /// Whether the elapsed time is invalid: bit 0 is set (the attribute `ElapsedTimeValidity`, INVALID).
            pub fn elapsed_time_invalid(self) -> bool {
                self.0 & Self::ELAPSED_TIME_VALIDITY != 0
            }

            /// Whether the protection is blocked: bit 1 is set (the attribute `Blocked`, BLOCKED).
            pub fn blocked(self) -> bool {
                self.0 & Self::BLOCKED != 0
            }

            /// Whether the value is substituted: bit 2 is set (the attribute `Substituted`, SUBSTITUTED).
            pub fn substituted(self) -> bool {
                self.0 & Self::SUBSTITUTED != 0
            }

            /// Whether the event is not topical: bit 3 is set (the attribute `Topical`, NOTTOPICAL; a clear bit is
            /// TOPICAL, the inverse of the bit).
            pub fn not_topical(self) -> bool {
                self.0 & Self::TOPICAL != 0
            }

            /// Whether the event is invalid: bit 4 is set (the attribute `EventValidity`, INVALID).
            pub fn event_invalid(self) -> bool {
                self.0 & Self::EVENT_VALIDITY != 0
            }
        }
    };
}

common_protection_flags!(SingleFlags);
common_protection_flags!(PackedFlags);

impl SingleFlags {
    /// The state of the event: its bits 6 and 7 (`EventState_hi` and `EventState_lo`).
    pub fn event_state(self) -> ProtectionEventState {
        ProtectionEventState::from_code(get_code(
            self.0,
            Self::EVENT_STATE_HI,
            Self::EVENT_STATE_LO,
        ))
    }

    /// The same flags with another event state; the other bits are kept.
    #[must_use]
    pub fn with_event_state(self, state: ProtectionEventState) -> Self {
        Self(put_code(
            self.0,
            state.to_code(),
            Self::EVENT_STATE_HI,
            Self::EVENT_STATE_LO,
        ))
    }
}

impl EventFlags {
    /// Whether bit 0 is set: the general start (`StartGeneral`) or the general trip (`TripGeneral`), by the attribute.
    pub fn general(self) -> bool {
        self.0 & Self::GENERAL != 0
    }

    /// Whether bit 1 is set: the start or trip of phase 1 (`StartPhase1`, `TripPhase1`).
    pub fn phase1(self) -> bool {
        self.0 & Self::PHASE1 != 0
    }

    /// Whether bit 2 is set: the start or trip of phase 2 (`StartPhase2`, `TripPhase2`).
    pub fn phase2(self) -> bool {
        self.0 & Self::PHASE2 != 0
    }

    /// Whether bit 3 is set: the start or trip of phase 3 (`StartPhase3`, `TripPhase3`).
    pub fn phase3(self) -> bool {
        self.0 & Self::PHASE3 != 0
    }

    /// Whether bit 4 is set: the start or trip of earth (`StartEarth`, `TripEarth`).
    pub fn earth(self) -> bool {
        self.0 & Self::EARTH != 0
    }

    /// Whether bit 5 is set: the start or trip of reverse (`StartReverse`, `TripReverse`).
    pub fn reverse(self) -> bool {
        self.0 & Self::REVERSE != 0
    }
}
