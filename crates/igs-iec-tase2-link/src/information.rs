// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The Information Message (IEC 60870-6-503 §5.3.9, §6.8, §7.2.5.4 and §7.2.5.5.4; IEC 60870-6-802 §6.4 and §7.3), sans I/O.
//!
//! The server reports a message as an MMS Information Report of two named variables, in this order: the header (`Info_Mess_Header`,
//! of type InfoMessHeader) and the buffer (`Info_Buff_XX`, of type InfoBufXX, where XX is the size of the buffer in octets). The
//! buffer is at least as large as the stream, and the `Size` of the header says how many of its octets are valid.
//!
//! The condition that triggers a report is a local matter (§7.2.5.4.1): the caller calls `report` when its condition occurs. The
//! time `at` is given by the caller, as for the other transfer sets. Nothing here reads a clock or sends a message.

use std::time::Duration;

use igs_iec_tase2_model::generated::iso802::clause_6_4::{Number, ReferenceNum};
use igs_iec_tase2_model::generated::iso802::{InfoBufXX, InfoMessHeader};
use igs_iec_tase2_model::mms::MmsValue;
use igs_iec_tase2_model::value::OutOfRange;
use igs_iso9506_mms_codec::generated::types::Data;

/// The name of the variable of the header of a message (IEC 60870-6-503 §8.5).
pub const HEADER_NAME: &str = "Info_Mess_Header";

/// The sizes of the buffers, in octets. 6-802 §6.4 gives 64, 256 and 1024 as examples and does not list the sizes; the list
/// extends them by a factor of four (decision D-062).
pub const BUFFER_SIZES: [usize; 6] = [64, 256, 1024, 4096, 16384, 65536];

/// The greatest stream of a message: the greatest buffer.
pub const MAX_STREAM_OCTETS: usize = 65536;

/// Why a message or its report is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageError {
    /// A reference, or the size, is outside the range of its type.
    OutOfRange(OutOfRange),
    /// The stream is longer than the greatest buffer.
    TooLong {
        /// The number of octets offered.
        octets: usize,
        /// The greatest number of octets.
        max: usize,
    },
    /// The `Size` of a received header is negative, or greater than the buffer it describes.
    InvalidSize {
        /// The size in the header.
        size: i64,
        /// The number of octets of the buffer.
        octets: usize,
    },
}

impl From<OutOfRange> for MessageError {
    fn from(error: OutOfRange) -> Self {
        Self::OutOfRange(error)
    }
}

/// The size of the buffer that holds `octets`: the smallest of [`BUFFER_SIZES`] that is at least `octets`.
fn buffer_size(octets: usize) -> Result<usize, MessageError> {
    BUFFER_SIZES
        .iter()
        .copied()
        .find(|size| *size >= octets)
        .ok_or(MessageError::TooLong {
            octets,
            max: MAX_STREAM_OCTETS,
        })
}

/// The name of the buffer variable that holds `octets` (`Info_Buff_XX`, with XX the size of the smallest buffer that holds them).
pub fn buffer_name(octets: usize) -> Result<String, MessageError> {
    Ok(format!("Info_Buff_{}", buffer_size(octets)?))
}

/// An Information Message to send: its references and its stream of octets.
#[derive(Debug, Clone, PartialEq)]
pub struct InformationMessage {
    header: InfoMessHeader,
    stream: Vec<u8>,
}

impl InformationMessage {
    /// A message with the given references and stream. Its `Size` is the length of the stream.
    pub fn new(
        info_reference: i64,
        local_reference: i64,
        message_id: i64,
        stream: Vec<u8>,
    ) -> Result<Self, MessageError> {
        if stream.len() > MAX_STREAM_OCTETS {
            return Err(MessageError::TooLong {
                octets: stream.len(),
                max: MAX_STREAM_OCTETS,
            });
        }
        let size = i64::try_from(stream.len()).map_err(|_| MessageError::TooLong {
            octets: stream.len(),
            max: MAX_STREAM_OCTETS,
        })?;
        Ok(Self {
            header: InfoMessHeader {
                info_reference: ReferenceNum::new(info_reference)?,
                localreference: ReferenceNum::new(local_reference)?,
                message_id: ReferenceNum::new(message_id)?,
                size: Number::new(size)?,
            },
            stream,
        })
    }

    /// The message that a received header and buffer describe: the first `Size` octets of the buffer are its stream.
    pub fn from_parts(header: InfoMessHeader, buffer: &InfoBufXX) -> Result<Self, MessageError> {
        let size = header.size.get();
        let octets = buffer.0.len();
        let valid = usize::try_from(size)
            .ok()
            .filter(|valid| *valid <= octets)
            .ok_or(MessageError::InvalidSize { size, octets })?;
        let stream = buffer
            .0
            .get(..valid)
            .ok_or(MessageError::InvalidSize { size, octets })?
            .to_vec();
        Ok(Self { header, stream })
    }

    /// The Info Reference.
    pub fn info_reference(&self) -> i64 {
        self.header.info_reference.get()
    }

    /// The Local Reference.
    pub fn local_reference(&self) -> i64 {
        self.header.localreference.get()
    }

    /// The MessageId.
    pub fn message_id(&self) -> i64 {
        self.header.message_id.get()
    }

    /// The Info Stream: the valid octets.
    pub fn stream(&self) -> &[u8] {
        &self.stream
    }

    /// The buffer of the message: the stream, padded with zeros to the size of its buffer variable.
    fn padded(&self) -> Result<(String, InfoBufXX), MessageError> {
        let size = buffer_size(self.stream.len())?;
        let name = format!("Info_Buff_{size}");
        let mut octets = self.stream.clone();
        octets.resize(size, 0);
        Ok((name, InfoBufXX(octets)))
    }
}

/// A Transfer Report of an Information Message Transfer Set: the header and the buffer, as the MMS Information Report carries them.
#[derive(Debug, Clone, PartialEq)]
pub struct InformationReport {
    /// The number of the report on this transfer set, from 1.
    pub sequence: u32,
    /// The time of the report.
    pub at: Duration,
    /// The header of the message.
    pub header: InfoMessHeader,
    /// The name of the buffer variable (`Info_Buff_XX`).
    pub buffer_name: String,
    /// The buffer, whose first `Size` octets are valid.
    pub buffer: InfoBufXX,
}

impl InformationReport {
    /// The named variables of the report, in the order of 503 §7.2.5.5.4: the header, then the buffer.
    pub fn variables(&self) -> [(String, Data); 2] {
        [
            (HEADER_NAME.to_string(), self.header.to_mms()),
            (self.buffer_name.clone(), self.buffer.to_mms()),
        ]
    }
}

/// An Information Message Transfer Set (IEC 60870-6-503 §5.3.10.5), for one association. It is ENABLED by the Start Transfer
/// and DISABLED by the Stop Transfer; while it is ENABLED, each message whose condition occurs is reported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InformationTransferSet {
    enabled: bool,
    sequence: u32,
}

impl InformationTransferSet {
    /// The Start Transfer (§7.2.5.4.1): the transfer set is ENABLED, and the monitoring begins at once.
    pub fn start() -> Self {
        Self {
            enabled: true,
            sequence: 0,
        }
    }

    /// Whether the transfer set is ENABLED.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// The Stop Transfer: the transfer set is DISABLED, and no further message is reported.
    pub fn stop(&mut self) {
        self.enabled = false;
    }

    /// The message whose condition occurred at `at`: its report while the transfer set is ENABLED, nothing otherwise.
    pub fn report(
        &mut self,
        message: &InformationMessage,
        at: Duration,
    ) -> Result<Option<InformationReport>, MessageError> {
        if !self.enabled {
            return Ok(None);
        }
        let (buffer_name, buffer) = message.padded()?;
        self.sequence = self.sequence.saturating_add(1);
        Ok(Some(InformationReport {
            sequence: self.sequence,
            at,
            header: message.header.clone(),
            buffer_name,
            buffer,
        }))
    }
}
