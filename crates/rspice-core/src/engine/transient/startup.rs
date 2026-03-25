use super::{
    AbortSignal, AnalysisCommand, Engine, Netlist, STARTUP_RECOVERY_DELTA_V, SimulationError, Value,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum InitialSolutionMode {
    DcOperatingPoint,
    RobustDcFallback,
    LinearizedSeed,
}

impl Engine {
    pub(super) fn nonlinear_startup_warmup_seed(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        seed: &[Value],
    ) -> Vec<Value> {
        const WARMUP_ITERS: usize = 96;
        const MAX_WARMUP_DELTA_V: Value = 2e-1;

        let size = circuit.matrix_size();
        let mut solution = seed.to_vec();
        let mut rhs = vec![0.0; size];

        for _ in 0..WARMUP_ITERS {
            matrix.clear_values();
            rhs.fill(0.0);

            // Keep the warmup matrix well-conditioned for highly floating
            // transistor stacks while still allowing nonlinear bias formation.
            for i in 0..size {
                matrix.add(i, i, 1e-6);
            }
            circuit.stamp_dc_direct(matrix, &mut rhs);
            if circuit.has_nonlinear_devices() {
                circuit.update_nonlinear(&solution);
                circuit.stamp_nonlinear(matrix, &mut rhs, &solution);
                circuit.stamp_behavioral(matrix, &mut rhs, &solution, 0.0);
            }

            let Ok(mut proposal) = matrix.solve(&rhs) else {
                break;
            };

            for i in 0..size {
                let old = solution[i];
                let mut new_v = proposal[i];
                if !new_v.is_finite() {
                    new_v = old;
                }
                let delta = (new_v - old).clamp(-MAX_WARMUP_DELTA_V, MAX_WARMUP_DELTA_V);
                proposal[i] = old + delta;
            }

            if circuit.has_nonlinear_devices() {
                circuit.update_nonlinear(&proposal);
            }
            if self.voltage_convergence_met(&solution, &proposal) {
                solution = proposal;
                break;
            }
            solution = proposal;
        }

        solution
    }

    pub(super) fn solve_transient_initial_solution(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Value>, InitialSolutionMode), SimulationError> {
        match self.solve_dc_operating_point_with_abort(netlist, circuit, matrix, abort) {
            Ok(solution) => Ok((solution, InitialSolutionMode::DcOperatingPoint)),
            Err(SimulationError::Aborted) => Err(SimulationError::Aborted),
            Err(primary_err) => {
                log::warn!(
                    "Transient initial DC operating point failed: {}. Retrying with robust DC aids.",
                    primary_err
                );

                let mut robust_cfg = self.config.clone();
                robust_cfg.max_iterations = robust_cfg.max_iterations.max(120);
                robust_cfg.convergence_config = super::super::ConvergenceConfig::robust()
                    .with_voltage_tolerances(self.voltage_reltol(), self.voltage_abstol())
                    .with_current_tolerance(self.current_abstol())
                    .with_residual_reltol(self.residual_reltol());
                let robust_engine = super::super::Engine::new(robust_cfg);

                match robust_engine
                    .solve_dc_operating_point_with_abort(netlist, circuit, matrix, abort)
                {
                    Ok(solution) => {
                        log::warn!(
                            "Transient startup recovered using robust DC convergence fallback."
                        );
                        return Ok((solution, InitialSolutionMode::RobustDcFallback));
                    }
                    Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
                    Err(robust_err) => {
                        log::warn!(
                            "Robust transient-start DC retry also failed: {}. Trying linearized startup seed.",
                            robust_err
                        );

                        // Level-6 legacy decks often expose weakly-conditioned DC
                        // operating points where strict re-validation fails even
                        // though continuation produces a materially better startup
                        // state than a pure linearized seed.
                        let has_level6_mos = circuit.mosfets.devices.iter().any(|m| m.level == 6);
                        if has_level6_mos {
                            let seed_guess = vec![0.0; circuit.matrix_size()];
                            match robust_engine.source_stepping_nonlinear_with_guess_and_abort(
                                circuit,
                                matrix,
                                &seed_guess,
                                abort,
                            ) {
                                Ok(mut continuation_seed) => {
                                    for v in &mut continuation_seed {
                                        if !v.is_finite() {
                                            *v = 0.0;
                                        }
                                    }
                                    continuation_seed = self.nonlinear_startup_warmup_seed(
                                        circuit,
                                        matrix,
                                        &continuation_seed,
                                    );
                                    log::warn!(
                                        "Transient startup using Level-6 continuation seed after DC OP failure."
                                    );
                                    return Ok((
                                        continuation_seed,
                                        InitialSolutionMode::LinearizedSeed,
                                    ));
                                }
                                Err(seed_err) => {
                                    log::warn!(
                                        "Level-6 continuation seed failed: {}. Falling back to linearized seed.",
                                        seed_err
                                    );
                                }
                            }
                        }
                    }
                }

                // Last-resort seed: linearized solve with nonlinear devices effectively open.
                // This keeps transient progression possible for strongly nonlinear decks that
                // fail strict t=0 operating-point convergence.
                match self.solve_linear(circuit, matrix) {
                    Ok(mut solution) => {
                        for v in &mut solution {
                            if !v.is_finite() {
                                *v = 0.0;
                            }
                        }
                        solution = self.nonlinear_startup_warmup_seed(circuit, matrix, &solution);
                        log::warn!(
                            "Transient startup using linearized initial seed after DC OP failure."
                        );
                        Ok((solution, InitialSolutionMode::LinearizedSeed))
                    }
                    Err(linear_err) => Err(SimulationError::Circuit(format!(
                        "Transient startup failed: primary DC error: {}; linearized fallback error: {}",
                        primary_err, linear_err
                    ))),
                }
            }
        }
    }

    #[inline]
    pub(super) fn transient_source_step_hint(netlist: &Netlist, max_step: Value) -> Value {
        if let Some(step) = netlist.analyses.iter().find_map(|analysis| match analysis {
            AnalysisCommand::Tran { step, .. } if step.is_finite() && *step > 0.0 => Some(*step),
            _ => None,
        }) {
            step
        } else if max_step.is_finite() && max_step > 0.0 {
            (max_step / 10.0).max(1e-12)
        } else {
            1e-12
        }
    }

    #[inline]
    pub(super) fn startup_step_delta_limit(
        mode: InitialSolutionMode,
        time: Value,
        max_step: Value,
        base_limit: Value,
    ) -> Value {
        if Self::in_startup_recovery_window(mode, time, max_step) {
            base_limit.max(STARTUP_RECOVERY_DELTA_V)
        } else {
            base_limit
        }
    }

    #[inline]
    pub(super) fn in_startup_recovery_window(
        mode: InitialSolutionMode,
        time: Value,
        max_step: Value,
    ) -> bool {
        if mode != InitialSolutionMode::LinearizedSeed {
            return false;
        }
        // Keep the relaxed window bounded so this only assists the initial
        // operating-point recovery region.
        let relaxed_until = (max_step * 32.0).clamp(5e-9, 1e-7);
        time <= relaxed_until
    }

    #[inline]
    pub(super) fn startup_timestep_divisors(has_bjts: bool) -> (Value, Value) {
        if has_bjts {
            (1000.0, 10_000.0)
        } else {
            (10.0, 1000.0)
        }
    }
}
