// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! Code generation and maintenance of the workspace.
//!
//! `cargo xtask codegen` reads the data files of the licensed standards (`docs/spec`, a local symlink, not in
//! CI) and writes `crates/igs-iec-tase2-model/src/generated/`. `--check` regenerates in memory and fails if the
//! committed files differ, so that a stale file is caught.

mod codegen;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("codegen") => match codegen::run(args.get(1).map(String::as_str) == Some("--check")) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("xtask codegen: {error}");
                ExitCode::FAILURE
            }
        },
        _ => {
            eprintln!("usage: cargo xtask codegen [--check]");
            ExitCode::FAILURE
        }
    }
}
