"""The general `.DC` sweep form.

`run_dc_sweep` covers the linear single-source case. `DcSweep` plus
`run_dc_sweep_spec` covers everything else `.DC` accepts: explicit value
lists, logarithmic axes, and a nested second source.
"""

import numpy as np
import pytest

import rspice

DECK = """* Two independent sources
V1 in 0 0
V2 aux 0 0
R1 in out 1k
R2 out 0 1k
Raux aux out 1k
.end
"""


@pytest.fixture()
def netlist():
    return rspice.Netlist.parse(DECK)


class TestDcSweepSpec:
    def test_linear_axis_matches_the_shorthand(self, engine, netlist):
        spec = engine.run_dc_sweep_spec(netlist, rspice.DcSweep("V1", 0, 5, 1))
        shorthand = engine.run_dc_sweep(netlist, "V1", 0, 5, 1)
        np.testing.assert_allclose(spec.sweep_values, shorthand.sweep_values)
        np.testing.assert_allclose(
            spec.voltage_array("out"), shorthand.voltage_array("out")
        )

    def test_explicit_value_list(self, engine, netlist):
        values = [0.0, 1.8, 3.3, 5.0]
        sweep = rspice.DcSweep("V1", values=values)

        assert sweep.mode == "list"
        np.testing.assert_allclose(sweep.values, values)
        result = engine.run_dc_sweep_spec(netlist, sweep)
        np.testing.assert_allclose(result.sweep_values, values)

    def test_decade_axis(self):
        sweep = rspice.DcSweep("V1", 1, 1000, mode="dec", points=3)
        assert sweep.mode == "dec"
        assert sweep.values[0] == pytest.approx(1.0)
        assert sweep.values[-1] == pytest.approx(1000.0)
        # Three points per decade across three decades, endpoint included.
        assert sweep.num_points == 10

    def test_octave_axis(self):
        sweep = rspice.DcSweep("V1", 1, 8, mode="oct", points=2)
        assert sweep.mode == "oct"
        assert sweep.values[0] == pytest.approx(1.0)
        assert sweep.values[-1] == pytest.approx(8.0)

    def test_repr_summarizes_the_axis(self):
        sweep = rspice.DcSweep("V1", 0, 5, 1)
        assert repr(sweep) == "DcSweep(source='V1', mode='linear', points=6)"


class TestNestedSweep:
    def test_shape_and_ordering(self, engine, netlist):
        inner = rspice.DcSweep("V1", 0, 4, 2)
        outer = rspice.DcSweep("V2", values=[0.0, 1.0])
        result = engine.run_dc_sweep_spec(netlist, inner, sweep2=outer)

        assert result.is_nested
        assert result.shape == (2, 3)
        assert len(result) == 6
        # The inner axis varies fastest, matching how .DC emits its grid.
        np.testing.assert_allclose(result.sweep_values, [0, 2, 4, 0, 2, 4])

    def test_secondary_coordinate_is_addressable(self, engine, netlist):
        inner = rspice.DcSweep("V1", 0, 4, 2)
        outer = rspice.DcSweep("V2", values=[0.0, 1.0])
        result = engine.run_dc_sweep_spec(netlist, inner, sweep2=outer)

        assert result.secondary_value_at(0) == pytest.approx(0.0)
        assert result.secondary_value_at(3) == pytest.approx(1.0)

    def test_outer_source_actually_varies_the_circuit(self, engine, netlist):
        inner = rspice.DcSweep("V1", 0, 0, 1)
        outer = rspice.DcSweep("V2", values=[0.0, 3.0])
        result = engine.run_dc_sweep_spec(netlist, inner, sweep2=outer)

        # With V1 at 0 the output is set by V2 through Raux and R2.
        assert result.voltage(0, "out") == pytest.approx(0.0)
        assert result.voltage(1, "out") > 0.5

    def test_a_nested_sweep_needs_two_sources(self, engine, netlist):
        axis = rspice.DcSweep("V1", 0, 4, 2)
        with pytest.raises(ValueError, match="two different sources"):
            engine.run_dc_sweep_spec(netlist, axis, sweep2=rspice.DcSweep("V1", 0, 1, 1))

    def test_nested_result_exports(self, engine, netlist):
        inner = rspice.DcSweep("V1", 0, 4, 2)
        outer = rspice.DcSweep("V2", values=[0.0, 1.0])
        result = engine.run_dc_sweep_spec(netlist, inner, sweep2=outer)

        columns = result.export_columns
        assert columns[0] == "v-sweep(V1)"
        assert columns[1] == "v-sweep2(V2)"
        assert len(result.to_csv().splitlines()) == len(result) + 1


class TestValidation:
    @pytest.mark.parametrize(
        ("kwargs", "match"),
        [
            ({"start": 0, "stop": 5}, "requires start, stop, and step"),
            ({"start": 0, "stop": 5, "step": 0}, "non-zero"),
            ({"start": 0, "stop": 5, "step": -1}, "sign"),
            ({"mode": "list"}, "requires values"),
            ({"mode": "list", "values": []}, "must not be empty"),
            ({"start": 0, "stop": 5, "step": 1, "values": [1.0]}, "cannot be combined"),
            ({"mode": "dec", "points": 3, "values": [1.0]}, "cannot be combined"),
            ({"start": 1, "stop": 5, "step": 1, "points": 3}, "only valid with"),
            ({"start": -1, "stop": 5, "mode": "dec", "points": 3}, "positive"),
            ({"start": 1, "stop": 5, "mode": "dec"}, "require points"),
            ({"start": 1, "stop": 5, "mode": "dec", "points": 0}, "at least 1"),
            ({"start": 0, "stop": 5, "step": 1, "mode": "bogus"}, "mode must be"),
        ],
    )
    def test_unusable_axes_are_rejected(self, kwargs, match):
        with pytest.raises(ValueError, match=match):
            rspice.DcSweep("V1", **kwargs)

    def test_empty_source_is_rejected(self):
        with pytest.raises(ValueError, match="source"):
            rspice.DcSweep("  ", 0, 5, 1)

    @pytest.mark.parametrize("value", [float("nan"), float("inf")])
    def test_non_finite_bounds_are_rejected(self, value):
        with pytest.raises(ValueError, match="finite"):
            rspice.DcSweep("V1", 0, value, 1)

    def test_non_finite_list_values_are_rejected(self):
        with pytest.raises(ValueError, match="finite"):
            rspice.DcSweep("V1", values=[0.0, float("nan")])
