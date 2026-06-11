//! DC Analysis - operating point and sweep
//!
//! This module provides DC analysis functions:
//! - Operating point (DC OP) calculation
//! - DC sweep for I-V curve generation

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::solver::{SimulationResult, StaticMatrix};
use crate::{CircuitData, Netlist, Value};

const DC_SWEEP_CONTINUATION_MAX_SUBDIVISIONS: usize = 128;

impl Engine {
    fn build_empty_dc_result() -> SimulationResult {
        let mut result = SimulationResult::new(0, 0);
        result.node_names = vec!["0".to_string()];
        result
    }

    fn solve_nonlinear_dc_sweep_target_with_substeps(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        vsrc_idx: usize,
        from_value: Value,
        to_value: Value,
        seed: &[Value],
        min_subdivisions: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Value>, usize), SimulationError> {
        let span = to_value - from_value;
        if !span.is_finite() || span == 0.0 {
            circuit.voltage_sources.dc_values[vsrc_idx] = to_value;
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
            circuit.voltage_sources.dc_values[vsrc_idx] = from_value;
            let mut solution = seed.to_vec();
            let mut accepted = true;

            for step_idx in 1..=subdivisions {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let alpha = step_idx as Value / subdivisions as Value;
                circuit.voltage_sources.dc_values[vsrc_idx] = from_value + alpha * span;
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
        circuit.voltage_sources.dc_values[vsrc_idx] = from_value;
        Err(last_error.unwrap_or(SimulationError::ConvergenceFailed(
            self.config.max_iterations,
        )))
    }

    /// Run DC operating point analysis
    pub fn run_dc_op(&self, netlist: &Netlist) -> Result<SimulationResult, SimulationError> {
        self.run_dc_op_with_report(netlist).map(|(result, _)| result)
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

        let solution = engine.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;

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

    pub fn run_dc_sweep_with_abort(
        &self,
        netlist: &Netlist,
        source_name: &str,
        start: Value,
        stop: Value,
        step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
        use crate::analysis::DcSweep;

        let engine = self.resolved_for_netlist(netlist);
        let sweep = DcSweep::new(source_name.to_string(), start, stop, step);
        let sweep_points = sweep.points();

        if sweep_points.is_empty() {
            return Err(SimulationError::Circuit(
                "Invalid sweep parameters".to_string(),
            ));
        }

        if source_name.eq_ignore_ascii_case("TEMP") || source_name.eq_ignore_ascii_case("TEMPER") {
            let mut results = Vec::with_capacity(sweep_points.len());
            for &sweep_value in &sweep_points {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let mut swept = netlist.clone();
                swept.options.temp = Some(sweep_value);
                swept.params.set("TEMP", sweep_value);
                swept.params.set("TEMPER", sweep_value);
                results.push((sweep_value, self.run_dc_op(&swept)?));
            }
            return Ok(results);
        }

        // Build circuit once
        let mut circuit = engine.build_circuit(netlist)?;

        if circuit.num_nodes() == 0 {
            return Ok(sweep_points
                .into_iter()
                .map(|value| (value, Self::build_empty_dc_result()))
                .collect());
        }

        // Find source index (case-insensitive comparison - SPICE standard)
        let source_name_upper = source_name.to_uppercase();
        let vsrc_idx = circuit
            .voltage_sources
            .names
            .iter()
            .position(|n| n.to_uppercase() == source_name_upper)
            .ok_or_else(|| {
                SimulationError::Circuit(format!("Source not found: {}", source_name))
            })?;

        // Store original source state so the sweep is reversible even if a point fails.
        let original_value = circuit.voltage_sources.dc_values[vsrc_idx];
        let original_source_spec = circuit.voltage_sources.source_specs[vsrc_idx].take();

        // Build matrix structure (done once)
        let matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let mut matrix = matrix;

        let sorted_node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();

        let node_hints = self.collect_node_voltage_hints(netlist, &circuit);

        let sweep_result = (|| -> Result<Vec<(Value, SimulationResult)>, SimulationError> {
            let mut results = Vec::with_capacity(sweep_points.len());

            // Use previous solution as initial guess for next point.
            // For the first point, apply .NODESET/.IC hints if present.
            let mut prev_solution: Option<Vec<Value>> = None;
            let mut prev_sweep_value: Option<Value> = None;
            let mut dc_sweep_subdivisions = 2;

            for &sweep_value in &sweep_points {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                // Update source value
                circuit.voltage_sources.dc_values[vsrc_idx] = sweep_value;

                // Solve DC at this point
                // Key optimization: use previous solution as initial guess for faster convergence
                let solution = if circuit.has_nonlinear_devices() {
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
                                circuit.voltage_sources.dc_values[vsrc_idx] = sweep_value;
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
                                    circuit.restore_nonlinear_state(start_state);
                                    circuit.voltage_sources.dc_values[vsrc_idx] = previous_value;
                                    let (solution, subdivisions) = engine
                                        .solve_nonlinear_dc_sweep_target_with_substeps(
                                            &mut circuit,
                                            &mut matrix,
                                            vsrc_idx,
                                            previous_value,
                                            sweep_value,
                                            seed,
                                            dc_sweep_subdivisions,
                                            abort,
                                        )?;
                                    dc_sweep_subdivisions = subdivisions;
                                    solution
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
                    engine.solve_linear(&circuit, &mut matrix)?
                };

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

                results.push((sweep_value, result));
                prev_solution = Some(solution);
                prev_sweep_value = Some(sweep_value);
            }

            Ok(results)
        })();

        circuit.voltage_sources.dc_values[vsrc_idx] = original_value;
        circuit.voltage_sources.source_specs[vsrc_idx] = original_source_spec;

        sweep_result
    }
}
