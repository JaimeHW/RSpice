//! Newton damping, line search, and solution validation helpers.

use super::*;

/// The box every DC node voltage of a no-gain circuit lies in.
///
/// A network of positive resistors, inductors, capacitors, switches,
/// independent voltage sources and semiconductor devices whose terminal
/// currents flow from higher to lower potential has the no-gain property
/// (A. N. Willson, "The no-gain property for networks containing
/// three-terminal elements", IEEE Trans. Circuits Syst. CAS-22, 1975): no
/// DC node voltage leaves the range of the potentials its sources fix.
/// Projecting a Newton iterate onto a box that holds the solution moves no
/// component further from it.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::engine::convergence) struct NoGainVoltageBox {
    low: Value,
    high: Value,
    /// Rows of the nodal prefix that hold a state other than a potential.
    unbounded: Vec<bool>,
}

impl NoGainVoltageBox {
    /// The box for `circuit`, or `None` when an element can supply gain or a
    /// row holds a state the no-gain argument does not cover (temperature,
    /// a memristor's state, a code model's private unknown).
    pub(in crate::engine::convergence) fn of(circuit: &CircuitData) -> Option<Self> {
        if !circuit.vcvs.is_empty()
            || !circuit.vccs.is_empty()
            || !circuit.cccs.is_empty()
            || !circuit.ccvs.is_empty()
            || !circuit.behavioral_sources.is_empty()
            // A current source can drive a node to any potential.
            || circuit.current_sources.dc_values.iter().any(|&i| i != 0.0)
            || circuit.resistors.conductances.iter().any(|&g| g < 0.0)
            || !circuit.multi_winding_transformers.is_empty()
            || !circuit.jiles_atherton_inductors.is_empty()
            || !circuit.xyce_core_groups.is_empty()
            || !circuit.xyce_memristors.is_empty()
            || circuit.has_xspice_devices()
            || circuit.has_any_veriloga_devices()
            // Self-heating and SOI body/temperature states are not potentials.
            || circuit.has_b3soi_devices()
            || !circuit.vdmoses.is_empty()
            || !circuit.ekv26s.is_empty()
            || !circuit.ekv3s.is_empty()
            || circuit.bjts.devices.iter().any(|bjt| bjt.node_rth != 0)
            // An inductor IC the operating point enforces is a current source.
            || circuit.inductors.has_dc_initial_conditions()
            // BSIM gate tunnelling splits the gate-to-channel current between
            // source and drain by bias, and impact ionisation drives drain
            // current into the body by vds alone: either can flow towards
            // the higher potential, and with it a node can leave the rails
            // (the gate-tunnelling logic of `bsim4_native` settles at -1.57 V
            // under a 1 V supply).
            || circuit.bsim4v8.devices.iter().any(|device| {
                let core = &device.core;
                core.model.igc_mod != 0
                    || core.model.igb_mod != 0
                    || core.size.alpha0 != 0.0
                    || core.size.alpha1 != 0.0
            })
            || circuit
                .bsim3v3
                .devices
                .iter()
                .any(|device| device.core.size.alpha0 != 0.0 || device.core.size.alpha1 != 0.0)
        {
            return None;
        }
        #[cfg(feature = "veriloga")]
        if circuit.has_mixed_signal_hosts() {
            return None;
        }
        let sources = &circuit.voltage_sources;
        let (mut low, mut high, mut stacked) = (0.0_f64, 0.0_f64, 0.0_f64);
        for ((&value, &pos), &neg) in sources
            .dc_values
            .iter()
            .zip(&sources.node_pos)
            .zip(&sources.node_neg)
        {
            if !value.is_finite() {
                return None;
            }
            match (pos, neg) {
                (_, 0) => (low, high) = (low.min(value), high.max(value)),
                (0, _) => (low, high) = (low.min(-value), high.max(-value)),
                // A floating source fixes no potential, only a difference,
                // and can stack on any other source in either direction.
                _ => stacked += value.abs(),
            }
        }
        // A capacitor IC the operating point enforces is a voltage source
        // across the capacitor.
        let capacitors = &circuit.capacitors;
        for (branch, ic) in capacitors.ic_branch_indices.iter().zip(&capacitors.ic) {
            if branch.is_some() {
                let value = ic.unwrap_or(0.0);
                if !value.is_finite() {
                    return None;
                }
                stacked += value.abs();
            }
        }
        let mut unbounded = circuit.non_electrical_state_mask();
        for bjt in &circuit.bjts.devices {
            for node in [bjt.node_xf1, bjt.node_xf2] {
                if node > 0
                    && let Some(row) = unbounded.get_mut(node - 1)
                {
                    *row = true;
                }
            }
        }
        Some(Self {
            low: low - stacked,
            high: high + stacked,
            unbounded,
        })
    }

    /// Clamp each node potential of `solution` into the box.
    pub(in crate::engine::convergence) fn project(&self, solution: &mut [Value]) {
        for (value, &unbounded) in solution.iter_mut().zip(&self.unbounded) {
            if !unbounded {
                *value = value.clamp(self.low, self.high);
            }
        }
    }
}

/// The bound on how far one DC Newton iterate moves a self-heating BJT's
/// thermal node.
///
/// Temperature enters a junction's currents through Arrhenius factors whose
/// linearization holds over a few kelvin (a silicon saturation current
/// changes by e every kT²/qEg, about 7 K, at 300 K), while the power that
/// drives the thermal row multiplies currents by voltages that junction
/// limiting extrapolates over volts. The exact Newton step can then settle
/// the electrical residual with a temperature its linearization cannot
/// reach. Where a junction is biased beyond its activation energy the
/// collector current falls as the device heats, and the step heats it
/// instead of saturating it: vbic99_tran's second iterate asked for 271 K
/// and sent the collector to +56 V, where the isothermal step sends it to
/// -456 V towards saturation.
///
/// A thermal row that steps further than [`Self::STEP`] is held at the
/// bound and the iterate re-solved with it held. That is the exact Newton
/// step of the system with that temperature fixed: the heat flow that holds
/// the node there is found from the factored Jacobian, so the Jacobian is
/// neither changed nor relaxed. An iterate the bound held is not a converged
/// one: its linearized residual carries that heat flow.
pub(in crate::engine::convergence) struct ThermalStepBound {
    rows: Vec<usize>,
}

impl ThermalStepBound {
    /// Kelvin one iterate may move a thermal node.
    ///
    /// The scale is the Arrhenius one above. Xyce's VBIC 1.3 holds 5 K
    /// (limRTH, N_DEV_ADMSvbic13_4t.C:3837); vbic99_tran converges directly
    /// in 10-11 iterations for any bound from 2 to 50 K and fails at
    /// ngspice's 100 K logarithmic limit (vbicload.c:670).
    pub(in crate::engine::convergence) const STEP: Value = 10.0;

    pub(in crate::engine::convergence) fn of(circuit: &CircuitData) -> Option<Self> {
        let mut rows: Vec<usize> = circuit
            .bjts
            .devices
            .iter()
            .filter_map(|bjt| bjt.node_rth.checked_sub(1))
            .collect();
        rows.sort_unstable();
        rows.dedup();
        (!rows.is_empty()).then_some(Self { rows })
    }

    /// Hold every thermal row of `proposal` that steps further than
    /// [`Self::STEP`] from `previous` at the bound, re-solving the rest of
    /// the iterate through the Jacobian factored in `matrix`. Rows held at
    /// one pass stay held while the re-solve pushes others past the bound.
    /// A system that cannot be re-solved keeps the unbounded step.
    pub(in crate::engine::convergence) fn impose(
        &self,
        matrix: &mut StaticMatrix,
        denominator_floors: Option<&[Value]>,
        previous: &[Value],
        proposal: &mut [Value],
    ) {
        let unbounded = proposal.to_vec();
        // Each held row with its target and the response J⁻¹e of the iterate
        // to a unit heat flow injected there.
        let mut held: Vec<(usize, Value, Vec<Value>)> = Vec::new();
        loop {
            let held_before = held.len();
            for &row in &self.rows {
                let step = proposal[row] - previous[row];
                if step.abs() <= Self::STEP || held.iter().any(|(held_row, ..)| *held_row == row) {
                    continue;
                }
                let mut unit = vec![0.0; proposal.len()];
                unit[row] = 1.0;
                let mut response = Vec::with_capacity(proposal.len());
                let solved = Engine::solve_dc_linearization_system(
                    matrix,
                    &unit,
                    denominator_floors,
                    &mut response,
                    super::solve::DcSolve::NewtonIteration,
                );
                if solved.is_err() || !response.iter().all(|value| value.is_finite()) {
                    proposal.copy_from_slice(&unbounded);
                    return;
                }
                held.push((row, previous[row] + Self::STEP.copysign(step), response));
            }
            if held.len() == held_before {
                return;
            }
            // The heat flows that put every held row on its target together.
            let mut system: Vec<Vec<Value>> = held
                .iter()
                .map(|(row, target, _)| {
                    let mut equation: Vec<Value> =
                        held.iter().map(|(_, _, response)| response[*row]).collect();
                    equation.push(target - unbounded[*row]);
                    equation
                })
                .collect();
            let Some(flows) = Self::solve_small_dense(&mut system) else {
                proposal.copy_from_slice(&unbounded);
                return;
            };
            for (index, value) in proposal.iter_mut().enumerate() {
                *value = unbounded[index]
                    + held
                        .iter()
                        .zip(&flows)
                        .map(|((_, _, response), flow)| flow * response[index])
                        .sum::<Value>();
            }
        }
    }

    /// Gaussian elimination with partial pivoting on an augmented system.
    fn solve_small_dense(system: &mut [Vec<Value>]) -> Option<Vec<Value>> {
        let size = system.len();
        for column in 0..size {
            let pivot = (column..size)
                .max_by(|&a, &b| system[a][column].abs().total_cmp(&system[b][column].abs()))?;
            if !(system[pivot][column].abs() > 0.0) {
                return None;
            }
            system.swap(column, pivot);
            for row in column + 1..size {
                let factor = system[row][column] / system[column][column];
                for entry in column..=size {
                    system[row][entry] -= factor * system[column][entry];
                }
            }
        }
        let mut solution = vec![0.0; size];
        for row in (0..size).rev() {
            let known: Value = (row + 1..size)
                .map(|column| system[row][column] * solution[column])
                .sum();
            solution[row] = (system[row][size] - known) / system[row][row];
        }
        solution
            .iter()
            .all(|value| value.is_finite())
            .then_some(solution)
    }
}

impl Engine {
    #[inline]
    pub(in crate::engine::convergence) fn has_nonfinite_values(solution: &[Value]) -> bool {
        solution.iter().any(|v| !v.is_finite())
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
        step_norm: ScaledL2Norm,
    ) {
        let Some(prev_norm) = damping_state.prev_step_norm else {
            damping_state.prev_step_norm = Some(step_norm);
            damping_state.bank_rose_alpha = 1.0;
            return;
        };

        let ratio = step_norm.ratio(prev_norm);

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
            // VDMOS limits its channel and body diode in both dialects
            // (vdmosload.c), not only as Xyce LEVEL=18.
            || !circuit.vdmoses.devices.is_empty()
            || Self::generated_limiting_owns_newton_steps(circuit)
    }

    /// A generated Verilog-A model with an active `$limit` (VBIC's ngspice
    /// branch applies pnjlim to every junction) limits its iterates against
    /// the previous one exactly as a native SPICE device does, so the same
    /// full-step policy applies.
    fn generated_limiting_owns_newton_steps(circuit: &CircuitData) -> bool {
        #[cfg(feature = "veriloga-builtins-base")]
        {
            circuit
                .generated_veriloga_devices()
                .iter()
                .any(|device| device.limits_newton_steps())
        }
        #[cfg(not(feature = "veriloga-builtins-base"))]
        {
            let _ = circuit;
            false
        }
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
                let step_norm = ScaledL2Norm::between(old, proposal);
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
                let step_norm = ScaledL2Norm::between(old, &limited);
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
            let norm = ScaledL2Norm::between(&[0.0; 2], &[3.0 * scale, 4.0 * scale]);
            let ratio = norm.ratio(ScaledL2Norm::between(&[0.0; 2], &[scale, 0.0]));
            assert!((ratio - 5.0).abs() < 2e-15, "scale={scale}: {ratio}");
            let mut state = NewtonDampingState::default();
            Engine::update_bank_rose_alpha(&mut state, norm);
            Engine::update_bank_rose_alpha(
                &mut state,
                ScaledL2Norm::between(&[0.0; 2], &[6.0 * scale, 8.0 * scale]),
            );
            assert_eq!(state.bank_rose_alpha, 0.5, "scale={scale}");
        }
    }

    #[test]
    fn bank_rose_compares_steps_whose_norms_exceed_the_float_range() {
        let old = [-1e308; 2];
        let mut state = NewtonDampingState::default();
        Engine::update_bank_rose_alpha(&mut state, ScaledL2Norm::between(&old, &[5e307; 2]));
        Engine::update_bank_rose_alpha(&mut state, ScaledL2Norm::between(&old, &[1e308; 2]));
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
        circuit.bjts.add(Box::new(bjt.clone()));
        assert!(Engine::junction_limiting_owns_newton_steps(&circuit));

        bjt.set_voltage_limiting_enabled(false);
        circuit.bjts.devices.clear();
        circuit.bjts.add(Box::new(bjt));
        assert!(!Engine::junction_limiting_owns_newton_steps(&circuit));
    }

    fn built(deck: &str) -> CircuitData {
        let netlist = crate::netlist::Netlist::parse(deck).unwrap();
        Engine::default()
            .resolved_for_netlist(&netlist)
            .build_circuit(&netlist)
            .unwrap()
    }

    #[test]
    fn no_gain_box_spans_the_grounded_sources_and_every_floating_one() {
        let circuit = built(concat!(
            "box\n",
            "V1 a 0 5\n",
            "V2 0 n 2\n",
            "V3 b a 3\n",
            "R1 b n 1k\n",
            "M1 b a n n nm\n",
            ".model nm nmos\n",
            ".op\n",
            ".end\n",
        ));
        let voltage_box = NoGainVoltageBox::of(&circuit).unwrap();
        assert_eq!((voltage_box.low, voltage_box.high), (-5.0, 8.0));
        let mut solution = vec![1.0e138; circuit.matrix_size()];
        voltage_box.project(&mut solution);
        let nodes = circuit.num_nodes();
        assert!(solution[..nodes].iter().all(|&v| v == 8.0), "{solution:?}");
        assert!(
            solution[nodes..].iter().all(|&i| i == 1.0e138),
            "{solution:?}"
        );
    }

    #[test]
    fn an_element_with_gain_has_no_voltage_box() {
        for element in [
            "E1 c 0 a 0 100",
            "G1 c 0 a 0 1m",
            "I1 0 c 1m",
            "B1 c 0 V=10*V(a)",
        ] {
            let circuit = built(&format!(
                "gain\nV1 a 0 1\nR1 a 0 1k\nR2 c 0 1k\n{element}\n.op\n.end\n"
            ));
            assert_eq!(NoGainVoltageBox::of(&circuit), None, "{element}");
        }
    }

    #[test]
    fn bsim_gate_tunnelling_or_impact_ionisation_has_no_voltage_box() {
        let deck = |card: &str| {
            format!(
                "bsim\nVd d 0 1\nVg g 0 1\nM1 d g 0 0 nm l=0.1u w=1u\n.model nm nmos {card}\n.op\n.end\n"
            )
        };
        for card in [
            "level=54 igcmod=1",
            "level=54 igbmod=1",
            "level=54 alpha0=1e-6",
            "level=49 alpha0=1e-6",
        ] {
            assert_eq!(NoGainVoltageBox::of(&built(&deck(card))), None, "{card}");
        }
        for card in ["level=54", "level=49"] {
            assert!(
                NoGainVoltageBox::of(&built(&deck(card))).is_some(),
                "{card}"
            );
        }
    }

    /// Twenty default-card inverters: the linear presolve leaves every
    /// output at 0 V, where each gate's pull-up sits at vgd = 0 on the
    /// saturation edge (gds = 0) and its pull-down is cut off. The first
    /// Newton solve then put the last output near 1e138 V.
    #[test]
    fn an_inverter_chain_reaches_its_rails_without_leaving_them() {
        let mut deck = String::from(concat!(
            "chain\n",
            ".subckt inv in out vdd\n",
            "M2 out in 0 0 n1 w=1.8u l=1.2u\n",
            "M1 out in vdd vdd p1 w=3.6u l=1.2u\n",
            ".ends\n",
            "VDD vdd 0 2\n",
            "VIN in 0 2\n",
        ));
        for stage in 1..=20 {
            let input = if stage == 1 {
                "in".to_string()
            } else {
                format!("o{}", stage - 1)
            };
            deck.push_str(&format!("X{stage} {input} o{stage} vdd inv\n"));
        }
        deck.push_str(".model n1 nmos (level=1)\n.model p1 pmos (level=1)\n.op\n.end\n");
        let netlist = crate::netlist::Netlist::parse(&deck).unwrap();
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config.gmin_stepping = false;
        config.convergence_config.source_stepping = false;
        config.convergence_config.pseudo_transient = false;
        config.convergence_config.arc_length = false;
        let engine = Engine::new_with_resolved_config(config);
        let result = engine.run_dc_op(&netlist).unwrap();
        let output = |stage: usize| {
            let name = format!("o{stage}");
            let index = result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(&name))
                .unwrap();
            result.node_voltages[index]
        };
        for stage in 1..=20 {
            let rail = if stage % 2 == 0 { 2.0 } else { 0.0 };
            assert!(
                (output(stage) - rail).abs() < 1e-6,
                "o{stage} = {}",
                output(stage)
            );
        }
        let iterations = engine.convergence_quality().total_iterations;
        assert!(iterations <= 8, "{iterations} Newton iterations");
    }

    /// vbic99_tran's self-heating VBIC oscillated through 100 direct
    /// iterates in every dialect, its exact Newton heating the device by
    /// hundreds of kelvin to settle the collector (ngspice-46, whose heat row
    /// negates its own derivatives, converges in 13). With the thermal step
    /// bounded it takes the isothermal path to the operating point.
    #[test]
    fn a_bounded_thermal_step_takes_vbic99_to_its_operating_point_directly() {
        let netlist = crate::netlist::Netlist::parse(include_str!(
            "../../../../../tests/paranoia/vbic/vbic99_tran.sp"
        ))
        .unwrap();
        for dialect in [
            crate::engine::SpiceDialect::Ngspice,
            crate::engine::SpiceDialect::BestAvailable,
            crate::engine::SpiceDialect::Xyce,
        ] {
            let mut config = crate::engine::SimulationConfig::default();
            config.spice_dialect = dialect;
            config.convergence_config.gmin_stepping = false;
            config.convergence_config.source_stepping = false;
            config.convergence_config.pseudo_transient = false;
            config.convergence_config.arc_length = false;
            let engine = Engine::new_with_resolved_config(config);
            let result = engine
                .run_dc_op(&netlist)
                .unwrap_or_else(|error| panic!("{dialect:?}: {error}"));
            let voltage = |name: &str| {
                let index = result
                    .node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .unwrap();
                result.node_voltages[index]
            };
            // ngspice-46: v(3) = 0.2220443, v(t) = 1.440860.
            assert!(
                (voltage("3") - 0.22204).abs() < 1e-4,
                "{dialect:?} v(3) = {}",
                voltage("3")
            );
            assert!(
                (voltage("t") - 1.4409).abs() < 1e-3,
                "{dialect:?} v(t) = {}",
                voltage("t")
            );
            let iterations = engine.convergence_quality().total_iterations;
            assert!(
                iterations <= 20,
                "{dialect:?}: {iterations} Newton iterations"
            );
        }
    }
}
