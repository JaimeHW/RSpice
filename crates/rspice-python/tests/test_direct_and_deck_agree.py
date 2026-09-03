"""Every convenience method means the same thing as the deck card it spells.

`Engine.run_tran(...)` and `Engine.run(deck with .TRAN ...)` are two ways of
asking for one analysis. They are implemented as one: a convenience method
translates its arguments into the authored card a deck would carry and hands
that card to the same executor, planned by the same `DeckPlan`. These tests
hold that equality to numbers rather than to intent, so a future edit that
teaches one surface something the other does not know fails here.

The comparison is bit-exact. Both surfaces run the same solver on the same
circuit with the same configuration, so a difference of one ulp would mean they
are not, in fact, the same request. That makes the deck literals load-bearing:
`.TRAN 20u 1m` and `run_tran(stop_time=1e-3, max_step=20e-6)` differ in the
last bit of TMAX because `20 * 1e-6` and `2e-5` are different doubles. The
cards below therefore spell their values exactly as Python does, so any
remaining difference is a difference in meaning rather than in parsing.
"""

import numpy as np
import pytest

import rspice


RESISTIVE = """* Direct/deck equality: linear divider
V1 in 0 10 AC 1
R1 in out 1k
R2 out 0 1k
C1 out 0 1u
.end
"""

NONLINEAR = """* Direct/deck equality: diode
V1 in 0 SIN(0 1 1k) AC 1 DISTOF1 1
R1 in out 1k
D1 out 0 DMOD
.model DMOD D(IS=1e-14 N=1.0)
.end
"""

PARAMETRIC = """* Direct/deck equality: parametric divider
.param rval=1k
V1 in 0 10
R1 in out {rval}
R2 out 0 1k
.end
"""

RF = """* Direct/deck equality: RC at 1 MHz
V1 in 0 SIN(0 1 1e6) AC 1
R1 in out 1k
C1 out 0 1p
.end
"""

TWO_PORT = """* Direct/deck equality: 50 ohm series resistor
V1 p1 0 AC 1 portnum 1 z0 50
V2 p2 0 AC 0 portnum 2 z0 50
R1 p1 p2 50
.end
"""

STB_LOOP = """* Direct/deck equality: Tian probe in a feedback loop
V1 in 0 AC 1
R1 in a 1k
E1 b 0 a 0 -10
VPROBE b c 0
R2 c a 10k
R3 c 0 10k
.end
"""


def engine() -> rspice.Engine:
    return rspice.Engine()


def deck(circuit: str, card: str) -> rspice.Netlist:
    """Insert one analysis card into a circuit just before `.end`."""
    body, _, _ = circuit.rpartition(".end")
    return rspice.Netlist.parse(f"{body}{card}\n.end\n")


def assert_identical(direct, from_deck, what: str) -> None:
    direct = np.asarray(direct)
    from_deck = np.asarray(from_deck)
    assert direct.shape == from_deck.shape, f"{what}: shape"
    assert np.array_equal(direct, from_deck, equal_nan=True), what


def test_op_matches_its_card():
    report = engine().run(deck(RESISTIVE, ".OP"))
    direct = engine().run_dc_op(rspice.Netlist.parse(RESISTIVE))

    assert_identical(direct.node_voltages, report.op.node_voltages, "node voltages")
    assert_identical(direct.branch_currents, report.op.branch_currents, "branch currents")
    assert direct.node_names == report.op.node_names
    assert [d.name for d in direct.device_operating_points] == [
        d.name for d in report.op.device_operating_points
    ]


def test_dc_sweep_matches_its_card():
    report = engine().run(deck(RESISTIVE, ".DC V1 0 5 0.5"))
    direct = engine().run_dc_sweep(rspice.Netlist.parse(RESISTIVE), "V1", 0.0, 5.0, 0.5)

    assert len(direct) == len(report.dc)
    assert_identical(direct.sweep_values, report.dc.sweep_values, "sweep values")
    assert_identical(
        direct.voltage_array("out"), report.dc.voltage_array("out"), "V(out)"
    )


def test_nested_dc_sweep_spec_matches_its_card():
    circuit = """* Direct/deck equality: two swept sources
V1 in 0 1
V2 bias 0 1
R1 in out 1k
R2 out bias 1k
.end
"""
    report = engine().run(deck(circuit, ".DC V1 0 2 0.5 V2 0 1 0.5"))
    inner = rspice.DcSweep("V1", start=0.0, stop=2.0, step=0.5)
    outer = rspice.DcSweep("V2", start=0.0, stop=1.0, step=0.5)
    direct = engine().run_dc_sweep_spec(
        rspice.Netlist.parse(circuit), inner, sweep2=outer
    )

    assert direct.shape == report.dc.shape
    assert_identical(
        direct.voltage_array("out"), report.dc.voltage_array("out"), "V(out)"
    )


def test_ac_sweep_matches_its_card():
    report = engine().run(deck(RESISTIVE, ".AC DEC 10 1 100000"))
    direct = engine().run_ac_sweep(
        rspice.Netlist.parse(RESISTIVE), "dec", 10, 1.0, 100000.0
    )

    assert_identical(direct.frequencies, report.ac.frequencies, "frequencies")
    assert_identical(
        direct.voltage_complex("out"), report.ac.voltage_complex("out"), "V(out)"
    )


def test_ac_explicit_list_matches_the_same_grid_from_a_card():
    """The explicit-list form has no card spelling, so it is pinned to the grid."""
    report = engine().run(deck(RESISTIVE, ".AC DEC 10 1 100000"))
    grid = list(report.ac.frequencies)
    direct = engine().run_ac(rspice.Netlist.parse(RESISTIVE), grid)

    assert_identical(
        direct.voltage_complex("out"), report.ac.voltage_complex("out"), "V(out)"
    )


def test_tran_matches_its_card():
    report = engine().run(deck(RESISTIVE, ".TRAN 2e-05 1e-03"))
    direct = engine().run_tran(rspice.Netlist.parse(RESISTIVE), 1e-3, 2e-5)

    assert_identical(direct.time, report.tran.time, "time")
    assert_identical(
        direct.voltage_waveform("out"), report.tran.voltage_waveform("out"), "V(out)"
    )


def test_tran_default_ceiling_matches_the_card_that_states_it():
    """`run_tran` documents TMAX = window/50; the card spells that as TSTEP."""
    report = engine().run(deck(RESISTIVE, ".TRAN 2e-05 1e-03"))
    direct = engine().run_tran(rspice.Netlist.parse(RESISTIVE), 1e-3)

    assert_identical(direct.time, report.tran.time, "time")


def test_disto_sweep_matches_its_card():
    report = engine().run(deck(NONLINEAR, ".DISTO DEC 5 1000 10000"))
    direct = engine().run_distortion_sweep(
        rspice.Netlist.parse(NONLINEAR), "dec", 5, 1000.0, 10000.0
    )

    assert_identical(
        direct.f1_frequencies, report.distortion.f1_frequencies, "F1 grid"
    )
    assert_identical(
        direct.product("2f1").voltage_complex("out"),
        report.distortion.product("2f1").voltage_complex("out"),
        "HD2 V(out)",
    )


def test_tf_matches_its_card():
    report = engine().run(deck(RESISTIVE, ".TF V(out) V1"))
    direct = engine().run_transfer_function(
        rspice.Netlist.parse(RESISTIVE), "out", "V1"
    )

    assert direct.gain == report.tf.gain
    assert direct.input_impedance == report.tf.input_impedance
    assert direct.output_impedance == report.tf.output_impedance


def test_stb_matches_its_card():
    report = engine().run(deck(STB_LOOP, ".STB DEC 5 1 1000000 VPROBE"))
    direct = engine().run_stb(
        rspice.Netlist.parse(STB_LOOP), "VPROBE", "dec", 5, 1.0, 1000000.0
    )

    assert_identical(direct.frequencies, report.stb.frequencies, "frequencies")
    assert_identical(direct.loop_gain, report.stb.loop_gain, "loop gain")
    assert direct.phase_margin_degrees == report.stb.phase_margin_degrees


def test_pz_matches_its_card():
    report = engine().run(deck(RESISTIVE, ".PZ in 0 out 0 vol pz"))
    direct = engine().run_pz(
        rspice.Netlist.parse(RESISTIVE), "in", "out", input_type="voltage"
    )

    assert_identical(direct.poles_array, report.pz.poles_array, "poles")
    assert_identical(direct.zeros_array, report.pz.zeros_array, "zeros")
    assert direct.dc_gain == report.pz.dc_gain


def test_pz_omitted_reference_terminals_mean_ground_on_both_surfaces():
    """`run_pz` without references is the card that names node 0 explicitly."""
    grounded = engine().run_pz(
        rspice.Netlist.parse(RESISTIVE),
        "in",
        "out",
        input_negative="0",
        output_negative="0",
        input_type="voltage",
    )
    omitted = engine().run_pz(
        rspice.Netlist.parse(RESISTIVE), "in", "out", input_type="voltage"
    )

    assert_identical(omitted.poles_array, grounded.poles_array, "poles")
    assert omitted.dc_gain == grounded.dc_gain


def test_sens_matches_its_card():
    report = engine().run(deck(PARAMETRIC, ".SENS V(out)"))
    direct = engine().run_sensitivity_dc_complete(
        rspice.Netlist.parse(PARAMETRIC), "out"
    )

    assert direct.output == report.sensitivity.output
    assert direct.output_value == report.sensitivity.output_value
    assert [(s.element, s.parameter, s.absolute) for s in direct.sensitivities] == [
        (s.element, s.parameter, s.absolute) for s in report.sensitivity.sensitivities
    ]


def test_sens_ac_sweep_matches_its_card():
    report = engine().run(deck(PARAMETRIC, ".SENS V(out) AC DEC 5 1 10000"))
    direct = engine().run_sensitivity_ac_sweep(
        rspice.Netlist.parse(PARAMETRIC), "out", "dec", 5, 1.0, 10000.0
    )

    assert_identical(
        direct.frequencies, report.sensitivity_ac.frequencies, "frequencies"
    )
    assert [(s.element, s.parameter) for s in direct.sensitivities] == [
        (s.element, s.parameter) for s in report.sensitivity_ac.sensitivities
    ]


def test_monte_carlo_matches_its_card():
    report = engine().run(deck(PARAMETRIC, ".mc 8 seed 1234 dist gauss spread 0.02"))
    direct = engine().run_monte_carlo(
        rspice.Netlist.parse(PARAMETRIC),
        8,
        seed=1234,
        distribution="gaussian",
        spread=0.02,
    )

    assert direct.num_runs == report.monte_carlo.num_runs
    assert direct.variable_names == report.monte_carlo.variable_names
    for name in direct.variable_names:
        assert direct.mean(name) == report.monte_carlo.mean(name)
        assert direct.std_dev(name) == report.monte_carlo.std_dev(name)


def test_noise_matches_the_same_grid_from_a_card():
    report = engine().run(deck(RESISTIVE, ".NOISE V(out) V1 DEC 5 1 10000"))
    grid = [r.frequency for r in report.noise]
    direct = engine().run_noise(
        rspice.Netlist.parse(RESISTIVE), "out", grid, input_source="V1"
    )

    assert [r.frequency for r in direct] == [r.frequency for r in report.noise]
    assert [r.output_noise_rms for r in direct] == [
        r.output_noise_rms for r in report.noise
    ]


def test_s_parameters_match_the_same_grid_from_a_card():
    report = engine().run(deck(TWO_PORT, ".SP DEC 5 1000000 100000000"))
    grid = list(report.s_parameters.frequencies)
    direct = engine().run_s_parameters(rspice.Netlist.parse(TWO_PORT), grid)

    assert_identical(direct.s(2, 1), report.s_parameters.s(2, 1), "S21")


def test_hb_matches_the_card_that_states_the_same_tone():
    report = engine().run(deck(RF, ".HB 1e6"))
    direct = engine().run_hb(rspice.Netlist.parse(RF), 1e6)

    assert direct.fundamental_frequency == report.hb.fundamental_frequency
    assert direct.num_harmonics == report.hb.num_harmonics
    assert_identical(
        direct.coefficients("out"), report.hb.coefficients("out"), "carrier spectrum"
    )


def test_pss_matches_its_card():
    report = engine().run(deck(RF, ".PSS FUND=1e6 HARMS=5 POINTS=64"))
    direct = engine().run_pss(
        rspice.Netlist.parse(RF), 1e6, harmonics=5, points_per_period=64
    )

    assert direct.frequency == report.pss.frequency
    assert direct.iterations == report.pss.iterations
    assert_identical(direct.time, report.pss.time, "orbit time")
    assert_identical(
        direct.voltage_waveform("out"), report.pss.voltage_waveform("out"), "V(out)"
    )


def test_pac_matches_its_card_when_both_reuse_the_same_pss_orbit():
    card = (
        ".PSS FUND=1e6 HARMS=5 POINTS=64\n"
        ".PAC DEC 3 1000 10000 INPUT=V1 OUT=V(out) MAXSIDEBAND=2 FROM=PSS"
    )
    report = engine().run(deck(RF, card))

    netlist = rspice.Netlist.parse(RF)
    operating_point = engine().run_pss_operating_point(
        netlist, 1e6, harmonics=5, points_per_period=64
    )
    direct = engine().run_pac(
        netlist,
        1e6,
        1000.0,
        10000.0,
        3,
        "V1",
        "out",
        sideband_min=-2,
        sideband_max=2,
        pss=operating_point,
    )

    assert_identical(direct.frequencies, report.pac.frequencies, "frequencies")
    assert direct.sideband_min == report.pac.sideband_min
    assert direct.sideband_max == report.pac.sideband_max
    assert_identical(
        direct.voltage("out", 0), report.pac.voltage("out", 0), "V(out) sideband 0"
    )


def test_pnoise_matches_its_card_when_both_reuse_the_same_pss_orbit():
    card = (
        ".PSS FUND=1e6 HARMS=5 POINTS=64\n"
        ".PNOISE DEC 3 1 1000 OUT=V(out) MAXSIDEBAND=2 FROM=PSS"
    )
    report = engine().run(deck(RF, card))

    netlist = rspice.Netlist.parse(RF)
    operating_point = engine().run_pss_operating_point(
        netlist, 1e6, harmonics=5, points_per_period=64
    )
    offsets = list(report.pnoise.frequencies)
    direct = engine().run_pnoise(
        netlist, 1e6, offsets, "out", max_sideband=2, pss=operating_point
    )

    assert_identical(direct.frequencies, report.pnoise.frequencies, "offsets")
    assert_identical(
        direct.output_noise, report.pnoise.output_noise, "output noise"
    )


def test_envelope_matches_its_card():
    card = ".HB 1e6\n.ENVELOPE TSTOP=2e-06 MAXSTEP=1e-07"
    report = engine().run(deck(RF, card))

    netlist = rspice.Netlist.parse(RF)
    _, state = engine().run_hb_envelope(netlist, 1e6)
    direct, _ = engine().run_tran_from_hb_envelope(netlist, state, 2e-6, 1e-7)

    assert_identical(
        direct.time, report.envelope.continued_transient.time, "envelope slow time"
    )
    assert_identical(
        direct.voltage_waveform("out"),
        report.envelope.continued_transient.voltage_waveform("out"),
        "envelope V(out)",
    )


@pytest.mark.parametrize(
    ("call", "card"),
    [
        (
            lambda e, n: e.run_dc_sweep(n, "VNOPE", 0.0, 1.0, 0.1),
            ".DC VNOPE 0 1 0.1",
        ),
        (
            lambda e, n: e.run_transfer_function(n, "out", "VNOPE"),
            ".TF V(out) VNOPE",
        ),
        (
            lambda e, n: e.run_stb(n, "VNOPE", "dec", 5, 1.0, 1000.0),
            ".STB DEC 5 1 1000 VNOPE",
        ),
    ],
)
def test_a_bad_argument_fails_the_same_way_on_both_surfaces(call, card):
    """A request neither surface can serve fails identically on both."""
    netlist = rspice.Netlist.parse(RESISTIVE)
    with pytest.raises(rspice.RSpiceError) as direct_error:
        call(engine(), netlist)

    with pytest.raises(rspice.RSpiceError) as deck_error:
        engine().run(deck(RESISTIVE, card), continue_on_error=False)

    assert type(direct_error.value) is type(deck_error.value)
    assert str(direct_error.value) == str(deck_error.value)
