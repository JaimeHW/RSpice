"""Parametric stepping (`.STEP`)."""

import numpy as np
import pytest

import rspice



class TestStep:
    def test_engine_run_executes_step_directive(self, engine):
        netlist = rspice.Netlist.parse(
            """* STEP directive
.param rval=1k
V1 in 0 10
R1 in out {rval}
R2 out 0 1k
.step param rval list 1k 2k 5k
.end
"""
        )
        report = engine.run(netlist)
        assert report.step is not None
        assert report.step.primary_source.casefold() == "rval"
        np.testing.assert_array_equal(report.step.sweep_values, [1e3, 2e3, 5e3])
        assert report.step.voltage(2, "out") == pytest.approx(10 / 6, abs=1e-6)
        assert report.analyses_run == ["step"]

    def test_engine_run_executes_temperature_directive(self, engine, divider):
        netlist = rspice.Netlist.parse(
            """* TEMP directive
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.temp 25 100
.end
"""
        )
        report = engine.run(netlist)
        assert report.temperature is not None
        assert report.temperature.primary_source == "TEMP"
        np.testing.assert_array_equal(report.temperature.sweep_values, [25, 100])
        assert report.analyses_run == ["temp"]

    def test_step_varies_results(self, engine, param_divider):
        results = engine.run_step(param_divider, "rval", [1e3, 2e3, 5e3])
        assert len(results) == 3
        outs = [sol.voltage("out") for _, sol in results]
        assert outs[0] == pytest.approx(5.0, abs=1e-6)
        assert outs[1] == pytest.approx(10 * 1000 / 3000, abs=1e-6)
        assert outs[2] == pytest.approx(10 * 1000 / 6000, abs=1e-6)

    def test_step_element_name_raises(self, engine, divider):
        with pytest.raises(rspice.SimulationError):
            engine.run_step(divider, "R1", [1e3, 2e3])

    def test_step_empty_values_raise(self, engine, param_divider):
        with pytest.raises(ValueError, match="must not be empty"):
            engine.run_step(param_divider, "rval", [])

    def test_percentile_rejects_invalid_values(self, engine, param_divider):
        stats = engine.run_monte_carlo(
            param_divider, num_runs=10, seed=7
        ).get_variable("V(OUT)")
        assert stats is not None
        for value in (-1.0, 101.0, float("nan"), float("inf")):
            with pytest.raises(ValueError, match="0 to 100"):
                stats.percentile(value)

    def test_step_non_finite_values_raise_valueerror(self, engine, param_divider):
        with pytest.raises(ValueError, match="finite"):
            engine.run_step(param_divider, "rval", [1e3, float("nan")])
