//! VBIC dynamic snapshot solve paths and homotopy fallbacks.

#![allow(clippy::needless_range_loop)]

use super::*;
use std::cell::Cell;

thread_local! {
    /// Optional DETERMINISTIC iteration budget for nested best-effort device
    /// solves. When a bounded caller (the continuation loop) sets a budget, the
    /// expensive refinement / Levenberg-Marquardt loops stop early once the
    /// shared device-evaluation count is exhausted and return their best state
    /// so far, rather than burning many seconds exhausting full iteration budgets
    /// on a target the outer Newton will reject anyway. The budget is a pure
    /// evaluation COUNT (not wall-clock), so simulation output stays bit-for-bit
    /// deterministic across machines and runs. `None` (the default) preserves the
    /// original unbounded behavior for direct per-timepoint solves.
    static VBIC_BEST_EFFORT_EVAL_BUDGET: Cell<Option<u32>> = const { Cell::new(None) };
}

impl Engine {
    /// Total device-state evaluations a single bounded continuation to an
    /// external bias may spend across all its nested best-effort solves and
    /// homotopy fallbacks. A healthy 10-state reduced solve converges in well
    /// under a hundred evaluations; this generous ceiling only trips for
    /// genuinely unreachable targets (a diverging outer Newton iterate during a
    /// violent turn-on), where stopping promptly lets the outer loop reject the
    /// step and reduce dt instead of stalling for tens of seconds. Chosen by
    /// measurement: see the `measure_vbic_*` harness — passing decks
    /// (FO/CEamp/temp/diffamp healthy steps) stay far below this; the diffamp
    /// turn-on previously consumed >100k evaluations per continuation (~18s).
    pub(in crate::engine::transient) const VBIC_CONTINUATION_EVAL_BUDGET: u32 = 20_000;

    /// Run `body` with a deterministic device-evaluation budget applied to any
    /// nested best-effort device solve, restoring the previous budget afterward.
    #[inline]
    pub(in crate::engine::transient) fn with_vbic_best_effort_eval_budget<T>(
        budget: u32,
        body: impl FnOnce() -> T,
    ) -> T {
        let previous = VBIC_BEST_EFFORT_EVAL_BUDGET.with(|cell| {
            let previous = cell.get();
            // Nest conservatively: never raise an outer (smaller) budget.
            let effective = match previous {
                Some(existing) if existing < budget => existing,
                _ => budget,
            };
            cell.set(Some(effective));
            previous
        });
        let result = body();
        VBIC_BEST_EFFORT_EVAL_BUDGET.with(|cell| cell.set(previous));
        result
    }

    /// Charge one device-state evaluation against the active budget (if any) and
    /// report whether the budget has been exhausted. Always returns `false` when
    /// no budget is active (unbounded direct solves).
    #[inline]
    pub(in crate::engine::transient) fn vbic_best_effort_eval_budget_exhausted() -> bool {
        VBIC_BEST_EFFORT_EVAL_BUDGET.with(|cell| match cell.get() {
            Some(remaining) => {
                if remaining == 0 {
                    true
                } else {
                    cell.set(Some(remaining - 1));
                    false
                }
            }
            None => false,
        })
    }

    #[inline]
    pub(in crate::engine::transient) fn solve_vbic_dynamic_snapshot(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        coeff: &CompanionCoefficients,
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
            coeff,
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
                coeff,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )
        })
    }

    #[inline]
    pub(in crate::engine::transient) fn solve_vbic_dynamic_snapshot_primary(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        coeff: &CompanionCoefficients,
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
            coeff,
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
                coeff,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )
        })
    }

    pub(in crate::engine::transient) fn solve_vbic_dynamic_snapshot_with_collector_substrate_charge_homotopy(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        coeff: &CompanionCoefficients,
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
                coeff,
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
                coeff,
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

    pub(in crate::engine::transient) fn solve_vbic_dynamic_snapshot_with_excess_phase_homotopy(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        coeff: &CompanionCoefficients,
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
            coeff,
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
            coeff,
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
                coeff,
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
                coeff,
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
                        coeff,
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
                        coeff,
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
    pub(in crate::engine::transient) fn solve_vbic_dynamic_snapshot_direct(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        coeff: &CompanionCoefficients,
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
                coeff,
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
    pub(in crate::engine::transient) fn solve_vbic_dynamic_snapshot_best_effort(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        coeff: &CompanionCoefficients,
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
            coeff,
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
            coeff,
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
            coeff,
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
            if Self::vbic_best_effort_eval_budget_exhausted() {
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
                coeff,
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
                coeff,
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
                coeff,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seeded_snapshot.reduction.internal_voltages,
            )
            .unwrap_or((
                seeded_snapshot,
                transient_linearization,
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
                if Self::vbic_best_effort_eval_budget_exhausted() {
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
                        coeff,
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
                            coeff,
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
                        coeff,
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
                            coeff,
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
                        coeff,
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
                            coeff,
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
                    if next_state.as_ref().is_none_or(|best_state| {
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
                            coeff,
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
                                coeff,
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
                        if next_state.as_ref().is_none_or(|best_state| {
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
}
