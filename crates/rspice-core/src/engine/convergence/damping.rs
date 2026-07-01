//! Newton damping, line search, physical clamping, and solution validation helpers.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::convergence) fn has_clamped_values(
        solution: &[Value],
        node_count: usize,
    ) -> bool {
        solution.iter().any(|&v| !v.is_finite())
            || solution
                .iter()
                .take(node_count.min(solution.len()))
                .any(|&v| v.abs() >= 999.0)
    }

    #[inline]
    pub(in crate::engine::convergence) fn has_suspicious_uniformity(solution: &[Value]) -> bool {
        if solution.len() <= 4 {
            return false;
        }

        let mut counts: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        let mut min_v = Value::INFINITY;
        let mut max_v = Value::NEG_INFINITY;

        for &v in solution {
            if !v.is_finite() {
                return false;
            }
            min_v = min_v.min(v);
            max_v = max_v.max(v);
            let bucket = (v * 1000.0).round() as i32; // 1 mV quantization
            *counts.entry(bucket).or_insert(0) += 1;
        }

        let span = max_v - min_v;
        if span <= 1e-9 {
            return true;
        }

        let dominant = counts.values().copied().max().unwrap_or(0);
        let dominant_ratio = dominant as Value / solution.len() as Value;

        // Only flag near-constant stuck vectors; avoid false positives on
        // legitimate rail-distributed logic operating points.
        dominant_ratio >= 0.8 && counts.len() <= 3 && span <= 5e-2
    }

    #[inline]
    pub(in crate::engine::convergence) fn is_suspicious_solution(
        solution: &[Value],
        node_count: usize,
    ) -> bool {
        let node_limit = node_count.min(solution.len());
        Self::has_clamped_values(solution, node_limit)
            || Self::has_suspicious_uniformity(&solution[..node_limit])
    }

    #[inline]
    pub(in crate::engine::convergence) fn step_l2_norm(old: &[Value], new: &[Value]) -> Value {
        old.iter()
            .zip(new.iter())
            .map(|(&a, &b)| {
                let d = b - a;
                d * d
            })
            .sum::<Value>()
            .sqrt()
    }

    pub(in crate::engine::convergence) fn interpolate_solution(
        old: &[Value],
        proposal: &[Value],
        alpha: Value,
    ) -> Vec<Value> {
        old.iter()
            .zip(proposal.iter())
            .map(|(&old_v, &new_v)| old_v + alpha * (new_v - old_v))
            .collect()
    }

    pub(in crate::engine::convergence) fn limit_step_delta(
        old: &[Value],
        proposal: &[Value],
        max_delta: Value,
    ) -> Vec<Value> {
        old.iter()
            .zip(proposal.iter())
            .map(|(&old_v, &new_v)| {
                let delta = new_v - old_v;
                if delta.abs() > max_delta {
                    old_v + delta.signum() * max_delta
                } else {
                    new_v
                }
            })
            .collect()
    }

    pub(in crate::engine::convergence) fn update_bank_rose_alpha(
        damping_state: &mut NewtonDampingState,
        step_norm: Value,
    ) {
        let Some(prev_norm) = damping_state.prev_step_norm else {
            damping_state.prev_step_norm = Some(step_norm.max(1e-30));
            damping_state.bank_rose_alpha = 1.0;
            return;
        };

        let ratio = if prev_norm > 0.0 {
            step_norm / prev_norm
        } else {
            1.0
        };

        if ratio > 1.0 {
            damping_state.bank_rose_alpha *= 0.5;
        } else if ratio > 0.9 {
            damping_state.bank_rose_alpha *= 0.9;
        } else if ratio < 0.5 {
            damping_state.bank_rose_alpha *= 1.2;
        }

        damping_state.bank_rose_alpha = damping_state
            .bank_rose_alpha
            .clamp(Self::BANK_ROSE_ALPHA_MIN, Self::BANK_ROSE_ALPHA_MAX);
        damping_state.prev_step_norm = Some(step_norm.max(1e-30));
    }

    pub(in crate::engine::convergence) fn line_search_step<F>(
        old: &[Value],
        proposal: &[Value],
        merit: &mut F,
    ) -> Vec<Value>
    where
        F: FnMut(&[Value]) -> Option<Value>,
    {
        let base_merit = merit(old).unwrap_or(Value::INFINITY);
        let proposal_merit = merit(proposal).unwrap_or(Value::INFINITY);

        if proposal_merit.is_finite() && (!base_merit.is_finite() || proposal_merit <= base_merit) {
            return proposal.to_vec();
        }

        let mut best_solution = if base_merit.is_finite() {
            old.to_vec()
        } else {
            proposal.to_vec()
        };
        let mut best_merit = if base_merit.is_finite() {
            base_merit
        } else {
            proposal_merit
        };

        let mut alpha = Self::LINE_SEARCH_BACKTRACK;
        for _ in 0..Self::LINE_SEARCH_MAX_ITERS {
            let trial = Self::interpolate_solution(old, proposal, alpha);
            if let Some(trial_merit) = merit(&trial) {
                if trial_merit < best_merit {
                    best_merit = trial_merit;
                    best_solution = trial.clone();
                }

                let armijo_ok = !base_merit.is_finite()
                    || trial_merit <= base_merit * (1.0 - Self::ARMIJO_C1 * alpha);
                if armijo_ok {
                    return trial;
                }
            }
            alpha *= Self::LINE_SEARCH_BACKTRACK;
        }

        best_solution
    }

    /// Whether per-device iterate-replacement junction limiting governs the
    /// Newton step for this circuit. These compact models run SPICE/Xyce-style
    /// voltage limiting inside their `update` path: the full node step is the
    /// algorithm and the per-iterate branch replacement is the globalization.
    /// B3SOI also has a local branch/body limiter, but it still benefits from
    /// the engine's voltage damping in ordinary operating-point solves. Those
    /// callers use a progress-forcing line-search policy instead of this raw
    /// full-step bypass.
    pub(in crate::engine) fn junction_limiting_owns_newton_steps(circuit: &CircuitData) -> bool {
        circuit
            .bjts
            .devices
            .iter()
            .any(|bjt| bjt.uses_legacy_gummel_poon() || bjt.uses_vbic_dynamic_charges())
            || circuit
                .vdmoses
                .devices
                .iter()
                .any(|vdmos| vdmos.xyce_level18)
    }

    pub(in crate::engine) fn b3soi_limiter_owns_global_damping(
        &self,
        circuit: &CircuitData,
    ) -> bool {
        circuit.has_b3soi_devices()
            && matches!(
                self.config.convergence_config.damping_strategy,
                DampingStrategy::LineSearch | DampingStrategy::Combined
            )
    }

    pub(in crate::engine::convergence) fn apply_damping_strategy_with_junction_ownership<F>(
        &self,
        old: &[Value],
        proposal: &[Value],
        damping_state: &mut NewtonDampingState,
        junction_owns_steps: bool,
        merit: F,
    ) -> Vec<Value>
    where
        F: FnMut(&[Value]) -> Option<Value>,
    {
        if junction_owns_steps {
            return proposal.to_vec();
        }
        self.apply_damping_strategy(old, proposal, damping_state, merit)
    }

    pub(in crate::engine::convergence) fn apply_damping_strategy<F>(
        &self,
        old: &[Value],
        proposal: &[Value],
        damping_state: &mut NewtonDampingState,
        mut merit: F,
    ) -> Vec<Value>
    where
        F: FnMut(&[Value]) -> Option<Value>,
    {
        match self.config.convergence_config.damping_strategy {
            DampingStrategy::None => proposal.to_vec(),
            DampingStrategy::LineSearch => Self::line_search_step(old, proposal, &mut merit),
            DampingStrategy::VoltageLimiting => {
                let limited = Self::limit_step_delta(old, proposal, Self::MAX_DELTA_VOLTAGE_LIMIT);
                Self::line_search_step(old, &limited, &mut merit)
            }
            DampingStrategy::BankRose => {
                let step_norm = Self::step_l2_norm(old, proposal);
                Self::update_bank_rose_alpha(damping_state, step_norm);
                Self::interpolate_solution(old, proposal, damping_state.bank_rose_alpha)
            }
            DampingStrategy::Combined => {
                let limited = Self::limit_step_delta(old, proposal, Self::MAX_DELTA_VOLTAGE_LIMIT);
                let step_norm = Self::step_l2_norm(old, &limited);
                Self::update_bank_rose_alpha(damping_state, step_norm);
                let bank_rose_step =
                    Self::interpolate_solution(old, &limited, damping_state.bank_rose_alpha);
                Self::line_search_step(old, &bank_rose_step, &mut merit)
            }
        }
    }

    pub(in crate::engine::convergence) fn clamp_solution_to_physical_bounds(
        solution: &mut [Value],
        node_count: usize,
    ) {
        for v in solution.iter_mut() {
            if !v.is_finite() {
                *v = 0.0;
            }
        }

        let node_limit = node_count.min(solution.len());
        for v in solution.iter_mut().take(node_limit) {
            if v.abs() > Self::MAX_NODE_VOLTAGE {
                *v = v.signum() * Self::MAX_NODE_VOLTAGE;
            }
        }
    }

    #[inline]
    /// Apply ngspice-style per-iteration junction limiting to legacy
    /// Gummel-Poon BJT terminals.
    ///
    /// This is the engine-level analog of ngspice's `pnjlim` discipline: it
    /// runs on every transient Newton iterate, unconditionally, so that full
    /// Newton node updates stay safe around exponential junctions without a
    /// global trust region distorting switching-edge trajectories.
    ///
    /// Applies to every BJT charge model: the dispatching scale function uses
    /// the VBIC or legacy Gummel-Poon junction set as appropriate. Without
    /// the per-iterate discipline a Newton iterate can move a VBIC terminal
    /// by over a volt in one correction, driving the nested continuation
    /// solver at an unreachable bias for seconds per rescue (observed on
    /// vbic/diffamp at t=20ps: max_dv=1.04V with 6.9s failed continuations).
    /// The hidden excess-phase convergence check compares iterate-to-iterate,
    /// so limited steps remain consistent with the device-local state.
    pub(crate) fn limit_bjt_junction_external_updates(
        circuit: &CircuitData,
        proposal: &mut [Value],
        previous: &[Value],
        num_nodes: usize,
        protected_nodes: Option<&[bool]>,
    ) -> bool {
        let mut changed = false;
        for _ in 0..3 {
            let mut pass_changed = false;
            for bjt in &circuit.bjts.devices {
                let node_voltage = |values: &[Value], node: usize| {
                    if node == 0 {
                        0.0
                    } else {
                        values.get(node - 1).copied().unwrap_or(0.0)
                    }
                };
                let previous_external = [
                    node_voltage(previous, bjt.node_collector),
                    node_voltage(previous, bjt.node_base),
                    node_voltage(previous, bjt.node_emitter),
                    node_voltage(previous, bjt.node_substrate),
                ];
                let proposed_external = [
                    node_voltage(proposal, bjt.node_collector),
                    node_voltage(proposal, bjt.node_base),
                    node_voltage(proposal, bjt.node_emitter),
                    node_voltage(proposal, bjt.node_substrate),
                ];
                let Some(scale) = bjt
                    .junction_external_step_limit_scale_against_previous(
                        previous_external,
                        proposed_external,
                    )
                    .filter(|scale| scale.is_finite() && *scale + 1e-6 < 1.0)
                else {
                    continue;
                };

                for node in [
                    bjt.node_collector,
                    bjt.node_base,
                    bjt.node_emitter,
                    bjt.node_substrate,
                ] {
                    if node == 0 {
                        continue;
                    }
                    let proposal_idx = node - 1;
                    if proposal_idx >= num_nodes
                        || protected_nodes
                            .and_then(|protected| protected.get(proposal_idx))
                            .copied()
                            .unwrap_or(false)
                    {
                        continue;
                    }
                    let previous_value = previous[proposal_idx];
                    let proposal_value = proposal[proposal_idx];
                    let delta = proposal_value - previous_value;
                    if !delta.is_finite() || delta.abs() <= 0.0 {
                        continue;
                    }
                    proposal[proposal_idx] = previous_value + scale * delta;
                    pass_changed = true;
                }
            }

            if !pass_changed {
                break;
            }
            changed = true;
        }

        changed
    }

    pub(in crate::engine::convergence) fn validate_nonlinear_solution(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> bool {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return false;
        }

        // Re-evaluate one Newton linearization at the candidate point without
        // allowing limiter history from the trial evaluation to escape into the
        // caller's ongoing nonlinear iteration. Fallback candidates are static
        // operating-point solutions; their acceptance should be governed by the
        // residual and fixed-point update, not by the previous device history
        // used to protect Newton steps while finding them.
        let snapshot = circuit.nonlinear_state_snapshot();
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let node_count = circuit.num_nodes().min(size);
        let verdict = matrix.with_probe_values(|probe, rhs| {
            for i in 0..node_count {
                probe.add(i, i, gmin_floor);
            }
            circuit.stamp_dc_direct(probe, rhs);
            if self
                .try_stamp_static_probe_nonlinear_devices_for_dc(circuit, probe, rhs, solution)
                .is_err()
            {
                return false;
            }
            let residual_norm = self
                .residual_inf_norm(circuit, probe, solution, rhs)
                .unwrap_or(Value::INFINITY);
            let residual_converged = residual_norm.is_finite() && residual_norm <= 1.0;

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
            let residual_only_acceptable =
                residual_converged && circuit.has_jfet_gate_generation_branches();

            residual_converged && (fixed_point_converged || residual_only_acceptable)
        });
        circuit.restore_nonlinear_state(snapshot);
        verdict
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_clamp_only_limits_node_voltage_unknowns() {
        let mut solution = vec![1500.0, -1500.0, 2500.0, -2500.0];

        Engine::clamp_solution_to_physical_bounds(&mut solution, 2);

        assert_eq!(solution[0], Engine::MAX_NODE_VOLTAGE);
        assert_eq!(solution[1], -Engine::MAX_NODE_VOLTAGE);
        assert_eq!(solution[2], 2500.0);
        assert_eq!(solution[3], -2500.0);
    }

    #[test]
    fn physical_clamp_replaces_non_finite_unknowns_everywhere() {
        let mut solution = vec![0.0, Value::NAN, Value::INFINITY];

        Engine::clamp_solution_to_physical_bounds(&mut solution, 1);

        assert_eq!(solution, vec![0.0, 0.0, 0.0]);
    }
}
