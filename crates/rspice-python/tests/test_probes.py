"""SPICE output-specification handling.

`.FOUR` and the transient accessors share one probe grammar, so a branch
current or a differential node pair resolves the same way everywhere.
"""

import pickle

import numpy as np
import pytest

import rspice

# 1 V peak at 1 kHz across two 500 ohm resistors: I(V1) is 1 mA peak and
# V(in,mid) is 0.5 V peak, so every probe form has an exact expected value.
SINE = """* Probe forms
V1 in 0 SIN(0 1 1k)
R1 in mid 500
R2 mid 0 500
.end
"""


@pytest.fixture()
def tran(engine):
    return engine.run_tran(rspice.Netlist.parse(SINE), stop_time=20e-3, max_step=2e-6)


class TestSignalAccessor:
    def test_bare_name_and_voltage_probe_agree(self, tran):
        expected = tran.voltage_waveform("in")
        np.testing.assert_allclose(tran.signal("in"), expected)
        np.testing.assert_allclose(tran.signal("V(in)"), expected)
        np.testing.assert_allclose(tran.signal("v(IN)"), expected)

    def test_current_probe_matches_the_branch_accessor(self, tran):
        np.testing.assert_allclose(
            tran.signal("I(V1)"), tran.branch_current_waveform("V1")
        )

    def test_differential_probe_subtracts_the_reference(self, tran):
        np.testing.assert_allclose(
            tran.signal("V(in,mid)"),
            tran.voltage_waveform("in") - tran.voltage_waveform("mid"),
        )

    @pytest.mark.parametrize(
        "spec", ["", "   ", "V(a,b,c)", "I(V1,V2)", "V()", "V(in"]
    )
    def test_malformed_specifications_raise_value_error(self, tran, spec):
        with pytest.raises(ValueError):
            tran.signal(spec)

    # `P()` is a real device-power accessor in both Xyce and ngspice, so a
    # well-formed `P(R1)` this result does not carry is an unavailable signal
    # rather than a syntax error.
    @pytest.mark.parametrize(
        "spec", ["V(nosuch)", "I(nosuch)", "V(in,nosuch)", "P(R1)", "@nosuch[id]"]
    )
    def test_unknown_quantities_raise_key_error_naming_the_spec(self, tran, spec):
        pattern = (
            spec.replace("(", r"\(")
            .replace(")", r"\)")
            .replace("[", r"\[")
            .replace("]", r"\]")
        )
        with pytest.raises(KeyError, match=pattern):
            tran.signal(spec)


class TestFourier:
    def test_node_voltage_fundamental(self, tran):
        assert tran.fourier("in", 1e3).fundamental_magnitude == pytest.approx(
            1.0, rel=1e-2
        )

    def test_branch_current_fundamental(self, tran):
        harmonic = tran.fourier_current("V1", 1e3).fundamental_magnitude
        assert harmonic == pytest.approx(1e-3, rel=1e-2)

    def test_differential_fundamental(self, tran):
        harmonic = tran.fourier("in", 1e3, reference="mid").fundamental_magnitude
        assert harmonic == pytest.approx(0.5, rel=1e-2)

    def test_fourier_current_rejects_an_unknown_element(self, tran):
        with pytest.raises(KeyError):
            tran.fourier_current("nosuch", 1e3)

    @pytest.mark.parametrize("fundamental", [0.0, -1.0, float("inf"), float("nan")])
    def test_fourier_rejects_a_non_positive_fundamental(self, tran, fundamental):
        with pytest.raises(ValueError):
            tran.fourier("in", fundamental)
        with pytest.raises(ValueError):
            tran.fourier_current("V1", fundamental)

    def test_fourier_rejects_zero_harmonics(self, tran):
        with pytest.raises(ValueError):
            tran.fourier("in", 1e3, 0)

    def test_ground_has_a_zero_spectrum_with_undefined_thd(self, tran):
        result = tran.fourier("0", 1e3)

        assert result.thd is None
        assert result.thd_percent is None
        assert all(harmonic.magnitude == 0.0 for harmonic in result.harmonics)


# A diode's operating-point trace is retained per time point when the deck
# asks for it, so `@D1[Id]` is a real waveform the core resolver reaches
# through the same `@device[param]` grammar `.SAVE` and `.PRINT` use.
DIODE = """* Device-observable probes
V1 in 0 SIN(0 1 1k)
R1 in mid 500
D1 mid 0 DMODEL
.MODEL DMODEL D(IS=1e-12 N=1)
.save @D1[Id]
.end
"""


def signal_names(signals):
    return [signal.name.upper() for signal in signals]


class TestDeviceObservableProbe:
    """`@device[param]` is one grammar shared with `.SAVE` and `.PRINT`."""

    @pytest.fixture()
    def diode_tran(self, engine):
        return engine.run_tran(
            rspice.Netlist.parse(DIODE), stop_time=2e-3, max_step=2e-6
        )

    def test_device_parameter_probe_is_a_waveform(self, diode_tran):
        current = diode_tran.signal("@D1[Id]")
        assert len(current) == len(diode_tran.time)
        assert np.all(np.isfinite(current))
        assert np.max(current) > 0.0

    def test_device_parameter_probe_is_case_insensitive(self, diode_tran):
        np.testing.assert_allclose(
            diode_tran.signal("@d1[id]"), diode_tran.signal("@D1[Id]")
        )

    def test_fourier_of_a_device_parameter_matches_the_probe(self, diode_tran):
        spectrum = diode_tran.fourier_of("@D1[Id]", 1e3)
        assert spectrum.fundamental_magnitude > 0.0

    def test_fourier_of_a_differential_pair_matches_the_direct_call(self, tran):
        spectrum = tran.fourier_of("V(in,mid)", 1e3)
        assert spectrum.fundamental_magnitude == pytest.approx(
            tran.fourier("in", 1e3, reference="mid").fundamental_magnitude
        )


class TestSavedSignals:
    """`.SAVE` projection is available to Python, not only to the CLI."""

    DECK = DIODE.replace(".end", ".tran 2u 2m\n.print tran V(mid)\n.end")

    def test_a_saved_device_observable_survives_beside_a_print_card(self, engine):
        netlist = rspice.Netlist.parse(self.DECK)
        report = engine.run(netlist)
        signals = report.tran.saved_signals(netlist)

        assert signal_names(signals) == ["V(MID)", "@D1[ID]"]
        assert [signal.kind for signal in signals] == ["voltage", "parameter"]
        for signal in signals:
            assert len(signal.values) == len(report.tran.time)
            assert all(signal.validity)

    def test_a_deck_without_output_directives_keeps_everything(self, tran):
        names = signal_names(tran.saved_signals(rspice.Netlist.parse(SINE)))
        assert "V(IN)" in names
        assert "V(MID)" in names
        assert "I(V1)" in names

    def test_an_unavailable_saved_symbol_raises_the_typed_error(self, tran):
        deck = SINE.replace(".end", ".tran 2u 20m\n.save @R1[NotAParameter]\n.end")
        netlist = rspice.Netlist.parse(deck)
        with pytest.raises(rspice.SimulationError) as failure:
            tran.saved_signals(netlist)
        assert "@R1[NotAParameter]" in str(failure.value)

    def test_an_operating_point_projects_its_saved_signals(self, engine):
        deck = """* saved operating point
V1 in 0 5
R1 in out 1k
R2 out 0 1k
.op
.save V(out)
.end
"""
        netlist = rspice.Netlist.parse(deck)
        op = engine.run_dc_op(netlist)
        signals = op.saved_signals(netlist)
        assert signal_names(signals) == ["V(OUT)"]
        assert signals[0].values[0] == pytest.approx(2.5)


class TestFourDirective:
    """`.FOUR` in a deck reaches the same quantities as the direct calls."""

    DECK = SINE.replace(
        ".end", ".tran 2u 20m\n.four 1k V(in) I(V1) V(in,mid)\n.end"
    )

    def test_every_probe_form_is_evaluated(self, engine):
        report = engine.run(rspice.Netlist.parse(self.DECK))

        assert report.skipped == []
        assert len(report.fourier) == 3
        assert report.fourier[0].fundamental_magnitude == pytest.approx(1.0, rel=1e-2)
        assert report.fourier[1].fundamental_magnitude == pytest.approx(1e-3, rel=1e-2)
        assert report.fourier[2].fundamental_magnitude == pytest.approx(0.5, rel=1e-2)

    def test_deck_results_match_the_direct_calls(self, engine, tran):
        report = engine.run(rspice.Netlist.parse(self.DECK))
        assert report.fourier[1].fundamental_magnitude == pytest.approx(
            tran.fourier_current("V1", 1e3).fundamental_magnitude
        )

    def test_results_identify_the_authored_request_and_parent_transient(self, engine):
        deck = SINE.replace(
            ".end",
            ".tran 4u 20m\n.four 1k V(in)\n.tran 2u 20m\n.end",
        )
        report = engine.run(rspice.Netlist.parse(deck))

        assert len(report.fourier) == 1
        result = report.fourier[0]
        assert result.source_signal == "V(IN)"
        assert result.analysis_id == "four-001"
        assert result.parent_analysis_id == "tran-002"
        assert result.coordinate is None

        restored = pickle.loads(pickle.dumps(result))
        assert restored.source_signal == "V(IN)"
        assert restored.analysis_id == "four-001"
        assert restored.parent_analysis_id == "tran-002"

        record = next(record for record in report.records if record.kind == "four")
        assert record.analysis_id == "four-001"
        assert record.parent_analysis_id == "tran-002"
        assert record.coordinate is None

    def test_each_coordinate_retains_parent_provenance(self, engine):
        deck = """* Stepped Fourier provenance
.param amplitude=1
V1 in 0 SIN(0 {amplitude} 1k)
R1 in 0 1k
.step param amplitude list 1 2
.four 1k V(in)
.tran 2u 20m
.end
"""
        report = engine.run(rspice.Netlist.parse(deck))

        assert len(report.fourier) == 2
        assert [result.analysis_id for result in report.fourier] == [
            "four-001",
            "four-001",
        ]
        assert [result.parent_analysis_id for result in report.fourier] == [
            "tran-001",
            "tran-001",
        ]
        assert [
            result.coordinate.assignments[0].value for result in report.fourier
        ] == [1.0, 2.0]
        assert [result.fundamental_magnitude for result in report.fourier] == pytest.approx(
            [1.0, 2.0], rel=1e-2
        )

    def test_an_undefined_output_is_rejected_at_parse_time(self):
        # Output-symbol validation runs during parsing, so an unresolvable
        # `.four` output fails before any analysis is executed rather than
        # becoming a skipped record at run time.
        deck = SINE.replace(".end", ".tran 2u 2m\n.four 1k V(nosuch)\n.end")
        with pytest.raises(rspice.ParseError) as failure:
            rspice.Netlist.parse(deck)

        assert failure.value.kind == "undefined_output_symbols"
        unresolved = failure.value.unresolved_output_symbols
        assert [(item.directive, item.symbol, item.kind) for item in unresolved] == [
            ("four", "nosuch", "node")
        ]

    def test_four_without_a_transient_is_recorded_as_skipped(self, engine):
        deck = SINE.replace(".end", ".four 1k V(in)\n.end")
        report = engine.run(rspice.Netlist.parse(deck))

        four = [record for record in report.records if record.kind == "four"]
        assert len(four) == 1
        assert four[0].skipped
        assert "tran" in four[0].reason

    def test_four_with_too_short_a_transient_is_recorded_as_skipped(self, engine):
        deck = SINE.replace(".end", ".tran 2u 100u\n.four 1k V(in)\n.end")
        report = engine.run(rspice.Netlist.parse(deck))

        assert report.fourier == []
        four = [record for record in report.records if record.kind == "four"]
        assert len(four) == 1
        assert four[0].skipped
        assert "shorter than the required Fourier window" in four[0].reason
