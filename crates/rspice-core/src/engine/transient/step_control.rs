//! Newton step-control and transient convergence guard helpers.

use super::*;

/// How far the independent sources are expected to move over a candidate step:
/// across it, inside it, and along the ramp being tracked.
#[derive(Clone, Copy)]
pub(super) struct SourceActivityDeltas {
    pub expected_source_delta: Value,
    pub interior_source_delta: Value,
    pub source_ramp_tracking_delta: Value,
}

/// The floors and caps the source-activity bias may not cross.
#[derive(Clone, Copy)]
pub(super) struct StepBiasFloors {
    pub practical_min_dt: Value,
    pub preferred_min_dt: Value,
    pub recovery_cap_enabled: bool,
    pub nonlinear_source_ramp_cap_enabled: bool,
}

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
        circuit: &crate::circuit::CircuitData,
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

        // Capacitor-bearing decks carry dynamic state in node voltages, so a
        // tiny node delta there is not enough evidence to reject an otherwise
        // bounded recovery step.
        if !circuit.capacitors.is_empty() {
            return false;
        }

        // Inductor branch-current unknowns can show physical state progression
        // when node voltages remain fixed. Algebraic source currents can change
        // without moving the circuit state, so ignore them here.
        if circuit.inductors.branch_indices.is_empty() {
            return true;
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
    pub(super) fn transient_newton_iteration_budget(&self, startup_recovery: bool) -> usize {
        let standard_budget = match self.config.spice_dialect {
            // Xyce's NOX transient parameter set owns an independent MAXSTEP
            // budget whose 7.10 default is 20.
            SpiceDialect::Xyce => self
                .config
                .transient_nonlinear_max_iterations
                .unwrap_or(20)
                .max(1),
            // ngspice's NIiter() raises any smaller iteration limit to 100
            // (niiter.c) for every Newton solve, transient timepoints included.
            SpiceDialect::BestAvailable | SpiceDialect::Ngspice => self
                .config
                .transient_max_iterations
                .max(1)
                .max(NGSPICE_NIITER_MIN_ITERATIONS),
        };
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
        deltas: SourceActivityDeltas,
        floors: StepBiasFloors,
    ) -> Value {
        let SourceActivityDeltas {
            expected_source_delta,
            interior_source_delta,
            source_ramp_tracking_delta,
        } = deltas;
        let StepBiasFloors {
            practical_min_dt,
            preferred_min_dt,
            recovery_cap_enabled,
            nonlinear_source_ramp_cap_enabled,
        } = floors;
        let mut dt = proposed_dt.min(remaining_time);
        let source_is_moving_before_endpoint =
            !at_breakpoint || interior_source_delta >= SOURCE_ACTIVE_DELTA;
        if nonlinear_source_ramp_cap_enabled
            && source_ramp_tracking_delta.is_finite()
            && source_ramp_tracking_delta > 0.0
            && expected_source_delta.is_finite()
            && expected_source_delta > source_ramp_tracking_delta
            && source_is_moving_before_endpoint
        {
            let ramp_cap = dt * (source_ramp_tracking_delta / expected_source_delta);
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

    /// Spend one unit of the force-accept recovery budget.
    ///
    /// See [`FORCE_ACCEPT_COOLDOWN_RETRIES`] for the rule: an accepted point
    /// and a Newton retry that the cooldown holds the step width for are the
    /// same kind of recovery event, so the caps disarm two events after the
    /// force-accept instead of outliving a run that never fails Newton again.
    #[inline]
    pub(super) fn force_accept_cooldown_after_recovery_event(
        force_accept_cooldown: usize,
    ) -> usize {
        force_accept_cooldown.saturating_sub(1)
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The cooldown is a budget of recovery events, not of Newton failures.
    /// `run_transient` spends a unit at every accepted point, so a timepoint
    /// force-accepted out of LTE exhaustion disarms the recovery caps two
    /// accepted points later even when Newton never fails again — which is the
    /// only thing that used to decrement it, so the caps outlived the run.
    #[test]
    fn the_force_accept_cooldown_is_spent_by_accepted_points_not_only_newton_failures() {
        let mut cooldown = FORCE_ACCEPT_COOLDOWN_RETRIES;
        assert!(Engine::should_apply_active_source_recovery_cap(cooldown));

        cooldown = Engine::force_accept_cooldown_after_recovery_event(cooldown);
        assert_eq!(cooldown, 1);
        assert!(
            Engine::should_apply_active_source_recovery_cap(cooldown),
            "the first accepted point after the force-accept is still inside the recovery window"
        );

        cooldown = Engine::force_accept_cooldown_after_recovery_event(cooldown);
        assert_eq!(cooldown, 0);
        assert!(
            !Engine::should_apply_active_source_recovery_cap(cooldown),
            "two accepted points with no Newton failure must disarm the recovery caps"
        );

        cooldown = Engine::force_accept_cooldown_after_recovery_event(cooldown);
        assert_eq!(cooldown, 0, "a spent budget saturates instead of wrapping");
        assert!(!Engine::should_apply_active_source_recovery_cap(cooldown));
    }

    /// What that budget actually gates on the proposal side: while it is armed
    /// a step over a moving source is cut to `preferred_min_dt / 8`, and once
    /// it is spent the controller's own proposal stands.
    #[test]
    fn the_active_source_recovery_cap_follows_the_cooldown_budget() {
        let proposal_for = |cooldown: usize| {
            Engine::bias_transient_step_for_source_activity(
                1.0e-3,
                1.0,
                false,
                SourceActivityDeltas {
                    expected_source_delta: 1.0,
                    interior_source_delta: 1.0,
                    source_ramp_tracking_delta: Value::INFINITY,
                },
                StepBiasFloors {
                    practical_min_dt: 1.0e-9,
                    preferred_min_dt: 1.0e-4,
                    recovery_cap_enabled: Engine::should_apply_active_source_recovery_cap(cooldown),
                    nonlinear_source_ramp_cap_enabled: false,
                },
            )
        };

        let armed = proposal_for(FORCE_ACCEPT_COOLDOWN_RETRIES);
        assert!(
            (armed - 1.25e-5).abs() <= 1.0e-20,
            "armed proposal {armed:e} is not preferred_min_dt / 8"
        );
        let spent = proposal_for(0);
        assert!(
            (spent - 1.0e-3).abs() <= 1.0e-18,
            "spent proposal {spent:e} must be the controller's own step"
        );
    }
}
