//! DC Analysis - operating point and sweep
//!
//! This module provides DC analysis functions:
//! - Operating point (DC OP) calculation
//! - DC sweep for I-V curve generation

#![allow(clippy::too_many_arguments)]

use super::core::DcOpStartup;
use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::resource::{ResourceKind, ResourceLimitError};
use crate::solver::{SimulationResult, StaticMatrix};
use crate::{CircuitData, Netlist, Value};

const DC_SWEEP_CONTINUATION_MAX_SUBDIVISIONS: usize = 128;
const DC_SWEEP_RESULT_PREALLOC_LIMIT: usize = 4096;

/// Enumerate the sweep points a `.DC` spec expands to.
///
/// Bounded by the engine's analysis-point resource limit, so an unbounded
/// or hostile sweep specification fails here rather than during the run.
pub fn bounded_dc_sweep_points(
    engine: &Engine,
    spec: &crate::netlist::DcSweepSpec,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    spec.points_bounded_with_abort(engine.config.resource_limits.max_analysis_points, abort)
        .map_err(|error| match error {
            crate::netlist::SweepPointGenerationError::Aborted => SimulationError::Aborted,
            crate::netlist::SweepPointGenerationError::LimitExceeded { requested, limit } => {
                ResourceLimitError {
                    resource: ResourceKind::AnalysisPoints,
                    requested,
                    limit,
                }
                .into()
            }
        })
}

/// One accepted point from a DC sweep, including the solved node/branch result
/// and the per-device operating-point report cached at that bias.
#[derive(Debug, Clone)]
pub struct DcSweepPointResult {
    pub sweep_value: Value,
    pub result: SimulationResult,
    pub device_op_report: crate::circuit::DeviceOpReport,
}

fn dc_result_value_count(
    result: &SimulationResult,
    device_op_report: &crate::circuit::DeviceOpReport,
) -> usize {
    Engine::simulation_result_value_count(result).saturating_add(
        device_op_report
            .entries
            .iter()
            .map(|entry| entry.params.len())
            .fold(0usize, usize::saturating_add),
    )
}

fn dc_sweep_point_value_count(point: &DcSweepPointResult) -> usize {
    dc_result_value_count(&point.result, &point.device_op_report).saturating_add(1)
}

enum DcSweepSource {
    Voltage {
        index: usize,
        original_value: Value,
        original_source_spec: Option<crate::netlist::SourceSpec>,
    },
    Current {
        index: usize,
        original_value: Value,
        original_source_spec: Option<crate::netlist::SourceSpec>,
    },
}

/// Resolve a Xyce device-parameter `.DC` source to the canonical override
/// spelling consumed by the engine's AST perturbation path.
///
/// Xyce accepts both a bare passive instance (for example `R1`) and an
/// explicit device parameter (`R1:R`).  The former means the element's
/// primary value parameter.  Keeping this resolution in the engine makes the
/// direct DC API and the regression wrapper agree on the same semantics.
pub fn canonical_device_parameter_sweep_source(
    netlist: &Netlist,
    source_name: &str,
) -> Option<String> {
    let source_name = source_name.trim();
    let (device_name, requested_parameter) = source_name
        .rsplit_once(':')
        .map_or((source_name, None), |(device, parameter)| {
            (device.trim(), Some(parameter.trim()))
        });
    if device_name.is_empty() || requested_parameter.is_some_and(str::is_empty) {
        return None;
    }

    let element = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case(device_name))?;
    let parameter = match &element.kind {
        crate::netlist::ElementKind::Resistor { .. } => {
            // Resistor models expose additional numeric instance parameters;
            // the generic STEP/DC perturbation path validates and applies
            // those parameters when the deck requests one explicitly.
            requested_parameter.unwrap_or("R")
        }
        crate::netlist::ElementKind::Capacitor { .. } => {
            let parameter = requested_parameter.unwrap_or("C");
            if [
                "C",
                "CAP",
                "VALUE",
                "CAPACITANCE",
                "IC",
                "L",
                "LENGTH",
                "W",
                "WIDTH",
                "M",
                "MULT",
                "SCALE",
                "TEMP",
                "DTEMP",
                "TC1",
                "TC2",
            ]
            .iter()
            .any(|alias| parameter.eq_ignore_ascii_case(alias))
            {
                parameter
            } else {
                return None;
            }
        }
        crate::netlist::ElementKind::Inductor { .. } => {
            let parameter = requested_parameter.unwrap_or("L");
            if [
                "L",
                "IND",
                "VALUE",
                "INDUCTANCE",
                "M",
                "MULT",
                "SCALE",
                "TEMP",
                "DTEMP",
                "TC1",
                "TC2",
            ]
            .iter()
            .any(|alias| parameter.eq_ignore_ascii_case(alias))
            {
                parameter
            } else {
                return None;
            }
        }
        crate::netlist::ElementKind::JilesAthertonInductor { .. } => {
            let parameter = requested_parameter.unwrap_or("L");
            if ["L", "VALUE", "INDUCTANCE"]
                .iter()
                .any(|alias| parameter.eq_ignore_ascii_case(alias))
            {
                parameter
            } else {
                return None;
            }
        }
        _ => return None,
    };

    Some(format!(
        "{}:{}",
        element.name,
        parameter.to_ascii_uppercase()
    ))
}

impl DcSweepSource {
    fn set_value(&self, circuit: &mut CircuitData, value: Value) {
        match self {
            Self::Voltage { index, .. } => {
                circuit.voltage_sources.dc_values[*index] = value;
            }
            Self::Current { index, .. } => {
                circuit.current_sources.dc_values[*index] = value;
            }
        }
    }

    fn restore(self, circuit: &mut CircuitData) {
        match self {
            Self::Voltage {
                index,
                original_value,
                original_source_spec,
            } => {
                circuit.voltage_sources.dc_values[index] = original_value;
                circuit.voltage_sources.source_specs[index] = original_source_spec;
            }
            Self::Current {
                index,
                original_value,
                original_source_spec,
            } => {
                circuit.current_sources.dc_values[index] = original_value;
                circuit.current_sources.source_specs[index] = original_source_spec;
            }
        }
    }
}

impl Engine {
    /// Resolve a Xyce device-parameter `.DC` source to the canonical AST
    /// override spelling used by the engine and regression wrapper.
    pub fn canonical_device_parameter_sweep_source(
        netlist: &Netlist,
        source_name: &str,
    ) -> Option<String> {
        canonical_device_parameter_sweep_source(netlist, source_name)
    }

    fn populate_dc_observables(
        circuit: &mut CircuitData,
        solution: &[Value],
        result: &mut SimulationResult,
    ) -> Result<(), SimulationError> {
        let node_voltage = |node: usize| {
            if node == 0 {
                0.0
            } else {
                // A converged MNA solution contains one slot for every
                // non-ground circuit node. Callers construct the result only
                // after a successful solve of this exact circuit topology.
                solution[node - 1]
            }
        };

        // Every MNA branch unknown is an internal solution variable in Xyce.
        // Its exported solution name is the canonical instance name followed
        // by `_BRANCH`, independently of whether the owning element also has
        // a conventional I(...) accessor.
        for index in 0..result.branch_names.len().min(result.branch_currents.len()) {
            let name = &result.branch_names[index];
            let current = result.branch_currents[index];
            if !name.is_empty() {
                let internal_name = if name.to_ascii_uppercase().ends_with("_BRANCH") {
                    name.clone()
                } else {
                    format!("{name}_BRANCH")
                };
                result.push_dc_observable(format!("N({internal_name})"), current);
            }
        }

        // A nodal resistor has no MNA branch unknown, so evaluate its lead
        // current directly from the converged terminal voltages and the
        // conductance actually installed in this circuit instance.
        for ((name, stamp), conductance) in circuit
            .resistors
            .names
            .iter()
            .zip(&circuit.resistors.stamps)
            .zip(&circuit.resistors.conductances)
        {
            let voltage = node_voltage(stamp.pp.row) - node_voltage(stamp.nn.row);
            let current = voltage * conductance;
            let power = voltage * current;
            result.push_dc_observable(format!("I({name})"), current);
            result.push_dc_observable(format!("P({name})"), power);
            result.push_dc_observable(format!("W({name})"), power);
        }

        // Zero and near-zero resistors use an explicit MNA branch. Preserve
        // the same positive-to-negative lead convention and use the actual
        // solved branch current rather than dividing by a tiny resistance.
        for index in 0..circuit.resistor_branches.names.len() {
            let name = &circuit.resistor_branches.names[index];
            let voltage = node_voltage(circuit.resistor_branches.node_pos[index])
                - node_voltage(circuit.resistor_branches.node_neg[index]);
            let branch_ordinal = circuit.resistor_branches.branch_indices[index];
            // Explicit resistor branches are allocated after the node slots
            // and use the same one-based branch ordinal as every MNA branch.
            let current = solution[circuit.num_nodes() + branch_ordinal - 1];
            let power = voltage * current;
            result.push_dc_observable(format!("I({name})"), current);
            result.push_dc_observable(format!("P({name})"), power);
            result.push_dc_observable(format!("W({name})"), power);
        }

        // Native diodes are nodal nonlinear devices, so they do not allocate
        // an MNA branch unknown.  Their accepted lead current is nevertheless
        // a first-class Xyce observable and must be evaluated from the same
        // converged terminal voltage used by the device's constitutive law.
        // Keep the positive-to-negative anode/cathode convention consistent
        // with voltage-source and resistor I(...) observables.
        for diode in &circuit.diodes.devices {
            let voltage = node_voltage(diode.node_anode) - node_voltage(diode.node_cathode);
            let current = diode.current(voltage);
            result.push_dc_observable(format!("I({})", diode.name), current);
        }

        // A solution-dependent resistor is stamped as a behavioral current
        // expression so its complete Jacobian participates in Newton solves.
        // Preserve the resistor lead-observable contract at the accepted
        // solution without mutating the circuit's expression state.
        for source in &circuit.behavioral_sources.current_sources {
            if !source.has_two_terminal_observables() {
                continue;
            }
            let voltage = node_voltage(source.node_pos) - node_voltage(source.node_neg);
            let mut observable_source = source.clone();
            let current = observable_source.evaluate(solution, 0.0);
            let power = voltage * current;
            result.push_dc_observable(format!("I({})", source.name), current);
            result.push_dc_observable(format!("P({})", source.name), power);
            result.push_dc_observable(format!("W({})", source.name), power);
        }

        // Native Xyce memristor lead current and resistance are device outputs rather
        // than MNA branches. Evaluate both from the accepted state without
        // adding observation unknowns to the circuit matrix.
        for binding in &mut circuit.xyce_memristors {
            let v_pos = node_voltage(binding.node_pos);
            let v_neg = node_voltage(binding.node_neg);
            let x = node_voltage(binding.node_x);
            let cache = binding
                .device
                .evaluate(v_pos, v_neg, x, true)
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "{} memristor '{}' DC output evaluation failed: {error}",
                        binding.device.family_name(),
                        binding.name
                    ))
                })?;
            let voltage = v_pos - v_neg;
            let power = voltage * cache.current;
            if let Some(resistance) = cache.resistance {
                binding.resistance_store = resistance;
            }
            result.push_dc_observable(format!("I({})", binding.name), cache.current);
            result.push_dc_observable(format!("P({})", binding.name), power);
            result.push_dc_observable(format!("W({})", binding.name), power);
            result.push_dc_observable(format!("N({}:R)", binding.name), binding.resistance_store);
        }
        Ok(())
    }

    fn build_empty_dc_result() -> SimulationResult {
        let mut result = SimulationResult::new(0, 0);
        result.node_names = vec!["0".to_string()];
        result
    }

    fn solve_nonlinear_dc_sweep_target_with_substeps(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        sweep_source: &DcSweepSource,
        from_value: Value,
        to_value: Value,
        seed: &[Value],
        min_subdivisions: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Value>, usize), SimulationError> {
        let span = to_value - from_value;
        if !span.is_finite() || span == 0.0 {
            sweep_source.set_value(circuit, to_value);
            return self
                .solve_nonlinear_with_guess_and_abort(circuit, matrix, Some(seed), abort)
                .map(|solution| (solution, 1));
        }

        let start_state = circuit.nonlinear_state_snapshot();
        let mut subdivisions = min_subdivisions.max(2).next_power_of_two();
        let mut last_error = None;

        while subdivisions <= DC_SWEEP_CONTINUATION_MAX_SUBDIVISIONS {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }

            circuit.restore_nonlinear_state(start_state.clone());
            sweep_source.set_value(circuit, from_value);
            let mut solution = seed.to_vec();
            let mut accepted = true;

            for step_idx in 1..=subdivisions {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let alpha = step_idx as Value / subdivisions as Value;
                sweep_source.set_value(circuit, from_value + alpha * span);
                match self.solve_nonlinear_with_guess_and_abort(
                    circuit,
                    matrix,
                    Some(&solution),
                    abort,
                ) {
                    Ok(next_solution) => {
                        solution = next_solution;
                    }
                    Err(err) => {
                        last_error = Some(err);
                        accepted = false;
                        break;
                    }
                }
            }

            if accepted {
                return Ok((solution, subdivisions));
            }

            subdivisions *= 2;
        }

        circuit.restore_nonlinear_state(start_state);
        sweep_source.set_value(circuit, from_value);
        Err(last_error.unwrap_or(SimulationError::ConvergenceFailed(
            self.config.max_iterations,
        )))
    }

    /// Run DC operating point analysis
    pub fn run_dc_op(&self, netlist: &Netlist) -> Result<SimulationResult, SimulationError> {
        self.run_dc_op_with_abort(netlist, &NoAbort)
    }

    /// Run a DC operating point with cooperative cancellation.
    pub fn run_dc_op_with_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_dc_op_with_report_and_abort(netlist, abort)
            .map(|(result, _)| result)
    }

    /// Run DC operating point analysis and return the per-device
    /// operating-point report alongside the node solution.
    ///
    /// The report carries each semiconductor device's bias point and
    /// small-signal parameters (id/gm/gds/region for MOSFETs, ic/beta/gm for
    /// BJTs, vd/id/gd for diodes) as cached by the converged Newton solve.
    pub fn run_dc_op_with_report(
        &self,
        netlist: &Netlist,
    ) -> Result<(SimulationResult, crate::circuit::DeviceOpReport), SimulationError> {
        self.run_dc_op_with_report_and_abort(netlist, &NoAbort)
    }

    /// Run a DC operating point and device report with cooperative cancellation.
    pub fn run_dc_op_with_report_and_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<(SimulationResult, crate::circuit::DeviceOpReport), SimulationError> {
        self.run_dc_op_with_startup_report_and_abort(netlist, DcOpStartup::Automatic, abort)
    }

    /// Run an exact DC operating point with `.IC` node values held as hard
    /// equality constraints throughout the nonlinear solve.
    pub fn run_dc_op_forced_ic_with_report_and_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<(SimulationResult, crate::circuit::DeviceOpReport), SimulationError> {
        self.run_dc_op_with_startup_report_and_abort(
            netlist,
            DcOpStartup::ForceInitialConditions,
            abort,
        )
    }

    /// Use a caller-retained complete MNA solution as the explicit seed. The
    /// seed is accepted only when its exact dimension matches this circuit;
    /// callers must additionally bind it to their immutable netlist identity.
    pub fn run_dc_op_with_previous_solution_and_report_and_abort(
        &self,
        netlist: &Netlist,
        previous_solution: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<(SimulationResult, crate::circuit::DeviceOpReport), SimulationError> {
        self.run_dc_op_with_startup_report_and_abort(
            netlist,
            DcOpStartup::PreviousSolution(previous_solution),
            abort,
        )
    }

    /// Run from the exact all-zero MNA seed, bypassing automatic startup
    /// hints while retaining normal Newton convergence checks.
    pub fn run_dc_op_from_zero_with_report_and_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<(SimulationResult, crate::circuit::DeviceOpReport), SimulationError> {
        self.run_dc_op_with_startup_report_and_abort(netlist, DcOpStartup::Zero, abort)
    }

    fn run_dc_op_with_startup_report_and_abort(
        &self,
        netlist: &Netlist,
        startup: DcOpStartup<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<(SimulationResult, crate::circuit::DeviceOpReport), SimulationError> {
        let force_initial_conditions = matches!(startup, DcOpStartup::ForceInitialConditions);
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.reset_convergence_quality();
        let engine = self.resolved_for_netlist(netlist);

        // Build circuit from netlist
        let mut circuit = engine.build_circuit_with_abort(netlist, abort)?;

        if circuit.num_nodes() == 0 {
            if force_initial_conditions {
                return Err(SimulationError::Circuit(
                    "forced .IC operating point requires at least one valid .IC node voltage"
                        .to_owned(),
                ));
            }
            let result = Self::build_empty_dc_result();
            let report = crate::circuit::DeviceOpReport::default();
            engine.ensure_result_values(dc_result_value_count(&result, &report))?;
            return Ok((result, report));
        }

        // Build matrix structure (done once)
        let matrix = engine.build_matrix(&circuit)?;

        // Link phase: bake CSC indices into device storage for O(1) stamping
        circuit.link_indices(&matrix);

        let mut matrix = matrix;

        let solution = engine.solve_dc_operating_point_with_startup_and_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            startup,
            abort,
        )?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let solution = if !force_initial_conditions
            && (circuit.has_nonlinear_devices() || !circuit.generic_switches.is_empty())
        {
            engine
                .dc_static_probe_polished_solution(&mut circuit, &mut matrix, &solution)
                .unwrap_or(solution)
        } else {
            solution
        };
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let Some(message) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE evaluation failed: {message}"
            )));
        }

        // Build result
        let mut result = SimulationResult::new(circuit.num_nodes(), circuit.num_branches());

        // Populate node names from circuit (results include actual net names)
        let sorted_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();
        result.node_names = std::iter::once("0".to_string()) // Ground is node 0
            .chain(sorted_names)
            .collect();
        result.branch_names = branch_names;

        for (i, &v) in solution.iter().enumerate() {
            if i < circuit.num_nodes() {
                result.node_voltages[i + 1] = v; // +1 because node 0 is ground
            } else {
                result.branch_currents[i - circuit.num_nodes()] = v;
            }
        }
        Self::populate_dc_observables(&mut circuit, &solution, &mut result)?;
        let device_op_report = circuit.device_op_report();
        engine.ensure_result_values(dc_result_value_count(&result, &device_op_report))?;

        Ok((result, device_op_report))
    }

    /// Run DC sweep analysis
    ///
    /// Sweeps one source through a range of values, solving DC at each point.
    /// Returns a vector of (sweep_value, solution) pairs.
    pub fn run_dc_sweep(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        self.run_dc_sweep_with_abort(netlist, source_name, start, stop, step, &NoAbort)
    }

    /// Two-source DC sweep: the first (inner) source sweeps fully at every
    /// value of the second (outer) source, ngspice-style; results are the
    /// inner sweeps concatenated in outer order, each point tagged with the
    /// inner sweep value. With no second sweep this is a plain DC sweep.
    ///
    /// The outer source must be a top-level independent source (or `TEMP`).
    pub fn run_dc_sweep2_with_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
        sweep2: Option<&crate::netlist::DcSecondSweep>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        self.run_dc_sweep2_with_report_and_abort(
            netlist,
            source_name,
            start,
            stop,
            step,
            sweep2,
            abort,
        )
        .map(|points| {
            points
                .into_iter()
                .map(|point| (point.sweep_value, point.result))
                .collect()
        })
    }

    /// Two-source DC sweep that preserves per-point device operating reports.
    pub(crate) fn run_dc_sweep2_with_report_and_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
        sweep2: Option<&crate::netlist::DcSecondSweep>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let primary = crate::netlist::DcSweepSpec::linear(start, stop, step);
        self.run_dc_sweep2_spec_with_report_and_abort(netlist, source_name, &primary, sweep2, abort)
    }

    /// Two-source DC sweep using the full `.DC` sweep specification for
    /// non-linear sweep modes such as LIST, DEC, and OCT.
    pub fn run_dc_sweep2_spec_with_report_and_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        primary: &crate::netlist::DcSweepSpec,
        sweep2: Option<&crate::netlist::DcSecondSweep>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let Some(sweep2) = sweep2 else {
            return self.run_dc_sweep_spec_with_report_and_abort(
                netlist,
                source_name,
                primary,
                abort,
            );
        };

        let engine = self.resolved_for_netlist(netlist);
        let outer_points = bounded_dc_sweep_points(&engine, &sweep2.spec(), abort)?;
        if outer_points.is_empty() {
            return Err(SimulationError::Circuit(
                "Invalid second-source sweep parameters".to_string(),
            ));
        }
        let inner_point_count = bounded_dc_sweep_points(&engine, primary, abort)?.len();
        engine.ensure_analysis_points(outer_points.len().saturating_mul(inner_point_count))?;

        let outer_is_temp = sweep2.source.eq_ignore_ascii_case("TEMP")
            || sweep2.source.eq_ignore_ascii_case("TEMPER");
        let outer_is_parameter = Self::netlist_has_numeric_parameter(netlist, &sweep2.source);
        let outer_device_parameter =
            canonical_device_parameter_sweep_source(netlist, &sweep2.source);

        let mut results = Vec::new();
        let mut retained_values = 0usize;
        let mut any_outer_parameter_binding = false;
        for &outer_value in &outer_points {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let swept = if outer_is_temp {
                let mut swept = netlist.clone();
                swept.options.temp = Some(outer_value);
                swept.params.set("TEMP", outer_value);
                swept.params.set("TEMPER", outer_value);
                swept.params.set(
                    "VT",
                    crate::constants::thermal_voltage(crate::constants::celsius_to_kelvin(
                        outer_value,
                    )),
                );
                swept
            } else if outer_is_parameter {
                let (swept, bindings) = Self::create_perturbed_netlist_with_limits_and_abort(
                    netlist,
                    &sweep2.source,
                    outer_value,
                    engine.config.resource_limits,
                    abort,
                )?;
                any_outer_parameter_binding |= bindings > 0;
                swept
            } else if let Some(device_parameter) = &outer_device_parameter {
                let (swept, bindings) = Self::create_perturbed_netlist_with_limits_and_abort(
                    netlist,
                    device_parameter,
                    outer_value,
                    engine.config.resource_limits,
                    abort,
                )?;
                any_outer_parameter_binding |= bindings > 0;
                swept
            } else {
                let mut swept = netlist.clone();
                Self::override_independent_source_dc(&mut swept, &sweep2.source, outer_value)?;
                swept
            };
            let inner =
                self.run_dc_sweep_spec_with_report_and_abort(&swept, source_name, primary, abort)?;
            retained_values = inner.iter().fold(retained_values, |total, point| {
                total.saturating_add(dc_sweep_point_value_count(point))
            });
            engine.ensure_result_values(retained_values)?;
            results.extend(inner);
        }
        if (outer_is_parameter || outer_device_parameter.is_some())
            && netlist.source_text.is_some()
            && !any_outer_parameter_binding
        {
            return Err(SimulationError::Circuit(format!(
                "Second DC sweep parameter '{}' is not bound to any netlist expression",
                sweep2.source
            )));
        }
        Ok(results)
    }

    /// Set the DC operating value of a named top-level independent source.
    fn override_independent_source_dc(
        netlist: &mut Netlist,
        source_name: &str,
        value: Value,
    ) -> Result<(), SimulationError> {
        use crate::netlist::ElementKind;
        for element in &mut netlist.elements {
            if element.name.eq_ignore_ascii_case(source_name) {
                return match &mut element.kind {
                    ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                        *spec = spec.clone().with_dc_value(value);
                        Ok(())
                    }
                    _ => Err(SimulationError::Circuit(format!(
                        "Second DC sweep source '{}' must be an independent source",
                        source_name
                    ))),
                };
            }
        }
        Err(SimulationError::Circuit(format!(
            "Second DC sweep source not found: {}",
            source_name
        )))
    }

    pub fn run_dc_sweep_with_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        self.run_dc_sweep_with_report_and_abort(netlist, source_name, start, stop, step, abort)
            .map(|points| {
                points
                    .into_iter()
                    .map(|point| (point.sweep_value, point.result))
                    .collect()
            })
    }

    /// Run a DC sweep and return per-point device operating reports alongside
    /// the solved node/branch vectors.
    pub fn run_dc_sweep_with_report_and_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let spec = crate::netlist::DcSweepSpec::linear(start, stop, step);
        self.run_dc_sweep_spec_with_report_and_abort(netlist, source_name, &spec, abort)
    }

    /// Run a DC sweep from an already parsed sweep specification.
    pub(crate) fn run_dc_sweep_spec_with_report_and_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        spec: &crate::netlist::DcSweepSpec,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        let sweep_points = bounded_dc_sweep_points(&engine, spec, abort)?;

        if sweep_points.is_empty() {
            return Err(SimulationError::Circuit(
                "Invalid sweep parameters".to_string(),
            ));
        }

        if source_name.eq_ignore_ascii_case("TEMP") || source_name.eq_ignore_ascii_case("TEMPER") {
            let mut results =
                Vec::with_capacity(sweep_points.len().min(DC_SWEEP_RESULT_PREALLOC_LIMIT));
            let mut retained_values = 0usize;
            for &sweep_value in &sweep_points {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let mut swept = netlist.clone();
                swept.options.temp = Some(sweep_value);
                swept.params.set("TEMP", sweep_value);
                swept.params.set("TEMPER", sweep_value);
                swept.params.set(
                    "VT",
                    crate::constants::thermal_voltage(crate::constants::celsius_to_kelvin(
                        sweep_value,
                    )),
                );
                let (result, device_op_report) =
                    self.run_dc_op_with_report_and_abort(&swept, abort)?;
                let point = DcSweepPointResult {
                    sweep_value,
                    result,
                    device_op_report,
                };
                retained_values =
                    retained_values.saturating_add(dc_sweep_point_value_count(&point));
                engine.ensure_result_values(retained_values)?;
                results.push(point);
            }
            return Ok(results);
        }

        if Self::netlist_has_numeric_parameter(netlist, source_name) {
            return self.run_dc_parameter_sweep_spec_with_report_and_abort(
                netlist,
                source_name,
                &sweep_points,
                abort,
            );
        }

        if let Some(device_parameter) =
            canonical_device_parameter_sweep_source(netlist, source_name)
        {
            return self.run_dc_parameter_sweep_spec_with_report_and_abort(
                netlist,
                &device_parameter,
                &sweep_points,
                abort,
            );
        }

        // Build circuit once
        let mut circuit = engine.build_circuit_with_abort(netlist, abort)?;

        if circuit.num_nodes() == 0 {
            engine.ensure_result_shape(sweep_points.len(), 2)?;
            return Ok(sweep_points
                .into_iter()
                .map(|value| DcSweepPointResult {
                    sweep_value: value,
                    result: Self::build_empty_dc_result(),
                    device_op_report: crate::circuit::DeviceOpReport::default(),
                })
                .collect());
        }

        // Find source index (case-insensitive comparison - SPICE standard)
        let source_name_upper = source_name.to_uppercase();
        let sweep_source = if let Some(index) = circuit
            .voltage_sources
            .names
            .iter()
            .position(|n| n.to_uppercase() == source_name_upper)
        {
            // Store original source state so the sweep is reversible even if a point fails.
            DcSweepSource::Voltage {
                index,
                original_value: circuit.voltage_sources.dc_values[index],
                original_source_spec: circuit.voltage_sources.source_specs[index].take(),
            }
        } else if let Some(index) = circuit
            .current_sources
            .names
            .iter()
            .position(|n| n.to_uppercase() == source_name_upper)
        {
            DcSweepSource::Current {
                index,
                original_value: circuit.current_sources.dc_values[index],
                original_source_spec: circuit.current_sources.source_specs[index].take(),
            }
        } else {
            return Err(SimulationError::Circuit(format!(
                "Source not found: {}",
                source_name
            )));
        };

        // Build matrix structure (done once)
        let matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let mut matrix = matrix;

        let sorted_node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();

        let node_hints = self.collect_node_voltage_hints(netlist, &circuit);

        let sweep_result = (|| -> Result<Vec<DcSweepPointResult>, SimulationError> {
            let mut results =
                Vec::with_capacity(sweep_points.len().min(DC_SWEEP_RESULT_PREALLOC_LIMIT));
            let mut retained_values = 0usize;

            // Use previous solution as initial guess for next point.
            // For the first point, apply .NODESET/.IC hints if present.
            let mut prev_solution: Option<Vec<Value>> = None;
            let mut prev_sweep_value: Option<Value> = None;
            let mut dc_sweep_subdivisions = 2;

            for &sweep_value in &sweep_points {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                // Update source value.
                sweep_source.set_value(&mut circuit, sweep_value);

                // Solve DC at this point
                // Key optimization: use previous solution as initial guess for faster convergence
                let solution =
                    if circuit.has_nonlinear_devices() || !circuit.generic_switches.is_empty() {
                        if let Some(seed) = prev_solution.as_deref() {
                            let previous_value = prev_sweep_value.unwrap_or(sweep_value);
                            let start_state = circuit.nonlinear_state_snapshot();
                            match engine.solve_nonlinear_with_guess_and_abort(
                                &mut circuit,
                                &mut matrix,
                                Some(seed),
                                abort,
                            ) {
                                Ok(solution) => {
                                    dc_sweep_subdivisions = 2;
                                    solution
                                }
                                Err(_) => {
                                    circuit.restore_nonlinear_state(start_state.clone());
                                    sweep_source.set_value(&mut circuit, previous_value);
                                    match engine.solve_nonlinear_dc_sweep_target_with_substeps(
                                        &mut circuit,
                                        &mut matrix,
                                        &sweep_source,
                                        previous_value,
                                        sweep_value,
                                        seed,
                                        dc_sweep_subdivisions,
                                        abort,
                                    ) {
                                        Ok((solution, subdivisions)) => {
                                            dc_sweep_subdivisions = subdivisions;
                                            solution
                                        }
                                        Err(substep_error) => {
                                            if abort.is_aborted() {
                                                return Err(substep_error);
                                            }

                                            circuit.restore_nonlinear_state(start_state);
                                            sweep_source.set_value(&mut circuit, sweep_value);
                                            let fresh_attempt = if node_hints.is_empty() {
                                                engine.solve_nonlinear_with_node_hints_and_abort(
                                                    &mut circuit,
                                                    &mut matrix,
                                                    &[],
                                                    abort,
                                                )
                                            } else {
                                                engine.solve_nonlinear_with_node_hints_and_abort(
                                                    &mut circuit,
                                                    &mut matrix,
                                                    &node_hints,
                                                    abort,
                                                )
                                            };
                                            if let Ok(solution) = fresh_attempt {
                                                dc_sweep_subdivisions = 2;
                                                solution
                                            } else {
                                                return Err(substep_error);
                                            }
                                        }
                                    }
                                }
                            }
                        } else if node_hints.is_empty() {
                            engine.solve_nonlinear_with_node_hints_and_abort(
                                &mut circuit,
                                &mut matrix,
                                &[],
                                abort,
                            )?
                        } else {
                            engine.solve_nonlinear_with_node_hints_and_abort(
                                &mut circuit,
                                &mut matrix,
                                &node_hints,
                                abort,
                            )?
                        }
                    } else {
                        if abort.is_aborted() {
                            return Err(SimulationError::Aborted);
                        }
                        engine.solve_linear(&mut circuit, &mut matrix)?
                    };
                let solution =
                    if circuit.has_nonlinear_devices() || !circuit.generic_switches.is_empty() {
                        engine
                            .dc_static_probe_polished_solution(&mut circuit, &mut matrix, &solution)
                            .unwrap_or(solution)
                    } else {
                        solution
                    };

                if let Some(message) = circuit.take_xspice_evaluation_error() {
                    return Err(SimulationError::Circuit(format!(
                        "XSPICE evaluation failed: {message}"
                    )));
                }

                // Build result
                let mut result = SimulationResult::new(circuit.num_nodes(), circuit.num_branches());
                result.node_names = std::iter::once("0".to_string())
                    .chain(sorted_node_names.iter().cloned())
                    .collect();
                result.branch_names = branch_names.clone();
                for (i, &v) in solution.iter().enumerate() {
                    if i < circuit.num_nodes() {
                        result.node_voltages[i + 1] = v;
                    } else {
                        result.branch_currents[i - circuit.num_nodes()] = v;
                    }
                }
                Self::populate_dc_observables(&mut circuit, &solution, &mut result)?;

                let point = DcSweepPointResult {
                    sweep_value,
                    result,
                    device_op_report: circuit.device_op_report(),
                };
                retained_values =
                    retained_values.saturating_add(dc_sweep_point_value_count(&point));
                engine.ensure_result_values(retained_values)?;
                results.push(point);
                prev_solution = Some(solution);
                prev_sweep_value = Some(sweep_value);
            }

            Ok(results)
        })();

        sweep_source.restore(&mut circuit);

        sweep_result
    }

    fn run_dc_parameter_sweep_spec_with_report_and_abort(
        &self,
        netlist: &Netlist,
        param_name: &str,
        sweep_points: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let mut results =
            Vec::with_capacity(sweep_points.len().min(DC_SWEEP_RESULT_PREALLOC_LIMIT));
        let mut any_binding = false;
        let mut retained_values = 0usize;

        for &sweep_value in sweep_points {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let (swept, bindings) = Self::create_perturbed_netlist_with_limits_and_abort(
                netlist,
                param_name,
                sweep_value,
                self.config.resource_limits,
                abort,
            )?;
            any_binding |= bindings > 0;
            let (result, device_op_report) = self
                .run_dc_op_with_report_and_abort(&swept, abort)
                .map_err(|error| match error {
                    error @ SimulationError::Aborted
                    | error @ SimulationError::ResourceLimit(_)
                    | error @ SimulationError::Configuration(_) => error,
                    error => SimulationError::Circuit(format!(
                        "DC parameter sweep {param_name} = {sweep_value} failed: {error}"
                    )),
                })?;
            let point = DcSweepPointResult {
                sweep_value,
                result,
                device_op_report,
            };
            retained_values = retained_values.saturating_add(dc_sweep_point_value_count(&point));
            self.ensure_result_values(retained_values)?;
            results.push(point);
        }

        if netlist.source_text.is_some() && !any_binding {
            return Err(SimulationError::Circuit(format!(
                "DC sweep parameter '{}' is not bound to any netlist expression",
                param_name
            )));
        }

        Ok(results)
    }

    fn netlist_has_numeric_parameter(netlist: &Netlist, param_name: &str) -> bool {
        netlist
            .params
            .all_params()
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(param_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Netlist;
    use crate::engine::SpiceDialect;

    fn xyce_engine() -> Engine {
        Engine::new(
            crate::engine::SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce),
        )
    }

    fn assert_voltage(result: &SimulationResult, node: &str, expected: Value) {
        let actual = result
            .try_voltage_named(node)
            .unwrap_or_else(|| panic!("missing voltage for node {node}"));
        assert!(
            (actual - expected).abs() <= 1.0e-10,
            "expected V({node})={expected:.17e}, got {actual:.17e}"
        );
    }

    #[test]
    fn passive_device_parameter_dc_sweep_uses_canonical_ast_overrides() {
        let netlist = Netlist::parse(
            "passive device parameter sweep\n\
             V1 in 0 10\n\
             R1 in out 1\n\
             R2 out 0 1\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses");
        assert_eq!(
            Engine::canonical_device_parameter_sweep_source(&netlist, "R1"),
            Some("R1:R".to_string())
        );
        assert_eq!(
            Engine::canonical_device_parameter_sweep_source(&netlist, "r2:r"),
            Some("R2:R".to_string())
        );

        let points = xyce_engine()
            .run_dc_sweep2_spec_with_report_and_abort(
                &netlist,
                "R1:R",
                &crate::netlist::DcSweepSpec::linear(1.0, 3.0, 1.0),
                Some(&crate::netlist::DcSecondSweep::linear(
                    "R2:R".to_string(),
                    1.0,
                    2.0,
                    1.0,
                )),
                &NoAbort,
            )
            .expect("passive device sweep solves");

        assert_eq!(points.len(), 6);
        assert_voltage(&points[0].result, "out", 5.0);
        assert_voltage(&points[1].result, "out", 10.0 / 3.0);
        assert_voltage(&points[2].result, "out", 2.5);
        assert_voltage(&points[3].result, "out", 20.0 / 3.0);
        assert_voltage(&points[4].result, "out", 5.0);
        assert_voltage(&points[5].result, "out", 4.0);
    }

    #[test]
    fn bare_passive_device_dc_sweep_defaults_to_primary_value_parameter() {
        let netlist = Netlist::parse(
            "bare passive device parameter sweep\n\
             V1 in 0 10\n\
             R1 in out 1\n\
             R2 out 0 1\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses");
        let points = xyce_engine()
            .run_dc_sweep_spec_with_report_and_abort(
                &netlist,
                "R1",
                &crate::netlist::DcSweepSpec::linear(1.0, 2.0, 1.0),
                &NoAbort,
            )
            .expect("bare passive device sweep solves");

        assert_eq!(points.len(), 2);
        assert_voltage(&points[0].result, "out", 5.0);
        assert_voltage(&points[1].result, "out", 10.0 / 3.0);
    }

    #[test]
    fn forced_ic_fails_closed_for_an_empty_circuit() {
        let netlist = Netlist::parse("empty\n.op\n.end\n").expect("deck parses");
        let error = Engine::default()
            .run_dc_op_forced_ic_with_report_and_abort(&netlist, &NoAbort)
            .expect_err("force .IC must require an applied node voltage");
        assert!(
            error
                .to_string()
                .contains("requires at least one valid .IC node voltage"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn resolved_engine_preserves_explicit_temperature_over_deck_options() {
        let netlist = Netlist::parse(
            "authoritative temperature\n\
             V1 drive 0 1\n\
             R1 drive out 1k TC1=0.01\n\
             R2 out 0 1k\n\
             .options temp=25 tnom=25 reltol=1e-2\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses");
        let mut config = crate::engine::SimulationConfig::default();
        config.temperature = 125.0 + 273.15;
        config.tolerance = 1.0e-7;
        let engine = Engine::try_new_with_resolved_config(config).unwrap();
        let (result, _) = engine
            .run_dc_op_with_report_and_abort(&netlist, &NoAbort)
            .expect("explicit-temperature OP solves");
        assert_voltage(&result, "out", 1.0 / 3.0);
    }

    /// `.temp` is the classic spelling of the circuit temperature, and a deck
    /// that states one gets a different answer for every temperature-dependent
    /// device in it. Ignoring the card is therefore not a missing feature but
    /// a wrong answer returned without complaint, so these pin the physics and
    /// not just the parsed field.
    #[test]
    fn temp_directive_drives_diode_saturation_current() {
        // ngspice-46 oracle, converged: final point of `.dc v1 0 5 0.1` with
        // `.temp 85`. At 27 C the same deck answers 0.6929 V. Read the oracle
        // at tightened tolerances — ngspice seeds each sweep point from the
        // previous one and stops at RELTOL, which leaves its default-tolerance
        // answer 8e-6 V high and would swallow a real temperature error.
        let netlist = Netlist::parse(
            "diode forward transfer at 85c\n\
             v1 in 0 dc 0\n\
             r1 in anode 1k\n\
             d1 anode 0 dmod\n\
             .model dmod d(is=1e-14 n=1)\n\
             .temp 85\n\
             .dc v1 0 5 0.1\n\
             .end\n",
        )
        .expect("deck parses");
        let points = Engine::default()
            .run_dc_sweep_with_abort(&netlist, "v1", 0.0, 5.0, 0.1, &NoAbort)
            .expect("hot diode sweep solves");
        let (_, last) = points.last().expect("sweep produced points");
        let anode = last
            .try_voltage_named("anode")
            .expect("missing anode voltage");
        assert!(
            (anode - 5.966108e-01).abs() <= 1.0e-6,
            "expected the 85 C forward drop 5.966108e-01, got {anode:.7e}"
        );
    }

    #[test]
    fn temp_directive_drives_resistor_temperature_coefficients() {
        // R(T) = R0 * (1 + TC1*dT + TC2*dT^2) with dT = 85 - TNOM(27) = 58,
        // so the divider is exact in closed form: instance TC1 gives 1116 ohm
        // and the model card's TC1/TC2 pair gives 1149.64 ohm.
        for (deck, expected) in [
            (
                "resistor instance tc1 at 85c\n\
                 v1 in 0 dc 1\n\
                 r1 in out 1k tc1=0.002\n\
                 r2 out 0 1k\n\
                 .temp 85\n\
                 .op\n\
                 .end\n",
                1000.0 / (1116.0 + 1000.0),
            ),
            (
                "resistor model tc1/tc2 at 85c\n\
                 v1 in 0 dc 1\n\
                 r1 in out rmod 1k\n\
                 r2 out 0 1k\n\
                 .model rmod r(tc1=0.002 tc2=1e-5)\n\
                 .temp 85\n\
                 .op\n\
                 .end\n",
                1000.0 / (1149.64 + 1000.0),
            ),
        ] {
            let netlist = Netlist::parse(deck).expect("deck parses");
            let (result, _) = Engine::default()
                .run_dc_op_with_report_and_abort(&netlist, &NoAbort)
                .expect("hot divider solves");
            assert_voltage(&result, "out", expected);
        }
    }

    #[test]
    fn temp_directive_drives_behavioral_source_coefficients() {
        // ngspice's own `.temp` coverage is `regression/misc/asrc-tc-1.cir`,
        // which states this closed form and then checks it inside a `.control`
        // block — a block the conformance harness does not execute, which is
        // why that deck stayed green the whole time `.temp` was ignored.
        // dT = 127 - 27 = 100, so the coefficient scales the 100 V source by
        // 1 + 0.001*100. ngspice-46 answers 1.100000e+02.
        let netlist = Netlist::parse(
            "asrc temperature coefficient\n\
             v1 1 0 dc 100\n\
             b3 3 0 v=v(1) tc1=0.001\n\
             .temp 127\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses");
        let (result, _) = Engine::default()
            .run_dc_op_with_report_and_abort(&netlist, &NoAbort)
            .expect("hot behavioral source solves");
        assert_voltage(&result, "3", 110.0);
    }

    #[test]
    fn temp_directive_outranks_options_temp_at_the_engine() {
        // ngspice applies `.temp` after the deck is read, so it wins over
        // `.options temp` whichever card is written first.
        for deck in [
            "options first\n\
             v1 in 0 dc 1\n\
             r1 in out 1k tc1=0.002\n\
             r2 out 0 1k\n\
             .options temp=27\n\
             .temp 85\n\
             .op\n\
             .end\n",
            "temp first\n\
             v1 in 0 dc 1\n\
             r1 in out 1k tc1=0.002\n\
             r2 out 0 1k\n\
             .temp 85\n\
             .options temp=27\n\
             .op\n\
             .end\n",
        ] {
            let netlist = Netlist::parse(deck).expect("deck parses");
            let (result, _) = Engine::default()
                .run_dc_op_with_report_and_abort(&netlist, &NoAbort)
                .expect("mixed temperature deck solves");
            assert_voltage(&result, "out", 1000.0 / (1116.0 + 1000.0));
        }
    }

    #[test]
    fn explicit_temperature_still_outranks_the_temp_directive() {
        // A runner that sets a temperature is sweeping or overriding on
        // purpose; `.temp` is deck-level input and must not shadow it.
        let netlist = Netlist::parse(
            "explicit override\n\
             v1 in 0 dc 1\n\
             r1 in out 1k tc1=0.002\n\
             r2 out 0 1k\n\
             .temp 85\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses");
        let config = crate::engine::SimulationConfig {
            temperature: crate::constants::celsius_to_kelvin(27.0),
            ..Default::default()
        };
        let (result, _) = Engine::try_new_with_resolved_config(config)
            .expect("resolved config is valid")
            .run_dc_op_with_report_and_abort(&netlist, &NoAbort)
            .expect("overridden deck solves");
        assert_voltage(&result, "out", 0.5);
    }

    #[test]
    fn forced_ic_is_a_hard_constraint_and_skips_unconstrained_polish() {
        let netlist = Netlist::parse(
            "forced startup\n\
             V1 in 0 10\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .ic V(out)=2\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses");
        let (result, _) = Engine::default()
            .run_dc_op_forced_ic_with_report_and_abort(&netlist, &NoAbort)
            .expect("forced .IC solve succeeds");
        assert_voltage(&result, "out", 2.0);
    }

    #[test]
    fn previous_state_requires_the_complete_exact_mna_dimension() {
        let netlist =
            Netlist::parse("prior state\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n")
                .expect("deck parses");
        let error = Engine::default()
            .run_dc_op_with_previous_solution_and_report_and_abort(&netlist, &[0.5], &NoAbort)
            .expect_err("partial state must not be silently padded");
        assert!(
            error
                .to_string()
                .contains("incompatible with the current circuit"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn xyce_capacitor_ic_constrains_floating_terminal_during_dc_op() {
        let netlist = Netlist::parse(
            "floating capacitor IC constraint\n\
             V1 fixed 0 1\n\
             C1 fixed floating 1 IC=0\n\
             .OP\n\
             .END\n",
        )
        .expect("deck parses");
        let result = xyce_engine()
            .run_dc_op(&netlist)
            .expect("Xyce capacitor IC operating point solves");

        assert_voltage(&result, "fixed", 1.0);
        assert_voltage(&result, "floating", 1.0);
        assert_eq!(result.branch_names, ["V1", "C1"]);
        assert!(result.branch_current_named("C1").unwrap().abs() <= 1.0e-12);
    }

    #[test]
    fn xyce_capacitor_ic_supports_grounded_and_non_ground_constraints() {
        for (label, capacitor, expected) in [
            ("positive grounded", "C1 node 0 1 IC=2", 2.0),
            ("negative grounded", "C1 0 node 1 IC=2", -2.0),
            ("non-ground", "C1 fixed node 1 IC=2", 3.0),
        ] {
            let fixed_source = if label == "non-ground" {
                "V1 fixed 0 5\n"
            } else {
                ""
            };
            let deck =
                format!("{label} capacitor IC constraint\n{fixed_source}{capacitor}\n.OP\n.END\n");
            let netlist = Netlist::parse(&deck).expect("deck parses");
            let result = xyce_engine()
                .run_dc_op(&netlist)
                .unwrap_or_else(|error| panic!("{label} constraint must solve: {error}"));
            assert_voltage(&result, "node", expected);
        }
    }

    #[test]
    fn capacitor_without_ic_keeps_ordinary_open_circuit_dc_behavior() {
        let netlist = Netlist::parse(
            "ordinary capacitor DC\n\
             V1 source 0 4\n\
             R1 source out 1k\n\
             C1 out 0 1u\n\
             .OP\n\
             .END\n",
        )
        .expect("deck parses");
        let engine = xyce_engine();
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        assert_eq!(circuit.num_branches(), 1, "C1 must not allocate a branch");
        assert_eq!(circuit.capacitors.ic_branch_indices, [None]);

        let result = engine
            .run_dc_op(&netlist)
            .expect("DC operating point solves");
        assert_voltage(&result, "out", 4.0);
    }

    #[test]
    fn xyce_rejects_f_and_h_control_by_capacitor_lead_current() {
        for control in ["F1 out 0 C1 1", "H1 out 0 C1 1"] {
            let deck = format!(
                "invalid capacitor current control\nC1 n 0 1 IC=1\nR1 n 0 1\n{control}\nR2 out 0 1\n.OP\n.END\n"
            );
            let netlist = Netlist::parse(&deck).expect("deck parses");
            let error = xyce_engine()
                .run_dc_op(&netlist)
                .expect_err("Xyce F/H capacitor-current control must be rejected");
            assert!(
                error
                    .to_string()
                    .contains("require a voltage-source branch"),
                "unexpected rejection: {error}"
            );
        }
    }

    fn missing_pwl_path(name: &str) -> String {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        std::env::temp_dir()
            .join(format!(
                "rspice-missing-{name}-{}-{unique}.csv",
                std::process::id()
            ))
            .to_string_lossy()
            .replace('\\', "/")
    }

    #[test]
    fn dc_op_rejects_missing_pwl_file_source() {
        let path = missing_pwl_path("dc");
        let deck = format!(
            "missing PWL file\n\
             V1 in 0 PWL FILE=\"{path}\"\n\
             R1 in 0 1k\n\
             .op\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let err = Engine::default()
            .run_dc_op(&netlist)
            .expect_err("missing PWL file must fail before DC solve");

        assert!(
            err.to_string().contains("failed to load PWL file"),
            "unexpected error: {err}"
        );
        assert!(err.to_string().contains(&path));
    }

    #[test]
    fn dc_op_stamps_solution_independent_behavioral_source_in_linear_solve() {
        let deck = "\
behavioral source linear dc
B1 out 0 V={5}
R1 out 0 1k
.op
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("dc operating point should solve");

        let actual = result
            .try_voltage_named("out")
            .expect("out node voltage is present");
        assert!(
            (actual - 5.0).abs() < 1.0e-12,
            "expected behavioral source to drive V(out)=5, got {actual}"
        );
    }

    #[test]
    fn replaceground_applies_inside_behavioral_voltage_probes() {
        let deck = "\
behavioral ground aliases
V1 one 0 2
R1 one out_gnd 0.5
B1 out_gnd 0 I={2*V(out_gnd,gNd)}
V2 two 0 2
R2 two out_ground 0.5
B2 out_ground 0 I={2*V(out_ground,GROUND)}
V3 three 0 2
R3 three out_bang 0.5
B3 out_bang 0 I={2*V(out_bang,gnd!)}
.PREPROCESS REPLACEGROUND TRUE
.op
.end
";
        let netlist = Netlist::parse(deck).expect("ground-alias behavioral deck parses");
        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("ground-alias behavioral deck solves");

        for node in ["out_gnd", "out_ground", "out_bang"] {
            let actual = result
                .try_voltage_named(node)
                .unwrap_or_else(|| panic!("{node} voltage is present"));
            assert!(
                (actual - 1.0).abs() < 1.0e-10,
                "expected {node}=1 V, got {actual}"
            );
        }
    }

    #[test]
    fn xspice_integrator_initial_condition_is_initialized_before_dc_op() {
        let deck = r#"
* xspice integrator initial condition
V1 in 0 0
A1 in out integrator out_ic=5 out_lower_limit=-10 out_upper_limit=10
Rload out 0 1k
.op
.end
"#;
        let netlist = Netlist::parse(deck).expect("deck parses");
        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("dc operating point should solve");
        let out_idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap_or_else(|| panic!("out node present in {:?}", result.node_names));

        assert!(
            (result.node_voltages[out_idx] - 5.0).abs() < 1e-9,
            "expected integrator out_ic=5 to drive V(out), got {}",
            result.node_voltages[out_idx]
        );
    }

    #[test]
    fn dc_sweep_supports_independent_current_sources() {
        let deck = "\
current source dc sweep
I1 in 0 0
R1 in 0 1k
.dc I1 -1m 1m 1m
.print dc V(in)
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let results = Engine::default()
            .run_dc_sweep(&netlist, "I1", -1.0e-3, 1.0e-3, 1.0e-3)
            .expect("current-source DC sweep solves");

        assert_eq!(results.len(), 3);
        for ((actual_sweep, result), (expected_sweep, expected_voltage)) in
            results
                .iter()
                .zip([(-1.0e-3, 1.0), (0.0, 0.0), (1.0e-3, -1.0)])
        {
            assert!(
                (actual_sweep - expected_sweep).abs() < 1.0e-15,
                "unexpected sweep point {actual_sweep}, expected {expected_sweep}"
            );
            let actual_voltage = result
                .try_voltage_named("in")
                .expect("swept node voltage is present");
            assert!(
                (actual_voltage - expected_voltage).abs() < 1.0e-9,
                "unexpected V(in) at I1={actual_sweep}: {actual_voltage}, expected {expected_voltage}"
            );
        }
    }

    #[test]
    fn dc_sweep_retains_resistor_lead_power_and_internal_branch_observables() {
        let deck = "\
DC observable retention
VSRC1 1a 0 1
RLOAD1A 1a 1b 0.1
RLOAD1B 1b 0 1
.dc VSRC1 1 5 1
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let results = Engine::default()
            .run_dc_sweep(&netlist, "VSRC1", 1.0, 5.0, 1.0)
            .expect("DC sweep solves");

        assert_eq!(results.len(), 5);
        for (sweep_value, result) in &results {
            let current = sweep_value / 1.1;
            let resistor_voltage = current * 0.1;
            let power = resistor_voltage * current;
            assert!(
                (result.try_dc_observable_named("I(rload1a)").unwrap() - current).abs() < 1e-12
            );
            assert!((result.try_dc_observable_named("P(RLOAD1A)").unwrap() - power).abs() < 1e-12);
            assert!((result.try_dc_observable_named("W(rLoAd1A)").unwrap() - power).abs() < 1e-12);
            assert!(
                (result.try_dc_observable_named("N(VSRC1_BRANCH)").unwrap() + current).abs()
                    < 1e-12
            );
        }
    }

    #[test]
    fn modeled_solution_dependent_resistor_uses_model_temperature_law_and_observables() {
        let deck = r#"modeled solution-dependent resistor
VCTRL ctrl 0 2
RCTRL ctrl 0 1
VIN in 0 1
RDYN in out {0.5*V(ctrl)} RMOD TC1=0.1 TC2=0.2
RLOAD out 0 2
.model RMOD R (TCE=3)
.op
.end
"#;
        let netlist = Netlist::parse(deck).expect("deck parses");
        let mut config = crate::engine::SimulationConfig::default();
        config.temperature = crate::constants::celsius_to_kelvin(37.0);
        let result = Engine::new(config)
            .run_dc_op(&netlist)
            .expect("modeled solution-dependent resistor solves");

        let resistance = 1.01_f64.powf(30.0);
        let expected_current = 1.0 / (resistance + 2.0);
        let expected_voltage = 2.0 * expected_current;
        let expected_power = resistance * expected_current * expected_current;

        assert!((result.try_voltage_named("out").unwrap() - expected_voltage).abs() < 1.0e-9);
        assert!(
            (result.try_dc_observable_named("I(RDYN)").unwrap() - expected_current).abs() < 1.0e-9
        );
        assert!(
            (result.try_dc_observable_named("P(rdyn)").unwrap() - expected_power).abs() < 1.0e-9
        );
        assert!(
            (result.try_dc_observable_named("W(RdYn)").unwrap() - expected_power).abs() < 1.0e-9
        );
    }

    #[test]
    fn parameter_dc_sweep_observables_use_each_rebuilt_circuit() {
        let deck = "\
parameter-dependent DC observable
.param RVAL=1
V1 in 0 4
R1 in 0 {RVAL}
.dc RVAL 1 4 1
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let results = Engine::default()
            .run_dc_sweep(&netlist, "RVAL", 1.0, 4.0, 1.0)
            .expect("parameter DC sweep solves");

        assert_eq!(results.len(), 4);
        for (resistance, result) in &results {
            let expected_current = 4.0 / resistance;
            let actual = result
                .try_dc_observable_named("I(R1)")
                .expect("per-point resistor current is retained");
            assert!(
                (actual - expected_current).abs() < 1e-12,
                "R={resistance}: expected I(R1)={expected_current}, got {actual}"
            );
        }
    }

    #[test]
    fn explicit_branch_resistor_observables_use_solved_lead_current() {
        let deck = "\
zero-resistance branch observable
V1 in 0 1
R1 in mid 1
RZERO mid 0 0
.op
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("zero-resistance branch circuit solves");

        let current = result
            .try_dc_observable_named("I(RZERO)")
            .expect("explicit resistor branch current is retained");
        let power = result
            .try_dc_observable_named("P(RZERO)")
            .expect("explicit resistor branch power is retained");
        assert!((current - 1.0).abs() < 1e-12, "expected 1 A, got {current}");
        assert!(power.abs() < 1e-12, "expected zero power, got {power}");
    }

    #[test]
    fn dc_sweep_supports_nonlinear_independent_current_sources() {
        let deck = "\
current source nonlinear dc sweep
I1 in 0 0
D1 0 in DMOD
.model DMOD D(IS=1e-14 N=1)
.dc I1 1n 1u 4.995e-7
.print dc V(in)
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let results = Engine::default()
            .run_dc_sweep(&netlist, "I1", 1.0e-9, 1.0e-6, 4.995e-7)
            .expect("current-source nonlinear DC sweep solves");

        assert_eq!(results.len(), 3);
        let voltages = results
            .iter()
            .map(|(_, result)| result.try_voltage_named("in").expect("node is present"))
            .collect::<Vec<_>>();
        assert!(
            voltages.windows(2).all(|pair| pair[0] > pair[1]),
            "diode voltage should become more negative as source current increases: {voltages:?}"
        );
    }

    #[test]
    fn dc_sweep_reports_polished_high_current_diode_source_branch_current() {
        let deck = "\
high current diode branch current polish
.OPTIONS DEVICE TNOM=25 TEMP=25
VD 1 0 DC 0.05
D1 1 0 DXX
.MODEL DXX D (LEVEL=2 IS=1e-18 N=1)
.DC VD 1.2 1.2 1
.PRINT DC V(1) I(VD)
.END
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let results = Engine::default()
            .run_dc_sweep(&netlist, "VD", 1.2, 1.2, 1.0)
            .expect("DC sweep solves");

        assert_eq!(results.len(), 1);
        let result = &results[0].1;
        let voltage = result
            .try_voltage_named("1")
            .expect("swept node voltage is present");
        assert!(
            (voltage - 1.2).abs() <= 1.0e-12,
            "expected ideal source to force V(1)=1.2, got {voltage}"
        );

        let vt = crate::constants::thermal_voltage(crate::constants::celsius_to_kelvin(25.0));
        let expected = -1.0e-18 * ((1.2 / vt).exp() - 1.0);
        let current = result
            .branch_current_named("VD")
            .expect("VD branch current is present");
        let rel = (current - expected).abs() / expected.abs();
        assert!(
            rel <= 5.0e-3,
            "expected diode source current near Shockley value {expected:.12e}, got {current:.12e} (rel={rel:.3e})"
        );

        let diode_current = result
            .try_dc_observable_named("I(D1)")
            .expect("native diode lead current is retained");
        assert!(
            diode_current.is_finite() && diode_current > 0.0,
            "expected a finite forward diode lead current, got {diode_current}"
        );
    }

    #[test]
    fn dc_sweep_supports_dependent_parameter_sources() {
        let deck = "\
parameter dc sweep
.param testnorm={0.5K}
.param r1value={testnorm*2.0}
R2 1 0 7k
R1 1 2 {r1value}
V1 2 0 1000
.dc testnorm 0.5k 0.7k 0.1k
.print dc testnorm V(1)
.end
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let results = Engine::default()
            .run_dc_sweep(&netlist, "testnorm", 0.5e3, 0.7e3, 0.1e3)
            .expect("parameter DC sweep solves");

        assert_eq!(results.len(), 3);
        for ((actual_sweep, result), (expected_sweep, expected_voltage)) in results.iter().zip([
            (500.0, 875.0),
            (600.0, 853.6585365853658),
            (700.0, 833.3333333333334),
        ]) {
            assert!(
                (actual_sweep - expected_sweep).abs() < 1.0e-12,
                "unexpected sweep point {actual_sweep}, expected {expected_sweep}"
            );
            let actual_voltage = result
                .try_voltage_named("1")
                .expect("swept node voltage is present");
            assert!(
                (actual_voltage - expected_voltage).abs() < 1.0e-9,
                "unexpected V(1) at testnorm={actual_sweep}: {actual_voltage}, expected {expected_voltage}"
            );
        }
    }
}
