"""Pickle and copy support.

Multiprocessing is how most callers parallelize on the GIL-enabled
interpreters this package supports, so netlists, configurations, and results
have to survive a process boundary intact.
"""

import copy
import pickle

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
