"""AC analysis: name access, error discipline, sweeps, branch currents."""

import math
import pickle

import numpy as np
import pytest

import rspice


AC_DATA_DECK = """
V1 in 0 AC 1
R1 in out 1k
C1 out 0 159.154943091895p
.AC DATA=pts
.DATA pts
+ FREQ
+ 1
+ 1k
+ 1meg
.ENDDATA
.END
"""

FC = 1.0 / (2 * math.pi * 1e3 * 1e-6)  # RC corner: 159.155 Hz


class TestAcBasics:
    @pytest.mark.parametrize("magnitude", [1e-15, 1e-16, 1e-300])
    def test_nonzero_current_excitations_have_no_absolute_amplitude_cutoff(self, engine, magnitude):
        netlist = rspice.Netlist.parse_spice(
            f"* small current excitation\nI1 0 out DC 0 AC {magnitude} 37\nR1 out 0 1\n.end\n"
        )
        actual = engine.run_ac(netlist, [1e3]).voltage_complex("out")[0]
        expected = complex(math.cos(math.radians(37)), math.sin(math.radians(37)))
        assert abs(actual / magnitude - expected) < 1e-14

    @pytest.mark.parametrize("terminals", ["out out", "0 0"])
    def test_tied_current_terminals_preserve_other_ac_excitations(self, engine, terminals):
        netlist = rspice.Netlist.parse_spice(
            f"* tied AC current\nI1 0 out DC 0 AC 1 37\nI2 {terminals} DC 0 AC 1e100 37\nR1 out 0 1\n.end\n"
        )
        actual = engine.run_ac(netlist, [1e3]).voltage_complex("out")[0]
        expected = complex(math.cos(math.radians(37)), math.sin(math.radians(37)))
        assert abs(actual - expected) < 1e-14

    @pytest.mark.parametrize(
        "waveform",
        ["SIN(.65 .05 1meg)", "PULSE(.65 .7 0 1n 1n 5n 10n)", "PWL(0 .65 20n .7)"],
    )
    @pytest.mark.parametrize("source", ["V1 out 0", "I1 0 out"])
    def test_waveform_ac_terms_preserve_dc_bias(self, engine, waveform, source):
        for dc, expected in [("", 0.65), ("DC 0", 0.0)]:
            netlist = rspice.Netlist.parse_spice(
                f"* waveform bias\n{source} {waveform} AC 2 90 {dc}\nR1 out 0 1\n.end\n"
            )
            op = engine.run_dc_op(netlist)
            index = next(
                i for i, name in enumerate(op.node_names) if name.lower() == "out"
            )
            assert op.node_voltages[index] == pytest.approx(expected, abs=1e-12)
            ac = engine.run_ac(netlist, [1e3])
            assert ac.voltage_complex("out")[0] == pytest.approx(2j, abs=1e-12)

    @pytest.mark.parametrize("instance", ["M=3", "AREA=3", "AREA=1.5 M=2"])
    @pytest.mark.parametrize("kind, polarity", [("NPN", 1), ("PNP", -1)])
    def test_legacy_itf_scaling_preserves_input_charge(self, instance, kind, polarity):
        netlist = rspice.Netlist.parse_spice(f"""* Legacy ITF instance scaling
Vc c 0 {polarity*.72}
Vb b 0 PWL(0 {polarity*.65} 20n {polarity*.7}) AC 1 DC {polarity*.65}
Q1 c b 0 qm {instance}
.model qm {kind}(LEVEL=1 IS=1e-16 BF=100 TF=1n XTF=3 VTF=10 ITF=100u)
.options gmin=0
.end
""")
        engine = rspice.Engine(rspice.SimulationConfig(convergence=rspice.ConvergenceConfig(gmin_target=0)))
        result = engine.run_ac(netlist, [1e8])
        # Independent GP charge derivative using the SI thermal voltage.
        vt = 300.15*1.380649e-23/1.602176634e-19
        forward = 1e-16*math.expm1(.65/vt)
        conductance = 1e-16/vt*math.exp(.65/vt)
        fraction = forward/(forward+1e-4)
        extra = 3*math.exp((.65-.72)/14.4)*fraction**2
        capacitance = 1e-9*(conductance*(1+extra*(3-2*fraction))+forward*extra/14.4)
        expected = complex(-3*(conductance/100+1e-16/vt*math.exp((.65-.72)/vt)), -3*2*math.pi*1e8*capacitance)
        assert abs(result.branch_current_complex("Vb")[0]-expected) < 1e-11*abs(expected)

    @pytest.mark.parametrize("energy, dc, ac", [
        (0.0, [-1.8759381199128295e-05, -5.974425036580707e-06, 5.759162012220747e-06], [complex(7.475710679968615e-07, -7.2352565019688054e-06), complex(5.679107136074598e-07, 1.1568126579812277e-05), complex(-3.0678023251495346e-07, -5.895892422355515e-07)]),
        (-0.1, [-1.3203041813337315e-05, -4.205786531991028e-06, 4.052895950577105e-06], [complex(5.723237566711971e-07, -7.6979104375324e-06), complex(5.244002563240605e-07, 1.2426635278632646e-05), complex(-2.559863313225597e-07, -6.704596659558985e-07)]),
    ])
    def test_vbic_nonpositive_activation_energies_match_xyce(self, energy, dc, ac):
        netlist = rspice.Netlist.parse_spice(f"""* VBIC activation energies and signed source values
Vc c 0 .1
Vb b 0 +.7
Vs s 0 -.4
Vth th 0 DC 30 AC 1
Q1 c b 0 s th vm SW_ET=0 M=3
.model vm NPN(LEVEL=12 IS=1e-16 IBEI=1e-18 IBEN=1e-14 IBCI=1e-18 IBCN=1e-14 ISP=1e-15 IBEIP=1e-18 IBENP=1e-14 IBCIP=1e-16 IBCNP=1e-14 RCX=1 RCI=1 RBX=1 RBI=5 RE=1 RBP=5 RS=1 RTH=1000 CTH=1p CJE=1p CJC=2p CJEP=1p CJCP=1p TF=1n TR=2n TD=1n GMIN=0 TNOM=27 EA={energy} EAIE={energy} EAIC={energy} EAIS={energy} EANE={energy} EANC={energy} EANS={energy} EAP={energy})
.temp 27
.options gmin=0
.end
""")
        engine = rspice.Engine(rspice.SimulationConfig(convergence=rspice.ConvergenceConfig(gmin_target=0)))
        operating = engine.run_dc_op(netlist)
        small_signal = engine.run_ac(netlist, [1e8])
        for index, name in enumerate(["Vc", "Vb", "Vs"]):
            assert abs(operating.branch_current(name)-dc[index]) < 2e-6*max(abs(dc[index]), 1e-12)
            assert abs(small_signal.branch_current_complex(name)[0]-ac[index]) < 2e-6*max(abs(ac[index]), 1e-12)

    @pytest.mark.parametrize("vef, coefficient, collector, base", [
        (1e15, -0.049999999999999989,
         complex(11073210790.749592, -8091064247.664278),
         complex(91165705.28041226, 9220857782.629837)),
        (5.0, -0.0499999999995,
         complex(0.0010482816638976908, -0.0007608625817640124),
         complex(4.054942295825502e-6, 0.000859797326861502)),
        (5.0, -0.049999999999999989,
         complex(0.0010482816655048935, -0.0007608625829294785),
         complex(4.0549423005197475e-6, 0.0008597973281960521)),
    ])
    @pytest.mark.parametrize("level, kind, polarity", [(11, "NPN", 1), (12, "PNP", -1)])
    def test_vbic13_extreme_thermal_slope_retains_physical_currents(self, level, kind, polarity, vef, coefficient, collector, base):
        substrate = " 0" if level == 12 else ""
        netlist = rspice.Netlist.parse_spice(f"""* VBIC direct physical residual
Vc c 0 {polarity * 0.6}
Vb b 0 {polarity * 0.7}
Vth th 0 DC 20 AC 1
Q1 c b 0{substrate} th vm SW_ET=0 M=3
.model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 ISP=0 IBEIP=0 VEF={vef} VER=3 TCVEF={coefficient:.18f} TCVER=-0.02 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p CJE=1p CJC=1p TF=1n TR=2n QTF=0.3 TD=1n)
.temp 27
.options gmin=0
.end
""")
        engine = rspice.Engine(rspice.SimulationConfig(convergence=rspice.ConvergenceConfig(gmin_target=0)))
        result = engine.run_ac(netlist, [1e8])
        # Independent 80-digit evaluation, including the sub-femtovolt RBI drop.
        for branch, expected in [
            ("vc", polarity * collector),
            ("vb", polarity * base),
            ("vth", complex(-0.003, -0.0018849555921538759)),
        ]:
            assert result.branch_current_complex(branch)[0] == pytest.approx(expected, rel=2e-6)

    @pytest.mark.parametrize("level", [11, 12])
    @pytest.mark.parametrize("kind, polarity", [("NPN", 1), ("PNP", -1)])
    def test_vbic13_near_early_cutoff_with_transit_current_scaling(self, engine, level, kind, polarity):
        substrate = " 0" if level == 12 else ""
        netlist = rspice.Netlist.parse_spice(f"""* VBIC Early slopes and scaled transit current
Vc c 0 {polarity * 0.6}
Vb b 0 {polarity * 0.7}
Vth th 0 DC 20 AC 1
Q1 c b 0{substrate} th vm SW_ET=1 M=3
.model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 ISP=0 IBEIP=0 VEF=5 VER=3 TCVEF=-0.049999995000000005 TCVER=-0.049999995000000005 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 TMAXCLIP=100 CTH=1p CJE=1p CJC=1p TF=1n TR=2n QTF=0.3 XTF=2 VTF=2 ITF=1e-4 TD=1n QBM=1 NKF=0.4 IKF=1e-4 IKR=2e-4 AVC1=0.05 AVC2=0.3 TAVC=0.01)
.temp 27
.options gmin=0
.end
""")
        result = engine.run_ac(netlist, [1e8])
        # Independent Xyce 7.10 references, including physical generated heat.
        for branch, expected in [
            ("vc", polarity * complex(3.786701877360067e-5, -2.7352104484429096e-5)),
            ("vb", polarity * complex(-7.534236549061291e-7, 4.65210834241494e-5)),
            ("vth", complex(-0.0030220360332668655, -0.0018688434659563766)),
        ]:
            assert result.branch_current_complex(branch)[0] == pytest.approx(expected, rel=2e-6, abs=1e-15)

    @pytest.mark.parametrize("level", [11, 12])
    def test_vbic_tnf_thermal_derivative_matches_xyce(self, engine, level):
        substrate = " 0" if level == 12 else ""
        netlist = rspice.Netlist.parse_spice(f"""* VBIC TNF thermal derivative
Vc c 0 1.8
Vb b 0 0.7
Vth th 0 DC 20 AC 1
Q1 c b 0{substrate} th vm SW_ET=1 M=3 TRISE=20
.model vm NPN(LEVEL={level} IS=1e-16 NF=1.1 NR=1.2 ISRR=0.7 TNF=0.001 XISR=1.8 DEAR=0.1 IBEI=1e-18 IBCI=1e-18 IBEIP=0 ISP=0 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p TD=1n TF=1n TR=2n)
.temp 27
.options gmin=0
.end
""")
        result = engine.run_ac(netlist, [1e8])
        for branch, expected in [
            ("vc", complex(-1.084013990699429e-6, 7.885275583603683e-7)),
            ("vb", complex(-8.501427197947536e-7, -9.013856542434545e-7)),
            ("vth", complex(-0.0029974552824991404, -0.0018863758535638618)),
        ]:
            assert result.branch_current_complex(branch)[0] == pytest.approx(expected, rel=2e-6, abs=1e-15)

    @pytest.mark.parametrize("level", [11, 12])
    def test_vbic13_signed_reverse_charge_and_delay_matches_xyce(self, engine, level):
        substrate = " 0" if level == 12 else ""
        netlist = rspice.Netlist.parse_spice(f"""* Signed VBIC reverse charge and delay
Vc c 0 1
Vb b 0 DC -0.1 AC 1
Vth th 0 20
Q1 c b 0{substrate} th vm SW_ET=0 M=3
.model vm NPN(LEVEL={level} IS=1e-8 ISRR=0.7 IBEI=0 IBCI=0 IBEIP=0 ISP=0 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=10 RS=10 GMIN=0 TNOM=27 IKF=1e-10 IKR=1e-10 QBM=1 NKF=0.4 VEF=3 VER=4 TF=2n TR=1n TD=1n QTF=0.5 XTF=10 ITF=1e-3)
.temp 27
.options gmin=0
.end
""")
        result = engine.run_ac(netlist, [1e8])
        for branch, expected in [
            ("vc", complex(-1.2339551736312433e-6, 1.4936538192949223e-6)),
            ("vb", complex(-2.595701431573616e-11, -4.092821900053193e-6)),
        ]:
            assert result.branch_current_complex(branch)[0] == pytest.approx(expected, rel=2e-6, abs=1e-15)

    def test_vbic13_early_voltage_cutoff_matches_xyce(self, engine):
        netlist = rspice.Netlist.parse_spice(
            """* VBIC Early-voltage cutoff
Vc c 0 1.8
Vb b 0 0.7
Vth th 0 DC 20 AC 1
Q1 c b 0 th vm SW_ET=1
.model vm NPN(LEVEL=11 VEF=5 VER=3 TCVEF=-0.05 TCVER=-0.05 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVC1=0.05 AVC2=0.3 TAVC=0.01 TD=1n RTH=1000 TCRTH=0.005 TMAXCLIP=100 CTH=1p GMIN=1u TNOM=27)
.temp 27
.end
"""
        )
        result = engine.run_ac(netlist, [1e8])
        for branch, expected in [
            ("vc", complex(-8.663038807350953e-6, 6.275760707091533e-6)),
            ("vb", complex(4.317831416641038e-7, -4.178850145275066e-7)),
            ("vth", complex(-0.0008111544184809163, -0.000639322846682267)),
        ]:
            assert result.branch_current_complex(branch)[0] == pytest.approx(
                expected, rel=2e-7, abs=1e-14
            )

    @pytest.mark.parametrize("kind, polarity", [("NPN", 1), ("PNP", -1)])
    def test_vbic13_delayed_avalanche_matches_xyce(self, engine, kind, polarity):
        netlist = rspice.Netlist.parse_spice(
            f"""* Delayed VBIC avalanche
Vc c 0 {polarity * 1.8}
Vb b 0 DC {polarity * 0.7} AC 1
Q1 c b 0 vm SW_ET=0
.model vm {kind}(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVC1=0.05 AVC2=0.3 TD=1n GMIN=1u TNOM=27)
.temp 27
.end
"""
        )
        result = engine.run_ac(netlist, [1e8])
        for branch, expected in [
            ("vc", complex(-0.001774651889771782, 0.0012868112960925493)),
            ("vb", complex(9.590581606050419e-5, -9.066589478545957e-5)),
        ]:
            assert result.branch_current_complex(branch)[0] == pytest.approx(
                expected, rel=2e-7, abs=1e-14
            )

    @pytest.mark.parametrize("level", [11, 12])
    def test_vbic13_clipped_thermal_admittance_without_hidden_capacitance(
        self, engine, level
    ):
        substrate = " 0" if level == 12 else ""
        netlist = rspice.Netlist.parse(
            f"""* VBIC13 thermal AC
Vth th 0 DC 74 AC 1
Q1 0 0 0{substrate} th vm SW_ET=0
.model vm NPN(LEVEL={level} RTH=1000 TCRTH=0.005 TMAXCLIP=100 TNOM=27)
.temp 27
.end
"""
        )
        result = engine.run_ac(netlist, [1e3])
        tail = math.exp(-2)
        resistance = 1000 * (1 + 0.005 * (100 - tail - 27))
        expected = -(1 / resistance - 74 * 5 * tail / resistance**2)
        current = result.branch_current_complex("Vth")[0]
        assert current.real == pytest.approx(expected, abs=1e-12)
        assert current.imag == 0

    def test_lowpass_rolloff(self, engine, rc_lowpass):
        freqs = np.logspace(0, 5, 11)
        ac = engine.run_ac(rc_lowpass, freqs.tolist())
        mag = ac.voltage_magnitude("out")
        assert mag[0] == pytest.approx(1.0, abs=1e-3)
        assert mag[-1] < 2e-3
        np.testing.assert_allclose(ac.frequencies, freqs)
        assert ac.num_frequencies == 11

    def test_accepts_numpy_array_frequencies(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, np.logspace(0, 4, 5))
        assert ac.num_frequencies == 5

    def test_magnitude_at_corner_is_3db(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [FC])
        assert ac.voltage_db("out")[0] == pytest.approx(-3.0103, abs=0.01)
        assert ac.voltage_phase_degrees("out")[0] == pytest.approx(-45.0, abs=0.1)

    def test_complex_phasors(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [FC])
        h = ac.voltage_complex("out")
        assert h.dtype == np.complex128
        expected = 1.0 / (1.0 + 1j)
        assert h[0] == pytest.approx(expected, abs=1e-3)

    def test_name_and_index_agree(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0, 100.0])
        names = ac.node_names
        assert "OUT" in [n.upper() for n in names]
        idx = [n.upper() for n in names].index("OUT") + 1
        np.testing.assert_allclose(
            ac.voltage_magnitude("out"), ac.voltage_magnitude(idx)
        )

    def test_ground_is_zero(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        assert ac.voltage_magnitude(0)[0] == 0.0
        assert ac.voltage_magnitude("gnd")[0] == 0.0


class TestAcErrorDiscipline:
    def test_out_of_range_node_raises_indexerror(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        with pytest.raises(IndexError):
            ac.voltage_magnitude(99)
        with pytest.raises(IndexError):
            ac.voltage_db(99)
        with pytest.raises(IndexError):
            ac.voltage_complex(99)

    def test_unknown_node_name_raises_keyerror(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        with pytest.raises(KeyError):
            ac.voltage_magnitude("nonexistent")

    def test_bad_freq_index_raises_indexerror(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        with pytest.raises(IndexError):
            ac.magnitude_at(999, "out")
        with pytest.raises(IndexError):
            ac.phase_at(999, "out")
        assert ac.magnitude_at(0, "out") > 0.99

    def test_empty_frequencies_raise_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac(rc_lowpass, [])

    def test_negative_frequency_raises_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac(rc_lowpass, [10.0, -5.0])

    def test_non_finite_frequency_raises_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac(rc_lowpass, [10.0, float("nan")])


class TestAcBranchCurrents:
    def test_source_current_magnitude(self, engine, rc_lowpass):
        # Far above fc the capacitor is a short: |I| ≈ 1 V / 1 kΩ.
        ac = engine.run_ac(rc_lowpass, [1e6])
        assert "V1" in [n.upper() for n in ac.branch_names]
        i_mag = ac.branch_current_magnitude("V1")
        assert i_mag[0] == pytest.approx(1e-3, rel=1e-2)
        i_cplx = ac.branch_current_complex("V1")
        assert i_cplx.dtype == np.complex128
        assert abs(i_cplx[0]) == pytest.approx(i_mag[0], rel=1e-12)

    def test_unknown_branch_raises_keyerror(self, engine, rc_lowpass):
        ac = engine.run_ac(rc_lowpass, [10.0])
        with pytest.raises(KeyError):
            ac.branch_current_complex("V99")


class TestAcSweeps:
    def test_run_ac_sweep_decade(self, engine, rc_lowpass):
        ac = engine.run_ac_sweep(rc_lowpass, "dec", 10, 1.0, 1e5)
        freqs = ac.frequencies
        assert freqs[0] == pytest.approx(1.0)
        assert freqs[-1] == pytest.approx(1e5, rel=1e-6)
        # 5 decades x 10 points + endpoint
        assert len(freqs) == 51

    def test_ac_frequencies_matches_run_ac_sweep(self, engine, rc_lowpass):
        freqs = rspice.ac_frequencies("dec", 10, 1.0, 1e5)
        ac = engine.run_ac_sweep(rc_lowpass, "dec", 10, 1.0, 1e5)
        np.testing.assert_allclose(ac.frequencies, freqs)

    def test_linear_sweep(self, engine, rc_lowpass):
        ac = engine.run_ac_sweep(rc_lowpass, "lin", 11, 100.0, 200.0)
        np.testing.assert_allclose(ac.frequencies, np.linspace(100.0, 200.0, 11))

    def test_invalid_variation_raises_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac_sweep(rc_lowpass, "log", 10, 1.0, 1e5)

    def test_invalid_sweep_range_raises_valueerror(self, engine, rc_lowpass):
        with pytest.raises(ValueError):
            engine.run_ac_sweep(rc_lowpass, "lin", 2, 200.0, 100.0)

    def test_ac_frequencies_validates(self):
        with pytest.raises(ValueError):
            rspice.ac_frequencies("nope", 10, 1.0, 1e5)


class TestAcData:
    @pytest.mark.parametrize("frequency_column", ["FREQ", "HERTZ"])
    @pytest.mark.parametrize("route", ["direct", "deck"])
    def test_data_rows_apply_parameters_at_each_frequency(
        self, engine, frequency_column, route
    ):
        netlist = rspice.Netlist.parse(f"""* Parameter-aware AC DATA
.param rval=1k
V1 in 0 DC 0 AC 1
R1 in out {{rval}}
R2 out 0 1k
.ac DATA=points
.data points {frequency_column} rval
10 1k
20 2k
30 3k
.enddata
.end
""")
        result = (
            engine.run_ac_data(netlist, "points")
            if route == "direct" else engine.run(netlist).ac
        )
        np.testing.assert_array_equal(result.frequencies, [10.0, 20.0, 30.0])
        np.testing.assert_allclose(
            result.voltage_complex("out"), [0.5, 1.0 / 3.0, 0.25], atol=1e-12
        )

    def test_named_data_table_frequency_grid(self, engine):
        netlist = rspice.Netlist.parse(AC_DATA_DECK)
        result = engine.run_ac_data(netlist, "PTS")
        assert result.frequencies.tolist() == [1.0, 1.0e3, 1.0e6]

    def test_engine_run_executes_ac_data_directive(self, engine):
        report = engine.run(rspice.Netlist.parse(AC_DATA_DECK))
        assert report.ac is not None
        assert report.ac.frequencies.tolist() == [1.0, 1.0e3, 1.0e6]
        assert any(record.kind == "ac_data" and not record.skipped for record in report.records)

    def test_missing_data_table_is_rejected(self, engine, rc_lowpass):
        with pytest.raises(ValueError, match="unknown.*table"):
            engine.run_ac_data(rc_lowpass, "missing")

    @pytest.mark.parametrize(
        "columns, rows, message",
        [
            ("voltage", "1", "no FREQ or HERTZ"),
            ("FREQ HERTZ", "1 2", "ambiguous frequency"),
            ("FREQ", "-1", "frequency must be positive"),
        ],
    )
    def test_invalid_frequency_table_raises_value_error(
        self, engine, columns, rows, message
    ):
        netlist = rspice.Netlist.parse(f"""* Invalid AC table selected by the API
V1 in 0 AC 1
R1 in 0 1k
.data points {columns}
{rows}
.enddata
.end
""")
        with pytest.raises(ValueError, match=message):
            engine.run_ac_data(netlist, "points")


@pytest.mark.parametrize("route", ["direct", "sweep", "deck"])
def test_veriloga_finish_retains_only_solved_frequency_rows(engine, tmp_path, route):
    model = tmp_path / "finish.va"
    model.write_text("""module finish_model(p,n);
inout p,n; electrical p,n;
real count;
analog begin
    @(initial_step("ac")) count=1;
    @(final_step("ac")) count=count+1;
    if (analysis("ac") && !analysis("static") && count==1) $finish(1);
    I(p,n)<+count*1e-3*V(p,n);
end
endmodule
""", encoding="utf-8")
    netlist = rspice.Netlist.parse(f"""* Retained Verilog-A AC endpoint
V1 in 0 DC 0 AC 1
R1 in out 1k
X1 out 0 finish_model
.va "{model.as_posix()}" finish_model
.ac lin 13 10 1meg
.end
""")
    if route == "direct":
        result = engine.run_ac(netlist, np.linspace(10.0, 1e6, 13).tolist())
    elif route == "sweep":
        result = engine.run_ac_sweep(netlist, "lin", 13, 10.0, 1e6)
    else:
        result = engine.run(netlist).ac
    np.testing.assert_array_equal(result.frequencies, [10.0])
    assert result.num_frequencies == 1
    np.testing.assert_allclose(result.voltage_complex("out"), [1.0 / 3.0], atol=1e-12)
    with pytest.raises(IndexError):
        result.magnitude_at(1, "out")
    # Projection must describe the retained result rather than padding it to
    # the requested thirteen frequencies.
    document = result.document()
    assert document["pointCount"] == 1
    assert document["axes"][0]["values"]["values"] == [10.0]
    restored = pickle.loads(pickle.dumps(result))
    np.testing.assert_array_equal(restored.frequencies, [10.0])
    np.testing.assert_allclose(restored.voltage_complex("out"), [1.0 / 3.0], atol=1e-12)


@pytest.mark.parametrize("at_bias", [False, True])
@pytest.mark.parametrize("route", ["direct", "deck"])
def test_veriloga_data_finish_retains_completed_rows(engine, tmp_path, at_bias, route):
    model = tmp_path / "data_finish.va"
    model.write_text("""module data_finish(p,n);
inout p,n; electrical p,n;
parameter integer stop_now=0, at_bias=0;
analog begin
    if (stop_now < 0) $finish(99);
    if (stop_now && (at_bias || !analysis("static"))) $finish(1);
    I(p,n)<+1e-3*V(p,n);
end
endmodule
""", encoding="utf-8")
    netlist = rspice.Netlist.parse(f"""* Retain solved AC DATA prefix
.param stop_now=0
V1 in 0 DC 0 AC 1
R1 in out 1k
X1 out 0 data_finish stop_now={{stop_now}} at_bias={int(at_bias)}
.va "{model.as_posix()}" data_finish
.ac DATA=points
.data points HERTZ stop_now
10 0
20 1
30 -1
.enddata
.end
""")
    result = (
        engine.run_ac_data(netlist, "points")
        if route == "direct" else engine.run(netlist).ac
    )
    expected = [10.0] if at_bias else [10.0, 20.0]
    np.testing.assert_array_equal(result.frequencies, expected)
    np.testing.assert_allclose(
        result.voltage_complex("out"), np.full(len(expected), 0.5), atol=1e-12
    )
    assert result.document()["pointCount"] == len(expected)
