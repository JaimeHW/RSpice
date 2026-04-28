//! Residual reconstruction, convergence checks, and nonlinear merit functions.

use super::*;

impl Engine {
    #[inline]
    pub(crate) fn residual_convergence_met(
        &self,
        matrix: &StaticMatrix,
        solution: &[Value],
        rhs: &[Value],
    ) -> bool {
        match matrix.scaled_residual_inf_norm(
            solution,
            rhs,
            self.current_abstol(),
            self.residual_reltol(),
        ) {
            Ok(norm) => norm.is_finite() && norm <= 1.0,
            Err(_) => false,
        }
    }

    pub(in crate::engine::convergence) fn nonlinear_residual_converged_with_linear_stamp<F>(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        mut linear_stamp: F,
    ) -> bool
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return false;
        }

        let mut rhs = vec![0.0; size];
        matrix.clear_values();
        linear_stamp(circuit, matrix, &mut rhs);
        self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, solution);
        self.residual_convergence_met(matrix, solution, &rhs)
    }

    pub(in crate::engine::convergence) fn nonlinear_residual_converged(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> bool {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_residual_converged_with_linear_stamp(
            circuit,
            matrix,
            solution,
            |circuit, matrix, rhs| {
                let node_count = circuit.num_nodes().min(rhs.len());
                for i in 0..node_count {
                    matrix.add(i, i, gmin_floor);
                }
                circuit.stamp_dc_direct(matrix, rhs);
            },
        )
    }

    pub(in crate::engine::convergence) fn nonlinear_residual_converged_scaled(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        source_scale: Value,
    ) -> bool {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_residual_converged_with_linear_stamp(
            circuit,
            matrix,
            solution,
            |circuit, matrix, rhs| {
                let node_count = circuit.num_nodes().min(rhs.len());
                for i in 0..node_count {
                    matrix.add(i, i, gmin_floor);
                }
                circuit.stamp_dc_direct_scaled(matrix, rhs, source_scale);
            },
        )
    }

    pub(in crate::engine::convergence) fn nonlinear_residual_converged_with_gmin(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        gmin: Value,
    ) -> bool {
        self.nonlinear_residual_converged_with_linear_stamp(
            circuit,
            matrix,
            solution,
            |circuit, matrix, rhs| {
                let node_count = circuit.num_nodes().min(rhs.len());
                for i in 0..node_count {
                    matrix.add(i, i, gmin);
                }
                circuit.stamp_dc_direct(matrix, rhs);
            },
        )
    }

    pub(in crate::engine::convergence) fn nonlinear_residual_converged_with_pseudo_transient(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        anchor_solution: &[Value],
        pseudo_conductance: Value,
    ) -> bool {
        self.nonlinear_residual_converged_with_linear_stamp(
            circuit,
            matrix,
            solution,
            |circuit, matrix, rhs| {
                for i in 0..rhs.len() {
                    matrix.add(i, i, 1e-12 + pseudo_conductance);
                    rhs[i] += pseudo_conductance * anchor_solution[i];
                }
                circuit.stamp_dc_direct(matrix, rhs);
            },
        )
    }

    pub(in crate::engine::convergence) fn nonlinear_merit_with_linear_stamp<F>(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        mut linear_stamp: F,
    ) -> Option<Value>
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return None;
        }

        let mut rhs = vec![0.0; size];
        matrix.clear_values();
        linear_stamp(circuit, matrix, &mut rhs);
        self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, solution);

        let next_solution = matrix.solve(&rhs).ok()?;
        Some(Self::step_l2_norm(solution, &next_solution))
    }

    pub(in crate::engine::convergence) fn nonlinear_merit(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> Option<Value> {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            let node_count = circuit.num_nodes().min(rhs.len());
            for i in 0..node_count {
                matrix.add(i, i, gmin_floor);
            }
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }

    pub(in crate::engine::convergence) fn nonlinear_merit_scaled(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        source_scale: Value,
    ) -> Option<Value> {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            let node_count = circuit.num_nodes().min(rhs.len());
            for i in 0..node_count {
                matrix.add(i, i, gmin_floor);
            }
            circuit.stamp_dc_direct_scaled(matrix, rhs, source_scale);
        })
    }

    pub(in crate::engine::convergence) fn nonlinear_merit_with_gmin(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        gmin: Value,
    ) -> Option<Value> {
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            let node_count = circuit.num_nodes().min(rhs.len());
            for i in 0..node_count {
                matrix.add(i, i, gmin);
            }
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }

    pub(in crate::engine::convergence) fn nonlinear_merit_with_pseudo_transient(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        anchor_solution: &[Value],
        pseudo_conductance: Value,
    ) -> Option<Value> {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            for i in 0..rhs.len() {
                matrix.add(i, i, gmin_floor + pseudo_conductance);
                rhs[i] += pseudo_conductance * anchor_solution[i];
            }
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }
}
