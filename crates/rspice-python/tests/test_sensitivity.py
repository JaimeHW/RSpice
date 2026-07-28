"""DC and AC sensitivity analysis (`.SENS`)."""

import numpy as np
import pytest

import rspice



class TestSensitivity:
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
