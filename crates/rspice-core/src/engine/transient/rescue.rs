//! Per-timestep gmin-continuation rescue for the transient Newton loop.
//!
//! A saturation knife edge — a limiter-pinned iterate where no damped
//! fraction of the (junction-limited) Newton step decreases the true
//! residual — is a property of the static nonlinearity, so it repeats
//! identically at every timestep size: cutting dt cannot fix it, and the
//! cut cascade ends in a force-accept storm that poisons the charge
//! history. Production simulators solve such steps by continuation
//! instead: deform the step's system with diagonal shunts until Newton
//! converges trivially, then shrink the shunts level by level, tracking
//! the deformed solution continuously into the basin of the true one.
//! This is the transient sibling of DC gmin stepping (ngspice carries the
//! mechanism for operating points only, which is why decks like the VBIC
//! diffamp fail outright under ngspice when a transient trajectory grazes
//! such a manifold).
//!
//! The rescue reuses the engine's DC gmin schedule and the shared
//! transient system assembly, so the equations per level differ from the
//! main Newton loop's by exactly the diagonal shunt and nothing else. The
//! final level runs with a zero extra shunt: success therefore means the
//! genuine step system converged by the loop's own standards (voltage,
//! device, and residual tests), and the candidate flows into the normal
//! LTE acceptance machinery.

use super::*;

/// Backtracking trials per rescue Newton iteration (smallest fraction 2^-5).
const RESCUE_LINE_SEARCH_TRIALS: usize = 6;
/// Armijo sufficient-decrease coefficient, matching the DC line search.
const RESCUE_LINE_SEARCH_ARMIJO_C1: Value = 1e-4;
/// Bound additional continuation levels when a coarse shunt change fails.
const RESCUE_GMIN_MAX_REFINEMENTS: usize = 32;

impl Engine {
    /// Solve one transient step by gmin continuation after plain Newton has
    /// failed. `seed` is the last accepted solution (the most trustworthy
    /// basin point). Returns the converged candidate for `time = t + dt`,
    /// with the circuit's nonlinear state left updated at that candidate;
    /// returns `None` (with device state restored) when any level fails.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn rescue_transient_step_with_gmin_continuation(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        seed: &[Value],
        time: Value,
        dt: Value,
        ctx: &residual::TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Value>>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let snapshot = circuit.nonlinear_state_snapshot();
        let vbic_snapshot = vbic_snapshot_cache.to_vec();
        let rescued = self.walk_gmin_continuation_levels(
            circuit,
            matrix,
            rhs,
            seed,
            time,
            dt,
            ctx,
            vbic_snapshot_cache,
            abort,
        );
        // The walk ramps the device junction GMIN level by level; restore
        // the configured transient floor whether or not it succeeded.
        circuit.set_semiconductor_junction_gmin(
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target),
        );
        if !matches!(rescued, Ok(Some(_))) {
            circuit.restore_nonlinear_state(snapshot);
            vbic_snapshot_cache.clone_from_slice(&vbic_snapshot);
        }
        rescued
    }

    #[allow(clippy::too_many_arguments)]
    fn walk_gmin_continuation_levels(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        seed: &[Value],
        time: Value,
        dt: Value,
        ctx: &residual::TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Value>>, SimulationError> {
        let num_nodes = circuit.num_nodes();
        let budget = self.transient_newton_iteration_budget(false);

        // The DC schedule walks from a heavily shunted, nearly linear system
        // down to the configured GMIN target; the appended zero level then
        // solves the genuine step system (the baseline GMIN floor is part of
        // the shared assembly, not of this extra shunt).
        let mut levels = self.gmin_nonlinear_schedule();
        levels.push(0.0);

        let mut iterate = seed.to_vec();
        let mut level_index = 0;
        let mut refinements = 0;
        while level_index < levels.len() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let extra_gmin = levels[level_index];
            let level_seed = iterate.clone();
            let level_state = circuit.nonlinear_state_snapshot();
            let level_vbic_cache = vbic_snapshot_cache.to_vec();
            // ngspice gmin stepping moves `CKTgmin` itself, so the junction
            // parallels inside the compact models ramp with the level — that
            // is what flattens an exponential's knife edge; the diagonal
            // shunt alone regularizes only the node rows. The zero level
            // resolves to the configured floor, i.e. the genuine system.
            circuit
                .set_semiconductor_junction_gmin(self.effective_device_junction_gmin(extra_gmin));
            let mut level_converged = false;
            for level_iter in 0..budget {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                self.stamp_transient_system(
                    circuit,
                    matrix,
                    rhs,
                    &iterate,
                    time,
                    dt,
                    ctx,
                    vbic_snapshot_cache,
                    if level_iter == 0 {
                        VbicCachedSnapshotReuse::SeedOnly
                    } else {
                        VbicCachedSnapshotReuse::NewtonBypass
                    },
                    true,
                    extra_gmin,
                )?;
                let line_search_base_state = circuit.nonlinear_state_snapshot();
                let line_search_vbic_cache = vbic_snapshot_cache.to_vec();

                let Ok(mut sol) = matrix.solve(rhs) else {
                    break;
                };

                // Preserve finite Newton states at every scale. Device-local
                // limiting and the deformed-system merit below govern the
                // continuation; non-finite proposals cannot enter that search.
                if sol.iter().any(|value| !value.is_finite()) {
                    break;
                }
                let mut needs_constraint_projection = false;

                if !circuit.bjts.devices.is_empty()
                    && Self::limit_bjt_junction_external_updates(
                        circuit, &mut sol, &iterate, num_nodes, None,
                    )
                {
                    needs_constraint_projection = true;
                }
                if needs_constraint_projection {
                    circuit.enforce_ideal_voltage_constraints(&mut sol, time)?;
                }

                // Merit line search on the deformed system (the DC gmin
                // stepping pattern): accept the first fraction of the
                // limited step that decreases the true residual. The full
                // step is tried first, so healthy Newton iterations pay one
                // restamp and are otherwise untouched; backtracking is what
                // converts wandering around a stiff feedback loop into
                // monotone descent toward the level's solution.
                let full_step = sol;
                circuit.restore_nonlinear_state(line_search_base_state.clone());
                vbic_snapshot_cache.clone_from_slice(&line_search_vbic_cache);
                self.stamp_transient_system_with_generated_mode(
                    circuit,
                    matrix,
                    rhs,
                    &iterate,
                    time,
                    dt,
                    ctx,
                    vbic_snapshot_cache,
                    VbicCachedSnapshotReuse::NewtonBypass,
                    true,
                    extra_gmin,
                    crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe,
                )?;
                let base_merit = self
                    .residual_inf_norm(circuit, matrix, &iterate, rhs)
                    .unwrap_or(Value::INFINITY);
                let mut best_point: Option<Vec<Value>> = None;
                let mut best_merit = Value::INFINITY;
                let mut alpha: Value = 1.0;
                for _trial in 0..RESCUE_LINE_SEARCH_TRIALS {
                    if abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    circuit.restore_nonlinear_state(line_search_base_state.clone());
                    vbic_snapshot_cache.clone_from_slice(&line_search_vbic_cache);
                    let trial = Self::interpolate_solution(&iterate, &full_step, alpha);
                    self.stamp_transient_system_with_generated_mode(
                        circuit,
                        matrix,
                        rhs,
                        &trial,
                        time,
                        dt,
                        ctx,
                        vbic_snapshot_cache,
                        VbicCachedSnapshotReuse::NewtonBypass,
                        true,
                        extra_gmin,
                        crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe,
                    )?;
                    let trial_merit = self
                        .residual_inf_norm(circuit, matrix, &trial, rhs)
                        .unwrap_or(Value::INFINITY);
                    if trial_merit < best_merit {
                        best_merit = trial_merit;
                        best_point = Some(trial);
                    }
                    let armijo_ok = trial_merit <= 1.0
                        || trial_merit <= base_merit * (1.0 - RESCUE_LINE_SEARCH_ARMIJO_C1 * alpha);
                    if armijo_ok {
                        break;
                    }
                    alpha *= 0.5;
                }
                let accepted = best_point.unwrap_or(full_step);
                let accepted_merit = best_merit;

                // Every merit trial starts from the same base state and uses
                // raw generated-device equations. Commit exactly one limited
                // Newton stamp at the selected point so matrix/RHS, limiter
                // history, and convergence flags all describe `accepted`.
                circuit.restore_nonlinear_state(line_search_base_state);
                vbic_snapshot_cache.clone_from_slice(&line_search_vbic_cache);
                self.stamp_transient_system(
                    circuit,
                    matrix,
                    rhs,
                    &accepted,
                    time,
                    dt,
                    ctx,
                    vbic_snapshot_cache,
                    VbicCachedSnapshotReuse::NewtonBypass,
                    true,
                    extra_gmin,
                )?;

                let voltage_converged = Self::check_voltage_convergence_with_tolerances(
                    &iterate[..num_nodes],
                    &accepted[..num_nodes],
                    self.voltage_abstol(),
                    self.voltage_reltol(),
                );
                // `accepted_merit` came from the raw physical probe. The
                // committed Newton stamp may be affine-limited and therefore
                // cannot replace the physical residual as an acceptance test.
                let residual_converged = accepted_merit <= 1.0;
                iterate = accepted;
                if circuit.has_nonlinear_devices() {
                    circuit.update_nonlinear(&iterate);
                }
                let device_converged = !circuit.has_nonlinear_devices()
                    || self.transient_static_device_convergence_met(circuit);

                if voltage_converged && device_converged && residual_converged {
                    level_converged = true;
                    break;
                }
            }

            if !level_converged {
                // A decade-sized shunt change can cross a turning point of
                // the deformed equations. Retry from the last converged level
                // with a smaller change, as in DC continuation. This controls
                // the path without imposing any absolute voltage ceiling.
                if level_index == 0 || refinements >= RESCUE_GMIN_MAX_REFINEMENTS {
                    return Ok(None);
                }
                let previous_gmin = levels[level_index - 1];
                let intermediate = if extra_gmin > 0.0 {
                    (0.5 * previous_gmin.ln() + 0.5 * extra_gmin.ln()).exp()
                } else {
                    0.5 * previous_gmin
                };
                if intermediate <= extra_gmin || intermediate >= previous_gmin {
                    return Ok(None);
                }
                circuit.restore_nonlinear_state(level_state);
                vbic_snapshot_cache.clone_from_slice(&level_vbic_cache);
                iterate = level_seed;
                levels.insert(level_index, intermediate);
                refinements += 1;
                continue;
            }
            level_index += 1;
        }

        // The final level converged with a zero extra shunt, but its
        // residual test judged the linear system stamped at the previous
        // iterate. Prove the candidate against a fresh restamp of the true
        // system so the rescue's success claim matches the acceptance
        // standard used everywhere else.
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let converged = self.transient_nonlinear_residual_converged(
            circuit,
            matrix,
            rhs,
            &iterate,
            time,
            dt,
            ctx,
            None,
            vbic_snapshot_cache,
            None,
            None,
            None,
            None,
        )?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Ok(converged.then_some(iterate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gmin_rescue_cancellation_restores_device_and_charge_cache() {
        let engine = Engine::default();
        let netlist =
            Netlist::parse("rescue rollback\nR1 n 0 1\nQ1 n n n qm\n.model qm NPN(CJS=1n)\n.end\n")
                .unwrap();
        let mut circuit = engine.build_circuit(&netlist).unwrap();
        let mut matrix = engine.build_matrix(&circuit).unwrap();
        circuit.link_indices(&matrix);
        let floor =
            engine.effective_device_junction_gmin(engine.config.convergence_config.gmin_target);
        circuit.set_semiconductor_junction_gmin(floor);
        let seed = [1.0];
        circuit.update_nonlinear(&seed);
        circuit.update_nonlinear(&seed);
        let history =
            Engine::initialize_bjt_history(&circuit, &seed, ReactiveHistorySeed::SolvedBias);
        let snapshot = circuit.bjts.devices[0].charge_snapshot(1.0, 1.0, 1.0, 0.0);
        let mut cache = [Some(snapshot)];
        let expected_cache = circuit.bjts.devices[0]
            .encode_accepted_charge_snapshot_checkpoint(&snapshot)
            .unwrap();
        let expected_device = circuit.bjts.devices[0]
            .accepted_nonlinear_checkpoint()
            .unwrap();
        let coeff = CompanionCoefficients::backward_euler();
        let ctx = residual::TransientSystemContext {
            coeff: &coeff,
            xyce_one_step: false,
            xyce_one_step_order2: false,
            xyce_static_history: None,
            bsim4_trnqs_coeff: &coeff,
            bjt_history: &history,
            jfet_history: &Default::default(),
            diode_history: &Default::default(),
            diode_attempt_cache: None,
            mosfet_history: &Default::default(),
            mosfet_companion_slots: &[],
            vdmos_history: &Default::default(),
            vdmos_companion_slots: &[],
            b3soi_history: &Default::default(),
            b3soi_zero_first_transient_charge_derivative: false,
            bsim3_history: &Default::default(),
            bsim4_history: &Default::default(),
            ekv26_history: &Default::default(),
            suppress_gate_charge: false,
            baseline_diag_gmin: 0.0,
            tline_dc_refs: &[],
            coupled_tline_refs: &[],
            analysis_initial_step: false,
            analysis_final_step: false,
        };
        // Cancel after GMIN changes, after the first trial assembly, and
        // farther into Newton/backtracking. Every exit must restore the same
        // pre-rescue state, including a populated engine-owned charge cache.
        for threshold in [2, 3, 5] {
            let abort = crate::abort_signal::CountingAbort::new(threshold);
            let error = engine
                .rescue_transient_step_with_gmin_continuation(
                    &mut circuit,
                    &mut matrix,
                    &mut [0.0],
                    &seed,
                    1e-9,
                    1e-9,
                    &ctx,
                    &mut cache,
                    &abort,
                )
                .unwrap_err();
            assert!(matches!(error, SimulationError::Aborted));
            assert_eq!(abort.polls_after_abort(), 0);
            let bjt = &circuit.bjts.devices[0];
            assert_eq!(bjt.junction_gmin, floor);
            assert_eq!(
                bjt.accepted_nonlinear_checkpoint().unwrap(),
                expected_device
            );
            assert_eq!(
                bjt.encode_accepted_charge_snapshot_checkpoint(cache[0].as_ref().unwrap())
                    .unwrap(),
                expected_cache
            );
        }
    }
}
