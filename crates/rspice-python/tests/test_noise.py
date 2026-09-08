"""Small-signal noise analysis (`.NOISE`)."""

import math
import pickle

import pytest

import rspice

K_BOLTZMANN = 1.380649e-23


class TestNoise:
    @pytest.mark.parametrize("level, expected", [(11, 3.929629740476495e-10), (12, 3.929629740665435e-10)])
    def test_vbic13_extrinsic_flicker_and_identity(self, engine, level, expected):
        substrate = " 0" if level == 12 else ""
        netlist = rspice.Netlist.parse_spice(
            f"""* VBIC extrinsic flicker
Vcc vcc 0 3
Rc vcc c 1k
Vb drive 0 DC 0.7 AC 1
Rb drive b 1k
Vth th 0 0
Q1 c b 0{substrate} th vm SW_ET=0 M=3
.model vm NPN(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 GMIN=0 IBEIP=0 ISP=0 TNOM=27 WBE=0 KFN=1e-8 AFN=1.5 BFN=0.8)
.temp 27
.options gmin=0
.end
"""
        )
        point = engine.run_noise(netlist, "c", [1], temperature=300.15)[0]
        assert point.output_noise_density == pytest.approx(expected, rel=2e-7, abs=1e-25)
        source = next(c for c in point.contributions if c.mechanism == "FN_BEX")
        assert source.device_name.upper() == "Q1"
        assert source.noise_type == "Flicker"
        restored = pickle.loads(pickle.dumps(source))
        assert restored.mechanism == source.mechanism
        assert restored.output_contribution == source.output_contribution
        legacy = rspice.NoiseContribution._unpickle("Q1", "Shot", 1e-20, 10.0)
        assert legacy.mechanism is None

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
