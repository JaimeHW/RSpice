"""DC operating point and sweep analyses."""

import numpy as np
import pytest

import rspice


class TestDcOp:
    @pytest.mark.parametrize("level,kind,p", [(11, "NPN", 1), (12, "PNP", -1)])
    def test_vbic13_resistance_floor_preserves_small_currents(self, level, kind, p):
        substrate = " 0" if level == 12 else ""
        netlist = rspice.Netlist.parse_spice(
            f"""* VBIC resistance floor current precision
Vc c 0 {1.8*p}
Vb b 0 SIN({0.65*p} {0.04*p} 50Meg)
Vth th 0 SIN(20 5 50Meg)
Q1 c b 0{substrate} th vm SW_ET=1 M=3 TRISE=20
.model vm {kind}(LEVEL={level} IS=1e-16 NF=1.1 NR=1.2 ISRR=0.7 TNF=0.001 PNJMAXI=1n XISR=1.8 DEAR=0.1 IBEI=1e-18 IBCI=1e-18 IBEIP=0 ISP=0 RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p TD=1n TF=1n TR=2n)
.temp 27
.options gmin=0
.options reltol=1e-8 abstol=1e-14
.end
"""
        )
        engine = rspice.Engine(rspice.SimulationConfig(
            integration_method=rspice.IntegrationMethod.GEAR2,
            convergence=rspice.ConvergenceConfig(gmin_target=0)
        ))
        result = engine.run_dc_op(netlist)
        # Independently solved with 70-digit Decimal arithmetic and Xyce 7.10.
        for branch, current in [("Vc", -7.0452201493508116e-8), ("Vb", -8.783310826230572e-8)]:
            assert result.branch_current(branch) == pytest.approx(p*current, rel=1e-7, abs=0)
        transient = engine.run_tran(netlist, stop_time=2e-8, max_step=2e-12)
        assert transient.branch_current_waveform("Vc")[0] == pytest.approx(
            -p*7.0452201493508116e-8, rel=1e-7, abs=0
        )
        # Xyce Gear2, also checked with the reference maximum step halved.
        assert transient.branch_current_waveform("Vb")[-1] == pytest.approx(
            -p*9.760116458338403e-8, rel=1e-5, abs=0
        )

    @pytest.mark.parametrize("level,kind,p", [(11, "NPN", 1), (12, "PNP", -1)])
    def test_vbic13_pnjmaxi_global_option_matches_xyce(self, engine, level, kind, p):
        substrate = " 0" if level == 12 else ""
        netlist = rspice.Netlist.parse_spice(
            f"""* VBIC PNJMAXI option
Vc c 0 {p}
Vb b 0 {0.8*p}
Q1 c b 0{substrate} vm SW_ET=0
.model vm {kind}(LEVEL={level} IS=1e-16 IBEI=0 IBCI=0 IBEIP=0 ISP=0 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27)
.temp 27
.options DEVICE PNJMAXI=1u
.end
"""
        )
        assert engine.run_dc_op(netlist).branch_current("Vc") == pytest.approx(
            -p * 8.903669004624935e-6, rel=1e-7
        )

    @pytest.mark.parametrize(
        "level, kind, polarity, currents",
        [
            (11, "NPN", 1, [-6.251124606622771e-5, 3.931202464614644e-6]),
            (12, "PNP", -1, [5.919472910615865e-5, 1.1288947081972223e-6]),
        ],
    )
    def test_vbic13_extrinsic_avalanche_matches_xyce(
        self, engine, level, kind, polarity, currents
    ):
        substrate = " 0" if level == 12 else ""
        netlist = rspice.Netlist.parse_spice(
            f"""* VBIC13 extrinsic avalanche
Vc c 0 {polarity * 1.8}
Vb b 0 {polarity * 0.7}
Q1 c b 0{substrate} vm SW_ET=0
.model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVCX1=0.05 AVCX2=0.3 GMIN=1e-6 TNOM=27)
.temp 27
.end
"""
        )
        result = engine.run_dc_op(netlist)
        for branch, current in zip(("vc", "vb"), currents):
            assert result.branch_current(branch) == pytest.approx(current, rel=2e-7)

    @pytest.mark.parametrize(
        "control, current",
        [("", -5.69505259e-5), ("SW_ET=1", -5.69505259e-5), ("SW_ET=0", -5.67002151e-5)],
    )
    def test_vbic_self_heating_switch_matches_xyce(self, engine, control, current):
        netlist = rspice.Netlist.parse_spice(
            f"""* VBIC thermal switch
Vc c 0 1.2
Vb b 0 0.7
Q1 c b 0 vm {control}
.model vm NPN(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCI=0 RBI=0 RTH=1000 TNOM=27)
.temp 27
.end
"""
        )
        assert engine.run_dc_op(netlist).branch_current("vc") == pytest.approx(
            current, rel=1e-5
        )

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
