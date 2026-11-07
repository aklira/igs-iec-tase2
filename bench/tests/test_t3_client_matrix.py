# SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
#
# SPDX-License-Identifier: Apache-2.0
"""T3: the client interop matrix. The client under test (igstase2-client, bench/client) runs one operation per command
against the device under test (igstase2-server, bench/device, service device-tase2). Each test is one row of
docs/client-matrix.md; the last test checks that every test the matrix names exists here.
"""

from __future__ import annotations

import re
from pathlib import Path

import pytest

from igs_bench.compose import Bench

DEVICE = "172.31.110.11:102"
CLIENT_SERVICE = "client-tase2"
CLIENT = "/usr/local/bin/igstase2-client"
MATRIX = Path(__file__).resolve().parent.parent.parent / "docs" / "client-matrix.md"


@pytest.fixture(scope="module")
def bench():
    """One bench for the matrix: the device and the client, started once."""
    instance = Bench("t3-client-matrix.pcap")
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


def test_association_is_established_and_points_are_read(bench):
    result = run_client(bench, "get", "Flow1", "Level", "Temperature", "Breaker")
    assert result.returncode == 0, result.stdout + result.stderr
    values = dict(line.split(": ", 1) for line in lines(result))
    assert set(values) == {"Flow1", "Level", "Temperature", "Breaker"}
    assert values["Breaker"] in ("true", "false")
    assert float(values["Temperature"]) > 0


def test_an_ap_title_without_a_table_is_refused_with_status_1(bench):
    result = run_client(bench, "--ap-title", "1.3.9999.9", "get", "Flow1")
    assert result.returncode == 1, result.stdout + result.stderr
    assert "refused the association" in result.stderr


def test_a_table_identifier_that_differs_is_concluded_with_status_1(bench):
    result = run_client(bench, "--table-id", "BT-OTHER", "get", "Flow1")
    assert result.returncode == 1, result.stdout + result.stderr
    assert "do not match the agreement" in result.stderr


def test_a_point_that_does_not_exist_is_an_item_error(bench):
    result = run_client(bench, "get", "Missing")
    assert result.returncode == 0, result.stdout + result.stderr
    assert "Missing: error ObjectNonExistent" in lines(result)


def test_a_permitted_write_is_applied_and_read_back(bench):
    result = run_client(bench, "set", "Flow1=42")
    assert result.returncode == 0, result.stdout + result.stderr
    assert "Flow1: written" in lines(result)
    readback = run_client(bench, "get", "Flow1")
    assert "Flow1: 42" in lines(readback), readback.stdout


def test_a_write_the_table_does_not_permit_is_refused_per_item(bench):
    result = run_client(bench, "set", "Temperature=1.5")
    assert result.returncode == 0, result.stdout + result.stderr
    assert "Temperature: error ObjectAccessDenied" in lines(result)


def test_the_type_of_a_point_is_described(bench):
    result = run_client(bench, "type", "Flow1")
    assert result.returncode == 0, result.stdout + result.stderr
    assert "Flow1: Integer(Unsigned8(32))" in lines(result)


def test_the_members_of_a_data_set_are_listed(bench):
    result = run_client(bench, "members", "DS1")
    assert result.returncode == 0, result.stdout + result.stderr
    assert lines(result) == ["Flow1", "Level", "Temperature", "Breaker"]


def test_a_data_set_is_read_member_by_member(bench):
    result = run_client(bench, "read", "DS1")
    assert result.returncode == 0, result.stdout + result.stderr
    assert len(lines(result)) == 4


def test_a_data_set_write_answers_each_member(bench):
    result = run_client(bench, "write", "DS1", "1", "2", "3.0", "true")
    assert result.returncode == 0, result.stdout + result.stderr
    answers = lines(result)
    assert "Flow1: written" in answers
    assert "Level: error ObjectAccessDenied" in answers
    assert "Breaker: error ObjectAccessDenied" in answers


def test_a_data_set_the_client_creates_can_be_read_and_deleted(bench):
    created = run_client(bench, "create", "DS9", "Flow1", "Breaker")
    assert created.returncode == 0, created.stdout + created.stderr
    assert "created DS9" in lines(created)
    members = run_client(bench, "members", "DS9")
    assert lines(members) == ["Flow1", "Breaker"], members.stdout + members.stderr
    deleted = run_client(bench, "delete", "DS9")
    assert "deleted 1" in lines(deleted), deleted.stdout + deleted.stderr


def test_a_data_set_the_table_does_not_show_is_refused_with_status_1(bench):
    result = run_client(bench, "members", "NoSuchSet")
    assert result.returncode == 1, result.stdout + result.stderr
    assert "OBJECT-NON-EXISTENT" in result.stderr


def test_a_data_set_that_does_not_exist_cannot_be_deleted(bench):
    result = run_client(bench, "delete", "NoSuchSet")
    assert result.returncode == 1, result.stdout + result.stderr
    assert "OBJECT-UNDEFINED" in result.stderr


def test_a_transfer_set_reports_each_change_of_its_data_set(bench):
    result = run_client(bench, "--timeout-s", "20", "start", "TS1", "DS1", "--object-change", "--reports", "2")
    assert result.returncode == 0, result.stdout + result.stderr
    reports = [line for line in lines(result) if line.startswith("report ")]
    assert len(reports) == 2, result.stdout
    assert all("data set DS1" in line for line in reports)


def test_an_interval_transfer_set_reports_at_its_interval(bench):
    result = run_client(bench, "--timeout-s", "20", "start", "TS1", "DS1", "--interval-s", "2", "--reports", "2")
    assert result.returncode == 0, result.stdout + result.stderr
    assert len([line for line in lines(result) if line.startswith("report ")]) == 2


def test_a_report_by_exception_names_only_the_changed_members(bench):
    result = run_client(bench, "--timeout-s", "20", "start", "TS1", "DS1", "--rbe", "--object-change", "--reports", "2")
    assert result.returncode == 0, result.stdout + result.stderr
    reports = [line for line in lines(result) if line.startswith("report ")]
    assert len(reports) == 2
    for line in reports:
        assert "data set" not in line, line
        assert re.match(r"report \d+: \w+: ", line), line


def test_a_stop_of_a_transfer_set_that_is_not_active_is_refused(bench):
    result = run_client(bench, "stop", "TS1", "DS1")
    assert result.returncode == 1, result.stdout + result.stderr
    assert "refused the stop" in result.stderr


def test_a_transfer_set_on_a_data_set_that_does_not_exist_is_refused(bench):
    result = run_client(bench, "start", "TS1", "NoSuchSet", "--object-change")
    assert result.returncode == 1, result.stdout + result.stderr
    assert "refused the transfer set" in result.stderr


def test_every_test_the_client_matrix_names_exists_here():
    text = MATRIX.read_text(encoding="utf-8")
    names = set(re.findall(r"`(test_[a-z0-9_]+)`", text))
    here = Path(__file__).read_text(encoding="utf-8")
    missing = sorted(name for name in names if f"def {name}(" not in here)
    assert names, "the matrix names its tests"
    assert missing == [], f"named in the matrix but not found: {missing}"
