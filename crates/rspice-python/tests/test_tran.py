"""Transient analysis: waveforms, branch currents, Fourier, validation."""

import math

import numpy as np
import pytest

import rspice

RC_STEP = """* RC step response, tau = 100us
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
R1 in out 1k
C1 out 0 100n
.end
"""

SINE = """* 1 kHz sine into a divider
V1 in 0 SIN(0 1 1k)
R1 in out 1k
R2 out 0 1k
.end
"""


class TestTransient:
    def test_rc_step_settles_to_one(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        tran = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
        v_out = tran.voltage_waveform("out")
        assert isinstance(v_out, np.ndarray)
        assert len(v_out) == tran.num_points
        assert v_out[-1] == pytest.approx(1.0, abs=1e-3)
        assert tran.stop_time == pytest.approx(1e-3, rel=1e-3)

    def test_rc_time_constant(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        tran = engine.run_tran(netlist, stop_time=5e-4, max_step=5e-7)
        time = tran.time
        v_out = tran.voltage_waveform("out")
        # Value at t = tau should be 1 - 1/e.
        idx = int(np.searchsorted(time, 1e-4))
        assert v_out[idx] == pytest.approx(1.0 - math.exp(-1.0), abs=0.02)

    def test_ground_waveform_is_zero(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        assert float(np.max(np.abs(tran.voltage_waveform(0)))) == 0.0
        assert float(np.max(np.abs(tran.voltage_waveform("gnd")))) == 0.0

    def test_voltage_at(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        assert tran.voltage_at(0, 0) == 0.0
        with pytest.raises(IndexError):
            tran.voltage_at(1, 10**9)
        with pytest.raises(IndexError):
            tran.voltage_at(99, 0)

    def test_unknown_node_raises_keyerror(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        with pytest.raises(KeyError):
            tran.voltage_waveform("nonexistent")


class TestTransientBranchCurrents:
    def test_source_current_waveform(self, engine):
        netlist = rspice.Netlist.parse(RC_STEP)
        tran = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
        assert "V1" in [n.upper() for n in tran.branch_names]
        i_v1 = tran.branch_current_waveform("V1")
        assert len(i_v1) == tran.num_points
        # After settling no current flows; at t=0+ the full step appears
        # across R: |I| ≈ 1 V / 1 kΩ.
        assert abs(i_v1[-1]) < 5e-5
        assert np.max(np.abs(i_v1)) == pytest.approx(1e-3, rel=0.05)

    def test_unknown_branch_raises_keyerror(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        with pytest.raises(KeyError):
            tran.branch_current_waveform("L99")


class TestTransientValidation:
    def test_zero_stop_time_raises(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=0.0)

    def test_negative_stop_time_raises(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=-1.0)

    def test_non_finite_stop_time_raises(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=float("inf"))

    def test_invalid_max_step_raises(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=1e-3, max_step=-1.0)
        with pytest.raises(ValueError):
            engine.run_tran(rc_lowpass, stop_time=1e-3, max_step=0.0)


class TestFourier:
    def test_pure_sine_has_low_thd(self, engine):
        netlist = rspice.Netlist.parse(SINE)
        tran = engine.run_tran(netlist, stop_time=10e-3, max_step=2e-6)
        four = tran.fourier("out", fundamental=1e3)
        assert four.fundamental_magnitude == pytest.approx(0.5, rel=0.02)
        assert four.thd_percent < 1.0
        assert abs(four.dc_component) < 0.01
        assert len(four.harmonics) >= 5
        assert four.harmonics[0].n == 1
        assert four.harmonics[0].frequency == pytest.approx(1e3, rel=1e-6)
        assert len(four.magnitudes) == len(four.harmonics)

    def test_fundamental_phase_units(self, engine):
        # v(t) = A sin(wt) decomposes with a1=0, b1=A, so the fundamental
        # phase is atan2(-b, a) = -90 degrees = -pi/2 radians. Pins the
        # radians/degrees convention (core reports degrees; the binding
        # normalizes to radians with a *_degrees helper).
        netlist = rspice.Netlist.parse(SINE)
        tran = engine.run_tran(netlist, stop_time=10e-3, max_step=2e-6)
        fund = tran.fourier("out", fundamental=1e3).harmonics[0]
        assert fund.phase == pytest.approx(-math.pi / 2, abs=0.02)
        assert fund.phase_degrees == pytest.approx(-90.0, abs=1.0)

    def test_fourier_validation(self, engine, rc_lowpass):
        tran = engine.run_tran(rc_lowpass, stop_time=1e-4)
        with pytest.raises(ValueError):
            tran.fourier("out", fundamental=-1.0)
        with pytest.raises(ValueError):
            tran.fourier("out", fundamental=1e3, num_harmonics=0)
        with pytest.raises(KeyError):
            tran.fourier("nonexistent", fundamental=1e3)
