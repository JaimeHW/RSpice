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

impl Engine {
    /// Solve one transient step by gmin continuation after plain Newton has
    /// failed. `seed` is the last accepted solution (the most trustworthy
    /// basin point). Returns the converged candidate for `time = t + dt`,
    /// with the circuit's nonlinear state left updated at that candidate;
    /// returns `None` (with device state restored) when any level fails.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn rescue_transient_step_with_gmin_continuation(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        seed: &[Value],
        time: Value,
        dt: Value,
        ctx: &residual::TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
    ) -> Option<Vec<Value>> {
        let snapshot = circuit.nonlinear_state_snapshot();
        let rescued = self.walk_gmin_continuation_levels(
            circuit,
            matrix,
            rhs,
            seed,
            time,
            dt,
            ctx,
            vbic_snapshot_cache,
        );
        // The walk ramps the device junction GMIN level by level; restore
        // the configured transient floor whether or not it succeeded.
        circuit.set_semiconductor_junction_gmin(
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target),
        );
        if rescued.is_none() {
            circuit.restore_nonlinear_state(snapshot);
        }
        rescued
    }

    #[allow(clippy::too_many_arguments)]
    fn walk_gmin_continuation_levels(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        seed: &[Value],
        time: Value,
        dt: Value,
        ctx: &residual::TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
    ) -> Option<Vec<Value>> {
        let num_nodes = circuit.num_nodes();
        let budget =
            Self::transient_newton_iteration_budget(self.config.transient_max_iterations, false);

        // The DC schedule walks from a heavily shunted, nearly linear system
        // down to the configured GMIN target; the appended zero level then
        // solves the genuine step system (the baseline GMIN floor is part of
        // the shared assembly, not of this extra shunt).
        let mut levels = self.gmin_nonlinear_schedule();
        levels.push(0.0);

        let debug = std::env::var_os("RSPICE_NEWTON_DEBUG").is_some();
        let mut iterate = seed.to_vec();
        for &extra_gmin in &levels {
            // ngspice gmin stepping moves `CKTgmin` itself, so the junction
            // parallels inside the compact models ramp with the level — that
            // is what flattens an exponential's knife edge; the diagonal
            // shunt alone regularizes only the node rows. The zero level
            // resolves to the configured floor, i.e. the genuine system.
            circuit
                .set_semiconductor_junction_gmin(self.effective_device_junction_gmin(extra_gmin));
            let mut level_converged = false;
            let mut last_failure = (false, false, false);
            for level_iter in 0..budget {
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
                );
                let base_merit = matrix
                    .scaled_residual_inf_norm(
                        &iterate,
                        rhs,
                        self.current_abstol(),
                        self.residual_reltol(),
                    )
                    .unwrap_or(Value::INFINITY);

                let Ok(mut sol) = matrix.solve(rhs) else {
                    if debug {
                        log::warn!(
                            "GMIN-RESCUE solve failed at gmin={:.1e} iter={}",
                            extra_gmin,
                            level_iter
                        );
                    }
                    return None;
                };

                // Continuation discipline mirrors DC gmin stepping: hard
                // physical bounds instead of a per-iteration trust region
                // (the deformed solutions legitimately sit volts away from
                // the seed, so motion clamps starve the walk), plus the
                // device-level pnjlim junction limiting.
                let mut needs_constraint_projection = false;
                for (i, value) in sol.iter_mut().enumerate() {
                    let magnitude_limit = if i < num_nodes {
                        MAX_VOLTAGE
                    } else {
                        MAX_BRANCH_STATE_MAGNITUDE
                    };
                    if !value.is_finite() {
                        *value = iterate[i];
                        needs_constraint_projection = true;
                    } else if value.abs() > magnitude_limit {
                        *value = value.signum() * magnitude_limit;
                        needs_constraint_projection = true;
                    }
                }

                if !circuit.bjts.devices.is_empty()
                    && Self::limit_bjt_junction_external_updates(
                        circuit, &mut sol, &iterate, num_nodes, None,
                    )
                {
                    needs_constraint_projection = true;
                }
                if needs_constraint_projection {
                    circuit.enforce_ideal_voltage_constraints(&mut sol, time);
                }

                // Merit line search on the deformed system (the DC gmin
                // stepping pattern): accept the first fraction of the
                // limited step that decreases the true residual. The full
                // step is tried first, so healthy Newton iterations pay one
                // restamp and are otherwise untouched; backtracking is what
                // converts wandering around a stiff feedback loop into
                // monotone descent toward the level's solution.
                let full_step = sol;
                let mut best_point: Option<Vec<Value>> = None;
                let mut best_merit = Value::INFINITY;
                let mut alpha: Value = 1.0;
                for _trial in 0..RESCUE_LINE_SEARCH_TRIALS {
                    let trial: Vec<Value> = if alpha >= 1.0 {
                        full_step.clone()
                    } else {
                        iterate
                            .iter()
                            .zip(&full_step)
                            .map(|(from, to)| from + alpha * (to - from))
                            .collect()
                    };
                    self.stamp_transient_system(
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
                    );
                    let trial_merit = matrix
                        .scaled_residual_inf_norm(
                            &trial,
                            rhs,
                            self.current_abstol(),
                            self.residual_reltol(),
                        )
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

                let voltage_converged = Self::check_voltage_convergence_with_tolerances(
                    &iterate[..num_nodes],
                    &accepted[..num_nodes],
                    self.voltage_abstol(),
                    self.voltage_reltol(),
                );
                // The line search leaves the freshest stamp at (or near) the
                // accepted point, so this judges the true deformed-system
                // residual rather than the linear solve's.
                let residual_converged =
                    accepted_merit <= 1.0 || self.residual_convergence_met(matrix, &accepted, rhs);
                if debug && level_iter >= budget.saturating_sub(6) {
                    let max_dv = Self::max_abs_delta_prefix(&iterate, &accepted, num_nodes);
                    log::warn!(
                        "GMIN-RESCUE walk gmin={:.1e} iter={} max_dv={:.3e} merit={:.3e}",
                        extra_gmin,
                        level_iter,
                        max_dv,
                        accepted_merit
                    );
                }
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
                last_failure = (voltage_converged, device_converged, residual_converged);
            }

            if !level_converged {
                if debug {
                    log::warn!(
                        "GMIN-RESCUE level failed at gmin={:.1e}: voltage_conv={} device_conv={} residual_conv={}",
                        extra_gmin,
                        last_failure.0,
                        last_failure.1,
                        last_failure.2
                    );
                }
                return None;
            }
        }

        // The final level converged with a zero extra shunt, but its
        // residual test judged the linear system stamped at the previous
        // iterate. Prove the candidate against a fresh restamp of the true
        // system so the rescue's success claim matches the acceptance
        // standard used everywhere else.
        if !self.transient_nonlinear_residual_converged(
            circuit,
            matrix,
            rhs,
            &iterate,
            time,
            dt,
            ctx,
            vbic_snapshot_cache,
        ) {
            if debug {
                log::warn!("GMIN-RESCUE final restamp proof failed at t={:.6e}", time);
            }
            return None;
        }

        Some(iterate)
    }
}
