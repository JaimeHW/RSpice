//! Convergence helpers for Newton-Raphson iteration
//!
//! This module provides:
//! - GMIN stepping for difficult circuits
//! - Source stepping for convergence
//! - Linear and nonlinear solver interfaces

use super::{Engine, SimulationError};
use crate::solver::{SolverError, StaticMatrix};
use crate::{CircuitData, Value};

impl Engine {
    /// Try solving with a specific GMIN value
    pub(crate) fn try_solve_with_gmin(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        gmin: Value,
    ) -> Result<Vec<Value>, SolverError> {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];

        matrix.clear_values();
        rhs.fill(0.0);

        self.stamp_dc_direct(circuit, matrix, &mut rhs, gmin);
        matrix.solve(&rhs)
    }

    /// GMIN stepping: try progressively smaller GMIN values
    pub(crate) fn gmin_stepping(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SolverError> {
        // GMIN stepping sequence from large to small
        const GMIN_VALUES: &[Value] = &[1e-2, 1e-4, 1e-6, 1e-9, 1e-12];

        let mut solution = None;

        for &gmin in GMIN_VALUES {
            match self.try_solve_with_gmin(circuit, matrix, gmin) {
                Ok(sol) => {
                    solution = Some(sol);
                    // Continue to try smaller GMIN for better accuracy
                }
                Err(_) if solution.is_some() => {
                    // Can't solve with smaller GMIN, use the last successful one
                    break;
                }
                Err(e) if gmin == GMIN_VALUES[GMIN_VALUES.len() - 1] => {
                    // Last GMIN value failed and we have no solution
                    return Err(e);
                }
                Err(_) => {
                    // Try next GMIN value
                    continue;
                }
            }
        }

        solution.ok_or(SolverError::SingularMatrix)
    }

    /// Source stepping: ramp sources from 0 to 100%
    pub(crate) fn source_stepping(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SolverError> {
        // Source stepping sequence
        const SOURCE_SCALES: &[Value] = &[0.0, 0.1, 0.25, 0.5, 0.75, 1.0];
        const GMIN: Value = 1e-12;

        let size = circuit.matrix_size();
        let mut solution = vec![0.0; size]; // Start from zero

        for &scale in SOURCE_SCALES {
            let mut rhs = vec![0.0; size];

            matrix.clear_values();
            rhs.fill(0.0);

            self.stamp_dc_scaled(circuit, matrix, &mut rhs, GMIN, scale);

            match matrix.solve(&rhs) {
                Ok(sol) => {
                    solution = sol;
                }
                Err(e) if scale == 1.0 => {
                    return Err(e);
                }
                Err(_) => {
                    // Try to continue with the current solution
                    continue;
                }
            }
        }

        Ok(solution)
    }

    /// Solve a linear circuit (no nonlinear devices)
    pub(crate) fn solve_linear(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];

        matrix.clear_values();
        rhs.fill(0.0);
        self.stamp_dc_direct(circuit, matrix, &mut rhs, 1e-12);

        match matrix.solve(&rhs) {
            Ok(sol) => Ok(sol),
            Err(_) => {
                // Try GMIN stepping
                match self.gmin_stepping(circuit, matrix) {
                    Ok(sol) => Ok(sol),
                    Err(_) => {
                        // Try source stepping as last resort
                        self.source_stepping(circuit, matrix)
                            .map_err(SimulationError::Solver)
                    }
                }
            }
        }
    }

    /// Solve a nonlinear circuit using Newton-Raphson iteration
    pub(crate) fn solve_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut solution = vec![0.0; size];
        let mut rhs = vec![0.0; size];

        // Newton-Raphson iteration
        for _iteration in 0..self.config.max_iterations {
            // Clear matrix and RHS for this iteration
            matrix.clear_values();
            rhs.fill(0.0);

            // Add GMIN to diagonal for numerical stability
            for i in 0..size {
                matrix.add(i, i, 1e-12);
            }

            // Stamp linear devices
            circuit.stamp_dc_direct(matrix, &mut rhs);

            // Update nonlinear devices with current solution and stamp
            circuit.update_nonlinear(&solution);
            circuit.stamp_nonlinear(matrix, &mut rhs, &solution);

            // Solve linearized system
            let new_solution = matrix.solve(&rhs).map_err(SimulationError::Solver)?;

            // Check convergence (both voltage change and device convergence)
            let voltage_converged =
                Self::check_voltage_convergence(&solution, &new_solution, self.config.tolerance);
            let device_converged = circuit.nonlinear_converged(self.config.tolerance);

            solution = new_solution;

            if voltage_converged && device_converged {
                return Ok(solution);
            }
        }

        // If we didn't converge, try with source stepping
        self.source_stepping_nonlinear(circuit, matrix)
    }

    /// Check if voltage solution has converged
    pub(crate) fn check_voltage_convergence(
        old: &[Value],
        new: &[Value],
        tolerance: Value,
    ) -> bool {
        if old.len() != new.len() {
            return false;
        }
        for (&v_old, &v_new) in old.iter().zip(new.iter()) {
            let abs_diff = (v_new - v_old).abs();
            let rel_diff = if v_new.abs() > tolerance {
                abs_diff / v_new.abs()
            } else {
                0.0
            };
            if abs_diff > tolerance && rel_diff > 1e-3 {
                return false;
            }
        }
        true
    }

    /// Source stepping for nonlinear circuits
    pub(crate) fn source_stepping_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        const SOURCE_SCALES: &[Value] = &[0.0, 0.1, 0.25, 0.5, 0.75, 1.0];

        let size = circuit.matrix_size();
        let mut solution = vec![0.0; size];

        for &scale in SOURCE_SCALES {
            // Run Newton iterations at this source level
            for _iteration in 0..self.config.max_iterations {
                let mut rhs = vec![0.0; size];

                matrix.clear_values();

                // Add GMIN
                for i in 0..size {
                    matrix.add(i, i, 1e-12);
                }

                // Stamp linear devices with scaled sources
                circuit.resistors.stamp_all_direct(matrix);
                let num_nodes = circuit.num_nodes();
                circuit
                    .voltage_sources
                    .stamp_all_direct_scaled(matrix, &mut rhs, scale, |br| num_nodes + br);
                circuit.current_sources.stamp_all_scaled(&mut rhs, scale);

                // Stamp nonlinear devices
                circuit.update_nonlinear(&solution);
                circuit.stamp_nonlinear(matrix, &mut rhs, &solution);

                match matrix.solve(&rhs) {
                    Ok(new_solution) => {
                        let converged = Self::check_voltage_convergence(
                            &solution,
                            &new_solution,
                            self.config.tolerance,
                        );
                        solution = new_solution;
                        if converged && circuit.nonlinear_converged(self.config.tolerance) {
                            break;
                        }
                    }
                    Err(e) if scale == 1.0 => {
                        return Err(SimulationError::Solver(e));
                    }
                    Err(_) => {
                        break; // Try next scale
                    }
                }
            }
        }

        Ok(solution)
    }
}
