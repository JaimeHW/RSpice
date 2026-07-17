//! DC Analysis - operating point and sweep
//!
//! This module provides DC analysis functions:
//! - Operating point (DC OP) calculation
//! - DC sweep for I-V curve generation

#![allow(clippy::too_many_arguments)]

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::solver::{SimulationResult, StaticMatrix};
use crate::{CircuitData, Netlist, Value};

const DC_SWEEP_CONTINUATION_MAX_SUBDIVISIONS: usize = 128;
const DC_SWEEP_RESULT_PREALLOC_LIMIT: usize = 4096;

/// One accepted point from a DC sweep, including the solved node/branch result
/// and the per-device operating-point report cached at that bias.
#[derive(Debug, Clone)]
pub struct DcSweepPointResult {
    pub sweep_value: Value,
    pub result: SimulationResult,
    pub device_op_report: crate::circuit::DeviceOpReport,
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
    fn populate_dc_observables(
        circuit: &CircuitData,
        solution: &[Value],
        result: &mut SimulationResult,
    ) {
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
        for (name, current) in result.branch_names.iter().zip(&result.branch_currents) {
            if !name.is_empty() {
                let internal_name = if name.to_ascii_uppercase().ends_with("_BRANCH") {
                    name.clone()
                } else {
                    format!("{name}_BRANCH")
                };
                result
                    .dc_observables
                    .push((format!("N({internal_name})"), *current));
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
            result.dc_observables.push((format!("I({name})"), current));
            result.dc_observables.push((format!("P({name})"), power));
            result.dc_observables.push((format!("W({name})"), power));
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
            result.dc_observables.push((format!("I({name})"), current));
            result.dc_observables.push((format!("P({name})"), power));
            result.dc_observables.push((format!("W({name})"), power));
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
            result
                .dc_observables
                .push((format!("I({})", source.name), current));
            result
                .dc_observables
                .push((format!("P({})", source.name), power));
            result
                .dc_observables
                .push((format!("W({})", source.name), power));
        }
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
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);

        // Build circuit from netlist
        let mut circuit = engine.build_circuit(netlist)?;

        if circuit.num_nodes() == 0 {
            return Ok((
                Self::build_empty_dc_result(),
                crate::circuit::DeviceOpReport::default(),
            ));
        }

        // Build matrix structure (done once)
        let matrix = engine.build_matrix(&circuit)?;

        // Link phase: bake CSC indices into device storage for O(1) stamping
        circuit.link_indices(&matrix);

        let mut matrix = matrix;

        let solution = engine.solve_dc_operating_point_with_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            abort,
        )?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let solution = if circuit.has_nonlinear_devices() || !circuit.generic_switches.is_empty() {
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
        Self::populate_dc_observables(&circuit, &solution, &mut result);

        Ok((result, circuit.device_op_report()))
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
    pub fn run_dc_sweep2_with_report_and_abort(
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

        let outer_points = sweep2.spec().points();
        if outer_points.is_empty() {
            return Err(SimulationError::Circuit(
                "Invalid second-source sweep parameters".to_string(),
            ));
        }

        let outer_is_temp = sweep2.source.eq_ignore_ascii_case("TEMP")
            || sweep2.source.eq_ignore_ascii_case("TEMPER");
        let outer_is_parameter = Self::netlist_has_numeric_parameter(netlist, &sweep2.source);

        let mut results = Vec::new();
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
                    crate::constants::thermal_voltage(
                        crate::analysis::temperature::celsius_to_kelvin(outer_value),
                    ),
                );
                swept
            } else if outer_is_parameter {
                let (swept, bindings) =
                    Self::create_perturbed_netlist(netlist, &sweep2.source, outer_value)?;
                any_outer_parameter_binding |= bindings > 0;
                swept
            } else {
                let mut swept = netlist.clone();
                Self::override_independent_source_dc(&mut swept, &sweep2.source, outer_value)?;
                swept
            };
            let inner =
                self.run_dc_sweep_spec_with_report_and_abort(&swept, source_name, primary, abort)?;
            results.extend(inner);
        }
        if outer_is_parameter && netlist.source_text.is_some() && !any_outer_parameter_binding {
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
    pub fn run_dc_sweep_spec_with_report_and_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        spec: &crate::netlist::DcSweepSpec,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<DcSweepPointResult>, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        let sweep_points = spec.points();

        if sweep_points.is_empty() {
            return Err(SimulationError::Circuit(
                "Invalid sweep parameters".to_string(),
            ));
        }

        if source_name.eq_ignore_ascii_case("TEMP") || source_name.eq_ignore_ascii_case("TEMPER") {
            let mut results =
                Vec::with_capacity(sweep_points.len().min(DC_SWEEP_RESULT_PREALLOC_LIMIT));
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
                    crate::constants::thermal_voltage(
                        crate::analysis::temperature::celsius_to_kelvin(sweep_value),
                    ),
                );
                let (result, device_op_report) = self.run_dc_op_with_report(&swept)?;
                results.push(DcSweepPointResult {
                    sweep_value,
                    result,
                    device_op_report,
                });
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

        // Build circuit once
        let mut circuit = engine.build_circuit(netlist)?;

        if circuit.num_nodes() == 0 {
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
                Self::populate_dc_observables(&circuit, &solution, &mut result);

                results.push(DcSweepPointResult {
                    sweep_value,
                    result,
                    device_op_report: circuit.device_op_report(),
                });
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

        for &sweep_value in sweep_points {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let (swept, bindings) =
                Self::create_perturbed_netlist(netlist, param_name, sweep_value)?;
            any_binding |= bindings > 0;
            let (result, device_op_report) = self.run_dc_op_with_report(&swept).map_err(|err| {
                SimulationError::Circuit(format!(
                    "DC parameter sweep {} = {} failed: {}",
                    param_name, sweep_value, err
                ))
            })?;
            results.push(DcSweepPointResult {
                sweep_value,
                result,
                device_op_report,
            });
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
        config.temperature = crate::analysis::temperature::celsius_to_kelvin(37.0);
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

        let vt = crate::constants::thermal_voltage(
            crate::analysis::temperature::celsius_to_kelvin(25.0),
        );
        let expected = -1.0e-18 * ((1.2 / vt).exp() - 1.0);
        let current = result
            .branch_current_named("VD")
            .expect("VD branch current is present");
        let rel = (current - expected).abs() / expected.abs();
        assert!(
            rel <= 5.0e-3,
            "expected diode source current near Shockley value {expected:.12e}, got {current:.12e} (rel={rel:.3e})"
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
