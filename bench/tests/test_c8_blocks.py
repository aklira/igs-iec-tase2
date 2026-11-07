# SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
#
# SPDX-License-Identifier: Apache-2.0
"""C8: the client interop matrix of the blocks 2, 4 and 5 (decisions D-060, D-061, D-064, D-065, D-066). The client under test
(igstase2-client, bench/client) runs one command per invocation, against the device under test (igstase2-server, bench/device,
service device-tase2, configured by bench/device/tase2-demo.toml). Its device is Breaker12, a Select-Before-Operate breaker whose
command 2 closes it, and its information messages come every three seconds.

Each command is one association. A Select keeps the device armed for its client until the timeout of the table (5 s here), so the
tests that select wait for that timeout, or end with an operate that completes the cycle.
"""

from __future__ import annotations

import re
import time

import pytest

from igs_bench.compose import Bench

DEVICE = "172.31.110.11:102"
CLIENT_SERVICE = "client-tase2"
CLIENT = "/usr/local/bin/igstase2-client"
# The timeout of Breaker12 in the table of the demonstration file, and a margin for the client to finish.
TIMEOUT_S = 5
SETTLE_S = TIMEOUT_S + 2
# The hexadecimal of the stream of the demonstration: "bench-demo".
DEMO_STREAM_HEX = "62656e63682d64656d6f"


@pytest.fixture(scope="module")
def bench():
    """One bench for the blocks: the device and the client, started once."""
    instance = Bench("c8-blocks.pcap")
    instance.down()
    instance.up("device-tase2", CLIENT_SERVICE)
    try:
        ready = False
        for _ in range(30):
            result = run_client(instance, "get", "Flow1", check=False)
            if result.returncode == 0:
                ready = True
                break
        assert ready, "the device does not answer"
        yield instance
    finally:
        instance.down()


def run_client(bench: Bench, *args: str, check: bool = False, timeout: float = 90):
    """Runs igstase2-client in the client service, against the device (the global options come first)."""
    return bench.exec(CLIENT_SERVICE, CLIENT, "-s", DEVICE, *args, timeout=timeout, check=check)


def lines(result) -> list[str]:
    return [line for line in result.stdout.splitlines() if line.strip()]


def test_a_select_returns_the_check_back_id_of_the_table(bench):
    result = run_client(bench, "select", "Breaker12")
    assert result.returncode == 0, result.stdout + result.stderr
    assert lines(result) == ["selected Breaker12: CheckBackID 12"], result.stdout


def test_a_device_armed_for_one_client_refuses_another_select_and_releases_at_its_timeout(bench):
    # The previous test left Breaker12 armed for its client, a moment ago: another association is refused.
    refused = run_client(bench, "select", "Breaker12")
    assert refused.returncode == 1, refused.stdout + refused.stderr
    assert "TemporarilyUnavailable" in refused.stderr, refused.stderr
    # After the timeout, the device is IDLE again.
    time.sleep(SETTLE_S)
    state = run_client(bench, "get-tag", "Breaker12")
    assert state.returncode == 0, state.stdout + state.stderr
    assert "state IDLE" in state.stdout, state.stdout


def test_an_operate_selects_then_operates_the_sbo_device(bench):
    result = run_client(bench, "operate", "Breaker12", "1")
    assert result.returncode == 0, result.stdout + result.stderr
    assert lines(result) == ["operated Breaker12"], result.stdout
    # The simulated command completes at once: the device is IDLE afterwards.
    state = run_client(bench, "get-tag", "Breaker12")
    assert "state IDLE" in state.stdout, state.stdout


def test_an_operate_of_an_sbo_device_without_its_select_is_refused(bench):
    result = run_client(bench, "operate", "Breaker12", "1", "--direct")
    assert result.returncode == 1, result.stdout + result.stderr
    assert "refused the operate: TemporarilyUnavailable" in result.stderr, result.stderr


def test_a_close_only_inhibit_tag_refuses_the_closing_command_and_not_another(bench):
    try:
        assert run_client(bench, "set-tag", "Breaker12", "close-only-inhibit", "--reason", "crew on site").returncode == 0
        # The select is allowed under CLOSE-ONLY-INHIBIT. The closing command 2 is refused in its association.
        closing = run_client(bench, "operate", "Breaker12", "2")
        assert closing.returncode == 1, closing.stdout + closing.stderr
        assert "refused the operate: TemporarilyUnavailable" in closing.stderr, closing.stderr
        # The refused operate leaves the selection held until its timeout (decision D-064): the next association waits for it.
        time.sleep(SETTLE_S)
        other = run_client(bench, "operate", "Breaker12", "1")
        assert other.returncode == 0, other.stdout + other.stderr
    finally:
        run_client(bench, "set-tag", "Breaker12", "no-tag")


def test_a_tag_and_its_reason_are_read_and_inhibit_the_select(bench):
    try:
        assert run_client(bench, "set-tag", "Breaker12", "open-and-close-inhibit", "--reason", "maintenance").returncode == 0
        state = run_client(bench, "get-tag", "Breaker12")
        assert state.returncode == 0, state.stdout + state.stderr
        assert 'tag open-and-close-inhibit, reason "maintenance", state IDLE' in state.stdout, state.stdout
        refused = run_client(bench, "select", "Breaker12")
        assert refused.returncode == 1, refused.stdout + refused.stderr
        assert "TemporarilyUnavailable" in refused.stderr, refused.stderr
    finally:
        run_client(bench, "set-tag", "Breaker12", "no-tag")


def test_an_information_message_is_reported_to_the_client(bench):
    result = run_client(bench, "--timeout-s", "20", "start-info", "--messages", "2")
    assert result.returncode == 0, result.stdout + result.stderr
    messages = [line for line in lines(result) if line.startswith("message ")]
    assert len(messages) == 2, result.stdout
    pattern = re.compile(
        r"message (\d+): info reference 7, local reference 1, message id (\d+), 10 octets: " + DEMO_STREAM_HEX
    )
    ids = []
    for line in messages:
        match = pattern.fullmatch(line)
        assert match, line
        ids.append(int(match.group(2)))
    # The messages come in order, one per period (the first may have been sent before this association).
    assert ids[1] == ids[0] + 1, ids


def test_the_stop_of_the_information_transfer_set_is_accepted(bench):
    result = run_client(bench, "stop-info")
    assert result.returncode == 0, result.stdout + result.stderr
    assert lines(result) == ["stopped the information messages"], result.stdout


def test_an_integrity_check_reports_the_entire_data_set_with_rbe(bench):
    result = run_client(
        bench, "--timeout-s", "20", "start", "TS1", "DS1", "--rbe", "--integrity-s", "2", "--reports", "2"
    )
    assert result.returncode == 0, result.stdout + result.stderr
    reports = [line for line in lines(result) if line.startswith("report ")]
    assert len(reports) == 2, result.stdout
    for line in reports:
        # With RBE the report names its members: all four of DS1, the integrity check covers the data set.
        assert "Flow1,Level,Temperature,Breaker" in line, line


def test_a_buffered_object_change_is_reported_with_its_tle(bench):
    result = run_client(
        bench,
        "--timeout-s",
        "20",
        "start",
        "TS1",
        "DS1",
        "--object-change",
        "--buffer-s",
        "1",
        "--tle-s",
        "10",
        "--reports",
        "1",
    )
    assert result.returncode == 0, result.stdout + result.stderr
    reports = [line for line in lines(result) if line.startswith("report ")]
    assert len(reports) == 1 and "data set DS1" in reports[0], result.stdout
