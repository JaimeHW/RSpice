"""Loop-stability analysis through direct and netlist-driven APIs."""

import numpy as np
import pytest

import rspice


SINGLE_POLE_STB = """* single-pole loop
E1 eo 0 ctrl 0 -1000
VPROBE eo x 0
R1 x ctrl 1k
C1 ctrl 0 159.154943091895n
.stb dec 20 10 10meg probe=VPROBE
.end
"""


class TestStb:
    def test_direct_stb_exposes_loop_gain_and_margins(self, engine):
        netlist = rspice.Netlist.parse(SINGLE_POLE_STB)
        result = engine.run_stb(
            netlist,
            "VPROBE",
            variation="dec",
            points=20,
            start_freq=10.0,
            stop_freq=10e6,
        )

        assert result.probe_name == "VPROBE"
        assert result.success
        assert result.is_stable
        assert result.dc_gain_db == pytest.approx(60.0, abs=0.05)
        assert result.phase_margin_degrees == pytest.approx(90.0, abs=0.2)
        assert result.unity_gain_bandwidth == pytest.approx(1e6, rel=0.02)
        assert result.frequencies.dtype == np.float64
        assert result.loop_gain.dtype == np.complex128
        assert len(result.frequencies) == len(result.loop_gain)
        np.testing.assert_allclose(
            result.magnitude_db, 20.0 * np.log10(np.abs(result.loop_gain))
        )

    def test_engine_run_executes_stb_directive(self, engine):
        report = engine.run(rspice.Netlist.parse(SINGLE_POLE_STB))
        assert report.stb is not None
        assert report.stb.success
        assert report.analyses_run == ["stb"]
        assert report.skipped == []

    def test_invalid_probe_raises_simulation_error(self, engine):
        netlist = rspice.Netlist.parse(SINGLE_POLE_STB)
        with pytest.raises(rspice.SimulationError, match="not a voltage source"):
            engine.run_stb(netlist, "MISSING")
