"""Parametric stepping (`.STEP`)."""

import pickle

import numpy as np
import pytest

import rspice



class TestStep:
    def test_engine_run_executes_step_directive(self, engine):
        netlist = rspice.Netlist.parse(
            """* STEP directive
.param rval=1k
V1 in 0 10
R1 in out {rval}
R2 out 0 1k
.step param rval list 1k 2k 5k
.end
"""
        )
        report = engine.run(netlist)
        assert report.step is not None
        assert report.step.primary_source.casefold() == "rval"
        np.testing.assert_array_equal(report.step.sweep_values, [1e3, 2e3, 5e3])
        assert report.step.voltage(2, "out") == pytest.approx(10 / 6, abs=1e-6)
        assert report.analyses_run == ["op", "op", "op"]
        assert [record.analysis_id for record in report.records] == [
            "implicit-op-001",
            "implicit-op-001",
            "implicit-op-001",
        ]
        assert [record.coordinate.assignments[0].value for record in report.records] == [
            1e3,
            2e3,
            5e3,
        ]

    def test_engine_run_executes_temperature_directive(self, engine, divider):
        netlist = rspice.Netlist.parse(
            """* TEMP directive
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.temp 25 100
.end
"""
        )
        report = engine.run(netlist)
        assert report.temperature is not None
        assert report.temperature.primary_source == "TEMP"
        np.testing.assert_array_equal(report.temperature.sweep_values, [25, 100])
        assert report.analyses_run == ["op", "op"]
        assert [record.analysis_id for record in report.records] == [
            "implicit-op-001",
            "implicit-op-001",
        ]
        assert [record.coordinate.assignments[0].value for record in report.records] == [
            25,
            100,
        ]

    def test_temperature_wraps_transient_and_repeated_frequency_analyses(self, engine):
        netlist = rspice.Netlist.parse(
            """* TEMP analysis axis
V1 in 0 DC 1 AC 1
R1 in out 1k TC=0.01
R2 out 0 1k
.temp 25 100
.tran 1u 2u
.ac lin 2 1k 2k
.ac lin 3 10k 30k
.end
"""
        )

        report = engine.run(netlist)
        assert report.op is None
        assert report.temperature is None
        assert len(report.all_tran) == 2
        assert len(report.all_ac) == 4
        assert [record.kind for record in report.records] == [
            "tran",
            "ac",
            "ac",
            "tran",
            "ac",
            "ac",
        ]
        assert [record.analysis_id for record in report.records] == [
            "tran-001",
            "ac-001",
            "ac-002",
            "tran-001",
            "ac-001",
            "ac-002",
        ]
        assert [record.coordinate.ordinal for record in report.records] == [0, 0, 0, 1, 1, 1]
        assert [record.coordinate.assignments[0].value for record in report.records] == [
            25,
            25,
            25,
            100,
            100,
            100,
        ]
        frequency_lengths = [len(result.frequencies) for result in report.all_ac]
        assert frequency_lengths[:2] == frequency_lengths[2:]
        assert frequency_lengths[0] != frequency_lengths[1]

        restored_record = pickle.loads(pickle.dumps(report.records[-1]))
        assert restored_record.analysis_id == "ac-002"
        assert restored_record.coordinate.id == report.records[-1].coordinate.id
        assert restored_record.coordinate.assignments[0].value == 100

        repeated = engine.run(netlist)
        assert [
            (record.analysis_id, record.coordinate.id) for record in repeated.records
        ] == [
            (record.analysis_id, record.coordinate.id) for record in report.records
        ]

    def test_temperature_and_step_execute_as_a_canonical_cartesian_product(self, engine):
        netlist = rspice.Netlist.parse(
            """* TEMP plus STEP
.param rval=1k
V1 in 0 1
R1 in 0 {rval}
.temp 25 100
.step param rval list 1k 2k
.op
.end
"""
        )
        report = engine.run(netlist)

        assert report.step is None
        assert report.temperature is None
        assert len(report.all_op) == 4
        assert report.analyses_run == ["op", "op", "op", "op"]
        assert [record.analysis_id for record in report.records] == ["op-001"] * 4
        assert [record.coordinate.ordinal for record in report.records] == [0, 1, 2, 3]
        assert [
            [assignment.kind for assignment in record.coordinate.assignments]
            for record in report.records
        ] == [["step", "temperature"]] * 4
        assert [
            [assignment.value for assignment in record.coordinate.assignments]
            for record in report.records
        ] == [
            [1e3, 25],
            [2e3, 25],
            [1e3, 100],
            [2e3, 100],
        ]

    def test_scalar_run_records_repeated_analysis_ordinals(self, engine):
        netlist = rspice.Netlist.parse(
            """* repeated scalar analyses
V1 in 0 DC 1 AC 1
R1 in 0 1k
.tran 1u 2u
.ac lin 2 1k 2k
.ac lin 3 10k 30k
.end
"""
        )

        report = engine.run(netlist)

        assert [record.analysis_id for record in report.records] == [
            "tran-001",
            "ac-001",
            "ac-002",
        ]
        assert all(record.coordinate is None for record in report.records)

    def test_data_step_and_temperature_share_one_coordinate_order(self, engine):
        netlist = rspice.Netlist.parse(
            """* DATA plus STEP plus TEMP
.param gain=1 bias=1
V1 out 0 {gain+bias}
R1 out 0 1k
.data samples bias
10
20
.enddata
.step data=samples
.step param gain list 1 2
.temp 25 100
.op
.end
"""
        )

        report = engine.run(netlist)

        assert len(report.all_op) == 8
        assert [result.voltage("out") for result in report.all_op] == pytest.approx(
            [11, 21, 12, 22, 11, 21, 12, 22]
        )
        assert [
            [assignment.kind for assignment in record.coordinate.assignments]
            for record in report.records
        ] == [["data", "step", "temperature"]] * 8
        assert [
            record.coordinate.assignments[0].value_index for record in report.records
        ] == [0, 1, 0, 1, 0, 1, 0, 1]
        assert [
            record.coordinate.assignments[1].value for record in report.records
        ] == [1, 1, 2, 2, 1, 1, 2, 2]
        assert [
            record.coordinate.assignments[2].value for record in report.records
        ] == [25, 25, 25, 25, 100, 100, 100, 100]

    def test_measurements_are_evaluated_per_coordinate_without_stale_results(self, engine):
        netlist = rspice.Netlist.parse(
            """* coordinate-local measurements
.param amplitude=1
V1 out 0 {amplitude}
R1 out 0 1k
.step param amplitude list 1 2
.tran 1n 2n
.meas tran vmax MAX V(out)
.end
"""
        )

        report = engine.run(netlist)

        assert len(report.measurements) == 2
        assert [measurement.value for measurement in report.measurements] == pytest.approx(
            [1, 2]
        )
        assert [measurement.analysis_id for measurement in report.measurements] == [
            "tran-001",
            "tran-001",
        ]
        assert [
            measurement.coordinate.assignments[0].value
            for measurement in report.measurements
        ] == [1, 2]
        assert all(measurement.passed for measurement in report.measurements)

    def test_advanced_results_are_retained_for_every_coordinate(self, engine):
        netlist = rspice.Netlist.parse(
            """* stepped transfer functions
.param rval=1k
V1 in 0 1
R1 in out {rval}
R2 out 0 1k
.step param rval list 1k 2k
.tf V(out) V1
.end
"""
        )

        report = engine.run(netlist)

        assert len(report.all_tf) == 2
        assert [result.gain for result in report.all_tf] == pytest.approx([0.5, 1 / 3])
        assert report.tf.gain == pytest.approx(report.all_tf[-1].gain)
        assert [record.analysis_id for record in report.records] == ["tf-001"] * 2
        assert [record.coordinate.assignments[0].value for record in report.records] == [
            1e3,
            2e3,
        ]

    def test_step_varies_results(self, engine, param_divider):
        results = engine.run_step(param_divider, "rval", [1e3, 2e3, 5e3])
        assert len(results) == 3
        outs = [sol.voltage("out") for _, sol in results]
        assert outs[0] == pytest.approx(5.0, abs=1e-6)
        assert outs[1] == pytest.approx(10 * 1000 / 3000, abs=1e-6)
        assert outs[2] == pytest.approx(10 * 1000 / 6000, abs=1e-6)

    def test_step_element_name_raises(self, engine, divider):
        with pytest.raises(rspice.SimulationError):
            engine.run_step(divider, "R1", [1e3, 2e3])

    def test_step_empty_values_raise(self, engine, param_divider):
        with pytest.raises(ValueError, match="must not be empty"):
            engine.run_step(param_divider, "rval", [])

    def test_percentile_rejects_invalid_values(self, engine, param_divider):
        stats = engine.run_monte_carlo(
            param_divider, num_runs=10, seed=7
        ).get_variable("V(OUT)")
        assert stats is not None
        for value in (-1.0, 101.0, float("nan"), float("inf")):
            with pytest.raises(ValueError, match="0 to 100"):
                stats.percentile(value)

    def test_step_non_finite_values_raise_valueerror(self, engine, param_divider):
        with pytest.raises(ValueError, match="finite"):
            engine.run_step(param_divider, "rval", [1e3, float("nan")])
