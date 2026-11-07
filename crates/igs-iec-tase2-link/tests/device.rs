// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The devices (task C4, decision D-064): Select, Operate, the timeout, the local reset, the tags, and the owner of a VCC-specific
//! device. Each test gives the time of its inputs; nothing reads a clock.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::collections::BTreeSet;
use std::time::Duration;

use igs_iec_tase2_link::bilateral::{BilateralTable, DeviceAccess, ObjectName, Permission, Scope};
use igs_iec_tase2_link::data::ErrorCode;
use igs_iec_tase2_link::device::{
    Action, ClientId, Device, DeviceClass, DeviceState, Outcome, Tag, MAX_REASON_LEN,
};
use igs_iec_tase2_link::version::{Versions, STANDARD_VERSION};
use igs_iec_tase2_model::generated::iso503::VisibleString32;
use igs_iec_tase2_model::generated::iso802::SBOCheckBackName;

const A: ClientId = ClientId(1);
const B: ClientId = ClientId(2);

fn secs(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

fn permissions(list: &[Permission]) -> BTreeSet<Permission> {
    list.iter().copied().collect()
}

fn domain_name(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Domain("ICC1".to_string()),
        name: item.to_string(),
    }
}

fn vmd_name(item: &str) -> ObjectName {
    ObjectName {
        scope: Scope::Vmd,
        name: item.to_string(),
    }
}

fn versions() -> Versions {
    Versions {
        table_id: VisibleString32::new("BT-2026-001").unwrap(),
        tase2: STANDARD_VERSION,
    }
}

/// The table of one client: the device `name` with the given permissions, a CheckBackID of 12 and a timeout of 5 seconds.
fn table(designation: &str, name: &ObjectName, permitted: &[Permission]) -> BilateralTable {
    BilateralTable::new(designation, versions(), "ICC1").with_device(
        name.clone(),
        DeviceAccess {
            permitted: permissions(permitted),
            check_back_id: SBOCheckBackName::new(12).unwrap(),
            timeout: secs(5),
        },
    )
}

fn full(designation: &str, name: &ObjectName) -> BilateralTable {
    table(
        designation,
        name,
        &[
            Permission::Visibility,
            Permission::Select,
            Permission::Operate,
            Permission::SetTag,
        ],
    )
}

fn sbo(name: &ObjectName) -> Device {
    Device::new(name.clone(), DeviceClass::Sbo)
}

#[test]
fn a_select_arms_an_sbo_device_and_returns_its_check_back_id() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    assert_eq!(device.state(), DeviceState::Idle);
    let check_back = device.select(&table, A, secs(0)).unwrap();
    assert_eq!(check_back, SBOCheckBackName::new(12).unwrap());
    assert_eq!(device.state(), DeviceState::Armed);
    assert_eq!(device.tag().state, DeviceState::Armed);
}

#[test]
fn a_second_select_while_armed_is_refused_even_by_the_same_client() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device.select(&table, A, secs(0)).unwrap();
    assert_eq!(
        device.select(&table, A, secs(1)),
        Err(ErrorCode::TemporarilyUnavailable)
    );
    assert_eq!(
        device.select(&full("cc-b", &name), B, secs(1)),
        Err(ErrorCode::TemporarilyUnavailable)
    );
}

#[test]
fn a_select_of_a_non_sbo_device_is_refused() {
    let name = domain_name("Valve");
    let mut device = Device::new(name.clone(), DeviceClass::NonSbo);
    assert_eq!(
        device.select(&full("cc-a", &name), A, secs(0)),
        Err(ErrorCode::ObjectAccessDenied)
    );
    assert_eq!(device.state(), DeviceState::Idle);
}

#[test]
fn a_select_without_its_permission_is_refused_and_an_unknown_device_does_not_exist() {
    let name = domain_name("Breaker12");
    let mut device = sbo(&name);
    let without = table(
        "cc-a",
        &name,
        &[Permission::Visibility, Permission::Operate],
    );
    assert_eq!(
        device.select(&without, A, secs(0)),
        Err(ErrorCode::ObjectAccessDenied)
    );
    assert!(device.select(&full("cc-a", &name), A, secs(0)).is_ok());
    let other = domain_name("Nowhere");
    let mut unknown = sbo(&other);
    assert_eq!(
        unknown.select(&full("cc-a", &name), A, secs(0)),
        Err(ErrorCode::ObjectNonExistent)
    );
}

#[test]
fn an_operate_completed_with_success_returns_the_device_to_idle() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device.select(&table, A, secs(0)).unwrap();
    device.operate(&table, A, secs(1), false).unwrap();
    assert!(device.complete(Outcome::Success));
    assert_eq!(device.state(), DeviceState::Idle);
    assert_eq!(device.take_actions(), vec![Action::Success]);
    assert!(
        device.take_actions().is_empty(),
        "each action is taken once"
    );
}

#[test]
fn an_operate_completed_with_failure_signals_the_failure_action() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device.select(&table, A, secs(0)).unwrap();
    device.operate(&table, A, secs(1), false).unwrap();
    device.complete(Outcome::Failure);
    assert_eq!(device.state(), DeviceState::Idle);
    assert_eq!(device.take_actions(), vec![Action::Failure]);
}

#[test]
fn an_operate_without_a_selection_or_with_another_clients_selection_is_refused() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    assert_eq!(
        device.operate(&table, A, secs(0), false),
        Err(ErrorCode::TemporarilyUnavailable)
    );
    device.select(&table, A, secs(0)).unwrap();
    assert_eq!(
        device.operate(&full("cc-b", &name), B, secs(1), false),
        Err(ErrorCode::TemporarilyUnavailable)
    );
    // The selection is still A's: A can operate.
    assert!(device.operate(&table, A, secs(1), false).is_ok());
}

#[test]
fn a_second_operate_while_one_is_in_progress_is_refused() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device.select(&table, A, secs(0)).unwrap();
    device.operate(&table, A, secs(1), false).unwrap();
    assert_eq!(
        device.operate(&table, A, secs(2), false),
        Err(ErrorCode::TemporarilyUnavailable)
    );
}

#[test]
fn an_armed_device_returns_to_idle_at_its_timeout_and_signals_it() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device.select(&table, A, secs(10)).unwrap();
    assert_eq!(device.next_deadline(), Some(secs(15)));

    device.advance(secs(14));
    assert_eq!(
        device.state(),
        DeviceState::Armed,
        "one second before the timeout"
    );
    assert!(device.take_actions().is_empty());

    device.advance(secs(15));
    assert_eq!(device.state(), DeviceState::Idle);
    assert_eq!(device.take_actions(), vec![Action::Timeout]);
    assert_eq!(
        device.operate(&table, A, secs(16), false),
        Err(ErrorCode::TemporarilyUnavailable),
        "the selection has timed out"
    );
}

#[test]
fn a_selection_can_be_made_again_after_its_timeout() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device.select(&table, A, secs(0)).unwrap();
    device.advance(secs(5));
    assert!(device.select(&table, B, secs(6)).is_ok());
}

#[test]
fn the_timer_stops_while_an_operate_is_in_progress() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device.select(&table, A, secs(0)).unwrap();
    device.operate(&table, A, secs(1), false).unwrap();
    assert_eq!(device.next_deadline(), None);
    // The execution takes longer than the timeout: the selection does not time out during it.
    device.advance(secs(100));
    assert_eq!(device.state(), DeviceState::Armed);
    assert!(device.take_actions().is_empty());
    device.complete(Outcome::Success);
    assert_eq!(device.take_actions(), vec![Action::Success]);
}

#[test]
fn a_local_reset_returns_the_device_to_idle_and_ends_the_operate() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device.select(&table, A, secs(0)).unwrap();
    device.operate(&table, A, secs(1), false).unwrap();
    device.local_reset(None);
    assert_eq!(device.state(), DeviceState::Idle);
    assert_eq!(device.take_actions(), vec![Action::LocalReset]);
    assert!(
        !device.complete(Outcome::Success),
        "no operate is in progress after a local reset"
    );
}

#[test]
fn a_local_reset_may_set_the_tag_without_an_owner() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device.local_reset(Some(Tag::OpenAndCloseInhibit));
    let tag = device.tag();
    assert_eq!(tag.tag, Tag::OpenAndCloseInhibit);
    // Without an owner, any client may change the tag of a domain device, and a select is refused meanwhile.
    assert_eq!(
        device.select(&table, A, secs(0)),
        Err(ErrorCode::TemporarilyUnavailable)
    );
}

#[test]
fn an_open_and_close_inhibit_tag_refuses_select_and_operate() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device
        .set_tag(&table, A, Tag::OpenAndCloseInhibit, "maintenance")
        .unwrap();
    assert_eq!(
        device.select(&table, A, secs(0)),
        Err(ErrorCode::TemporarilyUnavailable)
    );
}

#[test]
fn a_close_only_inhibit_tag_refuses_only_close_and_raise_commands() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device
        .set_tag(&table, A, Tag::CloseOnlyInhibit, "")
        .unwrap();
    device.select(&table, A, secs(0)).unwrap();
    assert_eq!(
        device.operate(&table, A, secs(1), true),
        Err(ErrorCode::TemporarilyUnavailable),
        "a close or raise command is refused"
    );
    assert!(
        device.operate(&table, A, secs(1), false).is_ok(),
        "another command is allowed"
    );
}

#[test]
fn a_non_sbo_device_is_operated_directly_and_signals_its_outcome() {
    let name = domain_name("Valve");
    let table = full("cc-a", &name);
    let mut device = Device::new(name.clone(), DeviceClass::NonSbo);
    device.operate(&table, A, secs(0), false).unwrap();
    assert_eq!(device.state(), DeviceState::Idle);
    assert_eq!(
        device.operate(&table, A, secs(0), false),
        Err(ErrorCode::TemporarilyUnavailable),
        "one operate at a time"
    );
    device.complete(Outcome::Success);
    assert_eq!(device.take_actions(), vec![Action::Success]);
}

#[test]
fn an_operate_needs_its_permission() {
    let name = domain_name("Valve");
    let table = table("cc-a", &name, &[Permission::Visibility, Permission::Select]);
    let mut device = Device::new(name, DeviceClass::NonSbo);
    assert_eq!(
        device.operate(&table, A, secs(0), false),
        Err(ErrorCode::ObjectAccessDenied)
    );
}

#[test]
fn a_tag_code_is_two_bits_and_a_code_out_of_range_is_refused() {
    for tag in [Tag::NoTag, Tag::OpenAndCloseInhibit, Tag::CloseOnlyInhibit] {
        assert_eq!(Tag::from_code(tag.code()), Some(tag));
    }
    assert_eq!(Tag::NoTag.code(), 0);
    assert_eq!(Tag::OpenAndCloseInhibit.code(), 1);
    assert_eq!(Tag::CloseOnlyInhibit.code(), 2);
    assert_eq!(Tag::from_code(3), None);
}

#[test]
fn a_tag_keeps_its_reason_and_no_tag_clears_it() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    device
        .set_tag(&table, A, Tag::CloseOnlyInhibit, "crew on site")
        .unwrap();
    assert_eq!(device.tag().reason, "crew on site");
    device.set_tag(&table, A, Tag::NoTag, "ignored").unwrap();
    assert_eq!(device.tag().tag, Tag::NoTag);
    assert_eq!(device.tag().reason, "");
}

#[test]
fn a_reason_longer_than_the_text_string_is_refused() {
    let name = domain_name("Breaker12");
    let table = full("cc-a", &name);
    let mut device = sbo(&name);
    let long = "x".repeat(MAX_REASON_LEN + 1);
    assert_eq!(
        device.set_tag(&table, A, Tag::CloseOnlyInhibit, &long),
        Err(ErrorCode::TypeInconsistent)
    );
    assert_eq!(device.tag().tag, Tag::NoTag, "a refused tag is not kept");
}

#[test]
fn a_domain_device_tagged_by_one_client_may_be_changed_by_another() {
    let name = domain_name("Breaker12");
    let mut device = sbo(&name);
    device
        .set_tag(&full("cc-a", &name), A, Tag::CloseOnlyInhibit, "")
        .unwrap();
    device
        .set_tag(&full("cc-b", &name), B, Tag::NoTag, "")
        .unwrap();
    assert_eq!(device.tag().tag, Tag::NoTag);
}

#[test]
fn a_vcc_specific_device_tagged_by_one_client_is_changed_only_by_that_client() {
    let name = vmd_name("Breaker12");
    let mut device = sbo(&name);
    device
        .set_tag(&full("cc-a", &name), A, Tag::CloseOnlyInhibit, "")
        .unwrap();
    assert_eq!(
        device.set_tag(&full("cc-b", &name), B, Tag::NoTag, ""),
        Err(ErrorCode::ObjectAccessDenied)
    );
    assert_eq!(device.tag().tag, Tag::CloseOnlyInhibit);
    device
        .set_tag(&full("cc-a", &name), A, Tag::NoTag, "")
        .unwrap();
    // Once untagged, the device is free for any client.
    device
        .set_tag(&full("cc-b", &name), B, Tag::CloseOnlyInhibit, "")
        .unwrap();
}

#[test]
fn set_tag_needs_its_permission() {
    let name = domain_name("Breaker12");
    let table = table("cc-a", &name, &[Permission::Visibility, Permission::Select]);
    let mut device = sbo(&name);
    assert_eq!(
        device.set_tag(&table, A, Tag::CloseOnlyInhibit, ""),
        Err(ErrorCode::ObjectAccessDenied)
    );
}
