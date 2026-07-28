"""Pole-zero analysis (`.PZ`)."""

import numpy as np
import pytest

import rspice



class TestPoleZero:
    def test_explicit_ports_and_analysis_modes(self, engine, rc_lowpass):
        full = engine.run_pz(
            rc_lowpass,
            "in",
            "out",
            input_negative="0",
            output_negative="0",
            input_type="current",
            analysis="pz",
        )
        poles = engine.run_pz(
            rc_lowpass, "in", "out", analysis="poles"
        )
        zeros = engine.run_pz(
            rc_lowpass, "in", "out", analysis="zeros"
        )
        np.testing.assert_allclose(full.poles_array, poles.poles_array)
        assert poles.num_zeros == 0
        assert zeros.num_poles == 0

    def test_engine_run_executes_pz_directive(self, engine):
        netlist = rspice.Netlist.parse(
            """* RC PZ directive
I1 in 0 AC 1
R1 in out 1k
C1 out 0 1u
.pz in 0 out 0 cur pz
.end
"""
        )
        report = engine.run(netlist)
        assert report.pz is not None
        assert report.pz.num_poles == 1
        assert report.analyses_run == ["pz"]

    def test_pz_option_validation(self, engine, rc_lowpass):
        with pytest.raises(ValueError, match="input_type"):
            engine.run_pz(rc_lowpass, "in", "out", input_type="power")
        with pytest.raises(ValueError, match="analysis"):
            engine.run_pz(rc_lowpass, "in", "out", analysis="unknown")

    def test_rc_pole_location(self, engine, rc_lowpass):
        pz = engine.run_pz(rc_lowpass, "in", "out")
        assert pz.num_poles == 1
        assert pz.is_stable
        pole = pz.poles[0]
        assert pole.is_real
        # Pole at -1/RC = -1000 rad/s.
        assert pole.real == pytest.approx(-1000.0, rel=1e-6)
        assert pole.time_constant == pytest.approx(1e-3, rel=1e-6)
        assert pz.bandwidth_hz == pytest.approx(159.155, rel=1e-4)
        assert pz.dominant_pole_decay_hz == pytest.approx(159.155, rel=1e-4)

    def test_poles_array_and_complex_conversion(self, engine, rc_lowpass):
        pz = engine.run_pz(rc_lowpass, "in", "out")
        arr = pz.poles_array
        assert arr.dtype == np.complex128
        assert arr[0] == pytest.approx(-1000.0 + 0j, rel=1e-6)
        assert complex(pz.poles[0]) == pytest.approx(-1000.0 + 0j, rel=1e-6)
        assert pz.zeros_array.dtype == np.complex128

    def test_node_names_resolve(self, engine, rc_lowpass):
        by_name = engine.run_pz(rc_lowpass, "in", "out")
        by_index = engine.run_pz(rc_lowpass, 1, 2)
        assert by_name.poles[0].real == pytest.approx(
            by_index.poles[0].real, rel=1e-12
        )

    def test_unknown_node_raises_keyerror(self, engine, rc_lowpass):
        with pytest.raises(KeyError):
            engine.run_pz(rc_lowpass, "nonexistent", "out")
