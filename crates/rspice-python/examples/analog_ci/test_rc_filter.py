"""Analog regression testing with RSpice and pytest.

This file is the executable example for putting a circuit under CI: pass/fail
criteria live in the netlist as .MEAS statements, the deck runs with
``Engine.run``, and ``RunReport.assert_passed()`` plus ordinary asserts gate
the pipeline.

Run it from the repository root:

    cd crates/rspice-python
    python -m pip install maturin numpy pytest
    maturin develop --release
    python -m pytest examples/analog_ci -v
"""

import math

import pytest

import rspice

# Design under test: first-order RC lowpass.
R = 1e3  # Ohms
C = 100e-9  # Farads
TAU = R * C  # 100 us
FC = 1.0 / (2 * math.pi * TAU)  # 1591.5 Hz

RC_FILTER = f"""* RC lowpass regression deck
V1 in 0 AC 1 PULSE(0 1 0 1n 1n 1 2)
R1 in out {R}
C1 out 0 {C}

* Step-response acceptance criteria, evaluated by the simulator:
.tran {TAU / 100} {8 * TAU}
.meas tran t_half  FIND TIME WHEN V(out)=0.5
.meas tran v_final MAX V(out)
.meas tran trise   TRIG V(out) VAL=0.1 RISE=1 TARG V(out) VAL=0.9 RISE=1
.end
"""


@pytest.fixture(scope="module")
def report():
    engine = rspice.Engine()
    netlist = rspice.Netlist.parse(RC_FILTER)
    return engine.run(netlist)


class TestStepResponse:
    def test_all_measurements_evaluated(self, report):
        # Fails loudly if a .MEAS statement was skipped or failed to evaluate.
        report.assert_passed()

    def test_settles_to_input(self, report):
        assert report.measurement("v_final").value == pytest.approx(1.0, abs=1e-3)

    def test_half_voltage_time(self, report):
        # First-order step response: t(50%) = tau * ln 2.
        expected = TAU * math.log(2.0)
        assert report.measurement("t_half").value == pytest.approx(expected, rel=0.02)

    def test_rise_time(self, report):
        # 10%-90% rise time of a single pole: tau * ln 9.
        expected = TAU * math.log(9.0)
        assert report.measurement("trise").value == pytest.approx(expected, rel=0.02)


class TestFrequencyResponse:
    def test_corner_frequency(self):
        engine = rspice.Engine()
        netlist = rspice.Netlist.parse(RC_FILTER)
        ac = engine.run_ac_sweep(netlist, "dec", 50, FC / 100, FC * 100)
        gain_db = ac.voltage_db("out")

        # -3 dB at the corner, -20 dB/decade beyond it.
        corner_idx = int(abs(ac.frequencies - FC).argmin())
        assert gain_db[corner_idx] == pytest.approx(-3.01, abs=0.1)
        assert gain_db[-1] == pytest.approx(-40.0, abs=0.5)

    def test_pole_location(self):
        engine = rspice.Engine()
        netlist = rspice.Netlist.parse(RC_FILTER)
        pz = engine.run_pz(netlist, "in", "out")

        assert pz.is_stable
        assert pz.bandwidth_hz == pytest.approx(FC, rel=1e-3)


class TestRobustnessToComponentVariation:
    def test_divider_midpoint_stays_in_band_with_5pct_parts(self):
        engine = rspice.Engine()
        netlist = rspice.Netlist.parse(
            f"""* Monte Carlo divider study
.param rval={R}
V1 in 0 10
R1 in out {{rval}}
R2 out 0 {R}
.end
"""
        )

        mc = engine.run_monte_carlo(
            netlist, num_runs=300, seed=2026, distribution="uniform", spread=0.05
        )
        stats = mc.get_variable("V(OUT)")

        assert mc.num_failures == 0
        # Divider midpoint with one 5%-toleranced arm: stays within +/-2.6%.
        assert stats.min > 4.85
        assert stats.max < 5.15
