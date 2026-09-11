"""DC and AC sensitivity analysis (`.SENS`)."""

import numpy as np
import pickle
import pytest

import rspice



class TestSensitivity:
    def test_parameter_sensitivity_replays_same_card_dependencies(self, engine):
        netlist = rspice.Netlist.parse_spice(
            "Same-card dependencies\n.param base=2 derived={3*base}\n"
            "V1 in 0 DC 1 AC 1\nE1 out 0 in 0 {base+derived}\n.end\n"
        )
        assert engine.run_sensitivity(netlist, "out", "base", 2.0) == pytest.approx(4.0, rel=1e-8)
        assert engine.run_sensitivity_ac(netlist, "out", "base", 2.0, [1.0]) == pytest.approx([4.0], rel=1e-8)

    def test_xspice_sensitivity_excludes_boolean_controls(self, engine):
        netlist = rspice.Netlist.parse_spice(
            "Typed sensitivity\nV1 in 0 DC 0.2 AC 1\nA1 in out lim\n"
            ".model lim limit(gain=3 fraction=1 out_lower_limit=0 out_upper_limit=10 limit_range=0)\n"
            "R1 out 0 1meg\n.end\n"
        )
        dc = engine.run_sensitivity_dc_complete(netlist, "out", filters=["lim:*"])
        ac = engine.run_sensitivity_ac_complete(netlist, "out", [1.0], filters=["lim:*"])
        assert "LIM:FRACTION" not in dc.vector_names
        assert "LIM:FRACTION" not in ac.vector_names
        assert dc.get("LIM:GAIN").absolute == pytest.approx(0.2, abs=1e-9)
        assert ac.get("LIM:GAIN").absolute == pytest.approx([1.0 + 0j], abs=1e-9)
        with pytest.raises(rspice.SimulationError, match="no DC parameter matched"):
            engine.run_sensitivity_dc_complete(netlist, "out", filters=["lim:fraction"])

    def test_parameter_sensitivity_refines_curvature_and_rejects_kinks(self, engine):
        for expression, nominal in [("exp(100*gain)", 1.0), ("abs(gain)", 0.0)]:
            netlist = rspice.Netlist.parse_spice(
                f"Refinement\n.param gain={nominal}\nV1 in 0 DC 1 AC 1\n"
                f"E1 out 0 in 0 {{{expression}}}\n.end\n"
            )
            if nominal == 0.0:
                with pytest.raises(rspice.SimulationError, match="could not resolve"):
                    engine.run_sensitivity(netlist, "out", "gain", nominal)
                with pytest.raises(rspice.SimulationError, match="could not resolve"):
                    engine.run_sensitivity_ac(netlist, "out", "gain", nominal, [1.0])
            else:
                expected = 100.0 * np.exp(100.0)
                assert engine.run_sensitivity(netlist, "out", "gain", nominal) == pytest.approx(expected, rel=1e-5)
                assert engine.run_sensitivity_ac(netlist, "out", "gain", nominal, [1.0]) == pytest.approx([expected], rel=1e-5)

    def test_parameter_sensitivity_does_not_infer_domains_from_trial_failures(self, engine):
        netlist = rspice.Netlist.parse_spice(
            "Invalid trial\n.param gain=0\nV1 in 0 DC 1 AC 1\n"
            "E1 out 0 in 0 {1+gain}\nVFAIL conflict 0 1\n"
            "RFAIL conflict 0 {if(gain<0,0,1)}\n.end\n"
        )
        with pytest.raises(rspice.SimulationError, match="last trial failure"):
            engine.run_sensitivity(netlist, "out", "gain", 0.0)
        with pytest.raises(rspice.SimulationError, match="last trial failure"):
            engine.run_sensitivity_ac(netlist, "out", "gain", 0.0, [1.0])

    def test_zero_model_parameter_sensitivity_resolves_body_effect(self, engine):
        netlist = rspice.Netlist.parse_spice(
            "MOS body effect\nVG gate 0 DC 2 AC 1\nVD drain 0 2\nVB body 0 -1\n"
            "M1 drain gate 0 body NM W=1u L=1u\n"
            ".model NM NMOS(LEVEL=1 VTO=1 KP=1m GAMMA=0 PHI=0.6)\n.end\n"
        )
        expected = 1e-3 * (np.sqrt(1.6) - np.sqrt(0.6))
        dc = engine.run_sensitivity_dc_complete(
            netlist, "VD", filters=["NM:GAMMA"], output_is_current=True
        )
        ac = engine.run_sensitivity_ac_complete(
            netlist, "VD", [1.0, 1e9], filters=["NM:GAMMA"], output_is_current=True
        )
        assert dc.get("NM:GAMMA").absolute == pytest.approx(expected, rel=1e-5)
        assert ac.get("NM:GAMMA").absolute == pytest.approx([expected + 0j] * 2, rel=1e-5)

    def test_parameter_ac_magnitude_sensitivity_uses_the_nominal_phasor(self, engine):
        netlist = rspice.Netlist.parse_spice(
            "AC null\n.param gain=1\nV1 in 0 AC 1 60\nE1 out 0 in 0 {gain}\n.end\n"
        )
        with pytest.raises(rspice.SimulationError, match="nondifferentiable-magnitude"):
            engine.run_sensitivity_ac(netlist, "out", "gain", 0.0, [1.0])
        for nominal in [-1e-4, 1e-4]:
            values = engine.run_sensitivity_ac(
                netlist, "out", "gain", nominal, [1.0, 2.0], delta=1e-3
            )
            assert values == pytest.approx([np.sign(nominal)] * 2, rel=2e-12)

    def test_zero_output_availability_survives_python_and_pickle(self, engine):
        netlist = rspice.Netlist.parse_spice("Zero output\nV1 out 0 DC 0 AC 0\nR1 out 0 1\n.end\n")
        dc = engine.run_sensitivity_dc_complete(netlist, "out")
        assert dc.top() == []
        assert any(entry.absolute == 1.0 for entry in dc.sensitivities)
        for entry in dc.sensitivities:
            restored = pickle.loads(pickle.dumps(entry))
            assert restored.absolute == entry.absolute
            assert restored.normalized is None
            assert restored.percent_per_percent is None
            assert restored.normalized_unavailability == "zero-output"
        assert dc.document()["payload"]["entries"][0]["normalized"] == {"unavailable": "zero-output"}

        ac = engine.run_sensitivity_ac_complete(netlist, "out", [1.0, 2.0])
        assert ac.top(0) == []
        amplitude = next(entry for entry in ac.sensitivities if entry.absolute[0] == 1.0)
        for trace in [amplitude, pickle.loads(pickle.dumps(amplitude))]:
            assert np.all(trace.absolute == 1.0)
            assert np.all(np.isnan(trace.normalized))
            assert np.all(np.isnan(trace.magnitude))
            assert np.all(np.isnan(trace.phase))
            assert np.all(np.isnan(trace.db))
            assert trace.normalized_unavailability == ["zero-output"] * 2
            assert trace.magnitude_unavailability == ["nondifferentiable-magnitude"] * 2
            assert trace.phase_unavailability == ["zero-output"] * 2
            assert trace.phase_degrees_unavailability == ["zero-output"] * 2
            assert trace.db_unavailability == ["zero-output"] * 2
        assert ac.document()["schemaVersion"] == 5
        with pytest.raises(ValueError, match="rerun"):
            rspice.ElementSensitivity._unpickle(("V1", "V1", "VoltageSource", "dc"), 0.0, 1.0, 0.0)

    @pytest.mark.parametrize("current", [1e-300, 1e200])
    def test_decibel_sensitivity_preserves_extreme_output_scales(self, engine, current):
        netlist = rspice.Netlist.parse_spice(f"Scale\nI1 0 out AC {current}\nR1 out 0 1\n.end\n")
        trace = engine.run_sensitivity_ac_complete(netlist, "out", [1.0], filters=["R1"]).get("R1")
        assert trace.db == pytest.approx([20.0 / np.log(10.0)], rel=2e-10)
        assert trace.db_unavailability == [None]

    def test_sensitivity_ranking_compares_large_complex_magnitudes(self):
        def trace(name, component):
            return rspice.AcSensitivity._unpickle(
                (name, name, "Resistor", "value"), 1.0,
                ([(1.0, 0.0)], [(component, component)]),
                ([1.0], [0.0], [0.0]), (1, [None], [None], [None], [None]),
            )
        result = rspice.AcSensitivityResult._unpickle(
            "V(out)", [1.0], [(1.0, 0.0)], [trace("A", 1.3e308), trace("B", 1.7e308)]
        )
        assert [value.vector_name for value in result.top(0)] == ["B", "A"]

    def test_linearized_sensitivity_reports_all_elements(self, engine, divider):
        result = engine.run_sensitivity_linearized(divider, "out")
        assert isinstance(result, rspice.SensitivityResult)
        assert result.output_value == pytest.approx(5.0, abs=1e-9)
        assert len(result) >= 2
        r1 = result.get("R1")
        r2 = result.get("r2")
        assert isinstance(r1, rspice.ElementSensitivity)
        assert r1.element_type == "Resistor"
        assert r1.absolute == pytest.approx(-0.0025, rel=1e-5)
        assert r2.absolute == pytest.approx(0.0025, rel=1e-5)
        assert abs(result.top(1)[0].normalized) >= abs(result.top(2)[1].normalized)
        with pytest.raises(KeyError):
            result.get("missing")

    def test_engine_run_executes_dc_sensitivity_directive(self, engine):
        netlist = rspice.Netlist.parse(
            """* SENS directive
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.sens V(out)
.end
"""
        )
        report = engine.run(netlist)
        assert report.sensitivity is not None
        assert report.sensitivity.output_value == pytest.approx(5.0, abs=1e-9)
        assert report.analyses_run == ["sens"]

    def test_complete_dc_sensitivity_is_netlist_wide_and_filterable(self, engine):
        netlist = rspice.Netlist.parse(
            """* Complete DC sensitivity
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.end
"""
        )
        result = engine.run_sensitivity_dc_complete(netlist, "out", filters=["R*"])
        assert isinstance(result, rspice.SensitivityResult)
        assert result.output == "V(2)"
        assert result.output_value == pytest.approx(5.0, abs=1e-9)
        assert result.vector_names == ["R1", "R2"]
        assert result.get("R1").vector_name == "R1"
        assert result.get("R1").absolute == pytest.approx(-2.5e-3, rel=1e-6)
        assert result.get("R2").absolute == pytest.approx(2.5e-3, rel=1e-6)
        assert result.get("R1").normalized == pytest.approx(-0.5, rel=1e-6)

    def test_complete_dc_sensitivity_supports_branch_current_output(self, engine):
        netlist = rspice.Netlist.parse(
            """* Branch current DC sensitivity
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.end
"""
        )
        result = engine.run_sensitivity_dc_complete(
            netlist, "V1", filters=["R1"], output_is_current=True
        )
        assert result.output == "I(V1)"
        assert result.output_value == pytest.approx(-5e-3, rel=1e-9)
        assert result.vector_names == ["R1"]
        assert result.get("R1").absolute == pytest.approx(2.5e-6, rel=1e-6)

    def test_engine_run_executes_filtered_branch_current_dc_sensitivity(self, engine):
        netlist = rspice.Netlist.parse(
            """* Filtered branch-current SENS directive
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.sens I(V1) R1
.end
"""
        )
        report = engine.run(netlist)
        assert report.sensitivity is not None
        assert report.sensitivity.output == "I(V1)"
        assert report.sensitivity.vector_names == ["R1"]
        assert report.sensitivity.get("R1").absolute == pytest.approx(2.5e-6, rel=1e-6)
        assert report.analyses_run == ["sens"]
        assert report.skipped == []

    def test_divider_sensitivity_matches_analytic(self, engine, param_divider):
        # V(out) = 10 * 1k / (rval + 1k); dV/drval at 1k = -10*1k/(2k)^2.
        sens = engine.run_sensitivity(param_divider, "out", "rval", 1000.0)
        assert sens == pytest.approx(-2.5e-3, rel=0.01)

    def test_unbound_param_raises(self, engine, divider):
        with pytest.raises(rspice.SimulationError):
            engine.run_sensitivity(divider, "out", "R1", 1000.0)

    def test_validation(self, engine, param_divider):
        with pytest.raises(ValueError):
            engine.run_sensitivity(param_divider, "out", "rval", float("nan"))
        with pytest.raises(ValueError):
            engine.run_sensitivity(param_divider, "out", "rval", 1000.0, delta=-1.0)
        with pytest.raises(ValueError):
            engine.run_sensitivity_ac(
                param_divider, "out", "rval", float("nan"), [10.0, 100.0]
            )
        with pytest.raises(ValueError):
            engine.run_sensitivity_ac(
                param_divider, "out", "rval", 1000.0, [10.0, 100.0], delta=-1.0
            )

    def test_ac_sensitivity_shape(self, engine):
        netlist = rspice.Netlist.parse(
            """* Parametric RC lowpass
.param rval=1k
V1 in 0 AC 1
R1 in out {rval}
C1 out 0 1u
.end
"""
        )
        freqs = [10.0, 159.155, 1000.0]
        sens = engine.run_sensitivity_ac(netlist, "out", "rval", 1000.0, freqs)
        assert isinstance(sens, np.ndarray)
        assert len(sens) == 3
        # At DC the divider has no R dependence; at the corner it does.
        assert abs(sens[1]) > abs(sens[0])

    def test_complete_ac_sensitivity_is_complex_structured_and_filterable(self, engine):
        netlist = rspice.Netlist.parse(
            """* Complete AC sensitivity
V1 in 0 DC 0 AC 1 0
R1 in out 1k
R2 out 0 1k
.end
"""
        )
        result = engine.run_sensitivity_ac_complete(
            netlist, "out", [1.0, 1000.0], filters=["R*"]
        )

        assert isinstance(result, rspice.AcSensitivityResult)
        assert result.output == "V(2)"
        assert result.vector_names == ["R1", "R2"]
        assert result.frequencies.dtype == np.float64
        assert result.output_complex.dtype == np.complex128
        assert result.output_complex == pytest.approx([0.5 + 0j, 0.5 + 0j])

        r1 = result.get("r1")
        assert isinstance(r1, rspice.AcSensitivity)
        assert r1.absolute.dtype == np.complex128
        assert r1.absolute == pytest.approx([-2.5e-4 + 0j] * 2, rel=1e-6)
        assert r1.normalized == pytest.approx([-0.5 + 0j] * 2, rel=1e-6)
        assert r1.percent_per_percent == pytest.approx([-0.5 + 0j] * 2, rel=1e-6)
        assert r1.magnitude == pytest.approx([-2.5e-4] * 2, rel=1e-6)
        assert r1.phase == pytest.approx([0.0, 0.0], abs=1e-12)
        assert result.top(0, 1)[0].vector_name in {"R1", "R2"}
        with pytest.raises(KeyError):
            result.get("missing")
        with pytest.raises(IndexError):
            result.top(2)

    def test_complete_ac_sensitivity_supports_branch_current_output(self, engine):
        netlist = rspice.Netlist.parse(
            """* Branch current sensitivity
V1 in 0 AC 1
R1 in out 1k
R2 out 0 1k
.end
"""
        )
        result = engine.run_sensitivity_ac_complete(
            netlist,
            "V1",
            [1000.0],
            filters=["R1"],
            output_is_current=True,
        )
        assert result.output == "I(V1)"
        assert result.get("R1").absolute[0] == pytest.approx(2.5e-7 + 0j, rel=1e-6)

    def test_engine_run_executes_complete_ac_sensitivity_directive(self, engine):
        netlist = rspice.Netlist.parse(
            """* AC SENS directive
V1 in 0 AC 1
R1 in out 1k
R2 out 0 1k
.sens V(out) R* AC LIN 3 1 10
.end
"""
        )
        report = engine.run(netlist)
        assert report.sensitivity is None
        assert isinstance(report.sensitivity_ac, rspice.AcSensitivityResult)
        assert report.sensitivity_ac.vector_names == ["R1", "R2"]
        assert report.analyses_run == ["sens_ac"]
        assert report.skipped == []
