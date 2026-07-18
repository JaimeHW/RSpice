"""Type stubs for the rspice extension module."""

import os
from typing import Iterator, Literal, Sequence, final

import numpy as np
import numpy.typing as npt

__version__: str
__author__: str
__all__ = [
    "__version__",
    "__author__",
    "Netlist",
    "ParseDiagnostic",
    "NetlistSourceLocation",
    "StartupDirectiveScope",
    "StartupDiagnostic",
    "UnresolvedOutputSymbol",
    "HealthReport",
    "Engine",
    "SimulationConfig",
    "ResourceLimits",
    "ConvergenceConfig",
    "BypassConfig",
    "DampingStrategy",
    "IntegrationMethod",
    "SimulationResult",
    "DeviceOperatingPoint",
    "TransientResult",
    "CompressedTransientResult",
    "TransientCheckpoint",
    "AcResult",
    "DistortionResult",
    "DcSweepResult",
    "NoiseResult",
    "NoiseContribution",
    "MonteCarloResult",
    "AcSensitivity",
    "AcSensitivityResult",
    "ElementSensitivity",
    "SensitivityResult",
    "VariableStatistics",
    "PoleZeroResult",
    "ComplexValue",
    "FourierResult",
    "Harmonic",
    "TransferFunctionResult",
    "StbResult",
    "SParameterResult",
    "PssResult",
    "HbResult",
    "PacResult",
    "PeriodicNoiseContribution",
    "PeriodicNoiseResult",
    "OscillatorNoiseResult",
    "Measurement",
    "AnalysisRecord",
    "RunReport",
    "ac_frequencies",
    "RSpiceError",
    "ParseError",
    "SimulationError",
    "ConvergenceError",
    "CancelledError",
    "MeasurementError",
]

class RSpiceError(Exception):
    """Base exception for all RSpice errors."""

class ParseError(RSpiceError):
    """Raised when netlist parsing fails due to syntax or semantic errors."""

    kind: Literal[
        "syntax",
        "unknown_device",
        "invalid_node",
        "duplicate_name",
        "missing_subcircuit_ends",
        "duplicate_subcircuit_port_binding",
        "global_subcircuit_port_binding",
        "undefined_mutual_inductor_reference",
        "undefined_output_symbols",
        "conflicting_startup_directives",
        "device_initial_condition_duplicate_directive",
        "device_initial_condition_missing_information",
        "device_initial_condition_malformed_directive",
        "device_initial_condition_source_unavailable",
        "device_initial_condition_malformed_source",
        "device_initial_condition_nonfinite_value",
        "device_initial_condition_unresolved_source",
        "device_initial_condition_invalid_arity",
        "device_initial_condition_unsupported_target",
        "missing_parameter",
        "undefined_parameter",
        "invalid_value",
        "resource_limit",
        "io",
    ]
    category: Literal[
        "device_initial_condition",
        "subcircuit_binding",
        "mutual_inductor_reference",
        "output_symbol_validation",
        "resource_limit",
        "startup_directive_validation",
    ] | None
    line: int | None
    detail: str | None
    source: str | None
    primary_line: int | None
    primary_source: str | None
    related_line: int | None
    related_source: str | None
    detected_line: int | None
    detected_source: str | None
    boundary: Literal["end_card", "alter_card", "end_of_source"] | None
    authored_name: str | None
    canonical_name: str | None
    qualified_name: str | None
    subcircuit_name: str | None
    canonical_subcircuit_name: str | None
    instance_name: str | None
    canonical_instance_name: str | None
    qualified_instance_name: str | None
    formal_port: str | None
    first_position: int | None
    conflicting_position: int | None
    first_actual_node: str | None
    conflicting_actual_node: str | None
    position: int | None
    actual_node: str | None
    authored_coupling_name: str | None
    canonical_coupling_name: str | None
    qualified_coupling_name: str | None
    authored_inductor_name: str | None
    canonical_inductor_name: str | None
    qualified_inductor_name: str | None
    scope_name: str | None
    reference_position: int | None
    device: str | None
    requested_path: str | None
    value_index: int | None
    value: float | None
    expected: str | None
    actual: int | None
    device_type: str | None
    unresolved_output_symbols: list[UnresolvedOutputSymbol] | None
    first_startup_kind: Literal["ic", "nodeset"] | None
    conflicting_startup_kind: Literal["ic", "nodeset"] | None
    resource: str | None
    requested: int | None
    limit: int | None

class SimulationError(RSpiceError):
    """Raised when simulation fails due to circuit or solver errors."""

    kind: str
    iterations: int | None
    resource: str | None
    requested: int | None
    limit: int | None

class ConvergenceError(SimulationError):
    """Raised when Newton-Raphson iteration fails to converge."""

class CancelledError(SimulationError):
    """Raised in a simulation's calling thread after Engine.cancel()."""

class MeasurementError(RSpiceError):
    """Raised when .MEAS verification fails (see RunReport.assert_passed)."""

def ac_frequencies(
    variation: str, points: int, start_freq: float, stop_freq: float
) -> npt.NDArray[np.float64]: ...

@final
class ParseDiagnostic:
    @property
    def line(self) -> int: ...
    @property
    def severity(self) -> str: ...
    @property
    def code(self) -> str: ...
    @property
    def message(self) -> str: ...

@final
class NetlistSourceLocation:
    @property
    def line(self) -> int: ...
    @property
    def source(self) -> str | None: ...

@final
class StartupDirectiveScope:
    @property
    def kind(self) -> Literal["top_level", "subcircuit"]: ...
    @property
    def qualified_definition(self) -> str | None: ...
    @property
    def qualified_instances(self) -> list[str]: ...

@final
class StartupDiagnostic:
    @property
    def code(
        self,
    ) -> Literal[
        "startup-empty-directive",
        "startup-undefined-node",
        "startup-scoped-global-node",
    ]: ...
    @property
    def stage(self) -> Literal["parse", "startup_topology"]: ...
    @property
    def directive(self) -> Literal["ic", "nodeset"]: ...
    @property
    def origins(self) -> list[NetlistSourceLocation]: ...
    @property
    def scopes(self) -> list[StartupDirectiveScope]: ...
    @property
    def canonical_nodes(self) -> list[str]: ...

@final
class UnresolvedOutputSymbol:
    @property
    def directive(self) -> Literal["save", "probe", "print", "plot", "measure", "four"]: ...
    @property
    def operator(self) -> str: ...
    @property
    def symbol(self) -> str: ...
    @property
    def kind(self) -> Literal["node", "device"]: ...
    @property
    def line(self) -> int: ...
    @property
    def source(self) -> str | None: ...

@final
class Netlist:
    @staticmethod
    def parse(content: str, *, resource_limits: ResourceLimits | None = None) -> Netlist: ...
    @staticmethod
    def parse_spice(
        content: str, *, resource_limits: ResourceLimits | None = None
    ) -> Netlist: ...
    @staticmethod
    def parse_file(
        path: str | os.PathLike[str], *, resource_limits: ResourceLimits | None = None
    ) -> Netlist: ...
    @staticmethod
    def parse_with_includes(
        content: str,
        base_path: str | os.PathLike[str],
        *,
        resource_limits: ResourceLimits | None = None,
    ) -> Netlist: ...
    @property
    def num_elements(self) -> int: ...
    @property
    def num_models(self) -> int: ...
    @property
    def num_subcircuits(self) -> int: ...
    @property
    def num_analyses(self) -> int: ...
    @property
    def num_measurements(self) -> int: ...
    @property
    def title(self) -> str: ...
    @property
    def diagnostics(self) -> list[ParseDiagnostic]: ...
    @property
    def startup_diagnostics(self) -> list[StartupDiagnostic]: ...
    @property
    def element_names(self) -> list[str]: ...
    @property
    def model_names(self) -> list[str]: ...
    @property
    def subcircuit_names(self) -> list[str]: ...
    @property
    def measurement_names(self) -> list[str]: ...
    @property
    def measurement_specs(
        self,
    ) -> list[tuple[str, str, float | None, float | None]]: ...
    @property
    def analyses(self) -> list[str]: ...
    def is_global(self, node: str) -> bool: ...

@final
class DampingStrategy:
    NONE: DampingStrategy
    LINE_SEARCH: DampingStrategy
    VOLTAGE_LIMITING: DampingStrategy
    BANK_ROSE: DampingStrategy
    COMBINED: DampingStrategy

@final
class IntegrationMethod:
    BACKWARD_EULER: IntegrationMethod
    TRAPEZOIDAL: IntegrationMethod
    GEAR2: IntegrationMethod
    TRAP_GEAR: IntegrationMethod

@final
class BypassConfig:
    def __new__(
        cls,
        *,
        enabled: bool | None = None,
        reltol: float | None = None,
        abstol: float | None = None,
    ) -> BypassConfig: ...
    @staticmethod
    def with_tolerances(reltol: float, abstol: float) -> BypassConfig: ...
    enabled: bool
    reltol: float
    abstol: float

@final
class ConvergenceConfig:
    def __new__(
        cls,
        *,
        gmin_stepping: bool | None = None,
        source_stepping: bool | None = None,
        pseudo_transient: bool | None = None,
        arc_length: bool | None = None,
        damping_strategy: DampingStrategy | None = None,
        gmin_initial: float | None = None,
        gmin_target: float | None = None,
        voltage_reltol: float | None = None,
        residual_reltol: float | None = None,
        voltage_abstol: float | None = None,
        current_abstol: float | None = None,
        charge_abstol: float | None = None,
        verbose: bool | None = None,
    ) -> ConvergenceConfig: ...
    @staticmethod
    def fast() -> ConvergenceConfig: ...
    @staticmethod
    def robust() -> ConvergenceConfig: ...
    gmin_stepping: bool
    source_stepping: bool
    pseudo_transient: bool
    arc_length: bool
    damping_strategy: DampingStrategy
    gmin_initial: float
    gmin_target: float
    voltage_reltol: float
    residual_reltol: float
    voltage_abstol: float
    current_abstol: float
    charge_abstol: float
    verbose: bool

@final
class ResourceLimits:
    def __new__(
        cls,
        *,
        max_netlist_bytes: int | None = None,
        max_netlist_lines: int | None = None,
        max_expanded_source_bytes: int | None = None,
        max_dependency_source_bytes: int | None = None,
        max_external_data_bytes: int | None = None,
        max_external_data_values: int | None = None,
        max_shared_cache_bytes: int | None = None,
        max_include_depth: int | None = None,
        max_hierarchy_depth: int | None = None,
        max_flattened_elements: int | None = None,
        max_circuit_nodes: int | None = None,
        max_matrix_unknowns: int | None = None,
        max_analysis_points: int | None = None,
        max_result_values: int | None = None,
        max_batch_runs: int | None = None,
    ) -> ResourceLimits: ...
    @staticmethod
    def unlimited() -> ResourceLimits: ...
    max_netlist_bytes: int
    max_netlist_lines: int
    max_expanded_source_bytes: int
    max_dependency_source_bytes: int
    max_external_data_bytes: int
    max_external_data_values: int
    max_shared_cache_bytes: int
    max_include_depth: int
    max_hierarchy_depth: int
    max_flattened_elements: int
    max_circuit_nodes: int
    max_matrix_unknowns: int
    max_analysis_points: int
    max_result_values: int
    max_batch_runs: int

@final
class SimulationConfig:
    def __new__(
        cls,
        *,
        tolerance: float | None = None,
        max_iterations: int | None = None,
        transient_max_iterations: int | None = None,
        min_timestep: float | None = None,
        max_timestep: float | None = None,
        temperature: float | None = None,
        integration_method: IntegrationMethod | None = None,
        transient_trtol: float | None = None,
        convergence: ConvergenceConfig | None = None,
        bypass: BypassConfig | None = None,
        resource_limits: ResourceLimits | None = None,
    ) -> SimulationConfig: ...
    tolerance: float
    max_iterations: int
    transient_max_iterations: int
    min_timestep: float
    max_timestep: float
    temperature: float
    integration_method: IntegrationMethod
    transient_trtol: float
    convergence: ConvergenceConfig
    bypass: BypassConfig
    resource_limits: ResourceLimits

@final
class DeviceOperatingPoint:
    @property
    def name(self) -> str: ...
    @property
    def device_kind(self) -> str: ...
    @property
    def region(self) -> str | None: ...
    @property
    def params(self) -> dict[str, float]: ...
    @property
    def param_names(self) -> list[str]: ...
    def param(self, name: str) -> float: ...
    def __getitem__(self, name: str, /) -> float: ...

@final
class SimulationResult:
    def voltage(self, node: int | str) -> float: ...
    @property
    def node_voltages(self) -> npt.NDArray[np.float64]: ...
    @property
    def node_names(self) -> list[str]: ...
    def branch_current(self, name: str) -> float: ...
    @property
    def branch_currents(self) -> npt.NDArray[np.float64]: ...
    @property
    def branch_names(self) -> list[str]: ...
    @property
    def num_nodes(self) -> int: ...
    @property
    def device_operating_points(self) -> list[DeviceOperatingPoint]: ...
    def device_operating_point(self, name: str) -> DeviceOperatingPoint: ...

@final
class Harmonic:
    @property
    def n(self) -> int: ...
    @property
    def frequency(self) -> float: ...
    @property
    def magnitude(self) -> float: ...
    @property
    def phase(self) -> float: ...
    @property
    def phase_degrees(self) -> float: ...

@final
class FourierResult:
    @property
    def dc_component(self) -> float: ...
    @property
    def thd(self) -> float: ...
    @property
    def thd_percent(self) -> float: ...
    @property
    def harmonics(self) -> list[Harmonic]: ...
    @property
    def magnitudes(self) -> npt.NDArray[np.float64]: ...
    @property
    def fundamental_magnitude(self) -> float | None: ...

@final
class CompressedTransientResult:
    @property
    def time(self) -> npt.NDArray[np.float64]: ...
    @property
    def node_names(self) -> list[str]: ...
    @property
    def num_nodes(self) -> int: ...
    @property
    def num_points(self) -> int: ...
    @property
    def input_points(self) -> int: ...
    @property
    def compression_ratio(self) -> float: ...
    def voltage_waveform(self, node: int | str) -> npt.NDArray[np.float64]: ...
    def voltage_at(self, node: int | str, time: float) -> float: ...
    def resample(
        self, node: int | str, num_points: int
    ) -> tuple[npt.NDArray[np.float64], npt.NDArray[np.float64]]: ...

@final
class TransientCheckpoint:
    @staticmethod
    def load(path: str | os.PathLike[str]) -> TransientCheckpoint: ...
    def save(self, path: str | os.PathLike[str]) -> None: ...
    @property
    def time(self) -> float: ...
    @property
    def solution(self) -> npt.NDArray[np.float64]: ...
    @property
    def netlist_fingerprint(self) -> int: ...

@final
class TransientResult:
    @property
    def time(self) -> npt.NDArray[np.float64]: ...
    def voltage_waveform(self, node: int | str) -> npt.NDArray[np.float64]: ...
    def branch_current_waveform(self, name: str) -> npt.NDArray[np.float64]: ...
    @property
    def device_parameter_names(self) -> list[str]: ...
    def device_parameter_waveform(
        self, device: str, parameter: str
    ) -> npt.NDArray[np.float64]: ...
    def voltage_at(self, node: int, time_index: int) -> float: ...
    def fourier(
        self, node: int | str, fundamental: float, num_harmonics: int = 9
    ) -> FourierResult: ...
    @property
    def num_points(self) -> int: ...
    @property
    def num_nodes(self) -> int: ...
    @property
    def node_names(self) -> list[str]: ...
    @property
    def branch_names(self) -> list[str]: ...
    @property
    def stop_time(self) -> float: ...

@final
class AcResult:
    @property
    def frequencies(self) -> npt.NDArray[np.float64]: ...
    @property
    def num_frequencies(self) -> int: ...
    @property
    def node_names(self) -> list[str]: ...
    @property
    def branch_names(self) -> list[str]: ...
    def voltage_magnitude(self, node: int | str) -> npt.NDArray[np.float64]: ...
    def voltage_phase(self, node: int | str) -> npt.NDArray[np.float64]: ...
    def voltage_phase_degrees(self, node: int | str) -> npt.NDArray[np.float64]: ...
    def voltage_complex(self, node: int | str) -> npt.NDArray[np.complex128]: ...
    def voltage_db(self, node: int | str) -> npt.NDArray[np.float64]: ...
    def branch_current_complex(self, name: str) -> npt.NDArray[np.complex128]: ...
    def branch_current_magnitude(self, name: str) -> npt.NDArray[np.float64]: ...
    def magnitude_at(self, freq_index: int, node: int | str) -> float: ...
    def phase_at(self, freq_index: int, node: int | str) -> float: ...

@final
class DistortionResult:
    """Third-order harmonic or two-tone Volterra sweep."""

    @property
    def f1_frequencies(self) -> npt.NDArray[np.float64]: ...
    @property
    def f2_frequency(self) -> float | None: ...
    @property
    def f2_over_f1(self) -> float | None: ...
    @property
    def is_two_tone(self) -> bool: ...
    @property
    def num_points(self) -> int: ...
    @property
    def node_names(self) -> list[str]: ...
    @property
    def branch_names(self) -> list[str]: ...
    @property
    def available_products(self) -> list[str]: ...
    @property
    def fundamental_f1(self) -> AcResult: ...
    @property
    def fundamental_f2(self) -> AcResult | None: ...
    def product(self, name: str) -> AcResult: ...
    def voltage_ratio(
        self, name: str, node: int | str
    ) -> npt.NDArray[np.float64]: ...
    def voltage_db_relative(
        self, name: str, node: int | str
    ) -> npt.NDArray[np.float64]: ...
    def branch_current_ratio(
        self, name: str, branch: str
    ) -> npt.NDArray[np.float64]: ...
    def branch_current_db_relative(
        self, name: str, branch: str
    ) -> npt.NDArray[np.float64]: ...

@final
class DcSweepResult:
    @property
    def primary_source(self) -> str | None: ...
    @property
    def secondary_source(self) -> str | None: ...
    @property
    def secondary_sweep_values(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def shape(self) -> tuple[int, int]: ...
    @property
    def is_nested(self) -> bool: ...
    @property
    def sweep_values(self) -> npt.NDArray[np.float64]: ...
    def __len__(self) -> int: ...
    def __iter__(self) -> Iterator[tuple[float, SimulationResult]]: ...
    def __getitem__(self, index: int, /) -> tuple[float, SimulationResult]: ...
    def points(self) -> list[tuple[float, SimulationResult]]: ...
    def result_at(self, index: int) -> SimulationResult: ...
    def sweep_value_at(self, index: int) -> float: ...
    def secondary_value_at(self, index: int) -> float | None: ...
    def device_operating_points_at(self, index: int) -> list[DeviceOperatingPoint]: ...
    def voltage(self, index: int, node: int | str) -> float: ...
    def voltage_array(self, node: int | str) -> npt.NDArray[np.float64]: ...

@final
class NoiseContribution:
    @property
    def device_name(self) -> str: ...
    @property
    def noise_type(self) -> str: ...
    @property
    def output_contribution(self) -> float: ...
    @property
    def percentage(self) -> float: ...

@final
class NoiseResult:
    @property
    def frequency(self) -> float: ...
    @property
    def output_noise_density(self) -> float: ...
    @property
    def input_referred_density(self) -> float: ...
    @property
    def output_noise_rms(self) -> float: ...
    @property
    def input_referred_rms(self) -> float: ...
    @property
    def output_noise_dbv(self) -> float: ...
    @property
    def contributions(self) -> list[NoiseContribution]: ...
    def dominant_source(self) -> NoiseContribution | None: ...

@final
class VariableStatistics:
    @property
    def name(self) -> str: ...
    @property
    def mean(self) -> float: ...
    @property
    def std_dev(self) -> float: ...
    @property
    def min(self) -> float: ...
    @property
    def max(self) -> float: ...
    @property
    def samples(self) -> npt.NDArray[np.float64]: ...
    @property
    def histogram(self) -> list[int]: ...
    @property
    def bin_edges(self) -> npt.NDArray[np.float64]: ...
    def percentile(self, pct: float) -> float: ...
    @property
    def three_sigma_range(self) -> tuple[float, float]: ...
    @property
    def cv_percent(self) -> float | None: ...

@final
class MonteCarloResult:
    @property
    def num_runs(self) -> int: ...
    @property
    def all_converged(self) -> bool: ...
    @property
    def num_failures(self) -> int: ...
    def get_variable(self, name: str) -> VariableStatistics | None: ...
    @property
    def variable_names(self) -> list[str]: ...
    def mean(self, name: str) -> float | None: ...
    def std_dev(self, name: str) -> float | None: ...
    def range(self, name: str) -> tuple[float, float] | None: ...
    @property
    def success_rate(self) -> float: ...

@final
class AcSensitivity:
    @property
    def vector_name(self) -> str: ...
    @property
    def element(self) -> str: ...
    @property
    def element_type(self) -> str: ...
    @property
    def parameter(self) -> str: ...
    @property
    def nominal_value(self) -> float: ...
    @property
    def absolute(self) -> npt.NDArray[np.complex128]: ...
    @property
    def normalized(self) -> npt.NDArray[np.complex128]: ...
    @property
    def magnitude(self) -> npt.NDArray[np.float64]: ...
    @property
    def phase(self) -> npt.NDArray[np.float64]: ...
    @property
    def phase_degrees(self) -> npt.NDArray[np.float64]: ...
    @property
    def db(self) -> npt.NDArray[np.float64]: ...
    @property
    def percent_per_percent(self) -> npt.NDArray[np.complex128]: ...
    def __len__(self) -> int: ...

@final
class AcSensitivityResult:
    @property
    def output(self) -> str: ...
    @property
    def frequencies(self) -> npt.NDArray[np.float64]: ...
    @property
    def output_complex(self) -> npt.NDArray[np.complex128]: ...
    @property
    def output_magnitude(self) -> npt.NDArray[np.float64]: ...
    @property
    def output_phase(self) -> npt.NDArray[np.float64]: ...
    @property
    def output_phase_degrees(self) -> npt.NDArray[np.float64]: ...
    @property
    def sensitivities(self) -> list[AcSensitivity]: ...
    @property
    def vector_names(self) -> list[str]: ...
    def get(self, vector_name: str) -> AcSensitivity: ...
    def top(self, frequency_index: int, count: int = 10) -> list[AcSensitivity]: ...
    def __len__(self) -> int: ...

@final
class ElementSensitivity:
    @property
    def vector_name(self) -> str: ...
    @property
    def element(self) -> str: ...
    @property
    def element_type(self) -> str: ...
    @property
    def parameter(self) -> str: ...
    @property
    def nominal_value(self) -> float: ...
    @property
    def absolute(self) -> float: ...
    @property
    def normalized(self) -> float: ...
    @property
    def percent_per_percent(self) -> float: ...

@final
class SensitivityResult:
    @property
    def output(self) -> str: ...
    @property
    def output_value(self) -> float: ...
    @property
    def sensitivities(self) -> list[ElementSensitivity]: ...
    @property
    def vector_names(self) -> list[str]: ...
    def __len__(self) -> int: ...
    def get(
        self, element: str, parameter: str | None = None
    ) -> ElementSensitivity: ...
    def top(self, count: int = 10) -> list[ElementSensitivity]: ...

@final
class ComplexValue:
    @property
    def real(self) -> float: ...
    @property
    def imag(self) -> float: ...
    def __complex__(self) -> complex: ...
    @property
    def magnitude(self) -> float: ...
    @property
    def phase(self) -> float: ...
    @property
    def phase_degrees(self) -> float: ...
    @property
    def is_real(self) -> bool: ...
    @property
    def frequency_hz(self) -> float: ...
    @property
    def damping_factor(self) -> float: ...
    @property
    def time_constant(self) -> float | None: ...

@final
class PoleZeroResult:
    @property
    def poles(self) -> list[ComplexValue]: ...
    @property
    def zeros(self) -> list[ComplexValue]: ...
    @property
    def poles_array(self) -> npt.NDArray[np.complex128]: ...
    @property
    def zeros_array(self) -> npt.NDArray[np.complex128]: ...
    def real_poles(self) -> list[ComplexValue]: ...
    def complex_poles(self) -> list[ComplexValue]: ...
    @property
    def is_stable(self) -> bool: ...
    def dominant_pole(self) -> ComplexValue | None: ...
    @property
    def bandwidth_hz(self) -> float | None: ...
    @property
    def dominant_pole_decay_hz(self) -> float | None: ...
    @property
    def num_poles(self) -> int: ...
    @property
    def num_zeros(self) -> int: ...
    @property
    def dc_gain(self) -> float: ...
    @property
    def hf_gain(self) -> float | None: ...
    @property
    def input(self) -> str: ...
    @property
    def output(self) -> str: ...

@final
class StbResult:
    @property
    def frequencies(self) -> npt.NDArray[np.float64]: ...
    @property
    def loop_gain(self) -> npt.NDArray[np.complex128]: ...
    @property
    def magnitude(self) -> npt.NDArray[np.float64]: ...
    @property
    def magnitude_db(self) -> npt.NDArray[np.float64]: ...
    @property
    def phase_degrees(self) -> npt.NDArray[np.float64]: ...
    @property
    def probe_name(self) -> str: ...
    @property
    def gain_margin_db(self) -> float: ...
    @property
    def gain_margin_frequency(self) -> float: ...
    @property
    def phase_margin_degrees(self) -> float: ...
    @property
    def phase_margin_frequency(self) -> float: ...
    @property
    def dc_gain_db(self) -> float: ...
    @property
    def unity_gain_bandwidth(self) -> float: ...
    @property
    def conditionally_stable(self) -> bool: ...
    @property
    def num_crossovers(self) -> int: ...
    @property
    def success(self) -> bool: ...
    @property
    def warnings(self) -> list[str]: ...
    @property
    def is_stable(self) -> bool: ...
    @property
    def assessment(self) -> str: ...

@final
class SParameterResult:
    @property
    def frequencies(self) -> npt.NDArray[np.float64]: ...
    @property
    def port_names(self) -> list[str]: ...
    @property
    def reference_impedances(self) -> npt.NDArray[np.float64]: ...
    @property
    def num_ports(self) -> int: ...
    @property
    def num_points(self) -> int: ...
    @property
    def has_noise(self) -> bool: ...
    @property
    def noise_temperature(self) -> float | None: ...
    @property
    def has_two_port_noise_parameters(self) -> bool: ...
    def s(self, output_port: int, input_port: int) -> npt.NDArray[np.complex128]: ...
    def magnitude(
        self, output_port: int, input_port: int
    ) -> npt.NDArray[np.float64]: ...
    def magnitude_db(
        self, output_port: int, input_port: int
    ) -> npt.NDArray[np.float64]: ...
    def phase_degrees(
        self, output_port: int, input_port: int
    ) -> npt.NDArray[np.float64]: ...
    def cy(
        self, output_port: int, input_port: int
    ) -> npt.NDArray[np.complex128]: ...
    @property
    def noise_resistance(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def noise_factor(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def noise_figure_db(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def minimum_noise_factor(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def minimum_noise_figure_db(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def optimum_source_reflection(self) -> npt.NDArray[np.complex128] | None: ...
    @property
    def noise_parameters_valid(self) -> npt.NDArray[np.bool_] | None: ...
    @property
    def rn(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def nf(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def nfmin(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def sopt(self) -> npt.NDArray[np.complex128] | None: ...

@final
class PssResult:
    @property
    def time(self) -> npt.NDArray[np.float64]: ...
    @property
    def frequency(self) -> float: ...
    @property
    def period(self) -> float: ...
    @property
    def iterations(self) -> int: ...
    @property
    def residual_norm(self) -> float: ...
    @property
    def is_stable(self) -> bool: ...
    @property
    def period_detected(self) -> bool: ...
    @property
    def floquet_multipliers(self) -> npt.NDArray[np.complex128]: ...
    @property
    def node_names(self) -> list[str]: ...
    @property
    def num_nodes(self) -> int: ...
    @property
    def num_points(self) -> int: ...
    @property
    def num_harmonics(self) -> int: ...
    @property
    def harmonic_frequencies(self) -> npt.NDArray[np.float64]: ...
    def voltage_waveform(self, node: int | str) -> npt.NDArray[np.float64]: ...
    def voltage_at(self, node: int | str, time: float) -> float: ...
    def dc(self, node: int | str) -> float: ...
    def peak_to_peak(self, node: int | str) -> float: ...
    def harmonic_coefficients(
        self, node: int | str
    ) -> npt.NDArray[np.complex128]: ...
    def harmonic_magnitude(self, node: int | str) -> npt.NDArray[np.float64]: ...
    def harmonic_phase_degrees(
        self, node: int | str
    ) -> npt.NDArray[np.float64]: ...
    def harmonics(self, node: int | str) -> list[Harmonic]: ...
    def thd_percent(self, node: int | str) -> float: ...

@final
class HbResult:
    @property
    def converged(self) -> bool: ...
    @property
    def iterations(self) -> int: ...
    @property
    def residual_norm(self) -> float: ...
    @property
    def fundamental_frequency(self) -> float: ...
    @property
    def num_harmonics(self) -> int: ...
    @property
    def node_names(self) -> list[str]: ...
    @property
    def harmonic_frequencies(self) -> npt.NDArray[np.float64]: ...
    @property
    def solve_time_seconds(self) -> float: ...
    @property
    def is_valid(self) -> bool: ...
    def coefficients(self, node: str) -> npt.NDArray[np.complex128]: ...
    def magnitude(self, node: str) -> npt.NDArray[np.float64]: ...
    def phase_degrees(self, node: str) -> npt.NDArray[np.float64]: ...
    def dc(self, node: str) -> float: ...
    def rms(self, node: str) -> float: ...
    def thd_percent(self, node: str) -> float: ...

@final
class PacResult:
    @property
    def frequencies(self) -> npt.NDArray[np.float64]: ...
    @property
    def fundamental_frequency(self) -> float: ...
    @property
    def sideband_min(self) -> int: ...
    @property
    def sideband_max(self) -> int: ...
    @property
    def sidebands(self) -> list[int]: ...
    @property
    def node_names(self) -> list[str]: ...
    @property
    def input_source(self) -> str | None: ...
    @property
    def output_node(self) -> str | None: ...
    @property
    def converged(self) -> bool: ...
    def voltage(self, node: str, sideband: int) -> npt.NDArray[np.complex128]: ...
    def conversion_gain(
        self, input_sideband: int, output_sideband: int
    ) -> npt.NDArray[np.complex128]: ...
    def conversion_gain_db(
        self, input_sideband: int, output_sideband: int
    ) -> npt.NDArray[np.float64]: ...

@final
class PeriodicNoiseContribution:
    @property
    def name(self) -> str: ...
    @property
    def power_spectral_density(self) -> npt.NDArray[np.float64]: ...

@final
class PeriodicNoiseResult:
    @property
    def frequencies(self) -> npt.NDArray[np.float64]: ...
    @property
    def output_noise(self) -> npt.NDArray[np.float64]: ...
    @property
    def output_noise_density(self) -> npt.NDArray[np.float64]: ...
    @property
    def input_noise(self) -> npt.NDArray[np.float64] | None: ...
    @property
    def contributors(self) -> list[PeriodicNoiseContribution]: ...
    @property
    def fundamental_frequency(self) -> float: ...
    @property
    def converged(self) -> bool: ...
    def contribution(self, name: str) -> PeriodicNoiseContribution: ...

@final
class OscillatorNoiseResult:
    @property
    def frequencies(self) -> npt.NDArray[np.float64]: ...
    @property
    def phase_noise_dbc(self) -> npt.NDArray[np.float64]: ...
    @property
    def diffusion_constant(self) -> float: ...
    @property
    def period(self) -> float: ...
    @property
    def corner_frequency(self) -> float: ...
    @property
    def carrier_frequency(self) -> float: ...

@final
class TransferFunctionResult:
    @property
    def output(self) -> str: ...
    @property
    def input(self) -> str: ...
    @property
    def gain(self) -> float: ...
    @property
    def gain_db(self) -> float: ...
    @property
    def input_impedance(self) -> float: ...
    @property
    def output_impedance(self) -> float: ...

@final
class Measurement:
    @property
    def name(self) -> str: ...
    @property
    def analysis(self) -> str: ...
    @property
    def value(self) -> float | None: ...
    @property
    def error(self) -> str | None: ...
    @property
    def expected(self) -> float | None: ...
    @property
    def tolerance(self) -> float | None: ...
    @property
    def passed(self) -> bool: ...
    def __float__(self) -> float: ...

@final
class AnalysisRecord:
    @property
    def kind(self) -> str: ...
    @property
    def detail(self) -> str: ...
    @property
    def skipped(self) -> bool: ...
    @property
    def reason(self) -> str | None: ...

@final
class RunReport:
    @property
    def op(self) -> SimulationResult | None: ...
    @property
    def dc(self) -> DcSweepResult | None: ...
    @property
    def tran(self) -> TransientResult | None: ...
    @property
    def ac(self) -> AcResult | None: ...
    @property
    def distortion(self) -> DistortionResult | None: ...
    @property
    def hb(self) -> HbResult | None: ...
    @property
    def s_parameters(self) -> SParameterResult | None: ...
    @property
    def noise(self) -> list[NoiseResult] | None: ...
    @property
    def tf(self) -> TransferFunctionResult | None: ...
    @property
    def stb(self) -> StbResult | None: ...
    @property
    def pz(self) -> PoleZeroResult | None: ...
    @property
    def monte_carlo(self) -> MonteCarloResult | None: ...
    @property
    def step(self) -> DcSweepResult | None: ...
    @property
    def temperature(self) -> DcSweepResult | None: ...
    @property
    def sensitivity(self) -> SensitivityResult | None: ...
    @property
    def sensitivity_ac(self) -> AcSensitivityResult | None: ...
    @property
    def fourier(self) -> list[FourierResult]: ...
    @property
    def records(self) -> list[AnalysisRecord]: ...
    @property
    def measurements(self) -> list[Measurement]: ...
    def measurement(self, name: str) -> Measurement | None: ...
    @property
    def num_measurements(self) -> int: ...
    @property
    def all_passed(self) -> bool: ...
    @property
    def failures(self) -> list[Measurement]: ...
    @property
    def analyses_run(self) -> list[str]: ...
    @property
    def skipped(self) -> list[AnalysisRecord]: ...
    def assert_passed(self) -> None: ...

@final
class HealthReport:
    @property
    def status(self) -> Literal["ready"]: ...
    @property
    def ready(self) -> Literal[True]: ...
    @property
    def duration_seconds(self) -> float: ...
    @property
    def element_count(self) -> int: ...
    @property
    def node_count(self) -> int: ...
    @property
    def branch_count(self) -> int: ...
    @property
    def output_voltage(self) -> float: ...

@final
class Engine:
    def __new__(cls, config: SimulationConfig | None = None) -> Engine: ...
    def cancel(self) -> int: ...
    def health_check(self) -> HealthReport: ...
    @property
    def is_running(self) -> bool: ...
    @property
    def active_run_count(self) -> int: ...
    @property
    def progress(self) -> float | None: ...
    def run(self, netlist: Netlist) -> RunReport: ...
    def measure(
        self, netlist: Netlist, result: TransientResult | DcSweepResult
    ) -> list[Measurement]: ...
    def run_dc_op(self, netlist: Netlist) -> SimulationResult: ...
    def run_dc_sweep(
        self,
        netlist: Netlist,
        source_name: str,
        start: float,
        stop: float,
        step: float,
    ) -> DcSweepResult: ...
    def run_ac(self, netlist: Netlist, frequencies: Sequence[float]) -> AcResult: ...
    def run_ac_data(self, netlist: Netlist, table_name: str) -> AcResult: ...
    def run_ac_sweep(
        self,
        netlist: Netlist,
        variation: str,
        points: int,
        start_freq: float,
        stop_freq: float,
    ) -> AcResult: ...
    def run_distortion(
        self,
        netlist: Netlist,
        frequencies: Sequence[float],
        f2_over_f1: float | None = None,
    ) -> DistortionResult: ...
    def run_distortion_sweep(
        self,
        netlist: Netlist,
        variation: str,
        points: int,
        start_freq: float,
        stop_freq: float,
        f2_over_f1: float | None = None,
    ) -> DistortionResult: ...
    def run_s_parameters(
        self,
        netlist: Netlist,
        frequencies: Sequence[float],
        do_noise: bool = False,
    ) -> SParameterResult: ...
    def run_stb(
        self,
        netlist: Netlist,
        probe: str,
        variation: str = "dec",
        points: int = 50,
        start_freq: float = 1.0,
        stop_freq: float = 100e6,
    ) -> StbResult: ...
    def run_pss(
        self,
        netlist: Netlist,
        fundamental_frequency: float | None = None,
        *,
        harmonics: int = 9,
        tstab: float = 0.0,
        tstab_periods: int | None = None,
        max_iterations: int = 100,
        tolerance: float = 1e-6,
        abstol: float = 1e-12,
        damping: float = 1.0,
        max_period_change: float = 0.1,
        points_per_period: int = 256,
        integration_method: IntegrationMethod | None = None,
        autonomous: bool = False,
        period_guess: float | None = None,
        verbose: bool = False,
    ) -> PssResult: ...
    def run_hb(
        self,
        netlist: Netlist,
        fundamental_frequency: float,
        *,
        harmonics: int = 9,
        tolerance: float = 1e-6,
        max_iterations: int = 100,
        damping: float = 1.0,
        oversample: int = 2,
        use_krylov: bool = False,
        source_stepping: bool = False,
        abstol: float = 1e-12,
        min_damping: float = 0.1,
        collocation_points: int | None = None,
        max_mixing_order: int = 5,
        gmres_restart: int = 30,
        use_exact_jacobian: bool = True,
        source_name: str | None = None,
        verbose: bool = False,
    ) -> HbResult: ...
    def run_hb_multitone(
        self,
        netlist: Netlist,
        frequencies: Sequence[float],
        *,
        harmonics: Sequence[int] | None = None,
        source_names: Sequence[str] | None = None,
        tolerance: float = 1e-6,
        abstol: float = 1e-12,
        max_iterations: int = 100,
        damping: float = 1.0,
        min_damping: float = 0.1,
        oversample: int = 2,
        collocation_points: int | None = None,
        max_mixing_order: int = 5,
        use_krylov: bool = False,
        gmres_restart: int = 30,
        source_stepping: bool = False,
        use_exact_jacobian: bool = True,
        verbose: bool = False,
    ) -> HbResult: ...
    def run_pac(
        self,
        netlist: Netlist,
        fundamental_frequency: float,
        start_frequency: float,
        stop_frequency: float,
        points: int,
        input_source: str,
        output_node: str,
        *,
        variation: str = "dec",
        sideband_min: int | None = None,
        sideband_max: int = 5,
        reference_node: str | None = None,
        reltol: float = 1e-3,
        abstol: float = 1e-12,
    ) -> PacResult: ...
    def run_pnoise(
        self,
        netlist: Netlist,
        fundamental_frequency: float,
        offsets: Sequence[float],
        output_node: str,
        *,
        reference_node: str | None = None,
        input_source: str | None = None,
        max_sideband: int = 6,
    ) -> PeriodicNoiseResult: ...
    def run_oscillator_noise(
        self,
        netlist: Netlist,
        offsets: Sequence[float],
        *,
        period_guess: float,
        harmonics: int = 9,
        tstab: float = 0.0,
        tstab_periods: int = 20,
        max_iterations: int = 100,
        tolerance: float = 1e-6,
        abstol: float = 1e-12,
        damping: float = 1.0,
        max_period_change: float = 0.1,
        points_per_period: int = 256,
        integration_method: IntegrationMethod | None = None,
        verbose: bool = False,
    ) -> OscillatorNoiseResult: ...
    def run_tran(
        self,
        netlist: Netlist,
        stop_time: float,
        max_step: float | None = None,
        *,
        start_time: float = 0.0,
    ) -> TransientResult: ...
    def run_tran_checkpointed(
        self,
        netlist: Netlist,
        stop_time: float,
        max_step: float | None = None,
    ) -> tuple[TransientResult, TransientCheckpoint]: ...
    def run_tran_compressed(
        self,
        netlist: Netlist,
        stop_time: float,
        max_step: float | None = None,
        *,
        abs_tol: float = 1e-6,
        rel_tol: float = 1e-3,
        max_interval: float = 0.0,
    ) -> CompressedTransientResult: ...
    def resume_tran(
        self,
        netlist: Netlist,
        checkpoint: TransientCheckpoint,
        stop_time: float,
        max_step: float | None = None,
    ) -> tuple[TransientResult, TransientCheckpoint]: ...
    def run_noise(
        self,
        netlist: Netlist,
        output_node: int | str,
        frequencies: Sequence[float],
        temperature: float | None = None,
        input_source: str | None = None,
        reference_node: int | str | None = None,
    ) -> list[NoiseResult]: ...
    def run_pz(
        self,
        netlist: Netlist,
        input_node: int | str,
        output_node: int | str,
        *,
        input_negative: int | str | None = None,
        output_negative: int | str | None = None,
        input_type: str = "current",
        analysis: str = "pz",
    ) -> PoleZeroResult: ...
    def run_monte_carlo(
        self,
        netlist: Netlist,
        num_runs: int,
        seed: int | None = None,
        distribution: str = "gaussian",
        spread: float = 0.01,
        params: Sequence[str] | None = None,
    ) -> MonteCarloResult: ...
    def run_sensitivity(
        self,
        netlist: Netlist,
        output_node: int | str,
        param_name: str,
        param_value: float,
        delta: float | None = None,
    ) -> float: ...
    def run_sensitivity_linearized(
        self,
        netlist: Netlist,
        output_node: int | str,
        reference_node: int | str | None = None,
    ) -> SensitivityResult: ...
    def run_sensitivity_dc_complete(
        self,
        netlist: Netlist,
        output: int | str,
        reference: int | str | None = None,
        filters: Sequence[str] | None = None,
        output_is_current: bool = False,
    ) -> SensitivityResult: ...
    def run_sensitivity_ac(
        self,
        netlist: Netlist,
        output_node: int | str,
        param_name: str,
        param_value: float,
        frequencies: Sequence[float],
        delta: float | None = None,
    ) -> npt.NDArray[np.float64]: ...
    def run_sensitivity_ac_complete(
        self,
        netlist: Netlist,
        output: int | str,
        frequencies: Sequence[float],
        reference: int | str | None = None,
        filters: Sequence[str] | None = None,
        output_is_current: bool = False,
    ) -> AcSensitivityResult: ...
    def run_step(
        self, netlist: Netlist, param_name: str, values: Sequence[float]
    ) -> list[tuple[float, SimulationResult]]: ...
    def run_transfer_function(
        self,
        netlist: Netlist,
        output_node: str,
        input_source: str,
        output_is_current: bool = False,
        reference_node: str | None = None,
    ) -> TransferFunctionResult: ...
    @property
    def config(self) -> SimulationConfig: ...
