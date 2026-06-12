"""DC operating point and sweep analyses."""

import numpy as np
import pytest

import rspice


class TestDcOp:
    def test_divider_voltage_by_name_and_index(self, engine, divider):
        op = engine.run_dc_op(divider)
        assert op.voltage("out") == pytest.approx(5.0, abs=1e-6)
        assert op.voltage("in") == pytest.approx(10.0, abs=1e-9)
        assert op.voltage(0) == 0.0

    def test_node_voltages_is_ndarray(self, engine, divider):
        op = engine.run_dc_op(divider)
        v = op.node_voltages
        assert isinstance(v, np.ndarray)
        assert v[0] == 0.0
        assert op.num_nodes == 2
        assert len(op.node_names) == len(v)

    def test_branch_current(self, engine, divider):
        op = engine.run_dc_op(divider)
        # 10 V across 2k total: 5 mA out of the source's + terminal.
        assert abs(op.branch_current("V1")) == pytest.approx(5e-3, rel=1e-6)
        assert len(op.branch_names) == len(op.branch_currents)

    def test_unknown_branch_raises_keyerror(self, engine, divider):
        op = engine.run_dc_op(divider)
        with pytest.raises(KeyError):
            op.branch_current("Vnope")

    def test_unknown_node_raises_keyerror(self, engine, divider):
        op = engine.run_dc_op(divider)
        with pytest.raises(KeyError):
            op.voltage("nonexistent")

    def test_out_of_range_node_raises_indexerror(self, engine, divider):
        op = engine.run_dc_op(divider)
        with pytest.raises(IndexError):
            op.voltage(99)


class TestDcSweep:
    def test_sweep_values_and_results(self, engine, divider):
        sweep = engine.run_dc_sweep(divider, "V1", 0.0, 5.0, 1.0)
        assert len(sweep) == 6
        np.testing.assert_allclose(sweep.sweep_values, np.arange(0.0, 6.0))
        np.testing.assert_allclose(
            sweep.voltage_array("out"), np.arange(0.0, 6.0) / 2, atol=1e-9
        )

    def test_iteration_protocol(self, engine, divider):
        sweep = engine.run_dc_sweep(divider, "V1", 0.0, 5.0, 1.0)
        pairs = [(v, sol.voltage("out")) for v, sol in sweep]
        assert pairs[-1][0] == 5.0
        assert pairs[-1][1] == pytest.approx(2.5, abs=1e-9)
        assert len(list(sweep)) == 6  # iterable repeatedly

    def test_negative_indexing(self, engine, divider):
        sweep = engine.run_dc_sweep(divider, "V1", 0.0, 5.0, 1.0)
        value, sol = sweep[-1]
        assert value == 5.0
        assert sol.voltage("out") == pytest.approx(2.5, abs=1e-9)
        with pytest.raises(IndexError):
            sweep[6]
        with pytest.raises(IndexError):
            sweep[-7]

    def test_point_accessors(self, engine, divider):
        sweep = engine.run_dc_sweep(divider, "V1", 0.0, 5.0, 1.0)
        assert sweep.sweep_value_at(2) == pytest.approx(2.0)
        assert sweep.result_at(2).voltage("out") == pytest.approx(1.0, abs=1e-9)
        assert sweep.voltage(2, "out") == pytest.approx(1.0, abs=1e-9)
        with pytest.raises(IndexError):
            sweep.sweep_value_at(100)
        with pytest.raises(IndexError):
            sweep.result_at(100)

    def test_zero_step_raises_valueerror(self, engine, divider):
        with pytest.raises(ValueError):
            engine.run_dc_sweep(divider, "V1", 0.0, 5.0, 0.0)

    def test_non_finite_bounds_raise_valueerror(self, engine, divider):
        with pytest.raises(ValueError):
            engine.run_dc_sweep(divider, "V1", 0.0, float("inf"), 1.0)
        with pytest.raises(ValueError):
            engine.run_dc_sweep(divider, "V1", float("nan"), 5.0, 1.0)

    def test_unknown_source_raises_simulationerror(self, engine, divider):
        with pytest.raises(rspice.SimulationError):
            engine.run_dc_sweep(divider, "Vmissing", 0.0, 5.0, 1.0)
