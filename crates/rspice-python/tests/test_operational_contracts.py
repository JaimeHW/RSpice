"""Binding-level memory ownership and gross performance regression contracts."""

import gc
import time
import weakref

import numpy as np
import pytest

import rspice


DIVIDER = """* Operational contract fixture
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.end
"""


@pytest.fixture(scope="module")
def dc_result():
    return rspice.Engine().run_dc_op(rspice.Netlist.parse(DIVIDER))


def test_numpy_results_are_owned_and_do_not_alias_rust_storage(dc_result) -> None:
    first = dc_result.node_voltages
    assert first.dtype == np.float64
    assert first.flags.c_contiguous
    assert first.flags.owndata

    original = dc_result.voltage("out")
    first[2] = 12345.0

    assert dc_result.voltage("out") == pytest.approx(original)
    second = dc_result.node_voltages
    assert second[2] == pytest.approx(original)
    assert not np.shares_memory(first, second)


def test_temporary_numpy_exports_are_not_retained(dc_result) -> None:
    references = []
    for _ in range(1_000):
        array = dc_result.node_voltages
        references.append(weakref.ref(array))
    del array
    gc.collect()

    assert all(reference() is None for reference in references)


def test_scalar_result_access_has_no_pathological_binding_overhead(dc_result) -> None:
    # This intentionally generous ceiling is a regression tripwire, not a
    # microbenchmark. It catches accidental reparsing/simulation or blocking
    # synchronization in a scalar accessor without depending on runner speed.
    started = time.perf_counter()
    checksum = 0.0
    for _ in range(100_000):
        checksum += dc_result.voltage("out")
    elapsed = time.perf_counter() - started

    assert checksum == pytest.approx(500_000.0)
    assert elapsed < 10.0, f"100,000 scalar result reads took {elapsed:.3f}s"
