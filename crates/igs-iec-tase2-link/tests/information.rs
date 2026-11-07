// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The Information Message (task C3, decision D-061): the message, its buffer, its report, and the Information Message Transfer
//! Set, without I/O. The clock is the caller's: each test gives the time of its input.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::time::Duration;

use igs_iec_tase2_link::information::{
    buffer_name, InformationMessage, InformationTransferSet, MessageError, BUFFER_SIZES,
    HEADER_NAME, MAX_STREAM_OCTETS,
};
use igs_iec_tase2_model::generated::iso802::clause_6_4::{Number, ReferenceNum};
use igs_iec_tase2_model::generated::iso802::{InfoBufXX, InfoMessHeader};
use igs_iso9506_mms_codec::generated::types::Data;
use proptest::prop_assert;
use proptest::prop_assert_eq;

fn at(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

fn message(stream: Vec<u8>) -> InformationMessage {
    InformationMessage::new(7, 8, 9, stream).unwrap()
}

#[test]
fn the_buffer_is_the_smallest_size_that_holds_the_stream() {
    assert_eq!(buffer_name(0).unwrap(), "Info_Buff_64");
    assert_eq!(buffer_name(64).unwrap(), "Info_Buff_64");
    assert_eq!(buffer_name(65).unwrap(), "Info_Buff_256");
    assert_eq!(buffer_name(1024).unwrap(), "Info_Buff_1024");
    assert_eq!(buffer_name(1025).unwrap(), "Info_Buff_4096");
    assert_eq!(buffer_name(MAX_STREAM_OCTETS).unwrap(), "Info_Buff_65536");
}

#[test]
fn a_stream_longer_than_the_greatest_buffer_is_refused() {
    assert_eq!(
        InformationMessage::new(1, 1, 1, vec![0; MAX_STREAM_OCTETS + 1]),
        Err(MessageError::TooLong {
            octets: MAX_STREAM_OCTETS + 1,
            max: MAX_STREAM_OCTETS
        })
    );
    assert!(buffer_name(MAX_STREAM_OCTETS + 1).is_err());
}

#[test]
fn a_reference_outside_its_range_is_refused() {
    assert!(matches!(
        InformationMessage::new(i64::from(i32::MAX) + 1, 0, 0, vec![]),
        Err(MessageError::OutOfRange(_))
    ));
    assert!(matches!(
        InformationMessage::new(0, i64::from(i32::MIN) - 1, 0, vec![]),
        Err(MessageError::OutOfRange(_))
    ));
    assert!(InformationMessage::new(i64::from(i32::MAX), i64::from(i32::MIN), 0, vec![]).is_ok());
}

#[test]
fn the_message_keeps_its_references_and_its_stream() {
    let m = message(vec![1, 2, 3]);
    assert_eq!(m.info_reference(), 7);
    assert_eq!(m.local_reference(), 8);
    assert_eq!(m.message_id(), 9);
    assert_eq!(m.stream(), &[1, 2, 3]);
}

#[test]
fn a_report_names_the_header_then_the_padded_buffer() {
    let mut set = InformationTransferSet::start();
    let report = set
        .report(&message(vec![0xAA, 0xBB]), at(5))
        .unwrap()
        .unwrap();
    assert_eq!(report.sequence, 1);
    assert_eq!(report.at, at(5));
    assert_eq!(
        report.header.size.get(),
        2,
        "the size is the stream's length"
    );

    let variables = report.variables();
    assert_eq!(variables[0].0, HEADER_NAME);
    assert_eq!(variables[1].0, "Info_Buff_64");
    // The buffer has the size of its variable, and its valid octets are the stream.
    assert_eq!(report.buffer.0.len(), 64);
    assert_eq!(&report.buffer.0[..2], &[0xAA, 0xBB]);
    assert!(report.buffer.0[2..].iter().all(|octet| *octet == 0));
}

#[test]
fn the_variables_of_a_report_are_the_mms_values_of_its_header_and_buffer() {
    let mut set = InformationTransferSet::start();
    let report = set.report(&message(vec![5; 3]), at(1)).unwrap().unwrap();
    let variables = report.variables();
    assert!(matches!(variables[0].1, Data::Structure(ref items) if items.len() == 4));
    assert_eq!(variables[1].1, Data::OctetString(report.buffer.0.clone()));
}

#[test]
fn a_received_message_keeps_the_first_size_octets_of_its_buffer() {
    let header = InfoMessHeader {
        info_reference: ReferenceNum::new(1).unwrap(),
        localreference: ReferenceNum::new(2).unwrap(),
        message_id: ReferenceNum::new(3).unwrap(),
        size: Number::new(3).unwrap(),
    };
    let buffer = InfoBufXX(vec![10, 20, 30, 0, 0, 0, 0, 0]);
    let received = InformationMessage::from_parts(header, &buffer).unwrap();
    assert_eq!(received.stream(), &[10, 20, 30]);
    assert_eq!(received.info_reference(), 1);
    assert_eq!(received.local_reference(), 2);
    assert_eq!(received.message_id(), 3);
}

#[test]
fn a_received_size_beyond_the_buffer_or_negative_is_refused() {
    let header = |size: i64| InfoMessHeader {
        info_reference: ReferenceNum::new(0).unwrap(),
        localreference: ReferenceNum::new(0).unwrap(),
        message_id: ReferenceNum::new(0).unwrap(),
        size: Number::new(size).unwrap(),
    };
    let buffer = InfoBufXX(vec![0; 4]);
    assert_eq!(
        InformationMessage::from_parts(header(5), &buffer),
        Err(MessageError::InvalidSize { size: 5, octets: 4 })
    );
    assert_eq!(
        InformationMessage::from_parts(header(-1), &buffer),
        Err(MessageError::InvalidSize {
            size: -1,
            octets: 4
        })
    );
    // A size equal to the buffer is valid: every octet is part of the stream.
    assert!(InformationMessage::from_parts(header(4), &buffer).is_ok());
}

#[test]
fn a_received_message_round_trips_through_its_report() {
    let mut set = InformationTransferSet::start();
    let sent = message((0..100).collect());
    let report = set.report(&sent, at(2)).unwrap().unwrap();
    let received = InformationMessage::from_parts(report.header, &report.buffer).unwrap();
    assert_eq!(received, sent);
}

#[test]
fn a_transfer_set_reports_only_while_it_is_enabled() {
    let mut set = InformationTransferSet::start();
    assert!(set.is_enabled());
    assert!(set.report(&message(vec![1]), at(1)).unwrap().is_some());

    set.stop();
    assert!(!set.is_enabled());
    assert_eq!(set.report(&message(vec![2]), at(2)).unwrap(), None);
}

#[test]
fn the_reports_of_a_transfer_set_are_numbered_from_one() {
    let mut set = InformationTransferSet::start();
    let first = set.report(&message(vec![1]), at(1)).unwrap().unwrap();
    let second = set.report(&message(vec![2]), at(2)).unwrap().unwrap();
    assert_eq!((first.sequence, second.sequence), (1, 2));
}

#[test]
fn a_new_start_after_a_stop_enables_the_transfer_set_again() {
    let mut set = InformationTransferSet::start();
    set.stop();
    let mut again = InformationTransferSet::start();
    assert!(again.is_enabled());
    assert!(again.report(&message(vec![3]), at(3)).unwrap().is_some());
}

#[test]
fn the_greatest_stream_is_the_greatest_buffer() {
    assert_eq!(BUFFER_SIZES.last(), Some(&MAX_STREAM_OCTETS));
}

proptest::proptest! {
    /// Any stream within the greatest buffer round-trips: the report, read back as a received message, is the message sent.
    #[test]
    fn a_message_of_any_stream_round_trips_through_its_report(
        stream in proptest::collection::vec(proptest::arbitrary::any::<u8>(), 0..2048),
        references in (proptest::arbitrary::any::<i32>(), proptest::arbitrary::any::<i32>(), proptest::arbitrary::any::<i32>()),
    ) {
        let (info, local, id) = references;
        let sent = InformationMessage::new(i64::from(info), i64::from(local), i64::from(id), stream.clone()).unwrap();
        let report = InformationTransferSet::start().report(&sent, at(1)).unwrap().unwrap();
        // The buffer is a size of BUFFER_SIZES, at least the stream, and its first `Size` octets are the stream.
        prop_assert!(BUFFER_SIZES.contains(&report.buffer.0.len()));
        prop_assert!(report.buffer.0.len() >= stream.len());
        let received = InformationMessage::from_parts(report.header.clone(), &report.buffer).unwrap();
        prop_assert_eq!(received, sent);
    }

    /// Whatever the size in a received header, a message is built only when the size is a count of octets of the buffer.
    #[test]
    fn a_received_size_is_valid_only_within_its_buffer(size in -4i64..40, octets in 0usize..32) {
        let header = InfoMessHeader {
            info_reference: ReferenceNum::new(0).unwrap(),
            localreference: ReferenceNum::new(0).unwrap(),
            message_id: ReferenceNum::new(0).unwrap(),
            size: Number::new(size).unwrap(),
        };
        let buffer = InfoBufXX(vec![0; octets]);
        let valid = usize::try_from(size).is_ok_and(|valid| valid <= octets);
        prop_assert_eq!(InformationMessage::from_parts(header, &buffer).is_ok(), valid);
    }
}
