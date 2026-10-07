"""Loop-stability analysis through direct and netlist-driven APIs."""

import numpy as np
import pytest
import pickle
import json

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
    def test_circuit_poles_and_hidden_instability_survive_pickle(self, engine):
        for hidden in [False, True]:
            source = SINGLE_POLE_STB.replace(
                ".end", "Ghidden hidden 0 hidden 0 -1\nChidden hidden 0 1\n.end"
            ) if hidden else SINGLE_POLE_STB
            result = engine.run_stb(rspice.Netlist.parse(source), "VPROBE",
                                    variation="lin", points=3, start_freq=10, stop_freq=1e7)
            for value in [result, pickle.loads(pickle.dumps(result))]:
                assert value.circuit_pole_status == "available"
                assert value.circuit_pole_failure is None
                assert value.is_stable is (not hidden)
                assert len(value.circuit_poles) == (2 if hidden else 1)
                assert value.circuit_pole_evidence.kind == "qualified"
                if hidden:
                    assert value.circuit_poles[0].real == pytest.approx(1.0)

    def test_unsupported_pole_evidence_survives_pickle_without_a_stability_claim(self, engine):
        source = SINGLE_POLE_STB.replace(".end", "B1 ctrl 0 I={FREQ*1p*V(ctrl)}\n.end")
        result = engine.run_stb(rspice.Netlist.parse(source), "VPROBE",
                                variation="lin", points=3, start_freq=10, stop_freq=1e7)
        for value in [result, pickle.loads(pickle.dumps(result))]:
            assert value.circuit_pole_status == "unavailable"
            assert value.circuit_pole_failure["kind"] == "unsupported"
            assert value.is_stable is None
            assert value.circuit_poles is None
            assert value.circuit_pole_evidence is None

    def test_circuit_pole_pickle_rejects_corruption_and_keeps_legacy_absence(self, engine):
        result = engine.run_stb(rspice.Netlist.parse(SINGLE_POLE_STB), "VPROBE",
                                variation="lin", points=3, start_freq=10, stop_freq=1e7)
        restore, args = result.__reduce__()
        legacy = restore(*args[:-1])
        assert legacy.circuit_pole_status == "not_computed"
        assert legacy.circuit_poles is None
        assert legacy.is_stable is None
        malformed = list(args)
        state = json.loads(malformed[-1][1])
        state["spectrum"]["poles"] = []
        malformed[-1] = (1, json.dumps(state))
        with pytest.raises(ValueError, match="circuit-pole evidence"):
            restore(*malformed)
        malformed[-1] = (2, "{}")
        with pytest.raises(ValueError, match="circuit-pole pickle version"):
            restore(*malformed)

    @pytest.mark.parametrize("points", [1, 3])
    def test_zero_gain_has_explicit_bode_validity_and_survives_pickle(self, engine, points):
        result = engine.run_stb(
            rspice.Netlist.parse(SINGLE_POLE_STB.replace("-1000", "0")),
            "VPROBE", variation="lin", points=points, start_freq=10.0, stop_freq=1000.0,
        )
        for value in [result, pickle.loads(pickle.dumps(result))]:
            np.testing.assert_array_equal(value.loop_gain, np.zeros(points, dtype=complex))
            np.testing.assert_array_equal(value.magnitude, np.zeros(points))
            assert value.magnitude_validity.all()
            assert not value.magnitude_db_validity.any()
            assert not value.phase_validity.any()
            assert np.isnan(value.magnitude_db).all()
            assert np.isnan(value.phase_degrees).all()
            assert value.gain_margin_db is None
            assert value.phase_margin_degrees is None

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
        assert result.gain_margin_db is None
        assert result.gain_margin_frequency is None
        assert result.assessment == "PARTIAL MARGIN MEASUREMENT"
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

    @pytest.mark.parametrize("gain", [0.5, 1000.0])
    @pytest.mark.parametrize("points", [1, 3])
    def test_absent_margins_survive_single_point_and_truncated_sweeps(self, engine, gain, points):
        result = engine.run_stb(
            rspice.Netlist.parse(SINGLE_POLE_STB.replace("-1000", f"-{gain}")),
            "VPROBE", variation="lin", points=points,
            start_freq=10.0, stop_freq=1000.0,
        )
        for value in [result, pickle.loads(pickle.dumps(result))]:
            assert value.success
            assert len(value.frequencies) == points
            np.testing.assert_allclose(
                value.loop_gain, gain / (1 + 1j * value.frequencies / 1000.0), rtol=1e-10,
            )
            for name in ["gain_margin_db", "gain_margin_frequency", "phase_margin_degrees",
                         "phase_margin_frequency", "unity_gain_bandwidth"]:
                assert getattr(value, name) is None
            assert value.num_crossovers == 0
            assert not value.multiple_crossovers
            assert value.assessment == "NO MARGINS MEASURED"
        scalars = {scalar.name: scalar for scalar in result.scalars()}
        for name in ["gain_margin_db", "gain_margin_frequency", "phase_margin_degrees",
                     "phase_margin_frequency", "unity_gain_bandwidth"]:
            assert scalars[name].unavailable_reason == "no_crossover"

    def test_legacy_pickle_replaces_infinite_margin_and_stability_claims(self):
        result = rspice.StbResult._unpickle(
            [10.0, 100.0], [(0.5, -0.1), (0.4, -0.2)], "VP",
            [float("inf"), 0, float("inf"), 0, 0, 0],
            (False, 0, True), [], "WELL DAMPED",
        )
        assert result.success
        assert result.gain_margin_db is None
        assert result.phase_margin_degrees is None
        assert result.assessment == "NO MARGINS MEASURED"

    @pytest.mark.parametrize("flags,state", [
        ((False, 1, True), (2, None, (90.0, 10.0))),
        ((False, 1, True), (1, None, (float("nan"), 10.0))),
        ((False, 1, True), (1, None, (90.0, 0.0))),
        ((False, 1, True), (1, None, (90.0, 101.0))),
        ((False, 1, True), (1, None, None)),
        ((False, 1, True), (1, (1.0, 10.0), (90.0, 10.0))),
        ((True, 2, True), (1, None, (90.0, 10.0))),
    ])
    def test_invalid_margin_pickle_evidence_is_rejected(self, flags, state):
        with pytest.raises(ValueError, match="STB"):
            rspice.StbResult._unpickle(
                [1.0, 100.0], [(0.0, -2.0), (0.0, -0.5)], "VP", [0] * 6,
                flags, [], "ignored", (1, None), state,
            )

    def test_measured_zero_margin_survives_pickle(self):
        result = rspice.StbResult._unpickle(
            [100.0], [(-1.0, 0.0)], "VP", [0] * 6,
            (False, 1, True), [], "ignored", (1, None),
            (1, (0.0, 100.0), (0.0, 100.0)),
        )
        for value in [result, pickle.loads(pickle.dumps(result))]:
            assert value.gain_margin_db == 0.0
            assert value.phase_margin_degrees == 0.0
            assert value.gain_margin_frequency == value.phase_margin_frequency == 100.0
            assert value.assessment == "NONPOSITIVE MEASURED MARGIN"

    def test_phase_matches_the_core_bode_trace_and_survives_pickle(self, engine):
        netlist = rspice.Netlist.parse("""* three-pole loop
E1 eo 0 n3 0 -1000
VP eo x 0
R1 x n1 1k
C1 n1 0 159.154943091895n
E2 b1 0 n1 0 1
R2 b1 n2 1k
C2 n2 0 159.154943091895n
E3 b2 0 n2 0 1
R3 b2 n3 1k
C3 n3 0 159.154943091895n
.end
""")
        result = engine.run_stb(netlist, "VP", variation="dec", points=20,
                                start_freq=100.0, stop_freq=1e5)
        # T=1000/(1+j*f/1000)^3 crosses the phase branch cut inside this band.
        expected = -3 * np.degrees(np.arctan(result.frequencies / 1000.0))
        for value in [result, pickle.loads(pickle.dumps(result))]:
            np.testing.assert_allclose(value.phase_degrees, expected, rtol=0, atol=1e-8)
        phase = next(signal for signal in result.document()["signals"]
                     if signal["descriptor"]["canonicalName"] == "loop_gain_phase")
        np.testing.assert_array_equal(result.phase_degrees, phase["values"]["samples"])
