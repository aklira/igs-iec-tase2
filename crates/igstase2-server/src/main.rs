// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! `igstase2-server`: a demonstration TASE.2 server (task S2). It serves the points, data sets and transfer sets of a TOML
//! file to the clients of its Bilateral Tables, and makes the changes that the file describes by itself. Format and usage:
//! `docs/igstase2-server.md`.

mod config;
mod simulate;

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use igs_iec_tase2::TaseServer;

use crate::config::{build, parse, Simulation};
use crate::simulate::Stepper;

/// The command line.
#[derive(Debug, Parser)]
#[command(
    name = "igstase2-server",
    version,
    about = "A demonstration TASE.2 server: points from a TOML file, with simulated changes. The bench's device under test."
)]
struct Cli {
    /// The address to listen on (`IP:PORT`; TASE.2 uses port 102).
    #[arg(long, default_value = "127.0.0.1:102")]
    listen: String,
    /// The configuration file (format: docs/igstase2-server.md).
    #[arg(long)]
    config: PathBuf,
    /// The associations at once; the connections beyond the limit are closed.
    #[arg(long, default_value_t = 64, value_parser = clap::value_parser!(u32).range(1..=10_000))]
    max_associations: u32,
}

/// Runs one simulation: a change of its point at each period, until the server stops.
async fn run(simulation: Simulation, server: Arc<TaseServer>) {
    let mut stepper = Stepper::new(simulation.clone());
    let mut ticker = tokio::time::interval(simulation.period.max(Duration::from_millis(1)));
    loop {
        ticker.tick().await;
        let value = stepper.next();
        if server
            .set_value(simulation.name.clone(), value)
            .await
            .is_err()
        {
            return;
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let text = match std::fs::read_to_string(&cli.config) {
        Ok(text) => text,
        Err(error) => {
            eprintln!(
                "igstase2-server: cannot read {}: {error}",
                cli.config.display()
            );
            return ExitCode::from(2);
        }
    };
    let config = match parse(&text) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("igstase2-server: {}: {error}", cli.config.display());
            return ExitCode::from(2);
        }
    };
    let built = match build(&config, cli.max_associations as usize) {
        Ok(built) => built,
        Err(error) => {
            eprintln!("igstase2-server: {}: {error}", cli.config.display());
            return ExitCode::from(2);
        }
    };
    let server = match TaseServer::bind(&cli.listen, built.server, built.vmd).await {
        Ok(server) => Arc::new(server),
        Err(error) => {
            eprintln!(
                "igstase2-server: cannot listen on {}: {error:?}",
                cli.listen
            );
            return ExitCode::from(1);
        }
    };
    println!("igstase2-server: listening on {}", server.local_address());
    let simulations: Vec<_> = built
        .simulations
        .into_iter()
        .map(|simulation| tokio::spawn(run(simulation, Arc::clone(&server))))
        .collect();
    if let Err(error) = tokio::signal::ctrl_c().await {
        eprintln!("igstase2-server: the Ctrl-C signal is not available: {error}");
    }
    for task in simulations {
        task.abort();
        let _ = task.await;
    }
    match Arc::try_unwrap(server) {
        Ok(server) => server.shutdown().await,
        Err(_) => eprintln!("igstase2-server: the server is still in use at its stop"),
    }
    ExitCode::SUCCESS
}
