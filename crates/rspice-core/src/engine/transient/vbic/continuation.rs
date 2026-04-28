//! VBIC continuation and history-based seed generation helpers.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::transient) fn solve_vbic_dynamic_snapshot_for_continuation_step(
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
    pub(in crate::engine::transient) fn vbic_external_from_linear_history(
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
    pub(in crate::engine::transient) fn continue_vbic_snapshot_to_external_bias(
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
    pub(in crate::engine::transient) fn continue_vbic_snapshot_to_external_bias_from_snapshot(
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
    pub(in crate::engine::transient) fn vbic_continuation_min_remaining_step_scale(
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
    pub(in crate::engine::transient) fn vbic_continuation_step_from_snapshot(
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
    pub(in crate::engine::transient) fn vbic_continuation_step_after_accept(
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
    pub(in crate::engine::transient) fn finalize_vbic_continuation_target_snapshot(
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
    pub(in crate::engine::transient) fn vbic_continuation_seed_from_snapshot(
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
    pub(in crate::engine::transient) fn vbic_continuation_seed_from_accepted_path(
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
    pub(in crate::engine::transient) fn vbic_static_internal_state_from_dynamic(
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
    pub(in crate::engine::transient) fn resolve_vbic_snapshot_for_external_bias_with_linear_history(
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
    pub(in crate::engine::transient) fn vbic_dynamic_internal_seed_from_linear_history(
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
    pub(in crate::engine::transient) fn vbic_predictor_linear_branch_state_is_finite(
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
    pub(in crate::engine::transient) fn predict_vbic_linear_branch_state_from_history(
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
    pub(in crate::engine::transient) fn vbic_dynamic_internal_seed_from_predicted_linear_history(
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
    pub(in crate::engine::transient) fn vbic_dynamic_internal_seed_from_history_with_linear_history(
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
