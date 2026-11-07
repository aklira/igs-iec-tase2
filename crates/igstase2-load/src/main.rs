// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! `igstase2-load`: the load driver of the TASE.2 server (task S3). Many associations read every point of a server, write a
//! few, and (locally) measure the latency of a transfer report. `--local` runs the server in the process; `--server ADDR`
//! drives a server elsewhere, such as the bench's device. Usage: `igstase2-load --help`, and `docs/load-test.md`.

mod local;
mod stats;
mod workload;

use std::process::ExitCode;
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;
use igs_iec_tase2::TaseServer;

use crate::stats::Recorder;
use crate::workload::{run_client, Options};

/// The command line.
#[derive(Debug, Parser)]
#[command(
    name = "igstase2-load",
    version,
    about = "Load driver of the TASE.2 server: many associations read and write many points, with latencies measured."
)]
struct Cli {
    /// The address of a server to drive (`IP:PORT`). Not with --local.
    #[arg(long, conflicts_with = "local", required_unless_present = "local")]
    server: Option<String>,
    /// Runs the server in this process, with --points points, and measures the report latency too.
    #[arg(long)]
    local: bool,
    /// The number of points of the server (ICC1/P00000 ...). The table of the clients names them all.
    #[arg(long, default_value_t = 10_000, value_parser = clap::value_parser!(u64).range(2..=100_000))]
    points: u64,
    /// The number of associations, all running at once.
    #[arg(long, default_value_t = 4, value_parser = clap::value_parser!(u64).range(1..=256))]
    clients: u64,
    /// How many times each client reads every point.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u64).range(1..=100))]
    rounds: u64,
    /// The number of points in one read.
    #[arg(long, default_value_t = 100, value_parser = clap::value_parser!(u64).range(1..=1000))]
    chunk: u64,
    /// The number of writes of each client.
    #[arg(long, default_value_t = 10, value_parser = clap::value_parser!(u64).range(0..=10_000))]
    writes: u64,
    /// How long to wait for a report, in seconds.
    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u64).range(1..=600))]
    timeout_s: u64,
}

fn as_usize(value: u64) -> usize {
    usize::try_from(value).unwrap_or(usize::MAX)
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let options = Options {
        points: as_usize(cli.points),
        chunk: as_usize(cli.chunk),
        rounds: as_usize(cli.rounds),
        writes: as_usize(cli.writes),
        report: cli.local,
        timeout: std::time::Duration::from_secs(cli.timeout_s),
    };
    let started = Instant::now();
    let (address, server) = if cli.local {
        let (config, vmd) = match local::server(options.points) {
            Ok(pair) => pair,
            Err(error) => {
                eprintln!("igstase2-load: the local server cannot be built: {error}");
                return ExitCode::from(1);
            }
        };
        match TaseServer::bind("127.0.0.1:0", config, vmd).await {
            Ok(server) => {
                let server = Arc::new(server);
                (server.local_address().to_string(), Some(server))
            }
            Err(error) => {
                eprintln!("igstase2-load: the local server does not listen: {error}");
                return ExitCode::from(1);
            }
        }
    } else {
        (cli.server.clone().unwrap_or_default(), None)
    };

    let tasks: Vec<_> = (0..as_usize(cli.clients))
        .map(|index| {
            tokio::spawn(run_client(
                address.clone(),
                index,
                options.clone(),
                server.clone(),
            ))
        })
        .collect();
    let mut total = Recorder::default();
    for task in tasks {
        match task.await {
            Ok(recorder) => total.merge(recorder),
            Err(_) => total.errors = total.errors.saturating_add(1),
        }
    }
    let elapsed = started.elapsed();

    println!("{}", total.summary());
    println!(
        "elapsed {:.3} s; server {}; points {}; clients {}; rounds {}; chunk {}; writes {}",
        elapsed.as_secs_f64(),
        if cli.local { "local" } else { address.as_str() },
        options.points,
        cli.clients,
        options.rounds,
        options.chunk,
        options.writes,
    );

    let mut failed = total.errors > 0 || total.stray_reports > 0;
    if cli.local && total.count("report") == 0 {
        eprintln!("igstase2-load: no report was measured");
        failed = true;
    }
    if let Some(server) = server {
        if let Ok(server) = Arc::try_unwrap(server) {
            server.shutdown().await;
        }
    }
    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
