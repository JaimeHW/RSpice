"""DC operating point and sweep analyses."""

import numpy as np
import pytest

import rspice


class TestDcOp:
    def test_device_operating_point_report(self, engine):
        netlist = rspice.Netlist.parse(
            """* MOS operating point
VDD d 0 5
VG g 0 3
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS (LEVEL=1 VTO=1 KP=100u)
.op
.end
"""
        )
        result = engine.run_dc_op(netlist)
        assert len(result.device_operating_points) == 1
        m1 = result.device_operating_point("m1")
        assert isinstance(m1, rspice.DeviceOperatingPoint)
        assert m1.name == "M1"
        assert m1.device_kind == "MOSFET"
        assert m1.region
        assert {"id", "gm", "gds", "vgs", "vds"} <= set(m1.param_names)
        assert m1["gm"] > 0.0
        assert m1.params["gm"] == m1.param("GM")
        with pytest.raises(KeyError):
            m1.param("missing")
        with pytest.raises(KeyError):
            result.device_operating_point("missing")

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
    def test_device_operating_points_are_preserved_at_every_sweep_point(self, engine):
        netlist = rspice.Netlist.parse(
            """* MOS sweep operating points
VDD d 0 5
VG g 0 0
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS (LEVEL=1 VTO=1 KP=100u)
.end
"""
        )
        sweep = engine.run_dc_sweep(netlist, "VG", 0.0, 3.0, 1.0)
        assert len(sweep) == 4
        for index in range(len(sweep)):
            operating_points = sweep.device_operating_points_at(index)
            assert [point.name for point in operating_points] == ["M1"]
            _, result = sweep[index]
            assert result.device_operating_point("M1").params == operating_points[0].params
        assert sweep.device_operating_points_at(3)[0]["gm"] > 0.0
        with pytest.raises(IndexError):
            sweep.device_operating_points_at(99)

    def test_two_source_sweep_preserves_both_coordinates(self, engine):
        netlist = rspice.Netlist.parse(
            """* nested DC sweep
V1 in 0 0
V2 bias 0 0
R1 in out 1k
R2 bias out 1k
.dc V1 0 2 1 V2 0 1 1
.end
"""
        )
        sweep = engine.run(netlist).dc
        assert sweep.is_nested
        assert sweep.primary_source == "V1"
        assert sweep.secondary_source == "V2"
        assert sweep.shape == (2, 3)
        np.testing.assert_array_equal(sweep.sweep_values, [0, 1, 2, 0, 1, 2])
        np.testing.assert_array_equal(sweep.secondary_sweep_values, [0, 1])
        assert [sweep.secondary_value_at(i) for i in range(6)] == [0, 0, 0, 1, 1, 1]
        assert sweep.voltage(4, "out") == pytest.approx(1.0, abs=1e-9)

    def test_sweep_values_and_results(self, engine, divider):
        sweep = engine.run_dc_sweep(divider, "V1", 0.0, 5.0, 1.0)
        assert len(sweep) == 6
        np.testing.assert_allclose(sweep.sweep_values, np.arange(0.0, 6.0))
        np.testing.assert_allclose(
            sweep.voltage_array("out"), np.arange(0.0, 6.0) / 2, atol=1e-9
        )

    def test_run_uses_netlist_dc_list_mode(self, engine):
        netlist = rspice.Netlist.parse(
            """* DC list sweep
V1 in 0 0
R1 in out 1k
R2 out 0 1k
.dc V1 list 0 2 5
.end
"""
        )

        assert netlist.analyses == [".dc V1 list 0 2 5"]

        report = engine.run(netlist)

        assert report.analyses_run == ["dc"]
        assert report.dc is not None
        np.testing.assert_allclose(report.dc.sweep_values, [0.0, 2.0, 5.0])
        np.testing.assert_allclose(report.dc.voltage_array("out"), [0.0, 1.0, 2.5])

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

    def test_wrong_direction_step_raises_valueerror(self, engine, divider):
        with pytest.raises(ValueError, match="sign"):
            engine.run_dc_sweep(divider, "V1", 0.0, 5.0, -1.0)
        with pytest.raises(ValueError, match="sign"):
            engine.run_dc_sweep(divider, "V1", 5.0, 0.0, 1.0)

    def test_non_finite_bounds_raise_valueerror(self, engine, divider):
        with pytest.raises(ValueError):
            engine.run_dc_sweep(divider, "V1", 0.0, float("inf"), 1.0)
        with pytest.raises(ValueError):
            engine.run_dc_sweep(divider, "V1", float("nan"), 5.0, 1.0)

    def test_unknown_source_raises_simulationerror(self, engine, divider):
        with pytest.raises(rspice.SimulationError) as exc_info:
            engine.run_dc_sweep(divider, "Vmissing", 0.0, 5.0, 1.0)
        assert exc_info.value.kind == "circuit"
        assert exc_info.value.iterations is None
