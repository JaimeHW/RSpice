//! Newton damping, line search, and solution validation helpers.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::convergence) fn has_nonfinite_values(solution: &[Value]) -> bool {
        solution.iter().any(|v| !v.is_finite())
    }

    #[inline]
    pub(in crate::engine::convergence) fn step_l2_norm(
        old: &[Value],
        new: &[Value],
    ) -> ScaledStepNorm {
        let mut norm = ScaledStepNorm {
            scale: 0.0,
            squared_sum: 0.0,
        };
        for (&a, &b) in old.iter().zip(new) {
            let delta = b - a;
            // Opposite finite endpoints may have an unrepresentable delta.
            // Its half remains finite; a weight of four retains its square.
            let (magnitude, weight) = if delta.is_infinite() && a.is_finite() && b.is_finite() {
                ((0.5 * b - 0.5 * a).abs(), 4.0)
            } else {
                (delta.abs(), 1.0)
            };
            if magnitude.is_nan() {
                return ScaledStepNorm {
                    scale: Value::NAN,
                    squared_sum: Value::NAN,
                };
            }
            if magnitude > norm.scale {
                norm.squared_sum = weight + norm.squared_sum * (norm.scale / magnitude).powi(2);
                norm.scale = magnitude;
            } else if magnitude != 0.0 {
                norm.squared_sum += weight * (magnitude / norm.scale).powi(2);
            }
        }
        norm
    }

    /// Interpolate finite Newton endpoints with `alpha` in `[0, 1]`.
    #[inline]
    pub(in crate::engine) fn interpolate_newton_value(
        old_v: Value,
        new_v: Value,
        alpha: Value,
    ) -> Value {
        if alpha == 0.0 {
            return old_v;
        }
        if alpha == 1.0 {
            return new_v;
        }
        if (old_v < 0.0 && new_v > 0.0) || (old_v > 0.0 && new_v < 0.0) {
            // A convex sum avoids overflowing the endpoint difference.
            // Retain product, complement and sum roundoff so opposite
            // large terms can still leave a small physical trial value.
            let complement = 1.0 - alpha;
            let complement_error = (1.0 - complement) - alpha;
            let left = complement * old_v;
            let right = alpha * new_v;
            let sum = left + right;
            let right_part = sum - left;
            let sum_error = (left - (sum - right_part)) + (right - right_part);
            let correction = complement.mul_add(old_v, -left)
                + alpha.mul_add(new_v, -right)
                + complement_error * old_v
                + sum_error;
            sum + correction
        } else if alpha <= 0.5 {
            old_v + alpha * (new_v - old_v)
        } else {
            // Anchor near the proposed endpoint to avoid subtracting
            // nearly equal large values when a full step is approached.
            new_v + (1.0 - alpha) * (old_v - new_v)
        }
    }

    pub(in crate::engine) fn interpolate_solution(
        old: &[Value],
        proposal: &[Value],
        alpha: Value,
    ) -> Vec<Value> {
        if alpha == 0.0 {
            return old.to_vec();
        }
        if alpha == 1.0 {
            return proposal.to_vec();
        }
        old.iter()
            .zip(proposal.iter())
            .map(|(&old_v, &new_v)| Self::interpolate_newton_value(old_v, new_v, alpha))
            .collect()
    }

    pub(in crate::engine::convergence) fn limit_step_delta_with_state_mask(
        non_electrical_state_mask: &[bool],
        old: &[Value],
        proposal: &[Value],
        max_delta: Value,
    ) -> Vec<Value> {
        old.iter()
            .zip(proposal.iter())
            .enumerate()
            .map(|(index, (&old_v, &new_v))| {
                // The mask spans the nodal prefix only. Unknowns after that
                // prefix are MNA branch currents, not voltages; applying a
                // voltage-delta limit to them corrupts otherwise exact KCL
                // solutions for large current-driven networks.
                if index >= non_electrical_state_mask.len()
                    || non_electrical_state_mask
                        .get(index)
                        .copied()
                        .unwrap_or(false)
                {
                    return new_v;
                }
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
        step_norm: ScaledStepNorm,
    ) {
        let Some(prev_norm) = damping_state.prev_step_norm else {
            damping_state.prev_step_norm = Some(step_norm);
            damping_state.bank_rose_alpha = 1.0;
            return;
        };

        let ratio = if step_norm.scale == 0.0 {
            0.0
        } else if prev_norm.scale == 0.0 {
            Value::INFINITY
        } else {
            (step_norm.scale / prev_norm.scale)
                * (step_norm.squared_sum / prev_norm.squared_sum).sqrt()
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
        damping_state.prev_step_norm = Some(step_norm);
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
    /// Berkeley MOS1/2/3/6 (and the legacy BSIM front carried by `Mosfet`)
    /// use the same DEVfetlim/DEVlimvds/DEVpnjlim discipline as ngspice.
    /// B3SOI also has a local branch/body limiter, but it still benefits from
    /// the engine's voltage damping in ordinary operating-point solves. Those
    /// callers use a progress-forcing line-search policy instead of this raw
    /// full-step bypass.
    pub(in crate::engine) fn junction_limiting_owns_newton_steps(circuit: &CircuitData) -> bool {
        !circuit.diodes.is_empty()
            || !circuit.mosfets.is_empty()
            // BSIM3 and BSIM4 run the same b3ld.c/b4ld.c per-iterate
            // sequence (`DEVfetlim`/`DEVlimvds`/`DEVpnjlim`) against their
            // previous iterate, so the same argument applies: a merit-driven
            // shrink of a step the device has already limited stalls turn-on.
            // Left out, a 144-transistor NAND adder with default models never
            // reached an operating point under `Combined` damping (512
            // iterations) while the flat step converged in a hundred.
            || circuit.has_bsim3v3_devices()
            || circuit.has_bsim4v8_devices()
            || circuit
                .bjts
                .devices
                .iter()
                .any(|bjt| bjt.uses_legacy_junction_limiting() || bjt.uses_vbic_dynamic_charges())
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

    pub(in crate::engine::convergence) fn apply_damping_strategy_for_circuit<F>(
        &self,
        has_b3soi_devices: bool,
        non_electrical_state_mask: &[bool],
        step: DampingStep<'_>,
        junction_owns_steps: bool,
        mut merit: F,
    ) -> Vec<Value>
    where
        F: FnMut(&[Value]) -> Option<Value>,
    {
        let DampingStep {
            old,
            proposal,
            damping_state,
        } = step;
        if junction_owns_steps {
            return proposal.to_vec();
        }
        if has_b3soi_devices
            && matches!(
                self.config.convergence_config.damping_strategy,
                DampingStrategy::VoltageLimiting
            )
        {
            return Self::limit_step_delta_with_state_mask(
                non_electrical_state_mask,
                old,
                proposal,
                Self::MAX_DELTA_VOLTAGE_LIMIT,
            );
        }
        match self.config.convergence_config.damping_strategy {
            DampingStrategy::None => proposal.to_vec(),
            DampingStrategy::LineSearch => Self::line_search_step(old, proposal, &mut merit),
            DampingStrategy::VoltageLimiting => {
                let limited = Self::limit_step_delta_with_state_mask(
                    non_electrical_state_mask,
                    old,
                    proposal,
                    Self::MAX_DELTA_VOLTAGE_LIMIT,
                );
                Self::line_search_step(old, &limited, &mut merit)
            }
            DampingStrategy::BankRose => {
                let step_norm = Self::step_l2_norm(old, proposal);
                Self::update_bank_rose_alpha(damping_state, step_norm);
                Self::interpolate_solution(old, proposal, damping_state.bank_rose_alpha)
            }
            DampingStrategy::Combined => {
                let limited = Self::limit_step_delta_with_state_mask(
                    non_electrical_state_mask,
                    old,
                    proposal,
                    Self::MAX_DELTA_VOLTAGE_LIMIT,
                );
                let step_norm = Self::step_l2_norm(old, &limited);
                Self::update_bank_rose_alpha(damping_state, step_norm);
                let bank_rose_step =
                    Self::interpolate_solution(old, &limited, damping_state.bank_rose_alpha);
                Self::line_search_step(old, &bank_rose_step, &mut merit)
            }
        }
    }

    pub(in crate::engine::convergence) fn reset_nonfinite_values(solution: &mut [Value]) {
        for v in solution {
            if !v.is_finite() {
                *v = 0.0;
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
        let verdict = matrix.with_probe_values(|probe, rhs| {
            Self::stamp_nodal_gmin(circuit, probe, gmin_floor);
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
    use crate::device::Mosfet;

    #[test]
    fn step_norm_and_bank_rose_are_independent_of_absolute_scale() {
        for scale in [1e-300, 1e-200, 1.0, 1e200, 1e307] {
            let norm = Engine::step_l2_norm(&[0.0; 2], &[3.0 * scale, 4.0 * scale]);
            let magnitude = norm.scale * norm.squared_sum.sqrt();
            assert!(
                (magnitude / scale - 5.0).abs() < 2e-15,
                "scale={scale}: {magnitude}"
            );
            let mut state = NewtonDampingState::default();
            Engine::update_bank_rose_alpha(&mut state, norm);
            Engine::update_bank_rose_alpha(
                &mut state,
                Engine::step_l2_norm(&[0.0; 2], &[6.0 * scale, 8.0 * scale]),
            );
            assert_eq!(state.bank_rose_alpha, 0.5, "scale={scale}");
        }
    }

    #[test]
    fn bank_rose_compares_steps_whose_norms_exceed_the_float_range() {
        let old = [-1e308; 2];
        let mut state = NewtonDampingState::default();
        Engine::update_bank_rose_alpha(&mut state, Engine::step_l2_norm(&old, &[5e307; 2]));
        Engine::update_bank_rose_alpha(&mut state, Engine::step_l2_norm(&old, &[1e308; 2]));
        assert_eq!(state.bank_rose_alpha, 0.5);
    }

    #[test]
    fn newton_interpolation_preserves_finite_endpoints_and_small_cancellations() {
        let old = [f64::MAX, -1e308, 1e100, -1e16];
        let new = [-f64::MAX, 1e308, 1.0, 1e16 - 2.0];
        assert_eq!(Engine::interpolate_solution(&old, &new, 0.0), old);
        assert_eq!(Engine::interpolate_solution(&old, &new, 1.0), new);
        let midpoint = Engine::interpolate_solution(&old, &new, 0.5);
        assert_eq!(midpoint[0], 0.0);
        assert_eq!(midpoint[1], 0.0);
        assert_eq!(midpoint[3], -1.0);
        for alpha in [0.5_f64.next_down(), 0.5_f64.next_up()] {
            let actual = Engine::interpolate_solution(&[-1e308], &[1e308], alpha)[0];
            let expected = (2.0 * alpha - 1.0) * 1e308;
            assert!(
                (actual / expected - 1.0).abs() < 4.0 * f64::EPSILON,
                "alpha={alpha}: {actual} vs {expected}"
            );
        }
        let alpha = 1.0_f64.next_down();
        let near_end = Engine::interpolate_solution(&[1e308], &[0.0], alpha)[0];
        assert_eq!(near_end, (1.0 - alpha) * 1e308);
    }

    #[test]
    fn line_search_can_reach_a_finite_root_across_extreme_endpoints() {
        let mut merit = |values: &[Value]| {
            let x = values[0] / 1e308;
            x.is_finite().then(|| (x * (2.0 + x)).abs())
        };
        assert_eq!(
            Engine::line_search_step(&[-1e308], &[1e308], &mut merit),
            vec![0.0]
        );
    }

    #[test]
    fn fallback_acceptance_uses_equations_for_large_and_uniform_solutions() {
        for bias in [0.0_f64, 0.01, 2500.0] {
            let mut circuit = CircuitData::new();
            for index in 0..6 {
                let node = circuit.get_or_create_node(&format!("n{index}"));
                circuit.resistors.add(format!("r{index}"), node, 0, 1.0);
                circuit
                    .current_sources
                    .add(format!("i{index}"), 0, node, bias);
                circuit.diodes.add(crate::device::Diode::spice_defaults(
                    format!("d{index}"),
                    0,
                    node,
                ));
            }
            let engine = Engine::default();
            let mut matrix = engine.build_matrix(&circuit).unwrap();
            circuit.link_indices(&matrix);
            let candidate = vec![bias; circuit.matrix_size()];
            assert_eq!(
                Engine::sanitize_initial_guess(&candidate, candidate.len()),
                candidate
            );
            let accepted = engine
                .evaluate_fallback_candidate(
                    &mut circuit,
                    &mut matrix,
                    candidate,
                    "test continuation",
                    &crate::abort_signal::NoAbort,
                )
                .unwrap();
            assert!(
                accepted.is_some(),
                "valid uniform solution at {bias} must pass residual validation"
            );
            let continued = engine
                .gmin_stepping_nonlinear_with_abort(
                    &mut circuit,
                    &mut matrix,
                    accepted.as_ref().unwrap(),
                    &crate::abort_signal::NoAbort,
                )
                .unwrap();
            assert!(engine.validate_nonlinear_solution(&mut circuit, &mut matrix, &continued));
            let invalid = vec![f64::NAN; circuit.matrix_size()];
            assert!(
                engine
                    .evaluate_fallback_candidate(
                        &mut circuit,
                        &mut matrix,
                        invalid,
                        "test continuation",
                        &crate::abort_signal::NoAbort,
                    )
                    .unwrap()
                    .is_none()
            );
        }
    }

    #[test]
    fn voltage_step_limit_does_not_clamp_mna_branch_currents() {
        let old = [0.0, 0.0, 0.0];
        let proposal = [4.0, -4.0, 3.06486];
        let non_electrical_state_mask = [false, false];

        let limited = Engine::limit_step_delta_with_state_mask(
            &non_electrical_state_mask,
            &old,
            &proposal,
            2.0,
        );

        assert_eq!(limited, vec![2.0, -2.0, 3.06486]);
    }

    #[test]
    fn classic_mos_local_limiting_owns_newton_steps() {
        let mut circuit = CircuitData::new();
        let drain = circuit.get_or_create_node("d");
        let gate = circuit.get_or_create_node("g");
        let source = circuit.get_or_create_node("s");
        let bulk = circuit.get_or_create_node("b");
        circuit.mosfets.add(Mosfet::new_pmos(
            "mp".to_string(),
            drain,
            gate,
            source,
            bulk,
        ));

        assert!(Engine::junction_limiting_owns_newton_steps(&circuit));
    }

    #[test]
    fn diode_pnjlim_owns_newton_steps() {
        let mut circuit = CircuitData::new();
        let anode = circuit.get_or_create_node("a");
        circuit.diodes.add(crate::device::Diode::spice_defaults(
            "d1".to_string(),
            anode,
            0,
        ));

        assert!(Engine::junction_limiting_owns_newton_steps(&circuit));
    }

    #[test]
    fn legacy_bjt_owns_newton_steps_only_while_voltage_limiting_is_enabled() {
        let mut circuit = CircuitData::new();
        let collector = circuit.get_or_create_node("c");
        let base = circuit.get_or_create_node("b");
        let mut bjt = crate::device::Bjt::new_npn("q1".to_string(), collector, base, 0);
        circuit.bjts.add(bjt.clone());
        assert!(Engine::junction_limiting_owns_newton_steps(&circuit));

        bjt.set_voltage_limiting_enabled(false);
        circuit.bjts.devices.clear();
        circuit.bjts.add(bjt);
        assert!(!Engine::junction_limiting_owns_newton_steps(&circuit));
    }
}
