//! VBIC transient hidden-state snapshot helpers.

use super::*;

impl Engine {
    #[inline]
    pub(super) fn rebalance_vbic_dynamic_thermal_state(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        snapshot: &mut crate::device::semiconductor::BjtChargeSnapshot,
    ) {
        let mut internal = snapshot.reduction.internal_voltages;
        let original_vrth = internal[BJT_THERMAL_STATE_INDEX];
        let minimum_vrth = bjt.minimum_thermal_rise();
        let mut best_internal = internal;
        let mut best_residual = Value::INFINITY;

        for _ in 0..8 {
            let (residual, derivative) = Self::vbic_transient_thermal_residual_and_derivative(
                bjt,
                vc,
                vb,
                ve,
                vs,
                internal,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            );
            let residual_abs = residual.abs();
            if residual_abs.is_finite() && residual_abs < best_residual {
                best_residual = residual_abs;
                best_internal = internal;
            }
            if !residual.is_finite() || !derivative.is_finite() || derivative.abs() < 1e-18 {
                break;
            }
            if residual_abs < 1e-12 {
                break;
            }

            let current_vrth = internal[BJT_THERMAL_STATE_INDEX];
            let max_step = (current_vrth - minimum_vrth + 10.0).max(1.0) * 0.5;
            let step = (-residual / derivative).clamp(-max_step, max_step);
            if step.abs() < 1e-12 {
                break;
            }

            let mut alpha = 1.0;
            let mut accepted = false;
            let mut best_candidate = internal;
            let mut best_candidate_residual = residual_abs;
            for _ in 0..10 {
                let candidate_vrth = (current_vrth + alpha * step).max(minimum_vrth);
                if (candidate_vrth - current_vrth).abs() < 1e-12 {
                    break;
                }

                let mut candidate = internal;
                candidate[BJT_THERMAL_STATE_INDEX] = candidate_vrth;
                let (candidate_residual, _) = Self::vbic_transient_thermal_residual_and_derivative(
                    bjt,
                    vc,
                    vb,
                    ve,
                    vs,
                    candidate,
                    method,
                    trap_order,
                    dt,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let candidate_abs = candidate_residual.abs();
                if candidate_abs.is_finite() && candidate_abs < best_candidate_residual {
                    best_candidate = candidate;
                    best_candidate_residual = candidate_abs;
                }
                if candidate_abs.is_finite() && candidate_abs < residual_abs {
                    internal = candidate;
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if accepted {
                continue;
            }
            if best_candidate_residual + 1e-15 < residual_abs {
                internal = best_candidate;
                continue;
            }
            break;
        }

        if best_residual.is_finite()
            && best_residual < 1e-9
            && (best_internal[BJT_THERMAL_STATE_INDEX] - original_vrth).abs() >= 1e-12
        {
            *snapshot = bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, best_internal);
        }
    }

    #[inline]
    pub(super) fn vbic_transient_thermal_residual_and_derivative(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> (Value, Value) {
        let thermal_charge_idx = BJT_DYNAMIC_CHARGE_COUNT - 3;
        let (mut residual, mut derivative) =
            bjt.vbic_dynamic_thermal_residual_and_derivative(vc, vb, ve, vs, internal);

        let cth = bjt.thermal_capacitance();
        let charge_factor = Self::jfet_companion_geq(method, trap_order, 1.0, dt);
        if cth > 0.0 && charge_factor > 0.0 {
            let vrth = internal[BJT_THERMAL_STATE_INDEX];
            let ieq = Self::linear_charge_history_ieq(
                method,
                trap_order,
                dt,
                q_prev[thermal_charge_idx],
                q_prev_prev[thermal_charge_idx],
                cq_prev[thermal_charge_idx],
            );
            residual += charge_factor * cth * vrth - ieq;
            derivative += charge_factor * cth;
        }

        (residual, derivative)
    }

    #[inline]
    pub(super) fn assemble_vbic_transient_linearization(
        bjt: &crate::device::Bjt,
        snapshot: &crate::device::semiconductor::BjtChargeSnapshot,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> Option<VbicTransientLinearization> {
        let charge_factor = Self::jfet_companion_geq(method, trap_order, 1.0, dt);
        if charge_factor <= 0.0 {
            return None;
        }

        let mut g_ii = snapshot.reduction.g_ii;
        let mut g_ie = snapshot.reduction.g_ie;
        let mut g_ei = snapshot.reduction.g_ei;
        let mut g_ee = snapshot.reduction.g_ee;
        let mut c_ii = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ie = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ei = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut c_ee = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut z_i = snapshot.reduction.z_i_static;
        let mut z_e = snapshot.reduction.z_e_static;
        let mut has_dynamic_charge = false;

        if bjt.uses_vbic_dynamic_charges() {
            for branch in bjt.vbic_delay_static_branches(&snapshot.reduction) {
                if !branch.is_active() {
                    continue;
                }
                let i_eq = branch.linearization_dot(
                    &snapshot.reduction.internal_voltages,
                    &snapshot.reduction.external_voltages,
                ) - branch.current;
                branch.accumulate_source(i_eq, &mut z_i, &mut z_e);
            }
            let thermal_branch = bjt.vbic_delay_static_thermal_branch(&snapshot.reduction);
            if thermal_branch.is_active() {
                // The dynamic reduction already carries the collector/emitter and xf delay
                // branch Jacobians. The excess-phase thermal-power correction is a separate
                // delta branch that must be stamped here to keep the temperature row
                // consistent with the delayed transport path.
                thermal_branch.accumulate_derivatives(&mut g_ii, &mut g_ie, &mut g_ei, &mut g_ee);
                let i_eq = thermal_branch.linearization_dot(
                    &snapshot.reduction.internal_voltages,
                    &snapshot.reduction.external_voltages,
                ) - thermal_branch.current;
                thermal_branch.accumulate_source(i_eq, &mut z_i, &mut z_e);
            }
        }

        for (branch_idx, full_branch) in snapshot.branches.iter().enumerate() {
            let (branch, ccap_history_sign) = if bjt.uses_vbic_dynamic_charges() {
                let Some(branch) =
                    Self::vbic_transient_owning_charge_branch(bjt, branch_idx, full_branch)
                else {
                    continue;
                };
                (
                    branch,
                    Self::vbic_transient_owning_charge_ccap_sign(bjt, branch_idx),
                )
            } else {
                if !full_branch.is_active() {
                    continue;
                }
                (*full_branch, 1.0)
            };
            branch.accumulate_derivatives(&mut c_ii, &mut c_ie, &mut c_ei, &mut c_ee);
            let cq_curr = Self::jfet_companion_ccap(
                method,
                trap_order,
                dt,
                branch.charge,
                q_prev[branch_idx],
                q_prev_prev[branch_idx],
                cq_prev[branch_idx],
            );
            let i_eq = charge_factor
                * branch.linearization_dot(
                    &snapshot.reduction.internal_voltages,
                    &snapshot.reduction.external_voltages,
                )
                - ccap_history_sign * cq_curr;
            branch.accumulate_source(i_eq, &mut z_i, &mut z_e);
            has_dynamic_charge = true;
        }

        if !has_dynamic_charge {
            return None;
        }

        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                g_ii[row][col] += charge_factor * c_ii[row][col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                g_ie[row][col] += charge_factor * c_ie[row][col];
            }
        }
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                g_ei[row][col] += charge_factor * c_ei[row][col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                g_ee[row][col] += charge_factor * c_ee[row][col];
            }
        }

        Some(VbicTransientLinearization {
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            z_i,
            z_e,
        })
    }

    #[inline]
    pub(super) fn vbic_transient_owning_charge_branch(
        bjt: &crate::device::Bjt,
        branch_idx: usize,
        branch: &BjtChargeBranch,
    ) -> Option<BjtChargeBranch> {
        if !branch.is_active() {
            return None;
        }

        let p = match bjt.bjt_type {
            crate::device::BjtType::Npn => 1.0,
            crate::device::BjtType::Pnp => -1.0,
        };
        match branch_idx {
            // ngspice transient integrates Qbe only against Vbei and injects the
            // resulting companion into the Ibe equation. The matrix stamp remains
            // a positive two-terminal conductance for both NPN and PNP; VBICtype
            // only changes the RHS current orientation.
            0 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VEI_STATE_INDEX],
            ),
            // Qbex is integrated only against Vbex.
            1 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VEI_STATE_INDEX],
            ),
            // Qbc is integrated only against Vbci.
            2 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VCI_STATE_INDEX],
            ),
            // Qbcx is integrated only against Vbcx.
            3 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VCX_STATE_INDEX],
            ),
            // Qbep is integrated only against Vbep.
            4 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VBP_STATE_INDEX],
            ),
            // Qbeo is integrated only against the external Vbe branch voltage.
            5 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_external[BJT_EXT_E_INDEX],
            ),
            // Qbco is integrated only against the external Vbc branch voltage.
            6 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_external[BJT_EXT_C_INDEX],
            ),
            // Qbcp is integrated only against Vbcp.
            7 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VBP_STATE_INDEX],
            ),
            // Qcth, Qxf1, and Qxf2 are single-state companions in ngspice.
            idx if idx == BJT_DYNAMIC_CHARGE_COUNT - 3 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                branch.d_internal[BJT_THERMAL_STATE_INDEX],
            ),
            idx if idx == BJT_DELAY_XF1_BRANCH_INDEX => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                branch.d_internal[BJT_DELAY_XF1_STATE_INDEX],
            ),
            idx if idx == BJT_DELAY_XF2_BRANCH_INDEX => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                branch.d_internal[BJT_DELAY_XF2_STATE_INDEX],
            ),
            _ => None,
        }
    }

    #[inline]
    pub(super) fn vbic_transient_owning_charge_ccap_sign(
        bjt: &crate::device::Bjt,
        branch_idx: usize,
    ) -> Value {
        let p = match bjt.bjt_type {
            crate::device::BjtType::Npn => 1.0,
            crate::device::BjtType::Pnp => -1.0,
        };
        match branch_idx {
            // ngspice keeps the owning-capacitance matrix orientation positive for
            // both NPN and PNP, but the companion history current enters through
            // branch RHS terms that are multiplied by VBICtype for these branches.
            0 | 1 | 2 | 4 | 5 | 6 | 7 => p,
            // Qbcx, Qcth, Qxf1, and Qxf2 are stamped without VBICtype on the RHS.
            _ => 1.0,
        }
    }

    #[inline]
    pub(super) fn vbic_branch_voltage_charge_branch(
        charge: Value,
        pos_internal: Option<usize>,
        neg_internal: Option<usize>,
        pos_external: Option<usize>,
        neg_external: Option<usize>,
        voltage_sign: Value,
        dq_dv: Value,
    ) -> Option<BjtChargeBranch> {
        if !dq_dv.is_finite() || dq_dv.abs() <= 0.0 {
            return None;
        }

        let mut branch = BjtChargeBranch {
            charge,
            pos_internal,
            neg_internal,
            pos_external,
            neg_external,
            ..Default::default()
        };
        if let Some(idx) = pos_internal {
            branch.d_internal[idx] += voltage_sign * dq_dv;
        }
        if let Some(idx) = neg_internal {
            branch.d_internal[idx] -= voltage_sign * dq_dv;
        }
        if let Some(idx) = pos_external {
            branch.d_external[idx] += voltage_sign * dq_dv;
        }
        if let Some(idx) = neg_external {
            branch.d_external[idx] -= voltage_sign * dq_dv;
        }
        Some(branch)
    }

    #[inline]
    pub(super) fn solve_vbic_internal_state_from_linearization(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        let (lu_internal, pivots_internal) =
            Self::lu_decompose_small_dense_real(&linearization.g_ii, BJT_INTERNAL_STATE_DIM)?;
        let mut rhs_internal = linearization.z_i;
        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                rhs_internal[row] -= linearization.g_ie[row][col] * external_voltages[col];
            }
        }
        Self::lu_solve_small_dense_real(
            &lu_internal,
            &pivots_internal,
            &rhs_internal,
            BJT_INTERNAL_STATE_DIM,
        )
    }

    #[inline]
    pub(super) fn solve_vbic_static_core_from_linearization(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        let mut g_static = [[0.0; BJT_STATIC_CORE_STATE_DIM]; BJT_STATIC_CORE_STATE_DIM];
        let mut rhs_static = [0.0; BJT_STATIC_CORE_STATE_DIM];
        for row in 0..BJT_STATIC_CORE_STATE_DIM {
            rhs_static[row] = linearization.z_i[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                rhs_static[row] -= linearization.g_ie[row][col] * external_voltages[col];
            }
            for col in BJT_STATIC_CORE_STATE_DIM..BJT_INTERNAL_STATE_DIM {
                rhs_static[row] -= linearization.g_ii[row][col] * internal_voltages[col];
            }
            for col in 0..BJT_STATIC_CORE_STATE_DIM {
                g_static[row][col] = linearization.g_ii[row][col];
            }
        }
        let (lu_static, pivots_static) =
            Self::lu_decompose_small_dense_real(&g_static, BJT_STATIC_CORE_STATE_DIM)?;
        let solved_static = Self::lu_solve_small_dense_real(
            &lu_static,
            &pivots_static,
            &rhs_static,
            BJT_STATIC_CORE_STATE_DIM,
        )?;
        let mut solved_internal = *internal_voltages;
        solved_internal[..BJT_STATIC_CORE_STATE_DIM].copy_from_slice(&solved_static);
        Some(solved_internal)
    }

    #[inline]
    pub(super) fn vbic_internal_equation_residual(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let mut residual = [0.0; BJT_INTERNAL_STATE_DIM];
        for row in 0..BJT_INTERNAL_STATE_DIM {
            residual[row] = -linearization.z_i[row];
            for col in 0..BJT_INTERNAL_STATE_DIM {
                residual[row] += linearization.g_ii[row][col] * internal_voltages[col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                residual[row] += linearization.g_ie[row][col] * external_voltages[col];
            }
        }
        residual
    }

    #[inline]
    pub(super) fn vbic_internal_equation_residual_norm(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Value {
        Self::vbic_internal_equation_residual(linearization, external_voltages, internal_voltages)
            .into_iter()
            .fold(0.0, |max_norm, value| max_norm.max(value.abs()))
    }

    #[inline]
    pub(super) fn vbic_internal_equation_residual_objective(
        residual: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Value {
        residual
            .iter()
            .map(|value| value * value)
            .sum::<Value>()
            .sqrt()
    }

    #[inline]
    pub(super) fn vbic_dynamic_state_evaluation_residual_objective(
        evaluation: &VbicDynamicStateEvaluation,
    ) -> Value {
        Self::vbic_internal_equation_residual_objective(&evaluation.3)
    }

    #[inline]
    pub(super) fn vbic_dynamic_static_core_residual_norm(
        residual: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Value {
        residual[..BJT_STATIC_CORE_STATE_DIM]
            .iter()
            .fold(0.0_f64, |max_norm, value| max_norm.max(value.abs()))
    }

    #[inline]
    pub(super) fn refine_vbic_dynamic_static_core_with_fixed_delay(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        mut current_state: VbicDynamicStateEvaluation,
        max_iterations: usize,
    ) -> VbicDynamicStateEvaluation {
        let mut current_objective =
            Self::vbic_dynamic_state_evaluation_residual_objective(&current_state);
        for iteration in 0..max_iterations {
            let static_residual_norm =
                Self::vbic_dynamic_static_core_residual_norm(&current_state.3);
            if static_residual_norm < 1e-10 {
                break;
            }

            let current_internal = current_state.0.reduction.internal_voltages;
            let Some(target_internal) = Self::solve_vbic_static_core_from_linearization(
                &current_state.1,
                &current_state.0.reduction.external_voltages,
                &current_internal,
            ) else {
                break;
            };
            if !target_internal.iter().all(|value| value.is_finite()) {
                break;
            }
            let max_static_delta = (0..BJT_STATIC_CORE_STATE_DIM)
                .map(|idx| (target_internal[idx] - current_internal[idx]).abs())
                .fold(0.0_f64, Value::max);
            if max_static_delta < 1e-12 {
                break;
            }

            let target_internal = Self::step_limit_vbic_dynamic_internal_target(
                current_internal,
                target_internal,
                iteration,
                current_state.4,
            );
            let Some(next_state) = Self::improve_vbic_dynamic_internal_state_toward_target(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                current_internal,
                current_state.4,
                current_objective,
                target_internal,
                current_internal,
                12,
            ) else {
                break;
            };
            let next_objective =
                Self::vbic_dynamic_state_evaluation_residual_objective(&next_state);
            if next_objective + 1e-15 >= current_objective {
                break;
            }
            current_state = next_state;
            current_objective = next_objective;
        }
        current_state
    }

    pub(super) const VBIC_DYNAMIC_BOUNDED_BEST_EFFORT_RESIDUAL_NORM: Value = 5e-2;
    pub(super) const VBIC_HOMOTOPY_MIN_LAMBDA_STEP: Value = 1e-6;
    pub(super) const VBIC_CONTINUATION_MIN_TRIAL_STEP: Value = 1.0 / 64.0;

    #[inline]
    pub(super) fn vbic_dynamic_snapshot_best_effort_is_bounded(
        result: &VbicBestEffortSolve,
    ) -> bool {
        result.3.is_finite()
            && result.3 <= Self::VBIC_DYNAMIC_BOUNDED_BEST_EFFORT_RESIDUAL_NORM
            && result
                .0
                .reduction
                .internal_voltages
                .iter()
                .all(|value| value.is_finite())
    }

    #[inline]
    pub(super) fn vbic_dynamic_snapshot_solution_is_acceptable(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> bool {
        let residual = Self::vbic_internal_equation_residual(
            linearization,
            external_voltages,
            internal_voltages,
        );
        let criteria = NonlinearConvergenceCriteria::default();
        let reltol = criteria.relative_tolerance();
        let current_tol = criteria.current_tolerance();

        residual.into_iter().enumerate().all(|(row, value)| {
            if !value.is_finite() {
                return false;
            }
            let lhs = linearization.g_ii[row]
                .iter()
                .zip(internal_voltages.iter())
                .map(|(coefficient, state)| coefficient * state)
                .sum::<Value>()
                + linearization.g_ie[row]
                    .iter()
                    .zip(external_voltages.iter())
                    .map(|(coefficient, voltage)| coefficient * voltage)
                    .sum::<Value>();
            let rhs = linearization.z_i[row];
            if !lhs.is_finite() || !rhs.is_finite() {
                return false;
            }
            let tolerance = current_tol + reltol * lhs.abs().max(rhs.abs());
            value.abs() <= tolerance
        })
    }

    #[inline]
    pub(super) fn choose_preferred_vbic_best_effort_result<F>(
        current: Option<VbicBestEffortSolve>,
        alternate: Option<VbicBestEffortSolve>,
        is_acceptable: F,
    ) -> Option<VbicBestEffortSolve>
    where
        F: Fn(&VbicBestEffortSolve) -> bool,
    {
        match (current, alternate) {
            (Some(current), Some(alternate)) => {
                let current_acceptable = is_acceptable(&current);
                let alternate_acceptable = is_acceptable(&alternate);
                if alternate_acceptable != current_acceptable {
                    if alternate_acceptable {
                        Some(alternate)
                    } else {
                        Some(current)
                    }
                } else if alternate.3 + 1e-18 < current.3 {
                    Some(alternate)
                } else {
                    Some(current)
                }
            }
            (Some(current), None) => Some(current),
            (None, Some(alternate)) => Some(alternate),
            (None, None) => None,
        }
    }

    #[inline]
    pub(super) fn vbic_dynamic_internal_state_step_limit_for_index(
        index: usize,
        _iteration: usize,
        residual_norm: Value,
    ) -> Value {
        match index {
            BJT_THERMAL_STATE_INDEX => {
                if residual_norm > 1e-2 {
                    2.0
                } else if residual_norm > 1e-4 {
                    1.0
                } else if residual_norm > 1e-8 {
                    0.5
                } else {
                    0.1
                }
            }
            _ => {
                if residual_norm > 1e-2 {
                    1.0
                } else if residual_norm > 1e-4 {
                    0.5
                } else if residual_norm > 1e-8 {
                    0.25
                } else {
                    0.1
                }
            }
        }
    }

    #[inline]
    pub(super) fn step_limit_vbic_dynamic_internal_target(
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_internal: [Value; BJT_INTERNAL_STATE_DIM],
        iteration: usize,
        residual_norm: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let mut max_raw_delta = 0.0_f64;
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            max_raw_delta = max_raw_delta.max((target_internal[idx] - current_internal[idx]).abs());
        }
        if !max_raw_delta.is_finite() || max_raw_delta < 1e-13 {
            return current_internal;
        }

        let mut alpha = 1.0_f64;
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            let delta = (target_internal[idx] - current_internal[idx]).abs();
            if !delta.is_finite() {
                return current_internal;
            }
            if delta < 1e-13 {
                continue;
            }
            let limit = Self::vbic_dynamic_internal_state_step_limit_for_index(
                idx,
                iteration,
                residual_norm,
            );
            alpha = alpha.min(limit / delta);
        }
        let alpha = if alpha.is_finite() {
            alpha.min(1.0)
        } else {
            return current_internal;
        };
        if alpha <= 0.0 {
            return current_internal;
        }

        let mut limited_target = current_internal;
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            limited_target[idx] =
                current_internal[idx] + alpha * (target_internal[idx] - current_internal[idx]);
        }
        limited_target
    }

    #[inline]
    pub(super) fn vbic_predictor_linear_branch_state(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> VbicPredictorLinearBranchState {
        let polarity = match bjt.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        };
        VbicPredictorLinearBranchState {
            vrcx: polarity * (external[BJT_EXT_C_INDEX] - internal[BJT_VCX_STATE_INDEX]),
            vrci: polarity * (internal[BJT_VCX_STATE_INDEX] - internal[BJT_VCI_STATE_INDEX]),
            vrbx: polarity * (external[1] - internal[BJT_VBX_STATE_INDEX]),
            vrbi: polarity * (internal[BJT_VBX_STATE_INDEX] - internal[BJT_VBI_STATE_INDEX]),
            vre: polarity * (external[BJT_EXT_E_INDEX] - internal[BJT_VEI_STATE_INDEX]),
            vrbp: polarity * (internal[BJT_VBP_STATE_INDEX] - internal[BJT_VCX_STATE_INDEX]),
            vrs: polarity * (external[3] - internal[BJT_VSI_STATE_INDEX]),
        }
    }

    #[inline]
    pub(super) fn evaluate_vbic_dynamic_internal_state(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> Option<VbicDynamicStateEvaluation> {
        let mut snapshot = bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, internal);
        Self::rebalance_vbic_dynamic_thermal_state(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            &mut snapshot,
        );
        let base_static_g = snapshot.reduction.g_reduced;
        let linearization = Self::assemble_vbic_transient_linearization(
            bjt,
            &snapshot,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        )?;
        let residual = Self::vbic_internal_equation_residual(
            &linearization,
            &snapshot.reduction.external_voltages,
            &snapshot.reduction.internal_voltages,
        );
        let residual_norm = residual
            .iter()
            .fold(0.0_f64, |max_norm, value| max_norm.max(value.abs()));
        Some((
            snapshot,
            linearization,
            base_static_g,
            residual,
            residual_norm,
        ))
    }

    pub(super) fn improve_vbic_dynamic_internal_state_toward_target(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        _current_residual_norm: Value,
        current_residual_objective: Value,
        target_internal: [Value; BJT_INTERNAL_STATE_DIM],
        envelope_reference: [Value; BJT_INTERNAL_STATE_DIM],
        max_backtracks: usize,
    ) -> Option<VbicDynamicStateEvaluation> {
        let mut alpha = 1.0;
        let mut best_state: Option<VbicDynamicStateEvaluation> = None;

        for _ in 0..max_backtracks {
            let mut candidate_internal = current_internal;
            for idx in 0..BJT_INTERNAL_STATE_DIM {
                candidate_internal[idx] =
                    current_internal[idx] + alpha * (target_internal[idx] - current_internal[idx]);
            }
            candidate_internal = bjt.limit_vbic_dynamic_internal_state_to_previous(
                candidate_internal,
                current_internal,
            );
            if !candidate_internal.iter().all(|value| value.is_finite()) {
                alpha *= 0.5;
                continue;
            }
            if !bjt.vbic_dynamic_internal_state_within_local_branch_envelope(
                candidate_internal,
                envelope_reference,
            ) {
                alpha *= 0.5;
                continue;
            }

            let Some(candidate_state) = Self::evaluate_vbic_dynamic_internal_state(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                candidate_internal,
            ) else {
                alpha *= 0.5;
                continue;
            };
            let candidate_objective =
                Self::vbic_dynamic_state_evaluation_residual_objective(&candidate_state);
            if candidate_state.4.is_finite()
                && best_state.as_ref().map_or(true, |best_state| {
                    candidate_objective + 1e-15
                        < Self::vbic_dynamic_state_evaluation_residual_objective(best_state)
                })
            {
                best_state = Some(candidate_state.clone());
            }
            if candidate_state.4.is_finite()
                && candidate_objective + 1e-15 < current_residual_objective
            {
                return Some(candidate_state);
            }
            alpha *= 0.5;
        }

        best_state.and_then(|best_state| {
            if Self::vbic_dynamic_state_evaluation_residual_objective(&best_state) + 1e-15
                < current_residual_objective
            {
                Some(best_state)
            } else {
                None
            }
        })
    }

    #[inline]
    pub(super) fn vbic_reduce_transient_external_system(
        linearization: &VbicTransientLinearization,
    ) -> Option<(
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        [Value; BJT_EXTERNAL_STATE_DIM],
    )> {
        let (lu_internal, pivots_internal) =
            Self::lu_decompose_small_dense_real(&linearization.g_ii, BJT_INTERNAL_STATE_DIM)?;

        let mut y_total = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        for col in 0..BJT_EXTERNAL_STATE_DIM {
            let mut rhs_internal = [0.0; BJT_INTERNAL_STATE_DIM];
            for row in 0..BJT_INTERNAL_STATE_DIM {
                rhs_internal[row] = -linearization.g_ie[row][col];
            }
            let solution = Self::lu_solve_small_dense_real(
                &lu_internal,
                &pivots_internal,
                &rhs_internal,
                BJT_INTERNAL_STATE_DIM,
            )?;
            for row in 0..BJT_EXTERNAL_STATE_DIM {
                let mut value = linearization.g_ee[row][col];
                for internal_idx in 0..BJT_INTERNAL_STATE_DIM {
                    value += linearization.g_ei[row][internal_idx] * solution[internal_idx];
                }
                y_total[row][col] = value;
            }
        }

        let z_solution = Self::lu_solve_small_dense_real(
            &lu_internal,
            &pivots_internal,
            &linearization.z_i,
            BJT_INTERNAL_STATE_DIM,
        )?;
        let mut reduced_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            reduced_i_eq[row] = linearization.z_e[row];
            for internal_idx in 0..BJT_INTERNAL_STATE_DIM {
                reduced_i_eq[row] -=
                    linearization.g_ei[row][internal_idx] * z_solution[internal_idx];
            }
        }

        Some((y_total, reduced_i_eq))
    }

    #[inline]
    pub(super) fn vbic_static_stamped_external_system(
        bjt: &crate::device::Bjt,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) -> (
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        [Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        bjt.stamped_reduced_external_system(external[0], external[1], external[2], external[3])
    }

    #[inline]
    pub(super) fn solve_vbic_dynamic_snapshot(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        Self::solve_vbic_dynamic_snapshot_primary(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            seed_internal,
        )
        .or_else(|| {
            Self::solve_vbic_dynamic_snapshot_with_collector_substrate_charge_homotopy(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )
        })
    }

    #[inline]
    pub(super) fn solve_vbic_dynamic_snapshot_primary(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        Self::solve_vbic_dynamic_snapshot_direct(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            seed_internal,
        )
        .or_else(|| {
            Self::solve_vbic_dynamic_snapshot_with_excess_phase_homotopy(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )
        })
    }

    pub(super) fn solve_vbic_dynamic_snapshot_with_collector_substrate_charge_homotopy(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        if !bjt.uses_vbic_dynamic_charges() {
            return None;
        }
        if bjt.qco <= 0.0 && bjt.cjcp <= 0.0 && bjt.ccso <= 0.0 {
            return None;
        }

        let scale_collector_substrate_history =
            |lambda: Value,
             q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
             q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
             cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT]| {
                let mut scaled_q_prev = *q_prev;
                let mut scaled_q_prev_prev = *q_prev_prev;
                let mut scaled_cq_prev = *cq_prev;
                // Keep the homotopy path self-consistent by scaling the stored
                // collector/substrate charge histories alongside the scaled VBIC
                // Qbc/Qbcx/Qbcp branch equations.
                for branch_idx in [
                    BJT_QBC_BRANCH_INDEX,
                    BJT_QBCX_BRANCH_INDEX,
                    BJT_QBCP_BRANCH_INDEX,
                ] {
                    scaled_q_prev[branch_idx] *= lambda;
                    scaled_q_prev_prev[branch_idx] *= lambda;
                    scaled_cq_prev[branch_idx] *= lambda;
                }
                (scaled_q_prev, scaled_q_prev_prev, scaled_cq_prev)
            };

        let mut lambda: Value = 0.0;
        let mut step: Value = 1.0;
        let mut current_state = {
            let scaled_bjt = bjt.vbic_collector_substrate_charge_homotopy_variant(0.0);
            let initial_seed = seed_internal
                .copied()
                .unwrap_or_else(|| scaled_bjt.dynamic_internal_state_seed(vc, vb, ve, vs));
            let (scaled_q_prev, scaled_q_prev_prev, scaled_cq_prev) =
                scale_collector_substrate_history(0.0, q_prev, q_prev_prev, cq_prev);
            Self::solve_vbic_dynamic_snapshot_primary(
                &scaled_bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &scaled_q_prev,
                &scaled_q_prev_prev,
                &scaled_cq_prev,
                Some(&initial_seed),
            )?
        };

        while lambda < 1.0 - 1e-15 {
            let candidate_lambda = (lambda + step).min(1.0);
            let scaled_bjt = bjt.vbic_collector_substrate_charge_homotopy_variant(candidate_lambda);
            let previous_internal = current_state.0.reduction.internal_voltages;
            let (scaled_q_prev, scaled_q_prev_prev, scaled_cq_prev) =
                scale_collector_substrate_history(candidate_lambda, q_prev, q_prev_prev, cq_prev);
            let Some(candidate_state) = Self::solve_vbic_dynamic_snapshot_primary(
                &scaled_bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &scaled_q_prev,
                &scaled_q_prev_prev,
                &scaled_cq_prev,
                Some(&previous_internal),
            ) else {
                if step <= Self::VBIC_HOMOTOPY_MIN_LAMBDA_STEP {
                    return None;
                }
                step *= 0.5;
                continue;
            };
            current_state = candidate_state;
            lambda = candidate_lambda;
            step = (step * 2.0).min(1.0 - lambda).max(1e-6);
        }

        Some(current_state)
    }

    pub(super) fn solve_vbic_dynamic_snapshot_with_excess_phase_homotopy(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        if !bjt.uses_vbic_dynamic_charges() || bjt.td <= 0.0 {
            return None;
        }

        let target_td = bjt.td;
        let scale_excess_phase_history =
            |lambda: Value,
             q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
             q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
             cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT]| {
                let mut scaled_q_prev = *q_prev;
                let mut scaled_q_prev_prev = *q_prev_prev;
                let mut scaled_cq_prev = *cq_prev;
                scaled_q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= lambda;
                scaled_q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= lambda;
                scaled_q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= lambda;
                scaled_q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= lambda;
                scaled_cq_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= lambda;
                scaled_cq_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= lambda;
                (scaled_q_prev, scaled_q_prev_prev, scaled_cq_prev)
            };
        let scale_excess_phase_seed =
            |lambda: Value, seed_internal: &[Value; BJT_INTERNAL_STATE_DIM]| {
                let mut scaled_seed = *seed_internal;
                scaled_seed[BJT_DELAY_XF1_STATE_INDEX] *= lambda;
                scaled_seed[BJT_DELAY_XF2_STATE_INDEX] *= lambda;
                scaled_seed
            };
        let (target_q_prev, target_q_prev_prev, target_cq_prev) =
            scale_excess_phase_history(1.0, q_prev, q_prev_prev, cq_prev);

        let mut base_bjt = bjt.clone();
        base_bjt.td = 0.0;
        let (base_q_prev, base_q_prev_prev, base_cq_prev) =
            scale_excess_phase_history(0.0, q_prev, q_prev_prev, cq_prev);
        let base_seed =
            seed_internal.map(|seed_internal| scale_excess_phase_seed(0.0, seed_internal));
        let live_base_seed = base_bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let mut current_result = Self::solve_vbic_dynamic_snapshot_best_effort(
            &base_bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &base_q_prev,
            &base_q_prev_prev,
            &base_cq_prev,
            base_seed.as_ref(),
        );
        let live_base_result = Self::solve_vbic_dynamic_snapshot_best_effort(
            &base_bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &base_q_prev,
            &base_q_prev_prev,
            &base_cq_prev,
            Some(&live_base_seed),
        );
        current_result = Self::choose_preferred_vbic_best_effort_result(
            current_result,
            live_base_result,
            |result| {
                Self::vbic_dynamic_snapshot_solution_is_acceptable(
                    &result.1,
                    &result.0.reduction.external_voltages,
                    &result.0.reduction.internal_voltages,
                )
            },
        );
        let mut current_result = current_result?;
        let mut lambda = 0.0_f64;
        let mut step = 0.25_f64;
        while lambda < 1.0 - 1e-15 {
            let candidate_lambda = (lambda + step).min(1.0);
            let mut stepped_bjt = bjt.clone();
            stepped_bjt.td = target_td * candidate_lambda;
            let (candidate_q_prev, candidate_q_prev_prev, candidate_cq_prev) =
                scale_excess_phase_history(candidate_lambda, q_prev, q_prev_prev, cq_prev);
            let previous_internal = current_result.0.reduction.internal_voltages;
            let live_candidate_seed = stepped_bjt.limit_vbic_dynamic_internal_state_to_previous(
                stepped_bjt.dynamic_internal_state_seed(vc, vb, ve, vs),
                previous_internal,
            );
            let mut candidate_result = Self::solve_vbic_dynamic_snapshot_best_effort(
                &stepped_bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &candidate_q_prev,
                &candidate_q_prev_prev,
                &candidate_cq_prev,
                Some(&previous_internal),
            );
            let live_candidate_result = Self::solve_vbic_dynamic_snapshot_best_effort(
                &stepped_bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &candidate_q_prev,
                &candidate_q_prev_prev,
                &candidate_cq_prev,
                Some(&live_candidate_seed),
            );
            candidate_result = Self::choose_preferred_vbic_best_effort_result(
                candidate_result,
                live_candidate_result,
                |result| {
                    Self::vbic_homotopy_candidate_is_acceptable(
                        &stepped_bjt,
                        [vc, vb, ve, vs],
                        previous_internal,
                        &result.0,
                        &result.1,
                    )
                },
            );
            let Some(candidate_result) = candidate_result else {
                if step <= Self::VBIC_HOMOTOPY_MIN_LAMBDA_STEP {
                    return None;
                }
                step *= 0.5;
                continue;
            };
            if !Self::vbic_homotopy_candidate_is_acceptable(
                &stepped_bjt,
                [vc, vb, ve, vs],
                previous_internal,
                &candidate_result.0,
                &candidate_result.1,
            ) {
                if step <= Self::VBIC_HOMOTOPY_MIN_LAMBDA_STEP {
                    return None;
                }
                step *= 0.5;
                continue;
            }
            current_result = candidate_result;
            lambda = candidate_lambda;
            if lambda < 1.0 - 1e-15 {
                let target_internal = current_result.0.reduction.internal_voltages;
                let live_target_seed = bjt.limit_vbic_dynamic_internal_state_to_previous(
                    bjt.dynamic_internal_state_seed(vc, vb, ve, vs),
                    target_internal,
                );
                let target_result = Self::choose_preferred_vbic_best_effort_result(
                    Self::solve_vbic_dynamic_snapshot_best_effort(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        &target_q_prev,
                        &target_q_prev_prev,
                        &target_cq_prev,
                        Some(&target_internal),
                    ),
                    Self::solve_vbic_dynamic_snapshot_best_effort(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        &target_q_prev,
                        &target_q_prev_prev,
                        &target_cq_prev,
                        Some(&live_target_seed),
                    ),
                    |result| {
                        Self::vbic_homotopy_candidate_is_acceptable(
                            bjt,
                            [vc, vb, ve, vs],
                            target_internal,
                            &result.0,
                            &result.1,
                        )
                    },
                );
                if let Some(target_result) = target_result
                    && Self::vbic_homotopy_candidate_is_acceptable(
                        bjt,
                        [vc, vb, ve, vs],
                        target_internal,
                        &target_result.0,
                        &target_result.1,
                    )
                {
                    current_result = target_result;
                    break;
                }
            }
            step = (step * 2.0).min((1.0 - lambda).max(0.0)).max(1e-6);
        }

        Some((current_result.0, current_result.1, current_result.2))
    }

    #[inline]
    pub(super) fn solve_vbic_dynamic_snapshot_direct(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        let (snapshot, linearization, base_static_g, _residual_norm) =
            Self::solve_vbic_dynamic_snapshot_best_effort(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )?;
        Self::vbic_dynamic_snapshot_solution_is_acceptable(
            &linearization,
            &snapshot.reduction.external_voltages,
            &snapshot.reduction.internal_voltages,
        )
        .then_some((snapshot, linearization, base_static_g))
    }

    #[inline]
    pub(super) fn solve_vbic_dynamic_snapshot_best_effort(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<VbicBestEffortSolve> {
        let mut seeded_snapshot = if let Some(seed_internal) = seed_internal {
            bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, *seed_internal)
        } else {
            bjt.charge_snapshot(vc, vb, ve, vs)
        };
        Self::rebalance_vbic_dynamic_thermal_state(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            &mut seeded_snapshot,
        );
        let mut base_static_g = seeded_snapshot.reduction.g_reduced;
        let mut transient_linearization = Self::assemble_vbic_transient_linearization(
            bjt,
            &seeded_snapshot,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        )?;
        let initial_residual = Self::vbic_internal_equation_residual(
            &transient_linearization,
            &seeded_snapshot.reduction.external_voltages,
            &seeded_snapshot.reduction.internal_voltages,
        );
        let polished_initial_state = Self::refine_vbic_dynamic_static_core_with_fixed_delay(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            (
                seeded_snapshot,
                transient_linearization,
                base_static_g,
                initial_residual,
                initial_residual
                    .iter()
                    .fold(0.0_f64, |max_norm, value| max_norm.max(value.abs())),
            ),
            6,
        );
        seeded_snapshot = polished_initial_state.0;
        transient_linearization = polished_initial_state.1;
        base_static_g = polished_initial_state.2;
        let mut current_residual_norm = polished_initial_state.4;
        let mut current_residual_objective =
            Self::vbic_internal_equation_residual_objective(&polished_initial_state.3);

        let max_refinements = if bjt.has_vbic_self_heating() {
            96
        } else if bjt.uses_vbic_dynamic_charges() {
            64
        } else {
            32
        };
        for iteration in 0..max_refinements {
            if current_residual_norm < 1e-14 {
                break;
            }
            let current_internal = seeded_snapshot.reduction.internal_voltages;
            let solved_internal = Self::solve_vbic_internal_state_from_linearization(
                &transient_linearization,
                &seeded_snapshot.reduction.external_voltages,
            )?;
            let target_internal = Self::step_limit_vbic_dynamic_internal_target(
                current_internal,
                solved_internal,
                iteration,
                current_residual_norm,
            );
            if !target_internal.iter().all(|value| value.is_finite()) {
                break;
            }

            let max_delay_state = target_internal[BJT_DELAY_XF1_STATE_INDEX]
                .abs()
                .max(target_internal[BJT_DELAY_XF2_STATE_INDEX].abs());
            static VBIC_INTERNAL_SOLVE_LOG_COUNT: std::sync::atomic::AtomicUsize =
                std::sync::atomic::AtomicUsize::new(0);
            let internal_log_count =
                VBIC_INTERNAL_SOLVE_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if max_delay_state > 1.0 && internal_log_count < 8 {
                log::warn!(
                    "VBIC internal solve {} ext={:?} seed_xf=({:.3e}, {:.3e}) solved_xf=({:.3e}, {:.3e}) z_xf=({:.3e}, {:.3e}) g_xf1={:?} g_xf2={:?}",
                    bjt.name,
                    seeded_snapshot.reduction.external_voltages,
                    seeded_snapshot.reduction.internal_voltages[BJT_DELAY_XF1_STATE_INDEX],
                    seeded_snapshot.reduction.internal_voltages[BJT_DELAY_XF2_STATE_INDEX],
                    target_internal[BJT_DELAY_XF1_STATE_INDEX],
                    target_internal[BJT_DELAY_XF2_STATE_INDEX],
                    transient_linearization.z_i[BJT_DELAY_XF1_STATE_INDEX],
                    transient_linearization.z_i[BJT_DELAY_XF2_STATE_INDEX],
                    transient_linearization.g_ii[BJT_DELAY_XF1_STATE_INDEX],
                    transient_linearization.g_ii[BJT_DELAY_XF2_STATE_INDEX],
                );
            }

            let max_delta = target_internal
                .iter()
                .zip(current_internal.iter())
                .map(|(solved, current)| (solved - current).abs())
                .fold(0.0, Value::max);
            if max_delta < 1e-12 {
                break;
            }

            let Some((
                solved_snapshot,
                solved_linearization,
                solved_static_g,
                solved_residual,
                solved_residual_norm,
            )) = Self::improve_vbic_dynamic_internal_state_toward_target(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                current_internal,
                current_residual_norm,
                current_residual_objective,
                target_internal,
                current_internal,
                12,
            )
            else {
                break;
            };

            let polished_state = Self::refine_vbic_dynamic_static_core_with_fixed_delay(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                (
                    solved_snapshot,
                    solved_linearization,
                    solved_static_g,
                    solved_residual,
                    solved_residual_norm,
                ),
                4,
            );
            base_static_g = polished_state.2;
            seeded_snapshot = polished_state.0;
            transient_linearization = polished_state.1;
            current_residual_norm = polished_state.4;
            current_residual_objective =
                Self::vbic_internal_equation_residual_objective(&polished_state.3);
        }

        if current_residual_norm > 1e-8 {
            let mut current_state = Self::evaluate_vbic_dynamic_internal_state(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seeded_snapshot.reduction.internal_voltages,
            )
            .unwrap_or((
                seeded_snapshot.clone(),
                transient_linearization.clone(),
                base_static_g,
                Self::vbic_internal_equation_residual(
                    &transient_linearization,
                    &seeded_snapshot.reduction.external_voltages,
                    &seeded_snapshot.reduction.internal_voltages,
                ),
                current_residual_norm,
            ));
            let mut current_residual_objective =
                Self::vbic_dynamic_state_evaluation_residual_objective(&current_state);
            for iteration in 0..16 {
                if current_state.4 < 1e-10 {
                    break;
                }

                let current_internal = current_state.0.reduction.internal_voltages;
                let current_external = current_state.0.reduction.external_voltages;
                let mut next_state = Self::solve_vbic_internal_state_from_linearization(
                    &current_state.1,
                    &current_external,
                )
                .and_then(|target_internal| {
                    let target_internal = Self::step_limit_vbic_dynamic_internal_target(
                        current_internal,
                        target_internal,
                        iteration,
                        current_state.4,
                    );
                    Self::improve_vbic_dynamic_internal_state_toward_target(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        current_internal,
                        current_state.4,
                        current_residual_objective,
                        target_internal,
                        current_internal,
                        12,
                    )
                    .map(|candidate_state| {
                        Self::refine_vbic_dynamic_static_core_with_fixed_delay(
                            bjt,
                            vc,
                            vb,
                            ve,
                            vs,
                            method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            candidate_state,
                            4,
                        )
                    })
                });

                let mut jacobian = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
                for col in 0..BJT_INTERNAL_STATE_DIM {
                    let base_value = current_internal[col];
                    let step = match col {
                        BJT_DELAY_XF1_STATE_INDEX | BJT_DELAY_XF2_STATE_INDEX => {
                            (base_value.abs() * 1e-3).max(1e-9)
                        }
                        BJT_THERMAL_STATE_INDEX => (base_value.abs() * 1e-4).max(1e-6),
                        _ => (base_value.abs() * 1e-6).max(1e-7),
                    };

                    let mut plus_internal = current_internal;
                    plus_internal[col] = base_value + step;
                    if col == BJT_THERMAL_STATE_INDEX {
                        plus_internal[col] = plus_internal[col].max(bjt.minimum_thermal_rise());
                    }
                    let Some(plus_state) = Self::evaluate_vbic_dynamic_internal_state(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        plus_internal,
                    ) else {
                        continue;
                    };

                    let use_central = col != BJT_THERMAL_STATE_INDEX
                        || base_value - step >= bjt.minimum_thermal_rise();
                    if use_central {
                        let mut minus_internal = current_internal;
                        minus_internal[col] = base_value - step;
                        if col == BJT_THERMAL_STATE_INDEX {
                            minus_internal[col] =
                                minus_internal[col].max(bjt.minimum_thermal_rise());
                        }
                        let Some(minus_state) = Self::evaluate_vbic_dynamic_internal_state(
                            bjt,
                            vc,
                            vb,
                            ve,
                            vs,
                            method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            minus_internal,
                        ) else {
                            continue;
                        };
                        let denom = plus_internal[col] - minus_internal[col];
                        if denom.abs() <= 0.0 {
                            continue;
                        }
                        for row in 0..BJT_INTERNAL_STATE_DIM {
                            jacobian[row][col] = (plus_state.3[row] - minus_state.3[row]) / denom;
                        }
                    } else {
                        let denom = plus_internal[col] - current_internal[col];
                        if denom.abs() <= 0.0 {
                            continue;
                        }
                        for row in 0..BJT_INTERNAL_STATE_DIM {
                            jacobian[row][col] = (plus_state.3[row] - current_state.3[row]) / denom;
                        }
                    }
                }

                let rhs = current_state.3.map(|value| -value);
                let Some((lu_internal, pivots_internal)) =
                    Self::lu_decompose_small_dense_real(&jacobian, BJT_INTERNAL_STATE_DIM)
                else {
                    break;
                };
                let Some(delta) = Self::lu_solve_small_dense_real(
                    &lu_internal,
                    &pivots_internal,
                    &rhs,
                    BJT_INTERNAL_STATE_DIM,
                ) else {
                    break;
                };
                let max_raw_delta = delta
                    .iter()
                    .fold(0.0_f64, |max_delta, value| max_delta.max(value.abs()));
                if max_raw_delta < 1e-12 {
                    break;
                }
                let mut target_internal = current_internal;
                for idx in 0..BJT_INTERNAL_STATE_DIM {
                    target_internal[idx] = current_internal[idx] + delta[idx];
                }
                target_internal = Self::step_limit_vbic_dynamic_internal_target(
                    current_internal,
                    target_internal,
                    iteration,
                    current_state.4,
                );
                if let Some(candidate_state) =
                    Self::improve_vbic_dynamic_internal_state_toward_target(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        current_internal,
                        current_state.4,
                        current_residual_objective,
                        target_internal,
                        current_internal,
                        12,
                    )
                    .map(|candidate_state| {
                        Self::refine_vbic_dynamic_static_core_with_fixed_delay(
                            bjt,
                            vc,
                            vb,
                            ve,
                            vs,
                            method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            candidate_state,
                            4,
                        )
                    })
                {
                    let candidate_objective =
                        Self::vbic_dynamic_state_evaluation_residual_objective(&candidate_state);
                    if next_state.as_ref().map_or(true, |best_state| {
                        candidate_objective + 1e-15
                            < Self::vbic_dynamic_state_evaluation_residual_objective(best_state)
                    }) {
                        next_state = Some(candidate_state);
                    }
                }

                let mut normal_matrix = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
                let mut gradient = [0.0; BJT_INTERNAL_STATE_DIM];
                for row in 0..BJT_INTERNAL_STATE_DIM {
                    for col in 0..BJT_INTERNAL_STATE_DIM {
                        let mut value = 0.0;
                        for inner in 0..BJT_INTERNAL_STATE_DIM {
                            value += jacobian[inner][row] * jacobian[inner][col];
                        }
                        normal_matrix[row][col] = value;
                    }
                    gradient[row] = (0..BJT_INTERNAL_STATE_DIM)
                        .map(|inner| jacobian[inner][row] * current_state.3[inner])
                        .sum();
                }
                let lm_diag_scale = (0..BJT_INTERNAL_STATE_DIM)
                    .map(|idx| normal_matrix[idx][idx].abs())
                    .fold(1.0_f64, Value::max);
                for lambda_scale in [1e-10, 1e-8, 1e-6, 1e-4, 1e-2, 1.0, 1e2] {
                    let mut damped_normal = normal_matrix;
                    let lambda = lm_diag_scale * lambda_scale;
                    for idx in 0..BJT_INTERNAL_STATE_DIM {
                        damped_normal[idx][idx] += lambda;
                    }
                    let Some((lu_internal, pivots_internal)) =
                        Self::lu_decompose_small_dense_real(&damped_normal, BJT_INTERNAL_STATE_DIM)
                    else {
                        continue;
                    };
                    let rhs = gradient.map(|value| -value);
                    let Some(delta) = Self::lu_solve_small_dense_real(
                        &lu_internal,
                        &pivots_internal,
                        &rhs,
                        BJT_INTERNAL_STATE_DIM,
                    ) else {
                        continue;
                    };
                    let max_lm_delta = delta
                        .iter()
                        .fold(0.0_f64, |max_delta, value| max_delta.max(value.abs()));
                    if max_lm_delta < 1e-12 {
                        continue;
                    }
                    let mut target_internal = current_internal;
                    for idx in 0..BJT_INTERNAL_STATE_DIM {
                        target_internal[idx] = current_internal[idx] + delta[idx];
                    }
                    target_internal = Self::step_limit_vbic_dynamic_internal_target(
                        current_internal,
                        target_internal,
                        iteration,
                        current_state.4,
                    );
                    if let Some(candidate_state) =
                        Self::improve_vbic_dynamic_internal_state_toward_target(
                            bjt,
                            vc,
                            vb,
                            ve,
                            vs,
                            method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            current_internal,
                            current_state.4,
                            current_residual_objective,
                            target_internal,
                            current_internal,
                            12,
                        )
                        .map(|candidate_state| {
                            Self::refine_vbic_dynamic_static_core_with_fixed_delay(
                                bjt,
                                vc,
                                vb,
                                ve,
                                vs,
                                method,
                                trap_order,
                                dt,
                                q_prev,
                                q_prev_prev,
                                cq_prev,
                                candidate_state,
                                4,
                            )
                        })
                    {
                        let candidate_objective =
                            Self::vbic_dynamic_state_evaluation_residual_objective(
                                &candidate_state,
                            );
                        if next_state.as_ref().map_or(true, |best_state| {
                            candidate_objective + 1e-15
                                < Self::vbic_dynamic_state_evaluation_residual_objective(best_state)
                        }) {
                            next_state = Some(candidate_state);
                        }
                    }
                }

                let Some(next_state) = next_state else {
                    break;
                };

                current_state = next_state;
                current_residual_objective =
                    Self::vbic_dynamic_state_evaluation_residual_objective(&current_state);
            }

            seeded_snapshot = current_state.0;
            transient_linearization = current_state.1;
            base_static_g = current_state.2;
            current_residual_norm = current_state.4;
        }

        Some((
            seeded_snapshot,
            transient_linearization,
            base_static_g,
            current_residual_norm,
        ))
    }

    /// ngspice-style hidden-state delta check used for device-local bypass
    /// diagnostics. In ngspice, these `hat` comparisons decide whether the
    /// device can reuse the previous linearization; they are not a standalone
    /// Newton acceptance gate after the internal nodes have been reduced out of
    /// the global system.
    #[inline]
    pub(super) fn vbic_snapshot_convergence_met(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_snapshot: &BjtChargeSnapshot,
        criteria: NonlinearConvergenceCriteria,
    ) -> bool {
        let previous = bjt.vbic_transient_convergence_state_for_snapshot(
            previous_external[0],
            previous_external[1],
            previous_external[2],
            previous_external[3],
            previous_snapshot,
        );
        let current = bjt.vbic_transient_convergence_state_for_snapshot(
            current_external[0],
            current_external[1],
            current_external[2],
            current_external[3],
            current_snapshot,
        );
        let reltol = criteria.relative_tolerance();
        let voltage_tol = criteria.voltage_tolerance();
        let current_tol = criteria.current_tolerance();

        let voltages_converged = current.voltages.iter().zip(previous.voltages.iter()).all(
            |(current_voltage, previous_voltage)| {
                let diff = (current_voltage - previous_voltage).abs();
                let tol = reltol * current_voltage.abs().max(previous_voltage.abs()) + voltage_tol;
                diff <= tol
            },
        );
        if !voltages_converged {
            return false;
        }

        let mut delta_internal = [0.0; BJT_INTERNAL_STATE_DIM];
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            delta_internal[idx] = current_snapshot.reduction.internal_voltages[idx]
                - previous_snapshot.reduction.internal_voltages[idx];
        }

        (0..VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT).all(|branch_idx| {
            // Mirror ngspice's VBIC load-time bypass check: compare the full
            // predicted branch current against the candidate branch current,
            // including the hidden excess-phase xf2 contribution in iciei.
            // Excluding that term can accept a stale delayed-transport state
            // even when the candidate misses the device-local predictor tolerances.
            let predicted = previous.currents[branch_idx]
                + previous.d_currents_d_internal[branch_idx]
                    .iter()
                    .zip(delta_internal.iter())
                    .enumerate()
                    .filter(|(idx, _)| *idx != BJT_THERMAL_STATE_INDEX)
                    .map(|(_, (derivative, delta))| derivative * delta)
                    .sum::<Value>();
            let actual = current.currents[branch_idx];
            let tol = reltol * predicted.abs().max(actual.abs()) + current_tol;
            (predicted - actual).abs() <= tol
        })
    }

    #[inline]
    pub(super) fn vbic_local_candidate_is_acceptable(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        candidate_snapshot: &BjtChargeSnapshot,
        candidate_linearization: &VbicTransientLinearization,
    ) -> bool {
        if Self::vbic_dynamic_snapshot_solution_is_acceptable(
            candidate_linearization,
            &candidate_snapshot.reduction.external_voltages,
            &candidate_snapshot.reduction.internal_voltages,
        ) {
            return true;
        }

        // ngspice's VBIC path ultimately accepts or bypasses local updates based on
        // branch/voltage predictor tolerances (`*_hat` checks), not on a separate
        // reduced hidden-state residual. During our local continuation fallback, a
        // candidate that meets those ngspice-style device tolerances should be
        // accepted even when the reduced internal equations are stricter.
        Self::vbic_snapshot_convergence_met(
            bjt,
            previous_external,
            previous_snapshot,
            candidate_snapshot.reduction.external_voltages,
            candidate_snapshot,
            NonlinearConvergenceCriteria::default(),
        )
    }

    #[inline]
    pub(super) fn vbic_continuation_candidate_is_acceptable(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        candidate_snapshot: &BjtChargeSnapshot,
        candidate_linearization: &VbicTransientLinearization,
    ) -> bool {
        Self::vbic_local_candidate_is_acceptable(
            bjt,
            previous_external,
            previous_snapshot,
            candidate_snapshot,
            candidate_linearization,
        )
    }

    #[inline]
    pub(super) fn vbic_homotopy_candidate_is_acceptable(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_internal: [Value; BJT_INTERNAL_STATE_DIM],
        candidate_snapshot: &BjtChargeSnapshot,
        candidate_linearization: &VbicTransientLinearization,
    ) -> bool {
        let previous_snapshot = bjt.charge_snapshot_for_dynamic_state(
            external[0],
            external[1],
            external[2],
            external[3],
            previous_internal,
        );
        Self::vbic_local_candidate_is_acceptable(
            bjt,
            external,
            &previous_snapshot,
            candidate_snapshot,
            candidate_linearization,
        )
    }

    #[inline]
    pub(super) fn solve_vbic_dynamic_snapshot_for_continuation_step(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        let previous_internal = previous_snapshot.reduction.internal_voltages;
        let limited_live_seed = bjt.limit_vbic_dynamic_internal_state_to_previous(
            bjt.dynamic_internal_state_seed(vc, vb, ve, vs),
            previous_internal,
        );
        let seeded_result = Self::solve_vbic_dynamic_snapshot_best_effort(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            seed_internal,
        );
        let anchored_result = seed_internal
            .filter(|seed| {
                seed.iter()
                    .zip(previous_internal.iter())
                    .any(|(lhs, rhs)| (*lhs - *rhs).abs() > 1e-18)
            })
            .map(|_| {
                Self::solve_vbic_dynamic_snapshot_best_effort(
                    bjt,
                    vc,
                    vb,
                    ve,
                    vs,
                    method,
                    trap_order,
                    dt,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                    Some(&previous_internal),
                )
            })
            .unwrap_or(None);
        let live_result = seed_internal
            .filter(|seed| {
                seed.iter()
                    .zip(limited_live_seed.iter())
                    .any(|(lhs, rhs)| (*lhs - *rhs).abs() > 1e-18)
            })
            .map(|_| {
                Self::solve_vbic_dynamic_snapshot_best_effort(
                    bjt,
                    vc,
                    vb,
                    ve,
                    vs,
                    method,
                    trap_order,
                    dt,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                    Some(&limited_live_seed),
                )
            })
            .unwrap_or(None);
        let mut preferred_result = Self::choose_preferred_vbic_best_effort_result(
            seeded_result,
            anchored_result,
            |result| {
                Self::vbic_continuation_candidate_is_acceptable(
                    bjt,
                    previous_external,
                    previous_snapshot,
                    &result.0,
                    &result.1,
                )
            },
        );
        preferred_result = Self::choose_preferred_vbic_best_effort_result(
            preferred_result,
            live_result,
            |result| {
                Self::vbic_continuation_candidate_is_acceptable(
                    bjt,
                    previous_external,
                    previous_snapshot,
                    &result.0,
                    &result.1,
                )
            },
        );
        if let Some((snapshot, linearization, base_static_g, _residual_norm)) = preferred_result
            && Self::vbic_continuation_candidate_is_acceptable(
                bjt,
                previous_external,
                previous_snapshot,
                &snapshot,
                &linearization,
            )
        {
            return Some((snapshot, linearization, base_static_g));
        }

        Self::solve_vbic_dynamic_snapshot_with_excess_phase_homotopy(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            seed_internal,
        )
        .or_else(|| {
            Self::solve_vbic_dynamic_snapshot_with_collector_substrate_charge_homotopy(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )
        })
    }

    #[inline]
    pub(super) fn vbic_excess_phase_device_convergence_met(
        &self,
        circuit: &crate::circuit::Circuit,
        previous_solution: &[Value],
        current_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
    ) -> bool {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let criteria = self.device_convergence_criteria();

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if !bjt.uses_vbic_dynamic_charges() || bjt.td <= 0.0 {
                continue;
            }

            let previous_external = [
                Self::node_voltage(previous_solution, bjt.node_collector),
                Self::node_voltage(previous_solution, bjt.node_base),
                Self::node_voltage(previous_solution, bjt.node_emitter),
                Self::node_voltage(previous_solution, bjt.node_substrate),
            ];
            let current_external = [
                Self::node_voltage(current_solution, bjt.node_collector),
                Self::node_voltage(current_solution, bjt.node_base),
                Self::node_voltage(current_solution, bjt.node_emitter),
                Self::node_voltage(current_solution, bjt.node_substrate),
            ];

            let previous_snapshot = vbic_snapshot_cache
                .get(idx)
                .copied()
                .flatten()
                .filter(|snapshot| {
                    snapshot
                        .reduction
                        .external_voltages
                        .iter()
                        .zip(previous_external.iter())
                        .all(|(cached, expected)| (*cached - *expected).abs() <= 1e-18)
                })
                .or_else(|| {
                    let seed_internal =
                        Self::vbic_dynamic_internal_seed_from_history_with_linear_history(
                            bjt,
                            previous_external[0],
                            previous_external[1],
                            previous_external[2],
                            previous_external[3],
                            history.dynamic_internal_prev.get(idx),
                            history.dynamic_internal_prev_prev.get(idx),
                            history.dynamic_linear_prev.get(idx),
                            history.dynamic_linear_prev_prev.get(idx),
                            dt,
                            history.accepted_dt_prev,
                        );
                    Self::solve_vbic_dynamic_snapshot(
                        bjt,
                        previous_external[0],
                        previous_external[1],
                        previous_external[2],
                        previous_external[3],
                        effective_method,
                        trap_order,
                        dt,
                        &history.charge_q_prev[idx],
                        &history.charge_q_prev_prev[idx],
                        &history.charge_cq_prev[idx],
                        Some(&seed_internal),
                    )
                    .map(|(snapshot, _, _)| snapshot)
                });
            let Some(previous_snapshot) = previous_snapshot else {
                return false;
            };

            let current_snapshot = Self::solve_vbic_dynamic_snapshot(
                bjt,
                current_external[0],
                current_external[1],
                current_external[2],
                current_external[3],
                effective_method,
                trap_order,
                dt,
                &history.charge_q_prev[idx],
                &history.charge_q_prev_prev[idx],
                &history.charge_cq_prev[idx],
                Some(&previous_snapshot.reduction.internal_voltages),
            )
            .map(|(snapshot, _, _)| snapshot);
            let Some(current_snapshot) = current_snapshot else {
                return false;
            };

            if !Self::vbic_snapshot_convergence_met(
                bjt,
                previous_external,
                &previous_snapshot,
                current_external,
                &current_snapshot,
                criteria,
            ) {
                return false;
            }
        }

        true
    }

    #[inline]
    pub(super) fn transient_static_device_convergence_met(
        &self,
        circuit: &crate::circuit::Circuit,
        has_vbic_excess_phase: bool,
    ) -> bool {
        let criteria = self.device_convergence_criteria();

        circuit.diodes.all_converged(criteria)
            && circuit.mosfets.all_converged(criteria)
            && circuit.jfets.iter().all(|jfet| jfet.is_converged(criteria))
            && circuit.vswitches.iter().all(|sw| sw.is_converged(criteria))
            && circuit.iswitches.iter().all(|sw| sw.is_converged(criteria))
            && circuit.xspice_converged(criteria.voltage_tolerance())
            && circuit.bjts.devices.iter().all(|bjt| {
                if has_vbic_excess_phase && bjt.uses_vbic_dynamic_charges() && bjt.td > 0.0 {
                    true
                } else {
                    bjt.is_converged(criteria)
                }
            })
    }

    #[inline]
    pub(super) fn vbic_snapshot_matches_external_bias(
        snapshot: &BjtChargeSnapshot,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
        voltage_abstol: Value,
        reltol: Value,
    ) -> bool {
        Self::check_voltage_convergence_with_tolerances(
            &snapshot.reduction.external_voltages,
            external,
            voltage_abstol,
            reltol,
        )
    }

    #[inline]
    pub(super) fn vbic_snapshot_matches_external_bias_exact(
        snapshot: &BjtChargeSnapshot,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) -> bool {
        snapshot
            .reduction
            .external_voltages
            .iter()
            .zip(external.iter())
            .all(|(cached, expected)| (*cached - *expected).abs() <= 1e-18)
    }

    #[inline]
    pub(super) fn vbic_external_from_linear_history(
        bjt: &crate::device::Bjt,
        internal: &[Value; BJT_INTERNAL_STATE_DIM],
        linear: &VbicPredictorLinearBranchState,
    ) -> [Value; BJT_EXTERNAL_STATE_DIM] {
        let polarity = match bjt.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        };
        [
            internal[BJT_VCX_STATE_INDEX] + polarity * linear.vrcx,
            internal[BJT_VBX_STATE_INDEX] + polarity * linear.vrbx,
            internal[BJT_VEI_STATE_INDEX] + polarity * linear.vre,
            internal[BJT_VSI_STATE_INDEX] + polarity * linear.vrs,
        ]
    }

    #[inline]
    pub(super) fn continue_vbic_snapshot_to_external_bias(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> Option<BjtChargeSnapshot> {
        let current_snapshot = Self::solve_vbic_dynamic_snapshot(
            bjt,
            previous_external[0],
            previous_external[1],
            previous_external[2],
            previous_external[3],
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            Some(&previous_internal),
        )
        .map(|(snapshot, _, _)| snapshot)?;
        Self::continue_vbic_snapshot_to_external_bias_from_snapshot(
            bjt,
            current_snapshot,
            target_external,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        )
    }

    #[inline]
    pub(super) fn continue_vbic_snapshot_to_external_bias_from_snapshot(
        bjt: &crate::device::Bjt,
        current_snapshot: BjtChargeSnapshot,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> Option<BjtChargeSnapshot> {
        let continuation_started_at = std::time::Instant::now();
        let previous_external = current_snapshot.reduction.external_voltages;
        let mut current_external = previous_external;
        let mut current_snapshot = current_snapshot;
        let mut previous_accepted_external: Option<[Value; BJT_EXTERNAL_STATE_DIM]> = None;
        let mut previous_accepted_internal: Option<[Value; BJT_INTERNAL_STATE_DIM]> = None;
        let lambda_for_external = |external: [Value; BJT_EXTERNAL_STATE_DIM]| {
            for idx in 0..BJT_EXTERNAL_STATE_DIM {
                let total_delta = target_external[idx] - previous_external[idx];
                if total_delta.abs() > 1e-30 {
                    return ((external[idx] - previous_external[idx]) / total_delta)
                        .clamp(0.0, 1.0);
                }
            }
            1.0
        };
        let mut lambda: Value = lambda_for_external(current_external);
        let mut step: Value = Self::vbic_continuation_step_from_snapshot(
            bjt,
            current_external,
            current_snapshot.reduction.internal_voltages,
            target_external,
        );
        let initial_step = step;
        let mut solve_attempts = 0usize;
        let mut accepted_steps = 0usize;
        let mut rejected_steps = 0usize;

        while lambda < 1.0 - 1e-15 {
            let next_external = [
                current_external[BJT_EXT_C_INDEX]
                    + (target_external[BJT_EXT_C_INDEX] - current_external[BJT_EXT_C_INDEX]) * step,
                current_external[BJT_EXT_B_INDEX]
                    + (target_external[BJT_EXT_B_INDEX] - current_external[BJT_EXT_B_INDEX]) * step,
                current_external[BJT_EXT_E_INDEX]
                    + (target_external[BJT_EXT_E_INDEX] - current_external[BJT_EXT_E_INDEX]) * step,
                current_external[BJT_EXT_S_INDEX]
                    + (target_external[BJT_EXT_S_INDEX] - current_external[BJT_EXT_S_INDEX]) * step,
            ];
            let candidate_lambda = lambda_for_external(next_external);
            let previous_internal = current_snapshot.reduction.internal_voltages;
            let seed_internal = Self::vbic_continuation_seed_from_accepted_path(
                bjt,
                previous_accepted_external,
                previous_accepted_internal,
                current_external,
                previous_internal,
                next_external,
            );
            let attempt_started_at = std::time::Instant::now();
            let next_snapshot_result = Self::solve_vbic_dynamic_snapshot_for_continuation_step(
                bjt,
                current_external,
                &current_snapshot,
                next_external[0],
                next_external[1],
                next_external[2],
                next_external[3],
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                Some(&seed_internal),
            );
            solve_attempts += 1;
            let attempt_elapsed = attempt_started_at.elapsed();
            let Some((next_snapshot, next_linearization, _)) = next_snapshot_result else {
                rejected_steps += 1;
                if attempt_elapsed >= std::time::Duration::from_millis(50) {
                    log::warn!(
                        "Slow VBIC continuation solve {} step={:.6e} lambda={:.6e}->{:.6e} ext=({:.6e}, {:.6e}, {:.6e}, {:.6e}) elapsed={:.3?} result=failed",
                        bjt.name,
                        step,
                        lambda,
                        candidate_lambda,
                        next_external[0],
                        next_external[1],
                        next_external[2],
                        next_external[3],
                        attempt_elapsed,
                    );
                }
                let min_step = Self::vbic_continuation_min_remaining_step_scale(
                    current_external,
                    target_external,
                );
                if step <= min_step * (1.0 + 1e-12) {
                    let continuation_elapsed = continuation_started_at.elapsed();
                    if continuation_elapsed >= std::time::Duration::from_millis(100) {
                        log::warn!(
                            "Slow VBIC continuation {} attempts={} accepts={} rejects={} initial_step={:.6e} final_lambda={:.6e} elapsed={:.3?} status=failed",
                            bjt.name,
                            solve_attempts,
                            accepted_steps,
                            rejected_steps,
                            initial_step,
                            lambda,
                            continuation_elapsed,
                        );
                    }
                    return None;
                }
                step = (step * 0.5).max(min_step);
                continue;
            };
            let residual_norm = Self::vbic_internal_equation_residual_norm(
                &next_linearization,
                &next_snapshot.reduction.external_voltages,
                &next_snapshot.reduction.internal_voltages,
            );
            let accepted_strictly = Self::vbic_dynamic_snapshot_solution_is_acceptable(
                &next_linearization,
                &next_snapshot.reduction.external_voltages,
                &next_snapshot.reduction.internal_voltages,
            );
            let accepted_by_predictor = !accepted_strictly
                && Self::vbic_continuation_candidate_is_acceptable(
                    bjt,
                    current_external,
                    &current_snapshot,
                    &next_snapshot,
                    &next_linearization,
                );
            // ngspice's VBIC path keeps advancing when the candidate satisfies
            // its local branch/voltage predictor tolerances, even if the
            // reduced hidden-state solve is stricter. Mirror that behavior for
            // intermediate continuation steps, then do one final strict polish
            // at the exact target bias before returning the snapshot.
            if accepted_strictly || accepted_by_predictor {
                accepted_steps += 1;
                if attempt_elapsed >= std::time::Duration::from_millis(50) {
                    log::warn!(
                        "Slow VBIC continuation solve {} step={:.6e} lambda={:.6e}->{:.6e} ext=({:.6e}, {:.6e}, {:.6e}, {:.6e}) elapsed={:.3?} result=accepted mode={} residual={:.6e}",
                        bjt.name,
                        step,
                        lambda,
                        candidate_lambda,
                        next_external[0],
                        next_external[1],
                        next_external[2],
                        next_external[3],
                        attempt_elapsed,
                        if accepted_strictly {
                            "strict"
                        } else {
                            "ngspice"
                        },
                        residual_norm,
                    );
                }
                previous_accepted_external = Some(current_external);
                previous_accepted_internal = Some(previous_internal);
                current_external = next_external;
                current_snapshot = next_snapshot;
                lambda = candidate_lambda;
                if lambda >= 1.0 - 1e-15 {
                    break;
                }
                let suggested_step = Self::vbic_continuation_step_from_snapshot(
                    bjt,
                    current_external,
                    current_snapshot.reduction.internal_voltages,
                    target_external,
                );
                step = Self::vbic_continuation_step_after_accept(
                    current_external,
                    target_external,
                    step,
                    suggested_step,
                );
                continue;
            }

            rejected_steps += 1;
            if attempt_elapsed >= std::time::Duration::from_millis(50) {
                log::warn!(
                    "Slow VBIC continuation solve {} step={:.6e} lambda={:.6e}->{:.6e} ext=({:.6e}, {:.6e}, {:.6e}, {:.6e}) elapsed={:.3?} result=rejected residual={:.6e}",
                    bjt.name,
                    step,
                    lambda,
                    candidate_lambda,
                    next_external[0],
                    next_external[1],
                    next_external[2],
                    next_external[3],
                    attempt_elapsed,
                    residual_norm,
                );
            }
            let min_step =
                Self::vbic_continuation_min_remaining_step_scale(current_external, target_external);
            if step <= min_step * (1.0 + 1e-12) {
                let continuation_elapsed = continuation_started_at.elapsed();
                if continuation_elapsed >= std::time::Duration::from_millis(100) {
                    log::warn!(
                        "Slow VBIC continuation {} attempts={} accepts={} rejects={} initial_step={:.6e} final_lambda={:.6e} elapsed={:.3?} status=failed",
                        bjt.name,
                        solve_attempts,
                        accepted_steps,
                        rejected_steps,
                        initial_step,
                        lambda,
                        continuation_elapsed,
                    );
                }
                return None;
            }
            step = (step * 0.5).max(min_step);
        }

        let continuation_elapsed = continuation_started_at.elapsed();
        if continuation_elapsed >= std::time::Duration::from_millis(100) {
            log::warn!(
                "Slow VBIC continuation {} attempts={} accepts={} rejects={} initial_step={:.6e} final_lambda={:.6e} elapsed={:.3?} status=ok",
                bjt.name,
                solve_attempts,
                accepted_steps,
                rejected_steps,
                initial_step,
                lambda,
                continuation_elapsed,
            );
        }
        let _ = current_external;
        Self::finalize_vbic_continuation_target_snapshot(
            bjt,
            current_snapshot,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        )
    }

    #[inline]
    pub(super) fn vbic_continuation_min_remaining_step_scale(
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
    ) -> Value {
        let max_delta = current_external
            .iter()
            .zip(target_external.iter())
            .map(|(current, target)| (target - current).abs())
            .fold(0.0_f64, Value::max);
        if !max_delta.is_finite()
            || max_delta <= NonlinearConvergenceCriteria::default().voltage_tolerance()
        {
            return 1.0;
        }
        (NonlinearConvergenceCriteria::default().voltage_tolerance() / max_delta).clamp(1e-6, 1.0)
    }

    #[inline]
    pub(super) fn vbic_continuation_step_from_snapshot(
        bjt: &crate::device::Bjt,
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
    ) -> Value {
        let min_scale =
            Self::vbic_continuation_min_remaining_step_scale(current_external, target_external);
        let current_static_internal =
            Self::vbic_static_internal_state_from_dynamic(current_internal);
        let suggested_scale = bjt
            .vbic_external_step_limit_scale_from_state(
                current_external,
                current_static_internal,
                target_external,
            )
            .unwrap_or(1.0);
        if !suggested_scale.is_finite() {
            return min_scale;
        }
        let min_trial_scale = if bjt.uses_vbic_dynamic_charges() {
            Self::VBIC_CONTINUATION_MIN_TRIAL_STEP
        } else {
            0.0
        };
        suggested_scale.max(min_trial_scale).clamp(min_scale, 1.0)
    }

    #[inline]
    pub(super) fn vbic_continuation_step_after_accept(
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_step: Value,
        suggested_step: Value,
    ) -> Value {
        let min_step =
            Self::vbic_continuation_min_remaining_step_scale(current_external, target_external);
        (current_step * 2.0).min(suggested_step).max(min_step)
    }

    #[inline]
    pub(super) fn finalize_vbic_continuation_target_snapshot(
        bjt: &crate::device::Bjt,
        snapshot: BjtChargeSnapshot,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> Option<BjtChargeSnapshot> {
        let external = snapshot.reduction.external_voltages;
        let continuation_seed = snapshot.reduction.internal_voltages;
        let live_seed = bjt.dynamic_internal_state_seed(
            external[BJT_EXT_C_INDEX],
            external[BJT_EXT_B_INDEX],
            external[BJT_EXT_E_INDEX],
            external[BJT_EXT_S_INDEX],
        );
        // Once continuation has already advanced to the exact target bias via
        // ngspice-style local predictor acceptance, keep that accepted target
        // snapshot if a final strict hidden-state polish is unavailable. ngspice
        // does not require an extra reduced hidden-state solve before it can
        // proceed with the accepted local update.
        Self::solve_vbic_dynamic_snapshot(
            bjt,
            external[BJT_EXT_C_INDEX],
            external[BJT_EXT_B_INDEX],
            external[BJT_EXT_E_INDEX],
            external[BJT_EXT_S_INDEX],
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            Some(&continuation_seed),
        )
        .or_else(|| {
            Self::solve_vbic_dynamic_snapshot(
                bjt,
                external[BJT_EXT_C_INDEX],
                external[BJT_EXT_B_INDEX],
                external[BJT_EXT_E_INDEX],
                external[BJT_EXT_S_INDEX],
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                Some(&live_seed),
            )
        })
        .map(|(snapshot, _, _)| snapshot)
        .or(Some(snapshot))
    }

    #[inline]
    pub(super) fn vbic_continuation_seed_from_snapshot(
        bjt: &crate::device::Bjt,
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        bjt.predict_vbic_dynamic_internal_state_from_previous_external_bias(
            current_external,
            current_internal,
            target_external,
        )
        .unwrap_or_else(|| {
            let mut live_seed = bjt.dynamic_internal_state_seed(
                target_external[BJT_EXT_C_INDEX],
                target_external[BJT_EXT_B_INDEX],
                target_external[BJT_EXT_E_INDEX],
                target_external[BJT_EXT_S_INDEX],
            );
            if bjt.uses_vbic_dynamic_charges() {
                live_seed[BJT_DELAY_XF1_STATE_INDEX] = current_internal[BJT_DELAY_XF1_STATE_INDEX];
                live_seed[BJT_DELAY_XF2_STATE_INDEX] = current_internal[BJT_DELAY_XF2_STATE_INDEX];
            }
            if bjt.has_vbic_self_heating() {
                live_seed[BJT_THERMAL_STATE_INDEX] = current_internal[BJT_THERMAL_STATE_INDEX];
            }
            bjt.limit_vbic_dynamic_internal_state_to_previous(live_seed, current_internal)
        })
    }

    #[inline]
    pub(super) fn vbic_continuation_seed_from_accepted_path(
        bjt: &crate::device::Bjt,
        previous_external: Option<[Value; BJT_EXTERNAL_STATE_DIM]>,
        previous_internal: Option<[Value; BJT_INTERNAL_STATE_DIM]>,
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let mut seed = Self::vbic_continuation_seed_from_snapshot(
            bjt,
            current_external,
            current_internal,
            target_external,
        );
        let (Some(previous_external), Some(previous_internal)) =
            (previous_external, previous_internal)
        else {
            return seed;
        };

        let previous_step = current_external
            .iter()
            .zip(previous_external.iter())
            .map(|(current, previous)| (current - previous).abs())
            .fold(0.0_f64, Value::max);
        let proposed_step = target_external
            .iter()
            .zip(current_external.iter())
            .map(|(target, current)| (target - current).abs())
            .fold(0.0_f64, Value::max);
        if !previous_step.is_finite()
            || !proposed_step.is_finite()
            || previous_step <= 1e-30
            || proposed_step <= 1e-30
        {
            return seed;
        }

        // When the continuation loop has already shrunk the external step after
        // a rejection, the accepted-path predictor must be allowed to collapse
        // back toward the current accepted state. Keeping a positive floor here
        // forces an outsized internal extrapolation even for microscopic
        // follow-up steps and can starve the local reduced solve.
        let continuation_scale = (proposed_step / previous_step).clamp(0.0, 2.0);
        for idx in 0..BJT_THERMAL_STATE_INDEX {
            let path_predicted = current_internal[idx]
                + (current_internal[idx] - previous_internal[idx]) * continuation_scale;
            let path_delta = path_predicted - current_internal[idx];
            let snapshot_delta = seed[idx] - current_internal[idx];
            if path_delta.is_finite()
                && snapshot_delta.is_finite()
                && (snapshot_delta.abs() <= 1e-18
                    || path_delta.abs() <= 1e-18
                    || path_delta.signum() == snapshot_delta.signum())
                && path_delta.abs() > snapshot_delta.abs()
            {
                seed[idx] = path_predicted;
            }
        }
        if bjt.uses_vbic_dynamic_charges() && bjt.td > 0.0 {
            for idx in [BJT_DELAY_XF1_STATE_INDEX, BJT_DELAY_XF2_STATE_INDEX] {
                seed[idx] = current_internal[idx]
                    + (current_internal[idx] - previous_internal[idx]) * continuation_scale;
            }
        }
        if bjt.has_vbic_self_heating() {
            seed[BJT_THERMAL_STATE_INDEX] = (current_internal[BJT_THERMAL_STATE_INDEX]
                + (current_internal[BJT_THERMAL_STATE_INDEX]
                    - previous_internal[BJT_THERMAL_STATE_INDEX])
                    * continuation_scale)
                .max(bjt.minimum_thermal_rise());
        }

        bjt.limit_vbic_dynamic_internal_state_to_previous(seed, current_internal)
    }

    #[inline]
    pub(super) fn vbic_static_internal_state_from_dynamic(
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> [Value; 8] {
        [
            current_internal[BJT_VCX_STATE_INDEX],
            current_internal[BJT_VCI_STATE_INDEX],
            current_internal[BJT_VBX_STATE_INDEX],
            current_internal[BJT_VBI_STATE_INDEX],
            current_internal[BJT_VEI_STATE_INDEX],
            current_internal[BJT_VBP_STATE_INDEX],
            current_internal[BJT_VSI_STATE_INDEX],
            current_internal[BJT_THERMAL_STATE_INDEX],
        ]
    }

    #[inline]
    pub(super) fn resolve_vbic_snapshot_for_external_bias_with_linear_history(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        history_internal_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_internal_prev_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_linear_prev: Option<&VbicPredictorLinearBranchState>,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        previous_dt: Value,
        cached_snapshot: Option<BjtChargeSnapshot>,
        cache_reuse: VbicCachedSnapshotReuse,
        voltage_abstol: Value,
        reltol: Value,
    ) -> Option<BjtChargeSnapshot> {
        if !bjt.uses_vbic_dynamic_charges() {
            return Some(bjt.charge_snapshot(external[0], external[1], external[2], external[3]));
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let cached_snapshot_matches = |snapshot: &BjtChargeSnapshot| match cache_reuse {
            VbicCachedSnapshotReuse::SeedOnly => {
                Self::vbic_snapshot_matches_external_bias_exact(snapshot, &external)
            }
            VbicCachedSnapshotReuse::NewtonBypass => Self::vbic_snapshot_matches_external_bias(
                snapshot,
                &external,
                voltage_abstol,
                reltol,
            ),
        };
        if let Some(snapshot) = cached_snapshot.filter(cached_snapshot_matches) {
            return Some(snapshot);
        }
        if matches!(cache_reuse, VbicCachedSnapshotReuse::NewtonBypass)
            && let Some(cached_snapshot) = cached_snapshot.as_ref()
        {
            let candidate_snapshot = bjt.charge_snapshot_for_dynamic_state(
                external[0],
                external[1],
                external[2],
                external[3],
                cached_snapshot.reduction.internal_voltages,
            );
            if let Some(candidate_linearization) = Self::assemble_vbic_transient_linearization(
                bjt,
                &candidate_snapshot,
                effective_method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            ) && Self::vbic_local_candidate_is_acceptable(
                bjt,
                cached_snapshot.reduction.external_voltages,
                cached_snapshot,
                &candidate_snapshot,
                &candidate_linearization,
            ) {
                // Match ngspice-style device bypass: when the cached hidden state
                // remains locally acceptable at the new external bias, reuse it
                // directly instead of paying for another reduced hidden-state
                // solve during the same Newton timepoint.
                return Some(candidate_snapshot);
            }
        }

        let seed_internal = Self::vbic_dynamic_internal_seed_from_history_with_linear_history(
            bjt,
            external[0],
            external[1],
            external[2],
            external[3],
            history_internal_prev,
            history_internal_prev_prev,
            history_linear_prev,
            history_linear_prev_prev,
            dt,
            previous_dt,
        );
        let live_seed = bjt.limit_vbic_dynamic_internal_state_to_previous(
            bjt.dynamic_internal_state_seed(external[0], external[1], external[2], external[3]),
            seed_internal,
        );
        let cached_acceptance_reference =
            cached_snapshot.map(|snapshot| (snapshot.reduction.external_voltages, snapshot));
        let history_acceptance_reference = history_internal_prev.zip(history_linear_prev).map(
            |(history_internal_prev, history_linear_prev)| {
                let previous_external = Self::vbic_external_from_linear_history(
                    bjt,
                    history_internal_prev,
                    history_linear_prev,
                );
                let previous_snapshot = bjt.charge_snapshot_for_dynamic_state(
                    previous_external[0],
                    previous_external[1],
                    previous_external[2],
                    previous_external[3],
                    *history_internal_prev,
                );
                (previous_external, previous_snapshot)
            },
        );
        let bounded_snapshot_if_acceptable = |result: &VbicBestEffortSolve| {
            if !Self::vbic_dynamic_snapshot_best_effort_is_bounded(result) {
                return None;
            }

            let strict = Self::vbic_dynamic_snapshot_solution_is_acceptable(
                &result.1,
                &result.0.reduction.external_voltages,
                &result.0.reduction.internal_voltages,
            );
            let predictor_ok = cached_acceptance_reference
                .or(history_acceptance_reference)
                .map(|(previous_external, previous_snapshot)| {
                    Self::vbic_local_candidate_is_acceptable(
                        bjt,
                        previous_external,
                        &previous_snapshot,
                        &result.0,
                        &result.1,
                    )
                })
                .unwrap_or(false);

            (strict || predictor_ok).then_some(result.0)
        };
        // Match ngspice's predictor/load flow: first solve the current timepoint
        // directly from a predicted/live seed, and only fall back to local
        // continuation when that direct solve cannot produce a usable snapshot.
        if let Some((snapshot, _, _)) = Self::solve_vbic_dynamic_snapshot(
            bjt,
            external[0],
            external[1],
            external[2],
            external[3],
            effective_method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            Some(&seed_internal),
        ) {
            return Some(snapshot);
        }
        let mut bounded_best_effort = Self::choose_preferred_vbic_best_effort_result(
            Self::solve_vbic_dynamic_snapshot_best_effort(
                bjt,
                external[0],
                external[1],
                external[2],
                external[3],
                effective_method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                Some(&seed_internal),
            ),
            Some(&live_seed)
                .filter(|seed| {
                    seed.iter()
                        .zip(seed_internal.iter())
                        .any(|(lhs, rhs)| (*lhs - *rhs).abs() > 1e-18)
                })
                .and_then(|seed| {
                    Self::solve_vbic_dynamic_snapshot_best_effort(
                        bjt,
                        external[0],
                        external[1],
                        external[2],
                        external[3],
                        effective_method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        Some(seed),
                    )
                }),
            Self::vbic_dynamic_snapshot_best_effort_is_bounded,
        );

        if let Some(cached_snapshot) = cached_snapshot {
            let cached_internal = cached_snapshot.reduction.internal_voltages;
            if cached_internal
                .iter()
                .zip(seed_internal.iter())
                .any(|(cached, seeded)| (*cached - *seeded).abs() > 1e-15)
                && let Some((snapshot, _, _)) = Self::solve_vbic_dynamic_snapshot(
                    bjt,
                    external[0],
                    external[1],
                    external[2],
                    external[3],
                    effective_method,
                    trap_order,
                    dt,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                    Some(&cached_internal),
                )
            {
                return Some(snapshot);
            }
            bounded_best_effort = Self::choose_preferred_vbic_best_effort_result(
                bounded_best_effort,
                Some(cached_internal)
                    .filter(|seed| {
                        seed.iter()
                            .zip(seed_internal.iter())
                            .any(|(lhs, rhs)| (*lhs - *rhs).abs() > 1e-18)
                    })
                    .and_then(|seed| {
                        Self::solve_vbic_dynamic_snapshot_best_effort(
                            bjt,
                            external[0],
                            external[1],
                            external[2],
                            external[3],
                            effective_method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            Some(&seed),
                        )
                    }),
                Self::vbic_dynamic_snapshot_best_effort_is_bounded,
            );
            if let Some(result) = bounded_best_effort.as_ref()
                && let Some(snapshot) = bounded_snapshot_if_acceptable(result)
            {
                return Some(snapshot.clone());
            }
            if let Some(snapshot) = Self::continue_vbic_snapshot_to_external_bias_from_snapshot(
                bjt,
                cached_snapshot,
                external,
                effective_method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            ) {
                return Some(snapshot);
            }
        } else if let (Some(history_internal_prev), Some(history_linear_prev)) =
            (history_internal_prev, history_linear_prev)
        {
            if let Some(result) = bounded_best_effort.as_ref()
                && let Some(snapshot) = bounded_snapshot_if_acceptable(result)
            {
                return Some(snapshot.clone());
            }
            let previous_external = Self::vbic_external_from_linear_history(
                bjt,
                history_internal_prev,
                history_linear_prev,
            );
            if let Some(snapshot) = Self::continue_vbic_snapshot_to_external_bias(
                bjt,
                previous_external,
                *history_internal_prev,
                external,
                effective_method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            ) {
                return Some(snapshot);
            }
        }

        if let Some(result) = bounded_best_effort.as_ref()
            && let Some(snapshot) = bounded_snapshot_if_acceptable(result)
        {
            return Some(snapshot);
        }

        None
    }

    #[inline]
    pub(super) fn vbic_runtime_snapshot_reuse_tolerances(
        voltage_abstol: Value,
        reltol: Value,
    ) -> (Value, Value) {
        (voltage_abstol, reltol)
    }

    #[inline]
    pub(super) fn vbic_dynamic_internal_seed_from_linear_history(
        bjt: &crate::device::Bjt,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        history_internal_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_linear_prev: &VbicPredictorLinearBranchState,
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        Self::vbic_dynamic_internal_seed_from_predicted_linear_history(
            bjt,
            target_external,
            history_internal_prev,
            history_linear_prev,
            None,
            0.0,
            0.0,
        )
    }

    #[inline]
    pub(super) fn vbic_predictor_linear_branch_state_is_finite(
        linear: &VbicPredictorLinearBranchState,
    ) -> bool {
        [
            linear.vrcx,
            linear.vrci,
            linear.vrbx,
            linear.vrbi,
            linear.vre,
            linear.vrbp,
            linear.vrs,
        ]
        .iter()
        .all(|value| value.is_finite())
    }

    #[inline]
    pub(super) fn predict_vbic_linear_branch_state_from_history(
        history_linear_prev: &VbicPredictorLinearBranchState,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        dt: Value,
        previous_dt: Value,
    ) -> VbicPredictorLinearBranchState {
        let predict_component = |previous: Value, previous_previous: Option<Value>| {
            Self::predict_transient_history_value(previous, previous_previous, dt, previous_dt)
        };

        VbicPredictorLinearBranchState {
            vrcx: predict_component(
                history_linear_prev.vrcx,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrcx),
            ),
            vrci: predict_component(
                history_linear_prev.vrci,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrci),
            ),
            vrbx: predict_component(
                history_linear_prev.vrbx,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrbx),
            ),
            vrbi: predict_component(
                history_linear_prev.vrbi,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrbi),
            ),
            vre: predict_component(
                history_linear_prev.vre,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vre),
            ),
            vrbp: predict_component(
                history_linear_prev.vrbp,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrbp),
            ),
            vrs: predict_component(
                history_linear_prev.vrs,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrs),
            ),
        }
    }

    #[inline]
    pub(super) fn vbic_dynamic_internal_seed_from_predicted_linear_history(
        bjt: &crate::device::Bjt,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        history_internal_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_linear_prev: &VbicPredictorLinearBranchState,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        dt: Value,
        previous_dt: Value,
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        let predicted_linear = Self::predict_vbic_linear_branch_state_from_history(
            history_linear_prev,
            history_linear_prev_prev,
            dt,
            previous_dt,
        );
        if !Self::vbic_predictor_linear_branch_state_is_finite(&predicted_linear) {
            return None;
        }

        let polarity = match bjt.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        };

        let mut seed_internal = *history_internal_prev;
        seed_internal[BJT_VCX_STATE_INDEX] =
            target_external[BJT_EXT_C_INDEX] - polarity * predicted_linear.vrcx;
        seed_internal[BJT_VCI_STATE_INDEX] =
            seed_internal[BJT_VCX_STATE_INDEX] - polarity * predicted_linear.vrci;
        seed_internal[BJT_VBX_STATE_INDEX] =
            target_external[BJT_EXT_B_INDEX] - polarity * predicted_linear.vrbx;
        seed_internal[BJT_VBI_STATE_INDEX] =
            seed_internal[BJT_VBX_STATE_INDEX] - polarity * predicted_linear.vrbi;
        seed_internal[BJT_VEI_STATE_INDEX] =
            target_external[BJT_EXT_E_INDEX] - polarity * predicted_linear.vre;
        seed_internal[BJT_VBP_STATE_INDEX] =
            seed_internal[BJT_VCX_STATE_INDEX] + polarity * predicted_linear.vrbp;
        seed_internal[BJT_VSI_STATE_INDEX] =
            target_external[BJT_EXT_S_INDEX] - polarity * predicted_linear.vrs;

        seed_internal
            .iter()
            .take(BJT_THERMAL_STATE_INDEX)
            .all(|value| value.is_finite())
            .then(|| seed_internal)
    }

    #[inline]
    pub(super) fn vbic_dynamic_internal_seed_from_history_with_linear_history(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        history_internal_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_internal_prev_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_linear_prev: Option<&VbicPredictorLinearBranchState>,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        dt: Value,
        previous_dt: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let Some(history_internal_prev) = history_internal_prev else {
            return live_seed;
        };
        if !history_internal_prev.iter().all(|value| value.is_finite()) {
            return live_seed;
        }
        let history_internal_prev_prev = history_internal_prev_prev
            .filter(|history| history.iter().all(|value| value.is_finite()));
        let history_linear_prev = history_linear_prev
            .filter(|linear| Self::vbic_predictor_linear_branch_state_is_finite(linear));
        let history_linear_prev_prev = history_linear_prev_prev
            .filter(|linear| Self::vbic_predictor_linear_branch_state_is_finite(linear));

        // With `PREDICTOR`, ngspice seeds explicit VBIC unknowns from accepted
        // history before re-evaluating the device at the current iterate.
        // The reduced formulation does not expose ngspice's explicit internal
        // state vector, so reconstruct the hidden internal nodes from the
        // predicted accepted branch history first, then let the dynamic solve
        // refine that seed at the current external iterate.
        let target_external = [vc, vb, ve, vs];
        let mut seed_internal = history_linear_prev
            .and_then(|history_linear_prev| {
                Self::vbic_dynamic_internal_seed_from_predicted_linear_history(
                    bjt,
                    target_external,
                    history_internal_prev,
                    history_linear_prev,
                    history_linear_prev_prev,
                    dt,
                    previous_dt,
                )
            })
            .or_else(|| {
                history_linear_prev.and_then(|history_linear_prev| {
                    Self::vbic_dynamic_internal_seed_from_linear_history(
                        bjt,
                        target_external,
                        history_internal_prev,
                        history_linear_prev,
                    )
                })
            })
            .unwrap_or(live_seed);
        if bjt.uses_vbic_dynamic_charges() && bjt.td > 0.0 {
            // Match ngspice's MODEINITPRED behavior for VBIC excess-phase states:
            // xf1 stays anchored to the accepted state1 solution, while xf2 is
            // linearly extrapolated from accepted history.
            seed_internal[BJT_DELAY_XF1_STATE_INDEX] =
                history_internal_prev[BJT_DELAY_XF1_STATE_INDEX];
            seed_internal[BJT_DELAY_XF2_STATE_INDEX] = Self::predict_transient_history_value(
                history_internal_prev[BJT_DELAY_XF2_STATE_INDEX],
                history_internal_prev_prev.map(|history_internal_prev_prev| {
                    history_internal_prev_prev[BJT_DELAY_XF2_STATE_INDEX]
                }),
                dt,
                previous_dt,
            );
        }

        if bjt.has_vbic_self_heating() {
            seed_internal[BJT_THERMAL_STATE_INDEX] = Self::predict_transient_history_value(
                history_internal_prev[BJT_THERMAL_STATE_INDEX],
                history_internal_prev_prev.map(|history_internal_prev_prev| {
                    history_internal_prev_prev[BJT_THERMAL_STATE_INDEX]
                }),
                dt,
                previous_dt,
            )
            .max(bjt.minimum_thermal_rise());
        }

        bjt.limit_vbic_dynamic_internal_state_to_previous(seed_internal, *history_internal_prev)
    }
}
