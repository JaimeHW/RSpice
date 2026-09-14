"""Sparse charge impulses are distinct from finite branch-current samples."""

import pickle
import struct

import numpy as np
import pytest
import rspice


@pytest.fixture(params=[False, True], ids=["full", "compressed"])
def result(request, engine, rc_lowpass):
    run = engine.run_tran_compressed if request.param else engine.run_tran
    return run(rc_lowpass, stop_time=1e-5, max_step=1e-6)


def with_impulses(result, rows, version=1):
    restore, state = result.__reduce__()
    return restore(*state[:-1], (version, rows))


def test_current_impulses_pickle_preserves_availability_and_legacy_shape(result):
    assert result.current_impulses is None
    restore, state = result.__reduce__()
    assert restore(*state[:-1]).current_impulses is None
    for rows in [None, []]:
        recorded = with_impulses(result, rows)
        restored = pickle.loads(pickle.dumps(recorded))
        assert restored.current_impulses == rows
        with pytest.raises(rspice.RSpiceNotImplementedError, match="restored from pickled state"):
            restored.document()


def test_current_impulses_preserve_signed_charge_without_changing_amperes(result):
    branch = result.branch_names[0]
    rows = [(branch, [(0.0, -1.25e-12), (float(result.time[-1]) / 3, float.fromhex("0x0.0000000000001p-1022"))])]
    original_current = result.branch_current_waveform(branch)
    recorded = with_impulses(result, rows)
    restored = pickle.loads(pickle.dumps(recorded))
    trace = restored.current_impulses[0]
    assert (trace.owner_kind, trace.name, trace.parameter, trace.complete) == ("branch", branch, None, False)
    for expected, actual in zip(rows[0][1], trace.points, strict=True):
        assert struct.pack("dd", *actual) == struct.pack("dd", *expected)
    np.testing.assert_array_equal(restored.branch_current_waveform(branch), original_current)
    with pytest.raises(rspice.RSpiceNotImplementedError, match="restored from pickled state"):
        restored.document()


@pytest.mark.parametrize("version", [0, 3, 999])
def test_current_impulses_refuse_unknown_versions(result, version):
    with pytest.raises(ValueError, match="current impulse pickle version"):
        with_impulses(result, None, version)


def test_current_impulses_refuse_malformed_charge_time_and_owner(result):
    branch = result.branch_names[0]
    for rows in [
        [(branch, [(0.0, 0.0)])],
        [(branch, [(0.0, float("nan"))])],
        [(branch, [(float("inf"), 1.0)])],
        [(branch, [(float(result.time[-1]) * 2, 1.0)])],
        [(branch, [(0.0, 1.0), (0.0, -1.0)])],
        [("unknown", [(0.0, 1.0)])],
        [(branch, [(0.0, 1.0)]), (branch.swapcase(), [(0.0, 2.0)])],
    ]:
        with pytest.raises(ValueError, match="current impulse"):
            with_impulses(result, rows)


def test_current_impulses_preserve_typed_coverage_and_readonly_snapshots(result):
    branch = result.branch_names[0]
    rows = [
        ("branch", branch, None, True, []),
        ("device_lead", "Q1", "ic", True, [(0.0, -1e-12)]),
        ("device_lead", "Q1", "ib", True, []),
    ]
    restored = pickle.loads(pickle.dumps(with_impulses(result, rows, 2)))
    assert restored.__reduce__()[1][-1] == (2, rows)
    assert [(t.owner_kind, t.name, t.parameter, t.complete, t.points)
            for t in restored.current_impulses] == rows
    trace = restored.current_impulses[1]
    assert isinstance(trace, rspice.CurrentImpulseTrace)
    assert "device_lead" in repr(trace)
    with pytest.raises(AttributeError):
        trace.complete = False
    trace.points.clear()
    assert restored.current_impulses[1].points == [(0.0, -1e-12)]


def test_current_impulses_refuse_ambiguous_owner_or_version_shape(result):
    branch = result.branch_names[0]
    for row in [
        ("branch", branch, "ic", True, []),
        ("device_lead", "Q1", None, True, []),
        ("device_lead", "Q1", "", True, []),
        ("device_lead", "", "ic", True, []),
        ("unknown", "Q1", None, True, []),
        ("branch", branch, None, False, []),
    ]:
        with pytest.raises(ValueError, match="current impulse"):
            with_impulses(result, [row], 2)
    for version, rows in [
        (1, [("branch", branch, None, True, [])]),
        (2, [(branch, [(0.0, 1e-12)])]),
    ]:
        with pytest.raises(ValueError, match="current impulse pickle version"):
            with_impulses(result, rows, version)
    rows = [("device_lead", "Q1", "ic", True, []),
            ("device_lead", "q1", "IC", True, [])]
    with pytest.raises(ValueError, match="duplicate current impulse"):
        with_impulses(result, rows, 2)
