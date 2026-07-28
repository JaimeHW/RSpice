"""Shared fixtures for the rspice binding test suite.

Where a new test goes
---------------------

One module per analysis, named for the directive it exercises: test_dc,
test_tran, test_ac, test_distortion, test_noise, test_pole_zero,
test_sensitivity, test_monte_carlo, test_step, test_transfer_function,
test_periodic_rf, test_s_parameters, test_stb, test_measure.

Behaviour that holds across every analysis gets a named contract module
instead, because filing it under one analysis would imply the others are
exempt:

- test_error_contract        exception types and their hierarchy
- test_operational_contracts readiness, memory ownership, gross performance
- test_threading             cancellation, GIL release, parallel engines
- test_pickle                round-tripping across a process boundary
- test_api_surface           the public surface, pinned against a golden
- test_export                Touchstone, SPICE raw, and CSV serialization
- test_introspection         walking a parsed deck
- test_netlist               parsing and title-line semantics

A module named for a grab-bag ("advanced", "misc") is the failure mode this
layout exists to prevent: it grows without an owner and nobody can say what
belongs in it.
"""

import pytest

import rspice

DIVIDER = """* Voltage divider
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.end
"""

RC_LOWPASS = """* RC lowpass, fc = 159.155 Hz
V1 in 0 AC 1 SIN(0 1 1k)
R1 in out 1k
C1 out 0 1u
.end
"""

PARAM_DIVIDER = """* Parametric divider
.param rval=1k
V1 in 0 10
R1 in out {rval}
R2 out 0 1k
.end
"""


@pytest.fixture(scope="session")
def engine():
    return rspice.Engine()


@pytest.fixture()
def divider():
    return rspice.Netlist.parse(DIVIDER)


@pytest.fixture()
def rc_lowpass():
    return rspice.Netlist.parse(RC_LOWPASS)


@pytest.fixture()
def param_divider():
    return rspice.Netlist.parse(PARAM_DIVIDER)
