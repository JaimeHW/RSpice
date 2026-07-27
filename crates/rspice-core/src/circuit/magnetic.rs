//! Transient stamping for magnetically coupled and nonlinear inductors.
//!
//! Coupled inductor pairs and multi-winding transformers need their mutual
//! terms restamped each timestep, and Jiles-Atherton cores need their
//! effective inductance refreshed from the current solution before companion
//! stamping so hysteresis state reaches the MNA coefficients.

use super::*;

impl CircuitData {
    /// Stamp ordinary transient inductor companions while leaving Xyce Core
    /// branches to their complete nonlinear DAE stamp.
    pub fn stamp_transient_inductor_companions(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
        num_nodes: usize,
    ) {
        let core_bindings = &self.jiles_atherton_inductors;
        self.inductors.stamp_transient_companion_where(
            matrix,
            rhs,
            dt,
            coeff,
            num_nodes,
            |index| {
                core_bindings
                    .iter()
                    .any(|binding| binding.inductor_index == index && binding.device.is_xyce_core())
            },
        );
    }

    /// Whether the circuit contains a single-winding Xyce nonlinear Core.
    /// Core branches are nonlinear transient participants even though they
    /// own their accepted-state lifecycle outside `update_nonlinear`.
    pub fn has_xyce_core_inductors(&self) -> bool {
        self.jiles_atherton_inductors
            .iter()
            .any(|binding| binding.device.is_xyce_core())
    }

    /// Whether every standalone inductor in the circuit is represented by a
    /// Xyce Core binding.  OneStep can split this topology into its static
    /// Core `F` and constant-charge `Q` terms; ordinary and mutually coupled
    /// inductors still require their native history mapping.
    pub fn has_only_xyce_core_inductors(&self) -> bool {
        !self.inductors.names.is_empty()
            && self.inductors.names.len() == self.jiles_atherton_inductors.len()
            && self
                .jiles_atherton_inductors
                .iter()
                .all(|binding| binding.device.is_xyce_core())
    }

    /// Restamp the single-winding Xyce Core branch equations with the pure
    /// constitutive endpoint evaluated at the current Newton iterate.
    ///
    /// A Core's DAE has a constant vacuum charge coefficient and a nonlinear
    /// static factor `mid(P)`. Treating that factor as an accepted-step linear
    /// inductance loses the coupled Newton solve, especially when the
    /// constitutive mid-factor is negative near zero field. This routine
    /// replaces only the branch-row linearization; KCL rows remain the normal
    /// MNA inductor rows.
    pub fn stamp_xyce_core_transient_companion(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
        one_step_order2: bool,
    ) {
        for binding in &mut self.jiles_atherton_inductors {
            if !binding.device.is_xyce_core() {
                continue;
            }
            let index = binding.inductor_index;
            let Some(&l_slot) = self.inductors.inductances.get(index) else {
                continue;
            };
            let Some(&i_prev) = self.inductors.i_prev.get(index) else {
                continue;
            };
            let i_prev_prev = self
                .inductors
                .i_prev_prev
                .get(index)
                .copied()
                .unwrap_or(i_prev);
            let v_prev = self.inductors.v_prev.get(index).copied().unwrap_or(0.0);
            let branch = self.num_nodes + self.inductors.branch_indices[index];
            let current = solution.get(branch - 1).copied().unwrap_or(i_prev);
            let voltage = if self.inductors.node_pos[index] == 0 {
                0.0
            } else {
                solution
                    .get(self.inductors.node_pos[index] - 1)
                    .copied()
                    .unwrap_or(0.0)
            } - if self.inductors.node_neg[index] == 0 {
                0.0
            } else {
                solution
                    .get(self.inductors.node_neg[index] - 1)
                    .copied()
                    .unwrap_or(0.0)
            };
            let nominal = binding.device.nominal_inductance();
            if !nominal.is_finite() || nominal <= 0.0 || !dt.is_finite() || dt <= 0.0 {
                continue;
            }

            // Xyce carries the accepted constitutive factor into the next
            // transient history evaluation.  The generic inductance slot is
            // refreshed after commit for charge-companion cancellation and
            // may be recomputed from a different magnetic endpoint near a
            // turning point; it is therefore not authoritative for F-history.
            let accepted_mid = binding.device.xyce_core_accepted_mid();
            let previous_mid = if accepted_mid.is_finite() && accepted_mid.abs() > 1.0e-12 {
                accepted_mid
            } else {
                1.0
            };
            let static_scale = if one_step_order2 { 0.5 } else { 1.0 };
            let carried_mag_update = binding.device.xyce_core_mag_update();
            // OneStep uses the physical DAE charge difference directly:
            // `(Q_{n+1}-Q_n)/dt`. The ordinary trapezoidal companion has a
            // `2/dt` conductance, so its coefficient must not leak into the
            // Core's order-2 DAE row.
            let charge_coeff = if one_step_order2 { 1.0 } else { coeff.coeff_g };
            let residual = |trial_current: Value, trial_voltage: Value| {
                let trial = binding.device.xyce_core_trial_with_update(
                    trial_current,
                    trial_voltage,
                    carried_mag_update,
                )?;
                let mut charge_derivative = charge_coeff * (trial_current - i_prev);
                if !one_step_order2 && coeff.needs_two_history {
                    charge_derivative += coeff.coeff_v_n_minus_1 * (i_prev - i_prev_prev);
                }
                charge_derivative *= nominal / dt;
                let previous_static_voltage =
                    if previous_mid.is_finite() && previous_mid.abs() > 1.0e-12 {
                        v_prev / previous_mid
                    } else {
                        0.0
                    };
                let history = if coeff.needs_current_history && !one_step_order2 {
                    previous_static_voltage
                } else {
                    0.0
                };
                if !trial.mid.is_finite() {
                    return None;
                }
                // MutIndNonLin contributes Q=L0*I and F=-V/mid to the DAE.
                // Keep the branch residual in that source-equivalent
                // orientation so its Jacobian retains the same conditioning
                // as Xyce's device row near a constitutive zero crossing.
                let static_branch = static_scale * trial_voltage / trial.mid;
                let f0 = if one_step_order2 {
                    // OneStep's `alpha_s=-1` makes its Newton matrix the
                    // physical DAE Jacobian, dQ/dx + dF/dx.  Keep this branch
                    // in that source orientation (F=-V/mid); the accepted
                    // static F history is added by the outer OneStep pass.
                    charge_derivative - static_branch + history
                } else {
                    // Native companion correction uses the negative physical
                    // DAE residual, V/mid - dQ/dt.
                    static_branch - charge_derivative + history
                };
                Some((f0, trial))
            };
            let Some((f0, trial)) = residual(current, voltage) else {
                continue;
            };
            if !f0.is_finite() {
                continue;
            }
            binding
                .device
                .set_xyce_core_mag_update(trial.magnetization_update);
            binding
                .device
                .cache_xyce_core_trial(current, voltage, trial);
            let jacobian_magnetization = trial.latest_magnetization;
            let d_mid_d_current =
                binding
                    .device
                    .xyce_core_dmid_d_current(current, voltage, jacobian_magnetization);
            let Some(d_mid_d_current) = d_mid_d_current else {
                continue;
            };
            let mid = trial.mid;
            let (d_current, d_voltage) = if one_step_order2 {
                (
                    static_scale * voltage * d_mid_d_current / (mid * mid)
                        + charge_coeff * nominal / dt,
                    -static_scale / mid,
                )
            } else {
                (
                    -static_scale * voltage * d_mid_d_current / (mid * mid)
                        - charge_coeff * nominal / dt,
                    static_scale / mid,
                )
            };
            if !d_current.is_finite() || !d_voltage.is_finite() {
                continue;
            }

            let desired_rhs = -f0 + d_current * current + d_voltage * voltage;
            if self.inductors.node_pos[index] > 0 {
                matrix.add(
                    branch - 1,
                    self.inductors.node_pos[index] - 1,
                    d_voltage - 1.0,
                );
                if one_step_order2 {
                    // The ordinary companion stamp is assembled after the
                    // OneStep static half-scale pass, so its KCL incidence
                    // is still unit-scaled.  Xyce scales the complete static
                    // F vector (including the inductor's KCL current) by
                    // one half; reduce that incidence here while retaining
                    // the unscaled Q contribution in the branch row.
                    matrix.add(self.inductors.node_pos[index] - 1, branch - 1, -0.5);
                }
            }
            if self.inductors.node_neg[index] > 0 {
                matrix.add(
                    branch - 1,
                    self.inductors.node_neg[index] - 1,
                    -d_voltage + 1.0,
                );
                if one_step_order2 {
                    matrix.add(self.inductors.node_neg[index] - 1, branch - 1, 0.5);
                }
            }
            let old_req = coeff.inductor_req(l_slot, dt);
            let old_veq = coeff.inductor_veq(l_slot, dt, i_prev, i_prev_prev, v_prev);
            matrix.add(branch - 1, branch - 1, d_current + old_req);
            rhs[branch - 1] += desired_rhs + old_veq;
        }
    }

    /// Stamp the accepted static `F` contribution of each Xyce Core.
    ///
    /// OneStep order-2 stores the accepted `F-B` vector separately from the
    /// transient `Q` companion.  Core branches are not part of the ordinary
    /// linear-inductor static stamp, so include their KCL current and
    /// nonlinear branch voltage term explicitly in that history probe.
    pub fn stamp_xyce_core_static_residual(&self, matrix: &mut StaticMatrix, solution: &[Value]) {
        for binding in &self.jiles_atherton_inductors {
            if !binding.device.is_xyce_core() {
                continue;
            }
            let index = binding.inductor_index;
            let branch = self.num_nodes + self.inductors.branch_indices[index];
            let current = solution.get(branch - 1).copied().unwrap_or(0.0);
            let voltage = if self.inductors.node_pos[index] == 0 {
                0.0
            } else {
                solution
                    .get(self.inductors.node_pos[index] - 1)
                    .copied()
                    .unwrap_or(0.0)
            } - if self.inductors.node_neg[index] == 0 {
                0.0
            } else {
                solution
                    .get(self.inductors.node_neg[index] - 1)
                    .copied()
                    .unwrap_or(0.0)
            };
            let mid = binding.device.xyce_core_static_mid(current, voltage);
            if !mid.is_finite() || mid.abs() <= 1.0e-12 {
                continue;
            }
            if self.inductors.node_pos[index] > 0 {
                matrix.add(self.inductors.node_pos[index] - 1, branch - 1, 1.0);
                matrix.add(branch - 1, self.inductors.node_pos[index] - 1, -1.0 / mid);
            }
            if self.inductors.node_neg[index] > 0 {
                matrix.add(self.inductors.node_neg[index] - 1, branch - 1, -1.0);
                matrix.add(branch - 1, self.inductors.node_neg[index] - 1, 1.0 / mid);
            }
        }
    }

    /// Refresh effective inductance values for all Jiles-Atherton inductors.
    ///
    /// Call this with the latest solution vector before transient companion
    /// stamping so nonlinear core state updates feed into the MNA coefficients.
    pub fn refresh_jiles_atherton_inductances(&mut self, solution: &[Value]) {
        use crate::device::NonlinearDevice;

        let num_nodes = self.num_nodes;
        for idx in 0..self.jiles_atherton_inductors.len() {
            let (inductor_index, l_eff) = {
                let binding = &mut self.jiles_atherton_inductors[idx];
                if binding.device.is_xyce_core() {
                    // Xyce solves the magnetic state as part of its coupled
                    // M/R DAE.  The native companion uses the last accepted
                    // differential inductance while Newton probes the next
                    // electrical state; mutating M during those probes makes
                    // the residual path-dependent and prevents convergence.
                    continue;
                }
                let branch_matrix_index = num_nodes + binding.branch_ordinal;
                binding.device.set_branch_index(branch_matrix_index);
                binding.device.update(solution);
                (
                    binding.inductor_index,
                    binding.device.effective_inductance(),
                )
            };

            if let Some(slot) = self.inductors.inductances.get_mut(inductor_index)
                && l_eff.is_finite()
                && l_eff > 0.0
            {
                *slot = l_eff.max(1e-18);
            }
        }
    }

    /// Advance Xyce Core states from an accepted transient solution and make
    /// the resulting differential inductance active for the next step.
    pub fn commit_xyce_core_inductances(&mut self, solution: &[Value], dt: Value) {
        let num_nodes = self.num_nodes;
        for binding in &mut self.jiles_atherton_inductors {
            if !binding.device.is_xyce_core() {
                continue;
            }
            let branch_matrix_index = num_nodes + binding.branch_ordinal;
            binding.device.set_branch_index(branch_matrix_index);
            binding.device.commit_xyce_core_solution(solution, dt);
            if let Some(slot) = self.inductors.inductances.get_mut(binding.inductor_index) {
                let l_eff = binding.device.effective_inductance();
                // A negative Xyce mid-factor is a valid constitutive state,
                // but the native companion cannot safely switch polarity
                // after an accepted step.  Retain the last positive Q
                // coefficient until the constitutive factor becomes usable.
                if l_eff.is_finite()
                    && (l_eff > 0.0 || binding.device.is_xyce_core_level2())
                    && l_eff.abs() > 1.0e-18
                {
                    *slot = l_eff;
                }
            }
        }
    }

    /// Stamp coupled inductor mutual-coupling overlays for transient analysis.
    ///
    /// The standalone inductors stamp their own self-inductance rows; each
    /// pair adds only the -r12 cross terms and mutual history sources.
    pub fn stamp_coupled_inductor_pairs_transient(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.coupled_inductor_pairs {
            let br1 = self.num_nodes + binding.branch1_ordinal;
            let br2 = self.num_nodes + binding.branch2_ordinal;
            binding
                .device
                .stamp_transient_mutual(br1, br2, dt, coeff, &mut stamper);
        }
    }

    /// Stamp multi-winding transformer companion models for transient analysis.
    pub fn stamp_multi_winding_transformers_transient(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.multi_winding_transformers {
            binding
                .device
                .stamp_transient_companion(dt, coeff, &mut stamper, &mut []);
        }
    }

    /// Replace inductive branch entries in `b - A*x` with DAE residuals
    /// evaluated from current differences. This is the correction-form
    /// counterpart to the absolute companion stamps above.
    pub fn stabilize_inductor_transient_correction_rhs(
        &self,
        correction_rhs: &mut [Value],
        iterate: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let core_indices = self
            .jiles_atherton_inductors
            .iter()
            .filter(|binding| binding.device.is_xyce_core())
            .map(|binding| binding.inductor_index)
            .collect::<Vec<_>>();
        self.inductors.overwrite_transient_correction_rhs_excluding(
            correction_rhs,
            iterate,
            dt,
            coeff,
            self.num_nodes,
            &core_indices,
        );
        for binding in &self.coupled_inductor_pairs {
            let branch1 = self.num_nodes + binding.branch1_ordinal;
            let branch2 = self.num_nodes + binding.branch2_ordinal;
            binding.device.add_transient_mutual_correction_rhs(
                branch1,
                branch2,
                correction_rhs,
                iterate,
                dt,
                coeff,
            );
        }
        for binding in &self.multi_winding_transformers {
            binding
                .device
                .overwrite_transient_correction_rhs(correction_rhs, iterate, dt, coeff);
        }
    }

    /// Update coupled inductor transient history from an accepted solution.
    pub fn update_coupled_inductor_pair_state(&mut self, solution: &[Value]) {
        let num_nodes = self.num_nodes;
        for binding in &mut self.coupled_inductor_pairs {
            let br1 = num_nodes + binding.branch1_ordinal;
            let br2 = num_nodes + binding.branch2_ordinal;
            binding
                .device
                .update_state_with_branches(solution, br1, br2);
        }
    }

    /// Update multi-winding transformer transient history from an accepted solution.
    pub fn update_multi_winding_transformer_state(&mut self, solution: &[Value]) {
        for binding in &mut self.multi_winding_transformers {
            binding.device.update_state_from_solution(solution);
        }
    }
}
