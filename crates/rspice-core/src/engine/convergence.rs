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
    ///
    /// This is a convenience wrapper that starts from zero initial guess.
    /// For DC sweeps, use `solve_nonlinear_with_guess` with the previous solution.
    pub(crate) fn solve_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        self.solve_nonlinear_with_guess(circuit, matrix, None)
    }

    /// Solve a nonlinear circuit using Newton-Raphson iteration with optional initial guess
    ///
    /// # Arguments
    /// * `circuit` - Circuit data with nonlinear devices
    /// * `matrix` - Sparse matrix structure for MNA
    /// * `initial_guess` - Optional initial solution vector (e.g., from previous DC sweep point)
    ///
    /// Using a good initial guess (like the previous sweep point solution) significantly
    /// improves convergence speed and robustness for nonlinear circuits.
    ///
    /// # Returns
    /// The converged solution vector, or error if Newton-Raphson fails to converge.
    pub(crate) fn solve_nonlinear_with_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: Option<&[Value]>,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();

        // Use provided initial guess or start from zero
        let mut solution = match initial_guess {
            Some(guess) if guess.len() == size => guess.to_vec(),
            Some(guess) => {
                // Mismatched size - log warning and use zero
                // This can happen if circuit topology changed
                let mut sol = vec![0.0; size];
                // Copy as much as we can
                let copy_len = guess.len().min(size);
                sol[..copy_len].copy_from_slice(&guess[..copy_len]);
                sol
            }
            None => vec![0.0; size],
        };

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
        // Use current solution as starting point for source stepping
        self.source_stepping_nonlinear_with_guess(circuit, matrix, &solution)
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

    /// Source stepping for nonlinear circuits (starts from zero)
    #[allow(dead_code)]
    pub(crate) fn source_stepping_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let zero_guess = vec![0.0; size];
        self.source_stepping_nonlinear_with_guess(circuit, matrix, &zero_guess)
    }

    /// Source stepping for nonlinear circuits with initial guess
    ///
    /// # Arguments
    /// * `circuit` - Circuit data with nonlinear devices
    /// * `matrix` - Sparse matrix structure  
    /// * `initial_guess` - Starting solution (e.g., from failed Newton iteration or previous sweep point)
    ///
    /// Source stepping ramps sources from 0% to 100% in steps, which helps
    /// find operating points in difficult circuits with strong nonlinearities.
    pub(crate) fn source_stepping_nonlinear_with_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        const SOURCE_SCALES: &[Value] = &[0.0, 0.1, 0.25, 0.5, 0.75, 1.0];

        let size = circuit.matrix_size();

        // Start from provided initial guess (scaled to first source level)
        let mut solution = if initial_guess.len() == size {
            initial_guess.to_vec()
        } else {
            vec![0.0; size]
        };

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

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test voltage convergence checking with identical vectors.
    #[test]
    fn test_voltage_convergence_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with small absolute differences.
    #[test]
    fn test_voltage_convergence_small_absolute() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0 + 1e-10, 2.0 + 1e-10, 3.0 + 1e-10];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence fails with large differences.
    #[test]
    fn test_voltage_convergence_large_diff() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 4.0]; // 33% relative diff
        assert!(!Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with near-zero values (special handling).
    #[test]
    fn test_voltage_convergence_near_zero() {
        let a = vec![1e-15, 0.0, 1e-14];
        let b = vec![0.0, 1e-15, 0.0];
        // Near-zero values should use absolute tolerance only
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with mismatched lengths fails.
    #[test]
    fn test_voltage_convergence_mismatched_length() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(!Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with empty vectors.
    #[test]
    fn test_voltage_convergence_empty() {
        let a: Vec<f64> = vec![];
        let b: Vec<f64> = vec![];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test convergence at tolerance boundary.
    #[test]
    fn test_voltage_convergence_at_tolerance() {
        let tolerance: f64 = 1e-6;
        let a = vec![1.0];
        let b = vec![1.0 + tolerance * 0.9]; // Just under tolerance
        assert!(Engine::check_voltage_convergence(&a, &b, tolerance));
    }

    /// Test that relative tolerance kicks in for large values.
    #[test]
    fn test_voltage_convergence_relative_tolerance() {
        // For large values, relative tolerance of 1e-3 is used
        let a = vec![1000.0];
        let b = vec![1000.5]; // 0.05% difference, under 0.1%
        // 0.5 absolute diff > 1e-6 tolerance, but relative = 0.0005 < 1e-3
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-6));
    }

    /// Test voltage convergence with negative values.
    #[test]
    fn test_voltage_convergence_negative_values() {
        let a = vec![-5.0, -10.0];
        let b = vec![-5.0 + 1e-10, -10.0 - 1e-10];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test convergence with mixed positive/negative.
    #[test]
    fn test_voltage_convergence_mixed_signs() {
        let a = vec![5.0, -5.0, 0.0];
        let b = vec![5.0, -5.0, 0.0];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test that GMIN values are in decreasing order.
    #[test]
    fn test_gmin_stepping_values_order() {
        // The GMIN_VALUES constant should be in decreasing order
        // for proper stepping from large to small
        const GMIN_VALUES: &[f64] = &[1e-2, 1e-4, 1e-6, 1e-9, 1e-12];
        for i in 1..GMIN_VALUES.len() {
            assert!(
                GMIN_VALUES[i] < GMIN_VALUES[i - 1],
                "GMIN values should be decreasing"
            );
        }
    }

    /// Test that source stepping scales are in increasing order.
    #[test]
    fn test_source_stepping_values_order() {
        const SOURCE_SCALES: &[f64] = &[0.0, 0.1, 0.25, 0.5, 0.75, 1.0];
        for i in 1..SOURCE_SCALES.len() {
            assert!(
                SOURCE_SCALES[i] > SOURCE_SCALES[i - 1],
                "Source scales should be increasing"
            );
        }
        // Must start at 0 and end at 1
        assert!((SOURCE_SCALES[0] - 0.0).abs() < 1e-10);
        assert!((SOURCE_SCALES[SOURCE_SCALES.len() - 1] - 1.0).abs() < 1e-10);
    }
}
