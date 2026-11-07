// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The time types of TASE.2 (IEC 60870-6-503 §8.2.4 to §8.2.6), and their conversion to and from `SystemTime`.
//!
//! A GMT time is an integer of 32 bits: the seconds since 1 January 1970 at 00:00:00 GMT (clause 8.2.4; the types for
//! minutes, hours and days count in their own unit, and are integers of the same width). A time stamp with
//! milliseconds (clause 8.2.5) is the GMT seconds and a milliseconds component of 16 bits. Its milliseconds are the
//! part of the second: 0 to 999 (decision D-011).

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use igs_iso9506_mms_codec::generated::types::Data;

use crate::builtin::{Integer16, Integer32};
use crate::generated::iso503::TimeStampExtended;
use crate::mms::{MmsValue, ValueError};
use crate::value::OutOfRange;

/// The greatest milliseconds of a time stamp: the part of a second (decision D-011).
pub const MAX_MILLISECONDS: i64 = 999;

/// The milliseconds since 1 January 1970, rounded down: negative before that date.
fn milliseconds_since_epoch(time: SystemTime) -> i128 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(after) => i128::try_from(after.as_millis()).unwrap_or(i128::MAX),
        Err(before) => {
            let before = before.duration();
            let whole = i128::try_from(before.as_millis()).unwrap_or(i128::MAX);
            // Rounding down a negative time takes the next millisecond further from the epoch, when there is a part left.
            let part = i128::from(before.subsec_nanos().checked_rem(1_000_000).unwrap_or(0) != 0);
            0i128.saturating_sub(whole.saturating_add(part))
        }
    }
}

/// The GMT seconds of a time, rounded down.
pub fn gmt_seconds_from_system_time(time: SystemTime) -> Result<Integer32, ValueError> {
    TimeStampExtended::from_system_time(time).map(|stamp| stamp.gmt_based_s)
}

/// The time of GMT seconds: the seconds since 1 January 1970 at 00:00:00 GMT.
pub fn system_time_from_gmt(seconds: Integer32) -> SystemTime {
    let seconds = seconds.get();
    let moved = if seconds >= 0 {
        UNIX_EPOCH.checked_add(Duration::from_secs(u64::try_from(seconds).unwrap_or(0)))
    } else {
        UNIX_EPOCH.checked_sub(Duration::from_secs(seconds.unsigned_abs()))
    };
    moved.unwrap_or(UNIX_EPOCH)
}

impl TimeStampExtended {
    /// The time stamp of GMT seconds and milliseconds, when the milliseconds are the part of a second.
    pub fn new(gmt_based_s: Integer32, milliseconds: Integer16) -> Result<Self, ValueError> {
        if !(0..=MAX_MILLISECONDS).contains(&milliseconds.get()) {
            return Err(ValueError::Range(OutOfRange {
                value: i128::from(milliseconds.get()),
                min: 0,
                max: i128::from(MAX_MILLISECONDS),
            }));
        }
        Ok(Self {
            gmt_based_s,
            milliseconds,
        })
    }

    /// The time stamp of a time, rounded down to the millisecond, when its seconds are in the range of GMT seconds.
    pub fn from_system_time(time: SystemTime) -> Result<Self, ValueError> {
        let total = milliseconds_since_epoch(time);
        let seconds = i64::try_from(total.div_euclid(1000)).map_err(|_| {
            ValueError::Range(OutOfRange {
                value: total.div_euclid(1000),
                min: i128::from(Integer32::MIN),
                max: i128::from(Integer32::MAX),
            })
        })?;
        let milliseconds = i64::try_from(total.rem_euclid(1000)).unwrap_or(0);
        Self::new(Integer32::new(seconds)?, Integer16::new(milliseconds)?)
    }

    /// The time of this time stamp.
    pub fn to_system_time(&self) -> SystemTime {
        let base = system_time_from_gmt(self.gmt_based_s);
        let milliseconds = u64::try_from(self.milliseconds.get()).unwrap_or(0);
        base.checked_add(Duration::from_millis(milliseconds))
            .unwrap_or(base)
    }
}

impl MmsValue for TimeStampExtended {
    fn to_mms(&self) -> Data {
        Data::Structure(vec![self.gmt_based_s.to_mms(), self.milliseconds.to_mms()])
    }

    fn from_mms(data: &Data) -> Result<Self, ValueError> {
        match data {
            Data::Structure(items) => match items.as_slice() {
                [seconds, milliseconds] => Self::new(
                    Integer32::from_mms(seconds)?,
                    Integer16::from_mms(milliseconds)?,
                ),
                _ => Err(ValueError::Format(
                    "a time stamp with milliseconds is a structure of two components",
                )),
            },
            other => Err(ValueError::kind_of("structure", other)),
        }
    }
}
