import numpy as np
import pytest

import rspice


F0 = 1.0e6


def parse(deck: str) -> rspice.Netlist:
    return rspice.Netlist.parse(deck)


def test_pss_exposes_periodic_waveforms_and_diagnostics():
    netlist = parse(
        f"""
V1 in 0 SIN(0 1 {F0})
R1 in out 1k
C1 out 0 159.154943091895p
.end
"""
    )
    result = rspice.Engine().run_pss(
        netlist,
        F0,
        harmonics=5,
        tstab_periods=8,
        tolerance=1e-7,
        abstol=1e-13,
        damping=0.8,
        max_period_change=0.05,
        points_per_period=64,
        integration_method=rspice.IntegrationMethod.TRAPEZOIDAL,
    )

    assert isinstance(result, rspice.PssResult)
    assert result.frequency == pytest.approx(F0, rel=1e-8)
    assert result.period == pytest.approx(1.0 / F0, rel=1e-8)
    assert result.num_points == len(result.time)
    assert result.num_nodes == len(result.node_names)
    assert result.num_harmonics == 5
    assert result.harmonic_frequencies.tolist() == pytest.approx(
        [0.0, F0, 2 * F0, 3 * F0, 4 * F0, 5 * F0]
    )
    waveform = result.voltage_waveform("out")
    assert waveform.shape == result.time.shape
    assert result.peak_to_peak("out") == pytest.approx(np.ptp(waveform), rel=1e-12)
    assert result.voltage_waveform("0").tolist() == [0.0] * result.num_points
    coefficients = result.harmonic_coefficients("out")
    assert coefficients.dtype == np.complex128
    assert abs(coefficients[1]) == pytest.approx(1 / np.sqrt(2), rel=0.03)
    assert result.harmonic_magnitude("out")[1] == pytest.approx(
        abs(coefficients[1]), rel=1e-12
    )
    # SIN starts at its zero crossing (-90-degree cosine-reference phasor),
    # and the one-pole network contributes another -45 degrees.
    assert result.harmonic_phase_degrees("out")[1] == pytest.approx(-135.0, abs=2.0)
    assert result.harmonics("out")[0].n == 1
    assert result.thd_percent("out") < 0.1
    assert np.count_nonzero(result.harmonic_coefficients("0")) == 0
    with pytest.raises(KeyError):
        result.voltage_waveform("missing")
    with pytest.raises(KeyError):
        result.harmonic_coefficients("missing")


def test_hb_exposes_complex_spectra_without_silent_node_fallback():
    netlist = parse(
        """
V1 in 0 AC 0.01
R1 in out 1k
R2 out 0 1k
.end
"""
    )
    result = rspice.Engine().run_hb(netlist, F0, harmonics=4)

    assert isinstance(result, rspice.HbResult)
    assert result.converged
    assert result.is_valid
    assert result.harmonic_frequencies.tolist() == pytest.approx(
        [0.0, F0, 2 * F0, 3 * F0, 4 * F0]
    )
    coefficients = result.coefficients("out")
    assert coefficients.dtype == np.complex128
    assert coefficients.shape == (5,)
    assert abs(coefficients[1]) == pytest.approx(0.005, rel=2e-3)
    with pytest.raises(KeyError):
        result.coefficients("missing")


def test_engine_run_executes_hb_card_and_hbint_order():
    netlist = parse(
        """
V1 in 0 AC 0.01
R1 in out 1k
R2 out 0 1k
.hb 1meg
.options hbint numfreq=4
.end
"""
    )
    report = rspice.Engine().run(netlist)
    assert isinstance(report.hb, rspice.HbResult)
    assert report.hb.converged
    assert report.hb.num_harmonics == 4
    assert report.hb.harmonic_frequencies.tolist() == pytest.approx(
        [0.0, F0, 2 * F0, 3 * F0, 4 * F0]
    )
    assert abs(report.hb.coefficients("out")[1]) == pytest.approx(0.005, rel=2e-3)
    assert [record.kind for record in report.records] == ["hb"]
    assert report.skipped == []


def test_multitone_hb_maps_each_tone_to_its_source():
    netlist = parse(
        """
V1 n1 0 AC 1
R1 n1 0 1k
V2 n2 0 AC 2
R2 n2 0 1k
.end
"""
    )
    result = rspice.Engine().run_hb_multitone(
        netlist,
        [F0, 2 * F0],
        harmonics=[2],
        source_names=["V1", "V2"],
    )
    assert result.fundamental_frequency == pytest.approx(F0)
    assert result.num_harmonics == 4
    assert abs(result.coefficients("n1")[1]) == pytest.approx(1.0, rel=2e-3)
    assert abs(result.coefficients("n1")[2]) < 1e-12
    assert abs(result.coefficients("n2")[1]) < 1e-12
    assert abs(result.coefficients("n2")[2]) == pytest.approx(2.0, rel=2e-3)


@pytest.mark.parametrize(
    "kwargs, message",
    [
        ({"frequencies": [F0, F0]}, "more than once"),
        (
            {"frequencies": [F0, 2 * F0], "harmonics": [2, 3, 4]},
            "harmonic orders",
        ),
        (
            {"frequencies": [F0, 2 * F0], "source_names": ["V1"]},
            "source names",
        ),
        ({"frequencies": [F0], "collocation_points": 8}, "must be odd"),
    ],
)
def test_multitone_hb_rejects_ambiguous_or_invalid_configuration(kwargs, message):
    netlist = parse("V1 n1 0 AC 1\nR1 n1 0 1k\n.end")
    with pytest.raises(ValueError, match=message):
        rspice.Engine().run_hb_multitone(netlist, **kwargs)


def test_pac_exposes_signed_sideband_conversion():
    netlist = parse(
        """
VIN in 0 DC 0 AC 1
R1 in out 1k
C1 out 0 159.154943091895p
.end
"""
    )
    result = rspice.Engine().run_pac(
        netlist,
        F0,
        1.0e5,
        5.0e5,
        3,
        "VIN",
        "out",
        variation="lin",
        sideband_min=-2,
        sideband_max=2,
        reltol=1e-7,
        abstol=1e-13,
    )

    assert isinstance(result, rspice.PacResult)
    assert result.converged
    assert result.frequencies.tolist() == pytest.approx([1.0e5, 3.0e5, 5.0e5])
    direct = result.conversion_gain(0, 0)
    assert direct.dtype == np.complex128
    assert abs(direct[0]) == pytest.approx(1 / np.sqrt(1.01), rel=2e-3)
    assert np.max(np.abs(result.conversion_gain(0, 1))) < 1e-9
    with pytest.raises(IndexError):
        result.voltage("out", 3)
    with pytest.raises(KeyError):
        result.voltage("missing", 0)


def test_pac_exposes_exact_mna_branch_currents():
    netlist = parse(
        """
VIN out 0 DC 0 AC 9 27
R1 out 0 2k
.end
"""
    )
    result = rspice.Engine().run_pac(
        netlist,
        F0,
        1.0e5,
        2.0e5,
        2,
        "VIN",
        "out",
        variation="lin",
        sideband_min=-1,
        sideband_max=1,
    )

    assert result.branch_names == ["VIN"]
    direct = result.branch_current("vin", 0)
    assert direct.dtype == np.complex128
    np.testing.assert_allclose(direct, -5.0e-4, rtol=1e-12, atol=1e-15)
    np.testing.assert_array_equal(
        result.branch_current("VIN", -1), np.zeros(2, dtype=np.complex128)
    )
    with pytest.raises(IndexError):
        result.branch_current("VIN", 2)
    with pytest.raises(KeyError):
        result.branch_current("missing", 0)


def test_pnoise_reports_psd_density_and_contributor_identity():
    netlist = parse(
        """
V1 in 0 DC 0
R1 in out 1k
R2 out 0 1k
C1 out 0 1n
.end
"""
    )
    offsets = [1.0e3, 1.0e5]
    result = rspice.Engine().run_pnoise(netlist, F0, offsets, "out", max_sideband=2)

    assert isinstance(result, rspice.PeriodicNoiseResult)
    assert result.converged
    assert result.frequencies.tolist() == offsets
    assert np.all(result.output_noise > 0.0)
    assert np.allclose(result.output_noise_density**2, result.output_noise)
    assert result.contributors
    summed = np.sum(
        [contribution.power_spectral_density for contribution in result.contributors], axis=0
    )
    assert np.allclose(summed, result.output_noise, rtol=1e-10, atol=0.0)
    first = result.contributors[0]
    assert result.contribution(first.name).name == first.name
    with pytest.raises(KeyError):
        result.contribution("missing")


def test_autonomous_oscillator_noise_has_analytic_white_noise_slope():
    netlist = parse(
        """
L1 osc 0 1u
C1 osc 0 1u
R1 osc 0 1k
B1 osc 0 I=-0.051*v(osc)+0.025*v(osc)*v(osc)*v(osc)
I1 0 osc PULSE(0 1 10u 10n 10n 1u 1)
.end
"""
    )
    result = rspice.Engine().run_oscillator_noise(
        netlist,
        [1.0e3, 1.0e4, 1.0e5],
        period_guess=6.3e-6,
        tstab_periods=30,
        max_iterations=60,
        tolerance=1e-6,
        abstol=1e-12,
        damping=0.8,
        max_period_change=0.05,
        integration_method=rspice.IntegrationMethod.TRAPEZOIDAL,
    )

    assert isinstance(result, rspice.OscillatorNoiseResult)
    assert result.carrier_frequency == pytest.approx(159.155e3, rel=5e-3)
    assert np.diff(result.phase_noise_dbc).tolist() == pytest.approx(
        [-20.0, -20.0], abs=0.5
    )
    assert result.diffusion_constant > 0.0
    assert result.corner_frequency > 0.0


@pytest.mark.parametrize(
    "call",
    [
        lambda engine, netlist: engine.run_pss(netlist),
        lambda engine, netlist: engine.run_hb(netlist, 0.0),
        lambda engine, netlist: engine.run_pac(
            netlist, F0, 1.0, 10.0, 0, "V1", "out"
        ),
        lambda engine, netlist: engine.run_pnoise(netlist, F0, [0.0], "out"),
        lambda engine, netlist: engine.run_pss(netlist, F0, abstol=0.0),
        lambda engine, netlist: engine.run_pss(netlist, F0, damping=0.05),
        lambda engine, netlist: engine.run_pss(netlist, F0, max_period_change=0.0),
        lambda engine, netlist: engine.run_pac(
            netlist, F0, 1.0, 10.0, 2, "V1", "out", reltol=0.0
        ),
        lambda engine, netlist: engine.run_oscillator_noise(
            netlist, [1.0], period_guess=1.0 / F0, harmonics=0
        ),
        lambda engine, netlist: engine.run_oscillator_noise(
            netlist, [0.0], period_guess=1.0 / F0
        ),
    ],
)
def test_periodic_analysis_arguments_are_validated(call):
    netlist = parse("V1 out 0 1\nR1 out 0 1k\n.end")
    with pytest.raises(ValueError):
        call(rspice.Engine(), netlist)


def test_periodic_result_types_have_public_module_identity():
    for cls in (
        rspice.PssResult,
        rspice.HbResult,
        rspice.PacResult,
        rspice.PeriodicNoiseContribution,
        rspice.PeriodicNoiseResult,
        rspice.OscillatorNoiseResult,
    ):
        assert cls.__module__ == "rspice"


PERIODIC_CARD_DECKS = [
    (".PSS FUND=1G", ".PSS", "pss-001"),
    (".HB 1G\n.PAC DEC 5 1k 1meg INPUT=V1 OUT=V(out)", ".PAC", "pac-001"),
    (".HB 1G\n.PNOISE DEC 5 1 1k OUT=V(out)", ".PNOISE", "pnoise-001"),
    (".HB 1G\n.ENVELOPE TSTOP=1u", ".ENVELOPE", "env-001"),
]


def periodic_card_deck(cards: str) -> rspice.Netlist:
    return parse(
        f"""
V1 in 0 SIN(0 1 {F0})
R1 in out 1k
C1 out 0 1p
{cards}
.end
"""
    )


@pytest.mark.parametrize(("cards", "card", "analysis_id"), PERIODIC_CARD_DECKS)
def test_authored_periodic_cards_are_refused_by_engine_run(cards, card, analysis_id):
    """Engine.run has no authored route for the periodic family yet."""
    netlist = periodic_card_deck(cards)
    with pytest.raises(NotImplementedError) as excinfo:
        rspice.Engine().run(netlist)
    assert isinstance(excinfo.value, rspice.RSpiceError)
    assert card in str(excinfo.value)
    assert analysis_id in str(excinfo.value)


def test_a_refused_periodic_deck_publishes_no_result():
    """The refusal precedes every directive, continue_on_error included."""
    netlist = periodic_card_deck(".OP\n.PSS FUND=1G")
    with pytest.raises(rspice.RSpiceNotImplementedError):
        rspice.Engine().run(netlist, continue_on_error=True)


def test_a_malformed_periodic_card_is_a_typed_parse_error():
    with pytest.raises(rspice.ParseError) as excinfo:
        parse("bad pss\nV1 out 0 1\nR1 out 0 1k\n.PSS FUND=1G HARMS=0\n.end\n")
    error = excinfo.value
    assert error.kind == "analysis_card"
    assert error.category == "analysis_card_validation"
    assert error.output_directive == ".PSS"
    assert error.reason == "invalid_number"
    assert error.parameter_name == "HARMS"
    assert error.line == 4
