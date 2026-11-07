// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The harness of the reference peer (task F4): an MMS association from the peer's stack to a server, then one
//! confirmed service (identify). Usage: `ref-peer <host:port>`.

use std::net::SocketAddr;
use std::process::ExitCode;

use rusty_mms_service::{create_mms_service_client, MmsServiceConnectionParameters};

#[tokio::main]
async fn main() -> ExitCode {
    let Some(address) = std::env::args().nth(1) else {
        eprintln!("usage: ref-peer <host:port>");
        return ExitCode::FAILURE;
    };
    let address: SocketAddr = match address.parse() {
        Ok(address) => address,
        Err(error) => {
            eprintln!("ref-peer: {address}: {error}");
            return ExitCode::FAILURE;
        }
    };
    let parameters = MmsServiceConnectionParameters::default();
    let mut client = match create_mms_service_client(address, parameters).await {
        Ok(client) => client,
        Err(error) => {
            eprintln!("ref-peer: association failed: {error:?}");
            return ExitCode::FAILURE;
        }
    };
    println!("ref-peer: association established with {address}");
    match client.identify().await {
        Ok(identity) => {
            println!("ref-peer: identity: {identity:?}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("ref-peer: identify failed: {error:?}");
            ExitCode::FAILURE
        }
    }
}
