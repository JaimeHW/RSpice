"""SPICE output-specification handling.

`.FOUR` and the transient accessors share one probe grammar, so a branch
current or a differential node pair resolves the same way everywhere.
"""

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
        "spec", ["", "   ", "P(R1)", "V(a,b,c)", "I(V1,V2)", "V()", "V(in"]
    )
    def test_malformed_specifications_raise_value_error(self, tran, spec):
        with pytest.raises(ValueError):
            tran.signal(spec)

    @pytest.mark.parametrize("spec", ["V(nosuch)", "I(nosuch)", "V(in,nosuch)"])
    def test_unknown_quantities_raise_key_error_naming_the_spec(self, tran, spec):
        with pytest.raises(KeyError, match=spec.replace("(", r"\(").replace(")", r"\)")):
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
