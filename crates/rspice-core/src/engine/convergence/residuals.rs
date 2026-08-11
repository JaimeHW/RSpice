//! Residual reconstruction, convergence checks, and nonlinear merit functions.

use super::*;

impl Engine {
    /// Reconstruct the accepted DC equations both with and without the
    /// simulator's final global nodal-conditioning diagonal.
    ///
    /// This is the authoritative post-solve floating-bias check. It evaluates
    /// the installed circuit—not syntax heuristics—so generated devices,
    /// XSPICE, controlled and behavioral sources, MOS gate isolation, and
    /// dialect-specific capacitor-IC constraints all participate through
    /// their ordinary DC stamps. An authored RSHUNT remains in the physical
    /// system. A row is reported only when removing nodal GMIN moves that same
    /// electrical KCL equation from accepted to rejected; reconstruction
    /// mismatch therefore cannot be misclassified as topology. Device trial
    /// state is restored after the probe.
    pub(in crate::engine) fn physical_dc_kcl_violation_nodes(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> Result<Vec<String>, SimulationError> {
        if solution.len() != circuit.matrix_size()
            || solution.iter().any(|value| !value.is_finite())
        {
            return Err(SimulationError::Circuit(
                "accepted DC solution is incompatible with the assembled circuit".to_string(),
            ));
        }

        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        if gmin_floor == 0.0 {
            return Ok(Vec::new());
        }

        let snapshot = circuit.nonlinear_state_snapshot();
        let result = matrix.with_probe_values(|probe, rhs| {
            circuit.stamp_dc_direct(probe, rhs);
            self.try_stamp_static_probe_nonlinear_devices_for_dc(circuit, probe, rhs, solution)?;
            self.conditioning_dependent_kcl_violation_nodes_from_probe(
                circuit, probe, rhs, solution, gmin_floor,
            )
        });
        circuit.restore_nonlinear_state(snapshot);
        result
    }

    /// Audit the exact t=0 transient operating-point equation family that
    /// accepted `solution`. Recovery seeds have no accepted contract and are
    /// intentionally never routed here.
    pub(in crate::engine) fn physical_transient_operating_point_kcl_violation_nodes(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        time: Value,
        contract: AcceptedTransientOperatingPointContract,
    ) -> Result<Vec<String>, SimulationError> {
        if solution.len() != circuit.matrix_size()
            || solution.iter().any(|value| !value.is_finite())
        {
            return Err(SimulationError::Circuit(
                "accepted transient operating-point solution is incompatible with the assembled circuit"
                    .to_string(),
            ));
        }
        if !contract.nodal_gmin.is_finite() || contract.nodal_gmin < 0.0 {
            return Err(SimulationError::Circuit(
                "accepted transient operating-point contract has an invalid nodal GMIN".to_string(),
            ));
        }
        if contract.nodal_gmin == 0.0 {
            return Ok(Vec::new());
        }

        let snapshot = circuit.nonlinear_state_snapshot();
        let result = matrix.with_probe_values(|probe, rhs| {
            if let Some(junction_gmin) = contract.junction_gmin {
                if !junction_gmin.is_finite() || junction_gmin < 0.0 {
                    return Err(SimulationError::Circuit(
                        "accepted transient operating-point contract has an invalid junction GMIN"
                            .to_string(),
                    ));
                }
                circuit.refresh_jiles_atherton_inductances(solution);
                match contract.linear_system {
                    TransientOperatingPointLinearSystem::IdealInductorShorts => {
                        Self::stamp_transient_operating_point_linear(
                            circuit, probe, rhs, time, 0.0, false,
                        );
                    }
                    TransientOperatingPointLinearSystem::CurrentSeededInductors => {
                        Self::stamp_transient_current_seed_linear(
                            circuit, probe, rhs, time, 0.0, false,
                        );
                    }
                }
                self.try_stamp_static_probe_nonlinear_devices_for_operating_point(
                    circuit,
                    probe,
                    rhs,
                    solution,
                    time,
                    crate::xspice::AnalysisType::Transient,
                    junction_gmin,
                )?;
            } else {
                Self::stamp_linear_transient_operating_point_system(
                    circuit,
                    probe,
                    rhs,
                    time,
                    0.0,
                    contract.linear_system,
                );
            }

            self.conditioning_dependent_kcl_violation_nodes_from_probe(
                circuit,
                probe,
                rhs,
                solution,
                contract.nodal_gmin,
            )
        });
        circuit.restore_nonlinear_state(snapshot);
        result
    }

    fn conditioning_dependent_kcl_violation_nodes_from_probe(
        &self,
        circuit: &CircuitData,
        probe: &mut StaticMatrix,
        rhs: &[Value],
        solution: &[Value],
        gmin_floor: Value,
    ) -> Result<Vec<String>, SimulationError> {
        let node_count = circuit.num_nodes().min(solution.len());
        let physical = probe
            .scaled_residual_norms_by_row_prefix(
                solution,
                rhs,
                self.residual_reltol(),
                node_count,
                |_| self.current_abstol(),
            )
            .map_err(SimulationError::Solver)?;

        let mut singular_components = Vec::new();
        for (row, normalized_residual) in physical.iter().copied().enumerate() {
            if normalized_residual <= 1.0 || circuit.is_non_electrical_state_matrix_index(row) {
                continue;
            }
            let Some(component) = circuit.dc_floating_component_for_matrix_row(row) else {
                continue;
            };
            if singular_components
                .iter()
                .any(|candidate| *candidate == component)
            {
                continue;
            }

            let component_rows = circuit.dc_floating_component_matrix_rows(component);
            if probe
                .principal_submatrix_is_singular(&component_rows)
                .unwrap_or(false)
            {
                singular_components.push(component);
            }
        }

        Self::stamp_nodal_gmin(circuit, probe, gmin_floor);
        let conditioned = probe
            .scaled_residual_norms_by_row_prefix(
                solution,
                rhs,
                self.residual_reltol(),
                node_count,
                |_| self.current_abstol(),
            )
            .map_err(SimulationError::Solver)?;

        let names = circuit.node_names_sorted();
        Ok(physical
            .iter()
            .zip(conditioned.iter())
            .enumerate()
            .filter(|(row, (physical, conditioned))| {
                !circuit.is_non_electrical_state_matrix_index(*row)
                    && **physical > 1.0
                    && **conditioned <= 1.0
            })
            .filter_map(|(row, _)| {
                let component = circuit.dc_floating_component_for_matrix_row(row)?;
                if !singular_components
                    .iter()
                    .any(|candidate| *candidate == component)
                {
                    return None;
                }
                let name = names
                    .get(row)
                    .map(|name| name.to_ascii_uppercase())
                    .unwrap_or_else(|| format!("<node #{}>", row + 1));
                Some(name)
            })
            .collect())
    }

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
        if self.config.spice_dialect == crate::engine::SpiceDialect::Xyce {
            // The Xyce NOX contract returns the iterate accepted by its
            // weighted-update and RHSTOL status tests.  Advancing that iterate
            // through RSpice's native fixed-point polish changes observable DC
            // results even though the Xyce solve has already converged.  Keep
            // native polishing for the other dialects, where it is part of
            // RSpice's higher-accuracy result contract.
            return None;
        }
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

        if accepted { polished_solution } else { None }
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

            if self.config.spice_dialect == crate::engine::SpiceDialect::Xyce {
                // NOX globalization compares the physical residual norm.
                // A tolerance-normalized residual asymptotes to 1/RELTOL for
                // every large disequilibrium, which makes substantially better
                // trial steps appear tied and can pin line search at its old
                // iterate on large current-driven networks.
                // NOX's line-search merit is Group::getNormF(), the
                // Euclidean residual norm. Its convergence status separately
                // applies the unscaled infinity norm checked above.
                probe
                    .raw_residual_norms(solution, rhs)
                    .ok()
                    .map(|(_, l2_norm)| l2_norm)
                    .filter(|norm| norm.is_finite())
            } else {
                self.residual_inf_norm(circuit, probe, solution, rhs)
            }
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

#[cfg(all(test, feature = "veriloga-model-diode-cmc"))]
mod tests {
    use super::*;
    use crate::netlist::Netlist;

    fn generated_diode_current(circuit: &CircuitData) -> Value {
        circuit
            .device_op_report()
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case("d1"))
            .and_then(|entry| {
                entry
                    .params
                    .iter()
                    .find_map(|(name, value)| (*name == "id").then_some(*value))
            })
            .expect("generated diode reports its anode current")
    }

    #[test]
    fn accepted_dc_polish_refreshes_generated_terminal_current_at_returned_solution() {
        let netlist = Netlist::parse(
            r#"
v1 a 0 dc 0.6
d1 a 0 dcmc
.model dcmc d level=2002
.op
.end
"#,
        )
        .expect("generated nonlinear diode fixture parses");
        let mut engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("generated nonlinear diode circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("generated nonlinear diode operating point converges");

        // Give the static proof a deliberately nearby candidate. The voltage
        // source returns the node to its accepted value, while the nonlinear
        // generated current differs at the candidate and polished points.
        engine.config.convergence_config.voltage_abstol = 1.0e-5;
        let anode = circuit
            .node_names_sorted()
            .iter()
            .position(|name| name.eq_ignore_ascii_case("a"))
            .expect("anode node exists");
        let mut candidate = solution.clone();
        candidate[anode] += 1.0e-7;
        matrix.with_probe_values(|probe, rhs| {
            engine
                .try_stamp_static_probe_nonlinear_devices_for_dc(
                    &mut circuit,
                    probe,
                    rhs,
                    &candidate,
                )
                .expect("candidate observation succeeds");
        });
        let candidate_current = generated_diode_current(&circuit);

        let polished = engine
            .dc_static_probe_polished_solution(&mut circuit, &mut matrix, &candidate)
            .expect("nearby nonlinear candidate is polished");
        engine
            .try_observe_dc_operating_point(&mut circuit, &mut matrix, &polished)
            .expect("accepted polished point is observed");
        let polished_current = generated_diode_current(&circuit);

        assert!(
            (polished[anode] - candidate[anode]).abs() > 0.0,
            "polish must move the nonlinear candidate"
        );
        assert!(
            (polished_current - candidate_current).abs() > f64::EPSILON,
            "accepted OP current must be observed at the returned solution, not retained from the proof candidate"
        );
    }

    #[test]
    fn rejected_dc_polish_fallback_refreshes_generated_terminal_current() {
        let netlist = Netlist::parse(
            r#"
v1 a 0 dc 0.6
d1 a 0 dcmc
.model dcmc d level=2002
.op
.end
"#,
        )
        .expect("generated nonlinear diode fixture parses");
        let mut engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("generated nonlinear diode circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("generated nonlinear diode operating point converges");

        let anode = circuit
            .node_names_sorted()
            .iter()
            .position(|name| name.eq_ignore_ascii_case("a"))
            .expect("anode node exists");
        let current_at_solution = generated_diode_current(&circuit);
        let mut fallback = solution;
        fallback[anode] += 1.0e-7;

        // An exact-zero polish tolerance rejects the nearby vector, so the
        // caller will retain it as the fallback result.
        engine.config.convergence_config.voltage_abstol = 1.0e-15;
        assert!(
            engine
                .dc_static_probe_polished_solution(&mut circuit, &mut matrix, &fallback)
                .is_none(),
            "nearby fallback must be rejected by exact fixed-point polish"
        );
        engine
            .try_observe_dc_operating_point(&mut circuit, &mut matrix, &fallback)
            .expect("fallback point is observed");
        let fallback_current = generated_diode_current(&circuit);

        assert!(
            (fallback_current - current_at_solution).abs() > f64::EPSILON,
            "fallback OP current must be refreshed at the returned vector"
        );
    }
}
