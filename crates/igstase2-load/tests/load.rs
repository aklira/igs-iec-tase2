// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The load driver (task S3), run as a process. The small runs check the driver's own correctness; the full run (10 000 points,
//! several associations) is ignored by default: `cargo test -p igstase2-load -- --ignored --nocapture`.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use std::process::Command;

fn load(args: &[&str]) -> (Option<i32>, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_igstase2-load"))
        .args(args)
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn a_small_local_run_reads_every_point_and_measures_one_report() {
    let (status, out, err) = load(&[
        "--local",
        "--points",
        "300",
        "--clients",
        "3",
        "--rounds",
        "1",
        "--writes",
        "3",
    ]);
    assert_eq!(status, Some(0), "{out}{err}");
    assert!(
        out.contains("read             n=9 "),
        "three clients, three chunks of one hundred each: {out}"
    );
    assert!(out.contains("report           n=1 "), "{out}");
    assert!(out.contains("errors=0 stray_reports=0"), "{out}");
}

#[test]
fn a_run_needs_a_server_or_the_local_one_and_not_both() {
    let (neither, _, _) = load(&["--points", "10"]);
    assert_eq!(neither, Some(2), "a server or --local is required");
    let (both, _, _) = load(&["--local", "--server", "127.0.0.1:102"]);
    assert_eq!(both, Some(2), "a server and --local are exclusive");
}

#[test]
#[ignore = "the full run: 10 000 points and several associations; run with --ignored"]
fn the_full_local_run_reads_10_000_points_for_eight_associations() {
    let (status, out, err) = load(&[
        "--local",
        "--points",
        "10000",
        "--clients",
        "8",
        "--rounds",
        "1",
        "--writes",
        "10",
    ]);
    println!("{out}");
    assert_eq!(status, Some(0), "{out}{err}");
    assert!(
        out.contains("read             n=800 "),
        "eight clients, one hundred chunks each: {out}"
    );
    assert!(out.contains("errors=0 stray_reports=0"), "{out}");
}
