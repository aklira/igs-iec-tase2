# SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
#
# SPDX-License-Identifier: Apache-2.0
"""F4: the reference peer associates with the device under test, and identifies it; the capture decodes in tshark.

The peer's MMS stack (rusty-mms-service, of rusty-iccp 0.9.2) is the initiator; the device is igsmms-server of
igs-iso9506-mms, the MMS server of this project's bench. No code of this repository takes part in the association.
"""

from __future__ import annotations

from igs_bench import capture, tshark

PEER = "172.31.110.21"
DEVICE = "172.31.110.10"


def test_the_reference_peer_associates_with_the_device_and_identifies_it(bench):
    bench.up("peer", "device", "capture")
    capture.wait_live(bench, service="capture")

    result = bench.exec("peer", "/usr/local/bin/ref-peer", f"{DEVICE}:102")
    assert "association established" in result.stdout, result.stdout + result.stderr
    assert "identity:" in result.stdout

    capture.stop(bench, service="capture")
    frames = tshark.decode(bench, service="capture")
    for frame in frames:
        print(frame.number, frame.src, ":".join(frame.protocols), frame.info)
    assert tshark.errors(bench, service="capture") == []
    assert tshark.find(frames, src=PEER, protocol="cotp", info="CR TPDU")
    assert tshark.find(frames, src=DEVICE, protocol="cotp", info="CC TPDU")
    assert tshark.find(frames, src=PEER, info="initiate-RequestPDU")
    assert tshark.find(frames, src=DEVICE, info="initiate-ResponsePDU")
    assert tshark.find(frames, src=PEER, info="confirmed-RequestPDU")
    assert tshark.find(frames, src=DEVICE, info="confirmed-ResponsePDU")
