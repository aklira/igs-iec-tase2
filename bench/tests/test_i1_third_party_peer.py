# SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
#
# SPDX-License-Identifier: Apache-2.0
"""I1: the interop of the client with a third-party TASE.2 server, the server of the peer's example configuration (service
peer-example). The peer is licensed and not in this repository: its domain and its table identifier come from the environment
(THIRD_PARTY_PEER_DOMAIN, THIRD_PARTY_PEER_TABLE_ID), and the tests are skipped when they are not set. The client's options
(transport selectors, the 2002 application context, the called AP-title and the AE-qualifiers) are those of the peer's example.
"""

from __future__ import annotations

import os

import pytest

from igs_bench.compose import Bench

PEER = "172.31.110.14:102"
CLIENT_SERVICE = "client-tase2"
CLIENT = "/usr/local/bin/igstase2-client"
DOMAIN = os.environ.get("THIRD_PARTY_PEER_DOMAIN")
TABLE_ID = os.environ.get("THIRD_PARTY_PEER_TABLE_ID")

pytestmark = pytest.mark.skipif(
    not (DOMAIN and TABLE_ID), reason="set THIRD_PARTY_PEER_DOMAIN and THIRD_PARTY_PEER_TABLE_ID to run the peer's tests"
)


@pytest.fixture(scope="module")
def bench():
    instance = Bench("i1-third-party-peer.pcap")
    instance.down()
    instance.up("peer-example", CLIENT_SERVICE)
    try:
        yield instance
    finally:
        instance.down()


def run_client(bench: Bench, *args: str, timeout: float = 90):
    return bench.exec(
        CLIENT_SERVICE,
        CLIENT,
        "-s",
        PEER,
        "--ap-title",
        "1.1.1.999",
        "--table-id",
        TABLE_ID,
        "--domain",
        DOMAIN,
        "--calling-tsap",
        "0001",
        "--called-tsap",
        "0001",
        "--legacy-application-context",
        "--called-ap-title",
        "1.1.1.999",
        "--calling-ae-qualifier",
        "12",
        "--called-ae-qualifier",
        "12",
        *args,
        timeout=timeout,
        check=False,
    )


def test_our_client_reads_a_point_of_the_peer_server(bench):
    result = run_client(bench, "get", "Discrete1")
    assert result.returncode == 0, result.stdout + result.stderr
    assert result.stdout.strip().startswith("Discrete1: "), result.stdout
    assert "error" not in result.stdout, result.stdout


@pytest.fixture(scope="module")
def reverse_bench():
    """Our server, configured for the peer's example client, and that client."""
    instance = Bench("i1-reverse.pcap")
    instance.down()
    instance.up("device-interop", "peer-client-example")
    try:
        yield instance
    finally:
        instance.down()


def test_the_example_client_of_the_peer_associates_with_our_server(reverse_bench):
    # The client's own log reports the association: it is accepted, and the client is connected. Its later requests name
    # objects that the configuration of our server does not hold; that is not checked here.
    result = reverse_bench.exec(
        "peer-client-example", "timeout", "20", "stdbuf", "-o0", "-e0", "/opt/peer/example-client", "172.31.110.15", timeout=60, check=False
    )
    output = result.stdout + result.stderr
    assert "association accepted" in output, output[-2000:]
    assert "Connected." in output, output[-2000:]
