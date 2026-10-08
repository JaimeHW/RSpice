"""Voltage actions remain distinct from samples and current charge."""
import cmath
import math
import pickle
import struct

import numpy as np
import pytest
import rspice


def with_voltage(result, rows, version=1):
    restore, state = result.__reduce__()
    return restore(*state[:-1], (version, rows))


@pytest.fixture(params=[False, True], ids=["full", "compressed"])
def result(request, engine, rc_lowpass):
    run = engine.run_tran_compressed if request.param else engine.run_tran
    return run(rc_lowpass, stop_time=1e-5, max_step=1e-6)


def test_voltage_history_pickle_retains_units_bits_and_readonly_snapshots(result):
    node = result.node_names[0]
    tiny = float.fromhex("0x0.0000000000001p-1022")
    time = float(result.time[-1]) / 3
    rows = [(node, True, [(0.0, -tiny), (time, 2e-12)], [(time, 2, -3e-30)])]
    original = result.voltage_waveform(node)
    restored = pickle.loads(pickle.dumps(with_voltage(result, rows)))
    trace = restored.voltage_impulses[0]
    assert isinstance(trace, rspice.VoltageImpulseTrace)
    assert trace.node_name == node and trace.complete
    assert trace.derivatives == rows[0][3]
    for actual, expected in zip(trace.points, rows[0][2], strict=True):
        assert struct.pack("dd", *actual) == struct.pack("dd", *expected)
    assert restored.__reduce__()[1][-1] == (1, rows)
    assert restored.current_impulses is None
    np.testing.assert_array_equal(restored.voltage_waveform(node), original)
    with pytest.raises(AttributeError):
        trace.complete = False
    trace.points.clear()
    assert restored.voltage_impulses[0].points == rows[0][2]


def test_voltage_history_pickle_preserves_legacy_and_empty_coverage(result):
    restore, state = result.__reduce__()
    assert restore(*state[:-1]).voltage_impulses is None
    assert restore(*state[:-2]).current_impulses is None
    assert restore(*state[:-2]).voltage_impulses is None
    for rows in [None, [], [(result.node_names[0], True, [], [])]]:
        restored = pickle.loads(pickle.dumps(with_voltage(result, rows)))
        assert restored.__reduce__()[1][-1] == (1, rows)
    with pytest.raises(ValueError, match="voltage impulse pickle version"):
        with_voltage(result, None, 2)


def test_voltage_history_refuses_malformed_owners_times_and_orders(result):
    node = result.node_names[0]
    time = float(result.time[-1])
    for rows in [
        [("missing", True, [], [])],
        [(node, False, [], [])],
        [(node, True, [(time, 0.0)], [])],
        [(node, True, [(time, float("nan"))], [])],
        [(node, True, [(time * 2, 1.0)], [])],
        [(node, True, [(time, 1.0), (time, 2.0)], [])],
        [(node, True, [], [(time, 0, 1.0)])],
        [(node, True, [], [(time, 1, 0.0)])],
        [(node, True, [], []), (node.swapcase(), True, [], [])],
    ]:
        with pytest.raises(ValueError, match="voltage impulse"):
            with_voltage(result, rows)


def test_voltage_fourier_and_measurements_include_distributional_terms(engine):
    source = "Voltage actions\nV1 out 0 0\nV2 ref 0 0\n"
    result = engine.run_tran(rspice.Netlist.parse_spice(source + ".end\n"),
                             stop_time=1.0, max_step=1.0 / 256)
    # INTEG spans the selected accepted rows. Put the singularity exactly at
    # its final row so this tests an undefined endpoint, not a cropped event.
    event_time = float(result.time[np.searchsorted(result.time, .25)])
    deck = rspice.Netlist.parse_spice(
        source + ".meas tran area INTEG V(out,ref)\n"
        f".meas tran boundary INTEG V(out) TO={event_time:.17e}\n.end\n"
    )
    rows = [
        ("out", True, [(0.0, 7.0), (event_time, .002)], [(event_time, 1, -1e-6)]),
        ("ref", True, [(event_time, -.001)], []),
    ]
    restored = pickle.loads(pickle.dumps(with_voltage(result, rows)))
    for fourier in [restored.fourier("out", 1.0, 4, reference="ref"),
                    restored.fourier_of("V(out,ref)", 1.0, 4)]:
        assert fourier.dc_component == pytest.approx(.003, rel=2e-13)
        for n, harmonic in enumerate(fourier.harmonics, start=1):
            omega = math.tau * n
            expected = 2 * (.003 - 1e-6j * omega) * cmath.exp(-1j * omega * event_time)
            actual = cmath.rect(harmonic.magnitude, math.radians(harmonic.phase_degrees))
            assert abs(actual - expected) < 2e-13
    measured = {m.name.lower(): m for m in engine.measure(deck, restored)}
    assert measured["area"].passed
    assert measured["area"].value == pytest.approx(.003, rel=2e-13)
    assert not measured["boundary"].passed
    assert measured["boundary"].raw_value is None
