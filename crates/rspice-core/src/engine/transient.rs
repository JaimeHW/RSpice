//! Transient Time-Domain Analysis
//!
//! This module provides time-domain simulation using:
//! - Adaptive timestep control with LTE estimation
//! - TrapGear method switching for stability
//! - Optional waveform compression for long simulations
//! - Cooperative abort for responsive cancellation

use super::{Engine, SimulationError, TransientResult};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::transient::{
    BreakpointManager, CompanionCoefficients, IntegrationMethod, LteEstimator, TimestepController,
    TrapGearController,
};
use crate::analysis::waveform::{CompressionConfig, TransientResultCompressed, WaveformRecorder};
use crate::netlist::AnalysisCommand;
use crate::{Netlist, Value};

/// Maximum voltage limit for solution values (matching DC solver)
///
/// Commercial simulators like Spectre/HSPICE use similar limits to prevent
/// Newton-Raphson divergence on stiff nonlinear circuits (e.g., BJT exponential I-V).
/// This value matches the DC solver's MAX_VOLTAGE in convergence.rs for consistency.
const MAX_VOLTAGE: Value = 1000.0;
/// Maximum allowed per-iteration node update during Newton damping.
///
/// This bound controls nonlinear solve trust-region size.
const MAX_NEWTON_ITER_DELTA_V: Value = 5e-2;
/// Maximum allowed node update when committing force-accepted steps.
///
/// This remains tight to avoid committing nonphysical jumps into reactive history.
const MAX_FORCE_ACCEPT_DELTA_V: Value = 5e-2;
/// Relaxed trust-region limit used only during early startup when DC OP failed and
/// transient had to begin from a linearized seed.
const STARTUP_RECOVERY_DELTA_V: Value = 2e-1;
/// Source edge magnitude that triggers transient source-step capping.
const SOURCE_ACTIVE_DELTA: Value = 1e-2;

#[derive(Debug, Clone, Default)]
struct JfetTransientHistory {
    vgs_prev: Vec<Value>,
    vgs_prev_prev: Vec<Value>,
    qgs_prev: Vec<Value>,
    qgs_prev_prev: Vec<Value>,
    cqgs_prev: Vec<Value>,
    vgd_prev: Vec<Value>,
    vgd_prev_prev: Vec<Value>,
    qgd_prev: Vec<Value>,
    qgd_prev_prev: Vec<Value>,
    cqgd_prev: Vec<Value>,
}

#[derive(Debug, Clone, Default)]
struct BjtTransientHistory {
    vbe_prev: Vec<Value>,
    vbe_prev_prev: Vec<Value>,
    ibe_prev: Vec<Value>,
    vbc_prev: Vec<Value>,
    vbc_prev_prev: Vec<Value>,
    ibc_prev: Vec<Value>,
    vcs_prev: Vec<Value>,
    vcs_prev_prev: Vec<Value>,
    ics_prev: Vec<Value>,
}

#[derive(Debug, Clone, Default)]
struct MosfetTransientHistory {
    vgs_prev: Vec<Value>,
    vgs_prev_prev: Vec<Value>,
    igs_prev: Vec<Value>,
    vgd_prev: Vec<Value>,
    vgd_prev_prev: Vec<Value>,
    igd_prev: Vec<Value>,
    vgb_prev: Vec<Value>,
    vgb_prev_prev: Vec<Value>,
    igb_prev: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InitialSolutionMode {
    DcOperatingPoint,
    RobustDcFallback,
    LinearizedSeed,
}

impl Engine {
    fn nonlinear_startup_warmup_seed(
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

    fn solve_transient_initial_solution(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Value>, InitialSolutionMode), SimulationError> {
        match self.solve_dc_operating_point_with_abort(netlist, circuit, matrix, abort) {
            Ok(solution) => return Ok((solution, InitialSolutionMode::DcOperatingPoint)),
            Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
            Err(primary_err) => {
                log::warn!(
                    "Transient initial DC operating point failed: {}. Retrying with robust DC aids.",
                    primary_err
                );

                let mut robust_cfg = self.config.clone();
                robust_cfg.max_iterations = robust_cfg.max_iterations.max(120);
                robust_cfg.convergence_config = super::ConvergenceConfig::robust()
                    .with_voltage_tolerances(self.voltage_reltol(), self.voltage_abstol())
                    .with_current_tolerance(self.current_abstol())
                    .with_residual_reltol(self.residual_reltol());
                let robust_engine = super::Engine::new(robust_cfg);

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
    fn transient_source_step_hint(netlist: &Netlist, max_step: Value) -> Value {
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
    fn add_breakpoint_if_in_range(breakpoints: &mut BreakpointManager, time: Value, tstop: Value) {
        if time.is_finite() && time >= 0.0 && time <= tstop {
            breakpoints.add(time);
        }
    }

    fn add_source_spec_breakpoints(
        breakpoints: &mut BreakpointManager,
        spec: &crate::netlist::SourceSpec,
        tstop: Value,
        tstep_hint: Value,
    ) {
        use crate::netlist::SourceSpec;

        match spec {
            SourceSpec::Dc(_) | SourceSpec::Ac { .. } | SourceSpec::DcAc { .. } => {}
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => {
                Self::add_source_spec_breakpoints(breakpoints, transient, tstop, tstep_hint);
            }
            SourceSpec::Pulse {
                delay,
                rise,
                fall,
                width,
                period,
                ..
            } => {
                let step_default = tstep_hint.max(1e-18);
                let stop_default = tstop.max(1e-18);
                let td = if delay.is_finite() {
                    delay.max(0.0)
                } else {
                    0.0
                };
                let tr = if rise.is_nan() { step_default } else { *rise };
                let tf = if fall.is_nan() { step_default } else { *fall };
                let pw = if width.is_nan() { stop_default } else { *width };
                let per = if period.is_nan() {
                    stop_default
                } else {
                    *period
                };

                let tr = if tr.is_finite() && tr > 0.0 {
                    tr
                } else {
                    step_default
                };
                let tf = if tf.is_finite() && tf > 0.0 {
                    tf
                } else {
                    step_default
                };
                let pw = if pw.is_finite() && pw >= 0.0 {
                    pw
                } else {
                    stop_default
                };

                let per_valid = per.is_finite() && per > 0.0;
                let max_cycles = if per_valid {
                    (((tstop - td).max(0.0) / per).ceil() as usize).saturating_add(1)
                } else {
                    1
                };
                let max_cycles = max_cycles.min(1_000_000);

                for cycle in 0..max_cycles {
                    let cycle_start = if per_valid {
                        td + per * cycle as Value
                    } else {
                        td
                    };
                    if cycle_start > tstop {
                        break;
                    }
                    Self::add_breakpoint_if_in_range(breakpoints, cycle_start, tstop);
                    Self::add_breakpoint_if_in_range(breakpoints, cycle_start + tr, tstop);
                    Self::add_breakpoint_if_in_range(breakpoints, cycle_start + tr + pw, tstop);
                    Self::add_breakpoint_if_in_range(
                        breakpoints,
                        cycle_start + tr + pw + tf,
                        tstop,
                    );
                    if !per_valid {
                        break;
                    }
                }
            }
            SourceSpec::Sin { delay, .. } => {
                Self::add_breakpoint_if_in_range(breakpoints, *delay, tstop);
            }
            SourceSpec::Pwl { points } => {
                for (time, _) in points {
                    Self::add_breakpoint_if_in_range(breakpoints, *time, tstop);
                }
            }
            SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
            } => match crate::device::pwl_file::load_pwl_file(path) {
                Ok(wf) => {
                    let wf =
                        wf.with_scaling(*time_scale, *value_scale, *time_offset, *value_offset);
                    for time in wf.scaled_knot_times() {
                        Self::add_breakpoint_if_in_range(breakpoints, time, tstop);
                    }
                }
                Err(err) => {
                    log::warn!(
                        "Failed to load PWL file '{}' for breakpoint extraction: {}",
                        path,
                        err
                    );
                }
            },
            SourceSpec::Exp { td1, td2, .. } => {
                Self::add_breakpoint_if_in_range(breakpoints, *td1, tstop);
                Self::add_breakpoint_if_in_range(breakpoints, *td2, tstop);
            }
        }
    }

    fn collect_transient_source_breakpoints(
        circuit: &crate::circuit::Circuit,
        tstop: Value,
        tstep_hint: Value,
        breakpoints: &mut BreakpointManager,
    ) {
        for spec in circuit
            .voltage_sources
            .source_specs
            .iter()
            .chain(circuit.current_sources.source_specs.iter())
            .filter_map(|spec| spec.as_ref())
        {
            Self::add_source_spec_breakpoints(breakpoints, spec, tstop, tstep_hint);
        }
    }

    #[inline]
    fn startup_step_delta_limit(
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
    fn in_startup_recovery_window(mode: InitialSolutionMode, time: Value, max_step: Value) -> bool {
        if mode != InitialSolutionMode::LinearizedSeed {
            return false;
        }
        // Keep the relaxed window bounded so this only assists the initial
        // operating-point recovery region.
        let relaxed_until = (max_step * 32.0).clamp(5e-9, 1e-7);
        time <= relaxed_until
    }

    #[inline]
    fn startup_timestep_divisors(has_bjts: bool) -> (Value, Value) {
        if has_bjts {
            (1000.0, 10_000.0)
        } else {
            (10.0, 1000.0)
        }
    }

    /// Run transient time-domain analysis
    ///
    /// Uses adaptive integration with automatic method switching (TrapGear).
    /// Trapezoidal integration is used normally for efficiency, but switches
    /// to Gear2/BDF2 when oscillations are detected for stability.
    ///
    /// For cancellable simulations, use [`run_tran_with_abort`] instead.
    pub fn run_tran(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
    ) -> Result<TransientResult, SimulationError> {
        self.run_tran_with_abort(netlist, tstop, max_step, &NoAbort)
    }

    #[inline]
    fn max_abs_delta_prefix(a: &[Value], b: &[Value], count: usize) -> Value {
        a.iter()
            .zip(b.iter())
            .take(count)
            .map(|(x, y)| (x - y).abs())
            .fold(0.0, Value::max)
    }

    #[inline]
    fn is_stale_step(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        expected_source_delta: Value,
        num_nodes: usize,
    ) -> bool {
        // No externally-driven movement expected for this step.
        if expected_source_delta <= 1e-12 {
            return false;
        }

        // If the entire solution moves orders of magnitude less than the source
        // should have moved, the solver likely accepted a stale state.
        let observed_delta =
            Self::max_abs_delta_prefix(previous_solution, candidate_solution, num_nodes);
        observed_delta <= expected_source_delta * 1e-3
    }

    #[inline]
    fn is_unbounded_step(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        expected_source_delta: Value,
        num_nodes: usize,
    ) -> bool {
        let observed_delta =
            Self::max_abs_delta_prefix(previous_solution, candidate_solution, num_nodes);
        // Guard only truly explosive step jumps. We intentionally allow bounded
        // multi-volt recovery movement here because force-accept paths may need
        // larger-than-source-following corrections to escape stiff NR stalls.
        let drive_scale = expected_source_delta.max(1e-6);
        let threshold = (drive_scale * 1e5).max(50.0);
        observed_delta > threshold
    }

    #[inline]
    fn is_clipped_force_candidate(
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
    fn node_voltage(solution: &[Value], node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            solution.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    fn differential_voltage(solution: &[Value], node_pos: usize, node_neg: usize) -> Value {
        Self::node_voltage(solution, node_pos) - Self::node_voltage(solution, node_neg)
    }

    #[inline]
    fn tline_transient_port_impedance(tl: &crate::device::TransmissionLine) -> Value {
        // For RLGC-based lossy lines, include a calibrated fraction of the
        // total distributed series resistance in each port's local driving-point
        // impedance. This better tracks ngspice LTRA/TXL near-end behavior than
        // a pure 1/Z0 companion while keeping lossless lines unchanged.
        const RLGC_PORT_R_WEIGHT: Value = 0.82;
        (tl.impedance() + RLGC_PORT_R_WEIGHT * tl.dc_series_resistance()).max(1e-12)
    }

    #[inline]
    fn tline_transient_port_conductance(tl: &crate::device::TransmissionLine) -> Value {
        1.0 / Self::tline_transient_port_impedance(tl)
    }

    #[inline]
    fn stamp_tline_port(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        node_pos: usize,
        node_neg: usize,
        g: Value,
        i_eq: Value,
    ) {
        if node_pos > 0 {
            matrix.add(node_pos - 1, node_pos - 1, g);
            if node_neg > 0 {
                matrix.add(node_pos - 1, node_neg - 1, -g);
            }
            rhs[node_pos - 1] += i_eq;
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add(node_neg - 1, node_pos - 1, -g);
            }
            matrix.add(node_neg - 1, node_neg - 1, g);
            rhs[node_neg - 1] -= i_eq;
        }
    }

    #[inline]
    fn stamp_two_terminal_companion(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        node_pos: usize,
        node_neg: usize,
        geq: Value,
        i_eq: Value,
    ) {
        if node_pos > 0 {
            matrix.add(node_pos - 1, node_pos - 1, geq);
            if node_neg > 0 {
                matrix.add(node_pos - 1, node_neg - 1, -geq);
            }
            rhs[node_pos - 1] += i_eq;
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add(node_neg - 1, node_pos - 1, -geq);
            }
            matrix.add(node_neg - 1, node_neg - 1, geq);
            rhs[node_neg - 1] -= i_eq;
        }
    }

    #[inline]
    fn jfet_branch_voltages(jfet: &crate::device::Jfet, voltages: &[Value]) -> (Value, Value) {
        if matches!(
            jfet.params.channel_model,
            crate::device::JfetChannelModel::Hfet1
        ) {
            if let Some((vgs, vgd, _vds)) = jfet.internal_branch_state_voltages() {
                return (vgs, vgd);
            }
        }
        let vg = Self::node_voltage(voltages, jfet.gate);
        let vd = Self::node_voltage(voltages, jfet.drain);
        let vs = Self::node_voltage(voltages, jfet.source);
        (vg - vs, vg - vd)
    }

    #[inline]
    fn jfet_charge_branch_voltages(
        jfet: &crate::device::Jfet,
        voltages: &[Value],
    ) -> (Value, Value) {
        // For HFET/MESA models, drive charge-history updates from limited internal
        // branch state (ngspice-compatible vgspp/vgdpp behavior). For legacy
        // Shichman-Hodges JFETs, use raw terminal deltas.
        if matches!(
            jfet.params.channel_model,
            crate::device::JfetChannelModel::Hfet1
        ) {
            if let Some((vgs, vgd, _vds)) = jfet.internal_branch_state_voltages() {
                return (vgs, vgd);
            }
        }

        let vg = Self::node_voltage(voltages, jfet.gate);
        let vd = Self::node_voltage(voltages, jfet.drain);
        let vs = Self::node_voltage(voltages, jfet.source);
        (vg - vs, vg - vd)
    }

    #[inline]
    fn effective_trapezoidal_order(
        method: IntegrationMethod,
        trap_order: u8,
        at_breakpoint: bool,
    ) -> u8 {
        match method {
            IntegrationMethod::BackwardEuler => 1,
            IntegrationMethod::Gear2 => 2,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => {
                if at_breakpoint {
                    1
                } else {
                    trap_order.clamp(1, 2)
                }
            }
        }
    }

    #[inline]
    fn jfet_companion_geq(
        method: IntegrationMethod,
        trap_order: u8,
        capacitance: Value,
        dt: Value,
    ) -> Value {
        if !capacitance.is_finite() || capacitance <= 0.0 || !dt.is_finite() || dt <= 0.0 {
            return 0.0;
        }
        match method {
            IntegrationMethod::BackwardEuler => capacitance / dt,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => {
                if trap_order <= 1 {
                    capacitance / dt
                } else {
                    2.0 * capacitance / dt
                }
            }
            IntegrationMethod::Gear2 => 1.5 * capacitance / dt,
        }
    }

    #[inline]
    fn jfet_companion_ccap(
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_curr: Value,
        q_prev: Value,
        q_prev_prev: Value,
        cq_prev: Value,
    ) -> Value {
        if !dt.is_finite() || dt <= 0.0 {
            return 0.0;
        }
        match method {
            IntegrationMethod::BackwardEuler => (q_curr - q_prev) / dt,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => {
                if trap_order <= 1 {
                    (q_curr - q_prev) / dt
                } else {
                    -cq_prev + 2.0 * (q_curr - q_prev) / dt
                }
            }
            IntegrationMethod::Gear2 => (1.5 * q_curr - 2.0 * q_prev + 0.5 * q_prev_prev) / dt,
        }
    }

    #[inline]
    fn jfet_companion_terms(
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        capacitance: Value,
        v_curr: Value,
        v_prev: Value,
        q_prev: Value,
        q_prev_prev: Value,
        cq_prev: Value,
    ) -> (Value, Value, Value, Value) {
        let geq = Self::jfet_companion_geq(method, trap_order, capacitance, dt);
        if geq == 0.0 {
            return (0.0, 0.0, q_prev, 0.0);
        }
        // Match ngspice HFET/JFET transient charge update:
        // q(n+1) = q(n) + C(n+1) * (v(n+1) - v(n))
        let q_curr = q_prev + capacitance * (v_curr - v_prev);
        let cq_curr =
            Self::jfet_companion_ccap(method, trap_order, dt, q_curr, q_prev, q_prev_prev, cq_prev);
        // Match ngspice load linearization contract for capacitive branches:
        //   i(v) ≈ ccap + geq * (v - v_hist) = geq * v - (geq * v_hist - ccap).
        // With our companion stamp convention (i = geq * v - i_eq), this gives:
        //   i_eq = geq * v_hist - ccap.
        // NOTE: This intentionally uses branch voltage history, not charge, because
        // q is not generally equal to C * v for voltage-dependent capacitances.
        let ieq = geq * v_curr - cq_curr;
        (geq, ieq, q_curr, cq_curr)
    }

    #[inline]
    fn initialize_bjt_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> BjtTransientHistory {
        let n = circuit.bjts.devices.len();
        let mut history = BjtTransientHistory {
            vbe_prev: Vec::with_capacity(n),
            vbe_prev_prev: Vec::with_capacity(n),
            ibe_prev: Vec::with_capacity(n),
            vbc_prev: Vec::with_capacity(n),
            vbc_prev_prev: Vec::with_capacity(n),
            ibc_prev: Vec::with_capacity(n),
            vcs_prev: Vec::with_capacity(n),
            vcs_prev_prev: Vec::with_capacity(n),
            ics_prev: Vec::with_capacity(n),
        };

        for bjt in &circuit.bjts.devices {
            let vc = Self::node_voltage(solution, bjt.node_collector);
            let vb = Self::node_voltage(solution, bjt.node_base);
            let ve = Self::node_voltage(solution, bjt.node_emitter);
            let vs = Self::node_voltage(solution, bjt.node_substrate);
            let vbe = vb - ve;
            let vbc = vb - vc;
            let vcs = vc - vs;
            history.vbe_prev.push(vbe);
            history.vbe_prev_prev.push(vbe);
            history.ibe_prev.push(0.0);
            history.vbc_prev.push(vbc);
            history.vbc_prev_prev.push(vbc);
            history.ibc_prev.push(0.0);
            history.vcs_prev.push(vcs);
            history.vcs_prev_prev.push(vcs);
            history.ics_prev.push(0.0);
        }

        history
    }

    #[inline]
    fn initialize_jfet_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> JfetTransientHistory {
        let n = circuit.jfets.len();
        let mut history = JfetTransientHistory {
            vgs_prev: Vec::with_capacity(n),
            vgs_prev_prev: Vec::with_capacity(n),
            qgs_prev: Vec::with_capacity(n),
            qgs_prev_prev: Vec::with_capacity(n),
            cqgs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            qgd_prev: Vec::with_capacity(n),
            qgd_prev_prev: Vec::with_capacity(n),
            cqgd_prev: Vec::with_capacity(n),
        };

        for jfet in &circuit.jfets {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, solution);
            let (vgs_charge, vgd_charge) = Self::jfet_charge_branch_voltages(jfet, solution);
            let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);
            let qgs = cgs.max(0.0) * vgs_charge;
            let qgd = cgd.max(0.0) * vgd_charge;
            history.vgs_prev.push(vgs_charge);
            history.vgs_prev_prev.push(vgs_charge);
            history.qgs_prev.push(qgs);
            history.qgs_prev_prev.push(qgs);
            history.cqgs_prev.push(0.0);
            history.vgd_prev.push(vgd_charge);
            history.vgd_prev_prev.push(vgd_charge);
            history.qgd_prev.push(qgd);
            history.qgd_prev_prev.push(qgd);
            history.cqgd_prev.push(0.0);
        }

        history
    }

    #[inline]
    fn initialize_mosfet_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> MosfetTransientHistory {
        let n = circuit.mosfets.len();
        let mut history = MosfetTransientHistory {
            vgs_prev: Vec::with_capacity(n),
            vgs_prev_prev: Vec::with_capacity(n),
            igs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            igd_prev: Vec::with_capacity(n),
            vgb_prev: Vec::with_capacity(n),
            vgb_prev_prev: Vec::with_capacity(n),
            igb_prev: Vec::with_capacity(n),
        };

        for mos in &circuit.mosfets.devices {
            let vg = Self::node_voltage(solution, mos.node_gate);
            let vd = Self::node_voltage(solution, mos.node_drain);
            let vs = Self::node_voltage(solution, mos.node_source);
            let vb = Self::node_voltage(solution, mos.node_bulk);
            let vgs = vg - vs;
            let vgd = vg - vd;
            let vgb = vg - vb;

            history.vgs_prev.push(vgs);
            history.vgs_prev_prev.push(vgs);
            history.igs_prev.push(0.0);

            history.vgd_prev.push(vgd);
            history.vgd_prev_prev.push(vgd);
            history.igd_prev.push(0.0);

            history.vgb_prev.push(vgb);
            history.vgb_prev_prev.push(vgb);
            history.igb_prev.push(0.0);
        }

        history
    }

    #[inline]
    fn stamp_bjt_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
    ) {
        let effective_method = match method {
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear if trap_order <= 1 => {
                IntegrationMethod::BackwardEuler
            }
            _ => method,
        };
        let coeff = CompanionCoefficients::for_method(effective_method);
        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            let vbe = history.vbe_prev[idx];
            let vbc = history.vbc_prev[idx];
            let (cbe, cbc) = bjt.junction_capacitances(vbe, vbc);

            if cbe.is_finite() && cbe > 0.0 {
                let geq = coeff.capacitor_geq(cbe, dt);
                let ieq = coeff.capacitor_ieq(
                    cbe,
                    dt,
                    history.vbe_prev[idx],
                    history.vbe_prev_prev[idx],
                    history.ibe_prev[idx],
                );
                Self::stamp_two_terminal_companion(
                    matrix,
                    rhs,
                    bjt.node_base,
                    bjt.node_emitter,
                    geq,
                    ieq,
                );
            }

            if cbc.is_finite() && cbc > 0.0 {
                let geq = coeff.capacitor_geq(cbc, dt);
                let ieq = coeff.capacitor_ieq(
                    cbc,
                    dt,
                    history.vbc_prev[idx],
                    history.vbc_prev_prev[idx],
                    history.ibc_prev[idx],
                );
                Self::stamp_two_terminal_companion(
                    matrix,
                    rhs,
                    bjt.node_base,
                    bjt.node_collector,
                    geq,
                    ieq,
                );
            }

            let ccs = bjt.cjcp;
            if ccs.is_finite()
                && ccs > 0.0
                && bjt.node_collector != bjt.node_substrate
            {
                let geq = coeff.capacitor_geq(ccs, dt);
                let ieq = coeff.capacitor_ieq(
                    ccs,
                    dt,
                    history.vcs_prev[idx],
                    history.vcs_prev_prev[idx],
                    history.ics_prev[idx],
                );
                Self::stamp_two_terminal_companion(
                    matrix,
                    rhs,
                    bjt.node_collector,
                    bjt.node_substrate,
                    geq,
                    ieq,
                );
            }
        }
    }

    #[inline]
    fn stamp_jfet_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &JfetTransientHistory,
    ) {
        let effective_method = match method {
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear if trap_order <= 1 => {
                IntegrationMethod::BackwardEuler
            }
            _ => method,
        };
        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, voltages);
            let (vgs_charge, vgd_charge) = Self::jfet_charge_branch_voltages(jfet, voltages);
            let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);

            if cgs.is_finite() && cgs > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    cgs,
                    vgs_charge,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.cqgs_prev[idx],
                );
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.gate, jfet.source, geq, ieq);
            }

            if cgd.is_finite() && cgd > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    cgd,
                    vgd_charge,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.cqgd_prev[idx],
                );
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.gate, jfet.drain, geq, ieq);
            }
        }
    }

    #[inline]
    fn stamp_mosfet_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
        history: &MosfetTransientHistory,
    ) {
        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            if mos.level == 6 {
                continue;
            }
            let (cgs, cgd, cgb) = mos.overlap_capacitances();

            if cgs.is_finite() && cgs > 0.0 {
                let geq = coeff.capacitor_geq(cgs, dt);
                let ieq = coeff.capacitor_ieq(
                    cgs,
                    dt,
                    history.vgs_prev[idx],
                    history.vgs_prev_prev[idx],
                    history.igs_prev[idx],
                );
                Self::stamp_two_terminal_companion(
                    matrix,
                    rhs,
                    mos.node_gate,
                    mos.node_source,
                    geq,
                    ieq,
                );
            }

            if cgd.is_finite() && cgd > 0.0 {
                let geq = coeff.capacitor_geq(cgd, dt);
                let ieq = coeff.capacitor_ieq(
                    cgd,
                    dt,
                    history.vgd_prev[idx],
                    history.vgd_prev_prev[idx],
                    history.igd_prev[idx],
                );
                Self::stamp_two_terminal_companion(
                    matrix,
                    rhs,
                    mos.node_gate,
                    mos.node_drain,
                    geq,
                    ieq,
                );
            }

            if cgb.is_finite() && cgb > 0.0 {
                let geq = coeff.capacitor_geq(cgb, dt);
                let ieq = coeff.capacitor_ieq(
                    cgb,
                    dt,
                    history.vgb_prev[idx],
                    history.vgb_prev_prev[idx],
                    history.igb_prev[idx],
                );
                Self::stamp_two_terminal_companion(
                    matrix,
                    rhs,
                    mos.node_gate,
                    mos.node_bulk,
                    geq,
                    ieq,
                );
            }
        }
    }

    #[inline]
    fn stamp_tline_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        tline_dc_refs: &[(Value, Value)],
    ) {
        for (idx, tl) in circuit.tlines.iter().enumerate() {
            let g = Self::tline_transient_port_conductance(tl);
            let (v1_ref, v2_ref) = tline_dc_refs.get(idx).copied().unwrap_or((0.0, 0.0));
            let atten = tl.attenuation();

            // Propagate deviations from the initial DC operating point so
            // constant biases remain invariant even on attenuated lines.
            let incoming1 = tl.delayed_backward_at(time) + (1.0 - atten) * v2_ref;
            let incoming2 = tl.delayed_forward_at(time) + (1.0 - atten) * v1_ref;
            let i_eq_p1 = g * incoming1;
            let i_eq_p2 = g * incoming2;

            Self::stamp_tline_port(matrix, rhs, tl.node1_pos, tl.node1_neg, g, i_eq_p1);
            Self::stamp_tline_port(matrix, rhs, tl.node2_pos, tl.node2_neg, g, i_eq_p2);
        }
    }

    #[inline]
    fn initialize_tline_history(
        circuit: &mut crate::circuit::Circuit,
        initial_solution: &[Value],
        initial_time: Value,
    ) -> Vec<(Value, Value)> {
        let mut refs = Vec::with_capacity(circuit.tlines.len());
        for tl in &mut circuit.tlines {
            tl.reset();
            let z_port = Self::tline_transient_port_impedance(tl);
            let g = 1.0 / z_port;
            let v1 = Self::differential_voltage(initial_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(initial_solution, tl.node2_pos, tl.node2_neg);
            refs.push((v1, v2));

            // Seed delayed-wave state from the initial OP so pre-edge steady states
            // are preserved (avoids artificial startup droop/ringing).
            // Port equations: i1 = g*(v1 - incoming1), i2 = g*(v2 - incoming2),
            // with incoming1 <- v2 and incoming2 <- v1 at t=0.
            let i1_actual = g * (v1 - v2);
            let i2_actual = g * (v2 - v1);
            let wave_scale = z_port / tl.impedance();
            tl.update_history(
                initial_time,
                v1,
                i1_actual * wave_scale,
                v2,
                i2_actual * wave_scale,
            );
        }
        refs
    }

    #[inline]
    fn recover_timestep_after_accepted_step(
        timestep: &mut TimestepController,
        lte_estimator: &LteEstimator,
        accepted_solution: &[Value],
        dt: Value,
        max_step: Value,
        is_strictly_linear_transient: bool,
        expected_source_delta: Value,
    ) {
        let scale = if is_strictly_linear_transient {
            1.0
        } else {
            let (lte, _) = lte_estimator.estimate(accepted_solution, dt);
            lte_estimator.recommend_scale(lte)
        };

        let mut next_dt = if scale > 1.0 {
            (dt * scale.min(1.5)).min(max_step)
        } else {
            (dt * 1.25).min(max_step)
        };
        if expected_source_delta.is_finite() && expected_source_delta > 0.0 {
            let source_cap = dt * (SOURCE_ACTIVE_DELTA / expected_source_delta).clamp(1.0, 4.0);
            next_dt = next_dt.min(source_cap);
        }
        timestep.force_step(next_dt);
    }

    #[inline]
    fn update_reactive_history(
        circuit: &mut crate::circuit::Circuit,
        accepted_solution: &[Value],
        accepted_time: Value,
        dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        bjt_history: &mut BjtTransientHistory,
        jfet_history: &mut JfetTransientHistory,
        mosfet_history: &mut MosfetTransientHistory,
        tline_dc_refs: &[(Value, Value)],
    ) {
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_new = Self::differential_voltage(accepted_solution, np, nn);

            // Compute new capacitor current from OLD history before rotating it.
            let coeff_update = CompanionCoefficients::for_method(method);
            let geq = coeff_update.capacitor_geq(circuit.capacitors.capacitances[cap_idx], dt);
            let ieq = coeff_update.capacitor_ieq(
                circuit.capacitors.capacitances[cap_idx],
                dt,
                circuit.capacitors.v_prev[cap_idx],
                circuit.capacitors.v_prev_prev[cap_idx],
                circuit.capacitors.i_prev[cap_idx],
            );
            let i_new = geq * v_new - ieq;

            let v_old = circuit.capacitors.v_prev[cap_idx];
            circuit.capacitors.v_prev_prev[cap_idx] = v_old;
            circuit.capacitors.v_prev[cap_idx] = v_new;
            circuit.capacitors.i_prev[cap_idx] = i_new;
        }

        for l_idx in 0..circuit.inductors.names.len() {
            let br = circuit.inductors.branch_indices[l_idx];
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_new = accepted_solution[br_idx];
                circuit.inductors.i_prev_prev[l_idx] = circuit.inductors.i_prev[l_idx];
                circuit.inductors.i_prev[l_idx] = i_new;

                let np = circuit.inductors.node_pos[l_idx];
                let nn = circuit.inductors.node_neg[l_idx];
                let v_new = Self::differential_voltage(accepted_solution, np, nn);
                circuit.inductors.v_prev[l_idx] = v_new;
            }
        }
        circuit.refresh_jiles_atherton_inductances(accepted_solution);

        // Update transmission-line delayed-wave history from the accepted state.
        for (idx, tl) in circuit.tlines.iter_mut().enumerate() {
            let z_port = Self::tline_transient_port_impedance(tl);
            let g = 1.0 / z_port;
            let v1 = Self::differential_voltage(accepted_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(accepted_solution, tl.node2_pos, tl.node2_neg);
            let (v1_ref, v2_ref) = tline_dc_refs.get(idx).copied().unwrap_or((0.0, 0.0));
            let atten = tl.attenuation();
            let incoming_port1 = tl.delayed_backward_at(accepted_time) + (1.0 - atten) * v2_ref;
            let incoming_port2 = tl.delayed_forward_at(accepted_time) + (1.0 - atten) * v1_ref;
            let i1_actual = g * v1 - g * incoming_port1;
            let i2_actual = g * v2 - g * incoming_port2;
            let wave_scale = z_port / tl.impedance();
            tl.update_history(
                accepted_time,
                v1,
                i1_actual * wave_scale,
                v2,
                i2_actual * wave_scale,
            );
        }

        let effective_method = match method {
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear if trap_order <= 1 => {
                IntegrationMethod::BackwardEuler
            }
            _ => method,
        };
        let coeff_update = CompanionCoefficients::for_method(effective_method);
        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            let vc = Self::node_voltage(accepted_solution, bjt.node_collector);
            let vb = Self::node_voltage(accepted_solution, bjt.node_base);
            let ve = Self::node_voltage(accepted_solution, bjt.node_emitter);
            let vs = Self::node_voltage(accepted_solution, bjt.node_substrate);
            let vbe = vb - ve;
            let vbc = vb - vc;
            let vcs = vc - vs;
            let (cbe, cbc) = bjt.junction_capacitances(vbe, vbc);

            if cbe.is_finite() && cbe > 0.0 {
                let geq = coeff_update.capacitor_geq(cbe, dt);
                let ieq = coeff_update.capacitor_ieq(
                    cbe,
                    dt,
                    bjt_history.vbe_prev[idx],
                    bjt_history.vbe_prev_prev[idx],
                    bjt_history.ibe_prev[idx],
                );
                let i_new = geq * vbe - ieq;
                let v_old = bjt_history.vbe_prev[idx];
                bjt_history.vbe_prev_prev[idx] = v_old;
                bjt_history.vbe_prev[idx] = vbe;
                bjt_history.ibe_prev[idx] = i_new;
            } else {
                bjt_history.vbe_prev_prev[idx] = bjt_history.vbe_prev[idx];
                bjt_history.vbe_prev[idx] = vbe;
                bjt_history.ibe_prev[idx] = 0.0;
            }

            if cbc.is_finite() && cbc > 0.0 {
                let geq = coeff_update.capacitor_geq(cbc, dt);
                let ieq = coeff_update.capacitor_ieq(
                    cbc,
                    dt,
                    bjt_history.vbc_prev[idx],
                    bjt_history.vbc_prev_prev[idx],
                    bjt_history.ibc_prev[idx],
                );
                let i_new = geq * vbc - ieq;
                let v_old = bjt_history.vbc_prev[idx];
                bjt_history.vbc_prev_prev[idx] = v_old;
                bjt_history.vbc_prev[idx] = vbc;
                bjt_history.ibc_prev[idx] = i_new;
            } else {
                bjt_history.vbc_prev_prev[idx] = bjt_history.vbc_prev[idx];
                bjt_history.vbc_prev[idx] = vbc;
                bjt_history.ibc_prev[idx] = 0.0;
            }

            let ccs = bjt.cjcp;
            if ccs.is_finite()
                && ccs > 0.0
                && bjt.node_collector != bjt.node_substrate
            {
                let geq = coeff_update.capacitor_geq(ccs, dt);
                let ieq = coeff_update.capacitor_ieq(
                    ccs,
                    dt,
                    bjt_history.vcs_prev[idx],
                    bjt_history.vcs_prev_prev[idx],
                    bjt_history.ics_prev[idx],
                );
                let i_new = geq * vcs - ieq;
                let v_old = bjt_history.vcs_prev[idx];
                bjt_history.vcs_prev_prev[idx] = v_old;
                bjt_history.vcs_prev[idx] = vcs;
                bjt_history.ics_prev[idx] = i_new;
            } else {
                bjt_history.vcs_prev_prev[idx] = bjt_history.vcs_prev[idx];
                bjt_history.vcs_prev[idx] = vcs;
                bjt_history.ics_prev[idx] = 0.0;
            }
        }

        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, accepted_solution);
            let (vgs_charge, vgd_charge) =
                Self::jfet_charge_branch_voltages(jfet, accepted_solution);
            let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);
            let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = Self::jfet_companion_terms(
                method,
                trap_order,
                dt,
                cgs,
                vgs_charge,
                jfet_history.vgs_prev[idx],
                jfet_history.qgs_prev[idx],
                jfet_history.qgs_prev_prev[idx],
                jfet_history.cqgs_prev[idx],
            );
            jfet_history.vgs_prev_prev[idx] = jfet_history.vgs_prev[idx];
            jfet_history.vgs_prev[idx] = vgs_charge;
            jfet_history.qgs_prev_prev[idx] = jfet_history.qgs_prev[idx];
            jfet_history.qgs_prev[idx] = qgs_curr;
            jfet_history.cqgs_prev[idx] = cqgs_curr;

            let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = Self::jfet_companion_terms(
                method,
                trap_order,
                dt,
                cgd,
                vgd_charge,
                jfet_history.vgd_prev[idx],
                jfet_history.qgd_prev[idx],
                jfet_history.qgd_prev_prev[idx],
                jfet_history.cqgd_prev[idx],
            );
            jfet_history.vgd_prev_prev[idx] = jfet_history.vgd_prev[idx];
            jfet_history.vgd_prev[idx] = vgd_charge;
            jfet_history.qgd_prev_prev[idx] = jfet_history.qgd_prev[idx];
            jfet_history.qgd_prev[idx] = qgd_curr;
            jfet_history.cqgd_prev[idx] = cqgd_curr;
        }

        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            if mos.level == 6 {
                let vg = Self::node_voltage(accepted_solution, mos.node_gate);
                let vd = Self::node_voltage(accepted_solution, mos.node_drain);
                let vs = Self::node_voltage(accepted_solution, mos.node_source);
                let vb = Self::node_voltage(accepted_solution, mos.node_bulk);
                mosfet_history.vgs_prev_prev[idx] = mosfet_history.vgs_prev[idx];
                mosfet_history.vgs_prev[idx] = vg - vs;
                mosfet_history.vgd_prev_prev[idx] = mosfet_history.vgd_prev[idx];
                mosfet_history.vgd_prev[idx] = vg - vd;
                mosfet_history.vgb_prev_prev[idx] = mosfet_history.vgb_prev[idx];
                mosfet_history.vgb_prev[idx] = vg - vb;
                mosfet_history.igs_prev[idx] = 0.0;
                mosfet_history.igd_prev[idx] = 0.0;
                mosfet_history.igb_prev[idx] = 0.0;
                continue;
            }
            let vg = Self::node_voltage(accepted_solution, mos.node_gate);
            let vd = Self::node_voltage(accepted_solution, mos.node_drain);
            let vs = Self::node_voltage(accepted_solution, mos.node_source);
            let vb = Self::node_voltage(accepted_solution, mos.node_bulk);
            let vgs = vg - vs;
            let vgd = vg - vd;
            let vgb = vg - vb;
            let (cgs, cgd, cgb) = mos.overlap_capacitances();

            if cgs.is_finite() && cgs > 0.0 {
                let geq = coeff_update.capacitor_geq(cgs, dt);
                let ieq = coeff_update.capacitor_ieq(
                    cgs,
                    dt,
                    mosfet_history.vgs_prev[idx],
                    mosfet_history.vgs_prev_prev[idx],
                    mosfet_history.igs_prev[idx],
                );
                let i_new = geq * vgs - ieq;
                let v_old = mosfet_history.vgs_prev[idx];
                mosfet_history.vgs_prev_prev[idx] = v_old;
                mosfet_history.vgs_prev[idx] = vgs;
                mosfet_history.igs_prev[idx] = i_new;
            } else {
                mosfet_history.vgs_prev_prev[idx] = mosfet_history.vgs_prev[idx];
                mosfet_history.vgs_prev[idx] = vgs;
                mosfet_history.igs_prev[idx] = 0.0;
            }

            if cgd.is_finite() && cgd > 0.0 {
                let geq = coeff_update.capacitor_geq(cgd, dt);
                let ieq = coeff_update.capacitor_ieq(
                    cgd,
                    dt,
                    mosfet_history.vgd_prev[idx],
                    mosfet_history.vgd_prev_prev[idx],
                    mosfet_history.igd_prev[idx],
                );
                let i_new = geq * vgd - ieq;
                let v_old = mosfet_history.vgd_prev[idx];
                mosfet_history.vgd_prev_prev[idx] = v_old;
                mosfet_history.vgd_prev[idx] = vgd;
                mosfet_history.igd_prev[idx] = i_new;
            } else {
                mosfet_history.vgd_prev_prev[idx] = mosfet_history.vgd_prev[idx];
                mosfet_history.vgd_prev[idx] = vgd;
                mosfet_history.igd_prev[idx] = 0.0;
            }

            if cgb.is_finite() && cgb > 0.0 {
                let geq = coeff_update.capacitor_geq(cgb, dt);
                let ieq = coeff_update.capacitor_ieq(
                    cgb,
                    dt,
                    mosfet_history.vgb_prev[idx],
                    mosfet_history.vgb_prev_prev[idx],
                    mosfet_history.igb_prev[idx],
                );
                let i_new = geq * vgb - ieq;
                let v_old = mosfet_history.vgb_prev[idx];
                mosfet_history.vgb_prev_prev[idx] = v_old;
                mosfet_history.vgb_prev[idx] = vgb;
                mosfet_history.igb_prev[idx] = i_new;
            } else {
                mosfet_history.vgb_prev_prev[idx] = mosfet_history.vgb_prev[idx];
                mosfet_history.vgb_prev[idx] = vgb;
                mosfet_history.igb_prev[idx] = 0.0;
            }
        }
    }

    /// Run transient analysis with abort signal for cancellation
    ///
    /// This method supports cooperative cancellation via the `AbortSignal` trait.
    /// The abort signal is checked every 1000 iterations for minimal overhead.
    ///
    /// # Arguments
    ///
    /// * `netlist` - The circuit netlist to simulate
    /// * `tstop` - Stop time for the simulation
    /// * `max_step` - Maximum timestep size
    /// * `abort` - Abort signal for cancellation (use `&NoAbort` if not needed)
    ///
    /// # Returns
    ///
    /// Returns simulation results up to the point of abort, or an error if aborted.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rspice_core::{Engine, AtomicAbort, AbortSignal};
    /// use std::sync::Arc;
    ///
    /// let abort = Arc::new(AtomicAbort::new());
    /// let abort_clone = Arc::clone(&abort);
    ///
    /// // In another thread: abort_clone.set();
    ///
    /// let result = engine.run_tran_with_abort(&netlist, 10e-3, 1e-6, &*abort);
    /// ```
    pub fn run_tran_with_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        engine.run_tran_with_abort_resolved(netlist, tstop, max_step, abort)
    }

    fn run_tran_with_abort_resolved(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, SimulationError> {
        let mut circuit = self.build_circuit(netlist)?;
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let source_step_hint = Self::transient_source_step_hint(netlist, max_step);
        circuit
            .voltage_sources
            .set_transient_context(source_step_hint, tstop);
        circuit
            .current_sources
            .set_transient_context(source_step_hint, tstop);

        // Get DC operating point as initial condition.
        let (mut solution, initial_solution_mode) =
            self.solve_transient_initial_solution(netlist, &mut circuit, &mut matrix, abort)?;
        let applied_ic = self.apply_initial_condition_overrides(netlist, &circuit, &mut solution);
        circuit.refresh_jiles_atherton_inductances(&solution);

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();
        let requires_conservative_nonlinear_limiting = circuit.has_physical_nonlinear_devices();
        let enforce_force_candidate_safety =
            requires_conservative_nonlinear_limiting || circuit.has_xspice_devices();
        let is_strictly_linear_transient =
            !circuit.has_nonlinear_devices() && !circuit.has_xspice_devices();

        // Initialize timestep controller.
        // BJT-heavy decks (notably VBIC regression circuits) need a smaller startup
        // timestep to capture sub-ns bias settling that ngspice resolves before
        // transitioning to larger steps.
        let has_bjts = !circuit.bjts.devices.is_empty();
        let (startup_div, min_div) = Self::startup_timestep_divisors(has_bjts);
        let initial_step = (max_step / startup_div).min(tstop / 100.0);
        let practical_min = (max_step / min_div).max(1e-15);
        let mut timestep = TimestepController::new(initial_step, practical_min, max_step);
        let mut breakpoints = BreakpointManager::new();
        Self::collect_transient_source_breakpoints(
            &circuit,
            tstop,
            source_step_hint,
            &mut breakpoints,
        );
        let mut lte_estimator =
            LteEstimator::with_tolerances(self.voltage_reltol(), self.voltage_abstol());

        // Integration method selection:
        // - TrapGear => adaptive trap/gear switching
        // - Other modes => fixed method (honor SimulationConfig exactly)
        let fixed_method = match self.config.integration_method {
            IntegrationMethod::TrapGear => None,
            method => Some(method),
        };
        let mut trapgear = TrapGearController::new();
        if let Some(method) = fixed_method {
            trapgear.force_method(method);
        }

        // Track integration method order for LTE scaling
        let method_order = |method: IntegrationMethod| -> u32 {
            match method {
                IntegrationMethod::BackwardEuler => 1,
                _ => 2, // Trapezoidal and Gear2 are both order 2
            }
        };
        let current_integration_method = |tg: &TrapGearController| -> IntegrationMethod {
            fixed_method.unwrap_or_else(|| tg.current_method())
        };

        // Initialize result storage with actual node names from netlist
        let node_names = circuit.node_names_sorted();

        // Debug: log node names and their indices to verify alignment
        log::info!("Node mapping (index -> name, DC voltage):");
        for (i, name) in node_names.iter().enumerate() {
            let dc_v = solution.get(i).copied().unwrap_or(0.0);
            log::info!("  Node[{}] = '{}', V_dc = {:.4}", i, name, dc_v);
        }
        if applied_ic > 0 {
            log::info!(
                "Applied {} .IC node override(s) to transient initial state",
                applied_ic
            );
        }

        let mut result = TransientResult {
            time: vec![0.0],
            voltages: (0..num_nodes)
                .map(|i| vec![solution.get(i).copied().unwrap_or(0.0)])
                .collect(),
            num_nodes,
            node_names,
        };
        let mut t = 0.0;

        // Initialize capacitor voltage history from DC solution
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_dc = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.capacitors.v_prev[cap_idx] = v_dc;
            circuit.capacitors.v_prev_prev[cap_idx] = v_dc;
            log::info!(
                "Capacitor {} init: v_dc={:.4}, np={}, nn={}",
                circuit.capacitors.names[cap_idx],
                v_dc,
                np,
                nn
            );
        }

        // Initialize inductor current and voltage history from DC solution
        for l_idx in 0..circuit.inductors.names.len() {
            let np = circuit.inductors.node_pos[l_idx];
            let nn = circuit.inductors.node_neg[l_idx];
            let br = circuit.inductors.branch_indices[l_idx];

            // Initialize voltage across inductor from DC solution
            let v_dc = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.inductors.v_prev[l_idx] = v_dc;

            // Initialize branch currents from DC solution
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_dc = solution[br_idx];
                circuit.inductors.i_prev[l_idx] = i_dc;
                circuit.inductors.i_prev_prev[l_idx] = i_dc;
            }
        }
        let tline_dc_refs = Self::initialize_tline_history(&mut circuit, &solution, 0.0);
        let mut bjt_history = Self::initialize_bjt_history(&circuit, &solution);
        let mut jfet_history = Self::initialize_jfet_history(&circuit, &solution);
        let mut mosfet_history = Self::initialize_mosfet_history(&circuit, &solution);

        // Main transient loop
        let mut retry_count = 0;
        let mut total_iterations = 0;
        let mut stale_accept_count = 0;
        let mut force_accept_cooldown = 0_usize; // Failed retries to defer dt shrink immediately after force-accept
        let mut trap_order = 1_u8; // ngspice-style trap order: start at 1, promote to 2 after accepted smooth step
        const MAX_RETRIES: usize = 200; // Maximum retries per timepoint before force-accept
        const FORCE_ACCEPT_COOLDOWN_RETRIES: usize = 2;
        // Keep cancellation responsiveness tight for large transient decks where a
        // single accepted step can still be expensive.
        const ABORT_CHECK_INTERVAL: usize = 16;
        let estimated_steps = ((tstop / max_step).ceil().max(1.0) as usize).saturating_add(1);
        let max_total_iterations = estimated_steps.saturating_mul(40).max(10_000_000);
        let mut last_progress_log = std::time::Instant::now();
        let mut rhs = vec![0.0; size];
        let mut new_solution = solution.clone();

        while t < tstop && total_iterations < max_total_iterations {
            // Progress logging every 2 seconds
            if last_progress_log.elapsed().as_secs() >= 2 {
                log::info!(
                    "Transient progress: t={:.3e}s / {:.3e}s ({:.1}%), {} iterations",
                    t,
                    tstop,
                    (t / tstop) * 100.0,
                    total_iterations
                );
                last_progress_log = std::time::Instant::now();
            }

            // Abort check - check every ABORT_CHECK_INTERVAL iterations for minimal overhead
            if total_iterations % ABORT_CHECK_INTERVAL == 0 {
                let is_aborted = abort.is_aborted();
                if total_iterations == 0 {
                    log::debug!("First abort check, is_aborted={}", is_aborted);
                }
                if is_aborted {
                    log::info!(
                        "Transient simulation aborted at t={:.3e}s ({:.1}% complete, {} iterations)",
                        t,
                        (t / tstop) * 100.0,
                        total_iterations
                    );
                    // Return error indicating abort - partial results are lost
                    return Err(SimulationError::Aborted);
                }
            }

            total_iterations += 1;
            if breakpoints.should_use_minimal_step() {
                timestep.force_step(1e-12);
                breakpoints.clear_breakpoint_flag();
            }
            let (dt, at_breakpoint) = breakpoints.limit_step(t, timestep.dt());
            if fixed_method.is_none() {
                trapgear.set_at_breakpoint(at_breakpoint);
            } else if let Some(method) = fixed_method {
                trapgear.force_method(method);
            }
            let configured_min_dt = self.config.min_timestep.max(1e-15);
            let mut dt = dt.min(tstop - t); // Don't overshoot tstop
            let mut expected_source_delta = circuit.voltage_sources.max_expected_delta(t, t + dt);
            if !at_breakpoint {
                if expected_source_delta >= SOURCE_ACTIVE_DELTA {
                    // During steep source transitions, permit sub-minimum timesteps
                    // to track fast waveform edges accurately.
                    let active_cap = (configured_min_dt / 8.0).max(practical_min);
                    if dt > active_cap {
                        dt = active_cap;
                        expected_source_delta = circuit.voltage_sources.max_expected_delta(t, t + dt);
                    }
                } else if dt < configured_min_dt {
                    // Away from sharp source transitions, keep production-grade
                    // timestep floor for performance and stability.
                    dt = configured_min_dt.min(tstop - t);
                    expected_source_delta = circuit.voltage_sources.max_expected_delta(t, t + dt);
                }
            }
            let step_time = t + dt;
            let newton_step_delta_limit = Self::startup_step_delta_limit(
                initial_solution_mode,
                step_time,
                max_step,
                MAX_NEWTON_ITER_DELTA_V,
            );
            let force_accept_delta_limit = Self::startup_step_delta_limit(
                initial_solution_mode,
                step_time,
                max_step,
                MAX_FORCE_ACCEPT_DELTA_V,
            );
            let current_method = current_integration_method(&trapgear);
            let step_trap_order =
                Self::effective_trapezoidal_order(current_method, trap_order, at_breakpoint);
            let coeff = CompanionCoefficients::for_method(current_method);

            // Prepare for Newton iteration at this timestep
            new_solution.clone_from(&solution);
            let mut had_solver_candidate = true;

            // Newton-Raphson iteration for this timestep.
            // Transient nonlinear regions (e.g., BJT turn-on) often need more
            // iterations than DC. Use a higher budget here to reduce force-accept.
            let tran_max_iterations = (self.config.max_iterations.saturating_mul(4)).min(400);
            let mut converged = false;
            for _iter in 0..tran_max_iterations {
                matrix.clear_values();
                rhs.fill(0.0);

                // Add the configured baseline GMIN only on node-voltage equations.
                // Branch-current equations (voltage source/inductor branches) must
                // not receive this shunt or transient references are biased.
                let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
                if gmin_floor > 0.0 {
                    for i in 0..num_nodes {
                        matrix.add(i, i, gmin_floor);
                    }
                }

                // Stamp linear devices (R, V, I) for transient.
                // Tline transient behavior is stamped separately via companions.
                circuit.stamp_transient_linear_direct(&mut matrix, &mut rhs);

                // Update voltage source RHS values for time-varying sources (PULSE, SIN, etc.)
                let num_nodes = circuit.num_nodes();
                circuit.voltage_sources.update_transient_rhs(
                    &mut rhs,
                    t + dt, // Evaluate at target time point
                    |br_ordinal| num_nodes + br_ordinal,
                );
                circuit
                    .current_sources
                    .update_transient_rhs(&mut rhs, t + dt);

                circuit.refresh_jiles_atherton_inductances(&new_solution);
                if circuit.has_nonlinear_devices() {
                    circuit.update_nonlinear(&new_solution);
                }

                // Stamp capacitor companion models for transient
                circuit
                    .capacitors
                    .stamp_transient_companion(&mut matrix, &mut rhs, dt, &coeff);

                // Stamp inductor companion models for transient
                circuit.inductors.stamp_transient_companion(
                    &mut matrix,
                    &mut rhs,
                    dt,
                    &coeff,
                    num_nodes,
                );
                Self::stamp_bjt_transient_companions(
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    current_method,
                    step_trap_order,
                    dt,
                    &bjt_history,
                );
                Self::stamp_jfet_transient_companions(
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &jfet_history,
                );
                Self::stamp_mosfet_transient_companions(
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    dt,
                    &coeff,
                    &mosfet_history,
                );
                Self::stamp_tline_companions(
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    t + dt,
                    &tline_dc_refs,
                );

                // Stamp nonlinear devices if present
                if circuit.has_nonlinear_devices() {
                    circuit.stamp_nonlinear(&mut matrix, &mut rhs, &new_solution);
                    circuit.stamp_behavioral(&mut matrix, &mut rhs, &new_solution, t + dt);
                }

                // Evaluate and stamp XSPICE code models
                if circuit.has_xspice_devices() {
                    circuit.evaluate_xspice_with_timestep(t + dt, dt, &new_solution);
                    circuit.stamp_xspice(&mut matrix, &mut rhs);
                }

                // Solve and check convergence
                match matrix.solve(&rhs) {
                    Ok(mut sol) => {
                        had_solver_candidate = true;
                        // Sanity check: detect and handle NaN/Inf/excessive values.
                        // IMPORTANT: Preserve the newest valid candidate when possible.
                        // If we keep the previous timestep guess here, force-accept can
                        // propagate a stale state and flatten non-source traces.
                        let mut has_bad_values = false;
                        let mut logged_divergence = false;

                        for (i, v) in sol.iter_mut().enumerate() {
                            if !v.is_finite() {
                                if !logged_divergence {
                                    log::debug!(
                                        "Transient: Newton divergence at t={:.3e}s, node {}: {:.3e} - reducing timestep",
                                        t + dt,
                                        i,
                                        *v
                                    );
                                    logged_divergence = true;
                                }
                                // Non-finite values cannot be used; fall back to prior guess.
                                *v = new_solution[i];
                                has_bad_values = true;
                            } else if v.abs() > MAX_VOLTAGE {
                                if !logged_divergence {
                                    log::debug!(
                                        "Transient: Newton divergence at t={:.3e}s, node {}: {:.3e} - reducing timestep",
                                        t + dt,
                                        i,
                                        *v
                                    );
                                    logged_divergence = true;
                                }
                                // Soft-limit finite overflow around previous Newton guess
                                // instead of hard-clamping to +/-MAX_VOLTAGE. Hard rail
                                // clamps can be force-accepted and then contaminate dynamic
                                // history with nonphysical state.
                                let old = new_solution[i];
                                let delta = *v - old;
                                if delta.is_finite() {
                                    *v = old + delta.signum() * newton_step_delta_limit;
                                } else {
                                    *v = old;
                                }
                                has_bad_values = true;
                            }
                        }

                        if requires_conservative_nonlinear_limiting {
                            // Trust-region damping is critical for stiff semiconductor
                            // nonlinearities, but it should not throttle linear decks.
                            for i in 0..num_nodes {
                                let old = new_solution[i];
                                let delta = sol[i] - old;
                                if delta.is_finite() && delta.abs() > newton_step_delta_limit {
                                    sol[i] = old + delta.signum() * newton_step_delta_limit;
                                }
                            }
                        }

                        // If this Newton step was numerically bad, keep the sanitized
                        // candidate and continue Newton iterations.
                        if has_bad_values {
                            new_solution = sol;
                            continue;
                        }

                        if is_strictly_linear_transient {
                            // A purely linear deck does not need Newton fixed-point
                            // iterations: one direct solve per timestep is exact.
                            new_solution = sol;
                            converged = true;
                            break;
                        }

                        let voltage_converged = self.voltage_convergence_met(&new_solution, &sol);
                        let linearized_residual_converged =
                            self.residual_convergence_met(&matrix, &sol, &rhs);

                        // CRITICAL: Update new_solution BEFORE checking device convergence
                        // Otherwise, BJT vbe/vbc are based on old guess, not new solve
                        new_solution = sol;

                        // Update nonlinear device state to new solution for accurate convergence check
                        if circuit.has_nonlinear_devices() {
                            circuit.update_nonlinear(&new_solution);
                        }

                        let device_converged = !circuit.has_nonlinear_devices()
                            || circuit.nonlinear_converged(self.device_convergence_tolerance());

                        if voltage_converged && device_converged && linearized_residual_converged {
                            converged = true;
                            break;
                        }
                    }
                    Err(e) => {
                        log::debug!(
                            "Transient solve failed at t={:.6e}, dt={:.3e}: {}",
                            t + dt,
                            dt,
                            e
                        );
                        break;
                    }
                }
            }

            if !converged {
                retry_count += 1;
                trap_order = 1;

                // Diagnostic logging for debugging convergence issues
                static CONV_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                let count = CONV_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if count < 10 {
                    // Check what specifically didn't converge
                    let v_conv = self.voltage_convergence_met(&solution, &new_solution);
                    let d_conv = !circuit.has_nonlinear_devices()
                        || circuit.nonlinear_converged(self.device_convergence_tolerance());
                    let r_conv = self.residual_convergence_met(&matrix, &new_solution, &rhs);
                    let max_dv = Self::max_abs_delta_prefix(&solution, &new_solution, num_nodes);
                    log::warn!(
                        "Newton non-converge at t={:.6e}, dt={:.3e}: voltage_conv={}, device_conv={}, residual_conv={}, max_dv={:.3e}, iter={}",
                        t,
                        dt,
                        v_conv,
                        d_conv,
                        r_conv,
                        max_dv,
                        total_iterations
                    );
                }

                // Diagnostic logging for debugging timestep issues
                if total_iterations < 100 || total_iterations % 10000 == 0 {
                    log::debug!(
                        "Newton non-convergence at t={:.3e}s, iter={}, dt={:.3e}s, reducing to {:.3e}s",
                        t,
                        total_iterations,
                        dt,
                        dt * 0.25
                    );
                }

                // Convergence failed - reduce timestep significantly (4x) and retry
                // BUT skip reduction if we're in post-force-accept cooldown to avoid ping-pong
                if force_accept_cooldown > 0 {
                    force_accept_cooldown -= 1;
                    // During cooldown, keep timestep at current level (don't shrink)
                } else {
                    timestep.force_step(dt * 0.25);
                }

                // Force accept when recovery is unlikely:
                // - After MAX_RETRIES attempts (regardless of timestep state), OR
                // - At minimum timestep AND at least MIN_RETRIES_AT_MIN have been tried
                // This prevents both infinite loops and force-accept floods
                const MIN_RETRIES_AT_MIN: usize = 3; // Give a few attempts at min dt
                let at_min_dt = timestep.is_at_minimum();
                let exhausted_retries = retry_count >= MAX_RETRIES;
                let exhausted_at_min = at_min_dt && retry_count >= MIN_RETRIES_AT_MIN;

                if exhausted_retries || exhausted_at_min {
                    let unbounded_force_candidate = Self::is_unbounded_step(
                        &solution,
                        &new_solution,
                        expected_source_delta,
                        num_nodes,
                    );
                    let stale_force_candidate = Self::is_stale_step(
                        &solution,
                        &new_solution,
                        expected_source_delta,
                        num_nodes,
                    );

                    if enforce_force_candidate_safety
                        && (unbounded_force_candidate
                            || !had_solver_candidate
                            || stale_force_candidate)
                    {
                        stale_accept_count += 1;
                        let boosted = (dt * 4.0).min(max_step);
                        if boosted > dt {
                            timestep.force_step(boosted);
                        }
                        if stale_accept_count >= 8 {
                            if unbounded_force_candidate {
                                log::error!(
                                    "Transient diverged at t={:.6e}s: repeated unbounded force-accept candidates",
                                    t
                                );
                            } else {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: stale force-accept candidates with active sources",
                                    t
                                );
                            }
                            return Err(SimulationError::ConvergenceFailed(total_iterations));
                        }
                        continue;
                    }
                    let clipped_force_candidate = Self::is_clipped_force_candidate(
                        &solution,
                        &new_solution,
                        num_nodes,
                        force_accept_delta_limit,
                    );
                    if clipped_force_candidate {
                        if fixed_method.is_none() {
                            trapgear.force_method(IntegrationMethod::Gear2);
                        }
                        timestep.force_step((dt * 0.5).min(max_step));
                    }
                    stale_accept_count = 0;

                    t += dt;
                    let hit_breakpoint = at_breakpoint || breakpoints.at_breakpoint(t);
                    if hit_breakpoint {
                        let restart_dt = breakpoints.mark_breakpoint_solved(t);
                        timestep.force_step(restart_dt.min(max_step));
                    }

                    // FORCE-ACCEPT: Use the unconverged Newton result as-is.
                    // While not fully converged, the Newton result is still a valid
                    // approximation that respects circuit equations (just with residual error).
                    // This is the standard SPICE approach for force-accept.
                    // Voltage sources are enforced to ensure correct input values.
                    circuit
                        .voltage_sources
                        .enforce_voltage_constraints(&mut new_solution, t);

                    // Keep forced movement tightly bounded to prevent long-run drift
                    // when a region requires repeated force-accepts.
                    for i in 0..num_nodes {
                        let old = solution[i];
                        let delta = new_solution[i] - old;
                        if delta.is_finite() && delta.abs() > force_accept_delta_limit {
                            new_solution[i] = old + delta.signum() * force_accept_delta_limit;
                        }
                    }
                    circuit
                        .voltage_sources
                        .enforce_voltage_constraints(&mut new_solution, t);

                    if circuit.has_nonlinear_devices() {
                        circuit.update_nonlinear(&new_solution);
                    }

                    let method_after_step = current_integration_method(&trapgear);
                    lte_estimator.record(&new_solution, dt);
                    lte_estimator.set_method_order(method_order(method_after_step));
                    if fixed_method.is_none() {
                        trapgear.update(&new_solution, dt);
                    }
                    Self::update_reactive_history(
                        &mut circuit,
                        &new_solution,
                        t,
                        dt,
                        current_method,
                        step_trap_order,
                        &mut bjt_history,
                        &mut jfet_history,
                        &mut mosfet_history,
                        &tline_dc_refs,
                    );
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_timestep();
                    }

                    solution.clone_from(&new_solution);
                    result.time.push(t);
                    for (i, voltages) in result.voltages.iter_mut().enumerate() {
                        voltages.push(solution.get(i).copied().unwrap_or(0.0));
                    }

                    // Debug: log force-accept values for node 0
                    let v0_force = solution.get(0).copied().unwrap_or(0.0);
                    static FORCE_LOG_COUNT: std::sync::atomic::AtomicUsize =
                        std::sync::atomic::AtomicUsize::new(0);
                    let count = FORCE_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if count < 20 {
                        log::warn!(
                            "FORCE-ACCEPT at t={:.6e}: v0={:.4}, retry_count={}",
                            t,
                            v0_force,
                            retry_count
                        );
                    }

                    retry_count = 0; // Reset for next timepoint
                    // Keep the accepted dt and only defer shrink for a couple of retries.
                    // Large cooldowns plus immediate dt growth can trap stiff switching decks
                    // in repeated force-accept loops instead of letting the controller retreat.
                    force_accept_cooldown = FORCE_ACCEPT_COOLDOWN_RETRIES;
                    Self::recover_timestep_after_accepted_step(
                        &mut timestep,
                        &lte_estimator,
                        &solution,
                        dt,
                        max_step,
                        is_strictly_linear_transient,
                        expected_source_delta,
                    );
                    if matches!(
                        current_method,
                        IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                    ) {
                        trap_order = if hit_breakpoint { 1 } else { 2 };
                    }
                }
                continue;
            }

            // Check LTE for physics accuracy
            let (lte, accept) = if is_strictly_linear_transient {
                (0.0, true)
            } else {
                lte_estimator.estimate(&new_solution, dt)
            };
            if !accept {
                retry_count += 1;
                trap_order = 1;
                let scale = lte_estimator.recommend_scale(lte);
                timestep.adjust(lte / scale);

                // Force accept when recovery is unlikely:
                // - After MAX_RETRIES attempts (regardless of timestep state), OR
                // - At minimum timestep AND at least MIN_RETRIES_AT_MIN have been tried
                // This prevents both infinite loops and force-accept floods
                const MIN_RETRIES_AT_MIN: usize = 3;
                let at_min_dt = timestep.is_at_minimum();
                let exhausted_retries = retry_count >= MAX_RETRIES;
                let exhausted_at_min = at_min_dt && retry_count >= MIN_RETRIES_AT_MIN;

                if exhausted_retries || exhausted_at_min {
                    let unbounded_force_candidate = Self::is_unbounded_step(
                        &solution,
                        &new_solution,
                        expected_source_delta,
                        num_nodes,
                    );
                    let stale_force_candidate = Self::is_stale_step(
                        &solution,
                        &new_solution,
                        expected_source_delta,
                        num_nodes,
                    );

                    if enforce_force_candidate_safety
                        && (unbounded_force_candidate
                            || !had_solver_candidate
                            || stale_force_candidate)
                    {
                        stale_accept_count += 1;
                        let boosted = (dt * 4.0).min(max_step);
                        if boosted > dt {
                            timestep.force_step(boosted);
                        }
                        if stale_accept_count >= 8 {
                            if unbounded_force_candidate {
                                log::error!(
                                    "Transient diverged at t={:.6e}s: repeated unbounded LTE force-accept candidates",
                                    t
                                );
                            } else {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: stale LTE force-accept candidates with active sources",
                                    t
                                );
                            }
                            return Err(SimulationError::ConvergenceFailed(total_iterations));
                        }
                        continue;
                    }
                    let clipped_force_candidate = Self::is_clipped_force_candidate(
                        &solution,
                        &new_solution,
                        num_nodes,
                        force_accept_delta_limit,
                    );
                    if clipped_force_candidate {
                        if fixed_method.is_none() {
                            trapgear.force_method(IntegrationMethod::Gear2);
                        }
                        timestep.force_step((dt * 0.5).min(max_step));
                    }
                    stale_accept_count = 0;

                    t += dt;
                    let hit_breakpoint = at_breakpoint || breakpoints.at_breakpoint(t);
                    if hit_breakpoint {
                        let restart_dt = breakpoints.mark_breakpoint_solved(t);
                        timestep.force_step(restart_dt.min(max_step));
                    }
                    circuit
                        .voltage_sources
                        .enforce_voltage_constraints(&mut new_solution, t);

                    for i in 0..num_nodes {
                        let old = solution[i];
                        let delta = new_solution[i] - old;
                        if delta.is_finite() && delta.abs() > force_accept_delta_limit {
                            new_solution[i] = old + delta.signum() * force_accept_delta_limit;
                        }
                    }
                    circuit
                        .voltage_sources
                        .enforce_voltage_constraints(&mut new_solution, t);

                    if circuit.has_nonlinear_devices() {
                        circuit.update_nonlinear(&new_solution);
                    }

                    let method_after_step = current_integration_method(&trapgear);
                    lte_estimator.record(&new_solution, dt);
                    lte_estimator.set_method_order(method_order(method_after_step));
                    if fixed_method.is_none() {
                        trapgear.update(&new_solution, dt);
                    }
                    Self::update_reactive_history(
                        &mut circuit,
                        &new_solution,
                        t,
                        dt,
                        current_method,
                        step_trap_order,
                        &mut bjt_history,
                        &mut jfet_history,
                        &mut mosfet_history,
                        &tline_dc_refs,
                    );
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_timestep();
                    }

                    solution.clone_from(&new_solution);
                    result.time.push(t);
                    for (i, voltages) in result.voltages.iter_mut().enumerate() {
                        voltages.push(solution.get(i).copied().unwrap_or(0.0));
                    }
                    retry_count = 0; // Reset for next timepoint
                    force_accept_cooldown = FORCE_ACCEPT_COOLDOWN_RETRIES;
                    Self::recover_timestep_after_accepted_step(
                        &mut timestep,
                        &lte_estimator,
                        &solution,
                        dt,
                        max_step,
                        is_strictly_linear_transient,
                        expected_source_delta,
                    );
                    if matches!(
                        current_method,
                        IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                    ) {
                        trap_order = if hit_breakpoint { 1 } else { 2 };
                    }
                }
                continue;
            }

            // Success - reset retry counter
            retry_count = 0;

            // Keep ideal source constraints exact before LTE and state updates.
            circuit
                .voltage_sources
                .enforce_voltage_constraints(&mut new_solution, t + dt);

            if Self::is_stale_step(&solution, &new_solution, expected_source_delta, num_nodes) {
                stale_accept_count += 1;
                let boosted = (dt * 2.0).min(max_step);
                if boosted > dt {
                    timestep.force_step(boosted);
                }
                if stale_accept_count >= 8 {
                    log::error!(
                        "Transient stalled near t={:.6e}s: repeated stale accepted steps with active sources",
                        t
                    );
                    return Err(SimulationError::ConvergenceFailed(total_iterations));
                }
                trap_order = 1;
                continue;
            }
            stale_accept_count = 0;

            // Accept this timestep
            t += dt;
            let hit_breakpoint = at_breakpoint || breakpoints.at_breakpoint(t);
            let method_after_step = current_integration_method(&trapgear);
            lte_estimator.record(&new_solution, dt);
            lte_estimator.set_method_order(method_order(method_after_step));
            if fixed_method.is_none() {
                trapgear.update(&new_solution, dt);
            }

            if circuit.has_nonlinear_devices() {
                circuit.update_nonlinear(&new_solution);
            }

            // CRITICAL: Grow timestep back after successful convergence
            // This prevents staying stuck at minimum timestep after force-accepts
            // LTE estimator recommends scale based on error - apply it to timestep
            let (lte, _) = if is_strictly_linear_transient {
                (0.0, true)
            } else {
                lte_estimator.estimate(&new_solution, dt)
            };
            let scale = if is_strictly_linear_transient {
                1.0
            } else {
                lte_estimator.recommend_scale(lte)
            };
            if scale > 1.0 {
                // Grow timestep gradually (limit to 1.5x per successful step)
                let new_dt = (dt * scale.min(1.5)).min(max_step);
                timestep.force_step(new_dt);
            }
            Self::update_reactive_history(
                &mut circuit,
                &new_solution,
                t,
                dt,
                current_method,
                step_trap_order,
                &mut bjt_history,
                &mut jfet_history,
                &mut mosfet_history,
                &tline_dc_refs,
            );

            // Accept XSPICE timestep (commit state changes)
            if circuit.has_xspice_devices() {
                circuit.accept_xspice_timestep();
            }

            solution.clone_from(&new_solution);

            // Store results
            result.time.push(t);
            for (i, voltages) in result.voltages.iter_mut().enumerate() {
                voltages.push(solution.get(i).copied().unwrap_or(0.0));
            }
            let scale = lte_estimator.recommend_scale(lte);
            timestep.adjust(lte / scale);
            if hit_breakpoint {
                let restart_dt = breakpoints.mark_breakpoint_solved(t);
                timestep.force_step(restart_dt.min(max_step));
            }
            if matches!(
                current_method,
                IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
            ) {
                trap_order = if hit_breakpoint { 1 } else { 2 };
            }
        }

        if t < tstop {
            log::error!(
                "Transient terminated early at t={:.6e}s / {:.6e}s after {} iterations",
                t,
                tstop,
                total_iterations
            );
            return Err(SimulationError::ConvergenceFailed(total_iterations));
        }

        log::info!(
            "Transient complete: {} time points computed",
            result.time.len()
        );

        // Debug: verify stored voltage range for node 0 (SIN source)
        if let Some(node0_voltages) = result.voltages.get(0) {
            let v_min = node0_voltages.iter().cloned().fold(f64::INFINITY, f64::min);
            let v_max = node0_voltages
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max);
            log::info!(
                "Stored voltages for node0 (SIN source): {} points, y_min={:.4}, y_max={:.4}",
                node0_voltages.len(),
                v_min,
                v_max
            );
        }

        Ok(result)
    }

    /// Run transient analysis with waveform compression
    ///
    /// Uses the `WaveformRecorder` to achieve 10-100x memory reduction for long
    /// simulations. The compression uses linear interpolation-based point decimation
    /// that preserves all significant signal transitions.
    pub fn run_tran_compressed(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        compression: CompressionConfig,
    ) -> Result<TransientResultCompressed, SimulationError> {
        // Reuse the robust transient solver path, then apply waveform compression
        // during result marshaling. This keeps compressed and uncompressed physics
        // behavior identical, avoiding divergence between solver implementations.
        let result = self.run_tran(netlist, tstop, max_step)?;

        if result.time.is_empty() {
            return Ok(TransientResultCompressed {
                time: Vec::new(),
                voltages: vec![Vec::new(); result.num_nodes],
                num_nodes: result.num_nodes,
                compression_ratio: 1.0,
                input_points: 0,
            });
        }

        let initial_values: Vec<Value> = result
            .voltages
            .iter()
            .map(|wave| wave.first().copied().unwrap_or(0.0))
            .collect();
        let mut recorder = WaveformRecorder::new(
            result.num_nodes,
            result.time[0],
            &initial_values,
            compression,
        );

        for point_idx in 1..result.time.len() {
            let values: Vec<Value> = result
                .voltages
                .iter()
                .map(|wave| wave.get(point_idx).copied().unwrap_or(0.0))
                .collect();
            recorder.record(result.time[point_idx], &values);
        }

        let final_values: Vec<Value> = result
            .voltages
            .iter()
            .map(|wave| wave.last().copied().unwrap_or(0.0))
            .collect();
        recorder.finalize(*result.time.last().unwrap_or(&tstop), &final_values);

        Ok(recorder.to_transient_result())
    }
}

#[cfg(test)]
mod abort_tests {
    use super::*;
    use crate::Engine;
    use crate::abort_signal::{CountingAbort, ImmediateAbort, NoAbort};

    #[test]
    fn test_jfet_companion_terms_backward_euler_matches_expected_linear_capacitor_behavior() {
        let c = 2e-12;
        let dt = 1e-9;
        let v_prev = 0.4;
        let v_curr = 0.5;
        let q_prev = c * v_prev;

        let (geq, ieq, q_curr, cq_curr) = Engine::jfet_companion_terms(
            IntegrationMethod::BackwardEuler,
            1,
            dt,
            c,
            v_curr,
            v_prev,
            q_prev,
            q_prev,
            0.0,
        );

        let i_companion = geq * v_curr - ieq;
        let i_expected = c * (v_curr - v_prev) / dt;
        assert!((q_curr - c * v_curr).abs() < 1e-24);
        assert!((cq_curr - i_expected).abs() < 1e-18);
        assert!((i_companion - i_expected).abs() < 1e-18);
    }

    #[test]
    fn test_jfet_companion_terms_trapezoidal_second_order_uses_previous_charge_current() {
        let c = 1.5e-12;
        let dt = 2e-9;
        let v_prev = 0.8;
        let v_curr = 0.7;
        let q_prev = c * v_prev;
        let q_prev_prev = c * 0.9;
        let cq_prev = (q_prev - q_prev_prev) / dt;

        let (geq, ieq, q_curr, cq_curr) = Engine::jfet_companion_terms(
            IntegrationMethod::Trapezoidal,
            2,
            dt,
            c,
            v_curr,
            v_prev,
            q_prev,
            q_prev_prev,
            cq_prev,
        );

        let q_expected = q_prev + c * (v_curr - v_prev);
        let cq_expected = -cq_prev + 2.0 * (q_expected - q_prev) / dt;
        let i_companion = geq * v_curr - ieq;

        assert!((q_curr - q_expected).abs() < 1e-24);
        assert!((cq_curr - cq_expected).abs() < 1e-18);
        assert!((i_companion - cq_expected).abs() < 1e-18);
    }

    #[test]
    fn test_jfet_companion_terms_does_not_inject_artificial_charge_when_capacitance_changes() {
        let dt = 1e-9;
        let c_prev = 1e-12;
        let c_curr = 2e-12;
        let v_prev = 0.6;
        let v_curr = 0.6;
        let q_prev = c_prev * v_prev;

        let (geq, ieq, q_curr, cq_curr) = Engine::jfet_companion_terms(
            IntegrationMethod::BackwardEuler,
            1,
            dt,
            c_curr,
            v_curr,
            v_prev,
            q_prev,
            q_prev,
            0.0,
        );

        let i_companion = geq * v_curr - ieq;
        let i_expected = 0.0;
        assert!((q_curr - q_prev).abs() < 1e-24);
        assert!(cq_curr.abs() < 1e-18);
        assert!((i_companion - i_expected).abs() < 1e-18);
    }

    #[test]
    fn test_jfet_charge_branch_voltages_mesa_uses_limited_internal_state() {
        use crate::device::NonlinearDevice;

        let mut z = crate::device::Jfet::njf("Z1", 1, 2, 3).enable_mesa_model();
        // Seed finite previous state so pnjlim/fetlim operate in iterative mode.
        z.update(&[0.0, 0.0, 0.0]);
        let voltages = [0.0, 3.0, 0.0];
        z.update(&voltages);

        let (vgs_charge, vgd_charge) = Engine::jfet_charge_branch_voltages(&z, &voltages);
        let (vgs_internal, vgd_internal, _) = z
            .internal_branch_state_voltages()
            .expect("expected finite internal branch state");
        let vgs_raw = voltages[1] - voltages[2];
        let vgd_raw = voltages[1] - voltages[0];

        // MESA path should follow limiter-clipped internal branch voltages.
        assert!((vgs_charge - vgs_internal).abs() < 1e-15);
        assert!((vgd_charge - vgd_internal).abs() < 1e-15);
        assert!(
            (vgs_charge - vgs_raw).abs() > 1e-6 || (vgd_charge - vgd_raw).abs() > 1e-6,
            "expected limited branch voltages to differ from raw branch voltages"
        );
    }

    #[test]
    fn test_jfet_charge_branch_voltages_hfet_level5_uses_internal_branch_state_when_available() {
        use crate::device::NonlinearDevice;

        let mut z = crate::device::Jfet::njf("Z1", 1, 2, 3).enable_hfet_model();
        z.update(&[0.0, 0.0, 0.0]);
        let voltages = [0.0, 3.0, 0.0];
        z.update(&voltages);

        let (vgs_charge, vgd_charge) = Engine::jfet_charge_branch_voltages(&z, &voltages);
        let (vgs_internal, vgd_internal, _) = z
            .internal_branch_state_voltages()
            .expect("expected finite internal branch state");
        let vgs_raw = voltages[1] - voltages[2];
        let vgd_raw = voltages[1] - voltages[0];

        // HFET level-5 charge history should follow the same limited branch
        // state used by model internals when that state is available.
        assert!((vgs_charge - vgs_internal).abs() < 1e-15);
        assert!((vgd_charge - vgd_internal).abs() < 1e-15);
        assert!(
            (vgs_charge - vgs_raw).abs() > 1e-6 || (vgd_charge - vgd_raw).abs() > 1e-6,
            "expected limited branch voltages to differ from raw branch voltages"
        );
    }

    #[test]
    fn test_jfet_charge_branch_voltages_falls_back_to_raw_terminals_without_internal_state() {
        let z = crate::device::Jfet::njf("Z1", 1, 2, 3);
        let voltages = [0.2, 1.1, -0.4];

        let (vgs_charge, vgd_charge) = Engine::jfet_charge_branch_voltages(&z, &voltages);
        let vgs_raw = voltages[1] - voltages[2];
        let vgd_raw = voltages[1] - voltages[0];

        assert!((vgs_charge - vgs_raw).abs() < 1e-15);
        assert!((vgd_charge - vgd_raw).abs() < 1e-15);
    }

    #[test]
    fn test_jfet_branch_voltage_helpers_shichman_use_raw_terminals_even_with_internal_state() {
        use crate::device::NonlinearDevice;

        let mut z = crate::device::Jfet::njf("Z1", 1, 2, 3);
        let stale_voltages = [0.0, 1.0, 0.0];
        z.update(&stale_voltages);

        let query_voltages = [0.4, -0.2, -0.6];
        let vgs_raw = query_voltages[1] - query_voltages[2];
        let vgd_raw = query_voltages[1] - query_voltages[0];
        let (vgs_internal, vgd_internal, _) = z
            .internal_branch_state_voltages()
            .expect("expected finite internal branch state");
        assert!(
            (vgs_internal - vgs_raw).abs() > 1e-6 || (vgd_internal - vgd_raw).abs() > 1e-6,
            "test setup requires stale internal branch state"
        );

        let (vgs_eval, vgd_eval) = Engine::jfet_branch_voltages(&z, &query_voltages);
        let (vgs_charge, vgd_charge) = Engine::jfet_charge_branch_voltages(&z, &query_voltages);

        assert!((vgs_eval - vgs_raw).abs() < 1e-15);
        assert!((vgd_eval - vgd_raw).abs() < 1e-15);
        assert!((vgs_charge - vgs_raw).abs() < 1e-15);
        assert!((vgd_charge - vgd_raw).abs() < 1e-15);
    }

    fn simple_rc_netlist() -> Netlist {
        // Simple RC circuit: V1 1 0 1V, R1 1 2 1k, C1 2 0 1u
        Netlist::parse(
            "Simple RC Circuit\n\
             V1 1 0 DC 1\n\
             R1 1 2 1k\n\
             C1 2 0 1u\n\
             .end",
        )
        .expect("Failed to parse netlist")
    }

    fn simple_bjt_amp_netlist() -> Netlist {
        Netlist::parse(
            "Simple BJT amplifier regression\n\
             V1 net3 0 SIN(0 2 1k 0 0 0)\n\
             CC1 net3 net5 1u\n\
             RR1 net5 0 1k\n\
             RR2 net4 net5 1k\n\
             RR3 net1 0 2.2k\n\
             RR4 net7 net1 1k\n\
             CC2 net7 0 10u\n\
             RR5 net4 net6 1k\n\
             V2 net4 0 DC 9\n\
             Q2 net6 net5 net1 npn_Q2\n\
             .MODEL npn_Q2 NPN (BF=100 IS=1e-15)\n\
             .end",
        )
        .expect("Failed to parse BJT amp netlist")
    }

    #[test]
    fn test_transient_no_abort_completes() {
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // Should complete successfully with NoAbort
        let result = engine.run_tran_with_abort(&netlist, 1e-3, 1e-5, &NoAbort);
        assert!(result.is_ok(), "Transient should complete without abort");

        let result = result.unwrap();
        assert!(result.time.len() > 1, "Should have multiple time points");
        assert!(result.time.last().copied().unwrap_or(0.0) >= 1e-3 * 0.99);
    }

    #[test]
    fn test_transient_immediate_abort_returns_error() {
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // ImmediateAbort should cause immediate termination
        let result = engine.run_tran_with_abort(&netlist, 1e-3, 1e-5, &ImmediateAbort);

        assert!(result.is_err(), "Should return error when aborted");
        match result {
            Err(SimulationError::Aborted) => {
                // Expected - simulation was aborted
            }
            Err(other) => panic!("Expected Aborted error, got: {:?}", other),
            Ok(_) => panic!("Expected error, got success"),
        }
    }

    #[test]
    fn test_transient_counting_abort_stops_at_threshold() {
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // CountingAbort(5) will return true after 5 checks
        // Since we check every 1000 iterations, this tests the abort path
        let abort = CountingAbort::new(5);
        let result = engine.run_tran_with_abort(&netlist, 10e-3, 1e-7, &abort);

        // With a long simulation and small timestep, we should hit abort
        if abort.count() >= 5 {
            // If we checked 5 times, we should have aborted
            assert!(result.is_err(), "Should abort after threshold checks");
        } else {
            // If simulation finished before 5 checks, that's OK
            assert!(result.is_ok(), "Simulation finished before abort threshold");
        }
    }

    #[test]
    fn test_run_tran_delegates_to_run_tran_with_abort() {
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // Regular run_tran should work the same as run_tran_with_abort(&NoAbort)
        let result1 = engine.run_tran(&netlist, 1e-4, 1e-6);
        let result2 = engine.run_tran_with_abort(&netlist, 1e-4, 1e-6, &NoAbort);

        assert!(result1.is_ok() && result2.is_ok());
        let r1 = result1.unwrap();
        let r2 = result2.unwrap();

        // Results should be identical
        assert_eq!(r1.time.len(), r2.time.len());
        assert_eq!(r1.num_nodes, r2.num_nodes);
    }

    #[test]
    fn test_abort_signal_checked_periodically() {
        // Verify that abort is not checked every single iteration
        // by using a counting abort and verifying the check count
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // Use a very high threshold so we don't actually abort
        let abort = CountingAbort::new(1_000_000);
        let _result = engine.run_tran_with_abort(&netlist, 1e-4, 1e-6, &abort);

        // The number of checks should be much less than total iterations
        // (we check every 1000 iterations)
        let check_count = abort.count();
        assert!(
            check_count > 0,
            "Should have checked abort at least once: {}",
            check_count
        );
        // For a short simulation, we shouldn't have too many checks
        assert!(
            check_count < 1000,
            "Check count {} seems too high for short simulation",
            check_count
        );
    }

    #[test]
    fn test_is_stale_step_false_when_source_static() {
        let prev = vec![1.0, 2.0, 3.0];
        let next = vec![1.0, 2.0, 3.0];
        assert!(!Engine::is_stale_step(&prev, &next, 0.0, prev.len()));
    }

    #[test]
    fn test_is_stale_step_true_when_solution_does_not_follow_source() {
        let prev = vec![0.5, 1.0, -2.0];
        let next = prev.clone();
        assert!(Engine::is_stale_step(&prev, &next, 1e-3, prev.len()));
    }

    #[test]
    fn test_is_stale_step_false_when_solution_moves_with_source() {
        let prev = vec![0.5, 1.0, -2.0];
        let next = vec![0.5002, 1.0001, -1.9999];
        assert!(!Engine::is_stale_step(&prev, &next, 1e-3, prev.len()));
    }

    #[test]
    fn test_is_unbounded_step_detects_runaway() {
        let prev = vec![1.0, 2.0];
        let next = vec![500.0, -400.0];
        assert!(Engine::is_unbounded_step(&prev, &next, 1e-4, prev.len()));
    }

    #[test]
    fn test_is_unbounded_step_false_for_reasonable_change() {
        let prev = vec![1.0, 2.0];
        let next = vec![1.05, 2.08];
        assert!(!Engine::is_unbounded_step(&prev, &next, 1e-3, prev.len()));
    }

    #[test]
    fn test_is_clipped_force_candidate_detects_any_clipping() {
        let prev = vec![0.0; 6];
        let next = vec![0.5, -0.5, 0.5, -0.5, 0.0, 0.0];
        assert!(Engine::is_clipped_force_candidate(
            &prev,
            &next,
            6,
            MAX_FORCE_ACCEPT_DELTA_V,
        ));
    }

    #[test]
    fn test_is_clipped_force_candidate_false_when_below_clip_threshold() {
        let prev = vec![0.0; 6];
        let next = vec![0.01, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert!(!Engine::is_clipped_force_candidate(
            &prev,
            &next,
            6,
            MAX_FORCE_ACCEPT_DELTA_V,
        ));
    }

    #[test]
    fn test_startup_step_delta_limit_relaxes_only_for_linearized_seed_window() {
        let base = 5e-2;
        let max_step = 5e-9;
        let early = Engine::startup_step_delta_limit(
            InitialSolutionMode::LinearizedSeed,
            20e-9,
            max_step,
            base,
        );
        let late = Engine::startup_step_delta_limit(
            InitialSolutionMode::LinearizedSeed,
            200e-9,
            max_step,
            base,
        );
        assert!(early > base);
        assert!((late - base).abs() < 1e-18);
    }

    #[test]
    fn test_startup_step_delta_limit_unchanged_for_dc_op_modes() {
        let base = 5e-2;
        let max_step = 5e-9;
        let dc = Engine::startup_step_delta_limit(
            InitialSolutionMode::DcOperatingPoint,
            20e-9,
            max_step,
            base,
        );
        let robust = Engine::startup_step_delta_limit(
            InitialSolutionMode::RobustDcFallback,
            20e-9,
            max_step,
            base,
        );
        assert!((dc - base).abs() < 1e-18);
        assert!((robust - base).abs() < 1e-18);
    }

    #[test]
    fn test_startup_timestep_divisors_for_bjt_decks() {
        let (startup_div, min_div) = Engine::startup_timestep_divisors(true);
        assert!((startup_div - 1000.0).abs() < 1e-12);
        assert!((min_div - 10_000.0).abs() < 1e-9);
    }

    #[test]
    fn test_startup_timestep_divisors_for_non_bjt_decks() {
        let (startup_div, min_div) = Engine::startup_timestep_divisors(false);
        assert!((startup_div - 10.0).abs() < 1e-12);
        assert!((min_div - 1000.0).abs() < 1e-9);
    }

    #[test]
    fn test_bjt_amp_retains_tail_dynamics_after_stall_region() {
        let engine = Engine::default();
        let netlist = simple_bjt_amp_netlist();

        let result = engine
            .run_tran(&netlist, 100e-6, 20e-9)
            .expect("BJT amplifier transient should complete");

        let tail_start_idx = result
            .time
            .iter()
            .position(|&t| t >= 70e-6)
            .expect("tail start must exist");

        let mut dynamic_nodes = 0usize;
        for node in 0..result.num_nodes {
            let tail = &result.voltages[node][tail_start_idx..];
            let mut v_min = Value::INFINITY;
            let mut v_max = Value::NEG_INFINITY;
            for &v in tail {
                v_min = v_min.min(v);
                v_max = v_max.max(v);
            }
            if v_max - v_min > 1e-3 {
                dynamic_nodes += 1;
            }
        }

        assert!(
            dynamic_nodes >= 3,
            "Expected multiple dynamic traces in tail, got {}",
            dynamic_nodes
        );
    }

    #[test]
    fn test_bjt_amp_no_catastrophic_step_jumps_or_negative_runaway() {
        let engine = Engine::default();
        let netlist = simple_bjt_amp_netlist();

        let result = engine
            .run_tran(&netlist, 100e-6, 20e-9)
            .expect("BJT amplifier transient should complete");

        let global_min = result
            .voltages
            .iter()
            .flat_map(|v| v.iter().copied())
            .fold(Value::INFINITY, Value::min);
        assert!(
            global_min > -100.0,
            "Unexpected catastrophic negative runaway detected: min={}",
            global_min
        );

        for node in 1..result.num_nodes {
            let max_step = result.voltages[node]
                .windows(2)
                .map(|w| (w[1] - w[0]).abs())
                .fold(0.0, Value::max);
            assert!(
                max_step < 0.25,
                "Node {} has nonphysical force-accept jump: max_step={}",
                node,
                max_step
            );
        }
    }

    #[test]
    fn test_bjt_amp_bias_nodes_do_not_go_unphysically_negative() {
        let engine = Engine::default();
        let netlist = simple_bjt_amp_netlist();

        let result = engine
            .run_tran(&netlist, 100e-6, 20e-9)
            .expect("BJT amplifier transient should complete");

        // Nodes 1..5 are biased by resistive paths to supply/ground in this topology.
        // They should not run away to deep negative voltages during startup.
        for node in 1..result.num_nodes {
            let v_min = result.voltages[node]
                .iter()
                .copied()
                .fold(Value::INFINITY, Value::min);
            assert!(
                v_min > -0.5,
                "Node {} dropped to nonphysical negative voltage: {}",
                node,
                v_min
            );
        }
    }

    #[test]
    fn test_bjt_amp_output_peak_and_crossing_match_expected_envelope() {
        let engine = Engine::default();
        let netlist = simple_bjt_amp_netlist();

        // Long enough to capture positive peak and downward crossing against net7.
        let result = engine
            .run_tran(&netlist, 600e-6, 20e-9)
            .expect("BJT amplifier transient should complete");

        let net1 = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("net1"))
            .expect("net1 not found");
        let net7 = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("net7"))
            .expect("net7 not found");

        let out = &result.voltages[net1];
        let bypass = &result.voltages[net7];
        assert!(!out.is_empty() && out.len() == bypass.len());

        // Startup bias should be around the expected emitter DC level (definitely not 0V).
        assert!(
            out[0] > 3.0 && out[0] < 4.5,
            "Unexpected net1 startup bias: {}",
            out[0]
        );

        let (peak_idx, peak_v) = out
            .iter()
            .copied()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .expect("peak should exist");
        let peak_t = result.time[peak_idx];
        assert!(
            peak_v > 5.0 && peak_v < 5.6,
            "net1 peak out of expected range: {}",
            peak_v
        );
        assert!(
            peak_t > 150e-6 && peak_t < 300e-6,
            "net1 peak time out of expected range: {}",
            peak_t
        );

        // After peak, net1 should cross below the bypass node in the falling half-cycle.
        let crossing_t = (peak_idx + 1..out.len())
            .find(|&i| out[i] <= bypass[i])
            .map(|i| result.time[i])
            .expect("expected net1/net7 crossing after peak");
        assert!(
            crossing_t > 300e-6 && crossing_t < 520e-6,
            "net1/net7 crossing time out of expected range: {}",
            crossing_t
        );
    }
}
