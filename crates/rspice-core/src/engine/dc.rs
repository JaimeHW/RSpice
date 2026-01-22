//! DC Analysis - operating point and sweep
//!
//! This module provides DC analysis functions:
//! - Operating point (DC OP) calculation
//! - DC sweep for I-V curve generation

use super::{Engine, SimulationError};
use crate::solver::SimulationResult;
use crate::{Netlist, Value};

impl Engine {
    /// Run DC operating point analysis
    pub fn run_dc_op(&self, netlist: &Netlist) -> Result<SimulationResult, SimulationError> {
        // Build circuit from netlist
        let mut circuit = self.build_circuit(netlist)?;

        if circuit.num_nodes() == 0 {
            return Err(SimulationError::Circuit("No nodes in circuit".to_string()));
        }

        // Build matrix structure (done once)
        let matrix = self.build_matrix(&circuit)?;

        // Link phase: bake CSC indices into device storage for O(1) stamping
        circuit.link_indices(&matrix);

        let mut matrix = matrix;

        // Choose solver based on circuit type
        let solution = if circuit.has_nonlinear_devices() {
            self.solve_nonlinear(&mut circuit, &mut matrix)?
        } else {
            self.solve_linear(&circuit, &mut matrix)?
        };

        // Build result
        let mut result = SimulationResult::new(circuit.num_nodes(), circuit.num_branches());

        // Populate node names from circuit (Spectre-style: results include actual net names)
        let sorted_names = circuit.node_names_sorted();
        result.node_names = std::iter::once("0".to_string()) // Ground is node 0
            .chain(sorted_names.into_iter())
            .collect();

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
        use crate::analysis::DcSweep;

        let sweep = DcSweep::new(source_name.to_string(), start, stop, step);
        let sweep_points = sweep.points();

        if sweep_points.is_empty() {
            return Err(SimulationError::Circuit(
                "Invalid sweep parameters".to_string(),
            ));
        }

        // Build circuit once
        let mut circuit = self.build_circuit(netlist)?;

        if circuit.num_nodes() == 0 {
            return Err(SimulationError::Circuit("No nodes in circuit".to_string()));
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
        let matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let mut matrix = matrix;

        let mut results = Vec::with_capacity(sweep_points.len());

        // Use previous solution as initial guess for next point
        let mut prev_solution: Option<Vec<Value>> = None;

        for &sweep_value in &sweep_points {
            // Update source value
            circuit.voltage_sources.dc_values[vsrc_idx] = sweep_value;

            // Solve DC at this point
            let solution = if circuit.has_nonlinear_devices() {
                // For nonlinear, would need to update with prev_solution as initial guess
                // For now, just solve fresh
                self.solve_nonlinear(&mut circuit, &mut matrix)?
            } else {
                self.solve_linear(&circuit, &mut matrix)?
            };

            // Build result
            let mut result = SimulationResult::new(circuit.num_nodes(), circuit.num_branches());
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
        let _ = prev_solution; // Suppress unused warning for now

        Ok(results)
    }
}
