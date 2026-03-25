//! DC Analysis - operating point and sweep
//!
//! This module provides DC analysis functions:
//! - Operating point (DC OP) calculation
//! - DC sweep for I-V curve generation

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::solver::SimulationResult;
use crate::{Netlist, Value};

impl Engine {
    fn build_empty_dc_result() -> SimulationResult {
        let mut result = SimulationResult::new(0, 0);
        result.node_names = vec!["0".to_string()];
        result
    }

    /// Run DC operating point analysis
    pub fn run_dc_op(&self, netlist: &Netlist) -> Result<SimulationResult, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);

        // Build circuit from netlist
        let mut circuit = engine.build_circuit(netlist)?;

        if circuit.num_nodes() == 0 {
            return Ok(Self::build_empty_dc_result());
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

        Ok(result)
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

        // Store original DC value
        let original_value = circuit.voltage_sources.dc_values[vsrc_idx];

        // Build matrix structure (done once)
        let matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let mut matrix = matrix;

        let mut results = Vec::with_capacity(sweep_points.len());
        let sorted_node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();

        let node_hints = self.collect_node_voltage_hints(netlist, &circuit);

        // Use previous solution as initial guess for next point.
        // For the first point, apply .NODESET/.IC hints if present.
        let mut prev_solution: Option<Vec<Value>> = None;

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
                    engine.solve_nonlinear_with_guess_and_abort(
                        &mut circuit,
                        &mut matrix,
                        Some(seed),
                        abort,
                    )?
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
        }

        // Restore original value
        circuit.voltage_sources.dc_values[vsrc_idx] = original_value;

        Ok(results)
    }
}
