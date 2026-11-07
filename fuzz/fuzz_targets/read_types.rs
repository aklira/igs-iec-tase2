// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! Reads random bytes as MMS data, and reads that data as each type of the model (task M6). A type that accepts a value
//! must read back the same value once it writes it: a failure is a panic, which the fuzzer reports as a crash.

#![no_main]

use std::fmt::Debug;

use igs_iec_tase2_model::builtin::{Integer16, Integer32, MmsIdentifier, Unsigned8};
use igs_iec_tase2_model::generated::iso503::{
    DSConditions, DSTransferSet, IMTransferSet, MMSObjectName, SupportedFeatures, TATransferSet,
    TAConditions, TSConditions, TSTransferSet, TimeStampExtended,
};
use igs_iec_tase2_model::generated::iso802::{
    COVCounter, ControlCommand, ControlConfig, ControlSetpointDiscrete, ControlSetpointReal, DOActual,
    DOCancel, DONewRevSched, DataFlags, DataState, DataStateSupplemental, EventFlags, PackedFlags,
    PackedProtectionEvent, SBO, SBOCheckBackName, SelectState, SingleFlags, SingleProtectionEvent, TagFlags,
    TagValue, TextString,
};
use igs_iec_tase2_model::generated::iso503::DSTransferSetBlockData;
use igs_iec_tase2_model::mms::MmsValue;
use igs_iso9506_mms_codec::{asn, generated::types::Data};
use libfuzzer_sys::fuzz_target;

/// Reads `data` as `T`; a value that is read is read back the same once written.
fn check<T: MmsValue + PartialEq + Debug>(data: &Data) {
    if let Ok(value) = T::from_mms(data) {
        let written = value.to_mms();
        let again = T::from_mms(&written);
        assert!(
            again.as_ref().ok() == Some(&value),
            "a value read from {data:?} is not read back after writing: {again:?}"
        );
    }
}

/// Reads `data` as each type of the model.
fn check_every_type(data: &Data) {
    check::<Integer16>(data);
    check::<Integer32>(data);
    check::<Unsigned8>(data);
    check::<COVCounter>(data);
    check::<MmsIdentifier>(data);
    check::<TimeStampExtended>(data);
    check::<TextString>(data);
    check::<DataFlags>(data);
    check::<DataState>(data);
    check::<DataStateSupplemental>(data);
    check::<SingleFlags>(data);
    check::<EventFlags>(data);
    check::<PackedFlags>(data);
    check::<TagFlags>(data);
    check::<SelectState>(data);
    check::<SBO>(data);
    check::<TagValue>(data);
    check::<ControlConfig>(data);
    check::<ControlCommand>(data);
    check::<ControlSetpointReal>(data);
    check::<ControlSetpointDiscrete>(data);
    check::<SBOCheckBackName>(data);
    check::<SingleProtectionEvent>(data);
    check::<PackedProtectionEvent>(data);
    check::<DONewRevSched>(data);
    check::<DOCancel>(data);
    check::<DOActual>(data);
    check::<DSConditions>(data);
    check::<TSConditions>(data);
    check::<TAConditions>(data);
    check::<SupportedFeatures>(data);
    check::<DSTransferSetBlockData>(data);
    check::<MMSObjectName>(data);
    check::<DSTransferSet>(data);
    check::<TSTransferSet>(data);
    check::<TATransferSet>(data);
    check::<IMTransferSet>(data);
}

fuzz_target!(|bytes: &[u8]| {
    let Ok(data) = asn::decode_value::<Data>(bytes) else {
        return;
    };
    check_every_type(&data);
    // The data read from the bytes, written and read again, is read the same way.
    if let Ok(encoded) = asn::encode_value::<Data>(&data) {
        if let Ok(again) = asn::decode_value::<Data>(&encoded) {
            check_every_type(&again);
        }
    }
});
