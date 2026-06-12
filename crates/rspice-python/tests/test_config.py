"""Configuration classes: kwargs construction, copy semantics, enums."""

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


class TestConvergenceConfig:
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
        for name in rspice.__all__:
            assert hasattr(rspice, name), f"__all__ lists missing attribute {name}"

    def test_exception_hierarchy(self):
        assert issubclass(rspice.ParseError, rspice.RSpiceError)
        assert issubclass(rspice.SimulationError, rspice.RSpiceError)
        assert issubclass(rspice.ConvergenceError, rspice.SimulationError)
        assert issubclass(rspice.MeasurementError, rspice.RSpiceError)
        assert issubclass(rspice.RSpiceError, Exception)
