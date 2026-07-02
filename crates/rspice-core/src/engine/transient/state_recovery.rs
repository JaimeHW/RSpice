//! Transient retry, timestep recovery, and solution-limiting helpers.

use super::*;

impl Engine {
    #[inline]
    pub(super) fn recover_timestep_after_accepted_step(
        timestep: &mut TimestepController,
        lte_estimator: &LteEstimator,
        accepted_solution: &[Value],
        dt: Value,
        max_step: Value,
        is_strictly_linear_transient: bool,
        expected_source_delta: Value,
        source_activity_growth_cap_enabled: bool,
        accepted_scale: Option<Value>,
    ) {
        // Strictly linear steps are solved directly, so they can recover from
        // breakpoint restart steps faster than Newton-limited nonlinear decks.
        let scale = if is_strictly_linear_transient {
            4.0
        } else if let Some(scale) = accepted_scale {
            scale
        } else {
            let (lte, _) = lte_estimator.estimate(accepted_solution, dt);
            lte_estimator.recommend_scale(lte)
        };

        let growth_limit = if is_strictly_linear_transient {
            4.0
        } else if source_activity_growth_cap_enabled {
            1.5
        } else {
            2.0
        };
        let mut next_dt = if scale > 1.0 {
            (dt * scale.min(growth_limit)).min(max_step)
        } else {
            (dt * 1.25).min(max_step)
        };
        if source_activity_growth_cap_enabled
            && expected_source_delta.is_finite()
            && expected_source_delta > 0.0
        {
            let source_cap = dt * (SOURCE_ACTIVE_DELTA / expected_source_delta).clamp(1.0, 4.0);
            next_dt = next_dt.min(source_cap);
        }
        timestep.force_step(next_dt);
    }

    #[inline]
    pub(super) fn nonconvergence_retry_timestep(dt: Value, max_step: Value) -> Value {
        if !dt.is_finite() || dt <= 0.0 {
            0.0
        } else {
            // Mirror ngspice's DCtran non-convergence recovery: retract the
            // failed timepoint and retry at one eighth of the rejected step.
            (dt * 0.125).min(max_step)
        }
    }

    #[inline]
    pub(super) fn apply_retry_timestep_floor(
        proposed_dt: Value,
        retry_floor_dt: Option<Value>,
        rejected_dt: Value,
        max_step: Value,
    ) -> Value {
        let rejected_cap = if rejected_dt.is_finite() && rejected_dt > 0.0 {
            rejected_dt.min(max_step)
        } else {
            max_step
        };
        let mut dt = proposed_dt.min(rejected_cap);
        if let Some(floor) = retry_floor_dt
            .filter(|floor| floor.is_finite() && *floor > 0.0)
            .map(|floor| floor.min(rejected_cap))
            .filter(|floor| *floor < rejected_cap * 0.999)
        {
            dt = dt.max(floor);
        }
        dt
    }

    #[inline]
    pub(super) fn is_at_effective_retry_minimum(
        timestep: &TimestepController,
        _retry_floor_dt: Option<Value>,
    ) -> bool {
        // The BJT retry floor is intentionally soft: it may damp the first
        // retreat from a large failed step, but it must not masquerade as the
        // solver's true minimum and trigger force-accept before Newton has had
        // a chance to retry at smaller physical timesteps.
        timestep.is_at_minimum()
    }

    #[inline]
    pub(super) fn should_skip_post_accept_timestep_control_on_first_step(
        accepted_point_count_before_push: usize,
    ) -> bool {
        // ngspice accepts the first transient point without any post-accept
        // truncation/LTE check, then retries the same delta on the next step.
        accepted_point_count_before_push <= 1
    }

    #[inline]
    pub(super) fn force_accept_recovery_timestep(
        dt: Value,
        preferred_min_dt: Value,
        max_step: Value,
        vbic_exact_limit: Option<Value>,
    ) -> Value {
        let mut next_dt = if dt.is_finite() && dt > 0.0 {
            if dt < preferred_min_dt {
                (dt * preferred_min_dt)
                    .sqrt()
                    .max(dt)
                    .min(preferred_min_dt)
                    .min(max_step)
            } else {
                (dt * 0.5).max(preferred_min_dt).min(max_step)
            }
        } else {
            preferred_min_dt.min(max_step)
        };
        if let Some(limit) =
            vbic_exact_limit.filter(|limit| limit.is_finite() && *limit > dt * 1.001)
        {
            next_dt = next_dt.min(limit.min(max_step));
        }
        next_dt
    }

    #[inline]
    pub(super) fn limit_transient_node_voltage_updates(
        proposal: &mut [Value],
        previous: &[Value],
        num_nodes: usize,
        delta_limit: Value,
        protected_nodes: &[bool],
    ) -> bool {
        let mut changed = false;
        for i in 0..num_nodes {
            if protected_nodes.get(i).copied().unwrap_or(false) {
                continue;
            }
            let old = previous[i];
            let delta = proposal[i] - old;
            if delta.is_finite() && delta.abs() > delta_limit {
                proposal[i] = old + delta.signum() * delta_limit;
                changed = true;
            }
        }
        changed
    }

    #[inline]
    pub(super) fn bounded_force_accept_candidate(
        circuit: &crate::circuit::Circuit,
        previous_solution: &[Value],
        candidate_solution: &[Value],
        accepted_time: Value,
        num_nodes: usize,
        force_accept_delta_limit: Value,
        protected_nodes: &[bool],
        ideal_output_pairs: &[(crate::NodeId, crate::NodeId)],
    ) -> Vec<Value> {
        let mut bounded = candidate_solution.to_vec();
        for i in 0..num_nodes {
            if protected_nodes.get(i).copied().unwrap_or(false) {
                continue;
            }
            let old = previous_solution[i];
            let delta = bounded[i] - old;
            if delta.is_finite() && delta.abs() > force_accept_delta_limit {
                bounded[i] = old + delta.signum() * force_accept_delta_limit;
            }
        }
        circuit.enforce_ideal_voltage_constraints(&mut bounded, accepted_time);
        // Force-accept is a last-resort recovery path, so keep every ideal
        // output supernode close to the previous accepted common mode instead of
        // letting protected source nodes drag a nonphysical midpoint into the
        // newly accepted state.
        Self::clip_ideal_output_common_modes(
            previous_solution,
            &mut bounded,
            force_accept_delta_limit,
            ideal_output_pairs,
        );
        Self::restore_algebraic_branch_currents(
            circuit,
            previous_solution,
            &mut bounded,
            num_nodes,
        );
        bounded
    }

    #[inline]
    pub(super) fn restore_algebraic_branch_currents(
        circuit: &crate::circuit::Circuit,
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        num_nodes: usize,
    ) {
        let mut restore_branch = |branch_ordinal: usize| {
            if branch_ordinal == 0 {
                return;
            }
            let Some(solution_idx) = num_nodes.checked_add(branch_ordinal - 1) else {
                return;
            };
            let Some(previous_value) = previous_solution.get(solution_idx).copied() else {
                return;
            };
            if let Some(candidate_value) = candidate_solution.get_mut(solution_idx) {
                *candidate_value = previous_value;
            }
        };

        for &branch_ordinal in &circuit.voltage_sources.branch_indices {
            restore_branch(branch_ordinal as usize);
        }
        // Keep dependent-source algebraic currents from the latest solver
        // candidate. Their output-branch currents directly close KCL at the
        // controlled output nodes, so blindly snapping them back to the
        // previously accepted state can inject a large node residual even when
        // the committed output voltage satisfies the ideal source relation.
    }

    #[inline]
    pub(super) fn clip_ideal_output_common_modes(
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        common_mode_delta_limit: Value,
        ideal_output_pairs: &[(crate::NodeId, crate::NodeId)],
    ) {
        for &(node_pos, node_neg) in ideal_output_pairs {
            Self::clip_two_terminal_common_mode(
                previous_solution,
                candidate_solution,
                node_pos,
                node_neg,
                common_mode_delta_limit,
            );
        }
    }

    #[inline]
    pub(super) fn clip_two_terminal_common_mode(
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        node_pos: usize,
        node_neg: usize,
        common_mode_delta_limit: Value,
    ) {
        if common_mode_delta_limit <= 0.0 || node_pos == 0 || node_neg == 0 {
            return;
        }

        let vp_idx = node_pos - 1;
        let vn_idx = node_neg - 1;
        if vp_idx >= previous_solution.len()
            || vn_idx >= previous_solution.len()
            || vp_idx >= candidate_solution.len()
            || vn_idx >= candidate_solution.len()
        {
            return;
        }

        let prev_vp = previous_solution[vp_idx];
        let prev_vn = previous_solution[vn_idx];
        let cand_vp = candidate_solution[vp_idx];
        let cand_vn = candidate_solution[vn_idx];
        if !(prev_vp.is_finite()
            && prev_vn.is_finite()
            && cand_vp.is_finite()
            && cand_vn.is_finite())
        {
            return;
        }

        let prev_midpoint = 0.5 * (prev_vp + prev_vn);
        let cand_midpoint = 0.5 * (cand_vp + cand_vn);
        let midpoint_delta = cand_midpoint - prev_midpoint;
        if midpoint_delta.abs() <= common_mode_delta_limit {
            return;
        }

        let clipped_midpoint = prev_midpoint + midpoint_delta.signum() * common_mode_delta_limit;
        let half_diff = 0.5 * (cand_vp - cand_vn);
        candidate_solution[vp_idx] = clipped_midpoint + half_diff;
        candidate_solution[vn_idx] = clipped_midpoint - half_diff;
    }
}
