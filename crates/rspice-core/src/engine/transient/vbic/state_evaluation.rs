//! VBIC dynamic-state evaluation, step limiting, and predictor helpers.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::transient) fn vbic_dynamic_snapshot_best_effort_is_bounded(
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
    pub(in crate::engine::transient) fn vbic_dynamic_snapshot_solution_is_acceptable(
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
    pub(in crate::engine::transient) fn choose_preferred_vbic_best_effort_result<F>(
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
    pub(in crate::engine::transient) fn vbic_dynamic_internal_state_step_limit_for_index(
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
    pub(in crate::engine::transient) fn step_limit_vbic_dynamic_internal_target(
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
    pub(in crate::engine::transient) fn vbic_predictor_linear_branch_state(
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
    pub(in crate::engine::transient) fn evaluate_vbic_dynamic_internal_state(
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

    pub(in crate::engine::transient) fn improve_vbic_dynamic_internal_state_toward_target(
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
                && best_state.as_ref().is_none_or(|best_state| {
                    candidate_objective + 1e-15
                        < Self::vbic_dynamic_state_evaluation_residual_objective(best_state)
                })
            {
                best_state = Some(candidate_state);
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
}
