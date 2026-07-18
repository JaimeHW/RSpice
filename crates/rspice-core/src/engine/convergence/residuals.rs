//! Residual reconstruction, convergence checks, and nonlinear merit functions.

use super::*;

impl Engine {
    fn nonlinear_probe_residual_converged(
        &self,
        circuit: &CircuitData,
        probe: &mut StaticMatrix,
        solution: &[Value],
        rhs: &[Value],
    ) -> bool {
        if self.config.spice_dialect == crate::engine::SpiceDialect::Xyce {
            // Xyce 7.10's DC NOX status test accepts the candidate when
            // the unscaled infinity norm of F=A*x-b is below RHSTOL
            // (1e-6 by default). It deliberately does not require the
            // stricter fixed-point polish used by native RSpice.
            probe
                .raw_residual_inf_norm(solution, rhs)
                .is_ok_and(|norm| norm.is_finite() && norm < 1.0e-6)
        } else {
            self.residual_probe_fixed_point_converged(circuit, probe, solution, rhs)
        }
    }

    pub(in crate::engine) fn dc_static_probe_polished_solution(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> Option<Vec<Value>> {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|value| !value.is_finite()) {
            return None;
        }

        let snapshot = circuit.nonlinear_state_snapshot();
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let node_count = circuit.num_nodes().min(size);
        let mut polished_solution = None;
        let accepted = matrix.with_probe_values(|probe, rhs| {
            Self::stamp_nodal_gmin(circuit, probe, gmin_floor);
            circuit.stamp_dc_direct(probe, rhs);
            if self
                .try_stamp_static_probe_nonlinear_devices_for_dc(circuit, probe, rhs, solution)
                .is_err()
            {
                return false;
            }

            let Ok(next_solution) = probe.solve(rhs) else {
                return false;
            };
            if next_solution.iter().any(|value| !value.is_finite()) {
                return false;
            }

            let checked_nodes = node_count.min(next_solution.len());
            let voltage_abstol = self.voltage_abstol();
            let nodes_fixed = solution[..checked_nodes]
                .iter()
                .zip(next_solution[..checked_nodes].iter())
                .all(|(current, next)| (next - current).abs() <= voltage_abstol);
            if nodes_fixed {
                polished_solution = Some(next_solution);
            }
            nodes_fixed
        });
        circuit.restore_nonlinear_state(snapshot);

        if accepted {
            if let Some(ref polished) = polished_solution {
                self.update_device_states_for_dc(circuit, polished);
            }
            polished_solution
        } else {
            None
        }
    }

    pub(in crate::engine::convergence) fn residual_probe_fixed_point_converged(
        &self,
        circuit: &CircuitData,
        probe: &mut StaticMatrix,
        solution: &[Value],
        rhs: &[Value],
    ) -> bool {
        if !self.residual_convergence_met(circuit, probe, solution, rhs) {
            return false;
        }

        let Ok(next_solution) = probe.solve(rhs) else {
            return false;
        };
        let node_count = circuit
            .num_nodes()
            .min(solution.len())
            .min(next_solution.len());
        let voltage_abstol = self.voltage_abstol();
        let fixed_point_converged = solution[..node_count]
            .iter()
            .zip(next_solution[..node_count].iter())
            .all(|(current, next)| (next - current).abs() <= voltage_abstol);
        fixed_point_converged || circuit.has_jfet_gate_generation_branches()
    }

    #[inline]
    pub(crate) fn residual_inf_norm(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        rhs: &[Value],
    ) -> Option<Value> {
        let node_rows = circuit.num_nodes().min(rhs.len());
        let current_abstol = self.current_abstol();
        let voltage_abstol = self.voltage_abstol();
        // MNA rows before `num_nodes` are KCL equations; branch rows are
        // voltage constraints for current unknowns.
        matrix
            .scaled_residual_inf_norm_by_row(solution, rhs, self.residual_reltol(), |row| {
                if row < node_rows {
                    current_abstol
                } else {
                    voltage_abstol
                }
            })
            .ok()
            .filter(|norm| norm.is_finite())
    }

    #[inline]
    pub(crate) fn residual_convergence_met(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        rhs: &[Value],
    ) -> bool {
        match self.residual_inf_norm(circuit, matrix, solution, rhs) {
            Some(norm) => norm <= 1.0,
            None => false,
        }
    }

    pub(in crate::engine::convergence) fn nonlinear_residual_converged_with_linear_stamp_for_operating_point<
        F,
    >(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
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
            if self
                .try_stamp_static_probe_nonlinear_devices_for_operating_point(
                    circuit,
                    probe,
                    rhs,
                    solution,
                    time,
                    analysis,
                    junction_gmin,
                )
                .is_err()
            {
                return false;
            }
            self.nonlinear_probe_residual_converged(circuit, probe, solution, rhs)
        });
        circuit.restore_nonlinear_state(snapshot);
        converged
    }

    pub(in crate::engine::convergence) fn try_nonlinear_residual_converged_with_linear_stamp_and_junction_gmin<
        F,
    >(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        junction_gmin: Value,
        mut linear_stamp: F,
    ) -> Result<bool, SimulationError>
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return Ok(false);
        }

        let snapshot = circuit.nonlinear_state_snapshot();
        let mut stamp_error = None;
        let converged = matrix.with_probe_values(|probe, rhs| {
            linear_stamp(circuit, probe, rhs);
            if let Err(err) = self
                .try_stamp_static_probe_nonlinear_devices_for_dc_with_junction_gmin(
                    circuit,
                    probe,
                    rhs,
                    solution,
                    junction_gmin,
                )
            {
                stamp_error = Some(err);
                return false;
            }
            self.nonlinear_probe_residual_converged(circuit, probe, solution, rhs)
        });
        circuit.restore_nonlinear_state(snapshot);
        if let Some(err) = stamp_error {
            return Err(err);
        }
        Ok(converged)
    }

    pub(in crate::engine::convergence) fn try_nonlinear_residual_converged(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> Result<bool, SimulationError> {
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        self.try_nonlinear_residual_converged_with_linear_stamp_and_junction_gmin(
            circuit,
            matrix,
            solution,
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target),
            |circuit, matrix, rhs| {
                Self::stamp_nodal_gmin(circuit, matrix, gmin_floor);
                circuit.stamp_dc_direct(matrix, rhs);
            },
        )
    }

    pub(in crate::engine::convergence) fn try_nonlinear_residual_converged_scaled(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        source_scale: Value,
    ) -> Result<bool, SimulationError> {
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        self.try_nonlinear_residual_converged_with_linear_stamp_and_junction_gmin(
            circuit,
            matrix,
            solution,
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target),
            |circuit, matrix, rhs| {
                Self::stamp_nodal_gmin(circuit, matrix, gmin_floor);
                circuit.stamp_dc_direct_scaled(matrix, rhs, source_scale);
            },
        )
    }

    pub(in crate::engine::convergence) fn try_nonlinear_residual_converged_with_gmin(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        gmin: Value,
    ) -> Result<bool, SimulationError> {
        let junction_gmin = self.effective_device_junction_gmin(gmin);
        self.try_nonlinear_residual_converged_with_linear_stamp_and_junction_gmin(
            circuit,
            matrix,
            solution,
            junction_gmin,
            |circuit, matrix, rhs| {
                Self::stamp_nodal_gmin(circuit, matrix, gmin);
                circuit.stamp_dc_direct(matrix, rhs);
            },
        )
    }

    pub(in crate::engine::convergence) fn try_nonlinear_residual_converged_with_pseudo_transient(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        anchor_solution: &[Value],
        pseudo_conductance: Value,
    ) -> Result<bool, SimulationError> {
        self.try_nonlinear_residual_converged_with_linear_stamp_and_junction_gmin(
            circuit,
            matrix,
            solution,
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target),
            |circuit, matrix, rhs| {
                for i in 0..rhs.len() {
                    matrix.add(i, i, pseudo_conductance);
                    rhs[i] += pseudo_conductance * anchor_solution[i];
                }
                Self::stamp_matrix_conditioning_diagonal(circuit, matrix, rhs.len(), 1e-12);
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
        linear_stamp: F,
    ) -> Option<Value>
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        self.nonlinear_merit_with_linear_stamp_for_operating_point(
            circuit,
            matrix,
            solution,
            0.0,
            crate::xspice::AnalysisType::DcOp,
            junction_gmin,
            linear_stamp,
        )
    }

    pub(in crate::engine::convergence) fn nonlinear_merit_with_linear_stamp_for_operating_point<F>(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
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
            if self
                .try_stamp_static_probe_nonlinear_devices_for_operating_point(
                    circuit,
                    probe,
                    rhs,
                    solution,
                    time,
                    analysis,
                    junction_gmin,
                )
                .is_err()
            {
                return None;
            }

            self.residual_inf_norm(circuit, probe, solution, rhs)
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
            Self::stamp_nodal_gmin(circuit, matrix, gmin_floor);
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
            Self::stamp_nodal_gmin(circuit, matrix, gmin_floor);
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
                Self::stamp_nodal_gmin(circuit, matrix, gmin);
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
                matrix.add(i, i, pseudo_conductance);
                rhs[i] += pseudo_conductance * anchor_solution[i];
            }
            Self::stamp_matrix_conditioning_diagonal(circuit, matrix, rhs.len(), gmin_floor);
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }
}
