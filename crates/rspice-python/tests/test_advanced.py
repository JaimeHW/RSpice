"""Noise, pole-zero, Monte Carlo, sensitivity, step, transfer function."""

import math

import numpy as np
import pytest

import rspice

K_BOLTZMANN = 1.380649e-23


class TestNoise:
    def test_resistor_thermal_noise_matches_theory(self, engine, rc_lowpass):
        # Output noise of R || C: S_vo(f) = 4kTR / (1 + (f/fc)^2).
        temp = 300.0
        results = engine.run_noise(
            rc_lowpass, "out", [10.0, 159.155, 10_000.0], temperature=temp
        )
        assert len(results) == 3
        s_thermal = 4.0 * K_BOLTZMANN * temp * 1e3
        fc = 1.0 / (2 * math.pi * 1e3 * 1e-6)
        for r in results:
            expected = s_thermal / (1.0 + (r.frequency / fc) ** 2)
            assert r.output_noise_density == pytest.approx(expected, rel=0.01)
        dom = results[0].dominant_source()
        assert dom is not None
        assert dom.device_name.upper() == "R1"
        assert dom.noise_type == "Thermal"

    def test_output_node_by_index(self, engine, rc_lowpass):
        by_name = engine.run_noise(rc_lowpass, "out", [100.0])
        by_index = engine.run_noise(rc_lowpass, 2, [100.0])
        assert by_name[0].output_noise_density == pytest.approx(
            by_index[0].output_noise_density, rel=1e-12
        )

    def test_unknown_node_raises_keyerror(self, engine, rc_lowpass):
        with pytest.raises(KeyError):
            engine.run_noise(rc_lowpass, "nonexistent", [100.0])

    def test_validation(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_noise(rc_lowpass, "out", [])
        with pytest.raises(ValueError):
            engine.run_noise(rc_lowpass, "out", [100.0], temperature=-10.0)

    def test_run_evaluates_noise_measurements(self, engine):
        netlist = rspice.Netlist.parse(
            """* Noise measurement
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1u
.noise V(out) V1 LIN 3 10 1000
.meas noise max_onoise MAX onoise_spectrum
.end
"""
        )

        report = engine.run(netlist)

        assert report.noise is not None
        assert report.num_measurements == 1
        measurement = report.measurement("max_onoise")
        assert measurement is not None
        assert measurement.analysis == "NOISE"
        assert measurement.passed
        assert measurement.value > 0.0
        assert report.all_passed


class TestPoleZero:
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


class TestMonteCarlo:
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


class TestSensitivity:
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


class TestStep:
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
        with pytest.raises(rspice.SimulationError, match="no sweep values"):
            engine.run_step(param_divider, "rval", [])

    def test_step_non_finite_values_raise_valueerror(self, engine, param_divider):
        with pytest.raises(ValueError, match="finite"):
            engine.run_step(param_divider, "rval", [1e3, float("nan")])

class TestTransferFunction:
    def test_divider_gain_and_impedances(self, engine, divider):
        tf = engine.run_transfer_function(divider, "out", "V1")
        assert tf.gain == pytest.approx(0.5, rel=1e-6)
        assert tf.input_impedance == pytest.approx(2000.0, rel=1e-6)
        # Thevenin output impedance: 1k || 1k = 500.
        assert tf.output_impedance == pytest.approx(500.0, rel=1e-6)
        assert tf.gain_db == pytest.approx(20 * math.log10(0.5), rel=1e-6)

    def test_unknown_source_raises(self, engine, divider):
        with pytest.raises(rspice.SimulationError):
            engine.run_transfer_function(divider, "out", "V99")
