//! Newton step-control and transient convergence guard helpers.

use super::*;

impl Engine {
    #[inline]
    pub(super) fn max_abs_delta_prefix(a: &[Value], b: &[Value], count: usize) -> Value {
        a.iter()
            .zip(b.iter())
            .take(count)
            .map(|(x, y)| (x - y).abs())
            .fold(0.0, Value::max)
    }

    #[inline]
    pub(super) fn top_abs_delta_prefix_named(
        a: &[Value],
        b: &[Value],
        node_names: &[String],
        count: usize,
        max_items: usize,
    ) -> Vec<String> {
        let mut deltas: Vec<(usize, Value)> = a
            .iter()
            .zip(b.iter())
            .take(count)
            .enumerate()
            .map(|(idx, (x, y))| (idx, (x - y).abs()))
            .filter(|(_, delta)| delta.is_finite() && *delta > 0.0)
            .collect();
        deltas.sort_by(|lhs, rhs| rhs.1.total_cmp(&lhs.1));
        deltas
            .into_iter()
            .take(max_items)
            .map(|(idx, delta)| {
                let name = node_names
                    .get(idx)
                    .map(String::as_str)
                    .unwrap_or("<unnamed>");
                format!("{name}:{delta:.3e}")
            })
            .collect()
    }

    #[inline]
    pub(super) fn max_abs_delta(a: &[Value], b: &[Value]) -> Value {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .fold(0.0, Value::max)
    }

    #[inline]
    pub(super) fn max_abs_delta_branch_ordinals(
        a: &[Value],
        b: &[Value],
        num_nodes: usize,
        branch_ordinals: &[crate::NodeId],
    ) -> Value {
        branch_ordinals
            .iter()
            .filter_map(|branch_ordinal| {
                let idx = num_nodes.checked_add(branch_ordinal.checked_sub(1)?)?;
                let x = *a.get(idx)?;
                let y = *b.get(idx)?;
                Some((x - y).abs())
            })
            .fold(0.0, Value::max)
    }

    #[inline]
    pub(super) fn is_stagnant_force_candidate(
        circuit: &crate::circuit::Circuit,
        previous_solution: &[Value],
        candidate_solution: &[Value],
        num_nodes: usize,
        voltage_tolerance: Value,
        current_tolerance: Value,
    ) -> bool {
        let node_threshold = voltage_tolerance.max(1e-18);
        let node_delta =
            Self::max_abs_delta_prefix(previous_solution, candidate_solution, num_nodes);
        if node_delta > node_threshold {
            return false;
        }

        // Inductor branch-current unknowns can show physical state progression
        // when node voltages remain fixed. Capacitor-only decks carry their
        // dynamic state in node voltages, so a tiny node delta there is not
        // enough evidence to reject an otherwise bounded recovery step.
        if circuit.inductors.branch_indices.is_empty() {
            return circuit.capacitors.is_empty();
        }

        let current_threshold = current_tolerance.max(1e-18);
        let dynamic_current_delta = Self::max_abs_delta_branch_ordinals(
            previous_solution,
            candidate_solution,
            num_nodes,
            &circuit.inductors.branch_indices,
        );
        dynamic_current_delta <= current_threshold
    }

    #[inline]
    pub(super) fn is_stale_step(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        expected_source_delta: Value,
        num_nodes: usize,
    ) -> bool {
        // Only police stale accepts when sources are strongly active on this step.
        // Weak source movement can legitimately yield tiny accepted deltas in
        // high-rejection circuits (for example differential stages).
        if expected_source_delta <= SOURCE_ACTIVE_DELTA {
            return false;
        }

        // If the entire solution moves orders of magnitude less than the source
        // should have moved, the solver likely accepted a stale state.
        let observed_delta =
            Self::max_abs_delta_prefix(previous_solution, candidate_solution, num_nodes);
        observed_delta <= expected_source_delta * 1e-3
    }

    #[inline]
    pub(super) fn is_unbounded_step(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        expected_source_delta: Value,
        num_nodes: usize,
        protected_nodes: &[bool],
    ) -> bool {
        let observed_delta = previous_solution
            .iter()
            .zip(candidate_solution.iter())
            .take(num_nodes)
            .enumerate()
            .filter(|(idx, _)| !protected_nodes.get(*idx).copied().unwrap_or(false))
            .map(|(_, (x, y))| (x - y).abs())
            .fold(0.0, Value::max);
        // Guard only truly explosive step jumps. We intentionally allow bounded
        // multi-volt recovery movement here because force-accept paths may need
        // larger-than-source-following corrections to escape stiff NR stalls.
        let drive_scale = expected_source_delta.max(1e-6);
        let threshold = (drive_scale * 1e5).max(50.0);
        observed_delta > threshold
    }

    #[inline]
    pub(super) fn is_excessive_quiet_force_candidate(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        expected_source_delta: Value,
        num_nodes: usize,
        clip_limit: Value,
    ) -> bool {
        if expected_source_delta > SOURCE_ACTIVE_DELTA {
            return false;
        }
        if !(clip_limit.is_finite() && clip_limit > 0.0) {
            return false;
        }
        let observed_delta =
            Self::max_abs_delta_prefix(previous_solution, candidate_solution, num_nodes);
        let quiet_limit = (expected_source_delta.max(0.0) * 16.0)
            .max(clip_limit * 0.25)
            .min(clip_limit * 0.5);
        observed_delta >= quiet_limit
    }

    #[inline]
    pub(super) fn is_clipped_force_candidate(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        num_nodes: usize,
        clip_limit: Value,
    ) -> bool {
        let clip_threshold = clip_limit * 0.99;
        let mut clipped = 0usize;

        for (old_v, new_v) in previous_solution
            .iter()
            .zip(candidate_solution.iter())
            .take(num_nodes)
        {
            let delta = *new_v - *old_v;
            if delta.abs() >= clip_threshold {
                clipped += 1;
            }
        }

        if clipped < 2 {
            return false;
        }

        let min_clipped_nodes = (num_nodes / 2).max(2);
        clipped >= min_clipped_nodes
    }

    #[inline]
    pub(super) fn transient_newton_iteration_budget(
        transient_max_iterations: usize,
        startup_recovery: bool,
    ) -> usize {
        // ngspice's NIiter() raises any smaller iteration limit to 100
        // (niiter.c) for every Newton solve, transient timepoints included.
        let standard_budget = transient_max_iterations
            .max(1)
            .max(NGSPICE_NIITER_MIN_ITERATIONS);
        if startup_recovery {
            standard_budget.min(128)
        } else {
            standard_budget
        }
    }

    #[inline]
    pub(super) fn adaptive_transient_newton_delta_limit(
        base_limit: Value,
        iteration: usize,
    ) -> Value {
        if !(base_limit.is_finite() && base_limit > 0.0) {
            return base_limit;
        }

        let growth_stage = iteration.saturating_sub(4) / 4;
        let multiplier = 2.0_f64.powi(growth_stage.min(5) as i32);
        (base_limit * multiplier).min(MAX_ADAPTIVE_NEWTON_ITER_DELTA_V)
    }

    #[inline]
    pub(super) fn bias_transient_step_for_source_activity(
        proposed_dt: Value,
        remaining_time: Value,
        at_breakpoint: bool,
        expected_source_delta: Value,
        interior_source_delta: Value,
        practical_min_dt: Value,
        preferred_min_dt: Value,
        recovery_cap_enabled: bool,
        nonlinear_source_ramp_cap_enabled: bool,
    ) -> Value {
        let mut dt = proposed_dt.min(remaining_time);
        let source_is_moving_before_endpoint =
            !at_breakpoint || interior_source_delta >= SOURCE_ACTIVE_DELTA;
        if nonlinear_source_ramp_cap_enabled
            && expected_source_delta.is_finite()
            && expected_source_delta > SOURCE_RAMP_TRACKING_DELTA
            && source_is_moving_before_endpoint
        {
            let ramp_cap = dt * (SOURCE_RAMP_TRACKING_DELTA / expected_source_delta);
            if ramp_cap.is_finite() && ramp_cap > 0.0 {
                dt = dt.min(ramp_cap.max(practical_min_dt));
            }
        }

        if at_breakpoint || !recovery_cap_enabled {
            return dt;
        }

        if expected_source_delta >= SOURCE_ACTIVE_DELTA {
            // Only recovery paths get this extra source-following cap. Normal
            // accepted-step progression is already governed by ngspice-style
            // breakpoints and truncation; shrinking every smooth ramp step here
            // phase-shifts otherwise converged waveforms.
            let active_cap = (preferred_min_dt / 8.0).max(practical_min_dt);
            if dt > active_cap {
                dt = active_cap;
            }
        }

        dt
    }

    #[inline]
    pub(super) fn should_apply_active_source_recovery_cap(force_accept_cooldown: usize) -> bool {
        force_accept_cooldown > 0
    }

    #[inline]
    pub(super) fn node_voltage(solution: &[Value], node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            solution.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    pub(super) fn differential_voltage(
        solution: &[Value],
        node_pos: usize,
        node_neg: usize,
    ) -> Value {
        Self::node_voltage(solution, node_pos) - Self::node_voltage(solution, node_neg)
    }

    #[inline]
    pub(super) fn differential_port_voltages(
        solution: &[Value],
        nodes: &[usize],
        reference: usize,
    ) -> Vec<Value> {
        let reference_voltage = Self::node_voltage(solution, reference);
        nodes
            .iter()
            .map(|&node| Self::node_voltage(solution, node) - reference_voltage)
            .collect()
    }
}
