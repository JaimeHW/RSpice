"""Loop-stability analysis through direct and netlist-driven APIs."""

import numpy as np
import pytest
import pickle

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

    def test_dc_gain_and_pickle_do_not_depend_on_sweep_start(self, engine):
        result = engine.run_stb(
            rspice.Netlist.parse(SINGLE_POLE_STB), "VPROBE",
            variation="lin", points=3, start_freq=1e5, stop_freq=1e7,
        )
        for value in [result, pickle.loads(pickle.dumps(result))]:
            assert value.dc_gain_db == pytest.approx(60.0, abs=1e-10)
            assert complex(value.dc_loop_gain) == pytest.approx(1000.0, abs=1e-8)
            assert value.frequencies[0] == 1e5
            assert len(value.frequencies) == 3

    def test_dc_pole_preserves_sweep_and_unavailability_across_pickle(self, engine):
        netlist = rspice.Netlist.parse("""* integrator loop
G1 eo 0 ctrl 0 1m
C1 eo 0 1u
VP eo x 0
R1 x ctrl 1k
.end
""")
        result = engine.run_stb(netlist, "VP", variation="lin", points=3,
                                start_freq=10.0, stop_freq=1000.0)
        for value in [result, pickle.loads(pickle.dumps(result))]:
            assert value.dc_loop_gain is None
            assert value.dc_gain_db is None
            assert any("DC loop gain" in warning for warning in value.warnings)
            np.testing.assert_allclose(value.loop_gain, 1000.0 / (2j * np.pi * value.frequencies), rtol=1e-10)

    def test_legacy_pickle_cannot_certify_its_first_sample_as_dc(self):
        result = rspice.StbResult._unpickle([], [], "VP", [0, 0, 0, 0, 20, 0],
                                           (False, 0, True), [], "legacy")
        assert result.dc_gain_db is None
        assert result.dc_loop_gain is None
        assert any("Legacy STB" in warning for warning in result.warnings)

    @pytest.mark.parametrize("state", [(2, None), (1, (float("nan"), 0.0))])
    def test_invalid_dc_pickle_evidence_is_rejected(self, state):
        with pytest.raises(ValueError, match="STB DC"):
            rspice.StbResult._unpickle([], [], "VP", [0] * 6,
                                      (False, 0, True), [], "legacy", state)
