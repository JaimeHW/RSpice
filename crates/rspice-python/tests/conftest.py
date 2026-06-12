"""Shared fixtures for the rspice binding test suite."""

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
