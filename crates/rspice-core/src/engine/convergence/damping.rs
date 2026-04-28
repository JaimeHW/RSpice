//! Newton damping, line search, physical clamping, and solution validation helpers.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::convergence) fn has_clamped_values(solution: &[Value]) -> bool {
        solution.iter().any(|&v| !v.is_finite() || v.abs() >= 999.0)
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
    pub(in crate::engine::convergence) fn is_suspicious_solution(solution: &[Value]) -> bool {
        Self::has_clamped_values(solution) || Self::has_suspicious_uniformity(solution)
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
        let mut best_solution = proposal.to_vec();
        let mut best_merit = merit(proposal).unwrap_or(Value::INFINITY);

        if best_merit.is_finite() && (!base_merit.is_finite() || best_merit <= base_merit) {
            return best_solution;
        }

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
                Self::limit_step_delta(old, proposal, Self::MAX_DELTA_VOLTAGE_LIMIT)
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
    ) {
        for v in solution.iter_mut() {
            if !v.is_finite() {
                *v = 0.0;
            } else if v.abs() > Self::MAX_NODE_VOLTAGE {
                *v = v.signum() * Self::MAX_NODE_VOLTAGE;
            }
        }
    }

    #[inline]
    pub(crate) fn limit_vbic_external_updates(
        circuit: &CircuitData,
        proposal: &mut [Value],
        previous: &[Value],
        num_nodes: usize,
        protected_nodes: Option<&[bool]>,
        excess_phase_only: bool,
    ) -> bool {
        let mut changed = false;
        for _ in 0..3 {
            let mut pass_changed = false;
            for bjt in &circuit.bjts.devices {
                if !bjt.uses_vbic_dynamic_charges() {
                    continue;
                }
                if excess_phase_only && bjt.td <= 0.0 {
                    continue;
                }

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
                    .vbic_external_step_limit_scale_against_previous(
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

        // Re-evaluate one Newton linearization at the candidate point and require
        // a small fixed-point update plus device-level convergence.
        let mut rhs = vec![0.0; size];
        matrix.clear_values();
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        let node_count = circuit.num_nodes().min(size);
        for i in 0..node_count {
            matrix.add(i, i, gmin_floor);
        }
        circuit.stamp_dc_direct(matrix, &mut rhs);
        self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, solution);
        let residual_converged = self.residual_convergence_met(matrix, solution, &rhs);

        let Ok(next_solution) = matrix.solve(&rhs) else {
            return false;
        };

        residual_converged && self.voltage_convergence_met(solution, &next_solution) && {
            self.update_device_states_for_dc(circuit, solution);
            circuit.nonlinear_converged(self.device_convergence_criteria())
        }
    }
}
