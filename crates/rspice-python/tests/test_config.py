"""Configuration classes: kwargs construction, copy semantics, enums."""

import ast
import math
from pathlib import Path

import pytest

import rspice


class TestSimulationConfig:
    def test_kwargs_constructor(self):
        config = rspice.SimulationConfig(
            tolerance=1e-12,
            max_iterations=200,
            temperature=350.0,
            integration_method=rspice.IntegrationMethod.GEAR2,
            convergence=rspice.ConvergenceConfig.robust(),
            bypass=rspice.BypassConfig(enabled=True),
        )
        assert config.tolerance == 1e-12
        assert config.max_iterations == 200
        assert config.temperature == 350.0
        assert config.integration_method == rspice.IntegrationMethod.GEAR2
        assert config.bypass.enabled is True

    def test_property_setters(self):
        config = rspice.SimulationConfig()
        config.tolerance = 1e-9
        config.transient_max_iterations = 33
        config.transient_trtol = 5.0
        config.integration_method = rspice.IntegrationMethod.TRAPEZOIDAL
        assert config.tolerance == 1e-9
        assert config.transient_max_iterations == 33
        assert config.transient_trtol == 5.0
        assert config.integration_method == rspice.IntegrationMethod.TRAPEZOIDAL

    def test_invalid_temperature_raises(self):
        with pytest.raises(ValueError):
            rspice.SimulationConfig(temperature=-10.0)

    @pytest.mark.parametrize(
        ("kwargs", "field", "bad_value"),
        [
            ({"tolerance": -1e-9}, "tolerance", -1e-9),
            ({"tolerance": math.nan}, "tolerance", math.nan),
            ({"max_iterations": 0}, "max_iterations", 0),
            ({"transient_max_iterations": 0}, "transient_max_iterations", 0),
            ({"min_timestep": -1e-12}, "min_timestep", -1e-12),
            ({"max_timestep": math.inf}, "max_timestep", math.inf),
            ({"transient_trtol": -1.0}, "transient_trtol", -1.0),
        ],
    )
    def test_invalid_numeric_kwargs_raise(self, kwargs, field, bad_value):
        with pytest.raises(ValueError):
            rspice.SimulationConfig(**kwargs)

        config = rspice.SimulationConfig()
        with pytest.raises(ValueError):
            setattr(config, field, bad_value)

    @pytest.mark.parametrize(
        "kwargs",
        [
            {"min_timestep": 1e-6, "max_timestep": 1e-9},
            {"max_timestep": 1e-15},
        ],
    )
    def test_invalid_timestep_ordering_raises(self, kwargs):
        with pytest.raises(ValueError, match="min_timestep"):
            rspice.SimulationConfig(**kwargs)

        config = rspice.SimulationConfig()
        if "min_timestep" in kwargs and "max_timestep" in kwargs:
            config.max_timestep = kwargs["max_timestep"]
            with pytest.raises(ValueError, match="min_timestep|max_timestep"):
                config.min_timestep = kwargs["min_timestep"]
        elif "min_timestep" in kwargs:
            with pytest.raises(ValueError, match="min_timestep|max_timestep"):
                config.min_timestep = kwargs["min_timestep"]
        else:
            with pytest.raises(ValueError, match="min_timestep|max_timestep"):
                config.max_timestep = kwargs["max_timestep"]

    def test_unbounded_default_accepts_large_preferred_minimum(self):
        config = rspice.SimulationConfig(min_timestep=1.0)
        assert config.min_timestep == 1.0
        assert math.isinf(config.max_timestep)

        config = rspice.SimulationConfig()
        config.min_timestep = 1.0
        assert config.min_timestep == 1.0
        assert math.isinf(config.max_timestep)

    def test_nested_assignment_works(self):
        config = rspice.SimulationConfig()
        conv = config.convergence
        conv.gmin_stepping = False
        config.convergence = conv
        assert config.convergence.gmin_stepping is False

    def test_getter_returns_copy_documented_behavior(self):
        # This is the documented copy semantics: mutating the returned
        # object does NOT write through. Keyword construction or whole-object
        # assignment is the supported path.
        config = rspice.SimulationConfig()
        original = config.convergence.gmin_stepping
        config.convergence.gmin_stepping = not original
        assert config.convergence.gmin_stepping is original

    def test_engine_uses_config(self):
        engine = rspice.Engine(rspice.SimulationConfig(tolerance=1e-12))
        assert engine.config.tolerance == 1e-12

    def test_resource_policy_roundtrips_with_copy_semantics(self):
        limits = rspice.ResourceLimits(max_batch_runs=7, max_analysis_points=1234)
        config = rspice.SimulationConfig(resource_limits=limits)
        assert config.resource_limits.max_batch_runs == 7
        assert config.resource_limits.max_analysis_points == 1234

        config.resource_limits.max_batch_runs = 99
        assert config.resource_limits.max_batch_runs == 7

        updated = config.resource_limits
        updated.max_batch_runs = 11
        config.resource_limits = updated
        assert config.resource_limits.max_batch_runs == 11


class TestResourceLimits:
    def test_defaults_kwargs_and_properties(self):
        limits = rspice.ResourceLimits(
            max_netlist_bytes=1024, max_parallel_workers=4, max_batch_runs=3
        )
        assert limits.max_netlist_bytes == 1024
        assert limits.max_parallel_workers == 4
        assert limits.max_batch_runs == 3
        assert limits.max_analysis_points > 0

        limits.max_result_values = 77
        assert limits.max_result_values == 77
        assert repr(limits).startswith("ResourceLimits(")

    def test_zero_can_disable_a_resource_and_unlimited_is_explicit(self):
        disabled = rspice.ResourceLimits(max_shared_cache_bytes=0)
        assert disabled.max_shared_cache_bytes == 0

        unlimited = rspice.ResourceLimits.unlimited()
        assert unlimited.max_netlist_bytes > rspice.ResourceLimits().max_netlist_bytes
        assert unlimited.max_batch_runs > rspice.ResourceLimits().max_batch_runs

    def test_engine_policy_rechecks_a_netlist_parsed_under_looser_defaults(self):
        source = "strict engine\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n"
        netlist = rspice.Netlist.parse_spice(source)
        engine = rspice.Engine(
            rspice.SimulationConfig(
                resource_limits=rspice.ResourceLimits(
                    max_netlist_bytes=len(source.encode()) - 1
                )
            )
        )

        with pytest.raises(rspice.SimulationError) as exc_info:
            engine.run_dc_op(netlist)

        error = exc_info.value
        assert error.kind == "resource_limit"
        assert error.resource == "netlist_bytes"
        assert error.requested == len(source.encode())
        assert error.limit == len(source.encode()) - 1


class TestConvergenceConfig:
    def test_gmin_range_must_progress_toward_target(self):
        with pytest.raises(ValueError, match="gmin_initial"):
            rspice.ConvergenceConfig(gmin_initial=1e-15, gmin_target=1e-12)

        config = rspice.ConvergenceConfig(gmin_initial=1e-6, gmin_target=1e-12)
        with pytest.raises(ValueError, match="gmin_initial"):
            config.gmin_initial = 1e-15
        assert config.gmin_initial == 1e-6
        with pytest.raises(ValueError, match="gmin_target"):
            config.gmin_target = 1e-3
        assert config.gmin_target == 1e-12

    def test_kwargs_constructor(self):
        conv = rspice.ConvergenceConfig(
            gmin_stepping=False,
            source_stepping=True,
            damping_strategy=rspice.DampingStrategy.COMBINED,
            voltage_reltol=1e-4,
            charge_abstol=1e-15,
            verbose=False,
        )
        assert conv.gmin_stepping is False
        assert conv.source_stepping is True
        assert conv.damping_strategy == rspice.DampingStrategy.COMBINED
        assert conv.voltage_reltol == 1e-4
        assert conv.charge_abstol == 1e-15

    def test_presets(self):
        fast = rspice.ConvergenceConfig.fast()
        robust = rspice.ConvergenceConfig.robust()
        assert robust.gmin_stepping is True
        assert repr(fast).startswith("ConvergenceConfig(")

    def test_all_properties_roundtrip(self):
        conv = rspice.ConvergenceConfig()
        conv.gmin_initial = 1e-10
        conv.gmin_target = 1e-14
        conv.residual_reltol = 1e-5
        conv.voltage_abstol = 1e-7
        conv.current_abstol = 1e-11
        conv.pseudo_transient = True
        conv.arc_length = True
        assert conv.gmin_initial == 1e-10
        assert conv.gmin_target == 1e-14
        assert conv.residual_reltol == 1e-5
        assert conv.voltage_abstol == 1e-7
        assert conv.current_abstol == 1e-11
        assert conv.pseudo_transient is True
        assert conv.arc_length is True

    @pytest.mark.parametrize(
        ("kwargs", "field", "bad_value"),
        [
            ({"gmin_initial": -1e-12}, "gmin_initial", -1e-12),
            ({"gmin_target": math.nan}, "gmin_target", math.nan),
            ({"voltage_reltol": -1e-3}, "voltage_reltol", -1e-3),
            ({"residual_reltol": math.inf}, "residual_reltol", math.inf),
            ({"voltage_abstol": -1e-9}, "voltage_abstol", -1e-9),
            ({"current_abstol": math.nan}, "current_abstol", math.nan),
            ({"charge_abstol": -1e-15}, "charge_abstol", -1e-15),
        ],
    )
    def test_invalid_numeric_fields_raise(self, kwargs, field, bad_value):
        with pytest.raises(ValueError):
            rspice.ConvergenceConfig(**kwargs)

        conv = rspice.ConvergenceConfig()
        with pytest.raises(ValueError):
            setattr(conv, field, bad_value)


class TestBypassConfig:
    def test_enabled_is_a_real_property(self):
        bypass = rspice.BypassConfig()
        assert bypass.enabled is False
        bypass.enabled = True
        assert bypass.enabled is True

    def test_kwargs_constructor(self):
        bypass = rspice.BypassConfig(enabled=True, reltol=1e-4, abstol=1e-7)
        assert bypass.enabled is True
        assert bypass.reltol == 1e-4
        assert bypass.abstol == 1e-7

    def test_with_tolerances(self):
        bypass = rspice.BypassConfig.with_tolerances(1e-3, 1e-6)
        assert bypass.reltol == 1e-3
        assert bypass.abstol == 1e-6

    @pytest.mark.parametrize(
        ("kwargs", "field", "bad_value"),
        [
            ({"reltol": -1e-3}, "reltol", -1e-3),
            ({"abstol": math.nan}, "abstol", math.nan),
        ],
    )
    def test_invalid_numeric_fields_raise(self, kwargs, field, bad_value):
        with pytest.raises(ValueError):
            rspice.BypassConfig(**kwargs)

        bypass = rspice.BypassConfig()
        with pytest.raises(ValueError):
            setattr(bypass, field, bad_value)

    def test_with_tolerances_rejects_invalid_values(self):
        with pytest.raises(ValueError):
            rspice.BypassConfig.with_tolerances(math.inf, 1e-6)


class TestEnums:
    def test_damping_strategy_values(self):
        assert repr(rspice.DampingStrategy.COMBINED) == "DampingStrategy.COMBINED"
        assert rspice.DampingStrategy.NONE != rspice.DampingStrategy.COMBINED

    def test_integration_method_values(self):
        methods = [
            rspice.IntegrationMethod.BACKWARD_EULER,
            rspice.IntegrationMethod.TRAPEZOIDAL,
            rspice.IntegrationMethod.GEAR2,
            rspice.IntegrationMethod.TRAP_GEAR,
        ]
        assert len(set(map(repr, methods))) == 4


class TestModuleSurface:
    def test_version_attributes(self):
        assert isinstance(rspice.__version__, str)
        assert rspice.__version__.count(".") >= 1
        assert isinstance(rspice.__author__, str)

    def test_all_exports_exist(self):
        assert len(rspice.__all__) == len(set(rspice.__all__))
        for name in rspice.__all__:
            assert hasattr(rspice, name), f"__all__ lists missing attribute {name}"

    def test_runtime_exports_match_installed_type_stub(self):
        stub = Path(rspice.__file__).with_name("__init__.pyi")
        assert stub.is_file(), "installed package is missing __init__.pyi"
        module = ast.parse(stub.read_text(encoding="utf-8"), filename=str(stub))
        assignment = next(
            node
            for node in module.body
            if isinstance(node, ast.Assign)
            and any(isinstance(target, ast.Name) and target.id == "__all__" for target in node.targets)
        )
        stub_exports = ast.literal_eval(assignment.value)
        assert stub_exports == rspice.__all__

    def test_exception_hierarchy(self):
        assert issubclass(rspice.ParseError, rspice.RSpiceError)
        assert issubclass(rspice.SimulationError, rspice.RSpiceError)
        assert issubclass(rspice.ConvergenceError, rspice.SimulationError)
        assert issubclass(rspice.CancelledError, rspice.SimulationError)
        assert issubclass(rspice.MeasurementError, rspice.RSpiceError)
        assert issubclass(rspice.RSpiceError, Exception)
