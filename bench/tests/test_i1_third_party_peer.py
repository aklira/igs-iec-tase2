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
import re

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



@pytest.fixture(scope="module")
def peer_bench():
    """The data operations: a bench of their own, started after the reverse tests (which stop every service they start)."""
    instance = Bench("i1-data-operations.pcap")
    instance.down()
    instance.up("peer-example", CLIENT_SERVICE)
    try:
        yield instance
    finally:
        instance.down()


# The data operations, in our client's direction. The names of the peer's objects are not written here: the tests list them
# (GetNameList) and use what the peer answers.


def lines(result) -> list[str]:
    return [line.strip() for line in result.stdout.splitlines() if line.strip()]


def listed_names(peer_bench: Bench) -> list[str]:
    result = run_client(peer_bench, "names")
    assert result.returncode == 0, result.stdout + result.stderr
    return [line.strip() for line in result.stdout.splitlines() if line.strip()]


def is_number(answer: str) -> bool:
    """Whether the answer of a point is a plain number (not an error, nor a structure)."""
    return re.fullmatch(r"-?\d+(\.\d+)?", answer) is not None


def readable_values(peer_bench: Bench, names: list[str]) -> dict[str, str]:
    """The answer of each name of the peer: its value, or the error it answers."""
    result = run_client(peer_bench, "get", *names)
    assert result.returncode == 0, result.stdout + result.stderr
    answers = {}
    for line in result.stdout.splitlines():
        name, separator, answer = line.partition(": ")
        if separator and name in names:
            answers[name] = answer
    return answers


def test_the_peer_lists_its_variables_and_answers_each_of_them(peer_bench):
    names = listed_names(peer_bench)
    assert names, "the peer lists no variable of the domain"
    answers = readable_values(peer_bench, names)
    assert set(answers) == set(names), (names, answers)


def test_a_data_set_of_two_peer_variables_is_created_read_and_deleted(peer_bench):
    answers = readable_values(peer_bench, listed_names(peer_bench))
    members = [name for name, answer in answers.items() if is_number(answer)][:2]
    assert len(members) == 2, answers
    try:
        created = run_client(peer_bench, "create", "BenchDS", *members)
        assert created.returncode == 0, created.stdout + created.stderr
        assert lines(run_client(peer_bench, "members", "BenchDS")) == members
        read = run_client(peer_bench, "read", "BenchDS")
        assert read.returncode == 0, read.stdout + read.stderr
        assert len(lines(read)) == len(members), read.stdout
    finally:
        deleted = run_client(peer_bench, "delete", "BenchDS")
        assert deleted.returncode == 0 and "deleted 1" in deleted.stdout, deleted.stdout + deleted.stderr


def test_a_write_the_peer_does_not_permit_is_answered_with_an_error(peer_bench):
    answers = readable_values(peer_bench, listed_names(peer_bench))
    # The first point whose value is a plain number: the write of an integer to it is the case of the peer's table.
    number = next((name for name, answer in answers.items() if is_number(answer)), None)
    assert number is not None, answers
    result = run_client(peer_bench, "set", f"{number}=1")
    assert result.returncode == 0, result.stdout + result.stderr
    assert f"{number}: error " in result.stdout, result.stdout


def test_the_data_sets_of_the_domain_are_listed(peer_bench):
    result = run_client(peer_bench, "names", "--data-sets")
    assert result.returncode == 0, result.stdout + result.stderr
