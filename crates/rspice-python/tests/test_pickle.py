"""Pickle and copy support.

Multiprocessing is how most callers parallelize on the GIL-enabled
interpreters this package supports, so netlists, configurations, and results
have to survive a process boundary intact.
"""

import copy
import pickle

import numpy as np
import pytest

import rspice

DECK = """* Parametric divider
.param rval=1k
V1 in 0 10
R1 in out {rval}
R2 out 0 1k
.op
.end
"""


def round_trip(obj, protocol=pickle.HIGHEST_PROTOCOL):
    return pickle.loads(pickle.dumps(obj, protocol))


class TestEnums:
    @pytest.mark.parametrize(
        "member",
        [
            rspice.DampingStrategy.NONE,
            rspice.DampingStrategy.LINE_SEARCH,
            rspice.DampingStrategy.VOLTAGE_LIMITING,
            rspice.DampingStrategy.BANK_ROSE,
            rspice.DampingStrategy.COMBINED,
            rspice.IntegrationMethod.BACKWARD_EULER,
            rspice.IntegrationMethod.TRAPEZOIDAL,
            rspice.IntegrationMethod.GEAR2,
            rspice.IntegrationMethod.TRAP_GEAR,
        ],
    )
    def test_members_round_trip(self, member):
        assert round_trip(member) == member
        assert copy.deepcopy(member) == member


class TestConfiguration:
    def test_bypass_config(self):
        original = rspice.BypassConfig(enabled=True, reltol=2e-3, abstol=1e-9)
        restored = round_trip(original)
        assert restored == original
        assert restored.enabled and restored.reltol == 2e-3

    def test_convergence_config(self):
        original = rspice.ConvergenceConfig.robust()
        restored = round_trip(original)
        assert restored == original
        assert restored.damping_strategy == original.damping_strategy
        assert restored.charge_abstol == original.charge_abstol

    def test_resource_limits(self):
        original = rspice.ResourceLimits(max_batch_runs=77, max_netlist_bytes=4096)
        restored = round_trip(original)
        assert restored == original
        assert restored.max_batch_runs == 77

    def test_unlimited_resource_limits(self):
        assert round_trip(rspice.ResourceLimits.unlimited()) == (
            rspice.ResourceLimits.unlimited()
        )

    def test_simulation_config_carries_nested_objects(self):
        original = rspice.SimulationConfig(
            tolerance=1e-11,
            temperature=350.0,
            integration_method=rspice.IntegrationMethod.GEAR2,
            convergence=rspice.ConvergenceConfig.robust(),
            bypass=rspice.BypassConfig(enabled=True, reltol=2e-3),
            resource_limits=rspice.ResourceLimits(max_batch_runs=77),
        )
        restored = round_trip(original)

        assert restored == original
        assert restored.temperature == 350.0
        assert restored.integration_method == rspice.IntegrationMethod.GEAR2
        assert restored.convergence == original.convergence
        assert restored.bypass.reltol == 2e-3
        assert restored.resource_limits.max_batch_runs == 77

    def test_max_timestep_infinity_survives(self):
        original = rspice.SimulationConfig(max_timestep=float("inf"))
        assert round_trip(original).max_timestep == float("inf")

    def test_deepcopy_is_independent(self):
        original = rspice.BypassConfig(enabled=False)
        clone = copy.deepcopy(original)
        clone.enabled = True
        assert not original.enabled

    def test_engine_accepts_an_unpickled_config(self):
        config = rspice.SimulationConfig(tolerance=1e-10, temperature=310.0)
        engine = rspice.Engine(round_trip(config))
        assert engine.config.temperature == 310.0


class TestNetlist:
    def test_round_trip_preserves_structure(self):
        original = rspice.Netlist.parse(DECK)
        restored = round_trip(original)

        assert restored.title == original.title
        assert restored.num_elements == original.num_elements
        assert restored.element_names == original.element_names
        assert restored.analyses == original.analyses
        assert restored.source == original.source

    def test_unpickled_netlist_simulates_identically(self, engine):
        original = rspice.Netlist.parse(DECK)
        restored = round_trip(original)
        assert engine.run_dc_op(restored).voltage("out") == pytest.approx(
            engine.run_dc_op(original).voltage("out")
        )

    def test_parse_spice_semantics_survive(self):
        original = rspice.Netlist.parse_spice("My Amplifier\nV1 in 0 10\nR1 in 0 1k\n.end")
        restored = round_trip(original)
        assert restored.title == "My Amplifier"
        assert restored.num_elements == original.num_elements

    def test_resource_policy_is_carried(self):
        limits = rspice.ResourceLimits(max_netlist_bytes=4096)
        restored = round_trip(rspice.Netlist.parse(DECK, resource_limits=limits))
        # The policy is reapplied on unpickling, so a deck that no longer fits
        # would fail rather than silently parsing under the defaults.
        assert restored.num_elements == 3

    def test_source_path_is_carried(self, tmp_path):
        deck = tmp_path / "deck.cir"
        deck.write_text(DECK)
        original = rspice.Netlist.parse_file(deck)
        restored = round_trip(original)
        assert restored.source_path == original.source_path

    @pytest.mark.parametrize("protocol", range(2, pickle.HIGHEST_PROTOCOL + 1))
    def test_every_supported_protocol(self, protocol):
        original = rspice.Netlist.parse(DECK)
        assert round_trip(original, protocol).num_elements == original.num_elements

    def test_deepcopy(self):
        original = rspice.Netlist.parse(DECK)
        assert copy.deepcopy(original).element_names == original.element_names


ANALYSIS_DECK = """* Every result kind from one deck
.param rval=1k
V1 in 0 AC 1 SIN(0 1 1k)
R1 in out {rval}
C1 out 0 1u
.end
"""


@pytest.fixture(scope="module")
def analysis_netlist():
    return rspice.Netlist.parse(ANALYSIS_DECK)


class TestResults:
    """Every result a worker can return survives a process boundary."""

    def test_simulation_result(self, engine, analysis_netlist):
        original = engine.run_dc_op(analysis_netlist)
        restored = round_trip(original)

        assert restored.voltage("out") == original.voltage("out")
        assert restored.node_names == original.node_names
        assert restored.branch_names == original.branch_names
        assert restored.branch_current("V1") == original.branch_current("V1")

    def test_transient_result_including_derived_analyses(self, engine, analysis_netlist):
        original = engine.run_tran(analysis_netlist, stop_time=2e-3, max_step=2e-5)
        restored = round_trip(original)

        assert restored.num_points == original.num_points
        np.testing.assert_allclose(restored.time, original.time)
        np.testing.assert_allclose(
            restored.voltage_waveform("out"), original.voltage_waveform("out")
        )
        np.testing.assert_allclose(
            restored.branch_current_waveform("V1"),
            original.branch_current_waveform("V1"),
        )
        # Derived analyses must still work off the rebuilt result.
        assert (
            restored.fourier("out", 1e3).thd_percent
            == original.fourier("out", 1e3).thd_percent
        )
        assert restored.to_csv() == original.to_csv()

    def test_ac_result(self, engine, analysis_netlist):
        original = engine.run_ac_sweep(analysis_netlist, "dec", 5, 10.0, 1e4)
        restored = round_trip(original)

        np.testing.assert_allclose(restored.frequencies, original.frequencies)
        np.testing.assert_allclose(
            restored.voltage_complex("out"), original.voltage_complex("out")
        )
        np.testing.assert_allclose(
            restored.branch_current_complex("V1"),
            original.branch_current_complex("V1"),
        )
        assert restored.to_csv() == original.to_csv()

    def test_dc_sweep_result(self, engine, analysis_netlist):
        original = engine.run_dc_sweep(analysis_netlist, "V1", 0, 5, 1)
        restored = round_trip(original)

        assert len(restored) == len(original)
        np.testing.assert_allclose(restored.sweep_values, original.sweep_values)
        np.testing.assert_allclose(
            restored.voltage_array("out"), original.voltage_array("out")
        )
        assert restored.export_columns == original.export_columns

    def test_noise_results_keep_their_contributors(self, engine, analysis_netlist):
        original = engine.run_noise(analysis_netlist, "out", [1e3, 1e4])
        restored = round_trip(original)

        assert len(restored) == len(original)
        for left, right in zip(restored, original):
            assert left.frequency == right.frequency
            assert left.output_noise_density == right.output_noise_density
            assert [c.device_name for c in left.contributions] == [
                c.device_name for c in right.contributions
            ]

    def test_transfer_function_result(self, engine, analysis_netlist):
        original = engine.run_transfer_function(analysis_netlist, "out", "V1")
        restored = round_trip(original)
        assert restored.gain == original.gain
        assert restored.input_impedance == original.input_impedance

    def test_pole_zero_result(self, engine, analysis_netlist):
        original = engine.run_pz(analysis_netlist, "in", "out")
        restored = round_trip(original)

        assert restored.num_poles == original.num_poles
        assert restored.dc_gain == original.dc_gain
        np.testing.assert_allclose(restored.poles_array, original.poles_array)

    def test_fourier_result(self, engine, analysis_netlist):
        tran = engine.run_tran(analysis_netlist, stop_time=2e-3, max_step=2e-5)
        original = tran.fourier("out", 1e3)
        restored = round_trip(original)

        assert restored.thd_percent == original.thd_percent
        assert restored.dc_component == original.dc_component
        assert [h.magnitude for h in restored.harmonics] == [
            h.magnitude for h in original.harmonics
        ]

    def test_monte_carlo_result(self, engine, analysis_netlist):
        original = engine.run_monte_carlo(analysis_netlist, 24, seed=3)
        restored = round_trip(original)

        assert restored.variable_names == original.variable_names
        assert restored.num_runs == original.num_runs
        np.testing.assert_allclose(
            restored["V(OUT)"].samples, original["V(OUT)"].samples
        )
        assert restored["V(OUT)"].histogram == original["V(OUT)"].histogram

    def test_monte_carlo_payload_is_deterministic(self, engine, analysis_netlist):
        original = engine.run_monte_carlo(analysis_netlist, 16, seed=5)
        assert pickle.dumps(original) == pickle.dumps(original)

    def test_dc_sensitivity_result(self, engine, analysis_netlist):
        original = engine.run_sensitivity_dc_complete(analysis_netlist, "out")
        restored = round_trip(original)

        assert restored.vector_names == original.vector_names
        assert restored.output_value == original.output_value

    def test_ac_sensitivity_result(self, engine, analysis_netlist):
        original = engine.run_sensitivity_ac_complete(analysis_netlist, "out", [1e3])
        restored = round_trip(original)

        assert restored.vector_names == original.vector_names
        np.testing.assert_allclose(restored.output_complex, original.output_complex)
        left = restored.get(restored.vector_names[0])
        right = original.get(original.vector_names[0])
        np.testing.assert_allclose(left.absolute, right.absolute)
        np.testing.assert_allclose(left.db, right.db)

    def test_measurements_and_records(self, engine):
        deck = ANALYSIS_DECK.replace(
            ".end", ".tran 2u 2m\n.meas tran vmax MAX V(out)\n.end"
        )
        report = engine.run(rspice.Netlist.parse(deck))

        measurement = report.measurement("vmax")
        restored = round_trip(measurement)
        assert restored.value == measurement.value
        assert restored.passed == measurement.passed
        assert restored.analysis == measurement.analysis

        record = round_trip(report.records[0])
        assert record.kind == report.records[0].kind
        assert record.skipped == report.records[0].skipped


def _simulate_in_worker(payload):
    """Run one operating point in a spawned process.

    Defined at module scope because Windows spawns rather than forks, so the
    worker has to be importable by name.
    """
    netlist, config = payload
    return rspice.Engine(config).run_dc_op(netlist)


class TestProcessPool:
    """The reason pickling matters: parallelising across processes."""

    def test_netlist_and_result_cross_a_process_boundary(self):
        import multiprocessing

        netlist = rspice.Netlist.parse(DECK)
        config = rspice.SimulationConfig(tolerance=1e-10)

        context = multiprocessing.get_context("spawn")
        with context.Pool(1) as pool:
            result = pool.apply_async(
                _simulate_in_worker, ((netlist, config),)
            ).get(timeout=300)

        assert result.voltage("out") == pytest.approx(5.0)
        assert result.node_names == rspice.Engine().run_dc_op(netlist).node_names
