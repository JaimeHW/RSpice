//! Residual reconstruction, convergence checks, and nonlinear merit functions.

use super::*;

impl Engine {
    #[inline]
    pub(crate) fn residual_convergence_met(
        &self,
        matrix: &mut StaticMatrix,
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
        linear_stamp: F,
    ) -> bool
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let junction_gmin =
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target);
        self.nonlinear_residual_converged_with_linear_stamp_and_junction_gmin(
            circuit,
            matrix,
            solution,
            junction_gmin,
            linear_stamp,
        )
    }

    pub(in crate::engine::convergence) fn nonlinear_residual_converged_with_linear_stamp_and_junction_gmin<
        F,
    >(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        junction_gmin: Value,
        mut linear_stamp: F,
    ) -> bool
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return false;
        }

        let snapshot = circuit.nonlinear_state_snapshot();
        let converged = matrix.with_probe_values(|probe, rhs| {
            linear_stamp(circuit, probe, rhs);
            self.stamp_static_probe_nonlinear_devices_for_dc_with_junction_gmin(
                circuit,
                probe,
                rhs,
                solution,
                junction_gmin,
            );
            self.residual_convergence_met(probe, solution, rhs)
        });
        circuit.restore_nonlinear_state(snapshot);
        converged
    }

    pub(in crate::engine::convergence) fn nonlinear_residual_converged(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> bool {
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
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
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
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
        let junction_gmin = self.effective_device_junction_gmin(gmin);
        self.nonlinear_residual_converged_with_linear_stamp_and_junction_gmin(
            circuit,
            matrix,
            solution,
            junction_gmin,
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
        linear_stamp: F,
    ) -> Option<Value>
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let junction_gmin =
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target);
        self.nonlinear_merit_with_linear_stamp_and_junction_gmin(
            circuit,
            matrix,
            solution,
            junction_gmin,
            linear_stamp,
        )
    }

    pub(in crate::engine::convergence) fn nonlinear_merit_with_linear_stamp_and_junction_gmin<F>(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        junction_gmin: Value,
        mut linear_stamp: F,
    ) -> Option<Value>
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return None;
        }

        let snapshot = circuit.nonlinear_state_snapshot();
        let merit = matrix.with_probe_values(|probe, rhs| {
            linear_stamp(circuit, probe, rhs);
            self.stamp_static_probe_nonlinear_devices_for_dc_with_junction_gmin(
                circuit,
                probe,
                rhs,
                solution,
                junction_gmin,
            );

            probe
                .scaled_residual_inf_norm(
                    solution,
                    rhs,
                    self.current_abstol(),
                    self.residual_reltol(),
                )
                .ok()
                .filter(|norm| norm.is_finite())
        });
        circuit.restore_nonlinear_state(snapshot);
        merit
    }

    pub(in crate::engine::convergence) fn nonlinear_merit(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> Option<Value> {
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
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
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
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
        let junction_gmin = self.effective_device_junction_gmin(gmin);
        self.nonlinear_merit_with_linear_stamp_and_junction_gmin(
            circuit,
            matrix,
            solution,
            junction_gmin,
            |circuit, matrix, rhs| {
                let node_count = circuit.num_nodes().min(rhs.len());
                for i in 0..node_count {
                    matrix.add(i, i, gmin);
                }
                circuit.stamp_dc_direct(matrix, rhs);
            },
        )
    }

    pub(in crate::engine::convergence) fn nonlinear_merit_with_pseudo_transient(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        anchor_solution: &[Value],
        pseudo_conductance: Value,
    ) -> Option<Value> {
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            for i in 0..rhs.len() {
                matrix.add(i, i, gmin_floor + pseudo_conductance);
                rhs[i] += pseudo_conductance * anchor_solution[i];
            }
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }
}
