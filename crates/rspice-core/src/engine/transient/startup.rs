use super::{
    AbortSignal, AnalysisCommand, Engine, Netlist, SOURCE_ACTIVE_DELTA, STARTUP_RECOVERY_DELTA_V,
    SimulationError, Value,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum InitialSolutionMode {
    TransientOperatingPoint,
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
        const WARMUP_ITERS: usize = 16;
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
            circuit.stamp_generic_switches(matrix, &mut rhs, 0.0);
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
            if self.node_voltage_convergence_met(&solution, &proposal, circuit.num_nodes()) {
                solution = proposal;
                break;
            }
            solution = proposal;
        }

        solution
    }

    pub(super) fn nonlinear_transient_startup_warmup_seed(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        seed: &[Value],
        time: Value,
    ) -> Vec<Value> {
        const WARMUP_ITERS: usize = 16;
        const MAX_WARMUP_DELTA_V: Value = 2e-1;

        let size = circuit.matrix_size();
        let mut solution = seed.to_vec();
        let mut rhs = vec![0.0; size];

        for _ in 0..WARMUP_ITERS {
            matrix.clear_values();
            rhs.fill(0.0);

            for i in 0..size {
                matrix.add(i, i, 1e-6);
            }
            circuit.stamp_transient_operating_point_direct(matrix, &mut rhs);
            let num_nodes = circuit.num_nodes();
            circuit
                .voltage_sources
                .update_transient_rhs(&mut rhs, time, |br_ordinal| num_nodes + br_ordinal);
            circuit.current_sources.update_transient_rhs(&mut rhs, time);
            circuit.stamp_generic_switches(matrix, &mut rhs, time);

            if circuit.has_nonlinear_devices() {
                circuit.update_nonlinear(&solution);
                circuit.stamp_nonlinear(matrix, &mut rhs, &solution);
                circuit.stamp_behavioral(matrix, &mut rhs, &solution, time);
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
            if self.node_voltage_convergence_met(&solution, &proposal, circuit.num_nodes()) {
                solution = proposal;
                break;
            }
            solution = proposal;
        }

        solution
    }

    fn t0_transient_seed_after_dc_fallback(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        dc_solution: &[Value],
        abort: &dyn AbortSignal,
    ) -> Option<Vec<Value>> {
        let source_delta = circuit
            .voltage_sources
            .max_dc_to_transient_delta(0.0)
            .max(circuit.current_sources.max_dc_to_transient_delta(0.0));
        if source_delta < SOURCE_ACTIVE_DELTA {
            return None;
        }

        let Ok(mut transient_seed) =
            self.solve_linear_transient_operating_point_with_abort(circuit, matrix, 0.0, abort)
        else {
            return None;
        };

        for value in &mut transient_seed {
            if !value.is_finite() {
                *value = 0.0;
            }
        }

        let seed_delta = transient_seed
            .iter()
            .zip(dc_solution.iter())
            .map(|(tran, dc)| (tran - dc).abs())
            .fold(0.0, Value::max);

        if seed_delta < SOURCE_ACTIVE_DELTA {
            return None;
        }

        Some(self.nonlinear_transient_startup_warmup_seed(circuit, matrix, &transient_seed, 0.0))
    }

    fn t0_linearized_transient_startup_seed(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        abort: &dyn AbortSignal,
    ) -> Option<Vec<Value>> {
        let Ok(mut transient_seed) =
            self.solve_linear_transient_operating_point_with_abort(circuit, matrix, 0.0, abort)
        else {
            return None;
        };

        for value in &mut transient_seed {
            if !value.is_finite() {
                *value = 0.0;
            }
        }

        Some(self.nonlinear_transient_startup_warmup_seed(circuit, matrix, &transient_seed, 0.0))
    }

    fn solve_direct_dc_startup_with_abort(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let mut direct_cfg = self.config.clone();
        direct_cfg.convergence_config.gmin_stepping = false;
        direct_cfg.convergence_config.source_stepping = false;
        direct_cfg.convergence_config.pseudo_transient = false;
        direct_cfg.convergence_config.arc_length = false;
        let direct_engine = Engine::new(direct_cfg);
        direct_engine.solve_dc_operating_point_with_abort(netlist, circuit, matrix, abort)
    }

    #[inline]
    fn should_try_configured_dc_startup_aids(circuit: &crate::circuit::Circuit) -> bool {
        const MAX_CONFIGURED_DC_STARTUP_MATRIX_SIZE: usize = 160;
        // Junction-limited (legacy GP) circuits take ngspice's full-step
        // Newton; the configured aid chain is their proven startup path and
        // the linearized t=0 seed cannot substitute for a real operating
        // point, so the size heuristic must not skip the aids for them.
        if Self::junction_limiting_owns_newton_steps(circuit) {
            return true;
        }
        circuit.matrix_size() <= MAX_CONFIGURED_DC_STARTUP_MATRIX_SIZE
    }

    fn transient_startup_from_dc_solution(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        solution: Vec<Value>,
        abort: &dyn AbortSignal,
        mode: InitialSolutionMode,
    ) -> (Vec<Value>, InitialSolutionMode) {
        if let Some(seed) =
            self.t0_transient_seed_after_dc_fallback(circuit, matrix, &solution, abort)
        {
            log::warn!("Transient startup using source-consistent t=0 seed after DC startup.");
            (seed, InitialSolutionMode::LinearizedSeed)
        } else {
            (solution, mode)
        }
    }

    pub(super) fn solve_transient_initial_solution(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Value>, InitialSolutionMode), SimulationError> {
        let transient_node_hints = self.collect_node_voltage_hints(netlist, circuit);
        let transient_op = if circuit.has_nonlinear_devices() {
            self.solve_nonlinear_transient_op_with_node_hints_and_abort(
                circuit,
                matrix,
                0.0,
                &transient_node_hints,
                abort,
            )
        } else {
            self.solve_linear_transient_operating_point_with_abort(circuit, matrix, 0.0, abort)
        };

        match transient_op {
            Ok(solution) => {
                return Ok((solution, InitialSolutionMode::TransientOperatingPoint));
            }
            Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
            Err(transient_err) => {
                log::warn!(
                    "Transient initial operating point failed: {}. Trying direct DC operating point startup.",
                    transient_err
                );
            }
        }

        match self.solve_direct_dc_startup_with_abort(netlist, circuit, matrix, abort) {
            Ok(solution) => {
                return Ok(self.transient_startup_from_dc_solution(
                    circuit,
                    matrix,
                    solution,
                    abort,
                    InitialSolutionMode::DcOperatingPoint,
                ));
            }
            Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
            Err(direct_dc_err) => {
                log::warn!(
                    "Direct transient-start DC operating point failed: {}. Trying bounded startup recovery aids.",
                    direct_dc_err
                );
            }
        }

        if Self::should_try_configured_dc_startup_aids(circuit) {
            match self.solve_dc_operating_point_with_abort(netlist, circuit, matrix, abort) {
                Ok(solution) => {
                    log::warn!("Transient startup recovered using configured DC startup aids.");
                    return Ok(self.transient_startup_from_dc_solution(
                        circuit,
                        matrix,
                        solution,
                        abort,
                        InitialSolutionMode::DcOperatingPoint,
                    ));
                }
                Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
                Err(configured_dc_err) => {
                    log::warn!(
                        "Configured transient-start DC aids failed: {}. Trying source-consistent linearized startup seed.",
                        configured_dc_err
                    );
                }
            }
        }

        if let Some(seed) = self.t0_linearized_transient_startup_seed(circuit, matrix, abort) {
            log::warn!("Transient startup using source-consistent linearized t=0 seed.");
            return Ok((seed, InitialSolutionMode::LinearizedSeed));
        }

        log::warn!(
            "Source-consistent linearized transient seed failed. Falling back to configured DC operating point startup."
        );

        match self.solve_dc_operating_point_with_abort(netlist, circuit, matrix, abort) {
            Ok(solution) => {
                if let Some(seed) =
                    self.t0_transient_seed_after_dc_fallback(circuit, matrix, &solution, abort)
                {
                    log::warn!(
                        "Transient startup using source-consistent t=0 seed after DC OP fallback."
                    );
                    Ok((seed, InitialSolutionMode::LinearizedSeed))
                } else {
                    Ok((solution, InitialSolutionMode::DcOperatingPoint))
                }
            }
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
            return base_limit.max(STARTUP_RECOVERY_DELTA_V);
        }
        base_limit
    }

    #[inline]
    pub(super) fn startup_force_accept_delta_limit(
        mode: InitialSolutionMode,
        time: Value,
        max_step: Value,
        base_limit: Value,
    ) -> Value {
        if Self::in_startup_recovery_window(mode, time, max_step) {
            return base_limit.max(STARTUP_RECOVERY_DELTA_V);
        }
        base_limit
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
            // Keep BJT startup conservative, but avoid picosecond lock-in that can
            // stall long-sweep regression decks with gentle source excitation.
            (50.0, 200.0)
        } else {
            (10.0, 1000.0)
        }
    }

    #[inline]
    pub(super) fn ngspice_initial_timestep(
        stop_time: Value,
        tran_step_hint: Option<Value>,
        hinted_max_step: Value,
    ) -> Value {
        let stop_window = if stop_time.is_finite() && stop_time > 0.0 {
            stop_time / 100.0
        } else {
            Value::INFINITY
        };
        let step_seed = tran_step_hint
            .filter(|step| step.is_finite() && *step > 0.0)
            .or_else(|| {
                hinted_max_step
                    .is_finite()
                    .then_some(hinted_max_step)
                    .filter(|step| *step > 0.0)
            })
            .unwrap_or(stop_window);

        // dctran.c first seeds delta as MIN(TSTOP/100, TSTEP)/10, then
        // cuts the first timepoint after the t=0 breakpoint by another 10.
        (stop_window.min(step_seed) / 100.0).max(1e-30)
    }

    #[inline]
    pub(super) fn ngspice_t0_breakpoint_limited_initial_timestep(
        initial_step: Value,
        first_breakpoint_after_zero: Option<Value>,
    ) -> Value {
        let Some(first_breakpoint_after_zero) = first_breakpoint_after_zero else {
            return initial_step;
        };
        if !(first_breakpoint_after_zero.is_finite() && first_breakpoint_after_zero > 0.0) {
            return initial_step;
        }

        // At t=0, dctran.c is sitting on the initial breakpoint. Once source
        // accept hooks have inserted the first source corner, ngspice limits
        // the first solved timestep to one-tenth of that gap and then applies
        // the first-timepoint divide-by-ten, for an effective gap / 100 cap.
        initial_step.min(first_breakpoint_after_zero / 100.0)
    }

    #[inline]
    pub(super) fn startup_practical_min_timestep(
        has_bjts: bool,
        hinted_max_step: Value,
        min_div: Value,
        tran_step_hint: Option<Value>,
    ) -> Value {
        let mut practical_min = (hinted_max_step / min_div).max(1e-15);
        if has_bjts
            && let Some(step) = tran_step_hint.filter(|step| step.is_finite() && *step > 0.0)
        {
            // Keep stiff BJT decks from collapsing to femtosecond timesteps,
            // while still allowing enough headroom below the requested .tran
            // cadence to resolve startup nonlinearities before force-accept.
            practical_min = practical_min.max((step * 0.05).min(hinted_max_step));
        }
        practical_min
    }

    #[inline]
    pub(super) fn legacy_bjt_startup_retry_floor(
        has_bjts: bool,
        step_time: Value,
        hinted_max_step: Value,
        source_activity_delta: Value,
        initial_timestep: Value,
        preferred_min_timestep: Value,
    ) -> Option<Value> {
        if !has_bjts {
            return None;
        }
        if !source_activity_delta.is_finite() {
            return None;
        }
        let quiet_window_end = if hinted_max_step.is_finite() && hinted_max_step > 0.0 {
            (hinted_max_step * 32.0).clamp(5e-9, 1e-7)
        } else {
            5e-9
        };
        if !step_time.is_finite() || step_time > quiet_window_end {
            return None;
        }

        // Quiet startup points must be allowed to shrink/retry normally. A
        // synthetic floor there is indistinguishable from a true minimum step
        // to the retry controller, which can force-accept nonconverged BJT
        // charge states and poison the transient history before any source
        // transition occurs. Keep the floor only for active source edges, where
        // it prevents pathological recovery-sized steps while still resolving
        // the waveform.
        if source_activity_delta < SOURCE_ACTIVE_DELTA {
            return None;
        }
        let retry_floor = initial_timestep.min(preferred_min_timestep);
        if retry_floor.is_finite() && retry_floor > 0.0 {
            Some((retry_floor * 0.5).max(1e-15).min(retry_floor))
        } else {
            None
        }
    }

    #[inline]
    pub(super) fn ngspice_hard_min_timestep(
        hinted_max_step: Value,
        preferred_min_timestep: Value,
    ) -> Value {
        let hinted_max_step = if hinted_max_step.is_finite() && hinted_max_step > 0.0 {
            hinted_max_step
        } else {
            0.0
        };
        let preferred_min_timestep =
            if preferred_min_timestep.is_finite() && preferred_min_timestep > 0.0 {
                preferred_min_timestep
            } else {
                Value::INFINITY
            };
        let ngspice_delmin = (hinted_max_step * 1e-11).max(1e-30);

        preferred_min_timestep.min(ngspice_delmin)
    }

    #[inline]
    pub(super) fn ngspice_breakpoint_tolerance(hinted_max_step: Value) -> Value {
        let hinted_max_step = if hinted_max_step.is_finite() && hinted_max_step > 0.0 {
            hinted_max_step
        } else {
            0.0
        };

        // Local ngspice is built with XSPICE, so dctran.c initializes
        // CKTminBreak to 10 * CKTdelmin, and traninit.c sets CKTdelmin to
        // 1e-11 * CKTmaxStep.
        (hinted_max_step * 1e-10).max(1e-30)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(lhs: Value, rhs: Value) {
        let scale = lhs.abs().max(rhs.abs()).max(1.0);
        assert!(
            (lhs - rhs).abs() <= 1e-15 * scale,
            "left={lhs:.16e}, right={rhs:.16e}"
        );
    }

    #[test]
    fn t0_breakpoint_caps_initial_timestep_to_ngspice_first_gap_rule() {
        let initial = Engine::ngspice_initial_timestep(10e-6, Some(0.1e-6), 0.1e-6);

        assert_close(initial, 1e-9);
        assert_close(
            Engine::ngspice_t0_breakpoint_limited_initial_timestep(initial, Some(1e-9)),
            1e-11,
        );
    }

    #[test]
    fn t0_breakpoint_leaves_smaller_seed_unchanged() {
        let initial = Engine::ngspice_initial_timestep(150e-9, Some(0.5e-9), 0.5e-9);

        assert_close(initial, 5e-12);
        assert_close(
            Engine::ngspice_t0_breakpoint_limited_initial_timestep(initial, Some(2e-9)),
            5e-12,
        );
    }

    #[test]
    fn breakpoint_tolerance_matches_xspice_minbreak_rule() {
        assert_close(Engine::ngspice_breakpoint_tolerance(1e-11), 1e-21);
        assert_close(Engine::ngspice_breakpoint_tolerance(0.5e-9), 5e-20);
    }
}
