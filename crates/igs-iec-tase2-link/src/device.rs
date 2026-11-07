// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The devices (IEC 60870-6-503 §5.3.12, §7.2.6; IEC 60870-6-802 §5.2 and §7.1), sans I/O: the Select-Before-Operate cycle
//! and the Direct-Control operation, the Tag, and the actions the device signals.
//!
//! An SBO device is IDLE or ARMED. Select takes it from IDLE to ARMED and returns its CheckBackID. The Operate of an ARMED device,
//! by the client that selected it, takes the device back to IDLE when the caller reports its outcome with `complete`. The device
//! returns to IDLE by its Timeout (the time of the selecting client's agreement), by a Local Reset, or by the outcome of Operate.
//! The Timeout timer stops while an Operate is in progress: it covers the selection, not the execution.
//!
//! The time is given by the caller, as for the other transfer sets. Nothing here reads a clock or executes a command: the caller
//! executes the command of an accepted Operate, and reports the outcome.

use std::time::Duration;

use igs_iec_tase2_model::generated::iso802::SBOCheckBackName;

use crate::bilateral::{Access, BilateralTable, Object, ObjectName, Permission, Scope};
use crate::data::ErrorCode;

/// The longest reason of a tag: the TextString of 802 §6.2.2 has a width of 255.
pub const MAX_REASON_LEN: usize = 255;

/// The client that selects, operates or tags a device: one identifier per association.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientId(pub u64);

/// The class of a device: Select-Before-Operate, or Direct-Control (the DeviceClass of 802 §5.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    /// The device must be selected before it is operated.
    Sbo,
    /// The device is operated directly.
    NonSbo,
}

/// The Tag of a device (802 §5.2 and §7.1). The codes are those of the two bits `Tag_hi` and `Tag_lo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    /// NO-TAG: Select and Operate are allowed.
    NoTag,
    /// OPEN-AND-CLOSE-INHIBIT: neither Select nor Operate is allowed.
    OpenAndCloseInhibit,
    /// CLOSE-ONLY-INHIBIT: Operate is refused for a Close or Raise command.
    CloseOnlyInhibit,
}

impl Tag {
    /// The two-bit code of the tag: NO-TAG = 0, OPEN-AND-CLOSE-INHIBIT = 1, CLOSE-ONLY-INHIBIT = 2 (802 §7.1).
    pub fn code(self) -> u8 {
        match self {
            Self::NoTag => 0,
            Self::OpenAndCloseInhibit => 1,
            Self::CloseOnlyInhibit => 2,
        }
    }

    /// The tag of a two-bit code; `None` for a code that the standard does not give (3).
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::NoTag),
            1 => Some(Self::OpenAndCloseInhibit),
            2 => Some(Self::CloseOnlyInhibit),
            _ => None,
        }
    }
}

/// The state of a device: IDLE, or ARMED (the Tag_state bit of 802 §7.1 is 0 for IDLE and 1 for ARMED).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    /// No selection is held.
    Idle,
    /// A selection is held, by one client.
    Armed,
}

/// The tag of a device, its reason and its state, as Get Tag returns them (802 §5.2, §7.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagState {
    /// The tag.
    pub tag: Tag,
    /// The reason of the tag; empty for NO-TAG.
    pub reason: String,
    /// The state of the device.
    pub state: DeviceState,
}

/// The outcome of an Operate, as the caller reports it after it executed the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The command was executed to completion: the Success action.
    Success,
    /// The command was not executed to completion: the Failure action. The client receives HARDWARE-FAULT (503 §7.2.6.3.3).
    Failure,
}

/// An action that the device signals (503 §5.3.12.7 to §5.3.12.10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// The ARMED state timed out and the device went back to IDLE.
    Timeout,
    /// A local action took the device back to IDLE.
    LocalReset,
    /// An Operate was completed successfully.
    Success,
    /// An Operate failed.
    Failure,
}

/// The selection held by a client: when it was made, and the timeout of the client's agreement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Selection {
    by: ClientId,
    since: Duration,
    timeout: Duration,
}

/// A device of the model, in the state of its control cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    name: ObjectName,
    class: DeviceClass,
    selection: Option<Selection>,
    operating: Option<ClientId>,
    tag: Tag,
    reason: String,
    tag_owner: Option<ClientId>,
    actions: Vec<Action>,
}

impl Device {
    /// A device of the given class, in IDLE, with NO-TAG.
    pub fn new(name: ObjectName, class: DeviceClass) -> Self {
        Self {
            name,
            class,
            selection: None,
            operating: None,
            tag: Tag::NoTag,
            reason: String::new(),
            tag_owner: None,
            actions: Vec::new(),
        }
    }

    /// The name of the device.
    pub fn name(&self) -> &ObjectName {
        &self.name
    }

    /// The class of the device.
    pub fn class(&self) -> DeviceClass {
        self.class
    }

    /// The state of the device: ARMED while a selection is held.
    pub fn state(&self) -> DeviceState {
        if self.selection.is_some() {
            DeviceState::Armed
        } else {
            DeviceState::Idle
        }
    }

    /// The tag, its reason and the state of the device (Get Tag).
    pub fn tag(&self) -> TagState {
        TagState {
            tag: self.tag,
            reason: self.reason.clone(),
            state: self.state(),
        }
    }

    /// The time of the next timer, or `None` when no timer runs: the caller calls `advance` at that time. The timer of a
    /// selection stops while an Operate is in progress.
    pub fn next_deadline(&self) -> Option<Duration> {
        match (self.selection, self.operating) {
            (Some(selection), None) => Some(selection.since.saturating_add(selection.timeout)),
            _ => None,
        }
    }

    /// Time passes: an ARMED device whose timer has run out goes back to IDLE, and the Timeout action is signalled.
    pub fn advance(&mut self, now: Duration) {
        if self.operating.is_some() {
            return;
        }
        if let Some(selection) = self.selection {
            if now.saturating_sub(selection.since) >= selection.timeout {
                self.selection = None;
                self.actions.push(Action::Timeout);
            }
        }
    }

    /// Select (503 §5.3.12.3, §7.2.6.2): the device goes from IDLE to ARMED for this client, and the CheckBackID of the table is
    /// returned. The Select fails for a non-SBO device, for a device that is ARMED or busy, and for a device tagged
    /// OPEN-AND-CLOSE-INHIBIT.
    pub fn select(
        &mut self,
        table: &BilateralTable,
        client: ClientId,
        now: Duration,
    ) -> Result<SBOCheckBackName, ErrorCode> {
        check_access(table, &self.name, Permission::Select)?;
        self.advance(now);
        if self.class == DeviceClass::NonSbo {
            return Err(ErrorCode::ObjectAccessDenied);
        }
        if self.selection.is_some()
            || self.operating.is_some()
            || self.tag == Tag::OpenAndCloseInhibit
        {
            return Err(ErrorCode::TemporarilyUnavailable);
        }
        let check_back_id = table
            .check_back_id(&self.name)
            .ok_or(ErrorCode::ObjectNonExistent)?;
        let timeout = table
            .device_timeout(&self.name)
            .ok_or(ErrorCode::ObjectNonExistent)?;
        self.selection = Some(Selection {
            by: client,
            since: now,
            timeout,
        });
        Ok(check_back_id)
    }

    /// Operate (503 §5.3.12.4, §7.2.6.3): the command is accepted for execution, and the caller executes it and reports its
    /// outcome with `complete`. An SBO device must be ARMED for this client. The Tag refuses the command: OPEN-AND-CLOSE-INHIBIT
    /// refuses every command, CLOSE-ONLY-INHIBIT the commands that close or raise (`close_or_raise`, which the device's own
    /// mapping of its commands gives: the standard leaves it local).
    pub fn operate(
        &mut self,
        table: &BilateralTable,
        client: ClientId,
        now: Duration,
        close_or_raise: bool,
    ) -> Result<(), ErrorCode> {
        check_access(table, &self.name, Permission::Operate)?;
        self.advance(now);
        if self.operating.is_some() {
            return Err(ErrorCode::TemporarilyUnavailable);
        }
        if self.class == DeviceClass::Sbo {
            match self.selection {
                Some(selection) if selection.by == client => {}
                _ => return Err(ErrorCode::TemporarilyUnavailable),
            }
        }
        match self.tag {
            Tag::NoTag => {}
            Tag::OpenAndCloseInhibit => return Err(ErrorCode::TemporarilyUnavailable),
            Tag::CloseOnlyInhibit if close_or_raise => {
                return Err(ErrorCode::TemporarilyUnavailable)
            }
            Tag::CloseOnlyInhibit => {}
        }
        self.operating = Some(client);
        Ok(())
    }

    /// The outcome of the Operate in progress: the device goes back to IDLE, and the Success or Failure action is signalled.
    /// Returns `false`, and does nothing, when no Operate is in progress (for instance after a Local Reset).
    pub fn complete(&mut self, outcome: Outcome) -> bool {
        if self.operating.take().is_none() {
            return false;
        }
        self.selection = None;
        self.actions.push(match outcome {
            Outcome::Success => Action::Success,
            Outcome::Failure => Action::Failure,
        });
        true
    }

    /// A Local Reset (503 §5.3.12.8): a local action takes the device back to IDLE, and ends an Operate in progress. The tag may
    /// be changed by the local action: `tag` is its new value, with no owner among the clients.
    pub fn local_reset(&mut self, tag: Option<Tag>) {
        self.selection = None;
        self.operating = None;
        if let Some(tag) = tag {
            self.tag = tag;
            self.tag_owner = None;
            if tag == Tag::NoTag {
                self.reason.clear();
            }
        }
        self.actions.push(Action::LocalReset);
    }

    /// Set Tag (503 §5.3.12.5, 802 §5.2): the tag and its reason are set. For a VCC-specific device (VMD scope), a tagged device
    /// may be changed only by the client that tagged it. A reason of more than 255 octets is refused.
    pub fn set_tag(
        &mut self,
        table: &BilateralTable,
        client: ClientId,
        tag: Tag,
        reason: &str,
    ) -> Result<(), ErrorCode> {
        check_access(table, &self.name, Permission::SetTag)?;
        if reason.len() > MAX_REASON_LEN {
            return Err(ErrorCode::TypeInconsistent);
        }
        let vcc_specific = self.name.scope == Scope::Vmd;
        if vcc_specific {
            if let Some(owner) = self.tag_owner {
                if owner != client && self.tag != Tag::NoTag {
                    return Err(ErrorCode::ObjectAccessDenied);
                }
            }
        }
        self.tag = tag;
        if tag == Tag::NoTag {
            self.reason.clear();
            self.tag_owner = None;
        } else {
            self.reason = reason.to_string();
            self.tag_owner = Some(client);
        }
        Ok(())
    }

    /// The actions signalled since the last call.
    pub fn take_actions(&mut self) -> Vec<Action> {
        std::mem::take(&mut self.actions)
    }
}

/// The access check of a device operation, as the errors of 503 §7.2.6 name them.
fn check_access(
    table: &BilateralTable,
    name: &ObjectName,
    permission: Permission,
) -> Result<(), ErrorCode> {
    match table.check(&Object::Device(name.clone()), permission) {
        Access::Granted => Ok(()),
        Access::UnknownObject => Err(ErrorCode::ObjectNonExistent),
        Access::NotPermitted(_) | Access::IntervalOutOfAgreement | Access::OutOfTime => {
            Err(ErrorCode::ObjectAccessDenied)
        }
    }
}
