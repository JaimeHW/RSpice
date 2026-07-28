"""Monte Carlo analysis and its summary statistics."""

import numpy as np
import pytest

import rspice



class TestMonteCarlo:
    def test_engine_run_executes_mc_directive(self, engine):
        netlist = rspice.Netlist.parse(
            """* MC directive
.param rval=1k
V1 in 0 10
R1 in out {rval}
R2 out 0 1k
.mc 12 seed 9 dist gauss spread 0.01 params rval
.end
"""
        )
        report = engine.run(netlist)
        assert report.monte_carlo is not None
        assert report.monte_carlo.num_runs == 12
        assert report.analyses_run == ["mc"]

    def test_statistics_match_theory(self, engine, param_divider):
        netlist = rspice.Netlist.parse(
            """* Two-parameter divider
.param r1v=1k
.param r2v=1k
V1 in 0 10
R1 in out {r1v}
R2 out 0 {r2v}
.end
"""
        )
        mc = engine.run_monte_carlo(netlist, num_runs=400, seed=7)
        assert mc.num_runs == 400
        assert mc.num_failures == 0
        stats = mc.get_variable("V(OUT)")
        assert stats is not None
        # dV/V per resistor = 1/4 relative; two independent 1% gaussians:
        # sigma_V = 5 V * 0.01 * sqrt(2)/2 ≈ 35.4 mV.
        assert stats.mean == pytest.approx(5.0, abs=0.02)
        assert stats.std_dev == pytest.approx(0.0354, rel=0.25)
        assert stats.min < stats.mean < stats.max
        assert stats.percentile(1) < stats.percentile(99)
        lo, hi = stats.three_sigma_range
        assert lo < stats.mean < hi

    def test_seed_reproducibility(self, engine, param_divider):
        a = engine.run_monte_carlo(param_divider, num_runs=50, seed=42)
        b = engine.run_monte_carlo(param_divider, num_runs=50, seed=42)
        va = a.get_variable("V(OUT)")
        vb = b.get_variable("V(OUT)")
        assert va.mean == vb.mean
        np.testing.assert_array_equal(va.samples, vb.samples)

    def test_random_seed_when_omitted(self, engine, param_divider):
        a = engine.run_monte_carlo(param_divider, num_runs=20)
        assert a.num_runs == 20

    def test_uniform_distribution_bounds(self, engine, param_divider):
        mc = engine.run_monte_carlo(
            param_divider, num_runs=200, seed=3, distribution="uniform", spread=0.05
        )
        stats = mc.get_variable("V(OUT)")
        # rval in [950, 1050] -> V(out) in [10*1000/2050, 10*1000/1950].
        assert stats.min >= 10 * 1000 / 2051
        assert stats.max <= 10 * 1000 / 1949

    def test_param_filter(self, engine):
        netlist = rspice.Netlist.parse(
            """* Two-parameter divider
.param r1v=1k
.param r2v=1k
V1 in 0 10
R1 in out {r1v}
R2 out 0 {r2v}
.end
"""
        )
        mc = engine.run_monte_carlo(netlist, num_runs=100, seed=5, params=["r1v"])
        assert mc.num_runs == 100

    def test_validation(self, engine, param_divider):
        with pytest.raises(ValueError):
            engine.run_monte_carlo(param_divider, num_runs=0)
        with pytest.raises(ValueError):
            engine.run_monte_carlo(param_divider, num_runs=10, distribution="cauchy")
        with pytest.raises(ValueError):
            engine.run_monte_carlo(param_divider, num_runs=10, spread=-0.1)
        with pytest.raises(rspice.SimulationError):
            engine.run_monte_carlo(param_divider, num_runs=10, params=["nonexistent"])
