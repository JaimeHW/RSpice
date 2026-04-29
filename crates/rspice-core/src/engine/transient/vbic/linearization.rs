//! VBIC transient linearization, thermal rebalance, and reduced-system assembly.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::transient) fn rebalance_vbic_dynamic_thermal_state(
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
    pub(in crate::engine::transient) fn vbic_transient_thermal_residual_and_derivative(
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
    pub(in crate::engine::transient) fn assemble_vbic_transient_linearization(
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
        let use_vbic_dynamic_charges = bjt.uses_vbic_dynamic_charges();

        if use_vbic_dynamic_charges {
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
            let (branch, ccap_history_sign) = if use_vbic_dynamic_charges {
                let Some(branch) =
                    Self::vbic_transient_owning_charge_branch(bjt, branch_idx, full_branch)
                else {
                    continue;
                };
                (
                    branch,
                    Self::vbic_transient_owning_charge_ccap_sign(bjt, branch_idx),
                )
            } else if branch_idx == BJT_QBE_BRANCH_INDEX {
                let capbe = -full_branch.d_internal[BJT_VEI_STATE_INDEX];
                let capbe_vbc = -full_branch.d_internal[BJT_VCI_STATE_INDEX];

                if capbe_vbc.is_finite() && capbe_vbc.abs() > 0.0 {
                    let geqcb = charge_factor * capbe_vbc;
                    let mut cross_branch = crate::device::semiconductor::BjtCurrentBranch {
                        pos_internal: full_branch.pos_internal,
                        neg_internal: full_branch.neg_internal,
                        pos_external: full_branch.pos_external,
                        neg_external: full_branch.neg_external,
                        ..Default::default()
                    };
                    cross_branch.d_internal[BJT_VBI_STATE_INDEX] = geqcb;
                    cross_branch.d_internal[BJT_VCI_STATE_INDEX] = -geqcb;
                    cross_branch.accumulate_derivatives(&mut g_ii, &mut g_ie, &mut g_ei, &mut g_ee);
                    let i_eq = cross_branch.linearization_dot(
                        &snapshot.reduction.internal_voltages,
                        &snapshot.reduction.external_voltages,
                    ) - cross_branch.current;
                    cross_branch.accumulate_source(i_eq, &mut z_i, &mut z_e);
                }

                if !capbe.is_finite() || capbe <= 0.0 {
                    continue;
                }

                let mut owning_branch = *full_branch;
                owning_branch.d_internal = [0.0; BJT_INTERNAL_STATE_DIM];
                owning_branch.d_external = [0.0; BJT_EXTERNAL_STATE_DIM];
                owning_branch.d_internal[BJT_VBI_STATE_INDEX] = capbe;
                owning_branch.d_internal[BJT_VEI_STATE_INDEX] = -capbe;
                (owning_branch, 1.0)
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
    pub(in crate::engine::transient) fn vbic_transient_owning_charge_branch(
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
    pub(in crate::engine::transient) fn vbic_transient_owning_charge_ccap_sign(
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
    pub(in crate::engine::transient) fn vbic_branch_voltage_charge_branch(
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
    pub(in crate::engine::transient) fn solve_vbic_internal_state_from_linearization(
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
    pub(in crate::engine::transient) fn solve_vbic_static_core_from_linearization(
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
    pub(in crate::engine::transient) fn vbic_internal_equation_residual(
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
    pub(in crate::engine::transient) fn vbic_internal_equation_residual_norm(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Value {
        Self::vbic_internal_equation_residual(linearization, external_voltages, internal_voltages)
            .into_iter()
            .fold(0.0, |max_norm, value| max_norm.max(value.abs()))
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_internal_equation_residual_objective(
        residual: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Value {
        residual
            .iter()
            .map(|value| value * value)
            .sum::<Value>()
            .sqrt()
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_dynamic_state_evaluation_residual_objective(
        evaluation: &VbicDynamicStateEvaluation,
    ) -> Value {
        Self::vbic_internal_equation_residual_objective(&evaluation.3)
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_dynamic_static_core_residual_norm(
        residual: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Value {
        residual[..BJT_STATIC_CORE_STATE_DIM]
            .iter()
            .fold(0.0_f64, |max_norm, value| max_norm.max(value.abs()))
    }

    #[inline]
    pub(in crate::engine::transient) fn refine_vbic_dynamic_static_core_with_fixed_delay(
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
    pub(in crate::engine::transient) fn vbic_reduce_transient_external_system(
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
    pub(in crate::engine::transient) fn vbic_static_stamped_external_system(
        bjt: &crate::device::Bjt,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) -> (
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        [Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        bjt.stamped_reduced_external_system(external[0], external[1], external[2], external[3])
    }
}
