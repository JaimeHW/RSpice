"""Exception hierarchy and Engine.run failure handling.

Every failure this library raises must be catchable as ``rspice.RSpiceError``
without breaking callers that catch the builtin Python exception for that
failure mode, and a single failing directive must not discard the results of
every other directive in the deck.
"""

import pytest

import rspice

DIVIDER = """* Voltage divider
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.op
.end
"""

BAD_DIRECTIVE = """* One unresolvable directive among healthy ones
V1 in 0 1
R1 in out 1k
R2 out 0 1k
.op
.pz nosuch 0 out 0 vol pz
.tran 1u 100u
.end
"""


@pytest.fixture()
def op(engine):
    return engine.run_dc_op(rspice.Netlist.parse(DIVIDER))


class TestHybridExceptions:
    """Lookup and argument errors satisfy both contracts at once."""

    def test_unknown_node_is_both_rspice_error_and_key_error(self, op):
        with pytest.raises(rspice.RSpiceError):
            op.voltage("nosuch")
        with pytest.raises(KeyError):
            op.voltage("nosuch")
        with pytest.raises(rspice.RSpiceKeyError):
            op.voltage("nosuch")

    def test_out_of_range_index_is_both_rspice_error_and_index_error(self, op):
        with pytest.raises(rspice.RSpiceError):
            op.voltage(9999)
        with pytest.raises(IndexError):
            op.voltage(9999)

    def test_invalid_argument_is_both_rspice_error_and_value_error(self, engine):
        netlist = rspice.Netlist.parse(DIVIDER)
        with pytest.raises(rspice.RSpiceError):
            engine.run_ac(netlist, [])
        with pytest.raises(ValueError):
            engine.run_ac(netlist, [])

    def test_wrong_argument_type_is_both_rspice_error_and_type_error(self, engine, op):
        netlist = rspice.Netlist.parse(DIVIDER)
        with pytest.raises(rspice.RSpiceError):
            engine.measure(netlist, op)
        with pytest.raises(TypeError):
            engine.measure(netlist, op)

    @pytest.mark.parametrize(
        ("name", "builtin"),
        [
            ("RSpiceKeyError", KeyError),
            ("RSpiceIndexError", IndexError),
            ("RSpiceValueError", ValueError),
            ("RSpiceTypeError", TypeError),
        ],
    )
    def test_hybrid_classes_are_exported_with_both_bases(self, name, builtin):
        cls = getattr(rspice, name)
        assert issubclass(cls, rspice.RSpiceError)
        assert issubclass(cls, builtin)
        assert name in rspice.__all__

    def test_parse_failures_remain_parse_errors(self):
        with pytest.raises(rspice.ParseError):
            rspice.Netlist.parse("V1 in 0 10\nRbogus\n.end")


class TestRunFailureHandling:
    """A failing directive is recorded, not fatal, unless asked otherwise."""

    def test_failed_directive_becomes_a_skipped_record(self, engine):
        report = engine.run(rspice.Netlist.parse(BAD_DIRECTIVE))

        kinds = {record.kind: record for record in report.records}
        assert set(kinds) == {"op", "pz", "tran"}
        assert kinds["pz"].skipped
        assert "NOSUCH" in kinds["pz"].reason.upper()
        assert not kinds["op"].skipped
        assert not kinds["tran"].skipped

    def test_directives_after_a_failure_still_run(self, engine):
        report = engine.run(rspice.Netlist.parse(BAD_DIRECTIVE))
        assert report.op is not None
        assert report.tran is not None
        assert report.pz is None

    def test_a_skipped_directive_still_fails_the_gate(self, engine):
        report = engine.run(rspice.Netlist.parse(BAD_DIRECTIVE))
        assert not report.all_passed
        with pytest.raises(rspice.MeasurementError, match="skipped"):
            report.assert_passed()

    def test_strict_mode_raises_the_original_error(self, engine):
        with pytest.raises(rspice.RSpiceError):
            engine.run(rspice.Netlist.parse(BAD_DIRECTIVE), continue_on_error=False)

    def test_healthy_deck_records_nothing_as_skipped(self, engine):
        report = engine.run(rspice.Netlist.parse(DIVIDER))
        assert report.skipped == []


class TestRepeatedDirectives:
    """A deck may carry more than one directive of the same kind."""

    DECK = """* Two AC bands
V1 in 0 AC 1
R1 in out 1k
C1 out 0 1u
.ac dec 5 1 100
.ac dec 5 1k 100k
.end
"""

    def test_every_result_is_retained_in_deck_order(self, engine):
        report = engine.run(rspice.Netlist.parse(self.DECK))

        assert len(report.all_ac) == 2
        assert report.all_ac[0].frequencies[0] == pytest.approx(1.0)
        assert report.all_ac[1].frequencies[0] == pytest.approx(1e3)

    def test_the_singular_accessor_is_the_last_result(self, engine):
        report = engine.run(rspice.Netlist.parse(self.DECK))
        assert report.ac.frequencies[0] == report.all_ac[-1].frequencies[0]

    def test_plural_accessors_exist_for_every_repeatable_kind(self, engine):
        report = engine.run(rspice.Netlist.parse(DIVIDER))
        assert len(report.all_op) == 1
        assert report.all_dc == []
        assert report.all_tran == []
        assert report.all_noise == []


class TestResultConsistency:
    """Accessors that used to disagree now behave identically."""

    DIODE = """* Diode sweep
V1 in 0 1
R1 in mid 1k
D1 mid 0 DM
.model DM D(IS=1e-14)
.end
"""

    def test_result_at_carries_device_operating_points(self, engine):
        sweep = engine.run_dc_sweep(rspice.Netlist.parse(self.DIODE), "V1", 0, 1, 0.5)

        from_index = sweep[0][1].device_operating_points
        from_method = sweep.result_at(0).device_operating_points
        assert len(from_method) == len(from_index) == 1
        assert from_method[0].name == from_index[0].name

    def test_unknown_monte_carlo_variable_raises(self, engine, param_divider):
        result = engine.run_monte_carlo(param_divider, 16, seed=7)

        assert "V(OUT)" in result
        assert result["V(OUT)"].mean == pytest.approx(result.mean("V(OUT)"))
        assert result.try_variable("V(NOPE)") is None
        for probe in (result.get_variable, result.mean, result.std_dev, result.range):
            with pytest.raises(KeyError):
                probe("V(NOPE)")

    def test_configuration_objects_compare_by_value(self):
        assert rspice.SimulationConfig(tolerance=1e-9) == rspice.SimulationConfig(
            tolerance=1e-9
        )
        assert rspice.SimulationConfig(tolerance=1e-9) != rspice.SimulationConfig(
            tolerance=1e-8
        )
        assert rspice.ConvergenceConfig.robust() == rspice.ConvergenceConfig.robust()
        assert rspice.ConvergenceConfig.robust() != rspice.ConvergenceConfig.fast()
        assert rspice.BypassConfig(enabled=True) != rspice.BypassConfig(enabled=False)
        assert rspice.ResourceLimits() == rspice.ResourceLimits()
        assert rspice.ResourceLimits() != rspice.ResourceLimits.unlimited()

    def test_nested_configuration_participates_in_equality(self):
        left = rspice.SimulationConfig(convergence=rspice.ConvergenceConfig.robust())
        right = rspice.SimulationConfig(convergence=rspice.ConvergenceConfig.fast())
        assert left != right
