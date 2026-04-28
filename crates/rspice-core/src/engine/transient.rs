//! Transient Time-Domain Analysis
//!
//! This module provides time-domain simulation using:
//! - Adaptive timestep control with LTE estimation
//! - TrapGear method switching for stability
//! - Optional waveform compression for long simulations
//! - Cooperative abort for responsive cancellation

#![allow(clippy::too_many_arguments)]
use super::{Engine, SimulationError, TransientResult};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::transient::{
    BreakpointManager, CompanionCoefficients, IntegrationMethod, LteEstimator, TimestepController,
    TrapGearController,
};
use crate::analysis::waveform::{CompressionConfig, TransientResultCompressed, WaveformRecorder};
use crate::device::semiconductor::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeBranch,
    BjtChargeSnapshot, BjtType, VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT,
};
use crate::device::{NonlinearConvergenceCriteria, NonlinearDevice};
use crate::netlist::AnalysisCommand;
use crate::{Netlist, Value};

mod startup;

/// Maximum voltage limit for solution values (matching DC solver)
///
/// Commercial simulators like Spectre/HSPICE use similar limits to prevent
/// Newton-Raphson divergence on stiff nonlinear circuits (e.g., BJT exponential I-V).
/// This value matches the DC solver's MAX_VOLTAGE in convergence.rs for consistency.
const MAX_VOLTAGE: Value = 1000.0;
/// Conservative magnitude limit for branch-state unknowns (currents and auxiliary
/// MNA variables). These states can legitimately exceed node-voltage scales in
/// tightly coupled passive networks, so they need a separate guardrail.
const MAX_BRANCH_STATE_MAGNITUDE: Value = 1e12;
/// Maximum allowed per-iteration node update during Newton damping.
///
/// This bound controls nonlinear solve trust-region size.
const MAX_NEWTON_ITER_DELTA_V: Value = 1e-2;
/// Maximum allowed node update when committing force-accepted steps.
///
/// This remains tight to avoid committing nonphysical jumps into reactive history.
const MAX_FORCE_ACCEPT_DELTA_V: Value = 5e-2;
/// Relaxed trust-region limit used only during early startup when DC OP failed and
/// transient had to begin from a linearized seed.
const STARTUP_RECOVERY_DELTA_V: Value = 2e-1;
/// Moderately relaxed early-step trust region for VBIC excess-phase decks.
///
/// The hidden xf states need recovery headroom during startup, but the external
/// node Newton solve should still stay on a sub-volt leash; the device-level
/// branch limiting handles the large internal transport correction.
const VBIC_STARTUP_RECOVERY_DELTA_V: Value = 2e-1;
/// Source edge magnitude that triggers transient source-step capping.
const SOURCE_ACTIVE_DELTA: Value = 1e-2;
/// Safety cap for synthesized transmission-line arrival breakpoints.
const MAX_PROPAGATED_TLINE_BREAKPOINTS: usize = 200_000;
/// Safety cap for dynamically scheduled transmission-line arrival breakpoints.
const MAX_DYNAMIC_TLINE_BREAKPOINTS: usize = 200_000;
/// ngspice default transient truncation tolerance factor (`CKTtrtol`).
///
/// VBIC relies on the classic SPICE charge-state truncation controller in
/// `vbictrunc.c`/`CKTterr`, which uses a more permissive scaling than the
/// generic predictor-based LTE controller.
const NGSPICE_DEFAULT_TRTOL: Value = 7.0;
const VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL: Value = 1e-15;
const VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL: Value = 1e-12;
const BJT_VBIC_TRUNCATION_BRANCH_COUNT: usize = BJT_DYNAMIC_CHARGE_COUNT - 3;
const BJT_VCX_STATE_INDEX: usize = 0;
const BJT_VCI_STATE_INDEX: usize = 1;
const BJT_VBX_STATE_INDEX: usize = 2;
const BJT_VBI_STATE_INDEX: usize = 3;
const BJT_VEI_STATE_INDEX: usize = 4;
const BJT_VBP_STATE_INDEX: usize = 5;
const BJT_VSI_STATE_INDEX: usize = 6;
const BJT_THERMAL_STATE_INDEX: usize = BJT_INTERNAL_STATE_DIM - 3;
const BJT_DELAY_XF1_BRANCH_INDEX: usize = BJT_DYNAMIC_CHARGE_COUNT - 2;
const BJT_DELAY_XF2_BRANCH_INDEX: usize = BJT_DYNAMIC_CHARGE_COUNT - 1;
const BJT_QBE_BRANCH_INDEX: usize = 0;
const BJT_QBC_BRANCH_INDEX: usize = 2;
const BJT_QBCX_BRANCH_INDEX: usize = 3;
const BJT_QBCP_BRANCH_INDEX: usize = 7;
const BJT_DELAY_XF1_STATE_INDEX: usize = BJT_INTERNAL_STATE_DIM - 2;
const BJT_DELAY_XF2_STATE_INDEX: usize = BJT_INTERNAL_STATE_DIM - 1;
const BJT_STATIC_CORE_STATE_DIM: usize = BJT_INTERNAL_STATE_DIM - 2;
const BJT_EXT_C_INDEX: usize = 0;
const BJT_EXT_B_INDEX: usize = 1;
const BJT_EXT_E_INDEX: usize = 2;
const BJT_EXT_S_INDEX: usize = 3;

#[derive(Debug, Clone, Default)]
struct JfetTransientHistory {
    vgs_prev: Vec<Value>,
    vgs_prev_prev: Vec<Value>,
    qgs_prev: Vec<Value>,
    qgs_prev_prev: Vec<Value>,
    qgs_prev_prev_prev: Vec<Value>,
    cqgs_prev: Vec<Value>,
    vgd_prev: Vec<Value>,
    vgd_prev_prev: Vec<Value>,
    qgd_prev: Vec<Value>,
    qgd_prev_prev: Vec<Value>,
    qgd_prev_prev_prev: Vec<Value>,
    cqgd_prev: Vec<Value>,
    accepted_dt_prev: Value,
    accepted_dt_prev_prev: Value,
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
    charge_q_prev: Vec<[Value; BJT_DYNAMIC_CHARGE_COUNT]>,
    charge_q_prev_prev: Vec<[Value; BJT_DYNAMIC_CHARGE_COUNT]>,
    charge_q_prev_prev_prev: Vec<[Value; BJT_DYNAMIC_CHARGE_COUNT]>,
    charge_cq_prev: Vec<[Value; BJT_DYNAMIC_CHARGE_COUNT]>,
    dynamic_internal_prev: Vec<[Value; BJT_INTERNAL_STATE_DIM]>,
    dynamic_internal_prev_prev: Vec<[Value; BJT_INTERNAL_STATE_DIM]>,
    dynamic_linear_prev: Vec<VbicPredictorLinearBranchState>,
    dynamic_linear_prev_prev: Vec<VbicPredictorLinearBranchState>,
    accepted_dt_prev: Value,
    accepted_dt_prev_prev: Value,
}

#[derive(Debug, Clone, Copy)]
struct VbicTransientLinearization {
    g_ii: [[Value; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    g_ie: [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    g_ei: [[Value; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    g_ee: [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    z_i: [Value; BJT_INTERNAL_STATE_DIM],
    z_e: [Value; BJT_EXTERNAL_STATE_DIM],
}

type VbicDynamicStateEvaluation = (
    BjtChargeSnapshot,
    VbicTransientLinearization,
    [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    [Value; BJT_INTERNAL_STATE_DIM],
    Value,
);

type VbicBestEffortSolve = (
    BjtChargeSnapshot,
    VbicTransientLinearization,
    [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    Value,
);

#[derive(Debug, Clone, Copy, Default)]
struct VbicPredictorLinearBranchState {
    vrcx: Value,
    vrci: Value,
    vrbx: Value,
    vrbi: Value,
    vre: Value,
    vrbp: Value,
    vrs: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VbicCachedSnapshotReuse {
    SeedOnly,
    NewtonBypass,
}

#[derive(Debug, Clone, Default)]
struct MosfetTransientHistory {
    vgs_prev: Vec<Value>,
    vgs_prev_prev: Vec<Value>,
    capgs_prev_half: Vec<Value>,
    qgs_prev: Vec<Value>,
    qgs_prev_prev: Vec<Value>,
    qgs_prev_prev_prev: Vec<Value>,
    cqgs_prev: Vec<Value>,
    vgd_prev: Vec<Value>,
    vgd_prev_prev: Vec<Value>,
    capgd_prev_half: Vec<Value>,
    qgd_prev: Vec<Value>,
    qgd_prev_prev: Vec<Value>,
    qgd_prev_prev_prev: Vec<Value>,
    cqgd_prev: Vec<Value>,
    vgb_prev: Vec<Value>,
    vgb_prev_prev: Vec<Value>,
    capgb_prev_half: Vec<Value>,
    qgb_prev: Vec<Value>,
    qgb_prev_prev: Vec<Value>,
    qgb_prev_prev_prev: Vec<Value>,
    cqgb_prev: Vec<Value>,
    vbs_j_prev: Vec<Value>,
    vbs_j_prev_prev: Vec<Value>,
    qbs_prev: Vec<Value>,
    qbs_prev_prev: Vec<Value>,
    cqbs_prev: Vec<Value>,
    vbd_j_prev: Vec<Value>,
    vbd_j_prev_prev: Vec<Value>,
    qbd_prev: Vec<Value>,
    qbd_prev_prev: Vec<Value>,
    cqbd_prev: Vec<Value>,
    accepted_dt_prev: Value,
    accepted_dt_prev_prev: Value,
}

#[derive(Debug, Clone, Default)]
struct CoupledTlineReferenceState {
    near_modal: Vec<Value>,
    far_modal: Vec<Value>,
}

impl Engine {
    #[inline]
    fn max_expected_source_delta(circuit: &crate::circuit::Circuit, t0: Value, t1: Value) -> Value {
        circuit
            .voltage_sources
            .max_expected_delta(t0, t1)
            .max(circuit.current_sources.max_expected_delta(t0, t1))
    }

    #[inline]
    fn startup_source_activity_delta_for_retry_floor(
        circuit: &crate::circuit::Circuit,
        time: Value,
        attempted_dt: Value,
        tstop: Value,
        initial_timestep: Value,
        preferred_min_timestep: Value,
    ) -> Value {
        let retry_floor = initial_timestep.min(preferred_min_timestep);
        let activity_horizon = if retry_floor.is_finite() && retry_floor > 0.0 {
            attempted_dt.max(retry_floor)
        } else {
            attempted_dt
        };
        let remaining = (tstop - time).max(0.0);
        let activity_horizon = activity_horizon.min(remaining);

        if activity_horizon.is_finite() && activity_horizon > 0.0 {
            Self::max_expected_source_delta(circuit, time, time + activity_horizon)
        } else {
            Self::max_expected_source_delta(circuit, time, time + attempted_dt)
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

    fn transmission_line_delays(circuit: &crate::circuit::Circuit) -> Vec<Value> {
        let mut delays: Vec<Value> = circuit
            .tlines
            .iter()
            .map(crate::device::TransmissionLine::delay)
            .chain(
                circuit
                    .coupled_tlines
                    .iter()
                    .flat_map(crate::device::CoupledTransmissionLine::propagation_delays),
            )
            .filter(|delay| delay.is_finite() && *delay > 0.0)
            .collect();
        delays.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        delays.dedup_by(|a, b| {
            let scale = a.abs().max(b.abs()).max(1.0);
            (*a - *b).abs() <= scale * 1e-12
        });
        delays
    }

    fn collect_transient_tline_breakpoints(
        circuit: &crate::circuit::Circuit,
        source_breakpoints: &[Value],
        tstop: Value,
        breakpoints: &mut BreakpointManager,
    ) {
        if source_breakpoints.is_empty() {
            return;
        }

        let delays = Self::transmission_line_delays(circuit);
        if delays.is_empty() {
            return;
        }

        let mut generated = 0_usize;
        'origins: for &origin in source_breakpoints {
            for &delay in &delays {
                let mut arrival = origin + delay;
                while arrival.is_finite() && arrival <= tstop {
                    if breakpoints.add(arrival) {
                        generated += 1;
                        if generated >= MAX_PROPAGATED_TLINE_BREAKPOINTS {
                            log::warn!(
                                "Capped propagated transmission-line breakpoints at {} entries (tstop={:.3e}s)",
                                MAX_PROPAGATED_TLINE_BREAKPOINTS,
                                tstop
                            );
                            break 'origins;
                        }
                    }
                    arrival += delay;
                }
            }
        }
    }

    #[inline]
    fn wave_event_exceeds_tolerance(
        previous: Value,
        current: Value,
        reltol: Value,
        abstol: Value,
    ) -> bool {
        if !previous.is_finite() || !current.is_finite() {
            return false;
        }
        let scale = previous.abs().max(current.abs());
        let threshold = abstol.max(scale * reltol);
        (current - previous).abs() > threshold
    }

    #[inline]
    fn maybe_schedule_tline_arrival_breakpoint(
        breakpoints: &mut BreakpointManager,
        event_time: Value,
        delay: Value,
        tstop: Value,
        previous_wave: Value,
        current_wave: Value,
        reltol: Value,
        abstol: Value,
        dynamic_breakpoints_added: &mut usize,
        warned_dynamic_breakpoint_cap: &mut bool,
    ) {
        if !Self::wave_event_exceeds_tolerance(previous_wave, current_wave, reltol, abstol) {
            return;
        }
        if !(event_time.is_finite() && delay.is_finite() && delay > 0.0) {
            return;
        }

        let arrival = event_time + delay;
        if !(arrival.is_finite() && arrival > event_time && arrival <= tstop) {
            return;
        }

        if *dynamic_breakpoints_added >= MAX_DYNAMIC_TLINE_BREAKPOINTS {
            if !*warned_dynamic_breakpoint_cap {
                log::warn!(
                    "Capped dynamic transmission-line breakpoints at {} entries (tstop={:.3e}s)",
                    MAX_DYNAMIC_TLINE_BREAKPOINTS,
                    tstop
                );
                *warned_dynamic_breakpoint_cap = true;
            }
            return;
        }

        if breakpoints.add(arrival) {
            *dynamic_breakpoints_added += 1;
            if *dynamic_breakpoints_added >= MAX_DYNAMIC_TLINE_BREAKPOINTS
                && !*warned_dynamic_breakpoint_cap
            {
                log::warn!(
                    "Capped dynamic transmission-line breakpoints at {} entries (tstop={:.3e}s)",
                    MAX_DYNAMIC_TLINE_BREAKPOINTS,
                    tstop
                );
                *warned_dynamic_breakpoint_cap = true;
            }
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
    fn top_abs_delta_prefix_named(
        a: &[Value],
        b: &[Value],
        node_names: &[String],
        count: usize,
        max_items: usize,
    ) -> Vec<String> {
        let mut deltas: Vec<(usize, Value)> = a
            .iter()
            .zip(b.iter())
            .take(count)
            .enumerate()
            .map(|(idx, (x, y))| (idx, (x - y).abs()))
            .filter(|(_, delta)| delta.is_finite() && *delta > 0.0)
            .collect();
        deltas.sort_by(|lhs, rhs| rhs.1.total_cmp(&lhs.1));
        deltas
            .into_iter()
            .take(max_items)
            .map(|(idx, delta)| {
                let name = node_names
                    .get(idx)
                    .map(String::as_str)
                    .unwrap_or("<unnamed>");
                format!("{name}:{delta:.3e}")
            })
            .collect()
    }

    #[inline]
    fn max_abs_delta(a: &[Value], b: &[Value]) -> Value {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .fold(0.0, Value::max)
    }

    #[inline]
    fn max_abs_delta_branch_ordinals(
        a: &[Value],
        b: &[Value],
        num_nodes: usize,
        branch_ordinals: &[crate::NodeId],
    ) -> Value {
        branch_ordinals
            .iter()
            .filter_map(|branch_ordinal| {
                let branch_ordinal = usize::try_from(*branch_ordinal).ok()?;
                let idx = num_nodes.checked_add(branch_ordinal.checked_sub(1)?)?;
                let x = *a.get(idx)?;
                let y = *b.get(idx)?;
                Some((x - y).abs())
            })
            .fold(0.0, Value::max)
    }

    #[inline]
    fn is_stagnant_force_candidate(
        circuit: &crate::circuit::Circuit,
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

        // Only dynamic branch-current unknowns represent physical state progression
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
    fn force_candidate_vbic_hidden_bypass_metric_met(
        &self,
        circuit: &crate::circuit::Circuit,
        has_vbic_excess_phase: bool,
        previous_solution: &[Value],
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
    ) -> bool {
        has_vbic_excess_phase
            && self.vbic_excess_phase_device_convergence_met(
                circuit,
                previous_solution,
                candidate_solution,
                method,
                trap_order,
                dt,
                history,
                vbic_snapshot_cache,
            )
    }

    #[inline]
    fn is_stale_step(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        expected_source_delta: Value,
        num_nodes: usize,
    ) -> bool {
        // Only police stale accepts when sources are strongly active on this step.
        // Weak source movement can legitimately yield tiny accepted deltas in
        // high-rejection circuits (for example differential stages).
        if expected_source_delta <= SOURCE_ACTIVE_DELTA {
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
    fn is_excessive_quiet_force_candidate(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        expected_source_delta: Value,
        num_nodes: usize,
        clip_limit: Value,
    ) -> bool {
        if expected_source_delta > SOURCE_ACTIVE_DELTA {
            return false;
        }
        let observed_delta =
            Self::max_abs_delta_prefix(previous_solution, candidate_solution, num_nodes);
        observed_delta > clip_limit * 2.0
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
    fn transient_newton_iteration_budget(
        max_iterations: usize,
        has_vbic_excess_phase: bool,
        retry_count: usize,
    ) -> usize {
        let standard_budget = max_iterations.saturating_mul(4).min(400);
        if !has_vbic_excess_phase {
            standard_budget
        } else if retry_count == 0 {
            standard_budget.max(64).min(96)
        } else {
            standard_budget.max(64)
        }
    }

    #[inline]
    fn vbic_relaxed_convergence_met(
        has_vbic_excess_phase: bool,
        voltage_converged_relaxed: bool,
        device_converged: bool,
        linearized_residual_converged: bool,
    ) -> bool {
        has_vbic_excess_phase
            && voltage_converged_relaxed
            && device_converged
            && linearized_residual_converged
    }

    #[inline]
    fn min_retries_at_minimum_timestep(
        has_vbic_excess_phase: bool,
        step_time: Value,
        hinted_max_step: Value,
    ) -> usize {
        let startup_retry_window = hinted_max_step * 0.1;
        if has_vbic_excess_phase
            && step_time.is_finite()
            && hinted_max_step.is_finite()
            && startup_retry_window.is_finite()
            && step_time <= startup_retry_window
        {
            3
        } else {
            1
        }
    }

    #[inline]
    fn bias_transient_step_for_source_activity(
        proposed_dt: Value,
        remaining_time: Value,
        at_breakpoint: bool,
        expected_source_delta: Value,
        practical_min_dt: Value,
        preferred_min_dt: Value,
    ) -> Value {
        let mut dt = proposed_dt.min(remaining_time);
        if at_breakpoint {
            return dt;
        }

        if expected_source_delta >= SOURCE_ACTIVE_DELTA {
            // During steep source transitions, permit sub-preferred timesteps to
            // track waveform edges accurately, but keep the source-following cap
            // comfortably above the true hard recovery floor.
            let active_cap = (preferred_min_dt / 8.0).max(practical_min_dt);
            if dt > active_cap {
                dt = active_cap;
            }
        }

        dt
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
    fn differential_port_voltages(
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

    #[inline]
    fn tline_transient_port_impedance(tl: &crate::device::TransmissionLine) -> Value {
        // Keep the local port relation anchored to the characteristic
        // impedance; lossy model-card behavior is captured through delayed-wave
        // attenuation and history smoothing rather than by distorting the
        // immediate Z0 boundary condition.
        tl.impedance().max(1e-12)
    }

    #[inline]
    fn tline_transient_port_conductance(tl: &crate::device::TransmissionLine) -> Value {
        1.0 / Self::tline_transient_port_impedance(tl)
    }

    #[inline]
    fn tline_transient_wave_attenuation(tl: &crate::device::TransmissionLine) -> Value {
        tl.attenuation()
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
    fn stamp_tline_cross_conductance(
        matrix: &mut crate::solver::StaticMatrix,
        node_row_pos: usize,
        node_row_neg: usize,
        node_col_pos: usize,
        node_col_neg: usize,
        g_cross: Value,
    ) {
        if g_cross == 0.0 {
            return;
        }

        if node_row_pos > 0 {
            if node_col_pos > 0 {
                matrix.add(node_row_pos - 1, node_col_pos - 1, g_cross);
            }
            if node_col_neg > 0 {
                matrix.add(node_row_pos - 1, node_col_neg - 1, -g_cross);
            }
        }
        if node_row_neg > 0 {
            if node_col_pos > 0 {
                matrix.add(node_row_neg - 1, node_col_pos - 1, -g_cross);
            }
            if node_col_neg > 0 {
                matrix.add(node_row_neg - 1, node_col_neg - 1, g_cross);
            }
        }
    }

    #[inline]
    fn stamp_tline_two_port(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        tl: &crate::device::TransmissionLine,
        response: crate::device::TlineTransientResponse,
    ) {
        Self::stamp_tline_port(
            matrix,
            rhs,
            tl.node1_pos,
            tl.node1_neg,
            response.self_conductance(),
            response.i_eq_port1(),
        );
        Self::stamp_tline_port(
            matrix,
            rhs,
            tl.node2_pos,
            tl.node2_neg,
            response.self_conductance(),
            response.i_eq_port2(),
        );
        Self::stamp_tline_cross_conductance(
            matrix,
            tl.node1_pos,
            tl.node1_neg,
            tl.node2_pos,
            tl.node2_neg,
            response.mutual_conductance(),
        );
        Self::stamp_tline_cross_conductance(
            matrix,
            tl.node2_pos,
            tl.node2_neg,
            tl.node1_pos,
            tl.node1_neg,
            response.mutual_conductance(),
        );
    }

    #[inline]
    fn stamp_shared_reference_port(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        nodes: &[usize],
        reference: usize,
        admittance: &[Vec<Value>],
        eq_currents: &[Value],
    ) {
        let row_sums: Vec<Value> = admittance
            .iter()
            .map(|row| row.iter().copied().sum())
            .collect();

        for (row_idx, &node_row) in nodes.iter().enumerate() {
            if node_row == 0 {
                continue;
            }
            for (col_idx, &node_col) in nodes.iter().enumerate() {
                if node_col > 0 {
                    matrix.add(node_row - 1, node_col - 1, admittance[row_idx][col_idx]);
                }
            }
            if reference > 0 {
                matrix.add(node_row - 1, reference - 1, -row_sums[row_idx]);
            }
            rhs[node_row - 1] += eq_currents.get(row_idx).copied().unwrap_or(0.0);
        }

        if reference > 0 {
            let mut ref_injection = 0.0;
            for (col_idx, &node_col) in nodes.iter().enumerate() {
                if node_col > 0 {
                    matrix.add(reference - 1, node_col - 1, -row_sums[col_idx]);
                }
                ref_injection -= eq_currents.get(col_idx).copied().unwrap_or(0.0);
            }
            let ref_sum: Value = row_sums.iter().copied().sum();
            matrix.add(reference - 1, reference - 1, ref_sum);
            rhs[reference - 1] += ref_injection;
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
    fn stamp_external_reduced_system(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        nodes: &[usize; BJT_EXTERNAL_STATE_DIM],
        y: &[[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        i_eq: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let node_row = nodes[row];
            if node_row == 0 {
                continue;
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let node_col = nodes[col];
                if node_col > 0 {
                    matrix.add(node_row - 1, node_col - 1, y[row][col]);
                }
            }
            rhs[node_row - 1] += i_eq[row];
        }
    }

    #[inline]
    fn jfet_branch_voltages(jfet: &crate::device::Jfet, voltages: &[Value]) -> (Value, Value) {
        if matches!(
            jfet.params.channel_model,
            crate::device::JfetChannelModel::Hfet1
        ) && let Some((vgs, vgd, _vds)) = jfet.internal_branch_state_voltages()
        {
            return (vgs, vgd);
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
        // ngspice HFET1 keeps two voltage tracks: limited vgs/vgd for nonlinear
        // channel/capacitance evaluation, and raw vgspp/vgdpp for charge history.
        // The transient qgs/qgd update must follow the raw branch voltage so a
        // source edge injects the same charge current even when DEVfetlim limits
        // the nonlinear control voltage during Newton iterations.
        if matches!(
            jfet.params.channel_model,
            crate::device::JfetChannelModel::Hfet1
        ) && jfet.params.hfet_level >= 5
        {
            let vg = Self::node_voltage(voltages, jfet.gate);
            let vd = Self::node_voltage(voltages, jfet.drain);
            let vs = Self::node_voltage(voltages, jfet.source);
            return (vg - vs, vg - vd);
        }

        // MESA/HFET2-style devices keep using the limited internal branch state,
        // matching the existing level-2..4 path and its convergence behavior.
        if matches!(
            jfet.params.channel_model,
            crate::device::JfetChannelModel::Hfet1
        ) && let Some((vgs, vgd, _vds)) = jfet.internal_branch_state_voltages()
        {
            return (vgs, vgd);
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
    fn effective_companion_method(method: IntegrationMethod, trap_order: u8) -> IntegrationMethod {
        match method {
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear if trap_order <= 1 => {
                IntegrationMethod::BackwardEuler
            }
            _ => method,
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
        // Match ngspice nonlinear charge-branch transient update:
        // q(n+1) = q(n) + C(n+1) * (v(n+1) - v(n))
        let q_curr = q_prev + capacitance * (v_curr - v_prev);
        let cq_curr =
            Self::jfet_companion_ccap(method, trap_order, dt, q_curr, q_prev, q_prev_prev, cq_prev);
        // Match ngspice load linearization contract for capacitive branches:
        //   i(v) â‰ˆ ccap + geq * (v - v_hist) = geq * v - (geq * v_hist - ccap).
        // With our companion stamp convention (i = geq * v - i_eq), this gives:
        //   i_eq = geq * v_hist - ccap.
        // NOTE: This intentionally uses branch voltage history, not charge, because
        // q is not generally equal to C * v for voltage-dependent capacitances.
        let ieq = geq * v_curr - cq_curr;
        (geq, ieq, q_curr, cq_curr)
    }

    #[inline]
    fn nonlinear_charge_companion_terms(
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        capacitance: Value,
        v_curr: Value,
        q_curr: Value,
        q_prev: Value,
        q_prev_prev: Value,
        cq_prev: Value,
    ) -> (Value, Value, Value, Value) {
        let geq = Self::jfet_companion_geq(method, trap_order, capacitance, dt);
        if geq == 0.0 {
            return (0.0, 0.0, q_curr, 0.0);
        }
        let cq_curr =
            Self::jfet_companion_ccap(method, trap_order, dt, q_curr, q_prev, q_prev_prev, cq_prev);
        let ieq = geq * v_curr - cq_curr;
        (geq, ieq, q_curr, cq_curr)
    }

    #[inline]
    fn linear_charge_history_ieq(
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: Value,
        q_prev_prev: Value,
        cq_prev: Value,
    ) -> Value {
        if !dt.is_finite() || dt <= 0.0 {
            return 0.0;
        }
        match method {
            IntegrationMethod::BackwardEuler => q_prev / dt,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => {
                if trap_order <= 1 {
                    q_prev / dt
                } else {
                    cq_prev + 2.0 * q_prev / dt
                }
            }
            IntegrationMethod::Gear2 => (2.0 * q_prev - 0.5 * q_prev_prev) / dt,
        }
    }

    #[inline]
    fn predict_transient_history_value(
        previous: Value,
        previous_previous: Option<Value>,
        dt: Value,
        previous_dt: Value,
    ) -> Value {
        let Some(previous_previous) = previous_previous else {
            return previous;
        };
        if !(dt.is_finite() && dt > 0.0 && previous_dt.is_finite() && previous_dt > 0.0) {
            return previous;
        }

        let xfact = dt / previous_dt;
        let predicted = (1.0 + xfact) * previous - xfact * previous_previous;
        if predicted.is_finite() {
            predicted
        } else {
            previous
        }
    }

    #[inline]
    fn rebalance_vbic_dynamic_thermal_state(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        snapshot: &mut crate::device::semiconductor::BjtChargeSnapshot,
    ) {
        let mut internal = snapshot.reduction.internal_voltages;
        let original_vrth = internal[BJT_THERMAL_STATE_INDEX];
        let minimum_vrth = bjt.minimum_thermal_rise();
        let mut best_internal = internal;
        let mut best_residual = Value::INFINITY;

        for _ in 0..8 {
            let (residual, derivative) = Self::vbic_transient_thermal_residual_and_derivative(
                bjt,
                vc,
                vb,
                ve,
                vs,
                internal,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            );
            let residual_abs = residual.abs();
            if residual_abs.is_finite() && residual_abs < best_residual {
                best_residual = residual_abs;
                best_internal = internal;
            }
            if !residual.is_finite() || !derivative.is_finite() || derivative.abs() < 1e-18 {
                break;
            }
            if residual_abs < 1e-12 {
                break;
            }

            let current_vrth = internal[BJT_THERMAL_STATE_INDEX];
            let max_step = (current_vrth - minimum_vrth + 10.0).max(1.0) * 0.5;
            let step = (-residual / derivative).clamp(-max_step, max_step);
            if step.abs() < 1e-12 {
                break;
            }

            let mut alpha = 1.0;
            let mut accepted = false;
            let mut best_candidate = internal;
            let mut best_candidate_residual = residual_abs;
            for _ in 0..10 {
                let candidate_vrth = (current_vrth + alpha * step).max(minimum_vrth);
                if (candidate_vrth - current_vrth).abs() < 1e-12 {
                    break;
                }

                let mut candidate = internal;
                candidate[BJT_THERMAL_STATE_INDEX] = candidate_vrth;
                let (candidate_residual, _) = Self::vbic_transient_thermal_residual_and_derivative(
                    bjt,
                    vc,
                    vb,
                    ve,
                    vs,
                    candidate,
                    method,
                    trap_order,
                    dt,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let candidate_abs = candidate_residual.abs();
                if candidate_abs.is_finite() && candidate_abs < best_candidate_residual {
                    best_candidate = candidate;
                    best_candidate_residual = candidate_abs;
                }
                if candidate_abs.is_finite() && candidate_abs < residual_abs {
                    internal = candidate;
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if accepted {
                continue;
            }
            if best_candidate_residual + 1e-15 < residual_abs {
                internal = best_candidate;
                continue;
            }
            break;
        }

        if best_residual.is_finite()
            && best_residual < 1e-9
            && (best_internal[BJT_THERMAL_STATE_INDEX] - original_vrth).abs() >= 1e-12
        {
            *snapshot = bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, best_internal);
        }
    }

    #[inline]
    fn vbic_transient_thermal_residual_and_derivative(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> (Value, Value) {
        let thermal_charge_idx = BJT_DYNAMIC_CHARGE_COUNT - 3;
        let (mut residual, mut derivative) =
            bjt.vbic_dynamic_thermal_residual_and_derivative(vc, vb, ve, vs, internal);

        let cth = bjt.thermal_capacitance();
        let charge_factor = Self::jfet_companion_geq(method, trap_order, 1.0, dt);
        if cth > 0.0 && charge_factor > 0.0 {
            let vrth = internal[BJT_THERMAL_STATE_INDEX];
            let ieq = Self::linear_charge_history_ieq(
                method,
                trap_order,
                dt,
                q_prev[thermal_charge_idx],
                q_prev_prev[thermal_charge_idx],
                cq_prev[thermal_charge_idx],
            );
            residual += charge_factor * cth * vrth - ieq;
            derivative += charge_factor * cth;
        }

        (residual, derivative)
    }

    #[inline]
    fn assemble_vbic_transient_linearization(
        bjt: &crate::device::Bjt,
        snapshot: &crate::device::semiconductor::BjtChargeSnapshot,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> Option<VbicTransientLinearization> {
        let charge_factor = Self::jfet_companion_geq(method, trap_order, 1.0, dt);
        if charge_factor <= 0.0 {
            return None;
        }

        let mut g_ii = snapshot.reduction.g_ii;
        let mut g_ie = snapshot.reduction.g_ie;
        let mut g_ei = snapshot.reduction.g_ei;
        let mut g_ee = snapshot.reduction.g_ee;
        let mut c_ii = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ie = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ei = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut c_ee = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut z_i = snapshot.reduction.z_i_static;
        let mut z_e = snapshot.reduction.z_e_static;
        let mut has_dynamic_charge = false;

        if bjt.uses_vbic_dynamic_charges() {
            for branch in bjt.vbic_delay_static_branches(&snapshot.reduction) {
                if !branch.is_active() {
                    continue;
                }
                let i_eq = branch.linearization_dot(
                    &snapshot.reduction.internal_voltages,
                    &snapshot.reduction.external_voltages,
                ) - branch.current;
                branch.accumulate_source(i_eq, &mut z_i, &mut z_e);
            }
            let thermal_branch = bjt.vbic_delay_static_thermal_branch(&snapshot.reduction);
            if thermal_branch.is_active() {
                // The dynamic reduction already carries the collector/emitter and xf delay
                // branch Jacobians. The excess-phase thermal-power correction is a separate
                // delta branch that must be stamped here to keep the temperature row
                // consistent with the delayed transport path.
                thermal_branch.accumulate_derivatives(&mut g_ii, &mut g_ie, &mut g_ei, &mut g_ee);
                let i_eq = thermal_branch.linearization_dot(
                    &snapshot.reduction.internal_voltages,
                    &snapshot.reduction.external_voltages,
                ) - thermal_branch.current;
                thermal_branch.accumulate_source(i_eq, &mut z_i, &mut z_e);
            }
        }

        for (branch_idx, full_branch) in snapshot.branches.iter().enumerate() {
            let (branch, ccap_history_sign) = if bjt.uses_vbic_dynamic_charges() {
                let Some(branch) =
                    Self::vbic_transient_owning_charge_branch(bjt, branch_idx, full_branch)
                else {
                    continue;
                };
                (
                    branch,
                    Self::vbic_transient_owning_charge_ccap_sign(bjt, branch_idx),
                )
            } else {
                if !full_branch.is_active() {
                    continue;
                }
                (*full_branch, 1.0)
            };
            branch.accumulate_derivatives(&mut c_ii, &mut c_ie, &mut c_ei, &mut c_ee);
            let cq_curr = Self::jfet_companion_ccap(
                method,
                trap_order,
                dt,
                branch.charge,
                q_prev[branch_idx],
                q_prev_prev[branch_idx],
                cq_prev[branch_idx],
            );
            let i_eq = charge_factor
                * branch.linearization_dot(
                    &snapshot.reduction.internal_voltages,
                    &snapshot.reduction.external_voltages,
                )
                - ccap_history_sign * cq_curr;
            branch.accumulate_source(i_eq, &mut z_i, &mut z_e);
            has_dynamic_charge = true;
        }

        if !has_dynamic_charge {
            return None;
        }

        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                g_ii[row][col] += charge_factor * c_ii[row][col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                g_ie[row][col] += charge_factor * c_ie[row][col];
            }
        }
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                g_ei[row][col] += charge_factor * c_ei[row][col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                g_ee[row][col] += charge_factor * c_ee[row][col];
            }
        }

        Some(VbicTransientLinearization {
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            z_i,
            z_e,
        })
    }

    #[inline]
    fn vbic_transient_owning_charge_branch(
        bjt: &crate::device::Bjt,
        branch_idx: usize,
        branch: &BjtChargeBranch,
    ) -> Option<BjtChargeBranch> {
        if !branch.is_active() {
            return None;
        }

        let p = match bjt.bjt_type {
            crate::device::BjtType::Npn => 1.0,
            crate::device::BjtType::Pnp => -1.0,
        };
        match branch_idx {
            // ngspice transient integrates Qbe only against Vbei and injects the
            // resulting companion into the Ibe equation. The matrix stamp remains
            // a positive two-terminal conductance for both NPN and PNP; VBICtype
            // only changes the RHS current orientation.
            0 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VEI_STATE_INDEX],
            ),
            // Qbex is integrated only against Vbex.
            1 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VEI_STATE_INDEX],
            ),
            // Qbc is integrated only against Vbci.
            2 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VCI_STATE_INDEX],
            ),
            // Qbcx is integrated only against Vbcx.
            3 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VCX_STATE_INDEX],
            ),
            // Qbep is integrated only against Vbep.
            4 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VBP_STATE_INDEX],
            ),
            // Qbeo is integrated only against the external Vbe branch voltage.
            5 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_external[BJT_EXT_E_INDEX],
            ),
            // Qbco is integrated only against the external Vbc branch voltage.
            6 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_external[BJT_EXT_C_INDEX],
            ),
            // Qbcp is integrated only against Vbcp.
            7 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                -p * branch.d_internal[BJT_VBP_STATE_INDEX],
            ),
            // Qcth, Qxf1, and Qxf2 are single-state companions in ngspice.
            idx if idx == BJT_DYNAMIC_CHARGE_COUNT - 3 => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                branch.d_internal[BJT_THERMAL_STATE_INDEX],
            ),
            idx if idx == BJT_DELAY_XF1_BRANCH_INDEX => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                branch.d_internal[BJT_DELAY_XF1_STATE_INDEX],
            ),
            idx if idx == BJT_DELAY_XF2_BRANCH_INDEX => Self::vbic_branch_voltage_charge_branch(
                branch.charge,
                branch.pos_internal,
                branch.neg_internal,
                branch.pos_external,
                branch.neg_external,
                1.0,
                branch.d_internal[BJT_DELAY_XF2_STATE_INDEX],
            ),
            _ => None,
        }
    }

    #[inline]
    fn vbic_transient_owning_charge_ccap_sign(
        bjt: &crate::device::Bjt,
        branch_idx: usize,
    ) -> Value {
        let p = match bjt.bjt_type {
            crate::device::BjtType::Npn => 1.0,
            crate::device::BjtType::Pnp => -1.0,
        };
        match branch_idx {
            // ngspice keeps the owning-capacitance matrix orientation positive for
            // both NPN and PNP, but the companion history current enters through
            // branch RHS terms that are multiplied by VBICtype for these branches.
            0 | 1 | 2 | 4 | 5 | 6 | 7 => p,
            // Qbcx, Qcth, Qxf1, and Qxf2 are stamped without VBICtype on the RHS.
            _ => 1.0,
        }
    }

    #[inline]
    fn vbic_branch_voltage_charge_branch(
        charge: Value,
        pos_internal: Option<usize>,
        neg_internal: Option<usize>,
        pos_external: Option<usize>,
        neg_external: Option<usize>,
        voltage_sign: Value,
        dq_dv: Value,
    ) -> Option<BjtChargeBranch> {
        if !dq_dv.is_finite() || dq_dv.abs() <= 0.0 {
            return None;
        }

        let mut branch = BjtChargeBranch {
            charge,
            pos_internal,
            neg_internal,
            pos_external,
            neg_external,
            ..Default::default()
        };
        if let Some(idx) = pos_internal {
            branch.d_internal[idx] += voltage_sign * dq_dv;
        }
        if let Some(idx) = neg_internal {
            branch.d_internal[idx] -= voltage_sign * dq_dv;
        }
        if let Some(idx) = pos_external {
            branch.d_external[idx] += voltage_sign * dq_dv;
        }
        if let Some(idx) = neg_external {
            branch.d_external[idx] -= voltage_sign * dq_dv;
        }
        Some(branch)
    }

    #[inline]
    fn solve_vbic_internal_state_from_linearization(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        let (lu_internal, pivots_internal) =
            Self::lu_decompose_small_dense_real(&linearization.g_ii, BJT_INTERNAL_STATE_DIM)?;
        let mut rhs_internal = linearization.z_i;
        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                rhs_internal[row] -= linearization.g_ie[row][col] * external_voltages[col];
            }
        }
        Self::lu_solve_small_dense_real(
            &lu_internal,
            &pivots_internal,
            &rhs_internal,
            BJT_INTERNAL_STATE_DIM,
        )
    }

    #[inline]
    fn solve_vbic_static_core_from_linearization(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        let mut g_static = [[0.0; BJT_STATIC_CORE_STATE_DIM]; BJT_STATIC_CORE_STATE_DIM];
        let mut rhs_static = [0.0; BJT_STATIC_CORE_STATE_DIM];
        for row in 0..BJT_STATIC_CORE_STATE_DIM {
            rhs_static[row] = linearization.z_i[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                rhs_static[row] -= linearization.g_ie[row][col] * external_voltages[col];
            }
            for col in BJT_STATIC_CORE_STATE_DIM..BJT_INTERNAL_STATE_DIM {
                rhs_static[row] -= linearization.g_ii[row][col] * internal_voltages[col];
            }
            for col in 0..BJT_STATIC_CORE_STATE_DIM {
                g_static[row][col] = linearization.g_ii[row][col];
            }
        }
        let (lu_static, pivots_static) =
            Self::lu_decompose_small_dense_real(&g_static, BJT_STATIC_CORE_STATE_DIM)?;
        let solved_static = Self::lu_solve_small_dense_real(
            &lu_static,
            &pivots_static,
            &rhs_static,
            BJT_STATIC_CORE_STATE_DIM,
        )?;
        let mut solved_internal = *internal_voltages;
        solved_internal[..BJT_STATIC_CORE_STATE_DIM].copy_from_slice(&solved_static);
        Some(solved_internal)
    }

    #[inline]
    fn vbic_internal_equation_residual(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let mut residual = [0.0; BJT_INTERNAL_STATE_DIM];
        for row in 0..BJT_INTERNAL_STATE_DIM {
            residual[row] = -linearization.z_i[row];
            for col in 0..BJT_INTERNAL_STATE_DIM {
                residual[row] += linearization.g_ii[row][col] * internal_voltages[col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                residual[row] += linearization.g_ie[row][col] * external_voltages[col];
            }
        }
        residual
    }

    #[inline]
    fn vbic_internal_equation_residual_norm(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Value {
        Self::vbic_internal_equation_residual(linearization, external_voltages, internal_voltages)
            .into_iter()
            .fold(0.0, |max_norm, value| max_norm.max(value.abs()))
    }

    #[inline]
    fn vbic_internal_equation_residual_objective(
        residual: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> Value {
        residual
            .iter()
            .map(|value| value * value)
            .sum::<Value>()
            .sqrt()
    }

    #[inline]
    fn vbic_dynamic_state_evaluation_residual_objective(
        evaluation: &VbicDynamicStateEvaluation,
    ) -> Value {
        Self::vbic_internal_equation_residual_objective(&evaluation.3)
    }

    #[inline]
    fn vbic_dynamic_static_core_residual_norm(residual: &[Value; BJT_INTERNAL_STATE_DIM]) -> Value {
        residual[..BJT_STATIC_CORE_STATE_DIM]
            .iter()
            .fold(0.0_f64, |max_norm, value| max_norm.max(value.abs()))
    }

    #[inline]
    fn refine_vbic_dynamic_static_core_with_fixed_delay(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        mut current_state: VbicDynamicStateEvaluation,
        max_iterations: usize,
    ) -> VbicDynamicStateEvaluation {
        let mut current_objective =
            Self::vbic_dynamic_state_evaluation_residual_objective(&current_state);
        for iteration in 0..max_iterations {
            let static_residual_norm =
                Self::vbic_dynamic_static_core_residual_norm(&current_state.3);
            if static_residual_norm < 1e-10 {
                break;
            }

            let current_internal = current_state.0.reduction.internal_voltages;
            let Some(target_internal) = Self::solve_vbic_static_core_from_linearization(
                &current_state.1,
                &current_state.0.reduction.external_voltages,
                &current_internal,
            ) else {
                break;
            };
            if !target_internal.iter().all(|value| value.is_finite()) {
                break;
            }
            let max_static_delta = (0..BJT_STATIC_CORE_STATE_DIM)
                .map(|idx| (target_internal[idx] - current_internal[idx]).abs())
                .fold(0.0_f64, Value::max);
            if max_static_delta < 1e-12 {
                break;
            }

            let target_internal = Self::step_limit_vbic_dynamic_internal_target(
                current_internal,
                target_internal,
                iteration,
                current_state.4,
            );
            let Some(next_state) = Self::improve_vbic_dynamic_internal_state_toward_target(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                current_internal,
                current_state.4,
                current_objective,
                target_internal,
                current_internal,
                12,
            ) else {
                break;
            };
            let next_objective =
                Self::vbic_dynamic_state_evaluation_residual_objective(&next_state);
            if next_objective + 1e-15 >= current_objective {
                break;
            }
            current_state = next_state;
            current_objective = next_objective;
        }
        current_state
    }

    const VBIC_DYNAMIC_INTERNAL_ACCEPT_RESIDUAL_NORM: Value = 1e-6;
    const VBIC_DYNAMIC_BOUNDED_BEST_EFFORT_RESIDUAL_NORM: Value = 5e-2;
    const VBIC_HOMOTOPY_MIN_LAMBDA_STEP: Value = 1e-6;
    const VBIC_CONTINUATION_MIN_TRIAL_STEP: Value = 1.0 / 64.0;

    #[inline]
    fn vbic_dynamic_snapshot_residual_is_acceptable(residual_norm: Value) -> bool {
        residual_norm.is_finite()
            && residual_norm <= Self::VBIC_DYNAMIC_INTERNAL_ACCEPT_RESIDUAL_NORM
    }

    #[inline]
    fn vbic_dynamic_snapshot_best_effort_is_bounded(result: &VbicBestEffortSolve) -> bool {
        result.3.is_finite()
            && result.3 <= Self::VBIC_DYNAMIC_BOUNDED_BEST_EFFORT_RESIDUAL_NORM
            && result
                .0
                .reduction
                .internal_voltages
                .iter()
                .all(|value| value.is_finite())
    }

    #[inline]
    fn vbic_dynamic_snapshot_solution_is_acceptable(
        linearization: &VbicTransientLinearization,
        external_voltages: &[Value; BJT_EXTERNAL_STATE_DIM],
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
    ) -> bool {
        let residual = Self::vbic_internal_equation_residual(
            linearization,
            external_voltages,
            internal_voltages,
        );
        let criteria = NonlinearConvergenceCriteria::default();
        let reltol = criteria.relative_tolerance();
        let current_tol = criteria.current_tolerance();

        residual.into_iter().enumerate().all(|(row, value)| {
            if !value.is_finite() {
                return false;
            }
            let lhs = linearization.g_ii[row]
                .iter()
                .zip(internal_voltages.iter())
                .map(|(coefficient, state)| coefficient * state)
                .sum::<Value>()
                + linearization.g_ie[row]
                    .iter()
                    .zip(external_voltages.iter())
                    .map(|(coefficient, voltage)| coefficient * voltage)
                    .sum::<Value>();
            let rhs = linearization.z_i[row];
            if !lhs.is_finite() || !rhs.is_finite() {
                return false;
            }
            let tolerance = current_tol + reltol * lhs.abs().max(rhs.abs());
            value.abs() <= tolerance
        })
    }

    #[inline]
    fn choose_preferred_vbic_best_effort_result<F>(
        current: Option<VbicBestEffortSolve>,
        alternate: Option<VbicBestEffortSolve>,
        is_acceptable: F,
    ) -> Option<VbicBestEffortSolve>
    where
        F: Fn(&VbicBestEffortSolve) -> bool,
    {
        match (current, alternate) {
            (Some(current), Some(alternate)) => {
                let current_acceptable = is_acceptable(&current);
                let alternate_acceptable = is_acceptable(&alternate);
                if alternate_acceptable != current_acceptable {
                    if alternate_acceptable {
                        Some(alternate)
                    } else {
                        Some(current)
                    }
                } else if alternate.3 + 1e-18 < current.3 {
                    Some(alternate)
                } else {
                    Some(current)
                }
            }
            (Some(current), None) => Some(current),
            (None, Some(alternate)) => Some(alternate),
            (None, None) => None,
        }
    }

    #[inline]
    fn vbic_dynamic_internal_state_step_limit_for_index(
        index: usize,
        _iteration: usize,
        residual_norm: Value,
    ) -> Value {
        match index {
            BJT_THERMAL_STATE_INDEX => {
                if residual_norm > 1e-2 {
                    2.0
                } else if residual_norm > 1e-4 {
                    1.0
                } else if residual_norm > 1e-8 {
                    0.5
                } else {
                    0.1
                }
            }
            _ => {
                if residual_norm > 1e-2 {
                    1.0
                } else if residual_norm > 1e-4 {
                    0.5
                } else if residual_norm > 1e-8 {
                    0.25
                } else {
                    0.1
                }
            }
        }
    }

    #[inline]
    fn step_limit_vbic_dynamic_internal_target(
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_internal: [Value; BJT_INTERNAL_STATE_DIM],
        iteration: usize,
        residual_norm: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let mut max_raw_delta = 0.0_f64;
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            max_raw_delta = max_raw_delta.max((target_internal[idx] - current_internal[idx]).abs());
        }
        if !max_raw_delta.is_finite() || max_raw_delta < 1e-13 {
            return current_internal;
        }

        let mut alpha = 1.0_f64;
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            let delta = (target_internal[idx] - current_internal[idx]).abs();
            if !delta.is_finite() {
                return current_internal;
            }
            if delta < 1e-13 {
                continue;
            }
            let limit = Self::vbic_dynamic_internal_state_step_limit_for_index(
                idx,
                iteration,
                residual_norm,
            );
            alpha = alpha.min(limit / delta);
        }
        let alpha = if alpha.is_finite() {
            alpha.min(1.0)
        } else {
            return current_internal;
        };
        if alpha <= 0.0 {
            return current_internal;
        }

        let mut limited_target = current_internal;
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            limited_target[idx] =
                current_internal[idx] + alpha * (target_internal[idx] - current_internal[idx]);
        }
        limited_target
    }

    #[inline]
    fn vbic_predictor_linear_branch_state(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> VbicPredictorLinearBranchState {
        let polarity = match bjt.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        };
        VbicPredictorLinearBranchState {
            vrcx: polarity * (external[BJT_EXT_C_INDEX] - internal[BJT_VCX_STATE_INDEX]),
            vrci: polarity * (internal[BJT_VCX_STATE_INDEX] - internal[BJT_VCI_STATE_INDEX]),
            vrbx: polarity * (external[1] - internal[BJT_VBX_STATE_INDEX]),
            vrbi: polarity * (internal[BJT_VBX_STATE_INDEX] - internal[BJT_VBI_STATE_INDEX]),
            vre: polarity * (external[BJT_EXT_E_INDEX] - internal[BJT_VEI_STATE_INDEX]),
            vrbp: polarity * (internal[BJT_VBP_STATE_INDEX] - internal[BJT_VCX_STATE_INDEX]),
            vrs: polarity * (external[3] - internal[BJT_VSI_STATE_INDEX]),
        }
    }

    #[inline]
    fn evaluate_vbic_dynamic_internal_state(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> Option<VbicDynamicStateEvaluation> {
        let mut snapshot = bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, internal);
        Self::rebalance_vbic_dynamic_thermal_state(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            &mut snapshot,
        );
        let base_static_g = snapshot.reduction.g_reduced;
        let linearization = Self::assemble_vbic_transient_linearization(
            bjt,
            &snapshot,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        )?;
        let residual = Self::vbic_internal_equation_residual(
            &linearization,
            &snapshot.reduction.external_voltages,
            &snapshot.reduction.internal_voltages,
        );
        let residual_norm = residual
            .iter()
            .fold(0.0_f64, |max_norm, value| max_norm.max(value.abs()));
        Some((
            snapshot,
            linearization,
            base_static_g,
            residual,
            residual_norm,
        ))
    }

    fn improve_vbic_dynamic_internal_state_toward_target(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        _current_residual_norm: Value,
        current_residual_objective: Value,
        target_internal: [Value; BJT_INTERNAL_STATE_DIM],
        envelope_reference: [Value; BJT_INTERNAL_STATE_DIM],
        max_backtracks: usize,
    ) -> Option<VbicDynamicStateEvaluation> {
        let mut alpha = 1.0;
        let mut best_state: Option<VbicDynamicStateEvaluation> = None;

        for _ in 0..max_backtracks {
            let mut candidate_internal = current_internal;
            for idx in 0..BJT_INTERNAL_STATE_DIM {
                candidate_internal[idx] =
                    current_internal[idx] + alpha * (target_internal[idx] - current_internal[idx]);
            }
            candidate_internal = bjt.limit_vbic_dynamic_internal_state_to_previous(
                candidate_internal,
                current_internal,
            );
            if !candidate_internal.iter().all(|value| value.is_finite()) {
                alpha *= 0.5;
                continue;
            }
            if !bjt.vbic_dynamic_internal_state_within_local_branch_envelope(
                candidate_internal,
                envelope_reference,
            ) {
                alpha *= 0.5;
                continue;
            }

            let Some(candidate_state) = Self::evaluate_vbic_dynamic_internal_state(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                candidate_internal,
            ) else {
                alpha *= 0.5;
                continue;
            };
            let candidate_objective =
                Self::vbic_dynamic_state_evaluation_residual_objective(&candidate_state);
            if candidate_state.4.is_finite()
                && best_state.as_ref().map_or(true, |best_state| {
                    candidate_objective + 1e-15
                        < Self::vbic_dynamic_state_evaluation_residual_objective(best_state)
                })
            {
                best_state = Some(candidate_state.clone());
            }
            if candidate_state.4.is_finite()
                && candidate_objective + 1e-15 < current_residual_objective
            {
                return Some(candidate_state);
            }
            alpha *= 0.5;
        }

        best_state.and_then(|best_state| {
            if Self::vbic_dynamic_state_evaluation_residual_objective(&best_state) + 1e-15
                < current_residual_objective
            {
                Some(best_state)
            } else {
                None
            }
        })
    }

    #[inline]
    fn vbic_reduce_transient_external_system(
        linearization: &VbicTransientLinearization,
    ) -> Option<(
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        [Value; BJT_EXTERNAL_STATE_DIM],
    )> {
        let (lu_internal, pivots_internal) =
            Self::lu_decompose_small_dense_real(&linearization.g_ii, BJT_INTERNAL_STATE_DIM)?;

        let mut y_total = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        for col in 0..BJT_EXTERNAL_STATE_DIM {
            let mut rhs_internal = [0.0; BJT_INTERNAL_STATE_DIM];
            for row in 0..BJT_INTERNAL_STATE_DIM {
                rhs_internal[row] = -linearization.g_ie[row][col];
            }
            let solution = Self::lu_solve_small_dense_real(
                &lu_internal,
                &pivots_internal,
                &rhs_internal,
                BJT_INTERNAL_STATE_DIM,
            )?;
            for row in 0..BJT_EXTERNAL_STATE_DIM {
                let mut value = linearization.g_ee[row][col];
                for internal_idx in 0..BJT_INTERNAL_STATE_DIM {
                    value += linearization.g_ei[row][internal_idx] * solution[internal_idx];
                }
                y_total[row][col] = value;
            }
        }

        let z_solution = Self::lu_solve_small_dense_real(
            &lu_internal,
            &pivots_internal,
            &linearization.z_i,
            BJT_INTERNAL_STATE_DIM,
        )?;
        let mut reduced_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            reduced_i_eq[row] = linearization.z_e[row];
            for internal_idx in 0..BJT_INTERNAL_STATE_DIM {
                reduced_i_eq[row] -=
                    linearization.g_ei[row][internal_idx] * z_solution[internal_idx];
            }
        }

        Some((y_total, reduced_i_eq))
    }

    #[inline]
    fn vbic_static_stamped_external_system(
        bjt: &crate::device::Bjt,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) -> (
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        [Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        bjt.stamped_reduced_external_system(external[0], external[1], external[2], external[3])
    }

    #[inline]
    fn solve_vbic_dynamic_snapshot(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        Self::solve_vbic_dynamic_snapshot_primary(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            seed_internal,
        )
        .or_else(|| {
            Self::solve_vbic_dynamic_snapshot_with_collector_substrate_charge_homotopy(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )
        })
    }

    #[inline]
    fn solve_vbic_dynamic_snapshot_primary(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        Self::solve_vbic_dynamic_snapshot_direct(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            seed_internal,
        )
        .or_else(|| {
            Self::solve_vbic_dynamic_snapshot_with_excess_phase_homotopy(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )
        })
    }

    fn solve_vbic_dynamic_snapshot_with_collector_substrate_charge_homotopy(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        if !bjt.uses_vbic_dynamic_charges() {
            return None;
        }
        if bjt.qco <= 0.0 && bjt.cjcp <= 0.0 && bjt.ccso <= 0.0 {
            return None;
        }

        let scale_collector_substrate_history =
            |lambda: Value,
             q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
             q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
             cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT]| {
                let mut scaled_q_prev = *q_prev;
                let mut scaled_q_prev_prev = *q_prev_prev;
                let mut scaled_cq_prev = *cq_prev;
                // Keep the homotopy path self-consistent by scaling the stored
                // collector/substrate charge histories alongside the scaled VBIC
                // Qbc/Qbcx/Qbcp branch equations.
                for branch_idx in [
                    BJT_QBC_BRANCH_INDEX,
                    BJT_QBCX_BRANCH_INDEX,
                    BJT_QBCP_BRANCH_INDEX,
                ] {
                    scaled_q_prev[branch_idx] *= lambda;
                    scaled_q_prev_prev[branch_idx] *= lambda;
                    scaled_cq_prev[branch_idx] *= lambda;
                }
                (scaled_q_prev, scaled_q_prev_prev, scaled_cq_prev)
            };

        let mut lambda: Value = 0.0;
        let mut step: Value = 1.0;
        let mut current_state = {
            let scaled_bjt = bjt.vbic_collector_substrate_charge_homotopy_variant(0.0);
            let initial_seed = seed_internal
                .copied()
                .unwrap_or_else(|| scaled_bjt.dynamic_internal_state_seed(vc, vb, ve, vs));
            let (scaled_q_prev, scaled_q_prev_prev, scaled_cq_prev) =
                scale_collector_substrate_history(0.0, q_prev, q_prev_prev, cq_prev);
            Self::solve_vbic_dynamic_snapshot_primary(
                &scaled_bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &scaled_q_prev,
                &scaled_q_prev_prev,
                &scaled_cq_prev,
                Some(&initial_seed),
            )?
        };

        while lambda < 1.0 - 1e-15 {
            let candidate_lambda = (lambda + step).min(1.0);
            let scaled_bjt = bjt.vbic_collector_substrate_charge_homotopy_variant(candidate_lambda);
            let previous_internal = current_state.0.reduction.internal_voltages;
            let (scaled_q_prev, scaled_q_prev_prev, scaled_cq_prev) =
                scale_collector_substrate_history(candidate_lambda, q_prev, q_prev_prev, cq_prev);
            let Some(candidate_state) = Self::solve_vbic_dynamic_snapshot_primary(
                &scaled_bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &scaled_q_prev,
                &scaled_q_prev_prev,
                &scaled_cq_prev,
                Some(&previous_internal),
            ) else {
                if step <= Self::VBIC_HOMOTOPY_MIN_LAMBDA_STEP {
                    return None;
                }
                step *= 0.5;
                continue;
            };
            current_state = candidate_state;
            lambda = candidate_lambda;
            step = (step * 2.0).min(1.0 - lambda).max(1e-6);
        }

        Some(current_state)
    }

    fn solve_vbic_dynamic_snapshot_with_excess_phase_homotopy(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        if !bjt.uses_vbic_dynamic_charges() || bjt.td <= 0.0 {
            return None;
        }

        let target_td = bjt.td;
        let scale_excess_phase_history =
            |lambda: Value,
             q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
             q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
             cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT]| {
                let mut scaled_q_prev = *q_prev;
                let mut scaled_q_prev_prev = *q_prev_prev;
                let mut scaled_cq_prev = *cq_prev;
                scaled_q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= lambda;
                scaled_q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= lambda;
                scaled_q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= lambda;
                scaled_q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= lambda;
                scaled_cq_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= lambda;
                scaled_cq_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= lambda;
                (scaled_q_prev, scaled_q_prev_prev, scaled_cq_prev)
            };
        let scale_excess_phase_seed =
            |lambda: Value, seed_internal: &[Value; BJT_INTERNAL_STATE_DIM]| {
                let mut scaled_seed = *seed_internal;
                scaled_seed[BJT_DELAY_XF1_STATE_INDEX] *= lambda;
                scaled_seed[BJT_DELAY_XF2_STATE_INDEX] *= lambda;
                scaled_seed
            };
        let (target_q_prev, target_q_prev_prev, target_cq_prev) =
            scale_excess_phase_history(1.0, q_prev, q_prev_prev, cq_prev);

        let mut base_bjt = bjt.clone();
        base_bjt.td = 0.0;
        let (base_q_prev, base_q_prev_prev, base_cq_prev) =
            scale_excess_phase_history(0.0, q_prev, q_prev_prev, cq_prev);
        let base_seed =
            seed_internal.map(|seed_internal| scale_excess_phase_seed(0.0, seed_internal));
        let live_base_seed = base_bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let mut current_result = Self::solve_vbic_dynamic_snapshot_best_effort(
            &base_bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &base_q_prev,
            &base_q_prev_prev,
            &base_cq_prev,
            base_seed.as_ref(),
        );
        let live_base_result = Self::solve_vbic_dynamic_snapshot_best_effort(
            &base_bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &base_q_prev,
            &base_q_prev_prev,
            &base_cq_prev,
            Some(&live_base_seed),
        );
        current_result = Self::choose_preferred_vbic_best_effort_result(
            current_result,
            live_base_result,
            |result| {
                Self::vbic_dynamic_snapshot_solution_is_acceptable(
                    &result.1,
                    &result.0.reduction.external_voltages,
                    &result.0.reduction.internal_voltages,
                )
            },
        );
        let mut current_result = current_result?;
        let mut lambda = 0.0_f64;
        let mut step = 0.25_f64;
        while lambda < 1.0 - 1e-15 {
            let candidate_lambda = (lambda + step).min(1.0);
            let mut stepped_bjt = bjt.clone();
            stepped_bjt.td = target_td * candidate_lambda;
            let (candidate_q_prev, candidate_q_prev_prev, candidate_cq_prev) =
                scale_excess_phase_history(candidate_lambda, q_prev, q_prev_prev, cq_prev);
            let previous_internal = current_result.0.reduction.internal_voltages;
            let live_candidate_seed = stepped_bjt.limit_vbic_dynamic_internal_state_to_previous(
                stepped_bjt.dynamic_internal_state_seed(vc, vb, ve, vs),
                previous_internal,
            );
            let mut candidate_result = Self::solve_vbic_dynamic_snapshot_best_effort(
                &stepped_bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &candidate_q_prev,
                &candidate_q_prev_prev,
                &candidate_cq_prev,
                Some(&previous_internal),
            );
            let live_candidate_result = Self::solve_vbic_dynamic_snapshot_best_effort(
                &stepped_bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &candidate_q_prev,
                &candidate_q_prev_prev,
                &candidate_cq_prev,
                Some(&live_candidate_seed),
            );
            candidate_result = Self::choose_preferred_vbic_best_effort_result(
                candidate_result,
                live_candidate_result,
                |result| {
                    Self::vbic_homotopy_candidate_is_acceptable(
                        &stepped_bjt,
                        [vc, vb, ve, vs],
                        previous_internal,
                        &result.0,
                        &result.1,
                    )
                },
            );
            let Some(candidate_result) = candidate_result else {
                if step <= Self::VBIC_HOMOTOPY_MIN_LAMBDA_STEP {
                    return None;
                }
                step *= 0.5;
                continue;
            };
            if !Self::vbic_homotopy_candidate_is_acceptable(
                &stepped_bjt,
                [vc, vb, ve, vs],
                previous_internal,
                &candidate_result.0,
                &candidate_result.1,
            ) {
                if step <= Self::VBIC_HOMOTOPY_MIN_LAMBDA_STEP {
                    return None;
                }
                step *= 0.5;
                continue;
            }
            current_result = candidate_result;
            lambda = candidate_lambda;
            if lambda < 1.0 - 1e-15 {
                let target_internal = current_result.0.reduction.internal_voltages;
                let live_target_seed = bjt.limit_vbic_dynamic_internal_state_to_previous(
                    bjt.dynamic_internal_state_seed(vc, vb, ve, vs),
                    target_internal,
                );
                let target_result = Self::choose_preferred_vbic_best_effort_result(
                    Self::solve_vbic_dynamic_snapshot_best_effort(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        &target_q_prev,
                        &target_q_prev_prev,
                        &target_cq_prev,
                        Some(&target_internal),
                    ),
                    Self::solve_vbic_dynamic_snapshot_best_effort(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        &target_q_prev,
                        &target_q_prev_prev,
                        &target_cq_prev,
                        Some(&live_target_seed),
                    ),
                    |result| {
                        Self::vbic_homotopy_candidate_is_acceptable(
                            bjt,
                            [vc, vb, ve, vs],
                            target_internal,
                            &result.0,
                            &result.1,
                        )
                    },
                );
                if let Some(target_result) = target_result
                    && Self::vbic_homotopy_candidate_is_acceptable(
                        bjt,
                        [vc, vb, ve, vs],
                        target_internal,
                        &target_result.0,
                        &target_result.1,
                    )
                {
                    current_result = target_result;
                    break;
                }
            }
            step = (step * 2.0).min((1.0 - lambda).max(0.0)).max(1e-6);
        }

        Some((current_result.0, current_result.1, current_result.2))
    }

    #[inline]
    fn solve_vbic_dynamic_snapshot_direct(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        let (snapshot, linearization, base_static_g, _residual_norm) =
            Self::solve_vbic_dynamic_snapshot_best_effort(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )?;
        Self::vbic_dynamic_snapshot_solution_is_acceptable(
            &linearization,
            &snapshot.reduction.external_voltages,
            &snapshot.reduction.internal_voltages,
        )
        .then_some((snapshot, linearization, base_static_g))
    }

    #[inline]
    fn solve_vbic_dynamic_snapshot_best_effort(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<VbicBestEffortSolve> {
        let mut seeded_snapshot = if let Some(seed_internal) = seed_internal {
            bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, *seed_internal)
        } else {
            bjt.charge_snapshot(vc, vb, ve, vs)
        };
        Self::rebalance_vbic_dynamic_thermal_state(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            &mut seeded_snapshot,
        );
        let mut base_static_g = seeded_snapshot.reduction.g_reduced;
        let mut transient_linearization = Self::assemble_vbic_transient_linearization(
            bjt,
            &seeded_snapshot,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        )?;
        let initial_residual = Self::vbic_internal_equation_residual(
            &transient_linearization,
            &seeded_snapshot.reduction.external_voltages,
            &seeded_snapshot.reduction.internal_voltages,
        );
        let polished_initial_state = Self::refine_vbic_dynamic_static_core_with_fixed_delay(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            (
                seeded_snapshot,
                transient_linearization,
                base_static_g,
                initial_residual,
                initial_residual
                    .iter()
                    .fold(0.0_f64, |max_norm, value| max_norm.max(value.abs())),
            ),
            6,
        );
        seeded_snapshot = polished_initial_state.0;
        transient_linearization = polished_initial_state.1;
        base_static_g = polished_initial_state.2;
        let mut current_residual_norm = polished_initial_state.4;
        let mut current_residual_objective =
            Self::vbic_internal_equation_residual_objective(&polished_initial_state.3);

        let max_refinements = if bjt.has_vbic_self_heating() {
            96
        } else if bjt.uses_vbic_dynamic_charges() {
            64
        } else {
            32
        };
        for iteration in 0..max_refinements {
            if current_residual_norm < 1e-14 {
                break;
            }
            let current_internal = seeded_snapshot.reduction.internal_voltages;
            let solved_internal = Self::solve_vbic_internal_state_from_linearization(
                &transient_linearization,
                &seeded_snapshot.reduction.external_voltages,
            )?;
            let target_internal = Self::step_limit_vbic_dynamic_internal_target(
                current_internal,
                solved_internal,
                iteration,
                current_residual_norm,
            );
            if !target_internal.iter().all(|value| value.is_finite()) {
                break;
            }

            let max_delay_state = target_internal[BJT_DELAY_XF1_STATE_INDEX]
                .abs()
                .max(target_internal[BJT_DELAY_XF2_STATE_INDEX].abs());
            static VBIC_INTERNAL_SOLVE_LOG_COUNT: std::sync::atomic::AtomicUsize =
                std::sync::atomic::AtomicUsize::new(0);
            let internal_log_count =
                VBIC_INTERNAL_SOLVE_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if max_delay_state > 1.0 && internal_log_count < 8 {
                log::warn!(
                    "VBIC internal solve {} ext={:?} seed_xf=({:.3e}, {:.3e}) solved_xf=({:.3e}, {:.3e}) z_xf=({:.3e}, {:.3e}) g_xf1={:?} g_xf2={:?}",
                    bjt.name,
                    seeded_snapshot.reduction.external_voltages,
                    seeded_snapshot.reduction.internal_voltages[BJT_DELAY_XF1_STATE_INDEX],
                    seeded_snapshot.reduction.internal_voltages[BJT_DELAY_XF2_STATE_INDEX],
                    target_internal[BJT_DELAY_XF1_STATE_INDEX],
                    target_internal[BJT_DELAY_XF2_STATE_INDEX],
                    transient_linearization.z_i[BJT_DELAY_XF1_STATE_INDEX],
                    transient_linearization.z_i[BJT_DELAY_XF2_STATE_INDEX],
                    transient_linearization.g_ii[BJT_DELAY_XF1_STATE_INDEX],
                    transient_linearization.g_ii[BJT_DELAY_XF2_STATE_INDEX],
                );
            }

            let max_delta = target_internal
                .iter()
                .zip(current_internal.iter())
                .map(|(solved, current)| (solved - current).abs())
                .fold(0.0, Value::max);
            if max_delta < 1e-12 {
                break;
            }

            let Some((
                solved_snapshot,
                solved_linearization,
                solved_static_g,
                solved_residual,
                solved_residual_norm,
            )) = Self::improve_vbic_dynamic_internal_state_toward_target(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                current_internal,
                current_residual_norm,
                current_residual_objective,
                target_internal,
                current_internal,
                12,
            )
            else {
                break;
            };

            let polished_state = Self::refine_vbic_dynamic_static_core_with_fixed_delay(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                (
                    solved_snapshot,
                    solved_linearization,
                    solved_static_g,
                    solved_residual,
                    solved_residual_norm,
                ),
                4,
            );
            base_static_g = polished_state.2;
            seeded_snapshot = polished_state.0;
            transient_linearization = polished_state.1;
            current_residual_norm = polished_state.4;
            current_residual_objective =
                Self::vbic_internal_equation_residual_objective(&polished_state.3);
        }

        if current_residual_norm > 1e-8 {
            let mut current_state = Self::evaluate_vbic_dynamic_internal_state(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seeded_snapshot.reduction.internal_voltages,
            )
            .unwrap_or((
                seeded_snapshot.clone(),
                transient_linearization.clone(),
                base_static_g,
                Self::vbic_internal_equation_residual(
                    &transient_linearization,
                    &seeded_snapshot.reduction.external_voltages,
                    &seeded_snapshot.reduction.internal_voltages,
                ),
                current_residual_norm,
            ));
            let mut current_residual_objective =
                Self::vbic_dynamic_state_evaluation_residual_objective(&current_state);
            for iteration in 0..16 {
                if current_state.4 < 1e-10 {
                    break;
                }

                let current_internal = current_state.0.reduction.internal_voltages;
                let current_external = current_state.0.reduction.external_voltages;
                let mut next_state = Self::solve_vbic_internal_state_from_linearization(
                    &current_state.1,
                    &current_external,
                )
                .and_then(|target_internal| {
                    let target_internal = Self::step_limit_vbic_dynamic_internal_target(
                        current_internal,
                        target_internal,
                        iteration,
                        current_state.4,
                    );
                    Self::improve_vbic_dynamic_internal_state_toward_target(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        current_internal,
                        current_state.4,
                        current_residual_objective,
                        target_internal,
                        current_internal,
                        12,
                    )
                    .map(|candidate_state| {
                        Self::refine_vbic_dynamic_static_core_with_fixed_delay(
                            bjt,
                            vc,
                            vb,
                            ve,
                            vs,
                            method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            candidate_state,
                            4,
                        )
                    })
                });

                let mut jacobian = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
                for col in 0..BJT_INTERNAL_STATE_DIM {
                    let base_value = current_internal[col];
                    let step = match col {
                        BJT_DELAY_XF1_STATE_INDEX | BJT_DELAY_XF2_STATE_INDEX => {
                            (base_value.abs() * 1e-3).max(1e-9)
                        }
                        BJT_THERMAL_STATE_INDEX => (base_value.abs() * 1e-4).max(1e-6),
                        _ => (base_value.abs() * 1e-6).max(1e-7),
                    };

                    let mut plus_internal = current_internal;
                    plus_internal[col] = base_value + step;
                    if col == BJT_THERMAL_STATE_INDEX {
                        plus_internal[col] = plus_internal[col].max(bjt.minimum_thermal_rise());
                    }
                    let Some(plus_state) = Self::evaluate_vbic_dynamic_internal_state(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        plus_internal,
                    ) else {
                        continue;
                    };

                    let use_central = col != BJT_THERMAL_STATE_INDEX
                        || base_value - step >= bjt.minimum_thermal_rise();
                    if use_central {
                        let mut minus_internal = current_internal;
                        minus_internal[col] = base_value - step;
                        if col == BJT_THERMAL_STATE_INDEX {
                            minus_internal[col] =
                                minus_internal[col].max(bjt.minimum_thermal_rise());
                        }
                        let Some(minus_state) = Self::evaluate_vbic_dynamic_internal_state(
                            bjt,
                            vc,
                            vb,
                            ve,
                            vs,
                            method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            minus_internal,
                        ) else {
                            continue;
                        };
                        let denom = plus_internal[col] - minus_internal[col];
                        if denom.abs() <= 0.0 {
                            continue;
                        }
                        for row in 0..BJT_INTERNAL_STATE_DIM {
                            jacobian[row][col] = (plus_state.3[row] - minus_state.3[row]) / denom;
                        }
                    } else {
                        let denom = plus_internal[col] - current_internal[col];
                        if denom.abs() <= 0.0 {
                            continue;
                        }
                        for row in 0..BJT_INTERNAL_STATE_DIM {
                            jacobian[row][col] = (plus_state.3[row] - current_state.3[row]) / denom;
                        }
                    }
                }

                let rhs = current_state.3.map(|value| -value);
                let Some((lu_internal, pivots_internal)) =
                    Self::lu_decompose_small_dense_real(&jacobian, BJT_INTERNAL_STATE_DIM)
                else {
                    break;
                };
                let Some(delta) = Self::lu_solve_small_dense_real(
                    &lu_internal,
                    &pivots_internal,
                    &rhs,
                    BJT_INTERNAL_STATE_DIM,
                ) else {
                    break;
                };
                let max_raw_delta = delta
                    .iter()
                    .fold(0.0_f64, |max_delta, value| max_delta.max(value.abs()));
                if max_raw_delta < 1e-12 {
                    break;
                }
                let mut target_internal = current_internal;
                for idx in 0..BJT_INTERNAL_STATE_DIM {
                    target_internal[idx] = current_internal[idx] + delta[idx];
                }
                target_internal = Self::step_limit_vbic_dynamic_internal_target(
                    current_internal,
                    target_internal,
                    iteration,
                    current_state.4,
                );
                if let Some(candidate_state) =
                    Self::improve_vbic_dynamic_internal_state_toward_target(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        current_internal,
                        current_state.4,
                        current_residual_objective,
                        target_internal,
                        current_internal,
                        12,
                    )
                    .map(|candidate_state| {
                        Self::refine_vbic_dynamic_static_core_with_fixed_delay(
                            bjt,
                            vc,
                            vb,
                            ve,
                            vs,
                            method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            candidate_state,
                            4,
                        )
                    })
                {
                    let candidate_objective =
                        Self::vbic_dynamic_state_evaluation_residual_objective(&candidate_state);
                    if next_state.as_ref().map_or(true, |best_state| {
                        candidate_objective + 1e-15
                            < Self::vbic_dynamic_state_evaluation_residual_objective(best_state)
                    }) {
                        next_state = Some(candidate_state);
                    }
                }

                let mut normal_matrix = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
                let mut gradient = [0.0; BJT_INTERNAL_STATE_DIM];
                for row in 0..BJT_INTERNAL_STATE_DIM {
                    for col in 0..BJT_INTERNAL_STATE_DIM {
                        let mut value = 0.0;
                        for inner in 0..BJT_INTERNAL_STATE_DIM {
                            value += jacobian[inner][row] * jacobian[inner][col];
                        }
                        normal_matrix[row][col] = value;
                    }
                    gradient[row] = (0..BJT_INTERNAL_STATE_DIM)
                        .map(|inner| jacobian[inner][row] * current_state.3[inner])
                        .sum();
                }
                let lm_diag_scale = (0..BJT_INTERNAL_STATE_DIM)
                    .map(|idx| normal_matrix[idx][idx].abs())
                    .fold(1.0_f64, Value::max);
                for lambda_scale in [1e-10, 1e-8, 1e-6, 1e-4, 1e-2, 1.0, 1e2] {
                    let mut damped_normal = normal_matrix;
                    let lambda = lm_diag_scale * lambda_scale;
                    for idx in 0..BJT_INTERNAL_STATE_DIM {
                        damped_normal[idx][idx] += lambda;
                    }
                    let Some((lu_internal, pivots_internal)) =
                        Self::lu_decompose_small_dense_real(&damped_normal, BJT_INTERNAL_STATE_DIM)
                    else {
                        continue;
                    };
                    let rhs = gradient.map(|value| -value);
                    let Some(delta) = Self::lu_solve_small_dense_real(
                        &lu_internal,
                        &pivots_internal,
                        &rhs,
                        BJT_INTERNAL_STATE_DIM,
                    ) else {
                        continue;
                    };
                    let max_lm_delta = delta
                        .iter()
                        .fold(0.0_f64, |max_delta, value| max_delta.max(value.abs()));
                    if max_lm_delta < 1e-12 {
                        continue;
                    }
                    let mut target_internal = current_internal;
                    for idx in 0..BJT_INTERNAL_STATE_DIM {
                        target_internal[idx] = current_internal[idx] + delta[idx];
                    }
                    target_internal = Self::step_limit_vbic_dynamic_internal_target(
                        current_internal,
                        target_internal,
                        iteration,
                        current_state.4,
                    );
                    if let Some(candidate_state) =
                        Self::improve_vbic_dynamic_internal_state_toward_target(
                            bjt,
                            vc,
                            vb,
                            ve,
                            vs,
                            method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            current_internal,
                            current_state.4,
                            current_residual_objective,
                            target_internal,
                            current_internal,
                            12,
                        )
                        .map(|candidate_state| {
                            Self::refine_vbic_dynamic_static_core_with_fixed_delay(
                                bjt,
                                vc,
                                vb,
                                ve,
                                vs,
                                method,
                                trap_order,
                                dt,
                                q_prev,
                                q_prev_prev,
                                cq_prev,
                                candidate_state,
                                4,
                            )
                        })
                    {
                        let candidate_objective =
                            Self::vbic_dynamic_state_evaluation_residual_objective(
                                &candidate_state,
                            );
                        if next_state.as_ref().map_or(true, |best_state| {
                            candidate_objective + 1e-15
                                < Self::vbic_dynamic_state_evaluation_residual_objective(best_state)
                        }) {
                            next_state = Some(candidate_state);
                        }
                    }
                }

                let Some(next_state) = next_state else {
                    break;
                };

                current_state = next_state;
                current_residual_objective =
                    Self::vbic_dynamic_state_evaluation_residual_objective(&current_state);
            }

            seeded_snapshot = current_state.0;
            transient_linearization = current_state.1;
            base_static_g = current_state.2;
            current_residual_norm = current_state.4;
        }

        Some((
            seeded_snapshot,
            transient_linearization,
            base_static_g,
            current_residual_norm,
        ))
    }

    /// ngspice-style hidden-state delta check used for device-local bypass
    /// diagnostics. In ngspice, these `hat` comparisons decide whether the
    /// device can reuse the previous linearization; they are not a standalone
    /// Newton acceptance gate after the internal nodes have been reduced out of
    /// the global system.
    #[inline]
    fn vbic_snapshot_convergence_met(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_snapshot: &BjtChargeSnapshot,
        criteria: NonlinearConvergenceCriteria,
    ) -> bool {
        let previous = bjt.vbic_transient_convergence_state_for_snapshot(
            previous_external[0],
            previous_external[1],
            previous_external[2],
            previous_external[3],
            previous_snapshot,
        );
        let current = bjt.vbic_transient_convergence_state_for_snapshot(
            current_external[0],
            current_external[1],
            current_external[2],
            current_external[3],
            current_snapshot,
        );
        let reltol = criteria.relative_tolerance();
        let voltage_tol = criteria.voltage_tolerance();
        let current_tol = criteria.current_tolerance();

        let voltages_converged = current.voltages.iter().zip(previous.voltages.iter()).all(
            |(current_voltage, previous_voltage)| {
                let diff = (current_voltage - previous_voltage).abs();
                let tol = reltol * current_voltage.abs().max(previous_voltage.abs()) + voltage_tol;
                diff <= tol
            },
        );
        if !voltages_converged {
            return false;
        }

        let mut delta_internal = [0.0; BJT_INTERNAL_STATE_DIM];
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            delta_internal[idx] = current_snapshot.reduction.internal_voltages[idx]
                - previous_snapshot.reduction.internal_voltages[idx];
        }

        (0..VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT).all(|branch_idx| {
            // Mirror ngspice's VBIC load-time bypass check: compare the full
            // predicted branch current against the candidate branch current,
            // including the hidden excess-phase xf2 contribution in iciei.
            // Excluding that term can accept a stale delayed-transport state
            // even when the candidate misses the device-local predictor tolerances.
            let predicted = previous.currents[branch_idx]
                + previous.d_currents_d_internal[branch_idx]
                    .iter()
                    .zip(delta_internal.iter())
                    .enumerate()
                    .filter(|(idx, _)| *idx != BJT_THERMAL_STATE_INDEX)
                    .map(|(_, (derivative, delta))| derivative * delta)
                    .sum::<Value>();
            let actual = current.currents[branch_idx];
            let tol = reltol * predicted.abs().max(actual.abs()) + current_tol;
            (predicted - actual).abs() <= tol
        })
    }

    #[inline]
    fn vbic_local_candidate_is_acceptable(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        candidate_snapshot: &BjtChargeSnapshot,
        candidate_linearization: &VbicTransientLinearization,
    ) -> bool {
        if Self::vbic_dynamic_snapshot_solution_is_acceptable(
            candidate_linearization,
            &candidate_snapshot.reduction.external_voltages,
            &candidate_snapshot.reduction.internal_voltages,
        ) {
            return true;
        }

        // ngspice's VBIC path ultimately accepts or bypasses local updates based on
        // branch/voltage predictor tolerances (`*_hat` checks), not on a separate
        // reduced hidden-state residual. During our local continuation fallback, a
        // candidate that meets those ngspice-style device tolerances should be
        // accepted even when the reduced internal equations are stricter.
        Self::vbic_snapshot_convergence_met(
            bjt,
            previous_external,
            previous_snapshot,
            candidate_snapshot.reduction.external_voltages,
            candidate_snapshot,
            NonlinearConvergenceCriteria::default(),
        )
    }

    #[inline]
    fn vbic_continuation_candidate_is_acceptable(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        candidate_snapshot: &BjtChargeSnapshot,
        candidate_linearization: &VbicTransientLinearization,
    ) -> bool {
        Self::vbic_local_candidate_is_acceptable(
            bjt,
            previous_external,
            previous_snapshot,
            candidate_snapshot,
            candidate_linearization,
        )
    }

    #[inline]
    fn vbic_homotopy_candidate_is_acceptable(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_internal: [Value; BJT_INTERNAL_STATE_DIM],
        candidate_snapshot: &BjtChargeSnapshot,
        candidate_linearization: &VbicTransientLinearization,
    ) -> bool {
        let previous_snapshot = bjt.charge_snapshot_for_dynamic_state(
            external[0],
            external[1],
            external[2],
            external[3],
            previous_internal,
        );
        Self::vbic_local_candidate_is_acceptable(
            bjt,
            external,
            &previous_snapshot,
            candidate_snapshot,
            candidate_linearization,
        )
    }

    #[inline]
    fn solve_vbic_dynamic_snapshot_for_continuation_step(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        seed_internal: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
    ) -> Option<(
        crate::device::semiconductor::BjtChargeSnapshot,
        VbicTransientLinearization,
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    )> {
        let previous_internal = previous_snapshot.reduction.internal_voltages;
        let limited_live_seed = bjt.limit_vbic_dynamic_internal_state_to_previous(
            bjt.dynamic_internal_state_seed(vc, vb, ve, vs),
            previous_internal,
        );
        let seeded_result = Self::solve_vbic_dynamic_snapshot_best_effort(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            seed_internal,
        );
        let anchored_result = seed_internal
            .filter(|seed| {
                seed.iter()
                    .zip(previous_internal.iter())
                    .any(|(lhs, rhs)| (*lhs - *rhs).abs() > 1e-18)
            })
            .map(|_| {
                Self::solve_vbic_dynamic_snapshot_best_effort(
                    bjt,
                    vc,
                    vb,
                    ve,
                    vs,
                    method,
                    trap_order,
                    dt,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                    Some(&previous_internal),
                )
            })
            .unwrap_or(None);
        let live_result = seed_internal
            .filter(|seed| {
                seed.iter()
                    .zip(limited_live_seed.iter())
                    .any(|(lhs, rhs)| (*lhs - *rhs).abs() > 1e-18)
            })
            .map(|_| {
                Self::solve_vbic_dynamic_snapshot_best_effort(
                    bjt,
                    vc,
                    vb,
                    ve,
                    vs,
                    method,
                    trap_order,
                    dt,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                    Some(&limited_live_seed),
                )
            })
            .unwrap_or(None);
        let mut preferred_result = Self::choose_preferred_vbic_best_effort_result(
            seeded_result,
            anchored_result,
            |result| {
                Self::vbic_continuation_candidate_is_acceptable(
                    bjt,
                    previous_external,
                    previous_snapshot,
                    &result.0,
                    &result.1,
                )
            },
        );
        preferred_result = Self::choose_preferred_vbic_best_effort_result(
            preferred_result,
            live_result,
            |result| {
                Self::vbic_continuation_candidate_is_acceptable(
                    bjt,
                    previous_external,
                    previous_snapshot,
                    &result.0,
                    &result.1,
                )
            },
        );
        if let Some((snapshot, linearization, base_static_g, _residual_norm)) = preferred_result
            && Self::vbic_continuation_candidate_is_acceptable(
                bjt,
                previous_external,
                previous_snapshot,
                &snapshot,
                &linearization,
            )
        {
            return Some((snapshot, linearization, base_static_g));
        }

        Self::solve_vbic_dynamic_snapshot_with_excess_phase_homotopy(
            bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            seed_internal,
        )
        .or_else(|| {
            Self::solve_vbic_dynamic_snapshot_with_collector_substrate_charge_homotopy(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                seed_internal,
            )
        })
    }

    #[inline]
    fn vbic_excess_phase_device_convergence_met(
        &self,
        circuit: &crate::circuit::Circuit,
        previous_solution: &[Value],
        current_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
    ) -> bool {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let criteria = self.device_convergence_criteria();

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if !bjt.uses_vbic_dynamic_charges() || bjt.td <= 0.0 {
                continue;
            }

            let previous_external = [
                Self::node_voltage(previous_solution, bjt.node_collector),
                Self::node_voltage(previous_solution, bjt.node_base),
                Self::node_voltage(previous_solution, bjt.node_emitter),
                Self::node_voltage(previous_solution, bjt.node_substrate),
            ];
            let current_external = [
                Self::node_voltage(current_solution, bjt.node_collector),
                Self::node_voltage(current_solution, bjt.node_base),
                Self::node_voltage(current_solution, bjt.node_emitter),
                Self::node_voltage(current_solution, bjt.node_substrate),
            ];

            let previous_snapshot = vbic_snapshot_cache
                .get(idx)
                .copied()
                .flatten()
                .filter(|snapshot| {
                    snapshot
                        .reduction
                        .external_voltages
                        .iter()
                        .zip(previous_external.iter())
                        .all(|(cached, expected)| (*cached - *expected).abs() <= 1e-18)
                })
                .or_else(|| {
                    let seed_internal =
                        Self::vbic_dynamic_internal_seed_from_history_with_linear_history(
                            bjt,
                            previous_external[0],
                            previous_external[1],
                            previous_external[2],
                            previous_external[3],
                            history.dynamic_internal_prev.get(idx),
                            history.dynamic_internal_prev_prev.get(idx),
                            history.dynamic_linear_prev.get(idx),
                            history.dynamic_linear_prev_prev.get(idx),
                            dt,
                            history.accepted_dt_prev,
                        );
                    Self::solve_vbic_dynamic_snapshot(
                        bjt,
                        previous_external[0],
                        previous_external[1],
                        previous_external[2],
                        previous_external[3],
                        effective_method,
                        trap_order,
                        dt,
                        &history.charge_q_prev[idx],
                        &history.charge_q_prev_prev[idx],
                        &history.charge_cq_prev[idx],
                        Some(&seed_internal),
                    )
                    .map(|(snapshot, _, _)| snapshot)
                });
            let Some(previous_snapshot) = previous_snapshot else {
                return false;
            };

            let current_snapshot = Self::solve_vbic_dynamic_snapshot(
                bjt,
                current_external[0],
                current_external[1],
                current_external[2],
                current_external[3],
                effective_method,
                trap_order,
                dt,
                &history.charge_q_prev[idx],
                &history.charge_q_prev_prev[idx],
                &history.charge_cq_prev[idx],
                Some(&previous_snapshot.reduction.internal_voltages),
            )
            .map(|(snapshot, _, _)| snapshot);
            let Some(current_snapshot) = current_snapshot else {
                return false;
            };

            if !Self::vbic_snapshot_convergence_met(
                bjt,
                previous_external,
                &previous_snapshot,
                current_external,
                &current_snapshot,
                criteria,
            ) {
                return false;
            }
        }

        true
    }

    #[inline]
    fn transient_static_device_convergence_met(
        &self,
        circuit: &crate::circuit::Circuit,
        has_vbic_excess_phase: bool,
    ) -> bool {
        let criteria = self.device_convergence_criteria();

        circuit.diodes.all_converged(criteria)
            && circuit.mosfets.all_converged(criteria)
            && circuit.jfets.iter().all(|jfet| jfet.is_converged(criteria))
            && circuit.vswitches.iter().all(|sw| sw.is_converged(criteria))
            && circuit.iswitches.iter().all(|sw| sw.is_converged(criteria))
            && circuit.xspice_converged(criteria.voltage_tolerance())
            && circuit.bjts.devices.iter().all(|bjt| {
                if has_vbic_excess_phase && bjt.uses_vbic_dynamic_charges() && bjt.td > 0.0 {
                    true
                } else {
                    bjt.is_converged(criteria)
                }
            })
    }

    #[inline]
    fn vbic_snapshot_matches_external_bias(
        snapshot: &BjtChargeSnapshot,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
        voltage_abstol: Value,
        reltol: Value,
    ) -> bool {
        Self::check_voltage_convergence_with_tolerances(
            &snapshot.reduction.external_voltages,
            external,
            voltage_abstol,
            reltol,
        )
    }

    #[inline]
    fn vbic_snapshot_matches_external_bias_exact(
        snapshot: &BjtChargeSnapshot,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) -> bool {
        snapshot
            .reduction
            .external_voltages
            .iter()
            .zip(external.iter())
            .all(|(cached, expected)| (*cached - *expected).abs() <= 1e-18)
    }

    #[inline]
    fn resolve_vbic_snapshot_for_external_bias(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        history_internal_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_internal_prev_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        previous_dt: Value,
        cached_snapshot: Option<BjtChargeSnapshot>,
        voltage_abstol: Value,
        reltol: Value,
    ) -> Option<BjtChargeSnapshot> {
        Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
            bjt,
            external,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            history_internal_prev,
            history_internal_prev_prev,
            None,
            None,
            previous_dt,
            cached_snapshot,
            VbicCachedSnapshotReuse::SeedOnly,
            voltage_abstol,
            reltol,
        )
    }

    #[inline]
    fn vbic_external_from_linear_history(
        bjt: &crate::device::Bjt,
        internal: &[Value; BJT_INTERNAL_STATE_DIM],
        linear: &VbicPredictorLinearBranchState,
    ) -> [Value; BJT_EXTERNAL_STATE_DIM] {
        let polarity = match bjt.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        };
        [
            internal[BJT_VCX_STATE_INDEX] + polarity * linear.vrcx,
            internal[BJT_VBX_STATE_INDEX] + polarity * linear.vrbx,
            internal[BJT_VEI_STATE_INDEX] + polarity * linear.vre,
            internal[BJT_VSI_STATE_INDEX] + polarity * linear.vrs,
        ]
    }

    #[inline]
    fn continue_vbic_snapshot_to_external_bias(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> Option<BjtChargeSnapshot> {
        let current_snapshot = Self::solve_vbic_dynamic_snapshot(
            bjt,
            previous_external[0],
            previous_external[1],
            previous_external[2],
            previous_external[3],
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            Some(&previous_internal),
        )
        .map(|(snapshot, _, _)| snapshot)?;
        Self::continue_vbic_snapshot_to_external_bias_from_snapshot(
            bjt,
            current_snapshot,
            target_external,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        )
    }

    #[inline]
    fn continue_vbic_snapshot_to_external_bias_from_snapshot(
        bjt: &crate::device::Bjt,
        current_snapshot: BjtChargeSnapshot,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> Option<BjtChargeSnapshot> {
        let continuation_started_at = std::time::Instant::now();
        let previous_external = current_snapshot.reduction.external_voltages;
        let mut current_external = previous_external;
        let mut current_snapshot = current_snapshot;
        let mut previous_accepted_external: Option<[Value; BJT_EXTERNAL_STATE_DIM]> = None;
        let mut previous_accepted_internal: Option<[Value; BJT_INTERNAL_STATE_DIM]> = None;
        let lambda_for_external = |external: [Value; BJT_EXTERNAL_STATE_DIM]| {
            for idx in 0..BJT_EXTERNAL_STATE_DIM {
                let total_delta = target_external[idx] - previous_external[idx];
                if total_delta.abs() > 1e-30 {
                    return ((external[idx] - previous_external[idx]) / total_delta)
                        .clamp(0.0, 1.0);
                }
            }
            1.0
        };
        let mut lambda: Value = lambda_for_external(current_external);
        let mut step: Value = Self::vbic_continuation_step_from_snapshot(
            bjt,
            current_external,
            current_snapshot.reduction.internal_voltages,
            target_external,
        );
        let initial_step = step;
        let mut solve_attempts = 0usize;
        let mut accepted_steps = 0usize;
        let mut rejected_steps = 0usize;

        while lambda < 1.0 - 1e-15 {
            let next_external = [
                current_external[BJT_EXT_C_INDEX]
                    + (target_external[BJT_EXT_C_INDEX] - current_external[BJT_EXT_C_INDEX]) * step,
                current_external[BJT_EXT_B_INDEX]
                    + (target_external[BJT_EXT_B_INDEX] - current_external[BJT_EXT_B_INDEX]) * step,
                current_external[BJT_EXT_E_INDEX]
                    + (target_external[BJT_EXT_E_INDEX] - current_external[BJT_EXT_E_INDEX]) * step,
                current_external[BJT_EXT_S_INDEX]
                    + (target_external[BJT_EXT_S_INDEX] - current_external[BJT_EXT_S_INDEX]) * step,
            ];
            let candidate_lambda = lambda_for_external(next_external);
            let previous_internal = current_snapshot.reduction.internal_voltages;
            let seed_internal = Self::vbic_continuation_seed_from_accepted_path(
                bjt,
                previous_accepted_external,
                previous_accepted_internal,
                current_external,
                previous_internal,
                next_external,
            );
            let attempt_started_at = std::time::Instant::now();
            let next_snapshot_result = Self::solve_vbic_dynamic_snapshot_for_continuation_step(
                bjt,
                current_external,
                &current_snapshot,
                next_external[0],
                next_external[1],
                next_external[2],
                next_external[3],
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                Some(&seed_internal),
            );
            solve_attempts += 1;
            let attempt_elapsed = attempt_started_at.elapsed();
            let Some((next_snapshot, next_linearization, _)) = next_snapshot_result else {
                rejected_steps += 1;
                if attempt_elapsed >= std::time::Duration::from_millis(50) {
                    log::warn!(
                        "Slow VBIC continuation solve {} step={:.6e} lambda={:.6e}->{:.6e} ext=({:.6e}, {:.6e}, {:.6e}, {:.6e}) elapsed={:.3?} result=failed",
                        bjt.name,
                        step,
                        lambda,
                        candidate_lambda,
                        next_external[0],
                        next_external[1],
                        next_external[2],
                        next_external[3],
                        attempt_elapsed,
                    );
                }
                let min_step = Self::vbic_continuation_min_remaining_step_scale(
                    current_external,
                    target_external,
                );
                if step <= min_step * (1.0 + 1e-12) {
                    let continuation_elapsed = continuation_started_at.elapsed();
                    if continuation_elapsed >= std::time::Duration::from_millis(100) {
                        log::warn!(
                            "Slow VBIC continuation {} attempts={} accepts={} rejects={} initial_step={:.6e} final_lambda={:.6e} elapsed={:.3?} status=failed",
                            bjt.name,
                            solve_attempts,
                            accepted_steps,
                            rejected_steps,
                            initial_step,
                            lambda,
                            continuation_elapsed,
                        );
                    }
                    return None;
                }
                step = (step * 0.5).max(min_step);
                continue;
            };
            let residual_norm = Self::vbic_internal_equation_residual_norm(
                &next_linearization,
                &next_snapshot.reduction.external_voltages,
                &next_snapshot.reduction.internal_voltages,
            );
            let accepted_strictly = Self::vbic_dynamic_snapshot_solution_is_acceptable(
                &next_linearization,
                &next_snapshot.reduction.external_voltages,
                &next_snapshot.reduction.internal_voltages,
            );
            let accepted_by_predictor = !accepted_strictly
                && Self::vbic_continuation_candidate_is_acceptable(
                    bjt,
                    current_external,
                    &current_snapshot,
                    &next_snapshot,
                    &next_linearization,
                );
            // ngspice's VBIC path keeps advancing when the candidate satisfies
            // its local branch/voltage predictor tolerances, even if the
            // reduced hidden-state solve is stricter. Mirror that behavior for
            // intermediate continuation steps, then do one final strict polish
            // at the exact target bias before returning the snapshot.
            if accepted_strictly || accepted_by_predictor {
                accepted_steps += 1;
                if attempt_elapsed >= std::time::Duration::from_millis(50) {
                    log::warn!(
                        "Slow VBIC continuation solve {} step={:.6e} lambda={:.6e}->{:.6e} ext=({:.6e}, {:.6e}, {:.6e}, {:.6e}) elapsed={:.3?} result=accepted mode={} residual={:.6e}",
                        bjt.name,
                        step,
                        lambda,
                        candidate_lambda,
                        next_external[0],
                        next_external[1],
                        next_external[2],
                        next_external[3],
                        attempt_elapsed,
                        if accepted_strictly {
                            "strict"
                        } else {
                            "ngspice"
                        },
                        residual_norm,
                    );
                }
                previous_accepted_external = Some(current_external);
                previous_accepted_internal = Some(previous_internal);
                current_external = next_external;
                current_snapshot = next_snapshot;
                lambda = candidate_lambda;
                if lambda >= 1.0 - 1e-15 {
                    break;
                }
                let suggested_step = Self::vbic_continuation_step_from_snapshot(
                    bjt,
                    current_external,
                    current_snapshot.reduction.internal_voltages,
                    target_external,
                );
                step = Self::vbic_continuation_step_after_accept(
                    current_external,
                    target_external,
                    step,
                    suggested_step,
                );
                continue;
            }

            rejected_steps += 1;
            if attempt_elapsed >= std::time::Duration::from_millis(50) {
                log::warn!(
                    "Slow VBIC continuation solve {} step={:.6e} lambda={:.6e}->{:.6e} ext=({:.6e}, {:.6e}, {:.6e}, {:.6e}) elapsed={:.3?} result=rejected residual={:.6e}",
                    bjt.name,
                    step,
                    lambda,
                    candidate_lambda,
                    next_external[0],
                    next_external[1],
                    next_external[2],
                    next_external[3],
                    attempt_elapsed,
                    residual_norm,
                );
            }
            let min_step =
                Self::vbic_continuation_min_remaining_step_scale(current_external, target_external);
            if step <= min_step * (1.0 + 1e-12) {
                let continuation_elapsed = continuation_started_at.elapsed();
                if continuation_elapsed >= std::time::Duration::from_millis(100) {
                    log::warn!(
                        "Slow VBIC continuation {} attempts={} accepts={} rejects={} initial_step={:.6e} final_lambda={:.6e} elapsed={:.3?} status=failed",
                        bjt.name,
                        solve_attempts,
                        accepted_steps,
                        rejected_steps,
                        initial_step,
                        lambda,
                        continuation_elapsed,
                    );
                }
                return None;
            }
            step = (step * 0.5).max(min_step);
        }

        let continuation_elapsed = continuation_started_at.elapsed();
        if continuation_elapsed >= std::time::Duration::from_millis(100) {
            log::warn!(
                "Slow VBIC continuation {} attempts={} accepts={} rejects={} initial_step={:.6e} final_lambda={:.6e} elapsed={:.3?} status=ok",
                bjt.name,
                solve_attempts,
                accepted_steps,
                rejected_steps,
                initial_step,
                lambda,
                continuation_elapsed,
            );
        }
        let _ = current_external;
        Self::finalize_vbic_continuation_target_snapshot(
            bjt,
            current_snapshot,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        )
    }

    #[inline]
    fn vbic_continuation_min_remaining_step_scale(
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
    ) -> Value {
        let max_delta = current_external
            .iter()
            .zip(target_external.iter())
            .map(|(current, target)| (target - current).abs())
            .fold(0.0_f64, Value::max);
        if !max_delta.is_finite()
            || max_delta <= NonlinearConvergenceCriteria::default().voltage_tolerance()
        {
            return 1.0;
        }
        (NonlinearConvergenceCriteria::default().voltage_tolerance() / max_delta).clamp(1e-6, 1.0)
    }

    #[inline]
    fn vbic_continuation_step_from_snapshot(
        bjt: &crate::device::Bjt,
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
    ) -> Value {
        let min_scale =
            Self::vbic_continuation_min_remaining_step_scale(current_external, target_external);
        let current_static_internal =
            Self::vbic_static_internal_state_from_dynamic(current_internal);
        let suggested_scale = bjt
            .vbic_external_step_limit_scale_from_state(
                current_external,
                current_static_internal,
                target_external,
            )
            .unwrap_or(1.0);
        if !suggested_scale.is_finite() {
            return min_scale;
        }
        let min_trial_scale = if bjt.uses_vbic_dynamic_charges() {
            Self::VBIC_CONTINUATION_MIN_TRIAL_STEP
        } else {
            0.0
        };
        suggested_scale.max(min_trial_scale).clamp(min_scale, 1.0)
    }

    #[inline]
    fn vbic_continuation_step_after_accept(
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_step: Value,
        suggested_step: Value,
    ) -> Value {
        let min_step =
            Self::vbic_continuation_min_remaining_step_scale(current_external, target_external);
        (current_step * 2.0).min(suggested_step).max(min_step)
    }

    #[inline]
    fn finalize_vbic_continuation_target_snapshot(
        bjt: &crate::device::Bjt,
        snapshot: BjtChargeSnapshot,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
    ) -> Option<BjtChargeSnapshot> {
        let external = snapshot.reduction.external_voltages;
        let continuation_seed = snapshot.reduction.internal_voltages;
        let live_seed = bjt.dynamic_internal_state_seed(
            external[BJT_EXT_C_INDEX],
            external[BJT_EXT_B_INDEX],
            external[BJT_EXT_E_INDEX],
            external[BJT_EXT_S_INDEX],
        );
        // Once continuation has already advanced to the exact target bias via
        // ngspice-style local predictor acceptance, keep that accepted target
        // snapshot if a final strict hidden-state polish is unavailable. ngspice
        // does not require an extra reduced hidden-state solve before it can
        // proceed with the accepted local update.
        Self::solve_vbic_dynamic_snapshot(
            bjt,
            external[BJT_EXT_C_INDEX],
            external[BJT_EXT_B_INDEX],
            external[BJT_EXT_E_INDEX],
            external[BJT_EXT_S_INDEX],
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            Some(&continuation_seed),
        )
        .or_else(|| {
            Self::solve_vbic_dynamic_snapshot(
                bjt,
                external[BJT_EXT_C_INDEX],
                external[BJT_EXT_B_INDEX],
                external[BJT_EXT_E_INDEX],
                external[BJT_EXT_S_INDEX],
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                Some(&live_seed),
            )
        })
        .map(|(snapshot, _, _)| snapshot)
        .or(Some(snapshot))
    }

    #[inline]
    fn vbic_continuation_seed_from_snapshot(
        bjt: &crate::device::Bjt,
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        bjt.predict_vbic_dynamic_internal_state_from_previous_external_bias(
            current_external,
            current_internal,
            target_external,
        )
        .unwrap_or_else(|| {
            let mut live_seed = bjt.dynamic_internal_state_seed(
                target_external[BJT_EXT_C_INDEX],
                target_external[BJT_EXT_B_INDEX],
                target_external[BJT_EXT_E_INDEX],
                target_external[BJT_EXT_S_INDEX],
            );
            if bjt.uses_vbic_dynamic_charges() {
                live_seed[BJT_DELAY_XF1_STATE_INDEX] = current_internal[BJT_DELAY_XF1_STATE_INDEX];
                live_seed[BJT_DELAY_XF2_STATE_INDEX] = current_internal[BJT_DELAY_XF2_STATE_INDEX];
            }
            if bjt.has_vbic_self_heating() {
                live_seed[BJT_THERMAL_STATE_INDEX] = current_internal[BJT_THERMAL_STATE_INDEX];
            }
            bjt.limit_vbic_dynamic_internal_state_to_previous(live_seed, current_internal)
        })
    }

    #[inline]
    fn vbic_continuation_seed_from_accepted_path(
        bjt: &crate::device::Bjt,
        previous_external: Option<[Value; BJT_EXTERNAL_STATE_DIM]>,
        previous_internal: Option<[Value; BJT_INTERNAL_STATE_DIM]>,
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let mut seed = Self::vbic_continuation_seed_from_snapshot(
            bjt,
            current_external,
            current_internal,
            target_external,
        );
        let (Some(previous_external), Some(previous_internal)) =
            (previous_external, previous_internal)
        else {
            return seed;
        };

        let previous_step = current_external
            .iter()
            .zip(previous_external.iter())
            .map(|(current, previous)| (current - previous).abs())
            .fold(0.0_f64, Value::max);
        let proposed_step = target_external
            .iter()
            .zip(current_external.iter())
            .map(|(target, current)| (target - current).abs())
            .fold(0.0_f64, Value::max);
        if !previous_step.is_finite()
            || !proposed_step.is_finite()
            || previous_step <= 1e-30
            || proposed_step <= 1e-30
        {
            return seed;
        }

        // When the continuation loop has already shrunk the external step after
        // a rejection, the accepted-path predictor must be allowed to collapse
        // back toward the current accepted state. Keeping a positive floor here
        // forces an outsized internal extrapolation even for microscopic
        // follow-up steps and can starve the local reduced solve.
        let continuation_scale = (proposed_step / previous_step).clamp(0.0, 2.0);
        for idx in 0..BJT_THERMAL_STATE_INDEX {
            let path_predicted = current_internal[idx]
                + (current_internal[idx] - previous_internal[idx]) * continuation_scale;
            let path_delta = path_predicted - current_internal[idx];
            let snapshot_delta = seed[idx] - current_internal[idx];
            if path_delta.is_finite()
                && snapshot_delta.is_finite()
                && (snapshot_delta.abs() <= 1e-18
                    || path_delta.abs() <= 1e-18
                    || path_delta.signum() == snapshot_delta.signum())
                && path_delta.abs() > snapshot_delta.abs()
            {
                seed[idx] = path_predicted;
            }
        }
        if bjt.uses_vbic_dynamic_charges() && bjt.td > 0.0 {
            for idx in [BJT_DELAY_XF1_STATE_INDEX, BJT_DELAY_XF2_STATE_INDEX] {
                seed[idx] = current_internal[idx]
                    + (current_internal[idx] - previous_internal[idx]) * continuation_scale;
            }
        }
        if bjt.has_vbic_self_heating() {
            seed[BJT_THERMAL_STATE_INDEX] = (current_internal[BJT_THERMAL_STATE_INDEX]
                + (current_internal[BJT_THERMAL_STATE_INDEX]
                    - previous_internal[BJT_THERMAL_STATE_INDEX])
                    * continuation_scale)
                .max(bjt.minimum_thermal_rise());
        }

        bjt.limit_vbic_dynamic_internal_state_to_previous(seed, current_internal)
    }

    #[inline]
    fn vbic_static_internal_state_from_dynamic(
        current_internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> [Value; 8] {
        [
            current_internal[BJT_VCX_STATE_INDEX],
            current_internal[BJT_VCI_STATE_INDEX],
            current_internal[BJT_VBX_STATE_INDEX],
            current_internal[BJT_VBI_STATE_INDEX],
            current_internal[BJT_VEI_STATE_INDEX],
            current_internal[BJT_VBP_STATE_INDEX],
            current_internal[BJT_VSI_STATE_INDEX],
            current_internal[BJT_THERMAL_STATE_INDEX],
        ]
    }

    #[inline]
    fn resolve_vbic_snapshot_for_external_bias_with_linear_history(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        history_internal_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_internal_prev_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_linear_prev: Option<&VbicPredictorLinearBranchState>,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        previous_dt: Value,
        cached_snapshot: Option<BjtChargeSnapshot>,
        cache_reuse: VbicCachedSnapshotReuse,
        voltage_abstol: Value,
        reltol: Value,
    ) -> Option<BjtChargeSnapshot> {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let cached_snapshot_matches = |snapshot: &BjtChargeSnapshot| match cache_reuse {
            VbicCachedSnapshotReuse::SeedOnly => {
                Self::vbic_snapshot_matches_external_bias_exact(snapshot, &external)
            }
            VbicCachedSnapshotReuse::NewtonBypass => Self::vbic_snapshot_matches_external_bias(
                snapshot,
                &external,
                voltage_abstol,
                reltol,
            ),
        };
        if let Some(snapshot) = cached_snapshot.filter(cached_snapshot_matches) {
            return Some(snapshot);
        }
        if matches!(cache_reuse, VbicCachedSnapshotReuse::NewtonBypass)
            && let Some(cached_snapshot) = cached_snapshot.as_ref()
        {
            let candidate_snapshot = bjt.charge_snapshot_for_dynamic_state(
                external[0],
                external[1],
                external[2],
                external[3],
                cached_snapshot.reduction.internal_voltages,
            );
            if let Some(candidate_linearization) = Self::assemble_vbic_transient_linearization(
                bjt,
                &candidate_snapshot,
                effective_method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            ) && Self::vbic_local_candidate_is_acceptable(
                bjt,
                cached_snapshot.reduction.external_voltages,
                cached_snapshot,
                &candidate_snapshot,
                &candidate_linearization,
            ) {
                // Match ngspice-style device bypass: when the cached hidden state
                // remains locally acceptable at the new external bias, reuse it
                // directly instead of paying for another reduced hidden-state
                // solve during the same Newton timepoint.
                return Some(candidate_snapshot);
            }
        }

        let seed_internal = Self::vbic_dynamic_internal_seed_from_history_with_linear_history(
            bjt,
            external[0],
            external[1],
            external[2],
            external[3],
            history_internal_prev,
            history_internal_prev_prev,
            history_linear_prev,
            history_linear_prev_prev,
            dt,
            previous_dt,
        );
        let live_seed = bjt.limit_vbic_dynamic_internal_state_to_previous(
            bjt.dynamic_internal_state_seed(external[0], external[1], external[2], external[3]),
            seed_internal,
        );
        let cached_acceptance_reference =
            cached_snapshot.map(|snapshot| (snapshot.reduction.external_voltages, snapshot));
        let history_acceptance_reference = history_internal_prev.zip(history_linear_prev).map(
            |(history_internal_prev, history_linear_prev)| {
                let previous_external = Self::vbic_external_from_linear_history(
                    bjt,
                    history_internal_prev,
                    history_linear_prev,
                );
                let previous_snapshot = bjt.charge_snapshot_for_dynamic_state(
                    previous_external[0],
                    previous_external[1],
                    previous_external[2],
                    previous_external[3],
                    *history_internal_prev,
                );
                (previous_external, previous_snapshot)
            },
        );
        let bounded_snapshot_if_acceptable = |result: &VbicBestEffortSolve| {
            if !Self::vbic_dynamic_snapshot_best_effort_is_bounded(result) {
                return None;
            }

            let strict = Self::vbic_dynamic_snapshot_solution_is_acceptable(
                &result.1,
                &result.0.reduction.external_voltages,
                &result.0.reduction.internal_voltages,
            );
            let predictor_ok = cached_acceptance_reference
                .or(history_acceptance_reference)
                .map(|(previous_external, previous_snapshot)| {
                    Self::vbic_local_candidate_is_acceptable(
                        bjt,
                        previous_external,
                        &previous_snapshot,
                        &result.0,
                        &result.1,
                    )
                })
                .unwrap_or(false);

            (strict || predictor_ok).then_some(result.0)
        };
        // Match ngspice's predictor/load flow: first solve the current timepoint
        // directly from a predicted/live seed, and only fall back to local
        // continuation when that direct solve cannot produce a usable snapshot.
        if let Some((snapshot, _, _)) = Self::solve_vbic_dynamic_snapshot(
            bjt,
            external[0],
            external[1],
            external[2],
            external[3],
            effective_method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            Some(&seed_internal),
        ) {
            return Some(snapshot);
        }
        let mut bounded_best_effort = Self::choose_preferred_vbic_best_effort_result(
            Self::solve_vbic_dynamic_snapshot_best_effort(
                bjt,
                external[0],
                external[1],
                external[2],
                external[3],
                effective_method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                Some(&seed_internal),
            ),
            Some(&live_seed)
                .filter(|seed| {
                    seed.iter()
                        .zip(seed_internal.iter())
                        .any(|(lhs, rhs)| (*lhs - *rhs).abs() > 1e-18)
                })
                .and_then(|seed| {
                    Self::solve_vbic_dynamic_snapshot_best_effort(
                        bjt,
                        external[0],
                        external[1],
                        external[2],
                        external[3],
                        effective_method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        Some(seed),
                    )
                }),
            Self::vbic_dynamic_snapshot_best_effort_is_bounded,
        );

        if let Some(cached_snapshot) = cached_snapshot {
            let cached_internal = cached_snapshot.reduction.internal_voltages;
            if cached_internal
                .iter()
                .zip(seed_internal.iter())
                .any(|(cached, seeded)| (*cached - *seeded).abs() > 1e-15)
                && let Some((snapshot, _, _)) = Self::solve_vbic_dynamic_snapshot(
                    bjt,
                    external[0],
                    external[1],
                    external[2],
                    external[3],
                    effective_method,
                    trap_order,
                    dt,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                    Some(&cached_internal),
                )
            {
                return Some(snapshot);
            }
            bounded_best_effort = Self::choose_preferred_vbic_best_effort_result(
                bounded_best_effort,
                Some(cached_internal)
                    .filter(|seed| {
                        seed.iter()
                            .zip(seed_internal.iter())
                            .any(|(lhs, rhs)| (*lhs - *rhs).abs() > 1e-18)
                    })
                    .and_then(|seed| {
                        Self::solve_vbic_dynamic_snapshot_best_effort(
                            bjt,
                            external[0],
                            external[1],
                            external[2],
                            external[3],
                            effective_method,
                            trap_order,
                            dt,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                            Some(&seed),
                        )
                    }),
                Self::vbic_dynamic_snapshot_best_effort_is_bounded,
            );
            if let Some(result) = bounded_best_effort.as_ref()
                && let Some(snapshot) = bounded_snapshot_if_acceptable(result)
            {
                return Some(snapshot.clone());
            }
            if let Some(snapshot) = Self::continue_vbic_snapshot_to_external_bias_from_snapshot(
                bjt,
                cached_snapshot,
                external,
                effective_method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            ) {
                return Some(snapshot);
            }
        } else if let (Some(history_internal_prev), Some(history_linear_prev)) =
            (history_internal_prev, history_linear_prev)
        {
            if let Some(result) = bounded_best_effort.as_ref()
                && let Some(snapshot) = bounded_snapshot_if_acceptable(result)
            {
                return Some(snapshot.clone());
            }
            let previous_external = Self::vbic_external_from_linear_history(
                bjt,
                history_internal_prev,
                history_linear_prev,
            );
            if let Some(snapshot) = Self::continue_vbic_snapshot_to_external_bias(
                bjt,
                previous_external,
                *history_internal_prev,
                external,
                effective_method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            ) {
                return Some(snapshot);
            }
        }

        if let Some(result) = bounded_best_effort.as_ref()
            && let Some(snapshot) = bounded_snapshot_if_acceptable(result)
        {
            return Some(snapshot);
        }

        None
    }

    #[inline]
    fn vbic_runtime_snapshot_reuse_tolerances(
        voltage_abstol: Value,
        reltol: Value,
    ) -> (Value, Value) {
        (voltage_abstol, reltol)
    }

    #[inline]
    fn vbic_dynamic_internal_seed_from_history(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        history_internal_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_internal_prev_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        dt: Value,
        previous_dt: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        Self::vbic_dynamic_internal_seed_from_history_with_linear_history(
            bjt,
            vc,
            vb,
            ve,
            vs,
            history_internal_prev,
            history_internal_prev_prev,
            None,
            None,
            dt,
            previous_dt,
        )
    }

    #[inline]
    fn vbic_dynamic_internal_seed_from_linear_history(
        bjt: &crate::device::Bjt,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        history_internal_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_linear_prev: &VbicPredictorLinearBranchState,
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        Self::vbic_dynamic_internal_seed_from_predicted_linear_history(
            bjt,
            target_external,
            history_internal_prev,
            history_linear_prev,
            None,
            0.0,
            0.0,
        )
    }

    #[inline]
    fn vbic_predictor_linear_branch_state_is_finite(
        linear: &VbicPredictorLinearBranchState,
    ) -> bool {
        [
            linear.vrcx,
            linear.vrci,
            linear.vrbx,
            linear.vrbi,
            linear.vre,
            linear.vrbp,
            linear.vrs,
        ]
        .iter()
        .all(|value| value.is_finite())
    }

    #[inline]
    fn predict_vbic_linear_branch_state_from_history(
        history_linear_prev: &VbicPredictorLinearBranchState,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        dt: Value,
        previous_dt: Value,
    ) -> VbicPredictorLinearBranchState {
        let predict_component = |previous: Value, previous_previous: Option<Value>| {
            Self::predict_transient_history_value(previous, previous_previous, dt, previous_dt)
        };

        VbicPredictorLinearBranchState {
            vrcx: predict_component(
                history_linear_prev.vrcx,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrcx),
            ),
            vrci: predict_component(
                history_linear_prev.vrci,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrci),
            ),
            vrbx: predict_component(
                history_linear_prev.vrbx,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrbx),
            ),
            vrbi: predict_component(
                history_linear_prev.vrbi,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrbi),
            ),
            vre: predict_component(
                history_linear_prev.vre,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vre),
            ),
            vrbp: predict_component(
                history_linear_prev.vrbp,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrbp),
            ),
            vrs: predict_component(
                history_linear_prev.vrs,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrs),
            ),
        }
    }

    #[inline]
    fn vbic_dynamic_internal_seed_from_predicted_linear_history(
        bjt: &crate::device::Bjt,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        history_internal_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_linear_prev: &VbicPredictorLinearBranchState,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        dt: Value,
        previous_dt: Value,
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        let predicted_linear = Self::predict_vbic_linear_branch_state_from_history(
            history_linear_prev,
            history_linear_prev_prev,
            dt,
            previous_dt,
        );
        if !Self::vbic_predictor_linear_branch_state_is_finite(&predicted_linear) {
            return None;
        }

        let polarity = match bjt.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        };

        let mut seed_internal = *history_internal_prev;
        seed_internal[BJT_VCX_STATE_INDEX] =
            target_external[BJT_EXT_C_INDEX] - polarity * predicted_linear.vrcx;
        seed_internal[BJT_VCI_STATE_INDEX] =
            seed_internal[BJT_VCX_STATE_INDEX] - polarity * predicted_linear.vrci;
        seed_internal[BJT_VBX_STATE_INDEX] =
            target_external[BJT_EXT_B_INDEX] - polarity * predicted_linear.vrbx;
        seed_internal[BJT_VBI_STATE_INDEX] =
            seed_internal[BJT_VBX_STATE_INDEX] - polarity * predicted_linear.vrbi;
        seed_internal[BJT_VEI_STATE_INDEX] =
            target_external[BJT_EXT_E_INDEX] - polarity * predicted_linear.vre;
        seed_internal[BJT_VBP_STATE_INDEX] =
            seed_internal[BJT_VCX_STATE_INDEX] + polarity * predicted_linear.vrbp;
        seed_internal[BJT_VSI_STATE_INDEX] =
            target_external[BJT_EXT_S_INDEX] - polarity * predicted_linear.vrs;

        seed_internal
            .iter()
            .take(BJT_THERMAL_STATE_INDEX)
            .all(|value| value.is_finite())
            .then(|| seed_internal)
    }

    #[inline]
    fn vbic_dynamic_internal_seed_from_history_with_linear_history(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        history_internal_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_internal_prev_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_linear_prev: Option<&VbicPredictorLinearBranchState>,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        dt: Value,
        previous_dt: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let Some(history_internal_prev) = history_internal_prev else {
            return live_seed;
        };
        if !history_internal_prev.iter().all(|value| value.is_finite()) {
            return live_seed;
        }
        let history_internal_prev_prev = history_internal_prev_prev
            .filter(|history| history.iter().all(|value| value.is_finite()));
        let history_linear_prev = history_linear_prev
            .filter(|linear| Self::vbic_predictor_linear_branch_state_is_finite(linear));
        let history_linear_prev_prev = history_linear_prev_prev
            .filter(|linear| Self::vbic_predictor_linear_branch_state_is_finite(linear));

        // With `PREDICTOR`, ngspice seeds explicit VBIC unknowns from accepted
        // history before re-evaluating the device at the current iterate.
        // The reduced formulation does not expose ngspice's explicit internal
        // state vector, so reconstruct the hidden internal nodes from the
        // predicted accepted branch history first, then let the dynamic solve
        // refine that seed at the current external iterate.
        let target_external = [vc, vb, ve, vs];
        let mut seed_internal = history_linear_prev
            .and_then(|history_linear_prev| {
                Self::vbic_dynamic_internal_seed_from_predicted_linear_history(
                    bjt,
                    target_external,
                    history_internal_prev,
                    history_linear_prev,
                    history_linear_prev_prev,
                    dt,
                    previous_dt,
                )
            })
            .or_else(|| {
                history_linear_prev.and_then(|history_linear_prev| {
                    Self::vbic_dynamic_internal_seed_from_linear_history(
                        bjt,
                        target_external,
                        history_internal_prev,
                        history_linear_prev,
                    )
                })
            })
            .unwrap_or(live_seed);
        if bjt.uses_vbic_dynamic_charges() && bjt.td > 0.0 {
            // Match ngspice's MODEINITPRED behavior for VBIC excess-phase states:
            // xf1 stays anchored to the accepted state1 solution, while xf2 is
            // linearly extrapolated from accepted history.
            seed_internal[BJT_DELAY_XF1_STATE_INDEX] =
                history_internal_prev[BJT_DELAY_XF1_STATE_INDEX];
            seed_internal[BJT_DELAY_XF2_STATE_INDEX] = Self::predict_transient_history_value(
                history_internal_prev[BJT_DELAY_XF2_STATE_INDEX],
                history_internal_prev_prev.map(|history_internal_prev_prev| {
                    history_internal_prev_prev[BJT_DELAY_XF2_STATE_INDEX]
                }),
                dt,
                previous_dt,
            );
        }

        if bjt.has_vbic_self_heating() {
            seed_internal[BJT_THERMAL_STATE_INDEX] = Self::predict_transient_history_value(
                history_internal_prev[BJT_THERMAL_STATE_INDEX],
                history_internal_prev_prev.map(|history_internal_prev_prev| {
                    history_internal_prev_prev[BJT_THERMAL_STATE_INDEX]
                }),
                dt,
                previous_dt,
            )
            .max(bjt.minimum_thermal_rise());
        }

        bjt.limit_vbic_dynamic_internal_state_to_previous(seed_internal, *history_internal_prev)
    }

    #[inline]
    fn collect_vbic_truncation_charge_state(
        circuit: &crate::circuit::Circuit,
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: Option<&[Option<BjtChargeSnapshot>]>,
        snapshot_reuse_abstol: Value,
        snapshot_reuse_reltol: Value,
    ) -> Option<Vec<Value>> {
        let vbic_device_count = circuit
            .bjts
            .devices
            .iter()
            .filter(|bjt| bjt.uses_vbic_dynamic_charges())
            .count();
        if vbic_device_count == 0 {
            return None;
        }

        let mut charges =
            Vec::with_capacity(vbic_device_count.saturating_mul(BJT_VBIC_TRUNCATION_BRANCH_COUNT));

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if !bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            let vc = Self::node_voltage(voltages, bjt.node_collector);
            let vb = Self::node_voltage(voltages, bjt.node_base);
            let ve = Self::node_voltage(voltages, bjt.node_emitter);
            let vs = Self::node_voltage(voltages, bjt.node_substrate);
            let snapshot = Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                bjt,
                [vc, vb, ve, vs],
                method,
                trap_order,
                dt,
                &history.charge_q_prev[idx],
                &history.charge_q_prev_prev[idx],
                &history.charge_cq_prev[idx],
                history.dynamic_internal_prev.get(idx),
                history.dynamic_internal_prev_prev.get(idx),
                history.dynamic_linear_prev.get(idx),
                history.dynamic_linear_prev_prev.get(idx),
                history.accepted_dt_prev,
                vbic_snapshot_cache.and_then(|cache| cache.get(idx).copied().flatten()),
                VbicCachedSnapshotReuse::SeedOnly,
                snapshot_reuse_abstol,
                snapshot_reuse_reltol,
            )?;

            charges.extend(
                snapshot.branches[..BJT_VBIC_TRUNCATION_BRANCH_COUNT]
                    .iter()
                    .map(|branch| branch.charge),
            );
        }

        Some(charges)
    }

    #[inline]
    fn ngspice_vbic_truncation_factor(method: IntegrationMethod, order: u8) -> Value {
        match order.max(1) {
            1 => 0.5,
            2 => match method {
                IntegrationMethod::Gear2 => 0.222_222_222_2,
                _ => 0.083_333_333_33,
            },
            _ => 0.083_333_333_33,
        }
    }

    #[inline]
    fn ngspice_charge_truncation_limit(
        q_curr: Value,
        q_prev: Value,
        q_prev_prev: Value,
        q_prev_prev_prev: Value,
        cq_curr: Value,
        cq_prev: Value,
        dt: Value,
        prev_dt: Value,
        prev_prev_dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if !dt.is_finite() || dt <= 0.0 {
            return None;
        }

        let mut order = trap_order.clamp(1, 2);
        if order >= 2 && (!prev_dt.is_finite() || prev_dt <= 0.0) {
            order = 1;
        }
        if order >= 2 && (!prev_prev_dt.is_finite() || prev_prev_dt <= 0.0) {
            order = 1;
        }

        let volttol = current_abstol + reltol * cq_curr.abs().max(cq_prev.abs());
        let chargetol = reltol * q_curr.abs().max(q_prev.abs()).max(charge_abstol) / dt;
        let tol = volttol.max(chargetol);
        if !tol.is_finite() || tol <= 0.0 {
            return None;
        }

        let mut diff = [q_curr, q_prev, q_prev_prev, q_prev_prev_prev];
        let mut deltmp = [dt, prev_dt, prev_prev_dt];
        let mut j = usize::from(order);
        while j > 0 {
            for i in 0..=j {
                let denom = deltmp[i];
                if !denom.is_finite() || denom <= 0.0 {
                    return None;
                }
                diff[i] = (diff[i] - diff[i + 1]) / denom;
            }
            j -= 1;
            for i in 0..=j {
                deltmp[i] += deltmp[i + 1];
            }
        }

        let factor = Self::ngspice_vbic_truncation_factor(method, order);
        let denom = current_abstol.max(factor * diff[0].abs());
        if !denom.is_finite() || denom <= 0.0 {
            return None;
        }

        let mut limit = trtol.max(1.0) * tol / denom;
        if order >= 2 {
            limit = limit.sqrt();
        }
        (limit.is_finite() && limit > 0.0).then_some(limit)
    }

    #[inline]
    fn capacitor_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        prev_dt: Value,
        prev_prev_dt: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if !prev_dt.is_finite() || prev_dt <= 0.0 {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method(effective_method);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let capacitance = circuit.capacitors.capacitances[idx];
            if !capacitance.is_finite() || capacitance <= 0.0 {
                continue;
            }

            let voltage = Self::differential_voltage(candidate_solution, cap.pp.row, cap.nn.row);
            let q_curr = capacitance * voltage;
            let q_prev = capacitance * circuit.capacitors.v_prev[idx];
            let q_prev_prev = capacitance * circuit.capacitors.v_prev_prev[idx];
            let q_prev_prev_prev = capacitance * circuit.capacitors.v_prev_prev_prev[idx];
            let geq = coeff.capacitor_geq(capacitance, dt);
            let ieq = coeff.capacitor_ieq(
                capacitance,
                dt,
                circuit.capacitors.v_prev[idx],
                circuit.capacitors.v_prev_prev[idx],
                circuit.capacitors.i_prev[idx],
            );
            let cq_curr = geq * voltage - ieq;
            let cq_prev = circuit.capacitors.i_prev[idx];

            let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                q_curr,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_curr,
                cq_prev,
                dt,
                prev_dt,
                prev_prev_dt,
                effective_method,
                trap_order,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            ) else {
                continue;
            };
            found_branch = true;
            limit = limit.min(branch_limit);
        }

        found_branch.then_some(limit)
    }

    #[inline]
    fn vbic_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if !bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            let vc = Self::node_voltage(candidate_solution, bjt.node_collector);
            let vb = Self::node_voltage(candidate_solution, bjt.node_base);
            let ve = Self::node_voltage(candidate_solution, bjt.node_emitter);
            let vs = Self::node_voltage(candidate_solution, bjt.node_substrate);
            let candidate_external = [vc, vb, ve, vs];
            let cached_snapshot =
                vbic_snapshot_cache
                    .get(idx)
                    .copied()
                    .flatten()
                    .filter(|snapshot| {
                        Self::vbic_snapshot_matches_external_bias_exact(
                            snapshot,
                            &candidate_external,
                        )
                    });
            let snapshot = if let Some(snapshot) = cached_snapshot {
                snapshot
            } else {
                let (snapshot_reuse_abstol, snapshot_reuse_reltol) =
                    Self::vbic_runtime_snapshot_reuse_tolerances(voltage_abstol, reltol);
                Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                    bjt,
                    candidate_external,
                    method,
                    trap_order,
                    dt,
                    &history.charge_q_prev[idx],
                    &history.charge_q_prev_prev[idx],
                    &history.charge_cq_prev[idx],
                    history.dynamic_internal_prev.get(idx),
                    history.dynamic_internal_prev_prev.get(idx),
                    history.dynamic_linear_prev.get(idx),
                    history.dynamic_linear_prev_prev.get(idx),
                    history.accepted_dt_prev,
                    vbic_snapshot_cache.get(idx).copied().flatten(),
                    VbicCachedSnapshotReuse::SeedOnly,
                    snapshot_reuse_abstol,
                    snapshot_reuse_reltol,
                )?
            };

            for branch_idx in 0..BJT_VBIC_TRUNCATION_BRANCH_COUNT {
                let q_curr = snapshot.branches[branch_idx].charge;
                let q_prev = history.charge_q_prev[idx][branch_idx];
                let q_prev_prev = history.charge_q_prev_prev[idx][branch_idx];
                let q_prev_prev_prev = history.charge_q_prev_prev_prev[idx][branch_idx];
                let cq_prev = history.charge_cq_prev[idx][branch_idx];
                let cq_curr = Self::jfet_companion_ccap(
                    effective_method,
                    trap_order,
                    dt,
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );

                let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                    dt,
                    history.accepted_dt_prev,
                    history.accepted_dt_prev_prev,
                    effective_method,
                    trap_order,
                    reltol,
                    current_abstol,
                    charge_abstol,
                    trtol,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    fn legacy_bjt_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            let vc = Self::node_voltage(candidate_solution, bjt.node_collector);
            let vb = Self::node_voltage(candidate_solution, bjt.node_base);
            let ve = Self::node_voltage(candidate_solution, bjt.node_emitter);
            let vs = Self::node_voltage(candidate_solution, bjt.node_substrate);
            let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
            let (vbe, vbc, vcs) = Self::legacy_bjt_charge_branch_voltages(&snapshot);
            let charges = bjt.legacy_transient_charge_state(vbe, vbc, vcs);

            for (branch_idx, capacitance, q_curr) in [
                (BJT_QBE_BRANCH_INDEX, charges.capbe, charges.qbe),
                (BJT_QBC_BRANCH_INDEX, charges.capbc, charges.qbc),
                (BJT_QBCP_BRANCH_INDEX, charges.capcs, charges.qcs),
            ] {
                if capacitance <= 0.0 {
                    continue;
                }
                let q_prev = history.charge_q_prev[idx][branch_idx];
                let q_prev_prev = history.charge_q_prev_prev[idx][branch_idx];
                let q_prev_prev_prev = history.charge_q_prev_prev_prev[idx][branch_idx];
                let cq_prev = history.charge_cq_prev[idx][branch_idx];
                let cq_curr = Self::jfet_companion_ccap(
                    effective_method,
                    trap_order,
                    dt,
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );

                let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                    dt,
                    history.accepted_dt_prev,
                    history.accepted_dt_prev_prev,
                    effective_method,
                    trap_order,
                    reltol,
                    current_abstol,
                    charge_abstol,
                    trtol,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    fn bjt_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        if let Some(vbic_limit) = Self::vbic_ngspice_truncation_limit(
            circuit,
            candidate_solution,
            method,
            trap_order,
            dt,
            history,
            vbic_snapshot_cache,
            voltage_abstol,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        ) {
            limit = limit.min(vbic_limit);
            found_branch = true;
        }

        if let Some(legacy_limit) = Self::legacy_bjt_ngspice_truncation_limit(
            circuit,
            candidate_solution,
            method,
            trap_order,
            dt,
            history,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        ) {
            limit = limit.min(legacy_limit);
            found_branch = true;
        }

        found_branch.then_some(limit)
    }

    #[inline]
    fn jfet_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &JfetTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, candidate_solution);
            let (vgs_charge, vgd_charge) =
                Self::jfet_charge_branch_voltages(jfet, candidate_solution);
            let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);

            for (
                capacitance,
                voltage,
                voltage_prev,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_prev,
            ) in [
                (
                    cgs,
                    vgs_charge,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.qgs_prev_prev_prev[idx],
                    history.cqgs_prev[idx],
                ),
                (
                    cgd,
                    vgd_charge,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.qgd_prev_prev_prev[idx],
                    history.cqgd_prev[idx],
                ),
            ] {
                if !capacitance.is_finite() || capacitance <= 0.0 {
                    continue;
                }

                let (_geq, _ieq, q_curr, cq_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    capacitance,
                    voltage,
                    voltage_prev,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                    dt,
                    history.accepted_dt_prev,
                    history.accepted_dt_prev_prev,
                    effective_method,
                    trap_order,
                    reltol,
                    current_abstol,
                    charge_abstol,
                    trtol,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    fn mosfet_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &MosfetTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (vgs_eval, vds_eval, vbs_eval) = mos.eval_branch_voltages_at(candidate_solution);
            let (vgs, vgd, vgb) = mos.gate_charge_branch_voltages_at(candidate_solution);
            let (cgs_half, cgd_half, cgb_half) =
                mos.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();

            for (
                capacitance,
                voltage,
                voltage_prev,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_prev,
            ) in [
                (
                    cgs_half + history.capgs_prev_half[idx] + cgs_ov,
                    vgs,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.qgs_prev_prev_prev[idx],
                    history.cqgs_prev[idx],
                ),
                (
                    cgd_half + history.capgd_prev_half[idx] + cgd_ov,
                    vgd,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.qgd_prev_prev_prev[idx],
                    history.cqgd_prev[idx],
                ),
                (
                    cgb_half + history.capgb_prev_half[idx] + cgb_ov,
                    vgb,
                    history.vgb_prev[idx],
                    history.qgb_prev[idx],
                    history.qgb_prev_prev[idx],
                    history.qgb_prev_prev_prev[idx],
                    history.cqgb_prev[idx],
                ),
            ] {
                if !capacitance.is_finite() || capacitance <= 0.0 {
                    continue;
                }

                let (_geq, _ieq, q_curr, cq_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    capacitance,
                    voltage,
                    voltage_prev,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                    dt,
                    history.accepted_dt_prev,
                    history.accepted_dt_prev_prev,
                    effective_method,
                    trap_order,
                    reltol,
                    current_abstol,
                    charge_abstol,
                    trtol,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    fn min_truncation_limit(first: Option<Value>, second: Option<Value>) -> Option<Value> {
        match (first, second) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    #[inline]
    fn should_retry_ngspice_charge_truncation(limit: Value, dt: Value) -> bool {
        limit.is_finite() && dt.is_finite() && dt > 0.0 && limit <= 0.9 * dt
    }

    #[inline]
    fn should_promote_ngspice_charge_truncation(limit: Value, dt: Value) -> bool {
        limit.is_finite() && dt.is_finite() && dt > 0.0 && limit > 1.05 * dt
    }

    #[inline]
    fn next_trapezoidal_order_after_accepted_step(
        current_order: u8,
        hit_breakpoint: bool,
        should_promote: bool,
    ) -> u8 {
        if hit_breakpoint {
            1
        } else if current_order >= 2 || should_promote {
            2
        } else {
            1
        }
    }

    #[inline]
    fn trapezoidal_order_after_timestep_control_reject(current_order: u8) -> u8 {
        current_order.max(1)
    }

    #[inline]
    fn vbic_charge_lte_startup_window_end(
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
    ) -> Value {
        let maxstep_window = if hinted_max_step.is_finite() && hinted_max_step > 0.0 {
            hinted_max_step * 0.1
        } else {
            Value::INFINITY
        };
        let td_window = smallest_vbic_excess_phase_td
            .filter(|td| td.is_finite() && *td > 0.0)
            .map(|td| td * 32.0)
            .unwrap_or(maxstep_window);

        maxstep_window.min(td_window)
    }

    #[inline]
    fn vbic_excess_phase_startup_step_cap(
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
    ) -> Option<Value> {
        let td = smallest_vbic_excess_phase_td.filter(|td| td.is_finite() && *td > 0.0)?;
        let hinted_max_step = if hinted_max_step.is_finite() && hinted_max_step > 0.0 {
            hinted_max_step
        } else {
            Value::INFINITY
        };

        Some((td * 0.25).clamp(1e-15, hinted_max_step))
    }

    #[inline]
    fn should_use_vbic_charge_lte_startup_guard(
        has_vbic_excess_phase: bool,
        step_time: Value,
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
    ) -> bool {
        has_vbic_excess_phase
            && step_time.is_finite()
            && step_time
                <= Self::vbic_charge_lte_startup_window_end(
                    hinted_max_step,
                    smallest_vbic_excess_phase_td,
                )
    }

    #[inline]
    fn should_hold_vbic_excess_phase_first_order(
        has_vbic_excess_phase: bool,
        _accepted_time: Value,
        _hinted_max_step: Value,
        _smallest_vbic_excess_phase_td: Option<Value>,
    ) -> bool {
        // ngspice does not impose a VBIC-specific first-order hold once the
        // current timepoint has been accepted. Promotion back to order 2 is
        // controlled solely by the same order-2 truncation check used for any
        // trapezoidal transient step.
        let _ = has_vbic_excess_phase;
        false
    }

    #[inline]
    fn should_use_vbic_charge_lte_estimator(
        has_vbic_excess_phase: bool,
        step_time: Value,
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
        dt: Value,
        preferred_min_dt: Value,
    ) -> bool {
        dt.is_finite()
            && dt > preferred_min_dt
            && Self::should_use_vbic_charge_lte_startup_guard(
                has_vbic_excess_phase,
                step_time,
                hinted_max_step,
                smallest_vbic_excess_phase_td,
            )
    }

    #[inline]
    fn should_defer_voltage_lte_to_vbic_truncation(
        has_vbic_excess_phase: bool,
        step_time: Value,
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
        vbic_truncation_limit: Option<Value>,
        using_vbic_charge_lte_estimator: bool,
    ) -> bool {
        has_vbic_excess_phase
            && !using_vbic_charge_lte_estimator
            && vbic_truncation_limit.is_some()
            && Self::should_use_vbic_charge_lte_startup_guard(
                has_vbic_excess_phase,
                step_time,
                hinted_max_step,
                smallest_vbic_excess_phase_td,
            )
    }

    #[inline]
    fn bjt_charge_truncation_covers_transient_lte(
        circuit: &crate::circuit::Circuit,
        bjt_truncation_limit: Option<Value>,
    ) -> bool {
        bjt_truncation_limit.is_some()
            && !circuit.bjts.devices.is_empty()
            && circuit.capacitors.is_empty()
            && circuit.inductors.is_empty()
            && circuit.diodes.is_empty()
            && circuit.mosfets.is_empty()
            && circuit.jfets.is_empty()
            && circuit.tlines.is_empty()
            && circuit.coupled_tlines.is_empty()
            && circuit.coupled_inductor_pairs.is_empty()
            && circuit.multi_winding_transformers.is_empty()
            && circuit.jiles_atherton_inductors.is_empty()
            && !circuit.has_xspice_devices()
    }

    #[inline]
    fn jfet_charge_truncation_covers_transient_lte(
        circuit: &crate::circuit::Circuit,
        jfet_truncation_limit: Option<Value>,
    ) -> bool {
        jfet_truncation_limit.is_some()
            && !circuit.jfets.is_empty()
            && circuit.capacitors.is_empty()
            && circuit.inductors.is_empty()
            && circuit.diodes.is_empty()
            && circuit.bjts.devices.is_empty()
            && circuit.mosfets.is_empty()
            && circuit.tlines.is_empty()
            && circuit.coupled_tlines.is_empty()
            && circuit.coupled_inductor_pairs.is_empty()
            && circuit.multi_winding_transformers.is_empty()
            && circuit.jiles_atherton_inductors.is_empty()
            && !circuit.has_xspice_devices()
    }

    #[inline]
    fn mosfet_charge_truncation_covers_transient_lte(
        circuit: &crate::circuit::Circuit,
        mosfet_truncation_limit: Option<Value>,
    ) -> bool {
        mosfet_truncation_limit.is_some()
            && !circuit.mosfets.is_empty()
            && circuit.capacitors.is_empty()
            && circuit.inductors.is_empty()
            && circuit.diodes.is_empty()
            && circuit.bjts.devices.is_empty()
            && circuit.jfets.is_empty()
            && circuit.tlines.is_empty()
            && circuit.coupled_tlines.is_empty()
            && circuit.coupled_inductor_pairs.is_empty()
            && circuit.multi_winding_transformers.is_empty()
            && circuit.jiles_atherton_inductors.is_empty()
            && !circuit.has_xspice_devices()
    }

    #[inline]
    fn ngspice_device_truncation_covers_transient_lte(
        circuit: &crate::circuit::Circuit,
        capacitor_truncation_limit: Option<Value>,
        bjt_truncation_limit: Option<Value>,
        jfet_truncation_limit: Option<Value>,
        mosfet_truncation_limit: Option<Value>,
    ) -> bool {
        if circuit.has_xspice_devices()
            || !circuit.diodes.is_empty()
            || !circuit.inductors.is_empty()
            || !circuit.coupled_inductor_pairs.is_empty()
            || !circuit.multi_winding_transformers.is_empty()
            || !circuit.jiles_atherton_inductors.is_empty()
        {
            return false;
        }

        let capacitor_controlled =
            circuit.capacitors.is_empty() || capacitor_truncation_limit.is_some();
        let bjt_controlled = circuit.bjts.devices.is_empty() || bjt_truncation_limit.is_some();
        let jfet_controlled = circuit.jfets.is_empty() || jfet_truncation_limit.is_some();
        let mosfet_controlled = circuit.mosfets.is_empty() || mosfet_truncation_limit.is_some();

        capacitor_controlled && bjt_controlled && jfet_controlled && mosfet_controlled
    }

    #[inline]
    fn estimate_transient_lte(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        is_strictly_linear_transient: bool,
        bjt_history: &BjtTransientHistory,
        voltage_lte_estimator: &LteEstimator,
        vbic_charge_lte_estimator: Option<&LteEstimator>,
        vbic_snapshot_cache: Option<&[Option<BjtChargeSnapshot>]>,
        voltage_abstol: Value,
        reltol: Value,
    ) -> (Value, bool, bool) {
        if is_strictly_linear_transient {
            return (0.0, true, false);
        }

        let (snapshot_reuse_abstol, snapshot_reuse_reltol) =
            Self::vbic_runtime_snapshot_reuse_tolerances(voltage_abstol, reltol);
        if let Some(charge_lte_estimator) = vbic_charge_lte_estimator
            && let Some(vbic_charge_state) = Self::collect_vbic_truncation_charge_state(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                bjt_history,
                vbic_snapshot_cache,
                snapshot_reuse_abstol,
                snapshot_reuse_reltol,
            )
        {
            let (lte, accept) = charge_lte_estimator.estimate(&vbic_charge_state, dt);
            return (lte, accept, true);
        }

        let (lte, accept) =
            voltage_lte_estimator.estimate_prefix(candidate_solution, circuit.num_nodes(), dt);
        (lte, accept, false)
    }

    #[inline]
    fn recommend_transient_lte_scale(
        voltage_lte_estimator: &LteEstimator,
        vbic_charge_lte_estimator: Option<&LteEstimator>,
        lte: Value,
        uses_vbic_charge_lte: bool,
    ) -> Value {
        if uses_vbic_charge_lte {
            vbic_charge_lte_estimator
                .unwrap_or(voltage_lte_estimator)
                .recommend_scale(lte)
        } else {
            voltage_lte_estimator.recommend_scale(lte)
        }
    }

    #[inline]
    fn should_promote_trapezoidal_order(
        circuit: &crate::circuit::Circuit,
        accepted_solution: &[Value],
        method: IntegrationMethod,
        dt: Value,
        is_strictly_linear_transient: bool,
        history: &BjtTransientHistory,
        jfet_history: &JfetTransientHistory,
        mosfet_history: &MosfetTransientHistory,
        voltage_lte_estimator: &LteEstimator,
        vbic_charge_lte_estimator: Option<&LteEstimator>,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> bool {
        if !matches!(
            method,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
        ) {
            return false;
        }
        if !(dt.is_finite() && dt > 0.0) {
            return false;
        }
        // Match ngspice startup behavior: keep order-1 through the first accepted
        // transient step, then only promote when an order-2 truncation/LTE check
        // says the current timestep remains viable.
        if !(history.accepted_dt_prev.is_finite() && history.accepted_dt_prev > 0.0) {
            return false;
        }

        if !circuit.bjts.devices.is_empty() {
            let Some(limit) = Self::bjt_ngspice_truncation_limit(
                circuit,
                accepted_solution,
                method,
                2,
                dt,
                history,
                vbic_snapshot_cache,
                voltage_abstol,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            ) else {
                return false;
            };
            return Self::should_promote_ngspice_charge_truncation(limit, dt);
        }

        if !circuit.jfets.is_empty() {
            let Some(limit) = Self::jfet_ngspice_truncation_limit(
                circuit,
                accepted_solution,
                method,
                2,
                dt,
                jfet_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            ) else {
                return false;
            };
            if !Self::should_promote_ngspice_charge_truncation(limit, dt) {
                return false;
            }
            if Self::jfet_charge_truncation_covers_transient_lte(circuit, Some(limit)) {
                return true;
            }
        }

        if !circuit.mosfets.is_empty() {
            let Some(limit) = Self::mosfet_ngspice_truncation_limit(
                circuit,
                accepted_solution,
                method,
                2,
                dt,
                mosfet_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            ) else {
                return false;
            };
            if !Self::should_promote_ngspice_charge_truncation(limit, dt) {
                return false;
            }
            if Self::mosfet_charge_truncation_covers_transient_lte(circuit, Some(limit)) {
                return true;
            }
        }

        let (candidate_lte, accept, uses_vbic_charge_lte) = Self::estimate_transient_lte(
            circuit,
            accepted_solution,
            method,
            2,
            dt,
            is_strictly_linear_transient,
            history,
            voltage_lte_estimator,
            vbic_charge_lte_estimator,
            Some(vbic_snapshot_cache),
            voltage_abstol,
            reltol,
        );
        if !accept {
            return false;
        }

        let candidate_scale = if is_strictly_linear_transient {
            1.0
        } else {
            Self::recommend_transient_lte_scale(
                voltage_lte_estimator,
                vbic_charge_lte_estimator,
                candidate_lte,
                uses_vbic_charge_lte,
            )
        };
        candidate_scale >= 0.95
    }

    #[inline]
    fn record_vbic_truncation_charge_state(
        estimator: &mut Option<LteEstimator>,
        circuit: &crate::circuit::Circuit,
        accepted_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: Option<&[Option<BjtChargeSnapshot>]>,
        method_order: u32,
    ) {
        let Some(estimator) = estimator.as_mut() else {
            return;
        };

        if let Some(vbic_charge_state) = Self::collect_vbic_truncation_charge_state(
            circuit,
            accepted_solution,
            method,
            trap_order,
            dt,
            history,
            vbic_snapshot_cache,
            VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL,
            VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL,
        ) {
            estimator.record(&vbic_charge_state, dt);
            estimator.set_method_order(method_order);
        }
    }

    #[inline]
    fn lu_decompose_small_dense_real<const N: usize>(
        matrix: &[[Value; N]; N],
        dim: usize,
    ) -> Option<([[Value; N]; N], [usize; N])> {
        if dim == 0 {
            let mut pivots = [0usize; N];
            for (idx, pivot) in pivots.iter_mut().enumerate() {
                *pivot = idx;
            }
            return Some((*matrix, pivots));
        }

        let mut lu = *matrix;
        let mut pivots = [0usize; N];
        for (idx, pivot) in pivots.iter_mut().enumerate() {
            *pivot = idx;
        }

        for pivot in 0..dim {
            let mut best = pivot;
            let mut best_abs = lu[pivot][pivot].abs();
            for row in (pivot + 1)..dim {
                let value = lu[row][pivot].abs();
                if value > best_abs {
                    best = row;
                    best_abs = value;
                }
            }
            if best_abs < 1e-18 {
                return None;
            }
            if best != pivot {
                lu.swap(pivot, best);
                pivots.swap(pivot, best);
            }

            let pivot_value = lu[pivot][pivot];
            for row in (pivot + 1)..dim {
                lu[row][pivot] /= pivot_value;
                let factor = lu[row][pivot];
                for col in (pivot + 1)..dim {
                    lu[row][col] -= factor * lu[pivot][col];
                }
            }
        }

        Some((lu, pivots))
    }

    #[inline]
    fn lu_solve_small_dense_real<const N: usize>(
        lu: &[[Value; N]; N],
        pivots: &[usize; N],
        rhs: &[Value; N],
        dim: usize,
    ) -> Option<[Value; N]> {
        if dim == 0 {
            return Some([0.0; N]);
        }

        let mut x = [0.0; N];
        for row in 0..dim {
            x[row] = rhs[pivots[row]];
            for col in 0..row {
                x[row] -= lu[row][col] * x[col];
            }
        }

        for row in (0..dim).rev() {
            for col in (row + 1)..dim {
                x[row] -= lu[row][col] * x[col];
            }
            let diag = lu[row][row];
            if diag.abs() < 1e-18 {
                return None;
            }
            x[row] /= diag;
        }

        Some(x)
    }

    #[inline]
    fn legacy_bjt_charge_branch_voltages(snapshot: &BjtChargeSnapshot) -> (Value, Value, Value) {
        let internal = &snapshot.reduction.internal_voltages;
        (
            internal[BJT_VBI_STATE_INDEX] - internal[BJT_VEI_STATE_INDEX],
            internal[BJT_VBI_STATE_INDEX] - internal[BJT_VCI_STATE_INDEX],
            internal[BJT_VCX_STATE_INDEX] - internal[BJT_VSI_STATE_INDEX],
        )
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
            charge_q_prev: Vec::with_capacity(n),
            charge_q_prev_prev: Vec::with_capacity(n),
            charge_q_prev_prev_prev: Vec::with_capacity(n),
            charge_cq_prev: Vec::with_capacity(n),
            dynamic_internal_prev: Vec::with_capacity(n),
            dynamic_internal_prev_prev: Vec::with_capacity(n),
            dynamic_linear_prev: Vec::with_capacity(n),
            dynamic_linear_prev_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for bjt in &circuit.bjts.devices {
            let vc = Self::node_voltage(solution, bjt.node_collector);
            let vb = Self::node_voltage(solution, bjt.node_base);
            let ve = Self::node_voltage(solution, bjt.node_emitter);
            let vs = Self::node_voltage(solution, bjt.node_substrate);
            let vbe = vb - ve;
            let vbc = vb - vc;
            let vcs = vc - vs;
            let charge_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
            let (history_vbe, history_vbc, history_vcs) = if bjt.uses_vbic_dynamic_charges() {
                (vbe, vbc, vcs)
            } else {
                Self::legacy_bjt_charge_branch_voltages(&charge_snapshot)
            };
            history.vbe_prev.push(history_vbe);
            history.vbe_prev_prev.push(history_vbe);
            history.ibe_prev.push(0.0);
            history.vbc_prev.push(history_vbc);
            history.vbc_prev_prev.push(history_vbc);
            history.ibc_prev.push(0.0);
            history.vcs_prev.push(history_vcs);
            history.vcs_prev_prev.push(history_vcs);
            history.ics_prev.push(0.0);

            let mut charge_values = charge_snapshot.branches.map(|branch| branch.charge);
            if !bjt.uses_vbic_dynamic_charges() {
                let (legacy_vbe, legacy_vbc, legacy_vcs) =
                    Self::legacy_bjt_charge_branch_voltages(&charge_snapshot);
                let charges = bjt.legacy_transient_charge_state(legacy_vbe, legacy_vbc, legacy_vcs);
                charge_values[BJT_QBE_BRANCH_INDEX] = charges.qbe;
                charge_values[BJT_QBC_BRANCH_INDEX] = charges.qbc;
                charge_values[BJT_QBCP_BRANCH_INDEX] = charges.qcs;
            }
            let predictor_linear = Self::vbic_predictor_linear_branch_state(
                bjt,
                [vc, vb, ve, vs],
                charge_snapshot.reduction.internal_voltages,
            );
            history.charge_q_prev.push(charge_values);
            history.charge_q_prev_prev.push(charge_values);
            history.charge_q_prev_prev_prev.push(charge_values);
            history.charge_cq_prev.push([0.0; BJT_DYNAMIC_CHARGE_COUNT]);
            history
                .dynamic_internal_prev
                .push(charge_snapshot.reduction.internal_voltages);
            history
                .dynamic_internal_prev_prev
                .push(charge_snapshot.reduction.internal_voltages);
            history.dynamic_linear_prev.push(predictor_linear);
            history.dynamic_linear_prev_prev.push(predictor_linear);
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
            qgs_prev_prev_prev: Vec::with_capacity(n),
            cqgs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            qgd_prev: Vec::with_capacity(n),
            qgd_prev_prev: Vec::with_capacity(n),
            qgd_prev_prev_prev: Vec::with_capacity(n),
            cqgd_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
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
            history.qgs_prev_prev_prev.push(qgs);
            history.cqgs_prev.push(0.0);
            history.vgd_prev.push(vgd_charge);
            history.vgd_prev_prev.push(vgd_charge);
            history.qgd_prev.push(qgd);
            history.qgd_prev_prev.push(qgd);
            history.qgd_prev_prev_prev.push(qgd);
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
            capgs_prev_half: Vec::with_capacity(n),
            qgs_prev: Vec::with_capacity(n),
            qgs_prev_prev: Vec::with_capacity(n),
            qgs_prev_prev_prev: Vec::with_capacity(n),
            cqgs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            capgd_prev_half: Vec::with_capacity(n),
            qgd_prev: Vec::with_capacity(n),
            qgd_prev_prev: Vec::with_capacity(n),
            qgd_prev_prev_prev: Vec::with_capacity(n),
            cqgd_prev: Vec::with_capacity(n),
            vgb_prev: Vec::with_capacity(n),
            vgb_prev_prev: Vec::with_capacity(n),
            capgb_prev_half: Vec::with_capacity(n),
            qgb_prev: Vec::with_capacity(n),
            qgb_prev_prev: Vec::with_capacity(n),
            qgb_prev_prev_prev: Vec::with_capacity(n),
            cqgb_prev: Vec::with_capacity(n),
            vbs_j_prev: Vec::with_capacity(n),
            vbs_j_prev_prev: Vec::with_capacity(n),
            qbs_prev: Vec::with_capacity(n),
            qbs_prev_prev: Vec::with_capacity(n),
            cqbs_prev: Vec::with_capacity(n),
            vbd_j_prev: Vec::with_capacity(n),
            vbd_j_prev_prev: Vec::with_capacity(n),
            qbd_prev: Vec::with_capacity(n),
            qbd_prev_prev: Vec::with_capacity(n),
            cqbd_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for mos in &circuit.mosfets.devices {
            let (vgs, vds, vbs) = mos.eval_branch_voltages_at(solution);
            let vgd = vgs - vds;
            let vgb = vgs - vbs;
            let (cgs_half, cgd_half, cgb_half) = mos.transient_capacitance_halves_at(vgs, vds, vbs);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
            let cgs = 2.0 * cgs_half + cgs_ov;
            let cgd = 2.0 * cgd_half + cgd_ov;
            let cgb = 2.0 * cgb_half + cgb_ov;

            history.vgs_prev.push(vgs);
            history.vgs_prev_prev.push(vgs);
            history.capgs_prev_half.push(cgs_half);
            history.qgs_prev.push(cgs.max(0.0) * vgs);
            history.qgs_prev_prev.push(cgs.max(0.0) * vgs);
            history.qgs_prev_prev_prev.push(cgs.max(0.0) * vgs);
            history.cqgs_prev.push(0.0);

            history.vgd_prev.push(vgd);
            history.vgd_prev_prev.push(vgd);
            history.capgd_prev_half.push(cgd_half);
            history.qgd_prev.push(cgd.max(0.0) * vgd);
            history.qgd_prev_prev.push(cgd.max(0.0) * vgd);
            history.qgd_prev_prev_prev.push(cgd.max(0.0) * vgd);
            history.cqgd_prev.push(0.0);

            history.vgb_prev.push(vgb);
            history.vgb_prev_prev.push(vgb);
            history.capgb_prev_half.push(cgb_half);
            history.qgb_prev.push(cgb.max(0.0) * vgb);
            history.qgb_prev_prev.push(cgb.max(0.0) * vgb);
            history.qgb_prev_prev_prev.push(cgb.max(0.0) * vgb);
            history.cqgb_prev.push(0.0);

            let vbs_j = mos.body_source_charge_branch_voltage(vbs);
            let vbd_j = mos.body_drain_charge_branch_voltage(vds, vbs);
            let (qbs, _) = mos.body_source_junction_charge_and_capacitance_at(vbs);
            let (qbd, _) = mos.body_drain_junction_charge_and_capacitance_at(vds, vbs);
            history.vbs_j_prev.push(vbs_j);
            history.vbs_j_prev_prev.push(vbs_j);
            history.qbs_prev.push(qbs);
            history.qbs_prev_prev.push(qbs);
            history.cqbs_prev.push(0.0);
            history.vbd_j_prev.push(vbd_j);
            history.vbd_j_prev_prev.push(vbd_j);
            history.qbd_prev.push(qbd);
            history.qbd_prev_prev.push(qbd);
            history.cqbd_prev.push(0.0);
        }

        history
    }

    #[inline]
    fn stamp_bjt_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        cache_reuse: VbicCachedSnapshotReuse,
        voltage_abstol: Value,
        reltol: Value,
    ) {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let charge_factor = Self::jfet_companion_geq(effective_method, trap_order, 1.0, dt);
        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            let vc = Self::node_voltage(voltages, bjt.node_collector);
            let vb = Self::node_voltage(voltages, bjt.node_base);
            let ve = Self::node_voltage(voltages, bjt.node_emitter);
            let vs = Self::node_voltage(voltages, bjt.node_substrate);

            if bjt.uses_vbic_dynamic_charges() && charge_factor > 0.0 {
                let (snapshot_reuse_abstol, snapshot_reuse_reltol) =
                    Self::vbic_runtime_snapshot_reuse_tolerances(voltage_abstol, reltol);
                let cached_snapshot = vbic_snapshot_cache.get(idx).copied().flatten();
                let snapshot_start = std::time::Instant::now();
                let Some(snapshot) =
                    Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                        bjt,
                        [vc, vb, ve, vs],
                        method,
                        trap_order,
                        dt,
                        &history.charge_q_prev[idx],
                        &history.charge_q_prev_prev[idx],
                        &history.charge_cq_prev[idx],
                        history.dynamic_internal_prev.get(idx),
                        history.dynamic_internal_prev_prev.get(idx),
                        history.dynamic_linear_prev.get(idx),
                        history.dynamic_linear_prev_prev.get(idx),
                        history.accepted_dt_prev,
                        cached_snapshot,
                        cache_reuse,
                        snapshot_reuse_abstol,
                        snapshot_reuse_reltol,
                    )
                else {
                    vbic_snapshot_cache[idx] = None;
                    continue;
                };
                let snapshot_elapsed = snapshot_start.elapsed();
                static VBIC_SNAPSHOT_RESOLVE_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                if snapshot_elapsed.as_millis() >= 100 {
                    let log_count = VBIC_SNAPSHOT_RESOLVE_LOG_COUNT
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_count < 40 {
                        log::warn!(
                            "Slow VBIC snapshot resolve {} dt={:.3e} trap_order={} ext=({:.6e}, {:.6e}, {:.6e}, {:.6e}) cached={} elapsed={:.3?}",
                            bjt.name,
                            dt,
                            trap_order,
                            vc,
                            vb,
                            ve,
                            vs,
                            cached_snapshot.is_some(),
                            snapshot_elapsed,
                        );
                    }
                }
                let Some(linearization) = Self::assemble_vbic_transient_linearization(
                    bjt,
                    &snapshot,
                    effective_method,
                    trap_order,
                    dt,
                    &history.charge_q_prev[idx],
                    &history.charge_q_prev_prev[idx],
                    &history.charge_cq_prev[idx],
                ) else {
                    vbic_snapshot_cache[idx] = None;
                    continue;
                };
                let base_static_g = snapshot.reduction.g_reduced;
                vbic_snapshot_cache[idx] = Some(snapshot);

                let Some((y_total, reduced_i_eq)) =
                    Self::vbic_reduce_transient_external_system(&linearization)
                else {
                    vbic_snapshot_cache[idx] = None;
                    continue;
                };
                let (_base_static_g, base_static_i_eq) = Self::vbic_static_stamped_external_system(
                    bjt,
                    &snapshot.reduction.external_voltages,
                );

                let mut delta = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
                let mut delta_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
                for row in 0..BJT_EXTERNAL_STATE_DIM {
                    delta_i_eq[row] = reduced_i_eq[row] - base_static_i_eq[row];
                    for col in 0..BJT_EXTERNAL_STATE_DIM {
                        delta[row][col] = y_total[row][col] - base_static_g[row][col];
                    }
                }
                let max_delta_i_eq = delta_i_eq
                    .iter()
                    .map(|value| value.abs())
                    .fold(0.0, Value::max);
                static VBIC_DELTA_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                let delta_log_count =
                    VBIC_DELTA_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if max_delta_i_eq > 1.0 && delta_log_count < 20 {
                    log::warn!(
                        "VBIC transient delta {} max|di_eq|={:.3e}: total={:?} static={:?} delta={:?} xf=({:.3e}, {:.3e}) vrth={:.3e}",
                        bjt.name,
                        max_delta_i_eq,
                        reduced_i_eq,
                        base_static_i_eq,
                        delta_i_eq,
                        snapshot.reduction.internal_voltages[BJT_DELAY_XF1_STATE_INDEX],
                        snapshot.reduction.internal_voltages[BJT_DELAY_XF2_STATE_INDEX],
                        snapshot.reduction.internal_voltages[BJT_THERMAL_STATE_INDEX],
                    );
                }
                let nodes = [
                    bjt.node_collector,
                    bjt.node_base,
                    bjt.node_emitter,
                    bjt.node_substrate,
                ];
                Self::stamp_external_reduced_system(matrix, rhs, &nodes, &delta, &delta_i_eq);
                continue;
            }

            if charge_factor <= 0.0 {
                continue;
            }
            let mut snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
            let (legacy_vbe, legacy_vbc, legacy_vcs) =
                Self::legacy_bjt_charge_branch_voltages(&snapshot);
            let charges = bjt.legacy_transient_charge_state(legacy_vbe, legacy_vbc, legacy_vcs);

            if charges.capbe > 0.0 {
                snapshot.branches[BJT_QBE_BRANCH_INDEX] = BjtChargeBranch {
                    charge: charges.qbe,
                    pos_internal: Some(BJT_VBI_STATE_INDEX),
                    neg_internal: Some(BJT_VEI_STATE_INDEX),
                    d_internal: {
                        let mut d = [0.0; BJT_INTERNAL_STATE_DIM];
                        d[BJT_VBI_STATE_INDEX] = charges.capbe;
                        d[BJT_VEI_STATE_INDEX] = -charges.capbe;
                        d
                    },
                    ..Default::default()
                };
            }
            if charges.capbc > 0.0 {
                snapshot.branches[BJT_QBC_BRANCH_INDEX] = BjtChargeBranch {
                    charge: charges.qbc,
                    pos_internal: Some(BJT_VBI_STATE_INDEX),
                    neg_internal: Some(BJT_VCI_STATE_INDEX),
                    d_internal: {
                        let mut d = [0.0; BJT_INTERNAL_STATE_DIM];
                        d[BJT_VBI_STATE_INDEX] = charges.capbc;
                        d[BJT_VCI_STATE_INDEX] = -charges.capbc;
                        d
                    },
                    ..Default::default()
                };
            }
            if charges.capcs > 0.0 {
                snapshot.branches[BJT_QBCP_BRANCH_INDEX] = BjtChargeBranch {
                    charge: charges.qcs,
                    pos_internal: Some(BJT_VCX_STATE_INDEX),
                    neg_internal: Some(BJT_VSI_STATE_INDEX),
                    d_internal: {
                        let mut d = [0.0; BJT_INTERNAL_STATE_DIM];
                        d[BJT_VCX_STATE_INDEX] = charges.capcs;
                        d[BJT_VSI_STATE_INDEX] = -charges.capcs;
                        d
                    },
                    ..Default::default()
                };
            }

            let Some(linearization) = Self::assemble_vbic_transient_linearization(
                bjt,
                &snapshot,
                effective_method,
                trap_order,
                dt,
                &history.charge_q_prev[idx],
                &history.charge_q_prev_prev[idx],
                &history.charge_cq_prev[idx],
            ) else {
                continue;
            };
            let base_static_g = snapshot.reduction.g_reduced;
            let Some((y_total, reduced_i_eq)) =
                Self::vbic_reduce_transient_external_system(&linearization)
            else {
                continue;
            };
            let (_base_static_g, base_static_i_eq) = Self::vbic_static_stamped_external_system(
                bjt,
                &snapshot.reduction.external_voltages,
            );

            let mut delta = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
            let mut delta_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
            for row in 0..BJT_EXTERNAL_STATE_DIM {
                delta_i_eq[row] = reduced_i_eq[row] - base_static_i_eq[row];
                for col in 0..BJT_EXTERNAL_STATE_DIM {
                    delta[row][col] = y_total[row][col] - base_static_g[row][col];
                }
            }
            let nodes = [
                bjt.node_collector,
                bjt.node_base,
                bjt.node_emitter,
                bjt.node_substrate,
            ];
            Self::stamp_external_reduced_system(matrix, rhs, &nodes, &delta, &delta_i_eq);
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
        suppress_gate_charge: bool,
    ) {
        if suppress_gate_charge {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
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
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
    ) {
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (vgs_eval, vds_eval, vbs_eval) = mos.eval_branch_voltages_at(voltages);
            let (vgs, vgd, vgb) = mos.gate_charge_branch_voltages_at(voltages);
            let (cgs_half, cgd_half, cgb_half) =
                mos.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
            let cgs = cgs_half + history.capgs_prev_half[idx] + cgs_ov;
            let cgd = cgd_half + history.capgd_prev_half[idx] + cgd_ov;
            let cgb = cgb_half + history.capgb_prev_half[idx] + cgb_ov;

            if !suppress_gate_charge {
                let (geq_gs, ieq_gs, _qgs_curr, _cqgs_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    cgs,
                    vgs,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.cqgs_prev[idx],
                );
                if geq_gs > 0.0 {
                    Self::stamp_two_terminal_companion(
                        matrix,
                        rhs,
                        mos.node_gate,
                        mos.node_source,
                        geq_gs,
                        ieq_gs,
                    );
                }

                let (geq_gd, ieq_gd, _qgd_curr, _cqgd_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    cgd,
                    vgd,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.cqgd_prev[idx],
                );
                if geq_gd > 0.0 {
                    Self::stamp_two_terminal_companion(
                        matrix,
                        rhs,
                        mos.node_gate,
                        mos.node_drain,
                        geq_gd,
                        ieq_gd,
                    );
                }

                let (geq_gb, ieq_gb, _qgb_curr, _cqgb_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    cgb,
                    vgb,
                    history.vgb_prev[idx],
                    history.qgb_prev[idx],
                    history.qgb_prev_prev[idx],
                    history.cqgb_prev[idx],
                );
                if geq_gb > 0.0 {
                    Self::stamp_two_terminal_companion(
                        matrix,
                        rhs,
                        mos.node_gate,
                        mos.node_bulk,
                        geq_gb,
                        ieq_gb,
                    );
                }
            }

            let vbs_j = mos.body_source_charge_branch_voltage(vbs_eval);
            let vbd_j = mos.body_drain_charge_branch_voltage(vds_eval, vbs_eval);
            let (qbs_curr, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs_eval);
            let (qbd_curr, cbd) =
                mos.body_drain_junction_charge_and_capacitance_at(vds_eval, vbs_eval);
            let (bs_pos, bs_neg) = mos.body_source_charge_nodes();
            let (bd_pos, bd_neg) = mos.body_drain_charge_nodes();

            let (geq_bs, ieq_bs, _qbs_curr, _cqbs_curr) = Self::nonlinear_charge_companion_terms(
                effective_method,
                trap_order,
                dt,
                cbs,
                vbs_j,
                qbs_curr,
                history.qbs_prev[idx],
                history.qbs_prev_prev[idx],
                history.cqbs_prev[idx],
            );
            if geq_bs > 0.0 {
                Self::stamp_two_terminal_companion(matrix, rhs, bs_pos, bs_neg, geq_bs, ieq_bs);
            }

            let (geq_bd, ieq_bd, _qbd_curr, _cqbd_curr) = Self::nonlinear_charge_companion_terms(
                effective_method,
                trap_order,
                dt,
                cbd,
                vbd_j,
                qbd_curr,
                history.qbd_prev[idx],
                history.qbd_prev_prev[idx],
                history.cqbd_prev[idx],
            );
            if geq_bd > 0.0 {
                Self::stamp_two_terminal_companion(matrix, rhs, bd_pos, bd_neg, geq_bd, ieq_bd);
            }
        }
    }

    #[inline]
    fn stamp_tline_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        _tline_dc_refs: &[(Value, Value)],
    ) {
        for tl in &circuit.tlines {
            let response = tl.transient_port_response(time);
            Self::stamp_tline_two_port(matrix, rhs, tl, response);
        }
    }

    #[inline]
    fn stamp_coupled_tline_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        coupled_tline_refs: &[CoupledTlineReferenceState],
    ) {
        for (idx, tl) in circuit.coupled_tlines.iter().enumerate() {
            let refs = coupled_tline_refs.get(idx).cloned().unwrap_or_default();
            let incoming_near = tl.incoming_near_modal(time, &refs.far_modal);
            let incoming_far = tl.incoming_far_modal(time, &refs.near_modal);
            let eq_near = tl.port_equivalent_current(&incoming_near);
            let eq_far = tl.port_equivalent_current(&incoming_far);

            Self::stamp_shared_reference_port(
                matrix,
                rhs,
                &tl.near_nodes,
                tl.near_ref,
                tl.port_admittance(),
                &eq_near,
            );
            Self::stamp_shared_reference_port(
                matrix,
                rhs,
                &tl.far_nodes,
                tl.far_ref,
                tl.port_admittance(),
                &eq_far,
            );
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
    fn initialize_coupled_tline_history(
        circuit: &mut crate::circuit::Circuit,
        initial_solution: &[Value],
        initial_time: Value,
    ) -> Vec<CoupledTlineReferenceState> {
        let mut refs = Vec::with_capacity(circuit.coupled_tlines.len());
        for tl in &mut circuit.coupled_tlines {
            tl.reset();
            let near_physical =
                Self::differential_port_voltages(initial_solution, &tl.near_nodes, tl.near_ref);
            let far_physical =
                Self::differential_port_voltages(initial_solution, &tl.far_nodes, tl.far_ref);
            let near_modal = tl.modalize_port_voltage(&near_physical);
            let far_modal = tl.modalize_port_voltage(&far_physical);
            let near_currents = tl.port_currents(&near_physical, &far_modal);
            let far_currents = tl.port_currents(&far_physical, &near_modal);
            let near_modal_currents = tl.modalize_port_current(&near_currents);
            let far_modal_currents = tl.modalize_port_current(&far_currents);
            tl.update_modal_history(
                initial_time,
                &near_modal,
                &near_modal_currents,
                &far_modal,
                &far_modal_currents,
            );
            refs.push(CoupledTlineReferenceState {
                near_modal,
                far_modal,
            });
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
        accepted_scale: Option<Value>,
    ) {
        let scale = if is_strictly_linear_transient {
            1.0
        } else if let Some(scale) = accepted_scale {
            scale
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
    fn nonconvergence_retry_timestep(dt: Value, max_step: Value) -> Value {
        if !dt.is_finite() || dt <= 0.0 {
            0.0
        } else {
            // Mirror ngspice's DCtran non-convergence recovery: retract the
            // failed timepoint and retry at one eighth of the rejected step.
            (dt * 0.125).min(max_step)
        }
    }

    #[inline]
    fn apply_retry_timestep_floor(
        proposed_dt: Value,
        retry_floor_dt: Option<Value>,
        max_step: Value,
    ) -> Value {
        let mut dt = proposed_dt.min(max_step);
        if let Some(floor) = retry_floor_dt
            .filter(|floor| floor.is_finite() && *floor > 0.0)
            .map(|floor| floor.min(max_step))
        {
            dt = dt.max(floor);
        }
        dt
    }

    #[inline]
    fn is_at_effective_retry_minimum(
        timestep: &TimestepController,
        retry_floor_dt: Option<Value>,
    ) -> bool {
        let effective_min = retry_floor_dt
            .filter(|floor| floor.is_finite() && *floor > 0.0)
            .map(|floor| floor.max(timestep.hard_min_dt()))
            .unwrap_or_else(|| timestep.hard_min_dt());
        timestep.dt() <= effective_min * 1.001
    }

    #[inline]
    fn should_skip_post_accept_timestep_control_on_first_step(
        accepted_point_count_before_push: usize,
    ) -> bool {
        // ngspice accepts the first transient point without any post-accept
        // truncation/LTE check, then retries the same delta on the next step.
        accepted_point_count_before_push <= 1
    }

    #[inline]
    fn force_accept_recovery_timestep(
        dt: Value,
        preferred_min_dt: Value,
        max_step: Value,
        vbic_exact_limit: Option<Value>,
    ) -> Value {
        let mut next_dt = if dt.is_finite() && dt > 0.0 {
            if dt < preferred_min_dt {
                (dt * preferred_min_dt)
                    .sqrt()
                    .max(dt)
                    .min(preferred_min_dt)
                    .min(max_step)
            } else {
                (dt * 0.5).max(preferred_min_dt).min(max_step)
            }
        } else {
            preferred_min_dt.min(max_step)
        };
        if let Some(limit) = vbic_exact_limit.filter(|limit| limit.is_finite() && *limit > 0.0) {
            next_dt = next_dt.min(limit.min(max_step));
        }
        next_dt
    }

    #[inline]
    fn limit_transient_node_voltage_updates(
        proposal: &mut [Value],
        previous: &[Value],
        num_nodes: usize,
        delta_limit: Value,
        protected_nodes: &[bool],
    ) -> bool {
        let mut changed = false;
        for i in 0..num_nodes {
            if protected_nodes.get(i).copied().unwrap_or(false) {
                continue;
            }
            let old = previous[i];
            let delta = proposal[i] - old;
            if delta.is_finite() && delta.abs() > delta_limit {
                proposal[i] = old + delta.signum() * delta_limit;
                changed = true;
            }
        }
        changed
    }

    #[inline]
    fn limit_vbic_transient_external_updates(
        circuit: &crate::circuit::Circuit,
        proposal: &mut [Value],
        previous: &[Value],
        accepted: &[Value],
        num_nodes: usize,
        protected_nodes: &[bool],
        accepted_delta_limit: Value,
    ) -> bool {
        let mut changed = Self::limit_vbic_external_updates(
            circuit,
            proposal,
            previous,
            num_nodes,
            Some(protected_nodes),
            true,
        );
        if !std::ptr::eq(previous.as_ptr(), accepted.as_ptr()) {
            changed |= Self::limit_vbic_external_updates(
                circuit,
                proposal,
                accepted,
                num_nodes,
                Some(protected_nodes),
                true,
            );
        }
        if accepted_delta_limit.is_finite() && accepted_delta_limit > 0.0 {
            for bjt in &circuit.bjts.devices {
                if !bjt.uses_vbic_dynamic_charges() || bjt.td <= 0.0 {
                    continue;
                }
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
                        || protected_nodes.get(proposal_idx).copied().unwrap_or(false)
                    {
                        continue;
                    }
                    let accepted_value = accepted[proposal_idx];
                    let proposal_value = proposal[proposal_idx];
                    let delta = proposal_value - accepted_value;
                    if !delta.is_finite() || delta.abs() <= accepted_delta_limit {
                        continue;
                    }
                    proposal[proposal_idx] = accepted_value + delta.signum() * accepted_delta_limit;
                    changed = true;
                }
            }
        }
        changed
    }

    #[inline]
    fn bounded_force_accept_candidate(
        circuit: &crate::circuit::Circuit,
        previous_solution: &[Value],
        candidate_solution: &[Value],
        accepted_time: Value,
        num_nodes: usize,
        force_accept_delta_limit: Value,
        protected_nodes: &[bool],
        ideal_output_pairs: &[(crate::NodeId, crate::NodeId)],
    ) -> Vec<Value> {
        let mut bounded = candidate_solution.to_vec();
        for i in 0..num_nodes {
            if protected_nodes.get(i).copied().unwrap_or(false) {
                continue;
            }
            let old = previous_solution[i];
            let delta = bounded[i] - old;
            if delta.is_finite() && delta.abs() > force_accept_delta_limit {
                bounded[i] = old + delta.signum() * force_accept_delta_limit;
            }
        }
        circuit.enforce_ideal_voltage_constraints(&mut bounded, accepted_time);
        // Force-accept is a last-resort recovery path, so keep every ideal
        // output supernode close to the previous accepted common mode instead of
        // letting protected source nodes drag a nonphysical midpoint into the
        // newly accepted state.
        Self::clip_ideal_output_common_modes(
            previous_solution,
            &mut bounded,
            force_accept_delta_limit,
            ideal_output_pairs,
        );
        Self::restore_algebraic_branch_currents(
            circuit,
            previous_solution,
            &mut bounded,
            num_nodes,
        );
        bounded
    }

    #[inline]
    fn restore_algebraic_branch_currents(
        circuit: &crate::circuit::Circuit,
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        num_nodes: usize,
    ) {
        let mut restore_branch = |branch_ordinal: usize| {
            if branch_ordinal == 0 {
                return;
            }
            let Some(solution_idx) = num_nodes.checked_add(branch_ordinal - 1) else {
                return;
            };
            let Some(previous_value) = previous_solution.get(solution_idx).copied() else {
                return;
            };
            if let Some(candidate_value) = candidate_solution.get_mut(solution_idx) {
                *candidate_value = previous_value;
            }
        };

        for &branch_ordinal in &circuit.voltage_sources.branch_indices {
            restore_branch(branch_ordinal as usize);
        }
        // Keep dependent-source algebraic currents from the latest solver
        // candidate. Their output-branch currents directly close KCL at the
        // controlled output nodes, so blindly snapping them back to the
        // previously accepted state can inject a large node residual even when
        // the committed output voltage satisfies the ideal source relation.
    }

    #[inline]
    fn clip_floating_ideal_output_common_modes(
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        common_mode_delta_limit: Value,
        floating_ideal_output_pairs: &[(crate::NodeId, crate::NodeId)],
    ) {
        Self::clip_ideal_output_common_modes(
            previous_solution,
            candidate_solution,
            common_mode_delta_limit,
            floating_ideal_output_pairs,
        );
    }

    #[inline]
    fn clip_ideal_output_common_modes(
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        common_mode_delta_limit: Value,
        ideal_output_pairs: &[(crate::NodeId, crate::NodeId)],
    ) {
        for &(node_pos, node_neg) in ideal_output_pairs {
            Self::clip_two_terminal_common_mode(
                previous_solution,
                candidate_solution,
                node_pos,
                node_neg,
                common_mode_delta_limit,
            );
        }
    }

    #[inline]
    fn clip_two_terminal_common_mode(
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        node_pos: usize,
        node_neg: usize,
        common_mode_delta_limit: Value,
    ) {
        if common_mode_delta_limit <= 0.0 || node_pos == 0 || node_neg == 0 {
            return;
        }

        let vp_idx = node_pos - 1;
        let vn_idx = node_neg - 1;
        if vp_idx >= previous_solution.len()
            || vn_idx >= previous_solution.len()
            || vp_idx >= candidate_solution.len()
            || vn_idx >= candidate_solution.len()
        {
            return;
        }

        let prev_vp = previous_solution[vp_idx];
        let prev_vn = previous_solution[vn_idx];
        let cand_vp = candidate_solution[vp_idx];
        let cand_vn = candidate_solution[vn_idx];
        if !(prev_vp.is_finite()
            && prev_vn.is_finite()
            && cand_vp.is_finite()
            && cand_vn.is_finite())
        {
            return;
        }

        let prev_midpoint = 0.5 * (prev_vp + prev_vn);
        let cand_midpoint = 0.5 * (cand_vp + cand_vn);
        let midpoint_delta = cand_midpoint - prev_midpoint;
        if midpoint_delta.abs() <= common_mode_delta_limit {
            return;
        }

        let clipped_midpoint = prev_midpoint + midpoint_delta.signum() * common_mode_delta_limit;
        let half_diff = 0.5 * (cand_vp - cand_vn);
        candidate_solution[vp_idx] = clipped_midpoint + half_diff;
        candidate_solution[vn_idx] = clipped_midpoint - half_diff;
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
        vbic_snapshots: Option<&[Option<BjtChargeSnapshot>]>,
        suppress_gate_charge_history: bool,
        tline_dc_refs: &[(Value, Value)],
        coupled_tline_refs: &[CoupledTlineReferenceState],
        breakpoints: &mut BreakpointManager,
        tstop: Value,
        voltage_reltol: Value,
        voltage_abstol: Value,
        dynamic_breakpoints_added: &mut usize,
        warned_dynamic_breakpoint_cap: &mut bool,
    ) {
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_new = Self::differential_voltage(accepted_solution, np, nn);

            // Compute new capacitor current from OLD history before rotating it.
            let coeff_update = CompanionCoefficients::for_method(Self::effective_companion_method(
                method, trap_order,
            ));
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
            circuit.capacitors.v_prev_prev_prev[cap_idx] = circuit.capacitors.v_prev_prev[cap_idx];
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
        circuit.update_coupled_inductor_pair_state(accepted_solution);
        circuit.update_multi_winding_transformer_state(accepted_solution);
        circuit.refresh_jiles_atherton_inductances(accepted_solution);

        // Update transmission-line delayed-wave history from the accepted state.
        for (idx, tl) in circuit.tlines.iter_mut().enumerate() {
            let previous_forward = tl.launched_forward_wave();
            let previous_backward = tl.launched_backward_wave();
            let v1 = Self::differential_voltage(accepted_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(accepted_solution, tl.node2_pos, tl.node2_neg);
            let (_v1_ref, _v2_ref) = tline_dc_refs.get(idx).copied().unwrap_or((0.0, 0.0));
            let response = tl.transient_port_response(accepted_time);
            let (i1_actual, i2_actual) = response.port_currents(v1, v2);
            tl.update_history(accepted_time, v1, i1_actual, v2, i2_actual);
            if !tl.has_distributed_rlgc() {
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    accepted_time,
                    tl.delay(),
                    tstop,
                    previous_forward,
                    tl.launched_forward_wave(),
                    voltage_reltol,
                    voltage_abstol,
                    dynamic_breakpoints_added,
                    warned_dynamic_breakpoint_cap,
                );
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    accepted_time,
                    tl.delay(),
                    tstop,
                    previous_backward,
                    tl.launched_backward_wave(),
                    voltage_reltol,
                    voltage_abstol,
                    dynamic_breakpoints_added,
                    warned_dynamic_breakpoint_cap,
                );
            }
        }

        for (idx, tl) in circuit.coupled_tlines.iter_mut().enumerate() {
            let previous_mode_launches = tl.launched_modal_waves().collect::<Vec<_>>();
            let refs = coupled_tline_refs.get(idx).cloned().unwrap_or_default();
            let near_physical =
                Self::differential_port_voltages(accepted_solution, &tl.near_nodes, tl.near_ref);
            let far_physical =
                Self::differential_port_voltages(accepted_solution, &tl.far_nodes, tl.far_ref);
            let near_modal = tl.modalize_port_voltage(&near_physical);
            let far_modal = tl.modalize_port_voltage(&far_physical);
            let incoming_near = tl.incoming_near_modal(accepted_time, &refs.far_modal);
            let incoming_far = tl.incoming_far_modal(accepted_time, &refs.near_modal);
            let near_currents = tl.port_currents(&near_physical, &incoming_near);
            let far_currents = tl.port_currents(&far_physical, &incoming_far);
            let near_modal_currents = tl.modalize_port_current(&near_currents);
            let far_modal_currents = tl.modalize_port_current(&far_currents);
            tl.update_modal_history(
                accepted_time,
                &near_modal,
                &near_modal_currents,
                &far_modal,
                &far_modal_currents,
            );
            for (
                (delay, previous_forward, previous_backward),
                (_, current_forward, current_backward),
            ) in previous_mode_launches
                .into_iter()
                .zip(tl.launched_modal_waves())
            {
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    accepted_time,
                    delay,
                    tstop,
                    previous_forward,
                    current_forward,
                    voltage_reltol,
                    voltage_abstol,
                    dynamic_breakpoints_added,
                    warned_dynamic_breakpoint_cap,
                );
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    accepted_time,
                    delay,
                    tstop,
                    previous_backward,
                    current_backward,
                    voltage_reltol,
                    voltage_abstol,
                    dynamic_breakpoints_added,
                    warned_dynamic_breakpoint_cap,
                );
            }
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            let vc = Self::node_voltage(accepted_solution, bjt.node_collector);
            let vb = Self::node_voltage(accepted_solution, bjt.node_base);
            let ve = Self::node_voltage(accepted_solution, bjt.node_emitter);
            let vs = Self::node_voltage(accepted_solution, bjt.node_substrate);
            let external = [vc, vb, ve, vs];
            let vbe = vb - ve;
            let vbc = vb - vc;
            let vcs = vc - vs;
            if bjt.uses_vbic_dynamic_charges() {
                let snapshot_reuse_abstol = voltage_abstol.min(VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL);
                let snapshot_reuse_reltol = voltage_reltol.min(VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL);
                let cached_snapshot = vbic_snapshots
                    .and_then(|cache| cache.get(idx))
                    .copied()
                    .flatten();
                let Some(snapshot) =
                    Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                        bjt,
                        external,
                        method,
                        trap_order,
                        dt,
                        &bjt_history.charge_q_prev[idx],
                        &bjt_history.charge_q_prev_prev[idx],
                        &bjt_history.charge_cq_prev[idx],
                        bjt_history.dynamic_internal_prev.get(idx),
                        bjt_history.dynamic_internal_prev_prev.get(idx),
                        bjt_history.dynamic_linear_prev.get(idx),
                        bjt_history.dynamic_linear_prev_prev.get(idx),
                        bjt_history.accepted_dt_prev,
                        cached_snapshot,
                        VbicCachedSnapshotReuse::SeedOnly,
                        snapshot_reuse_abstol,
                        snapshot_reuse_reltol,
                    )
                else {
                    continue;
                };
                for (branch_idx, branch) in snapshot.branches.iter().enumerate() {
                    let q_prev = bjt_history.charge_q_prev[idx][branch_idx];
                    let q_prev_prev = bjt_history.charge_q_prev_prev[idx][branch_idx];
                    let cq_prev = bjt_history.charge_cq_prev[idx][branch_idx];
                    let cq_curr = Self::jfet_companion_ccap(
                        effective_method,
                        trap_order,
                        dt,
                        branch.charge,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                    );
                    bjt_history.charge_q_prev_prev_prev[idx][branch_idx] = q_prev_prev;
                    bjt_history.charge_q_prev_prev[idx][branch_idx] = q_prev;
                    bjt_history.charge_q_prev[idx][branch_idx] = branch.charge;
                    bjt_history.charge_cq_prev[idx][branch_idx] = cq_curr;
                }
                bjt_history.dynamic_internal_prev_prev[idx] =
                    bjt_history.dynamic_internal_prev[idx];
                bjt_history.dynamic_internal_prev[idx] = snapshot.reduction.internal_voltages;
                let predictor_linear = Self::vbic_predictor_linear_branch_state(
                    bjt,
                    external,
                    snapshot.reduction.internal_voltages,
                );
                bjt_history.dynamic_linear_prev_prev[idx] = bjt_history.dynamic_linear_prev[idx];
                bjt_history.dynamic_linear_prev[idx] = predictor_linear;
                bjt_history.vbe_prev_prev[idx] = bjt_history.vbe_prev[idx];
                bjt_history.vbe_prev[idx] = vbe;
                bjt_history.ibe_prev[idx] = 0.0;
                bjt_history.vbc_prev_prev[idx] = bjt_history.vbc_prev[idx];
                bjt_history.vbc_prev[idx] = vbc;
                bjt_history.ibc_prev[idx] = 0.0;
                bjt_history.vcs_prev_prev[idx] = bjt_history.vcs_prev[idx];
                bjt_history.vcs_prev[idx] = vcs;
                bjt_history.ics_prev[idx] = 0.0;
                continue;
            }
            let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
            let (legacy_vbe, legacy_vbc, legacy_vcs) =
                Self::legacy_bjt_charge_branch_voltages(&snapshot);
            let charges = bjt.legacy_transient_charge_state(legacy_vbe, legacy_vbc, legacy_vcs);
            let mut update_legacy_charge_branch =
                |branch_idx: usize, capacitance: Value, voltage: Value, charge: Value| {
                    let q_prev = bjt_history.charge_q_prev[idx][branch_idx];
                    let q_prev_prev = bjt_history.charge_q_prev_prev[idx][branch_idx];
                    let cq_prev = bjt_history.charge_cq_prev[idx][branch_idx];
                    let (_geq, _ieq, q_curr, cq_curr) = if capacitance > 0.0 {
                        Self::nonlinear_charge_companion_terms(
                            effective_method,
                            trap_order,
                            dt,
                            capacitance,
                            voltage,
                            charge,
                            q_prev,
                            q_prev_prev,
                            cq_prev,
                        )
                    } else {
                        (0.0, 0.0, 0.0, 0.0)
                    };
                    bjt_history.charge_q_prev_prev_prev[idx][branch_idx] = q_prev_prev;
                    bjt_history.charge_q_prev_prev[idx][branch_idx] = q_prev;
                    bjt_history.charge_q_prev[idx][branch_idx] = q_curr;
                    bjt_history.charge_cq_prev[idx][branch_idx] = cq_curr;
                    cq_curr
                };

            let qbe_current = update_legacy_charge_branch(
                BJT_QBE_BRANCH_INDEX,
                charges.capbe,
                legacy_vbe,
                charges.qbe,
            );
            let qbc_current = update_legacy_charge_branch(
                BJT_QBC_BRANCH_INDEX,
                charges.capbc,
                legacy_vbc,
                charges.qbc,
            );
            let qcs_current = update_legacy_charge_branch(
                BJT_QBCP_BRANCH_INDEX,
                charges.capcs,
                legacy_vcs,
                charges.qcs,
            );

            bjt_history.vbe_prev_prev[idx] = bjt_history.vbe_prev[idx];
            bjt_history.vbe_prev[idx] = legacy_vbe;
            bjt_history.ibe_prev[idx] = qbe_current;
            bjt_history.vbc_prev_prev[idx] = bjt_history.vbc_prev[idx];
            bjt_history.vbc_prev[idx] = legacy_vbc;
            bjt_history.ibc_prev[idx] = qbc_current;
            bjt_history.vcs_prev_prev[idx] = bjt_history.vcs_prev[idx];
            bjt_history.vcs_prev[idx] = legacy_vcs;
            bjt_history.ics_prev[idx] = qcs_current;
        }

        bjt_history.accepted_dt_prev_prev = bjt_history.accepted_dt_prev;
        bjt_history.accepted_dt_prev = dt;

        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, accepted_solution);
            let (vgs_charge, vgd_charge) =
                Self::jfet_charge_branch_voltages(jfet, accepted_solution);
            let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);
            jfet_history.vgs_prev_prev[idx] = jfet_history.vgs_prev[idx];
            jfet_history.vgs_prev[idx] = vgs_charge;
            jfet_history.vgd_prev_prev[idx] = jfet_history.vgd_prev[idx];
            jfet_history.vgd_prev[idx] = vgd_charge;
            if !suppress_gate_charge_history {
                let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgs,
                    vgs_charge,
                    jfet_history.vgs_prev_prev[idx],
                    jfet_history.qgs_prev[idx],
                    jfet_history.qgs_prev_prev[idx],
                    jfet_history.cqgs_prev[idx],
                );
                jfet_history.qgs_prev_prev_prev[idx] = jfet_history.qgs_prev_prev[idx];
                jfet_history.qgs_prev_prev[idx] = jfet_history.qgs_prev[idx];
                jfet_history.qgs_prev[idx] = qgs_curr;
                jfet_history.cqgs_prev[idx] = cqgs_curr;

                let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgd,
                    vgd_charge,
                    jfet_history.vgd_prev_prev[idx],
                    jfet_history.qgd_prev[idx],
                    jfet_history.qgd_prev_prev[idx],
                    jfet_history.cqgd_prev[idx],
                );
                jfet_history.qgd_prev_prev_prev[idx] = jfet_history.qgd_prev_prev[idx];
                jfet_history.qgd_prev_prev[idx] = jfet_history.qgd_prev[idx];
                jfet_history.qgd_prev[idx] = qgd_curr;
                jfet_history.cqgd_prev[idx] = cqgd_curr;
            }
        }
        jfet_history.accepted_dt_prev_prev = jfet_history.accepted_dt_prev;
        jfet_history.accepted_dt_prev = dt;

        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (vgs, vds, vbs) = mos.eval_branch_voltages_at(accepted_solution);
            let vgd = vgs - vds;
            let vgb = vgs - vbs;
            let (cgs_half, cgd_half, cgb_half) = mos.transient_capacitance_halves_at(vgs, vds, vbs);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
            let cgs = cgs_half + mosfet_history.capgs_prev_half[idx] + cgs_ov;
            let cgd = cgd_half + mosfet_history.capgd_prev_half[idx] + cgd_ov;
            let cgb = cgb_half + mosfet_history.capgb_prev_half[idx] + cgb_ov;
            mosfet_history.vgs_prev_prev[idx] = mosfet_history.vgs_prev[idx];
            mosfet_history.vgs_prev[idx] = vgs;
            mosfet_history.capgs_prev_half[idx] = cgs_half;
            mosfet_history.vgd_prev_prev[idx] = mosfet_history.vgd_prev[idx];
            mosfet_history.vgd_prev[idx] = vgd;
            mosfet_history.capgd_prev_half[idx] = cgd_half;
            mosfet_history.vgb_prev_prev[idx] = mosfet_history.vgb_prev[idx];
            mosfet_history.vgb_prev[idx] = vgb;
            mosfet_history.capgb_prev_half[idx] = cgb_half;
            if !suppress_gate_charge_history {
                let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgs,
                    vgs,
                    mosfet_history.vgs_prev_prev[idx],
                    mosfet_history.qgs_prev[idx],
                    mosfet_history.qgs_prev_prev[idx],
                    mosfet_history.cqgs_prev[idx],
                );
                mosfet_history.qgs_prev_prev_prev[idx] = mosfet_history.qgs_prev_prev[idx];
                mosfet_history.qgs_prev_prev[idx] = mosfet_history.qgs_prev[idx];
                mosfet_history.qgs_prev[idx] = qgs_curr;
                mosfet_history.cqgs_prev[idx] = cqgs_curr;

                let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgd,
                    vgd,
                    mosfet_history.vgd_prev_prev[idx],
                    mosfet_history.qgd_prev[idx],
                    mosfet_history.qgd_prev_prev[idx],
                    mosfet_history.cqgd_prev[idx],
                );
                mosfet_history.qgd_prev_prev_prev[idx] = mosfet_history.qgd_prev_prev[idx];
                mosfet_history.qgd_prev_prev[idx] = mosfet_history.qgd_prev[idx];
                mosfet_history.qgd_prev[idx] = qgd_curr;
                mosfet_history.cqgd_prev[idx] = cqgd_curr;

                let (_geq_gb, _ieq_gb, qgb_curr, cqgb_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgb,
                    vgb,
                    mosfet_history.vgb_prev_prev[idx],
                    mosfet_history.qgb_prev[idx],
                    mosfet_history.qgb_prev_prev[idx],
                    mosfet_history.cqgb_prev[idx],
                );
                mosfet_history.qgb_prev_prev_prev[idx] = mosfet_history.qgb_prev_prev[idx];
                mosfet_history.qgb_prev_prev[idx] = mosfet_history.qgb_prev[idx];
                mosfet_history.qgb_prev[idx] = qgb_curr;
                mosfet_history.cqgb_prev[idx] = cqgb_curr;
            }

            let vbs_j = mos.body_source_charge_branch_voltage(vbs);
            let vbd_j = mos.body_drain_charge_branch_voltage(vds, vbs);
            let (qbs_exact, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs);
            let (_geq_bs, _ieq_bs, qbs_curr, cqbs_curr) = Self::nonlinear_charge_companion_terms(
                method,
                trap_order,
                dt,
                cbs,
                vbs_j,
                qbs_exact,
                mosfet_history.qbs_prev[idx],
                mosfet_history.qbs_prev_prev[idx],
                mosfet_history.cqbs_prev[idx],
            );
            mosfet_history.vbs_j_prev_prev[idx] = mosfet_history.vbs_j_prev[idx];
            mosfet_history.vbs_j_prev[idx] = vbs_j;
            mosfet_history.qbs_prev_prev[idx] = mosfet_history.qbs_prev[idx];
            mosfet_history.qbs_prev[idx] = qbs_curr;
            mosfet_history.cqbs_prev[idx] = cqbs_curr;

            let (qbd_exact, cbd) = mos.body_drain_junction_charge_and_capacitance_at(vds, vbs);
            let (_geq_bd, _ieq_bd, qbd_curr, cqbd_curr) = Self::nonlinear_charge_companion_terms(
                method,
                trap_order,
                dt,
                cbd,
                vbd_j,
                qbd_exact,
                mosfet_history.qbd_prev[idx],
                mosfet_history.qbd_prev_prev[idx],
                mosfet_history.cqbd_prev[idx],
            );
            mosfet_history.vbd_j_prev_prev[idx] = mosfet_history.vbd_j_prev[idx];
            mosfet_history.vbd_j_prev[idx] = vbd_j;
            mosfet_history.qbd_prev_prev[idx] = mosfet_history.qbd_prev[idx];
            mosfet_history.qbd_prev[idx] = qbd_curr;
            mosfet_history.cqbd_prev[idx] = cqbd_curr;
        }
        mosfet_history.accepted_dt_prev_prev = mosfet_history.accepted_dt_prev;
        mosfet_history.accepted_dt_prev = dt;
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn transient_nonlinear_residual_converged(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        dt: Value,
        coeff: &CompanionCoefficients,
        current_method: IntegrationMethod,
        step_trap_order: u8,
        bjt_history: &BjtTransientHistory,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        jfet_history: &JfetTransientHistory,
        mosfet_history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
        tline_dc_refs: &[(Value, Value)],
        coupled_tline_refs: &[CoupledTlineReferenceState],
    ) -> bool {
        if solution.iter().any(|value| !value.is_finite()) {
            return false;
        }

        let num_nodes = circuit.num_nodes();
        matrix.clear_values();
        rhs.fill(0.0);

        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        if gmin_floor > 0.0 {
            for i in 0..num_nodes {
                matrix.add(i, i, gmin_floor);
            }
        }

        circuit.stamp_transient_linear_direct(matrix, rhs);
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);
        circuit.refresh_jiles_atherton_inductances(solution);

        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(solution);
        }

        circuit
            .capacitors
            .stamp_transient_companion(matrix, rhs, dt, coeff);
        circuit
            .inductors
            .stamp_transient_companion(matrix, rhs, dt, coeff, num_nodes);
        circuit.stamp_coupled_inductor_pairs_transient(matrix, rhs, dt, coeff);
        circuit.stamp_multi_winding_transformers_transient(matrix, rhs, dt, coeff);

        Self::stamp_bjt_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            current_method,
            step_trap_order,
            dt,
            bjt_history,
            vbic_snapshot_cache,
            VbicCachedSnapshotReuse::SeedOnly,
            self.voltage_abstol(),
            self.voltage_reltol(),
        );
        Self::stamp_jfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            current_method,
            step_trap_order,
            dt,
            jfet_history,
            suppress_gate_charge,
        );
        Self::stamp_mosfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            current_method,
            step_trap_order,
            dt,
            mosfet_history,
            suppress_gate_charge,
        );
        Self::stamp_tline_companions(circuit, matrix, rhs, time, tline_dc_refs);
        Self::stamp_coupled_tline_companions(circuit, matrix, rhs, time, coupled_tline_refs);

        if circuit.has_nonlinear_devices() {
            circuit.stamp_nonlinear(matrix, rhs, solution);
            circuit.stamp_behavioral(matrix, rhs, solution, time);
        }

        if circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_timestep(time, dt, solution);
            circuit.stamp_xspice(matrix, rhs);
        }

        self.residual_convergence_met(matrix, solution, rhs)
    }

    #[inline]
    fn should_prefer_dense_transient_solver(
        is_strictly_linear_transient: bool,
        size: usize,
        has_transformer_or_coupled_inductor: bool,
        has_xspice_devices: bool,
    ) -> bool {
        if is_strictly_linear_transient {
            return size <= 160 && has_transformer_or_coupled_inductor;
        }

        // Small nonlinear systems benefit from dense LU due to reduced sparse
        // symbolic/indirection overhead at this scale.
        !has_xspice_devices && size <= 64
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
        if circuit.num_nodes() == 0 && circuit.num_branches() == 0 {
            return Ok(TransientResult {
                time: vec![0.0],
                voltages: Vec::new(),
                branch_currents: Vec::new(),
                num_nodes: 0,
                node_names: Vec::new(),
                branch_names: Vec::new(),
            });
        }
        let hinted_max_step = circuit
            .transient_max_step_hint
            .map_or(max_step, |hint| max_step.min(hint));
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let source_step_hint = Self::transient_source_step_hint(netlist, hinted_max_step);
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
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&solution);
        }
        circuit.refresh_jiles_atherton_inductances(&solution);

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();
        let requires_conservative_nonlinear_limiting = circuit.has_physical_nonlinear_devices();
        let enforce_force_candidate_safety =
            requires_conservative_nonlinear_limiting || circuit.has_xspice_devices();
        let is_strictly_linear_transient =
            !circuit.has_nonlinear_devices() && !circuit.has_xspice_devices();
        let prefer_dense_solver = Self::should_prefer_dense_transient_solver(
            is_strictly_linear_transient,
            size,
            !circuit.multi_winding_transformers.is_empty()
                || !circuit.coupled_inductor_pairs.is_empty(),
            circuit.has_xspice_devices(),
        );

        // Initialize timestep controller.
        // BJT-heavy decks (notably VBIC regression circuits) need a smaller startup
        // timestep to capture sub-ns bias settling that ngspice resolves before
        // transitioning to larger steps.
        let has_bjts = !circuit.bjts.devices.is_empty();
        let has_vbic_dynamic_charges = circuit
            .bjts
            .devices
            .iter()
            .any(|bjt| bjt.uses_vbic_dynamic_charges());
        let has_vbic_excess_phase = circuit
            .bjts
            .devices
            .iter()
            .any(|bjt| bjt.uses_vbic_dynamic_charges() && bjt.td > 0.0);
        let smallest_vbic_excess_phase_td = circuit
            .bjts
            .devices
            .iter()
            .filter(|bjt| bjt.uses_vbic_dynamic_charges() && bjt.td.is_finite() && bjt.td > 0.0)
            .map(|bjt| bjt.td)
            .min_by(|lhs, rhs| lhs.total_cmp(rhs));
        let (_startup_div, min_div) =
            Self::startup_timestep_divisors(has_bjts, has_vbic_excess_phase);
        let tran_step_hint = netlist.analyses.iter().find_map(|analysis| match analysis {
            AnalysisCommand::Tran { step, .. } if step.is_finite() && *step > 0.0 => Some(*step),
            _ => None,
        });
        let initial_step = Self::ngspice_initial_timestep(tstop, tran_step_hint, hinted_max_step);
        let practical_min = Self::startup_practical_min_timestep_with_vbic_td(
            has_bjts,
            has_vbic_excess_phase,
            hinted_max_step,
            min_div,
            tran_step_hint,
            smallest_vbic_excess_phase_td,
        );
        let preferred_min_dt = practical_min.max(self.config.min_timestep.max(1e-15));
        let hard_min_dt = Self::ngspice_hard_min_timestep(hinted_max_step, preferred_min_dt);
        let mut timestep = TimestepController::new_with_preferred_min(
            initial_step,
            hard_min_dt,
            preferred_min_dt,
            hinted_max_step,
        );
        let mut breakpoints = BreakpointManager::new();
        Self::collect_transient_source_breakpoints(
            &circuit,
            tstop,
            source_step_hint,
            &mut breakpoints,
        );
        let source_breakpoint_times = breakpoints.times().to_vec();
        Self::collect_transient_tline_breakpoints(
            &circuit,
            &source_breakpoint_times,
            tstop,
            &mut breakpoints,
        );
        let mut dynamic_tline_breakpoints_added = 0_usize;
        let mut warned_dynamic_tline_breakpoint_cap = false;
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
        let effective_method_order = |method: IntegrationMethod, trap_order: u8| -> u32 {
            match method {
                IntegrationMethod::BackwardEuler => 1,
                IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear if trap_order <= 1 => {
                    1
                }
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

        let branch_names = circuit.branch_names_sorted();
        let mut result = TransientResult {
            time: vec![0.0],
            voltages: (0..num_nodes)
                .map(|i| vec![solution.get(i).copied().unwrap_or(0.0)])
                .collect(),
            branch_currents: (0..circuit.num_branches())
                .map(|i| vec![solution.get(num_nodes + i).copied().unwrap_or(0.0)])
                .collect(),
            num_nodes,
            node_names,
            branch_names,
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
            circuit.capacitors.v_prev_prev_prev[cap_idx] = v_dc;
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
        circuit.update_coupled_inductor_pair_state(&solution);
        circuit.update_multi_winding_transformer_state(&solution);
        let tline_dc_refs = Self::initialize_tline_history(&mut circuit, &solution, 0.0);
        let coupled_tline_refs =
            Self::initialize_coupled_tline_history(&mut circuit, &solution, 0.0);
        let mut bjt_history = Self::initialize_bjt_history(&circuit, &solution);
        // ngspice seeds CKTdeltaOld[] with maxstep before the first transient point.
        // Mirror that here so early VBIC truncation/order checks have the same
        // timestep history instead of falling back to a synthetic zero-history path.
        bjt_history.accepted_dt_prev = hinted_max_step;
        bjt_history.accepted_dt_prev_prev = hinted_max_step;
        let mut jfet_history = Self::initialize_jfet_history(&circuit, &solution);
        jfet_history.accepted_dt_prev = hinted_max_step;
        jfet_history.accepted_dt_prev_prev = hinted_max_step;
        let mut mosfet_history = Self::initialize_mosfet_history(&circuit, &solution);
        mosfet_history.accepted_dt_prev = hinted_max_step;
        mosfet_history.accepted_dt_prev_prev = hinted_max_step;
        let mut vbic_snapshot_cache = vec![None; circuit.bjts.devices.len()];
        let force_accept_protected_nodes = circuit.force_accept_protected_nodes();
        let ideal_output_pairs = circuit.ideal_voltage_output_pairs();

        // Main transient loop
        let mut retry_count = 0;
        let mut total_iterations = 0;
        let mut stale_accept_count = 0;
        let mut force_accept_cooldown = 0_usize; // Failed retries to defer dt shrink immediately after force-accept
        let mut trap_order = 1_u8; // ngspice-style trap order: start at 1, promote to 2 after accepted smooth step
        let mut inittran_gate_charge_phase = true;
        // Keep ngspice's exact VBICtrunc/CKTterr path authoritative for excess-phase
        // charge control, but still expose the reduced VBIC charge state to the
        // engine-level LTE controller so the broader transient loop can back off
        // before Newton falls into repeated delmin force-accept recovery.
        let mut vbic_charge_lte_estimator = has_vbic_excess_phase
            .then(|| LteEstimator::with_tolerances(self.voltage_reltol(), self.charge_abstol()));
        Self::record_vbic_truncation_charge_state(
            &mut vbic_charge_lte_estimator,
            &circuit,
            &solution,
            current_integration_method(&trapgear),
            trap_order,
            timestep.dt(),
            &bjt_history,
            Some(&vbic_snapshot_cache),
            effective_method_order(current_integration_method(&trapgear), trap_order),
        );
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
        let mut nonlinear_state_matches_solution = false;

        while t < tstop && total_iterations < max_total_iterations {
            // Progress logging every 2 seconds
            if last_progress_log.elapsed().as_secs() >= 2 {
                log::info!(
                    "Transient progress: t={:.12e}s / {:.3e}s ({:.1}%), dt={:.3e}, retries={}, order={}, {} iterations",
                    t,
                    tstop,
                    (t / tstop) * 100.0,
                    timestep.dt(),
                    retry_count,
                    trap_order,
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
            let mut dt = dt.min(tstop - t); // Don't overshoot tstop
            let mut expected_source_delta = Self::max_expected_source_delta(&circuit, t, t + dt);
            let biased_dt = Self::bias_transient_step_for_source_activity(
                dt,
                tstop - t,
                at_breakpoint,
                expected_source_delta,
                practical_min,
                timestep.preferred_min_dt(),
            );
            if biased_dt + 1e-30 < dt {
                dt = biased_dt;
                expected_source_delta = Self::max_expected_source_delta(&circuit, t, t + dt);
            }
            if let Some(vbic_startup_step_cap) = Self::vbic_excess_phase_startup_step_cap(
                hinted_max_step,
                smallest_vbic_excess_phase_td,
            )
            .filter(|cap| {
                has_vbic_excess_phase
                    && dt > *cap
                    && Self::should_use_vbic_charge_lte_startup_guard(
                        has_vbic_excess_phase,
                        t + dt,
                        hinted_max_step,
                        smallest_vbic_excess_phase_td,
                    )
            }) {
                dt = vbic_startup_step_cap;
                expected_source_delta = Self::max_expected_source_delta(&circuit, t, t + dt);
            }
            let step_time = t + dt;
            let retry_floor_source_activity_delta =
                Self::startup_source_activity_delta_for_retry_floor(
                    &circuit,
                    t,
                    dt,
                    tstop,
                    initial_step,
                    timestep.preferred_min_dt(),
                );
            let legacy_bjt_retry_floor_dt = Self::legacy_bjt_startup_retry_floor(
                has_bjts,
                has_vbic_excess_phase,
                step_time,
                hinted_max_step,
                retry_floor_source_activity_delta,
                initial_step,
                timestep.preferred_min_dt(),
            );
            let newton_step_delta_limit = Self::startup_step_delta_limit_with_vbic_td(
                initial_solution_mode,
                has_vbic_excess_phase,
                smallest_vbic_excess_phase_td,
                step_time,
                hinted_max_step,
                MAX_NEWTON_ITER_DELTA_V,
            );
            let force_accept_delta_limit = Self::startup_force_accept_delta_limit_with_vbic_td(
                initial_solution_mode,
                has_vbic_excess_phase,
                smallest_vbic_excess_phase_td,
                step_time,
                hinted_max_step,
                MAX_FORCE_ACCEPT_DELTA_V,
            );
            let current_method = current_integration_method(&trapgear);
            let step_trap_order =
                Self::effective_trapezoidal_order(current_method, trap_order, at_breakpoint);
            let coeff = CompanionCoefficients::for_method(Self::effective_companion_method(
                current_method,
                step_trap_order,
            ));
            let suppress_gate_charge = inittran_gate_charge_phase && t == 0.0;

            // Prepare for Newton iteration at this timestep by seeding the full
            // algebraic solution vector from accepted history when a predictor
            // state is available. ngspice's `NIpred()` predicts every solver
            // unknown, including branch-current equations, not just node
            // voltages. Matching that behavior materially improves the initial
            // Newton guess for source-heavy VBIC decks.
            if let Some(predicted_solution) =
                lte_estimator.predict_solution(dt, current_method, step_trap_order)
            {
                new_solution = predicted_solution;
            } else {
                new_solution.clone_from(&solution);
            }
            circuit.enforce_ideal_voltage_constraints(&mut new_solution, t + dt);
            Self::clip_ideal_output_common_modes(
                &solution,
                &mut new_solution,
                newton_step_delta_limit,
                &ideal_output_pairs,
            );
            for (i, value) in new_solution.iter_mut().enumerate() {
                let magnitude_limit = if i < num_nodes {
                    MAX_VOLTAGE
                } else {
                    MAX_BRANCH_STATE_MAGNITUDE
                };
                if !value.is_finite() {
                    *value = solution[i];
                } else if value.abs() > magnitude_limit {
                    let old = solution[i];
                    let delta = *value - old;
                    let limit = if i < num_nodes {
                        newton_step_delta_limit
                    } else {
                        magnitude_limit * 0.1
                    };
                    *value = if delta.is_finite() {
                        old + delta.signum() * limit
                    } else {
                        old
                    };
                }
            }
            if requires_conservative_nonlinear_limiting {
                let damped = Self::limit_transient_node_voltage_updates(
                    &mut new_solution,
                    &solution,
                    num_nodes,
                    newton_step_delta_limit,
                    &force_accept_protected_nodes,
                );
                if damped {
                    circuit.enforce_ideal_voltage_constraints(&mut new_solution, t + dt);
                }
                let vbic_damped = Self::limit_vbic_transient_external_updates(
                    &circuit,
                    &mut new_solution,
                    &solution,
                    &solution,
                    num_nodes,
                    &force_accept_protected_nodes,
                    newton_step_delta_limit,
                );
                if vbic_damped {
                    circuit.enforce_ideal_voltage_constraints(&mut new_solution, t + dt);
                }
                Self::clip_ideal_output_common_modes(
                    &solution,
                    &mut new_solution,
                    newton_step_delta_limit,
                    &ideal_output_pairs,
                );
            }
            let mut nonlinear_state_matches_new_solution = false;
            let mut had_solver_candidate = true;

            // Newton-Raphson iteration for this timestep.
            // Transient nonlinear regions (e.g., BJT turn-on) often need more
            // iterations than DC. Use a higher budget here to reduce force-accept.
            let tran_max_iterations = Self::transient_newton_iteration_budget(
                self.config.max_iterations,
                has_vbic_excess_phase,
                retry_count,
            );
            let mut converged = false;
            for _iter in 0..tran_max_iterations {
                let newton_stamp_start = std::time::Instant::now();
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
                if circuit.has_nonlinear_devices() && !nonlinear_state_matches_new_solution {
                    circuit.update_nonlinear(&new_solution);
                    nonlinear_state_matches_new_solution = true;
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
                circuit.stamp_coupled_inductor_pairs_transient(&mut matrix, &mut rhs, dt, &coeff);
                circuit.stamp_multi_winding_transformers_transient(
                    &mut matrix,
                    &mut rhs,
                    dt,
                    &coeff,
                );
                Self::stamp_bjt_transient_companions(
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &bjt_history,
                    &mut vbic_snapshot_cache,
                    if _iter == 0 {
                        VbicCachedSnapshotReuse::SeedOnly
                    } else {
                        VbicCachedSnapshotReuse::NewtonBypass
                    },
                    self.voltage_abstol(),
                    self.voltage_reltol(),
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
                    suppress_gate_charge,
                );
                Self::stamp_mosfet_transient_companions(
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &mosfet_history,
                    suppress_gate_charge,
                );
                Self::stamp_tline_companions(
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    t + dt,
                    &tline_dc_refs,
                );
                Self::stamp_coupled_tline_companions(
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    t + dt,
                    &coupled_tline_refs,
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
                let newton_stamp_elapsed = newton_stamp_start.elapsed();
                static TRANSIENT_NEWTON_STAMP_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                if newton_stamp_elapsed.as_millis() >= 100 {
                    let log_count = TRANSIENT_NEWTON_STAMP_LOG_COUNT
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_count < 40 {
                        log::warn!(
                            "Slow transient Newton stamp at t={:.6e}, dt={:.3e}, iter={}, elapsed={:.3?}",
                            t,
                            dt,
                            total_iterations,
                            newton_stamp_elapsed,
                        );
                    }
                }
                let newton_solve_start = std::time::Instant::now();
                let solve_result = if prefer_dense_solver {
                    matrix.solve_dense(&rhs)
                } else {
                    matrix.solve(&rhs)
                };
                let newton_solve_elapsed = newton_solve_start.elapsed();
                static TRANSIENT_NEWTON_SOLVE_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                if newton_solve_elapsed.as_millis() >= 100 {
                    let log_count = TRANSIENT_NEWTON_SOLVE_LOG_COUNT
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_count < 40 {
                        log::warn!(
                            "Slow transient Newton solve at t={:.6e}, dt={:.3e}, iter={}, elapsed={:.3?}",
                            t,
                            dt,
                            total_iterations,
                            newton_solve_elapsed,
                        );
                    }
                }

                match solve_result {
                    Ok(mut sol) => {
                        had_solver_candidate = true;
                        // Sanity check: detect and handle NaN/Inf/excessive values.
                        // IMPORTANT: Preserve the newest valid candidate when possible.
                        // If we keep the previous timestep guess here, force-accept can
                        // propagate a stale state and flatten non-source traces.
                        let mut has_bad_values = false;
                        let mut logged_divergence = false;

                        for (i, v) in sol.iter_mut().enumerate() {
                            let magnitude_limit = if i < num_nodes {
                                MAX_VOLTAGE
                            } else {
                                MAX_BRANCH_STATE_MAGNITUDE
                            };
                            if !v.is_finite() {
                                if !logged_divergence {
                                    log::debug!(
                                        "Transient: Newton divergence at t={:.3e}s, state {}: {:.3e} - reducing timestep",
                                        t + dt,
                                        i,
                                        *v
                                    );
                                    logged_divergence = true;
                                }
                                // Non-finite values cannot be used; fall back to prior guess.
                                *v = new_solution[i];
                                has_bad_values = true;
                            } else if v.abs() > magnitude_limit {
                                if !logged_divergence {
                                    log::debug!(
                                        "Transient: Newton divergence at t={:.3e}s, state {}: {:.3e} - reducing timestep",
                                        t + dt,
                                        i,
                                        *v
                                    );
                                    logged_divergence = true;
                                }
                                // Soft-limit finite overflow around the previous Newton
                                // guess instead of hard-clamping to a global rail. Hard
                                // clamps can be force-accepted and then contaminate
                                // dynamic history with nonphysical state.
                                let old = new_solution[i];
                                let delta = *v - old;
                                if delta.is_finite() {
                                    let limit = if i < num_nodes {
                                        newton_step_delta_limit
                                    } else {
                                        magnitude_limit * 0.1
                                    };
                                    *v = old + delta.signum() * limit;
                                } else {
                                    *v = old;
                                }
                                has_bad_values = true;
                            }
                        }

                        if requires_conservative_nonlinear_limiting {
                            // Trust-region damping is critical for stiff semiconductor
                            // nonlinearities, but it should not throttle linear decks or
                            // break ideal voltage-source equations by independently clipping
                            // their driven output nodes after each linear solve.
                            let damped = Self::limit_transient_node_voltage_updates(
                                &mut sol,
                                &new_solution,
                                num_nodes,
                                newton_step_delta_limit,
                                &force_accept_protected_nodes,
                            );
                            if damped {
                                circuit.enforce_ideal_voltage_constraints(&mut sol, t + dt);
                            }
                            let vbic_damped = Self::limit_vbic_transient_external_updates(
                                &circuit,
                                &mut sol,
                                &new_solution,
                                &solution,
                                num_nodes,
                                &force_accept_protected_nodes,
                                newton_step_delta_limit,
                            );
                            if vbic_damped {
                                circuit.enforce_ideal_voltage_constraints(&mut sol, t + dt);
                            }
                            Self::clip_ideal_output_common_modes(
                                &solution,
                                &mut sol,
                                newton_step_delta_limit,
                                &ideal_output_pairs,
                            );
                        }

                        // If this Newton step was numerically bad, keep the sanitized
                        // candidate and continue Newton iterations.
                        if has_bad_values {
                            new_solution = sol;
                            nonlinear_state_matches_new_solution = false;
                            continue;
                        }

                        if is_strictly_linear_transient {
                            // A purely linear deck does not need Newton fixed-point
                            // iterations: one direct solve per timestep is exact.
                            new_solution = sol;
                            converged = true;
                            break;
                        }

                        let voltage_converged = Self::check_voltage_convergence_with_tolerances(
                            &new_solution[..num_nodes],
                            &sol[..num_nodes],
                            self.voltage_abstol(),
                            self.voltage_reltol(),
                        );
                        let voltage_converged_relaxed = has_vbic_excess_phase
                            && Self::check_voltage_convergence_with_tolerances(
                                &new_solution[..num_nodes],
                                &sol[..num_nodes],
                                self.voltage_abstol() * 20.0,
                                self.voltage_reltol() * 20.0,
                            );
                        let linearized_residual_converged =
                            self.residual_convergence_met(&matrix, &sol, &rhs);
                        // CRITICAL: Update new_solution BEFORE checking device convergence
                        // Otherwise, BJT vbe/vbc are based on old guess, not new solve
                        new_solution = sol;
                        nonlinear_state_matches_new_solution = false;

                        // Update nonlinear device state to new solution for accurate convergence check
                        if circuit.has_nonlinear_devices() && !nonlinear_state_matches_new_solution
                        {
                            circuit.update_nonlinear(&new_solution);
                            nonlinear_state_matches_new_solution = true;
                        }

                        let static_device_converged = !circuit.has_nonlinear_devices()
                            || self.transient_static_device_convergence_met(
                                &circuit,
                                has_vbic_excess_phase,
                            );
                        let hidden_device_converged =
                            if has_vbic_excess_phase && static_device_converged {
                                // The reduced global Newton solve can satisfy external
                                // voltage/residual checks while the hidden VBIC excess-phase
                                // state still misses ngspice's device-local predictor
                                // tolerances. Treat that hidden-state metric as part of
                                // transient device convergence so delayed-transport startup
                                // candidates are retried instead of being accepted stale.
                                self.vbic_excess_phase_device_convergence_met(
                                    &circuit,
                                    &solution,
                                    &new_solution,
                                    current_method,
                                    step_trap_order,
                                    dt,
                                    &bjt_history,
                                    &vbic_snapshot_cache,
                                )
                            } else {
                                true
                            };
                        let device_converged = static_device_converged && hidden_device_converged;
                        let nonlinear_residual_converged = if has_vbic_excess_phase
                            && !linearized_residual_converged
                            && device_converged
                            && voltage_converged_relaxed
                        {
                            self.transient_nonlinear_residual_converged(
                                &mut circuit,
                                &mut matrix,
                                &mut rhs,
                                &new_solution,
                                t + dt,
                                dt,
                                &coeff,
                                current_method,
                                step_trap_order,
                                &bjt_history,
                                &mut vbic_snapshot_cache,
                                &jfet_history,
                                &mosfet_history,
                                suppress_gate_charge,
                                &tline_dc_refs,
                                &coupled_tline_refs,
                            )
                        } else {
                            false
                        };
                        let residual_converged =
                            linearized_residual_converged || nonlinear_residual_converged;

                        let strict_converged =
                            voltage_converged && device_converged && residual_converged;
                        let vbic_relaxed_converged = Self::vbic_relaxed_convergence_met(
                            has_vbic_excess_phase,
                            voltage_converged_relaxed,
                            device_converged,
                            residual_converged,
                        );

                        if strict_converged || vbic_relaxed_converged {
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
                nonlinear_state_matches_solution = false;
                retry_count += 1;
                trap_order = 1;

                // Diagnostic logging for debugging convergence issues
                static CONV_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                let count = CONV_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if count < 10 || (t > 9.5e-8 && dt < 1.0e-15) {
                    // Check what specifically didn't converge
                    let v_conv = self.voltage_convergence_met(&solution, &new_solution);
                    let d_conv_static = !circuit.has_nonlinear_devices()
                        || self.transient_static_device_convergence_met(
                            &circuit,
                            has_vbic_excess_phase,
                        );
                    let d_conv_hidden = if has_vbic_excess_phase {
                        self.vbic_excess_phase_device_convergence_met(
                            &circuit,
                            &solution,
                            &new_solution,
                            current_method,
                            step_trap_order,
                            dt,
                            &bjt_history,
                            &vbic_snapshot_cache,
                        )
                    } else {
                        true
                    };
                    let d_conv = d_conv_static && d_conv_hidden;
                    let r_conv = self.residual_convergence_met(&matrix, &new_solution, &rhs);
                    let max_dv = Self::max_abs_delta_prefix(&solution, &new_solution, num_nodes);
                    log::warn!(
                        "Newton non-converge at t={:.6e}, dt={:.3e}: voltage_conv={}, device_conv={}, static_device_conv={}, vbic_hidden_bypass_metric={}, residual_conv={}, max_dv={:.3e}, iter={}",
                        t,
                        dt,
                        v_conv,
                        d_conv,
                        d_conv_static,
                        d_conv_hidden,
                        r_conv,
                        max_dv,
                        total_iterations
                    );
                    if has_vbic_dynamic_charges {
                        let nonlinear_r_conv = self.transient_nonlinear_residual_converged(
                            &mut circuit,
                            &mut matrix,
                            &mut rhs,
                            &new_solution,
                            t + dt,
                            dt,
                            &coeff,
                            current_method,
                            step_trap_order,
                            &bjt_history,
                            &mut vbic_snapshot_cache,
                            &jfet_history,
                            &mosfet_history,
                            suppress_gate_charge,
                            &tline_dc_refs,
                            &coupled_tline_refs,
                        );
                        let nonlinear_norm = matrix
                            .scaled_residual_inf_norm(
                                &new_solution,
                                &rhs,
                                self.current_abstol(),
                                self.residual_reltol(),
                            )
                            .unwrap_or(Value::INFINITY);
                        if let Ok(residuals) = matrix.residual_vector(&new_solution, &rhs) {
                            let mut max_row = 0usize;
                            let mut max_norm = 0.0;
                            let mut max_residual = 0.0;
                            let mut max_ax = 0.0;
                            let mut max_rhs = 0.0;
                            for row in 0..residuals.len() {
                                let residual = residuals[row];
                                let row_rhs = rhs[row];
                                let row_ax = residual + row_rhs;
                                let scale = self.current_abstol()
                                    + self.residual_reltol() * row_ax.abs().max(row_rhs.abs());
                                let normalized = residual.abs() / scale.max(self.current_abstol());
                                if normalized > max_norm {
                                    max_row = row;
                                    max_norm = normalized;
                                    max_residual = residual;
                                    max_ax = row_ax;
                                    max_rhs = row_rhs;
                                }
                            }
                            log::warn!(
                                "Nonlinear restamp residual at t={:.6e}, dt={:.3e}: conv={}, norm={:.3e}, row={}, raw={:.3e}, ax={:.3e}, rhs={:.3e}",
                                t,
                                dt,
                                nonlinear_r_conv,
                                nonlinear_norm,
                                max_row,
                                max_residual,
                                max_ax,
                                max_rhs
                            );
                        } else {
                            log::warn!(
                                "Nonlinear restamp residual at t={:.6e}, dt={:.3e}: conv={}, norm={:.3e}, residual-vector=unavailable",
                                t,
                                dt,
                                nonlinear_r_conv,
                                nonlinear_norm
                            );
                        }
                    }
                }

                // Diagnostic logging for debugging timestep issues
                if total_iterations < 100 || total_iterations % 10000 == 0 {
                    log::debug!(
                        "Newton non-convergence at t={:.3e}s, iter={}, dt={:.3e}s, reducing to {:.3e}s",
                        t,
                        total_iterations,
                        dt,
                        Self::nonconvergence_retry_timestep(dt, max_step)
                    );
                }

                // Match ngspice's non-convergence recovery: retry at one eighth
                // of the rejected timestep, unless a force-accept cooldown is
                // temporarily holding dt steady to avoid ping-pong.
                if force_accept_cooldown > 0 {
                    force_accept_cooldown -= 1;
                    // During cooldown, keep timestep at current level (don't shrink)
                } else {
                    let retry_dt = Self::apply_retry_timestep_floor(
                        Self::nonconvergence_retry_timestep(dt, max_step),
                        legacy_bjt_retry_floor_dt,
                        max_step,
                    );
                    timestep.force_step(retry_dt);
                }

                // Force accept when recovery is unlikely:
                // - After MAX_RETRIES attempts (regardless of timestep state), OR
                // - At minimum timestep AND at least MIN_RETRIES_AT_MIN have been tried
                // This prevents both infinite loops and force-accept floods
                let at_min_dt =
                    Self::is_at_effective_retry_minimum(&timestep, legacy_bjt_retry_floor_dt);
                let exhausted_retries = retry_count >= MAX_RETRIES;
                let exhausted_at_min = at_min_dt
                    && retry_count
                        >= Self::min_retries_at_minimum_timestep(
                            has_vbic_excess_phase,
                            t + dt,
                            hinted_max_step,
                        );

                if exhausted_retries || exhausted_at_min {
                    let bounded_force_candidate = Self::bounded_force_accept_candidate(
                        &circuit,
                        &solution,
                        &new_solution,
                        t + dt,
                        num_nodes,
                        force_accept_delta_limit,
                        &force_accept_protected_nodes,
                        &ideal_output_pairs,
                    );
                    let unbounded_force_candidate = Self::is_unbounded_step(
                        &solution,
                        &bounded_force_candidate,
                        expected_source_delta,
                        num_nodes,
                    );
                    let excessive_quiet_force_candidate = Self::is_excessive_quiet_force_candidate(
                        &solution,
                        &bounded_force_candidate,
                        expected_source_delta,
                        num_nodes,
                        force_accept_delta_limit,
                    );
                    let stale_force_candidate = Self::is_stale_step(
                        &solution,
                        &bounded_force_candidate,
                        expected_source_delta,
                        num_nodes,
                    );
                    let vbic_hidden_bypass_metric = self
                        .force_candidate_vbic_hidden_bypass_metric_met(
                            &circuit,
                            has_vbic_excess_phase,
                            &solution,
                            &bounded_force_candidate,
                            current_method,
                            step_trap_order,
                            dt,
                            &bjt_history,
                            &vbic_snapshot_cache,
                        );
                    let stagnant_force_candidate = Self::is_stagnant_force_candidate(
                        &circuit,
                        &solution,
                        &bounded_force_candidate,
                        num_nodes,
                        self.voltage_abstol(),
                        self.current_abstol(),
                    ) && !vbic_hidden_bypass_metric;

                    if enforce_force_candidate_safety
                        && (unbounded_force_candidate
                            || excessive_quiet_force_candidate
                            || !had_solver_candidate
                            || stale_force_candidate
                            || stagnant_force_candidate)
                    {
                        stale_accept_count += 1;
                        let boosted = if stagnant_force_candidate || excessive_quiet_force_candidate
                        {
                            Self::force_accept_recovery_timestep(
                                dt,
                                timestep.preferred_min_dt(),
                                max_step,
                                None,
                            )
                        } else {
                            (dt * 4.0).min(max_step)
                        };
                        if boosted > dt {
                            timestep.force_step(boosted);
                        }
                        if stale_accept_count >= 8 {
                            if unbounded_force_candidate {
                                log::error!(
                                    "Transient diverged at t={:.6e}s: repeated unbounded force-accept candidates",
                                    t
                                );
                            } else if excessive_quiet_force_candidate {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: quiet-source force-accept candidates exceeded the bounded step envelope",
                                    t
                                );
                            } else if stale_force_candidate {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: stale force-accept candidates with active sources",
                                    t
                                );
                            } else if stagnant_force_candidate {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: stagnant force-accept candidates made no progress",
                                    t
                                );
                            }
                            return Err(SimulationError::ConvergenceFailed(total_iterations));
                        }
                        nonlinear_state_matches_solution = false;
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
                    let force_candidate_node_delta =
                        Self::max_abs_delta_prefix(&solution, &bounded_force_candidate, num_nodes);
                    let force_candidate_full_delta =
                        Self::max_abs_delta(&solution, &bounded_force_candidate);
                    let top_force_nodes = Self::top_abs_delta_prefix_named(
                        &solution,
                        &bounded_force_candidate,
                        &result.node_names,
                        num_nodes,
                        4,
                    );
                    stale_accept_count = 0;

                    t += dt;
                    let hit_breakpoint = at_breakpoint || breakpoints.at_breakpoint(t);
                    if hit_breakpoint {
                        let restart_dt = breakpoints.mark_breakpoint_solved(t);
                        timestep.force_step(restart_dt.min(max_step));
                    }

                    // FORCE-ACCEPT: Use the bounded Newton candidate as-is.
                    // Project ideal-source constraints first, then clip source-free
                    // node movement so the committed state stays physically bounded.
                    new_solution = bounded_force_candidate;
                    nonlinear_state_matches_new_solution = false;

                    if circuit.has_nonlinear_devices() {
                        circuit.update_nonlinear(&new_solution);
                        nonlinear_state_matches_new_solution = true;
                    }

                    let method_after_step = current_integration_method(&trapgear);
                    let accepted_step_trap_order =
                        Self::effective_trapezoidal_order(current_method, 1, false);
                    let force_accept_bjt_truncation_limit = if has_bjts {
                        Self::bjt_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &bjt_history,
                            &vbic_snapshot_cache,
                            self.voltage_abstol(),
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            NGSPICE_DEFAULT_TRTOL,
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_jfet_truncation_limit =
                        if !suppress_gate_charge && !circuit.jfets.is_empty() {
                            Self::jfet_ngspice_truncation_limit(
                                &circuit,
                                &new_solution,
                                current_method,
                                accepted_step_trap_order,
                                dt,
                                &jfet_history,
                                self.voltage_reltol(),
                                self.current_abstol(),
                                self.charge_abstol(),
                                NGSPICE_DEFAULT_TRTOL,
                            )
                            .filter(|limit| limit.is_finite() && *limit > 0.0)
                        } else {
                            None
                        };
                    let force_accept_capacitor_truncation_limit = if !circuit.capacitors.is_empty()
                    {
                        Self::capacitor_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            mosfet_history.accepted_dt_prev,
                            mosfet_history.accepted_dt_prev_prev,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            NGSPICE_DEFAULT_TRTOL,
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_mosfet_truncation_limit =
                        if !suppress_gate_charge && !circuit.mosfets.is_empty() {
                            Self::mosfet_ngspice_truncation_limit(
                                &circuit,
                                &new_solution,
                                current_method,
                                accepted_step_trap_order,
                                dt,
                                &mosfet_history,
                                self.voltage_reltol(),
                                self.current_abstol(),
                                self.charge_abstol(),
                                NGSPICE_DEFAULT_TRTOL,
                            )
                            .filter(|limit| limit.is_finite() && *limit > 0.0)
                        } else {
                            None
                        };
                    let force_accept_device_truncation_limit = Self::min_truncation_limit(
                        Self::min_truncation_limit(
                            Self::min_truncation_limit(
                                force_accept_capacitor_truncation_limit,
                                force_accept_bjt_truncation_limit,
                            ),
                            force_accept_jfet_truncation_limit,
                        ),
                        force_accept_mosfet_truncation_limit,
                    );
                    lte_estimator.record(&new_solution, dt);
                    lte_estimator.set_method_order(effective_method_order(
                        method_after_step,
                        accepted_step_trap_order,
                    ));
                    Self::record_vbic_truncation_charge_state(
                        &mut vbic_charge_lte_estimator,
                        &circuit,
                        &new_solution,
                        current_method,
                        accepted_step_trap_order,
                        dt,
                        &bjt_history,
                        Some(&vbic_snapshot_cache),
                        effective_method_order(method_after_step, accepted_step_trap_order),
                    );
                    if fixed_method.is_none() {
                        trapgear.update(&new_solution, dt);
                    }
                    Self::update_reactive_history(
                        &mut circuit,
                        &new_solution,
                        t,
                        dt,
                        current_method,
                        accepted_step_trap_order,
                        &mut bjt_history,
                        &mut jfet_history,
                        &mut mosfet_history,
                        None,
                        suppress_gate_charge,
                        &tline_dc_refs,
                        &coupled_tline_refs,
                        &mut breakpoints,
                        tstop,
                        self.voltage_reltol(),
                        self.voltage_abstol(),
                        &mut dynamic_tline_breakpoints_added,
                        &mut warned_dynamic_tline_breakpoint_cap,
                    );
                    inittran_gate_charge_phase = false;
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_timestep();
                    }

                    solution.clone_from(&new_solution);
                    nonlinear_state_matches_solution = nonlinear_state_matches_new_solution;
                    result.time.push(t);
                    for (i, voltages) in result.voltages.iter_mut().enumerate() {
                        voltages.push(solution.get(i).copied().unwrap_or(0.0));
                    }
                    for (i, currents) in result.branch_currents.iter_mut().enumerate() {
                        currents.push(solution.get(num_nodes + i).copied().unwrap_or(0.0));
                    }

                    let next_force_dt = Self::force_accept_recovery_timestep(
                        dt,
                        timestep.preferred_min_dt(),
                        max_step,
                        force_accept_device_truncation_limit,
                    );
                    let v0_force = solution.first().copied().unwrap_or(0.0);
                    static FORCE_LOG_COUNT: std::sync::atomic::AtomicUsize =
                        std::sync::atomic::AtomicUsize::new(0);
                    let count = FORCE_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if count < 32 || (t > 9.5e-8 && dt < 1.0e-15) {
                        log::warn!(
                            "FORCE-ACCEPT at t={:.12e}s accepted_t={:.12e}s dt={:.3e} next_dt={:.3e} node_dv={:.3e} full_dv={:.3e} top_nodes={:?} v0={:.4} trunc_limit={:?} retry_count={}",
                            t,
                            t + dt,
                            dt,
                            next_force_dt,
                            force_candidate_node_delta,
                            force_candidate_full_delta,
                            top_force_nodes,
                            v0_force,
                            force_accept_device_truncation_limit,
                            retry_count
                        );
                    }
                    retry_count = 0; // Reset for next timepoint
                    // Keep the accepted dt and only defer shrink for a couple of retries.
                    // Large cooldowns plus immediate dt growth can trap stiff switching decks
                    // in repeated force-accept loops instead of letting the controller retreat.
                    force_accept_cooldown = if has_vbic_excess_phase {
                        0
                    } else {
                        FORCE_ACCEPT_COOLDOWN_RETRIES
                    };
                    timestep.force_step(next_force_dt);
                    if matches!(
                        current_method,
                        IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                    ) {
                        trap_order = 1;
                    }
                }
                continue;
            }

            let first_accepted_transient_step =
                Self::should_skip_post_accept_timestep_control_on_first_step(result.time.len());
            let vbic_truncation_limit = if !first_accepted_transient_step && has_vbic_excess_phase {
                Self::vbic_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &bjt_history,
                    &vbic_snapshot_cache,
                    self.voltage_abstol(),
                    self.voltage_reltol(),
                    self.current_abstol(),
                    self.charge_abstol(),
                    NGSPICE_DEFAULT_TRTOL,
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let legacy_bjt_truncation_limit = if !first_accepted_transient_step && has_bjts {
                Self::legacy_bjt_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &bjt_history,
                    self.voltage_reltol(),
                    self.current_abstol(),
                    self.charge_abstol(),
                    NGSPICE_DEFAULT_TRTOL,
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let bjt_truncation_limit =
                Self::min_truncation_limit(vbic_truncation_limit, legacy_bjt_truncation_limit);
            let capacitor_truncation_limit =
                if !first_accepted_transient_step && !circuit.capacitors.is_empty() {
                    Self::capacitor_ngspice_truncation_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        mosfet_history.accepted_dt_prev,
                        mosfet_history.accepted_dt_prev_prev,
                        self.voltage_reltol(),
                        self.current_abstol(),
                        self.charge_abstol(),
                        NGSPICE_DEFAULT_TRTOL,
                    )
                    .filter(|limit| limit.is_finite() && *limit > 0.0)
                } else {
                    None
                };
            let jfet_truncation_limit = if !first_accepted_transient_step
                && !suppress_gate_charge
                && !circuit.jfets.is_empty()
            {
                Self::jfet_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &jfet_history,
                    self.voltage_reltol(),
                    self.current_abstol(),
                    self.charge_abstol(),
                    NGSPICE_DEFAULT_TRTOL,
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let mosfet_truncation_limit = if !first_accepted_transient_step
                && !suppress_gate_charge
                && !circuit.mosfets.is_empty()
            {
                Self::mosfet_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &mosfet_history,
                    self.voltage_reltol(),
                    self.current_abstol(),
                    self.charge_abstol(),
                    NGSPICE_DEFAULT_TRTOL,
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let device_truncation_limit = Self::min_truncation_limit(
                Self::min_truncation_limit(
                    Self::min_truncation_limit(capacitor_truncation_limit, bjt_truncation_limit),
                    jfet_truncation_limit,
                ),
                mosfet_truncation_limit,
            );

            if let Some(limit) = device_truncation_limit
                && Self::should_retry_ngspice_charge_truncation(limit, dt)
            {
                static DEVICE_TRUNC_REJECT_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                let log_count = DEVICE_TRUNC_REJECT_LOG_COUNT
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if log_count < 40 || (t > 9.5e-8 && dt < 1.0e-15) {
                    log::warn!(
                        "Device charge truncation reject at t={:.6e}, dt={:.3e}, limit={:.3e}, method={:?}, order={}",
                        t,
                        dt,
                        limit,
                        current_method,
                        step_trap_order
                    );
                }
                nonlinear_state_matches_solution = false;
                retry_count += 1;
                // Match ngspice truncation retries: keep the current integration
                // order and only reduce the timestep.
                trap_order = Self::trapezoidal_order_after_timestep_control_reject(step_trap_order);
                timestep.force_step(limit);
                continue;
            }

            // Check LTE for physics accuracy
            let active_vbic_charge_lte_estimator = if Self::should_use_vbic_charge_lte_estimator(
                has_vbic_excess_phase,
                t + dt,
                hinted_max_step,
                smallest_vbic_excess_phase_td,
                dt,
                timestep.preferred_min_dt(),
            ) {
                vbic_charge_lte_estimator.as_ref()
            } else {
                None
            };
            let using_vbic_charge_lte_estimator = active_vbic_charge_lte_estimator.is_some();
            let defer_voltage_lte_to_vbic_truncation =
                Self::should_defer_voltage_lte_to_vbic_truncation(
                    has_vbic_excess_phase,
                    t + dt,
                    hinted_max_step,
                    smallest_vbic_excess_phase_td,
                    vbic_truncation_limit,
                    using_vbic_charge_lte_estimator,
                );
            let defer_voltage_lte_to_bjt_truncation = !has_vbic_excess_phase
                && Self::bjt_charge_truncation_covers_transient_lte(&circuit, bjt_truncation_limit);
            let defer_voltage_lte_to_jfet_truncation = !has_vbic_excess_phase
                && !has_bjts
                && Self::jfet_charge_truncation_covers_transient_lte(
                    &circuit,
                    jfet_truncation_limit,
                );
            let defer_voltage_lte_to_mosfet_truncation = !has_vbic_excess_phase
                && !has_bjts
                && circuit.jfets.is_empty()
                && Self::mosfet_charge_truncation_covers_transient_lte(
                    &circuit,
                    mosfet_truncation_limit,
                );
            let defer_voltage_lte_to_ngspice_device_truncation = !has_vbic_excess_phase
                && Self::ngspice_device_truncation_covers_transient_lte(
                    &circuit,
                    capacitor_truncation_limit,
                    bjt_truncation_limit,
                    jfet_truncation_limit,
                    mosfet_truncation_limit,
                );
            let (lte, accept, uses_vbic_charge_lte) = if first_accepted_transient_step {
                (0.0, true, false)
            } else if defer_voltage_lte_to_vbic_truncation {
                // ngspice's excess-phase startup control is charge/truncation-driven.
                // When that authoritative truncation limit is available but our
                // supplemental reduced charge-LTE estimator is intentionally idle
                // (typically because dt collapsed near delmin), do not let the
                // generic voltage LTE controller create a false reject loop.
                (0.0, true, false)
            } else if defer_voltage_lte_to_bjt_truncation {
                // Classic SPICE drives legacy BJT timesteps from device charge
                // truncation (BJTtrunc -> CKTterr). For BJT-only reactive
                // decks, the generic node-voltage predictor is a supplemental
                // guard, not the authoritative LTE controller.
                (0.0, true, false)
            } else if defer_voltage_lte_to_jfet_truncation {
                // ngspice drives JFET/MESFET/HFET dynamic gate charge control
                // through device truncation (JFETtrunc/HFETtrunc -> CKTterr).
                // For JFET-only reactive decks, keep that charge controller
                // authoritative instead of letting generic node-voltage LTE
                // collapse the timestep around sharp nonlinear gate edges.
                (0.0, true, false)
            } else if defer_voltage_lte_to_mosfet_truncation {
                // MOS transient gate-charge control is the device-local
                // MOStrunc/MOS6trunc CKTterr path in ngspice. In MOS-only
                // reactive decks it is the authoritative timestep controller.
                (0.0, true, false)
            } else if defer_voltage_lte_to_ngspice_device_truncation {
                // Classic ngspice uses device-local CKTterr truncation drivers
                // (CAPtrunc, MOStrunc, BJTtrunc, etc.) rather than an additional
                // global node-voltage predictor. Transmission-line decks rely on
                // their model max-step/breakpoint controls plus those connected
                // dynamic devices.
                (0.0, true, false)
            } else {
                Self::estimate_transient_lte(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    is_strictly_linear_transient,
                    &bjt_history,
                    &lte_estimator,
                    active_vbic_charge_lte_estimator,
                    Some(&vbic_snapshot_cache),
                    self.voltage_abstol(),
                    self.voltage_reltol(),
                )
            };
            let lte_scale = if first_accepted_transient_step || is_strictly_linear_transient {
                1.0
            } else {
                Self::recommend_transient_lte_scale(
                    &lte_estimator,
                    active_vbic_charge_lte_estimator,
                    lte,
                    uses_vbic_charge_lte,
                )
            };
            if !accept {
                if uses_vbic_charge_lte {
                    static VBIC_CHARGE_LTE_REJECT_LOG_COUNT: std::sync::atomic::AtomicUsize =
                        std::sync::atomic::AtomicUsize::new(0);
                    let log_count = VBIC_CHARGE_LTE_REJECT_LOG_COUNT
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_count < 40 || (t > 9.5e-8 && dt < 1.0e-15) {
                        log::warn!(
                            "VBIC charge-LTE reject at t={:.6e}, dt={:.3e}, lte={:.3e}, scale={:.3e}, order={}, pref_min={:.3e}",
                            t,
                            dt,
                            lte,
                            lte_scale,
                            step_trap_order,
                            timestep.preferred_min_dt(),
                        );
                    }
                }
                nonlinear_state_matches_solution = false;
                retry_count += 1;
                // LTE/truncation rejects in ngspice retry the same order at a
                // smaller timestep instead of forcing trapezoidal back to order 1.
                trap_order = Self::trapezoidal_order_after_timestep_control_reject(step_trap_order);
                timestep.adjust(lte / lte_scale);
                let clamped_retry_dt = Self::apply_retry_timestep_floor(
                    timestep.dt(),
                    legacy_bjt_retry_floor_dt,
                    max_step,
                );
                if clamped_retry_dt > timestep.dt() + 1e-30 {
                    timestep.force_step(clamped_retry_dt);
                }

                // Force accept when recovery is unlikely:
                // - After MAX_RETRIES attempts (regardless of timestep state), OR
                // - At minimum timestep AND at least MIN_RETRIES_AT_MIN have been tried
                // This prevents both infinite loops and force-accept floods
                let at_min_dt =
                    Self::is_at_effective_retry_minimum(&timestep, legacy_bjt_retry_floor_dt);
                let exhausted_retries = retry_count >= MAX_RETRIES;
                let exhausted_at_min = at_min_dt
                    && retry_count
                        >= Self::min_retries_at_minimum_timestep(
                            has_vbic_excess_phase,
                            t + dt,
                            hinted_max_step,
                        );

                if exhausted_retries || exhausted_at_min {
                    let bounded_force_candidate = Self::bounded_force_accept_candidate(
                        &circuit,
                        &solution,
                        &new_solution,
                        t + dt,
                        num_nodes,
                        force_accept_delta_limit,
                        &force_accept_protected_nodes,
                        &ideal_output_pairs,
                    );
                    let unbounded_force_candidate = Self::is_unbounded_step(
                        &solution,
                        &bounded_force_candidate,
                        expected_source_delta,
                        num_nodes,
                    );
                    let excessive_quiet_force_candidate = Self::is_excessive_quiet_force_candidate(
                        &solution,
                        &bounded_force_candidate,
                        expected_source_delta,
                        num_nodes,
                        force_accept_delta_limit,
                    );
                    let stale_force_candidate = Self::is_stale_step(
                        &solution,
                        &bounded_force_candidate,
                        expected_source_delta,
                        num_nodes,
                    );
                    let vbic_hidden_bypass_metric = self
                        .force_candidate_vbic_hidden_bypass_metric_met(
                            &circuit,
                            has_vbic_excess_phase,
                            &solution,
                            &bounded_force_candidate,
                            current_method,
                            step_trap_order,
                            dt,
                            &bjt_history,
                            &vbic_snapshot_cache,
                        );
                    let stagnant_force_candidate = Self::is_stagnant_force_candidate(
                        &circuit,
                        &solution,
                        &bounded_force_candidate,
                        num_nodes,
                        self.voltage_abstol(),
                        self.current_abstol(),
                    ) && !vbic_hidden_bypass_metric;

                    if enforce_force_candidate_safety
                        && (unbounded_force_candidate
                            || excessive_quiet_force_candidate
                            || !had_solver_candidate
                            || stale_force_candidate
                            || stagnant_force_candidate)
                    {
                        stale_accept_count += 1;
                        let boosted = if stagnant_force_candidate || excessive_quiet_force_candidate
                        {
                            Self::force_accept_recovery_timestep(
                                dt,
                                timestep.preferred_min_dt(),
                                max_step,
                                None,
                            )
                        } else {
                            (dt * 4.0).min(max_step)
                        };
                        if boosted > dt {
                            timestep.force_step(boosted);
                        }
                        if stale_accept_count >= 8 {
                            if unbounded_force_candidate {
                                log::error!(
                                    "Transient diverged at t={:.6e}s: repeated unbounded LTE force-accept candidates",
                                    t
                                );
                            } else if excessive_quiet_force_candidate {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: quiet-source LTE force-accept candidates exceeded the bounded step envelope",
                                    t
                                );
                            } else if stale_force_candidate {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: stale LTE force-accept candidates with active sources",
                                    t
                                );
                            } else if stagnant_force_candidate {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: stagnant LTE force-accept candidates made no progress",
                                    t
                                );
                            }
                            return Err(SimulationError::ConvergenceFailed(total_iterations));
                        }
                        nonlinear_state_matches_solution = false;
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
                    new_solution = bounded_force_candidate;
                    nonlinear_state_matches_new_solution = false;

                    if circuit.has_nonlinear_devices() {
                        circuit.update_nonlinear(&new_solution);
                        nonlinear_state_matches_new_solution = true;
                    }

                    let method_after_step = current_integration_method(&trapgear);
                    let accepted_step_trap_order =
                        Self::effective_trapezoidal_order(current_method, 1, false);
                    let force_accept_bjt_truncation_limit = if has_bjts {
                        Self::bjt_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &bjt_history,
                            &vbic_snapshot_cache,
                            self.voltage_abstol(),
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            NGSPICE_DEFAULT_TRTOL,
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_jfet_truncation_limit =
                        if !suppress_gate_charge && !circuit.jfets.is_empty() {
                            Self::jfet_ngspice_truncation_limit(
                                &circuit,
                                &new_solution,
                                current_method,
                                accepted_step_trap_order,
                                dt,
                                &jfet_history,
                                self.voltage_reltol(),
                                self.current_abstol(),
                                self.charge_abstol(),
                                NGSPICE_DEFAULT_TRTOL,
                            )
                            .filter(|limit| limit.is_finite() && *limit > 0.0)
                        } else {
                            None
                        };
                    let force_accept_mosfet_truncation_limit =
                        if !suppress_gate_charge && !circuit.mosfets.is_empty() {
                            Self::mosfet_ngspice_truncation_limit(
                                &circuit,
                                &new_solution,
                                current_method,
                                accepted_step_trap_order,
                                dt,
                                &mosfet_history,
                                self.voltage_reltol(),
                                self.current_abstol(),
                                self.charge_abstol(),
                                NGSPICE_DEFAULT_TRTOL,
                            )
                            .filter(|limit| limit.is_finite() && *limit > 0.0)
                        } else {
                            None
                        };
                    let force_accept_device_truncation_limit = Self::min_truncation_limit(
                        Self::min_truncation_limit(
                            force_accept_bjt_truncation_limit,
                            force_accept_jfet_truncation_limit,
                        ),
                        force_accept_mosfet_truncation_limit,
                    );
                    lte_estimator.record(&new_solution, dt);
                    lte_estimator.set_method_order(effective_method_order(
                        method_after_step,
                        accepted_step_trap_order,
                    ));
                    Self::record_vbic_truncation_charge_state(
                        &mut vbic_charge_lte_estimator,
                        &circuit,
                        &new_solution,
                        current_method,
                        accepted_step_trap_order,
                        dt,
                        &bjt_history,
                        Some(&vbic_snapshot_cache),
                        effective_method_order(method_after_step, accepted_step_trap_order),
                    );
                    if fixed_method.is_none() {
                        trapgear.update(&new_solution, dt);
                    }
                    Self::update_reactive_history(
                        &mut circuit,
                        &new_solution,
                        t,
                        dt,
                        current_method,
                        accepted_step_trap_order,
                        &mut bjt_history,
                        &mut jfet_history,
                        &mut mosfet_history,
                        None,
                        suppress_gate_charge,
                        &tline_dc_refs,
                        &coupled_tline_refs,
                        &mut breakpoints,
                        tstop,
                        self.voltage_reltol(),
                        self.voltage_abstol(),
                        &mut dynamic_tline_breakpoints_added,
                        &mut warned_dynamic_tline_breakpoint_cap,
                    );
                    inittran_gate_charge_phase = false;
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_timestep();
                    }

                    solution.clone_from(&new_solution);
                    nonlinear_state_matches_solution = nonlinear_state_matches_new_solution;
                    result.time.push(t);
                    for (i, voltages) in result.voltages.iter_mut().enumerate() {
                        voltages.push(solution.get(i).copied().unwrap_or(0.0));
                    }
                    for (i, currents) in result.branch_currents.iter_mut().enumerate() {
                        currents.push(solution.get(num_nodes + i).copied().unwrap_or(0.0));
                    }
                    let next_force_dt = Self::force_accept_recovery_timestep(
                        dt,
                        timestep.preferred_min_dt(),
                        max_step,
                        force_accept_device_truncation_limit,
                    );
                    if t > 9.5e-8 && dt < 1.0e-15 {
                        log::warn!(
                            "LTE FORCE-ACCEPT at t={:.12e}s accepted_t={:.12e}s dt={:.3e} next_dt={:.3e} trunc_limit={:?} lte={:.3e} retry_count={}",
                            t,
                            t + dt,
                            dt,
                            next_force_dt,
                            force_accept_device_truncation_limit,
                            lte,
                            retry_count
                        );
                    }
                    retry_count = 0; // Reset for next timepoint
                    force_accept_cooldown = if has_vbic_excess_phase {
                        0
                    } else {
                        FORCE_ACCEPT_COOLDOWN_RETRIES
                    };
                    timestep.force_step(next_force_dt);
                    if matches!(
                        current_method,
                        IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                    ) {
                        trap_order = 1;
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
            nonlinear_state_matches_new_solution = false;

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
                nonlinear_state_matches_solution = false;
                continue;
            }
            stale_accept_count = 0;

            // Accept this timestep
            t += dt;
            let hit_breakpoint = at_breakpoint || breakpoints.at_breakpoint(t);
            let method_after_step = current_integration_method(&trapgear);
            lte_estimator.record(&new_solution, dt);
            lte_estimator
                .set_method_order(effective_method_order(method_after_step, step_trap_order));
            Self::record_vbic_truncation_charge_state(
                &mut vbic_charge_lte_estimator,
                &circuit,
                &new_solution,
                current_method,
                step_trap_order,
                dt,
                &bjt_history,
                Some(&vbic_snapshot_cache),
                effective_method_order(method_after_step, step_trap_order),
            );
            if fixed_method.is_none() {
                trapgear.update(&new_solution, dt);
            }

            if circuit.has_nonlinear_devices() && !nonlinear_state_matches_new_solution {
                circuit.update_nonlinear(&new_solution);
                nonlinear_state_matches_new_solution = true;
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
                Some(&vbic_snapshot_cache),
                suppress_gate_charge,
                &tline_dc_refs,
                &coupled_tline_refs,
                &mut breakpoints,
                tstop,
                self.voltage_reltol(),
                self.voltage_abstol(),
                &mut dynamic_tline_breakpoints_added,
                &mut warned_dynamic_tline_breakpoint_cap,
            );
            inittran_gate_charge_phase = false;

            // Accept XSPICE timestep (commit state changes)
            if circuit.has_xspice_devices() {
                circuit.accept_xspice_timestep();
            }

            solution.clone_from(&new_solution);
            nonlinear_state_matches_solution = nonlinear_state_matches_new_solution;

            // Store results
            result.time.push(t);
            for (i, voltages) in result.voltages.iter_mut().enumerate() {
                voltages.push(solution.get(i).copied().unwrap_or(0.0));
            }
            for (i, currents) in result.branch_currents.iter_mut().enumerate() {
                currents.push(solution.get(num_nodes + i).copied().unwrap_or(0.0));
            }
            if first_accepted_transient_step {
                timestep.force_step(dt);
            } else {
                Self::recover_timestep_after_accepted_step(
                    &mut timestep,
                    &lte_estimator,
                    &new_solution,
                    dt,
                    max_step,
                    is_strictly_linear_transient,
                    expected_source_delta,
                    Some(lte_scale),
                );
            }
            if hit_breakpoint {
                let restart_dt = breakpoints.mark_breakpoint_solved(t);
                timestep.force_step(restart_dt.min(max_step));
            }
            if !first_accepted_transient_step
                && let Some(limit) = device_truncation_limit
                && limit.is_finite()
                && limit > 0.0
                && limit + 1e-18 < timestep.dt()
            {
                if t > 9.5e-8 && dt < 1.0e-15 {
                    log::warn!(
                        "Device post-accept timestep cap at t={:.12e}, accepted_dt={:.3e}, requested_next={:.3e}, limit={:.3e}, order={}",
                        t,
                        dt,
                        timestep.dt(),
                        limit,
                        step_trap_order
                    );
                }
                timestep.force_step(limit);
            }
            if first_accepted_transient_step
                && matches!(
                    current_method,
                    IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                )
            {
                trap_order = 1;
            } else if matches!(
                current_method,
                IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
            ) {
                let should_promote = if Self::should_hold_vbic_excess_phase_first_order(
                    has_vbic_excess_phase,
                    t,
                    hinted_max_step,
                    smallest_vbic_excess_phase_td,
                ) {
                    false
                } else {
                    Self::should_promote_trapezoidal_order(
                        &circuit,
                        &new_solution,
                        current_method,
                        dt,
                        is_strictly_linear_transient,
                        &bjt_history,
                        &jfet_history,
                        &mosfet_history,
                        &lte_estimator,
                        vbic_charge_lte_estimator.as_ref(),
                        &vbic_snapshot_cache,
                        self.voltage_abstol(),
                        self.voltage_reltol(),
                        self.current_abstol(),
                        self.charge_abstol(),
                        NGSPICE_DEFAULT_TRTOL,
                    )
                };
                trap_order = Self::next_trapezoidal_order_after_accepted_step(
                    step_trap_order,
                    hit_breakpoint,
                    should_promote,
                );
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
        if let Some(node0_voltages) = result.voltages.first() {
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
                node_names: result.node_names.clone(),
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

        let mut compressed = recorder.to_transient_result();
        compressed.node_names = result.node_names.clone();
        Ok(compressed)
    }
}

#[cfg(test)]
mod abort_tests {
    use super::*;
    use crate::Engine;
    use crate::abort_signal::{CountingAbort, ImmediateAbort, NoAbort};
    use crate::device::MatrixStamper;
    use crate::device::NonlinearDevice;
    use crate::device::semiconductor::VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX;

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
    fn test_jfet_charge_branch_voltages_hfet_level5_uses_raw_terminal_state() {
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

        // HFET1 charge history must follow ngspice vgspp/vgdpp: raw
        // gate-to-internal-terminal voltages, not DEVfetlim-clipped controls.
        assert!((vgs_charge - vgs_raw).abs() < 1e-15);
        assert!((vgd_charge - vgd_raw).abs() < 1e-15);
        assert!(
            (vgs_internal - vgs_raw).abs() > 1e-6 || (vgd_internal - vgd_raw).abs() > 1e-6,
            "test setup should exercise a limiter-clipped HFET control state"
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

    #[test]
    fn test_transient_operating_point_seeds_tline_far_end_from_dc_fallback() {
        let netlist = Netlist::parse(
            "Transmission line transient operating point\n\
             V1 in 0 DC 5\n\
             T1 in 0 out 0 Z0=50 TD=1n\n\
             .tran 1p 1p\n\
             .end",
        )
        .expect("parse transmission-line operating point netlist");

        let engine = Engine::default();
        let result = engine
            .run_tran(&netlist, 1e-12, 1e-12)
            .expect("transient operating point should solve");

        let source = result
            .try_voltage_at_named("in", 0)
            .expect("source node should be present");
        let far_end = result
            .try_voltage_at_named("out", 0)
            .expect("far-end node should be present");

        assert!((source - 5.0).abs() < 1e-9);
        assert!(
            (far_end - source).abs() < 1e-9,
            "transient t=0 solve must seed the line far end from the DC fallback; got source={source:.12e}, far_end={far_end:.12e}"
        );
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

    fn legacy_bjt_charge_history_fixture() -> (
        crate::circuit::Circuit,
        Vec<Value>,
        usize,
        [Value; 3],
        [Value; 3],
    ) {
        let netlist = crate::Netlist::parse(
            "Legacy BJT charge-history fixture\n\
             Q1 c b e qnd\n\
             .MODEL qnd NPN (LEVEL=1 BF=50 IS=1e-15 RB=70 RC=40 CCS=2p TF=0.1n TR=10n CJE=0.9p CJC=1.5p PC=0.85 VA=50)\n\
             .end",
        )
        .expect("parse legacy BJT charge-history fixture");
        let engine = Engine::default();
        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("build legacy BJT charge-history circuit");

        let idx = circuit
            .bjts
            .devices
            .iter()
            .position(|device| device.name == "Q1")
            .expect("find legacy BJT charge-history Q1");
        let bjt = circuit.bjts.devices[idx].clone();
        let mut solution = vec![0.0; circuit.matrix_size()];
        for (node, value) in [
            (bjt.node_collector, 2.5),
            (bjt.node_base, 0.85),
            (bjt.node_emitter, 0.1),
            (bjt.node_substrate, 0.0),
        ] {
            if node > 0 {
                solution[node - 1] = value;
            }
        }
        circuit.update_nonlinear(&solution);

        let vc = Engine::node_voltage(&solution, bjt.node_collector);
        let vb = Engine::node_voltage(&solution, bjt.node_base);
        let ve = Engine::node_voltage(&solution, bjt.node_emitter);
        let vs = Engine::node_voltage(&solution, bjt.node_substrate);
        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let (legacy_vbe, legacy_vbc, legacy_vcs) =
            Engine::legacy_bjt_charge_branch_voltages(&snapshot);

        (
            circuit,
            solution,
            idx,
            [vb - ve, vb - vc, vc - vs],
            [legacy_vbe, legacy_vbc, legacy_vcs],
        )
    }

    fn jfet_charge_history_fixture() -> (crate::circuit::Circuit, Vec<Value>, usize) {
        let mut circuit = crate::circuit::Circuit::new();
        let drain = circuit.get_or_create_node("d");
        let gate = circuit.get_or_create_node("g");
        let source = circuit.get_or_create_node("s");
        let params = crate::device::JfetParams::new()
            .with_capacitances(2.0e-12, 1.25e-12)
            .with_vto(-1.5)
            .with_beta(1.0e-3);
        circuit
            .jfets
            .push(crate::device::Jfet::njf("J1", drain, gate, source).with_params(params));

        let idx = 0;
        let jfet = circuit.jfets[idx].clone();
        let mut solution = vec![0.0; circuit.matrix_size()];
        for (node, value) in [(jfet.drain, 2.0), (jfet.gate, 0.35), (jfet.source, -0.1)] {
            if node > 0 {
                solution[node - 1] = value;
            }
        }
        circuit.update_nonlinear(&solution);

        (circuit, solution, idx)
    }

    fn mosfet_charge_history_fixture() -> (crate::circuit::Circuit, Vec<Value>, usize) {
        let mut circuit = crate::circuit::Circuit::new();
        let drain = circuit.get_or_create_node("d");
        let gate = circuit.get_or_create_node("g");
        let source = circuit.get_or_create_node("s");
        let bulk = circuit.get_or_create_node("b");
        let params = std::collections::HashMap::from([
            ("LEVEL".to_string(), 6.0),
            ("VT0".to_string(), 0.69486),
            ("KC".to_string(), 3.8921e-5),
            ("NC".to_string(), 1.1739),
            ("KV".to_string(), 0.91602),
            ("NV".to_string(), 0.87225),
            ("GAMMA".to_string(), 0.60309),
            ("PHI".to_string(), 1.0),
            ("TOX".to_string(), 1.98e-8),
            ("CGSO".to_string(), 3.93e-10),
            ("CGDO".to_string(), 3.93e-10),
            ("CGBO".to_string(), 1.0e-10),
            ("LD".to_string(), 0.1e-6),
            ("LAMBDA0".to_string(), 0.013333),
            ("LAMBDA1".to_string(), 0.0),
        ]);
        let mosfet = crate::device::Mosfet::new_nmos("M1".to_string(), drain, gate, source, bulk)
            .with_params(&params)
            .with_geometry(5.0e-6, 1.0e-6);
        circuit.mosfets.add(mosfet);

        let idx = 0;
        let mosfet = circuit.mosfets.devices[idx].clone();
        let mut solution = vec![0.0; circuit.matrix_size()];
        for (node, value) in [
            (mosfet.node_drain, 1.8),
            (mosfet.node_gate, 1.0),
            (mosfet.node_source, 0.0),
            (mosfet.node_bulk, 0.0),
        ] {
            if node > 0 {
                solution[node - 1] = value;
            }
        }
        circuit.update_nonlinear(&solution);

        (circuit, solution, idx)
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
    fn test_initialize_bjt_history_uses_internal_legacy_charge_branch_voltages() {
        let (circuit, solution, idx, external, internal) = legacy_bjt_charge_history_fixture();

        assert!(
            (internal[0] - external[0]).abs() > 1e-9
                || (internal[1] - external[1]).abs() > 1e-9
                || (internal[2] - external[2]).abs() > 1e-9,
            "expected the legacy BJT fixture to exercise internal charge branch voltages"
        );

        let history = Engine::initialize_bjt_history(&circuit, &solution);
        let bjt = circuit.bjts.devices[idx].clone();
        let charges = bjt.legacy_transient_charge_state(internal[0], internal[1], internal[2]);

        for (label, actual, expected) in [
            ("vbe", history.vbe_prev[idx], internal[0]),
            ("vbc", history.vbc_prev[idx], internal[1]),
            ("vcs", history.vcs_prev[idx], internal[2]),
        ] {
            assert!(
                (actual - expected).abs() < 1e-12,
                "expected legacy {label} history to use internal charge branch voltage: actual={actual:.12e}, expected={expected:.12e}"
            );
        }
        for (label, actual, expected) in [
            (
                "qbe",
                history.charge_q_prev[idx][BJT_QBE_BRANCH_INDEX],
                charges.qbe,
            ),
            (
                "qbc",
                history.charge_q_prev[idx][BJT_QBC_BRANCH_INDEX],
                charges.qbc,
            ),
            (
                "qcs",
                history.charge_q_prev[idx][BJT_QBCP_BRANCH_INDEX],
                charges.qcs,
            ),
        ] {
            assert!(
                (actual - expected).abs() < 1e-18,
                "expected legacy {label} history charge to match the internal-voltage charge state: actual={actual:.12e}, expected={expected:.12e}"
            );
        }
    }

    #[test]
    fn test_update_reactive_history_uses_internal_legacy_charge_branch_voltages() {
        let (mut circuit, solution, idx, _external, internal) = legacy_bjt_charge_history_fixture();
        let mut bjt_history = Engine::initialize_bjt_history(&circuit, &solution);
        let mut jfet_history = Engine::initialize_jfet_history(&circuit, &solution);
        let mut mosfet_history = Engine::initialize_mosfet_history(&circuit, &solution);
        let mut breakpoints = BreakpointManager::new();
        let tline_dc_refs: Vec<(Value, Value)> = Vec::new();
        let coupled_tline_refs: Vec<CoupledTlineReferenceState> = Vec::new();
        let mut dynamic_breakpoints_added = 0;
        let mut warned_dynamic_breakpoint_cap = false;

        Engine::update_reactive_history(
            &mut circuit,
            &solution,
            0.0,
            1e-12,
            IntegrationMethod::Trapezoidal,
            1,
            &mut bjt_history,
            &mut jfet_history,
            &mut mosfet_history,
            None,
            false,
            &tline_dc_refs,
            &coupled_tline_refs,
            &mut breakpoints,
            200e-9,
            1e-3,
            1e-6,
            &mut dynamic_breakpoints_added,
            &mut warned_dynamic_breakpoint_cap,
        );

        let bjt = circuit.bjts.devices[idx].clone();
        let charges = bjt.legacy_transient_charge_state(internal[0], internal[1], internal[2]);

        for (label, actual, expected) in [
            ("vbe", bjt_history.vbe_prev[idx], internal[0]),
            ("vbc", bjt_history.vbc_prev[idx], internal[1]),
            ("vcs", bjt_history.vcs_prev[idx], internal[2]),
        ] {
            assert!(
                (actual - expected).abs() < 1e-12,
                "expected accepted-step legacy {label} history to stay on the internal charge branch voltage: actual={actual:.12e}, expected={expected:.12e}"
            );
        }
        for (label, actual, expected) in [
            (
                "qbe",
                bjt_history.charge_q_prev[idx][BJT_QBE_BRANCH_INDEX],
                charges.qbe,
            ),
            (
                "qbc",
                bjt_history.charge_q_prev[idx][BJT_QBC_BRANCH_INDEX],
                charges.qbc,
            ),
            (
                "qcs",
                bjt_history.charge_q_prev[idx][BJT_QBCP_BRANCH_INDEX],
                charges.qcs,
            ),
        ] {
            assert!(
                (actual - expected).abs() < 1e-18,
                "expected accepted-step legacy {label} history charge to match the internal-voltage charge state: actual={actual:.12e}, expected={expected:.12e}"
            );
        }
    }

    #[test]
    fn test_legacy_bjt_ngspice_truncation_limit_uses_internal_charge_state() {
        let (circuit, solution, idx, external, internal) = legacy_bjt_charge_history_fixture();
        let mut history = Engine::initialize_bjt_history(&circuit, &solution);
        history.accepted_dt_prev = 1e-12;
        history.accepted_dt_prev_prev = 1e-12;

        assert!(
            (internal[0] - external[0]).abs() > 1e-9
                || (internal[1] - external[1]).abs() > 1e-9
                || (internal[2] - external[2]).abs() > 1e-9,
            "fixture must distinguish external terminal voltages from internal charge branch voltages"
        );

        let bjt = &circuit.bjts.devices[idx];
        let mut candidate = solution.clone();
        for (node, value) in [
            (bjt.node_collector, 2.42),
            (bjt.node_base, 0.872),
            (bjt.node_emitter, 0.104),
            (bjt.node_substrate, 0.0),
        ] {
            if node > 0 {
                candidate[node - 1] = value;
            }
        }

        let dt = 1e-12;
        let actual = Engine::legacy_bjt_ngspice_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &history,
            1e-3,
            1e-12,
            1e-14,
            NGSPICE_DEFAULT_TRTOL,
        )
        .expect("legacy BJT charge truncation should produce a timestep limit");

        let vc = Engine::node_voltage(&candidate, bjt.node_collector);
        let vb = Engine::node_voltage(&candidate, bjt.node_base);
        let ve = Engine::node_voltage(&candidate, bjt.node_emitter);
        let vs = Engine::node_voltage(&candidate, bjt.node_substrate);
        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let (legacy_vbe, legacy_vbc, legacy_vcs) =
            Engine::legacy_bjt_charge_branch_voltages(&snapshot);
        let charges = bjt.legacy_transient_charge_state(legacy_vbe, legacy_vbc, legacy_vcs);
        let effective_method =
            Engine::effective_companion_method(IntegrationMethod::Trapezoidal, 1);

        let mut expected = 2.0 * dt;
        for (branch_idx, capacitance, q_curr) in [
            (BJT_QBE_BRANCH_INDEX, charges.capbe, charges.qbe),
            (BJT_QBC_BRANCH_INDEX, charges.capbc, charges.qbc),
            (BJT_QBCP_BRANCH_INDEX, charges.capcs, charges.qcs),
        ] {
            if capacitance <= 0.0 {
                continue;
            }
            let q_prev = history.charge_q_prev[idx][branch_idx];
            let q_prev_prev = history.charge_q_prev_prev[idx][branch_idx];
            let q_prev_prev_prev = history.charge_q_prev_prev_prev[idx][branch_idx];
            let cq_prev = history.charge_cq_prev[idx][branch_idx];
            let cq_curr = Engine::jfet_companion_ccap(
                effective_method,
                1,
                dt,
                q_curr,
                q_prev,
                q_prev_prev,
                cq_prev,
            );
            let branch_limit = Engine::ngspice_charge_truncation_limit(
                q_curr,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_curr,
                cq_prev,
                dt,
                history.accepted_dt_prev,
                history.accepted_dt_prev_prev,
                effective_method,
                1,
                1e-3,
                1e-12,
                1e-14,
                NGSPICE_DEFAULT_TRTOL,
            )
            .expect("manual legacy BJT branch truncation limit");
            expected = expected.min(branch_limit);
        }

        assert!(
            (actual - expected).abs() <= expected.abs().max(1.0) * 1e-12,
            "expected legacy BJT truncation limit to match internal charge branch CKTterr calculation: actual={actual:.12e}, expected={expected:.12e}"
        );
    }

    #[test]
    fn test_jfet_ngspice_truncation_limit_matches_gate_charge_cktterr() {
        let (circuit, solution, idx) = jfet_charge_history_fixture();
        let mut history = Engine::initialize_jfet_history(&circuit, &solution);
        history.accepted_dt_prev = 1e-12;
        history.accepted_dt_prev_prev = 1e-12;
        history.qgs_prev_prev[idx] = history.qgs_prev[idx] - 3.0e-15;
        history.qgs_prev_prev_prev[idx] = history.qgs_prev_prev[idx] - 2.0e-15;
        history.qgd_prev_prev[idx] = history.qgd_prev[idx] + 2.0e-15;
        history.qgd_prev_prev_prev[idx] = history.qgd_prev_prev[idx] + 1.0e-15;

        let jfet = &circuit.jfets[idx];
        let mut candidate = solution.clone();
        for (node, value) in [(jfet.drain, 1.82), (jfet.gate, 0.62), (jfet.source, -0.08)] {
            if node > 0 {
                candidate[node - 1] = value;
            }
        }

        let dt = 1e-12;
        let actual = Engine::jfet_ngspice_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &history,
            1e-3,
            1e-12,
            1e-14,
            NGSPICE_DEFAULT_TRTOL,
        )
        .expect("JFET charge truncation should produce a timestep limit");

        let effective_method =
            Engine::effective_companion_method(IntegrationMethod::Trapezoidal, 1);
        let (vgs_eval, vgd_eval) = Engine::jfet_branch_voltages(jfet, &candidate);
        let (vgs_charge, vgd_charge) = Engine::jfet_charge_branch_voltages(jfet, &candidate);
        let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);
        let mut expected = 2.0 * dt;
        for (capacitance, voltage, voltage_prev, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev) in [
            (
                cgs,
                vgs_charge,
                history.vgs_prev[idx],
                history.qgs_prev[idx],
                history.qgs_prev_prev[idx],
                history.qgs_prev_prev_prev[idx],
                history.cqgs_prev[idx],
            ),
            (
                cgd,
                vgd_charge,
                history.vgd_prev[idx],
                history.qgd_prev[idx],
                history.qgd_prev_prev[idx],
                history.qgd_prev_prev_prev[idx],
                history.cqgd_prev[idx],
            ),
        ] {
            if capacitance <= 0.0 {
                continue;
            }
            let (_geq, _ieq, q_curr, cq_curr) = Engine::jfet_companion_terms(
                effective_method,
                1,
                dt,
                capacitance,
                voltage,
                voltage_prev,
                q_prev,
                q_prev_prev,
                cq_prev,
            );
            let branch_limit = Engine::ngspice_charge_truncation_limit(
                q_curr,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_curr,
                cq_prev,
                dt,
                history.accepted_dt_prev,
                history.accepted_dt_prev_prev,
                effective_method,
                1,
                1e-3,
                1e-12,
                1e-14,
                NGSPICE_DEFAULT_TRTOL,
            )
            .expect("manual JFET gate charge branch truncation limit");
            expected = expected.min(branch_limit);
        }

        assert!(
            (actual - expected).abs() <= expected.abs().max(1.0) * 1e-12,
            "expected JFET truncation limit to match gate charge CKTterr calculation: actual={actual:.12e}, expected={expected:.12e}"
        );
    }

    #[test]
    fn test_mosfet_ngspice_truncation_limit_matches_gate_charge_cktterr() {
        let (circuit, solution, idx) = mosfet_charge_history_fixture();
        let mut history = Engine::initialize_mosfet_history(&circuit, &solution);
        history.accepted_dt_prev = 1e-12;
        history.accepted_dt_prev_prev = 1e-12;
        history.qgs_prev_prev[idx] = history.qgs_prev[idx] - 3.0e-15;
        history.qgs_prev_prev_prev[idx] = history.qgs_prev_prev[idx] - 2.0e-15;
        history.qgd_prev_prev[idx] = history.qgd_prev[idx] + 2.0e-15;
        history.qgd_prev_prev_prev[idx] = history.qgd_prev_prev[idx] + 1.0e-15;
        history.qgb_prev_prev[idx] = history.qgb_prev[idx] - 1.5e-15;
        history.qgb_prev_prev_prev[idx] = history.qgb_prev_prev[idx] - 0.5e-15;

        let mosfet = &circuit.mosfets.devices[idx];
        let mut candidate = solution.clone();
        for (node, value) in [
            (mosfet.node_drain, 1.62),
            (mosfet.node_gate, 1.14),
            (mosfet.node_source, 0.04),
            (mosfet.node_bulk, -0.02),
        ] {
            if node > 0 {
                candidate[node - 1] = value;
            }
        }

        let dt = 1e-12;
        let actual = Engine::mosfet_ngspice_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &history,
            1e-3,
            1e-12,
            1e-14,
            NGSPICE_DEFAULT_TRTOL,
        )
        .expect("MOSFET gate charge truncation should produce a timestep limit");

        let effective_method =
            Engine::effective_companion_method(IntegrationMethod::Trapezoidal, 1);
        let (vgs_eval, vds_eval, vbs_eval) = mosfet.eval_branch_voltages_at(&candidate);
        let (vgs, vgd, vgb) = mosfet.gate_charge_branch_voltages_at(&candidate);
        let (cgs_half, cgd_half, cgb_half) =
            mosfet.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval);
        let (cgs_ov, cgd_ov, cgb_ov) = mosfet.overlap_capacitances();
        let mut expected = 2.0 * dt;

        for (capacitance, voltage, voltage_prev, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev) in [
            (
                cgs_half + history.capgs_prev_half[idx] + cgs_ov,
                vgs,
                history.vgs_prev[idx],
                history.qgs_prev[idx],
                history.qgs_prev_prev[idx],
                history.qgs_prev_prev_prev[idx],
                history.cqgs_prev[idx],
            ),
            (
                cgd_half + history.capgd_prev_half[idx] + cgd_ov,
                vgd,
                history.vgd_prev[idx],
                history.qgd_prev[idx],
                history.qgd_prev_prev[idx],
                history.qgd_prev_prev_prev[idx],
                history.cqgd_prev[idx],
            ),
            (
                cgb_half + history.capgb_prev_half[idx] + cgb_ov,
                vgb,
                history.vgb_prev[idx],
                history.qgb_prev[idx],
                history.qgb_prev_prev[idx],
                history.qgb_prev_prev_prev[idx],
                history.cqgb_prev[idx],
            ),
        ] {
            if capacitance <= 0.0 {
                continue;
            }
            let (_geq, _ieq, q_curr, cq_curr) = Engine::jfet_companion_terms(
                effective_method,
                1,
                dt,
                capacitance,
                voltage,
                voltage_prev,
                q_prev,
                q_prev_prev,
                cq_prev,
            );
            let branch_limit = Engine::ngspice_charge_truncation_limit(
                q_curr,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_curr,
                cq_prev,
                dt,
                history.accepted_dt_prev,
                history.accepted_dt_prev_prev,
                effective_method,
                1,
                1e-3,
                1e-12,
                1e-14,
                NGSPICE_DEFAULT_TRTOL,
            )
            .expect("manual MOSFET gate charge branch truncation limit");
            expected = expected.min(branch_limit);
        }

        assert!(
            (actual - expected).abs() <= expected.abs().max(1.0) * 1e-12,
            "expected MOSFET truncation limit to match gate charge CKTterr calculation: actual={actual:.12e}, expected={expected:.12e}"
        );
    }

    #[test]
    fn test_bjt_charge_truncation_lte_deferral_requires_covered_reactive_topology() {
        let (mut circuit, _solution, _idx, _external, _internal) =
            legacy_bjt_charge_history_fixture();

        assert!(Engine::bjt_charge_truncation_covers_transient_lte(
            &circuit,
            Some(1e-12)
        ));
        assert!(!Engine::bjt_charge_truncation_covers_transient_lte(
            &circuit, None
        ));

        circuit.capacitors.add("Cextra".to_string(), 1, 0, 1e-12);
        assert!(!Engine::bjt_charge_truncation_covers_transient_lte(
            &circuit,
            Some(1e-12)
        ));
    }

    #[test]
    fn test_jfet_charge_truncation_lte_deferral_requires_covered_reactive_topology() {
        let (mut circuit, _solution, _idx) = jfet_charge_history_fixture();

        assert!(Engine::jfet_charge_truncation_covers_transient_lte(
            &circuit,
            Some(1e-12)
        ));
        assert!(!Engine::jfet_charge_truncation_covers_transient_lte(
            &circuit, None
        ));

        circuit.capacitors.add("Cextra".to_string(), 1, 0, 1e-12);
        assert!(!Engine::jfet_charge_truncation_covers_transient_lte(
            &circuit,
            Some(1e-12)
        ));
    }

    #[test]
    fn test_mosfet_charge_truncation_lte_deferral_requires_covered_reactive_topology() {
        let (mut circuit, _solution, _idx) = mosfet_charge_history_fixture();

        assert!(Engine::mosfet_charge_truncation_covers_transient_lte(
            &circuit,
            Some(1e-12)
        ));
        assert!(!Engine::mosfet_charge_truncation_covers_transient_lte(
            &circuit, None
        ));

        circuit.capacitors.add("Cextra".to_string(), 1, 0, 1e-12);
        assert!(!Engine::mosfet_charge_truncation_covers_transient_lte(
            &circuit,
            Some(1e-12)
        ));
    }

    #[test]
    fn test_capacitor_ngspice_truncation_limit_matches_cktterr_mapping() {
        let mut circuit = crate::circuit::Circuit::new();
        let node = circuit.get_or_create_node("out");
        let capacitance = 2.5e-12;
        circuit
            .capacitors
            .add("Cload".to_string(), node, 0, capacitance);
        circuit.capacitors.v_prev[0] = 1.0;
        circuit.capacitors.v_prev_prev[0] = 0.82;
        circuit.capacitors.v_prev_prev_prev[0] = 0.70;
        circuit.capacitors.i_prev[0] = 1.5e-6;

        let candidate = vec![1.22];
        let dt = 1e-12;
        let prev_dt = 1.1e-12;
        let prev_prev_dt = 0.9e-12;
        let actual = Engine::capacitor_ngspice_truncation_limit(
            &circuit,
            &candidate,
            IntegrationMethod::Trapezoidal,
            2,
            dt,
            prev_dt,
            prev_prev_dt,
            1e-3,
            1e-12,
            1e-14,
            NGSPICE_DEFAULT_TRTOL,
        )
        .expect("capacitor CKTterr limit");

        let effective_method =
            Engine::effective_companion_method(IntegrationMethod::Trapezoidal, 2);
        let coeff = CompanionCoefficients::for_method(effective_method);
        let geq = coeff.capacitor_geq(capacitance, dt);
        let ieq = coeff.capacitor_ieq(
            capacitance,
            dt,
            circuit.capacitors.v_prev[0],
            circuit.capacitors.v_prev_prev[0],
            circuit.capacitors.i_prev[0],
        );
        let voltage = candidate[0];
        let branch_limit = Engine::ngspice_charge_truncation_limit(
            capacitance * voltage,
            capacitance * circuit.capacitors.v_prev[0],
            capacitance * circuit.capacitors.v_prev_prev[0],
            capacitance * circuit.capacitors.v_prev_prev_prev[0],
            geq * voltage - ieq,
            circuit.capacitors.i_prev[0],
            dt,
            prev_dt,
            prev_prev_dt,
            effective_method,
            2,
            1e-3,
            1e-12,
            1e-14,
            NGSPICE_DEFAULT_TRTOL,
        )
        .expect("manual CKTterr limit");
        let expected = (2.0 * dt).min(branch_limit);

        assert!(
            (actual - expected).abs() <= expected.abs().max(1e-30) * 1e-12,
            "expected capacitor truncation to map voltage history into CKTterr charge state"
        );
    }

    #[test]
    fn test_ngspice_device_truncation_lte_deferral_allows_transmission_line_topology() {
        let mut circuit = crate::circuit::Circuit::new();
        let near = circuit.get_or_create_node("near");
        let far = circuit.get_or_create_node("far");
        circuit.capacitors.add("Cload".to_string(), near, 0, 1e-12);
        circuit.tlines.push(crate::device::TransmissionLine::new(
            "T1".to_string(),
            near,
            0,
            far,
            0,
            50.0,
            1e-9,
        ));

        assert!(Engine::ngspice_device_truncation_covers_transient_lte(
            &circuit,
            Some(1e-12),
            None,
            None,
            None,
        ));
        assert!(!Engine::ngspice_device_truncation_covers_transient_lte(
            &circuit, None, None, None, None,
        ));

        circuit
            .inductors
            .add("Lunsupported".to_string(), near, 0, 1, 1e-9);
        assert!(!Engine::ngspice_device_truncation_covers_transient_lte(
            &circuit,
            Some(1e-12),
            None,
            None,
            None,
        ));
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
    fn test_is_stale_step_false_for_sub_threshold_source_activity() {
        let prev = vec![0.5, 1.0, -2.0];
        let next = prev.clone();
        assert!(!Engine::is_stale_step(
            &prev,
            &next,
            SOURCE_ACTIVE_DELTA * 0.1,
            prev.len()
        ));
    }

    #[test]
    fn test_is_stale_step_true_when_solution_does_not_follow_source() {
        let prev = vec![0.5, 1.0, -2.0];
        let next = prev.clone();
        assert!(Engine::is_stale_step(
            &prev,
            &next,
            SOURCE_ACTIVE_DELTA * 2.0,
            prev.len()
        ));
    }

    #[test]
    fn test_is_stale_step_false_when_solution_moves_with_source() {
        let prev = vec![0.5, 1.0, -2.0];
        let next = vec![0.5002, 1.0001, -1.9999];
        assert!(!Engine::is_stale_step(
            &prev,
            &next,
            SOURCE_ACTIVE_DELTA * 2.0,
            prev.len()
        ));
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
    fn test_is_excessive_quiet_force_candidate_rejects_large_quiet_jump() {
        let prev = vec![1.0, 2.0];
        let next = vec![1.2, 2.0];
        assert!(Engine::is_excessive_quiet_force_candidate(
            &prev,
            &next,
            0.0,
            prev.len(),
            MAX_FORCE_ACCEPT_DELTA_V,
        ));
    }

    #[test]
    fn test_is_excessive_quiet_force_candidate_allows_same_jump_with_active_source() {
        let prev = vec![1.0, 2.0];
        let next = vec![1.2, 2.0];
        assert!(!Engine::is_excessive_quiet_force_candidate(
            &prev,
            &next,
            SOURCE_ACTIVE_DELTA * 2.0,
            prev.len(),
            MAX_FORCE_ACCEPT_DELTA_V,
        ));
    }

    #[test]
    fn test_is_stagnant_force_candidate_detects_identical_solution() {
        let circuit = crate::circuit::Circuit::new();
        let previous = vec![1.0, -2.0, 3.5, 0.0];
        let candidate = previous.clone();
        assert!(Engine::is_stagnant_force_candidate(
            &circuit,
            &previous,
            &candidate,
            previous.len(),
            1e-12,
            1e-12,
        ));
    }

    #[test]
    fn test_is_stagnant_force_candidate_allows_meaningful_solution_change() {
        let circuit = crate::circuit::Circuit::new();
        let previous = vec![1.0, -2.0, 3.5, 0.0];
        let candidate = vec![1.0, -2.0, 3.5 + 1e-6, 0.0];
        assert!(!Engine::is_stagnant_force_candidate(
            &circuit,
            &previous,
            &candidate,
            previous.len(),
            1e-12,
            1e-12,
        ));
    }

    #[test]
    fn test_is_stagnant_force_candidate_ignores_algebraic_source_current_motion() {
        let mut circuit = crate::circuit::Circuit::new();
        circuit.voltage_sources.add("V1".to_string(), 1, 0, 1, 1.0);

        let previous = vec![1.0, 0.0];
        let candidate = vec![1.0, 2.5e-4];
        assert!(Engine::is_stagnant_force_candidate(
            &circuit, &previous, &candidate, 1, 1e-12, 1e-12,
        ));
    }

    #[test]
    fn test_is_stagnant_force_candidate_keeps_dynamic_inductor_current_progress() {
        let mut circuit = crate::circuit::Circuit::new();
        circuit.inductors.add("L1".to_string(), 1, 0, 1, 1e-9);

        let previous = vec![1.0, 0.0];
        let candidate = vec![1.0, 2.5e-4];
        assert!(!Engine::is_stagnant_force_candidate(
            &circuit, &previous, &candidate, 1, 1e-12, 1e-12,
        ));
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
    fn test_bounded_force_accept_candidate_clips_floating_voltage_source_common_mode() {
        let mut circuit = crate::circuit::Circuit::new();
        let node_pos = circuit.get_or_create_node("P");
        let node_neg = circuit.get_or_create_node("N");
        circuit
            .voltage_sources
            .add("V1".to_string(), node_pos, node_neg, 1, 0.01);

        let previous = vec![1.655, 1.645];
        let candidate = vec![100.0, 100.0];
        let protected_nodes = circuit.force_accept_protected_nodes();
        let floating_pairs = circuit.force_accept_floating_ideal_output_pairs();

        let bounded = Engine::bounded_force_accept_candidate(
            &circuit,
            &previous,
            &candidate,
            0.0,
            circuit.num_nodes(),
            MAX_FORCE_ACCEPT_DELTA_V,
            &protected_nodes,
            &floating_pairs,
        );

        let midpoint_prev = 0.5 * (previous[0] + previous[1]);
        let midpoint_bounded = 0.5 * (bounded[0] + bounded[1]);
        assert!(
            (midpoint_bounded - (midpoint_prev + MAX_FORCE_ACCEPT_DELTA_V)).abs() < 1e-12,
            "expected floating source midpoint to be clipped against the accepted common mode"
        );
        assert!(
            ((bounded[0] - bounded[1]) - 0.01).abs() < 1e-12,
            "expected floating source differential to remain exact after common-mode clipping"
        );
        assert!(
            Engine::max_abs_delta_prefix(&previous, &bounded, circuit.num_nodes())
                <= MAX_FORCE_ACCEPT_DELTA_V + 1e-12
        );
        assert!(!Engine::is_unbounded_step(
            &previous,
            &bounded,
            1e-6,
            circuit.num_nodes()
        ));
    }

    #[test]
    fn test_clip_floating_ideal_output_common_modes_clips_regular_newton_source_midpoint() {
        let mut circuit = crate::circuit::Circuit::new();
        let node_pos = circuit.get_or_create_node("P");
        let node_neg = circuit.get_or_create_node("N");
        circuit
            .voltage_sources
            .add("V1".to_string(), node_pos, node_neg, 1, 0.01);

        let previous = vec![1.655, 1.645];
        let mut candidate = vec![3.41, 3.40];
        let ideal_pairs = circuit.ideal_voltage_output_pairs();

        Engine::clip_ideal_output_common_modes(&previous, &mut candidate, 0.2, &ideal_pairs);

        let midpoint_prev = 0.5 * (previous[0] + previous[1]);
        let midpoint_candidate = 0.5 * (candidate[0] + candidate[1]);
        assert!(
            (midpoint_candidate - (midpoint_prev + 0.2)).abs() < 1e-12,
            "expected floating source midpoint clipping during regular Newton limiting"
        );
        assert!(
            ((candidate[0] - candidate[1]) - 0.01).abs() < 1e-12,
            "expected floating source differential to remain exact after regular Newton clipping"
        );
    }

    #[test]
    fn test_bounded_force_accept_candidate_restores_voltage_source_branch_current() {
        let mut circuit = crate::circuit::Circuit::new();
        let node_pos = circuit.get_or_create_node("P");
        circuit
            .voltage_sources
            .add("V1".to_string(), node_pos, 0, 1, 1.0);

        let previous = vec![1.0, 2.5e-6];
        let candidate = vec![1.0, 1.4e11];
        let protected_nodes = circuit.force_accept_protected_nodes();
        let floating_pairs = circuit.force_accept_floating_ideal_output_pairs();

        let bounded = Engine::bounded_force_accept_candidate(
            &circuit,
            &previous,
            &candidate,
            0.0,
            circuit.num_nodes(),
            MAX_FORCE_ACCEPT_DELTA_V,
            &protected_nodes,
            &floating_pairs,
        );

        assert!((bounded[0] - 1.0).abs() < 1e-12);
        assert!(
            (bounded[1] - previous[1]).abs() < 1e-18,
            "expected algebraic voltage-source branch current to stay at the accepted value"
        );
    }

    #[test]
    fn test_bounded_force_accept_candidate_preserves_vcvs_branch_current_candidate() {
        let mut circuit = crate::circuit::Circuit::new();
        let out_pos = circuit.get_or_create_node("OUTP");
        let out_neg = circuit.get_or_create_node("OUTN");
        let ctrl_pos = circuit.get_or_create_node("CTRLP");
        let ctrl_neg = circuit.get_or_create_node("CTRLN");
        circuit.vcvs.add(
            "E1".to_string(),
            out_pos,
            out_neg,
            ctrl_pos,
            ctrl_neg,
            1,
            2.0,
        );

        let previous = vec![1.5, -0.5, 2.0, 1.0, 2.5e-6];
        let candidate = vec![1.5, -0.5, 2.0, 1.0, 7.5e-4];
        let protected_nodes = circuit.force_accept_protected_nodes();
        let floating_pairs = circuit.force_accept_floating_ideal_output_pairs();

        let bounded = Engine::bounded_force_accept_candidate(
            &circuit,
            &previous,
            &candidate,
            0.0,
            circuit.num_nodes(),
            MAX_FORCE_ACCEPT_DELTA_V,
            &protected_nodes,
            &floating_pairs,
        );

        assert!(
            (bounded[4] - candidate[4]).abs() < 1e-18,
            "expected the controlled-source branch current to remain aligned with the solver candidate"
        );
    }

    #[test]
    fn test_bounded_force_accept_candidate_preserves_inductor_branch_current_candidate() {
        let mut circuit = crate::circuit::Circuit::new();
        let node_pos = circuit.get_or_create_node("P");
        circuit
            .inductors
            .add("L1".to_string(), node_pos, 0, 1, 1e-9);

        let previous = vec![1.0, 2.5e-6];
        let candidate = vec![1.0, 3.0e-4];
        let protected_nodes = circuit.force_accept_protected_nodes();
        let floating_pairs = circuit.force_accept_floating_ideal_output_pairs();

        let bounded = Engine::bounded_force_accept_candidate(
            &circuit,
            &previous,
            &candidate,
            0.0,
            circuit.num_nodes(),
            MAX_FORCE_ACCEPT_DELTA_V,
            &protected_nodes,
            &floating_pairs,
        );

        assert!(
            (bounded[1] - candidate[1]).abs() < 1e-18,
            "expected dynamic inductor branch current to remain available as force-accept state"
        );
    }

    #[test]
    fn test_bounded_force_accept_candidate_clips_floating_vcvs_common_mode() {
        let mut circuit = crate::circuit::Circuit::new();
        let out_pos = circuit.get_or_create_node("OUTP");
        let out_neg = circuit.get_or_create_node("OUTN");
        let ctrl_pos = circuit.get_or_create_node("CTRLP");
        let ctrl_neg = circuit.get_or_create_node("CTRLN");
        circuit.vcvs.add(
            "E1".to_string(),
            out_pos,
            out_neg,
            ctrl_pos,
            ctrl_neg,
            1,
            2.0,
        );

        let previous = vec![1.5, -0.5, 2.0, 1.0];
        let candidate = vec![200.0, 198.0, 2.0, 1.0];
        let protected_nodes = circuit.force_accept_protected_nodes();
        let floating_pairs = circuit.force_accept_floating_ideal_output_pairs();

        let bounded = Engine::bounded_force_accept_candidate(
            &circuit,
            &previous,
            &candidate,
            0.0,
            circuit.num_nodes(),
            MAX_FORCE_ACCEPT_DELTA_V,
            &protected_nodes,
            &floating_pairs,
        );

        let midpoint_prev = 0.5 * (previous[0] + previous[1]);
        let midpoint_bounded = 0.5 * (bounded[0] + bounded[1]);
        assert!(
            (midpoint_bounded - (midpoint_prev + MAX_FORCE_ACCEPT_DELTA_V)).abs() < 1e-12,
            "expected floating VCVS midpoint to be clipped against the accepted common mode"
        );
        assert!(
            ((bounded[0] - bounded[1]) - 2.0).abs() < 1e-12,
            "expected floating VCVS differential to remain exact after common-mode clipping"
        );
        assert!(
            Engine::max_abs_delta_prefix(&previous, &bounded, circuit.num_nodes())
                <= MAX_FORCE_ACCEPT_DELTA_V + 1e-12
        );
        assert!(!Engine::is_unbounded_step(
            &previous,
            &bounded,
            1e-6,
            circuit.num_nodes()
        ));
    }

    #[test]
    fn test_bounded_force_accept_candidate_does_not_clip_anchored_voltage_source_common_mode() {
        let mut circuit = crate::circuit::Circuit::new();
        let node_pos = circuit.get_or_create_node("P");
        let node_neg = circuit.get_or_create_node("N");
        circuit
            .voltage_sources
            .add("V1".to_string(), node_pos, node_neg, 1, 0.01);
        circuit.resistors.add("R1".to_string(), node_neg, 0, 1e3);

        let previous = vec![1.655, 1.645];
        let candidate = vec![1.725, 1.715];
        let protected_nodes = circuit.force_accept_protected_nodes();
        let floating_pairs = circuit.force_accept_floating_ideal_output_pairs();

        assert!(
            floating_pairs.is_empty(),
            "expected the grounded resistor path to anchor the source common mode"
        );

        let bounded = Engine::bounded_force_accept_candidate(
            &circuit,
            &previous,
            &candidate,
            0.0,
            circuit.num_nodes(),
            MAX_FORCE_ACCEPT_DELTA_V,
            &protected_nodes,
            &floating_pairs,
        );

        assert!((bounded[0] - candidate[0]).abs() < 1e-12);
        assert!((bounded[1] - candidate[1]).abs() < 1e-12);
        assert!(
            (0.5 * (bounded[0] + bounded[1]) - 0.5 * (candidate[0] + candidate[1])).abs() < 1e-12
        );
    }

    #[test]
    fn test_transient_newton_iteration_budget_keeps_standard_budget_for_vbic_decks() {
        assert_eq!(Engine::transient_newton_iteration_budget(50, false, 0), 200);
        assert_eq!(Engine::transient_newton_iteration_budget(50, true, 0), 96);
        assert_eq!(Engine::transient_newton_iteration_budget(50, true, 1), 200);
    }

    #[test]
    fn test_transient_newton_iteration_budget_preserves_minimum_floor_for_low_vbic_budget() {
        assert_eq!(Engine::transient_newton_iteration_budget(8, false, 0), 32);
        assert_eq!(Engine::transient_newton_iteration_budget(8, true, 0), 64);
        assert_eq!(Engine::transient_newton_iteration_budget(8, true, 3), 64);
    }

    #[test]
    fn test_vbic_relaxed_convergence_requires_device_and_residual_convergence() {
        assert!(!Engine::vbic_relaxed_convergence_met(
            true, true, false, true
        ));
        assert!(!Engine::vbic_relaxed_convergence_met(
            true, true, true, false
        ));
        assert!(!Engine::vbic_relaxed_convergence_met(
            false, true, true, true
        ));
        assert!(Engine::vbic_relaxed_convergence_met(true, true, true, true));
    }

    #[test]
    fn test_force_accept_recovery_timestep_retreats_from_current_step() {
        assert!(
            (Engine::force_accept_recovery_timestep(8e-12, 1e-12, 1e-8, None) - 4e-12).abs()
                < 1e-24
        );
    }

    #[test]
    fn test_nonconvergence_retry_timestep_matches_ngspice_eighth_step_cut() {
        assert!((Engine::nonconvergence_retry_timestep(8e-12, 1e-8) - 1e-12).abs() < 1e-24);
    }

    #[test]
    fn test_should_skip_post_accept_timestep_control_on_first_step_only() {
        assert!(Engine::should_skip_post_accept_timestep_control_on_first_step(1));
        assert!(!Engine::should_skip_post_accept_timestep_control_on_first_step(2));
    }

    #[test]
    fn test_force_accept_recovery_timestep_respects_max_step_cap() {
        assert!(
            (Engine::force_accept_recovery_timestep(8e-12, 1e-12, 3e-12, None) - 3e-12).abs()
                < 1e-24
        );
    }

    #[test]
    fn test_force_accept_recovery_timestep_recovers_tiny_step_geometrically() {
        assert!(
            (Engine::force_accept_recovery_timestep(1e-19, 1e-12, 1e-8, None)
                - 3.162_277_660_168_379_3e-16)
                .abs()
                < 1e-28
        );
    }

    #[test]
    fn test_force_accept_recovery_timestep_recovers_sub_preferred_step_geometrically() {
        assert!(
            (Engine::force_accept_recovery_timestep(8e-13, 1e-12, 1e-8, None)
                - 8.944_271_909_999_16e-13)
                .abs()
                < 1e-25
        );
    }

    #[test]
    fn test_force_accept_recovery_timestep_caps_recovery_with_exact_vbic_limit() {
        assert!(
            (Engine::force_accept_recovery_timestep(8e-12, 1e-12, 1e-8, Some(2.5e-12)) - 2.5e-12)
                .abs()
                < 1e-24
        );
    }

    #[test]
    fn test_force_accept_recovery_timestep_respects_subfloor_vbic_limit() {
        assert!(
            (Engine::force_accept_recovery_timestep(1e-19, 1e-12, 1e-8, Some(2.5e-17)) - 2.5e-17)
                .abs()
                < 1e-28
        );
    }

    #[test]
    fn test_force_accept_recovery_timestep_ignores_nonphysical_vbic_limit() {
        assert!(
            (Engine::force_accept_recovery_timestep(1e-19, 1e-12, 1e-8, Some(-1.0))
                - 3.162_277_660_168_379_3e-16)
                .abs()
                < 1e-28
        );
    }

    #[test]
    fn test_should_retry_ngspice_charge_truncation_matches_ngspice_reject_threshold() {
        assert!(Engine::should_retry_ngspice_charge_truncation(
            8.9e-13, 1.0e-12
        ));
        assert!(Engine::should_retry_ngspice_charge_truncation(
            9.0e-13, 1.0e-12
        ));
        assert!(!Engine::should_retry_ngspice_charge_truncation(
            9.1e-13, 1.0e-12
        ));
        assert!(!Engine::should_retry_ngspice_charge_truncation(
            1.4e-12, 1.0e-12
        ));
    }

    #[test]
    fn test_should_promote_ngspice_charge_truncation_matches_ngspice_threshold() {
        assert!(!Engine::should_promote_ngspice_charge_truncation(
            1.04e-12, 1.0e-12
        ));
        assert!(!Engine::should_promote_ngspice_charge_truncation(
            1.05e-12, 1.0e-12
        ));
        assert!(Engine::should_promote_ngspice_charge_truncation(
            1.06e-12, 1.0e-12
        ));
    }

    #[test]
    fn test_next_trapezoidal_order_after_accepted_step_preserves_second_order() {
        assert_eq!(
            Engine::next_trapezoidal_order_after_accepted_step(2, false, false),
            2
        );
    }

    #[test]
    fn test_next_trapezoidal_order_after_accepted_step_promotes_first_order_when_allowed() {
        assert_eq!(
            Engine::next_trapezoidal_order_after_accepted_step(1, false, true),
            2
        );
    }

    #[test]
    fn test_next_trapezoidal_order_after_accepted_step_restarts_at_breakpoints() {
        assert_eq!(
            Engine::next_trapezoidal_order_after_accepted_step(2, true, true),
            1
        );
    }

    #[test]
    fn test_trapezoidal_order_after_timestep_control_reject_preserves_second_order() {
        assert_eq!(
            Engine::trapezoidal_order_after_timestep_control_reject(2),
            2
        );
    }

    #[test]
    fn test_trapezoidal_order_after_timestep_control_reject_keeps_first_order() {
        assert_eq!(
            Engine::trapezoidal_order_after_timestep_control_reject(1),
            1
        );
    }

    #[test]
    fn test_vbic_charge_lte_startup_window_end_prefers_td_scaled_window() {
        let end = Engine::vbic_charge_lte_startup_window_end(1e-8, Some(2e-11));
        assert!((end - 6.4e-10).abs() < 1e-22);
    }

    #[test]
    fn test_vbic_charge_lte_startup_window_end_falls_back_to_maxstep_window_without_td() {
        let end = Engine::vbic_charge_lte_startup_window_end(1e-8, None);
        assert!((end - 1e-9).abs() < 1e-22);
    }

    #[test]
    fn test_vbic_excess_phase_startup_step_cap_tracks_td_quarter() {
        let cap = Engine::vbic_excess_phase_startup_step_cap(1e-8, Some(2e-11))
            .expect("expected td-derived startup cap");
        assert!((cap - 5e-12).abs() < 1e-24);
    }

    #[test]
    fn test_vbic_excess_phase_startup_step_cap_returns_none_without_td() {
        assert!(Engine::vbic_excess_phase_startup_step_cap(1e-8, None).is_none());
    }

    #[test]
    fn test_should_use_vbic_charge_lte_startup_guard_turns_off_after_startup_window() {
        assert!(Engine::should_use_vbic_charge_lte_startup_guard(
            true,
            6.4e-10,
            1e-8,
            Some(2e-11)
        ));
        assert!(!Engine::should_use_vbic_charge_lte_startup_guard(
            true,
            6.5e-10,
            1e-8,
            Some(2e-11)
        ));
    }

    #[test]
    fn test_should_hold_vbic_excess_phase_first_order_follows_ngspice_no_special_case() {
        assert!(!Engine::should_hold_vbic_excess_phase_first_order(
            true,
            6.4e-10,
            1e-8,
            Some(2e-11)
        ));
        assert!(!Engine::should_hold_vbic_excess_phase_first_order(
            true,
            6.5e-10,
            1e-8,
            Some(2e-11)
        ));
        assert!(!Engine::should_hold_vbic_excess_phase_first_order(
            false,
            1e-10,
            1e-8,
            Some(2e-11)
        ));
    }

    #[test]
    fn test_should_use_vbic_charge_lte_estimator_disables_recovery_sized_steps() {
        assert!(!Engine::should_use_vbic_charge_lte_estimator(
            true,
            5e-10,
            1e-8,
            Some(2e-11),
            1e-12,
            1e-12
        ));
        assert!(!Engine::should_use_vbic_charge_lte_estimator(
            true,
            5e-10,
            1e-8,
            Some(2e-11),
            5e-13,
            1e-12
        ));
        assert!(Engine::should_use_vbic_charge_lte_estimator(
            true,
            5e-10,
            1e-8,
            Some(2e-11),
            2e-12,
            1e-12
        ));
    }

    #[test]
    fn test_should_defer_voltage_lte_to_vbic_truncation_when_charge_lte_is_temporarily_idle() {
        assert!(Engine::should_defer_voltage_lte_to_vbic_truncation(
            true,
            5e-10,
            1e-8,
            Some(2e-11),
            Some(8e-13),
            false,
        ));
    }

    #[test]
    fn test_should_not_defer_voltage_lte_without_vbic_truncation_limit_or_charge_window() {
        assert!(!Engine::should_defer_voltage_lte_to_vbic_truncation(
            true,
            8e-10,
            1e-8,
            Some(2e-11),
            Some(8e-13),
            false,
        ));
        assert!(!Engine::should_defer_voltage_lte_to_vbic_truncation(
            true,
            5e-10,
            1e-8,
            Some(2e-11),
            None,
            false,
        ));
        assert!(!Engine::should_defer_voltage_lte_to_vbic_truncation(
            true,
            5e-10,
            1e-8,
            Some(2e-11),
            Some(8e-13),
            true,
        ));
    }

    #[test]
    fn test_min_retries_at_minimum_timestep_allows_extra_vbic_retries() {
        assert_eq!(
            Engine::min_retries_at_minimum_timestep(false, 5e-10, 1e-8),
            1
        );
        assert_eq!(
            Engine::min_retries_at_minimum_timestep(true, 5e-10, 1e-8),
            3
        );
        assert_eq!(Engine::min_retries_at_minimum_timestep(true, 2e-9, 1e-8), 1);
    }

    #[test]
    fn test_bias_transient_step_for_source_activity_keeps_recovery_dt_below_preferred_floor() {
        let dt =
            Engine::bias_transient_step_for_source_activity(1e-15, 1e-6, false, 1e-6, 1e-11, 1e-12);

        assert!((dt - 1e-15).abs() < 1e-27);
    }

    #[test]
    fn test_bias_transient_step_for_source_activity_caps_large_active_source_step() {
        let dt =
            Engine::bias_transient_step_for_source_activity(1e-8, 1e-6, false, 5e-2, 1e-11, 8e-11);

        assert!((dt - 1e-11).abs() < 1e-23);
    }

    #[test]
    fn test_effective_companion_method_restarts_trapezoidal_with_backward_euler() {
        assert_eq!(
            Engine::effective_companion_method(IntegrationMethod::Trapezoidal, 1),
            IntegrationMethod::BackwardEuler
        );
        assert_eq!(
            Engine::effective_companion_method(IntegrationMethod::TrapGear, 1),
            IntegrationMethod::BackwardEuler
        );
        assert_eq!(
            Engine::effective_companion_method(IntegrationMethod::Trapezoidal, 2),
            IntegrationMethod::Trapezoidal
        );
        assert_eq!(
            Engine::effective_companion_method(IntegrationMethod::Gear2, 2),
            IntegrationMethod::Gear2
        );
    }

    #[test]
    fn test_breakpoint_restart_uses_backward_euler_linear_reactive_coefficients() {
        let c = 1e-12;
        let dt = 5e-10;
        let v_prev = 1.25;
        let i_prev = 7.0;

        let coeff = CompanionCoefficients::for_method(Engine::effective_companion_method(
            IntegrationMethod::Trapezoidal,
            1,
        ));

        let geq = coeff.capacitor_geq(c, dt);
        let ieq = coeff.capacitor_ieq(c, dt, v_prev, 0.0, i_prev);

        assert!((geq - (c / dt)).abs() < 1e-18);
        assert!((ieq - (c * v_prev / dt)).abs() < 1e-18);
    }

    #[test]
    fn test_update_reactive_history_uses_restart_order_for_capacitor_current() {
        let c = 1e-12;
        let dt = 5e-10;
        let v_prev = 1.0;
        let v_new = 1.2;
        let stale_trapezoidal_current = 7.0;

        let mut circuit = crate::circuit::Circuit::new();
        let node = circuit.get_or_create_node("p");
        circuit.capacitors.add("C1".to_string(), node, 0, c);
        circuit.capacitors.v_prev[0] = v_prev;
        circuit.capacitors.v_prev_prev[0] = v_prev;
        circuit.capacitors.i_prev[0] = stale_trapezoidal_current;

        let solution = vec![v_new; circuit.matrix_size()];
        let mut bjt_history = Engine::initialize_bjt_history(&circuit, &solution);
        let mut jfet_history = Engine::initialize_jfet_history(&circuit, &solution);
        let mut mosfet_history = Engine::initialize_mosfet_history(&circuit, &solution);
        let mut breakpoints = BreakpointManager::new();
        let tline_dc_refs: Vec<(Value, Value)> = Vec::new();
        let coupled_tline_refs: Vec<CoupledTlineReferenceState> = Vec::new();
        let mut dynamic_breakpoints_added = 0;
        let mut warned_dynamic_breakpoint_cap = false;

        Engine::update_reactive_history(
            &mut circuit,
            &solution,
            dt,
            dt,
            IntegrationMethod::Trapezoidal,
            1,
            &mut bjt_history,
            &mut jfet_history,
            &mut mosfet_history,
            None,
            false,
            &tline_dc_refs,
            &coupled_tline_refs,
            &mut breakpoints,
            10.0 * dt,
            1e-3,
            1e-6,
            &mut dynamic_breakpoints_added,
            &mut warned_dynamic_breakpoint_cap,
        );

        let expected_backward_euler_current = c * (v_new - v_prev) / dt;
        let stale_trapezoidal_identity_current =
            (2.0 * c / dt) * v_new - ((2.0 * c / dt) * v_prev + stale_trapezoidal_current);

        assert!(
            (circuit.capacitors.i_prev[0] - expected_backward_euler_current).abs() < 1e-18,
            "accepted order-1 trapezoidal restart must update capacitor history with backward Euler current"
        );
        assert!(
            (circuit.capacitors.i_prev[0] - stale_trapezoidal_identity_current).abs() > 1e-3,
            "test setup must distinguish restart-order history from stale trapezoidal history"
        );
    }

    #[test]
    fn test_linear_charge_history_ieq_matches_trapezoidal_companion_identity() {
        let dt = 5e-10;
        let q_prev = 4.2e-12;
        let q_prev_prev = 3.8e-12;
        let cq_prev = -7.5e-4;
        let q_curr = 5.1e-12;
        let geq = Engine::jfet_companion_geq(IntegrationMethod::Trapezoidal, 2, 1.0, dt);
        let cq_curr = Engine::jfet_companion_ccap(
            IntegrationMethod::Trapezoidal,
            2,
            dt,
            q_curr,
            q_prev,
            q_prev_prev,
            cq_prev,
        );
        let ieq = Engine::linear_charge_history_ieq(
            IntegrationMethod::Trapezoidal,
            2,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        );

        assert!(((geq * q_curr - cq_curr) - ieq).abs() < 1e-18);
    }

    #[test]
    fn test_startup_step_delta_limit_relaxes_only_for_linearized_seed_window() {
        let base = 5e-2;
        let max_step = 5e-9;
        let early = Engine::startup_step_delta_limit(
            startup::InitialSolutionMode::LinearizedSeed,
            20e-9,
            max_step,
            base,
        );
        let late = Engine::startup_step_delta_limit(
            startup::InitialSolutionMode::LinearizedSeed,
            200e-9,
            max_step,
            base,
        );
        assert!(early > base);
        assert!((late - base).abs() < 1e-18);
    }

    #[test]
    fn test_startup_step_delta_limit_unchanged_for_non_recovery_startup_modes() {
        let base = 5e-2;
        let max_step = 5e-9;
        let tranop = Engine::startup_step_delta_limit(
            startup::InitialSolutionMode::TransientOperatingPoint,
            20e-9,
            max_step,
            base,
        );
        let dc = Engine::startup_step_delta_limit(
            startup::InitialSolutionMode::DcOperatingPoint,
            20e-9,
            max_step,
            base,
        );
        let robust = Engine::startup_step_delta_limit(
            startup::InitialSolutionMode::RobustDcFallback,
            20e-9,
            max_step,
            base,
        );
        assert!((tranop - base).abs() < 1e-18);
        assert!((dc - base).abs() < 1e-18);
        assert!((robust - base).abs() < 1e-18);
    }

    #[test]
    fn test_startup_step_delta_limit_relaxes_early_vbic_excess_phase_steps_after_dc_op() {
        let base = 1e-2;
        let relaxed = Engine::startup_step_delta_limit_with_vbic_td(
            startup::InitialSolutionMode::DcOperatingPoint,
            true,
            Some(20e-12),
            100e-12,
            10e-9,
            base,
        );
        let late = Engine::startup_step_delta_limit_with_vbic_td(
            startup::InitialSolutionMode::DcOperatingPoint,
            true,
            Some(20e-12),
            1e-9,
            10e-9,
            base,
        );
        assert!((relaxed - VBIC_STARTUP_RECOVERY_DELTA_V).abs() < 1e-18);
        assert!((late - base).abs() < 1e-18);
    }

    #[test]
    fn test_startup_step_delta_limit_uses_stronger_relaxation_for_vbic_linearized_seed() {
        let base = 5e-2;
        let relaxed = Engine::startup_step_delta_limit_with_vbic_td(
            startup::InitialSolutionMode::LinearizedSeed,
            true,
            Some(20e-12),
            20e-12,
            10e-9,
            base,
        );
        assert!((relaxed - VBIC_STARTUP_RECOVERY_DELTA_V).abs() < 1e-18);
    }

    #[test]
    fn test_startup_force_accept_delta_limit_relaxes_early_vbic_excess_phase_steps_after_dc_op() {
        let base = 5e-2;
        let relaxed = Engine::startup_force_accept_delta_limit_with_vbic_td(
            startup::InitialSolutionMode::DcOperatingPoint,
            true,
            Some(20e-12),
            100e-12,
            10e-9,
            base,
        );
        assert!((relaxed - VBIC_STARTUP_RECOVERY_DELTA_V).abs() < 1e-18);
    }

    #[test]
    fn test_startup_force_accept_delta_limit_returns_to_base_after_vbic_td_window() {
        let base = 5e-2;
        let late = Engine::startup_force_accept_delta_limit_with_vbic_td(
            startup::InitialSolutionMode::DcOperatingPoint,
            true,
            Some(20e-12),
            1e-9,
            10e-9,
            base,
        );
        assert!((late - base).abs() < 1e-18);
    }

    #[test]
    fn test_startup_force_accept_delta_limit_preserves_linearized_seed_relaxation_for_vbic() {
        let base = 5e-2;
        let relaxed = Engine::startup_force_accept_delta_limit_with_vbic_td(
            startup::InitialSolutionMode::LinearizedSeed,
            true,
            Some(20e-12),
            20e-12,
            10e-9,
            base,
        );
        assert!((relaxed - VBIC_STARTUP_RECOVERY_DELTA_V).abs() < 1e-18);
    }

    #[test]
    fn test_startup_timestep_divisors_for_bjt_decks() {
        let (startup_div, min_div) = Engine::startup_timestep_divisors(true, false);
        assert!((startup_div - 50.0).abs() < 1e-12);
        assert!((min_div - 200.0).abs() < 1e-9);
    }

    #[test]
    fn test_startup_timestep_divisors_for_vbic_excess_phase_decks() {
        let (startup_div, min_div) = Engine::startup_timestep_divisors(true, true);
        assert!((startup_div - 1000.0).abs() < 1e-12);
        assert!((min_div - 10_000.0).abs() < 1e-9);
    }

    #[test]
    fn test_startup_timestep_divisors_for_non_bjt_decks() {
        let (startup_div, min_div) = Engine::startup_timestep_divisors(false, false);
        assert!((startup_div - 10.0).abs() < 1e-12);
        assert!((min_div - 1000.0).abs() < 1e-9);
    }

    #[test]
    fn test_ngspice_initial_timestep_uses_tran_print_step_instead_of_tmax() {
        let initial = Engine::ngspice_initial_timestep(2e-10, Some(1e-9), 10e-9);
        assert!((initial - 2e-13).abs() < 1e-24);
    }

    #[test]
    fn test_ngspice_initial_timestep_falls_back_to_max_step_without_tran_hint() {
        let initial = Engine::ngspice_initial_timestep(1e-6, None, 20e-9);
        assert!((initial - 1e-9).abs() < 1e-21);
    }

    #[test]
    fn test_ngspice_initial_timestep_ignores_nonphysical_tran_hint() {
        let initial = Engine::ngspice_initial_timestep(1e-6, Some(-1.0), 20e-9);
        assert!((initial - 1e-9).abs() < 1e-21);
    }

    #[test]
    fn test_startup_practical_min_timestep_uses_tran_step_floor_for_bjt_decks() {
        let (_, min_div) = Engine::startup_timestep_divisors(true, false);
        let practical_min =
            Engine::startup_practical_min_timestep(true, false, 10e-9, min_div, Some(1e-9));
        assert!((practical_min - 5e-11).abs() < 1e-20);
    }

    #[test]
    fn test_startup_practical_min_timestep_uses_finer_vbic_floor() {
        let (_, min_div) = Engine::startup_timestep_divisors(true, true);
        let practical_min =
            Engine::startup_practical_min_timestep(true, true, 10e-9, min_div, Some(1e-9));
        assert!((practical_min - 1e-11).abs() < 1e-21);
    }

    #[test]
    fn test_startup_practical_min_timestep_keeps_divisor_floor_without_step_hint() {
        let (_, min_div) = Engine::startup_timestep_divisors(true, false);
        let practical_min =
            Engine::startup_practical_min_timestep(true, false, 10e-9, min_div, None);
        assert!((practical_min - 5e-11).abs() < 1e-21);
    }

    #[test]
    fn test_startup_practical_min_timestep_keeps_vbic_divisor_floor_without_step_hint() {
        let (_, min_div) = Engine::startup_timestep_divisors(true, true);
        let practical_min =
            Engine::startup_practical_min_timestep(true, true, 10e-9, min_div, None);
        assert!((practical_min - 1e-12).abs() < 1e-24);
    }

    #[test]
    fn test_startup_practical_min_timestep_limits_vbic_floor_by_smallest_delay() {
        let (_, min_div) = Engine::startup_timestep_divisors(true, true);
        let practical_min = Engine::startup_practical_min_timestep_with_vbic_td(
            true,
            true,
            10e-9,
            min_div,
            Some(1e-9),
            Some(20e-12),
        );
        assert!((practical_min - 1e-12).abs() < 1e-24);
    }

    #[test]
    fn test_startup_practical_min_timestep_keeps_existing_vbic_floor_for_slower_delay() {
        let (_, min_div) = Engine::startup_timestep_divisors(true, true);
        let practical_min = Engine::startup_practical_min_timestep_with_vbic_td(
            true,
            true,
            10e-9,
            min_div,
            Some(1e-9),
            Some(400e-12),
        );
        assert!((practical_min - 1e-11).abs() < 1e-21);
    }

    #[test]
    fn test_startup_practical_min_timestep_ignores_step_hint_for_non_bjt_decks() {
        let (_, min_div) = Engine::startup_timestep_divisors(false, false);
        let practical_min =
            Engine::startup_practical_min_timestep(false, false, 10e-9, min_div, Some(1e-9));
        assert!((practical_min - 1e-11).abs() < 1e-21);
    }

    #[test]
    fn test_ngspice_hard_min_timestep_allows_retry_below_preferred_floor() {
        let hard_min = Engine::ngspice_hard_min_timestep(10e-9, 1e-12);
        assert!((hard_min - 1e-19).abs() < 1e-31);
    }

    #[test]
    fn test_ngspice_hard_min_timestep_honors_smaller_user_floor() {
        let hard_min = Engine::ngspice_hard_min_timestep(10e-9, 1e-21);
        assert!((hard_min - 1e-21).abs() < 1e-33);
    }

    #[test]
    fn test_legacy_bjt_startup_retry_floor_uses_practical_floor_when_initial_step_is_larger() {
        let floor =
            Engine::legacy_bjt_startup_retry_floor(true, false, 1e-12, 2e-9, 0.0, 2e-10, 1e-10);
        assert_eq!(floor, Some(1e-10));
    }

    #[test]
    fn test_legacy_bjt_startup_retry_floor_respects_smaller_initial_step() {
        let floor =
            Engine::legacy_bjt_startup_retry_floor(true, false, 1e-12, 20e-12, 0.0, 1e-12, 20e-12);
        assert_eq!(floor, Some(1e-12));
    }

    #[test]
    fn test_legacy_bjt_startup_retry_floor_uses_half_floor_for_active_source() {
        let floor = Engine::legacy_bjt_startup_retry_floor(
            true,
            false,
            1e-12,
            2e-9,
            SOURCE_ACTIVE_DELTA,
            2e-10,
            1e-10,
        );
        assert_eq!(floor, Some(5e-11));
    }

    #[test]
    fn test_startup_source_activity_delta_for_retry_floor_looks_across_retry_floor() {
        let mut circuit = crate::circuit::Circuit::new();
        let input = circuit.get_or_create_node("in");
        circuit.voltage_sources.add_with_ac_and_spec(
            "VIN".to_string(),
            input,
            0,
            1,
            0.0,
            0.0,
            0.0,
            Some(crate::netlist::SourceSpec::Pulse {
                v1: 0.0,
                v2: 5.0,
                delay: 2e-9,
                rise: 2e-9,
                fall: 2e-9,
                width: 80e-9,
                period: 200e-9,
            }),
        );
        circuit.voltage_sources.set_transient_context(2e-9, 200e-9);

        let delta = Engine::startup_source_activity_delta_for_retry_floor(
            &circuit, 2.008e-9, 2e-12, 200e-9, 2e-10, 1e-10,
        );
        assert!(
            delta > SOURCE_ACTIVE_DELTA,
            "expected pulse activity over retry-floor horizon, got {:.6e}",
            delta
        );

        let floor = Engine::legacy_bjt_startup_retry_floor(
            true, false, 2.010e-9, 2e-9, delta, 2e-10, 1e-10,
        );
        assert_eq!(floor, Some(5e-11));
    }

    #[test]
    fn test_apply_retry_timestep_floor_prevents_quiet_bjt_delmin_collapse() {
        let retry_dt = Engine::apply_retry_timestep_floor(2e-22, Some(1e-12), 2e-9);
        assert!((retry_dt - 1e-12).abs() < 1e-24);
    }

    #[test]
    fn test_recover_timestep_after_accepted_step_caps_growth_below_generic_doubling() {
        let dt = 2e-13;
        let mut timestep = TimestepController::new_with_preferred_min(dt, 1e-19, 1e-12, 10e-9);
        let lte_estimator = LteEstimator::with_tolerances(1e-3, 1e-6);

        Engine::recover_timestep_after_accepted_step(
            &mut timestep,
            &lte_estimator,
            &[0.0],
            dt,
            10e-9,
            false,
            0.0,
            Some(2.0),
        );

        assert!(
            (timestep.dt() - 3e-13).abs() < 1e-24,
            "expected accepted-step recovery to cap growth at 1.5x instead of generic doubling, got {:.6e}",
            timestep.dt()
        );
    }

    #[test]
    fn test_recover_timestep_after_accepted_step_stays_flat_when_source_is_active() {
        let dt = 1e-9;
        let mut timestep = TimestepController::new_with_preferred_min(dt, 1e-19, 1e-12, 10e-9);
        let lte_estimator = LteEstimator::with_tolerances(1e-3, 1e-6);

        Engine::recover_timestep_after_accepted_step(
            &mut timestep,
            &lte_estimator,
            &[0.0],
            dt,
            10e-9,
            false,
            SOURCE_ACTIVE_DELTA * 4.0,
            Some(2.0),
        );

        assert!(
            (timestep.dt() - dt).abs() < 1e-24,
            "expected active-source accepted-step recovery to hold dt instead of growing, got {:.6e}",
            timestep.dt()
        );
    }

    #[test]
    #[ignore]
    fn debug_vbic_diffamp_step_control_replay_200ps() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let focused_source = source
            .replace(".TRAN 1n 1u 0 10n", ".TRAN 1n 2e-10 0 10n")
            .replace(".AC DEC 25 100k 1G\n", "")
            .replace(".print ac v(e1_p)\n", "");
        let netlist = crate::Netlist::parse(&focused_source).expect("parse focused diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.min_timestep = 1e-12;
        let engine = Engine::new(config);
        let result = engine
            .run_tran(&netlist, 2e-10, 10e-9)
            .expect("focused diffamp transient");

        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");
        let num_nodes = circuit.num_nodes();
        let num_branches = circuit.num_branches();
        let e1_p_idx = result
            .node_index_named("E1_P")
            .expect("E1_P node should exist")
            - 1;

        let solution_at = |point_idx: usize| {
            let mut solution = vec![0.0; num_nodes + num_branches];
            for node_idx in 0..num_nodes {
                solution[node_idx] = result.voltages[node_idx][point_idx];
            }
            for branch_idx in 0..num_branches {
                solution[num_nodes + branch_idx] = result.branch_currents[branch_idx][point_idx];
            }
            solution
        };

        let initial_solution = solution_at(0);
        let mut bjt_history = Engine::initialize_bjt_history(&circuit, &initial_solution);
        bjt_history.accepted_dt_prev = 10e-9;
        bjt_history.accepted_dt_prev_prev = 10e-9;
        let mut jfet_history = Engine::initialize_jfet_history(&circuit, &initial_solution);
        let mut mosfet_history = Engine::initialize_mosfet_history(&circuit, &initial_solution);
        let tline_dc_refs = Engine::initialize_tline_history(&mut circuit, &initial_solution, 0.0);
        let coupled_tline_refs =
            Engine::initialize_coupled_tline_history(&mut circuit, &initial_solution, 0.0);
        let mut breakpoints = BreakpointManager::new();
        let mut dynamic_tline_breakpoints_added = 0_usize;
        let mut warned_dynamic_tline_breakpoint_cap = false;
        let mut vbic_snapshot_cache = vec![None; circuit.bjts.devices.len()];
        let mut lte_estimator =
            LteEstimator::with_tolerances(engine.voltage_reltol(), engine.voltage_abstol());
        let mut vbic_charge_lte_estimator = Some(LteEstimator::with_tolerances(
            engine.voltage_reltol(),
            engine.charge_abstol(),
        ));
        let initial_dt = Engine::ngspice_initial_timestep(2e-10, Some(1e-9), 10e-9);
        Engine::record_vbic_truncation_charge_state(
            &mut vbic_charge_lte_estimator,
            &circuit,
            &initial_solution,
            IntegrationMethod::Trapezoidal,
            1,
            initial_dt,
            &bjt_history,
            Some(&vbic_snapshot_cache),
            1,
        );

        let smallest_td = Some(2e-11);
        let preferred_min_dt = 1e-12;
        let mut trap_order = 1_u8;

        for point_idx in 1..result.time.len().min(12) {
            let previous_time = result.time[point_idx - 1];
            let current_time = result.time[point_idx];
            let dt = current_time - previous_time;
            let previous_solution = solution_at(point_idx - 1);
            let solution = solution_at(point_idx);
            let hidden_converged = engine.vbic_excess_phase_device_convergence_met(
                &circuit,
                &previous_solution,
                &solution,
                IntegrationMethod::Trapezoidal,
                trap_order,
                dt,
                &bjt_history,
                &vbic_snapshot_cache,
            );
            let trunc_limit = Engine::vbic_ngspice_truncation_limit(
                &circuit,
                &solution,
                IntegrationMethod::Trapezoidal,
                trap_order,
                dt,
                &bjt_history,
                &vbic_snapshot_cache,
                engine.voltage_abstol(),
                engine.voltage_reltol(),
                engine.current_abstol(),
                engine.charge_abstol(),
                NGSPICE_DEFAULT_TRTOL,
            );
            let active_vbic_charge_lte = if Engine::should_use_vbic_charge_lte_estimator(
                true,
                current_time,
                10e-9,
                smallest_td,
                dt,
                preferred_min_dt,
            ) {
                vbic_charge_lte_estimator.as_ref()
            } else {
                None
            };
            let (lte, lte_accept, uses_charge_lte) = Engine::estimate_transient_lte(
                &circuit,
                &solution,
                IntegrationMethod::Trapezoidal,
                trap_order,
                dt,
                false,
                &bjt_history,
                &lte_estimator,
                active_vbic_charge_lte,
                Some(&vbic_snapshot_cache),
                engine.voltage_abstol(),
                engine.voltage_reltol(),
            );
            eprintln!(
                "replay step[{point_idx}] t={current_time:.12e}s dt={dt:.12e} order={} v(e1_p)={:.9e} hidden_converged={} trunc_limit={:?} retry={} lte={:.6e} lte_accept={} uses_charge_lte={}",
                trap_order,
                result.voltages[e1_p_idx][point_idx],
                hidden_converged,
                trunc_limit,
                trunc_limit
                    .map(|limit| Engine::should_retry_ngspice_charge_truncation(limit, dt))
                    .unwrap_or(false),
                lte,
                lte_accept,
                uses_charge_lte
            );

            lte_estimator.record(&solution, dt);
            let accepted_method_order = if trap_order <= 1 { 1 } else { 2 };
            lte_estimator.set_method_order(accepted_method_order);
            Engine::record_vbic_truncation_charge_state(
                &mut vbic_charge_lte_estimator,
                &circuit,
                &solution,
                IntegrationMethod::Trapezoidal,
                trap_order,
                dt,
                &bjt_history,
                Some(&vbic_snapshot_cache),
                accepted_method_order,
            );
            Engine::update_reactive_history(
                &mut circuit,
                &solution,
                previous_time,
                dt,
                IntegrationMethod::Trapezoidal,
                trap_order,
                &mut bjt_history,
                &mut jfet_history,
                &mut mosfet_history,
                Some(&vbic_snapshot_cache),
                false,
                &tline_dc_refs,
                &coupled_tline_refs,
                &mut breakpoints,
                2e-10,
                engine.voltage_reltol(),
                engine.voltage_abstol(),
                &mut dynamic_tline_breakpoints_added,
                &mut warned_dynamic_tline_breakpoint_cap,
            );

            let should_promote = if Engine::should_hold_vbic_excess_phase_first_order(
                true,
                current_time,
                10e-9,
                smallest_td,
            ) {
                false
            } else {
                Engine::should_promote_trapezoidal_order(
                    &circuit,
                    &solution,
                    IntegrationMethod::Trapezoidal,
                    dt,
                    false,
                    &bjt_history,
                    &jfet_history,
                    &mosfet_history,
                    &lte_estimator,
                    vbic_charge_lte_estimator.as_ref(),
                    &vbic_snapshot_cache,
                    engine.voltage_abstol(),
                    engine.voltage_reltol(),
                    engine.current_abstol(),
                    engine.charge_abstol(),
                    NGSPICE_DEFAULT_TRTOL,
                )
            };
            let next_order = Engine::next_trapezoidal_order_after_accepted_step(
                trap_order,
                false,
                should_promote,
            );
            eprintln!(
                "  replay post-accept[{point_idx}] should_promote={} next_order={} accepted_dt_prev={:.12e} accepted_dt_prev_prev={:.12e}",
                should_promote,
                next_order,
                bjt_history.accepted_dt_prev,
                bjt_history.accepted_dt_prev_prev
            );
            trap_order = next_order;
        }
    }

    fn vbic_focus_test_bjt() -> (crate::device::Bjt, f64, f64, f64, f64) {
        let netlist = vbic_focus_test_netlist(2e-11);
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build VBIC focus circuit");
        (circuit.bjts.devices[0].clone(), 4.1, 0.75, 0.0, 0.0)
    }

    fn vbic_focus_test_netlist(td: Value) -> crate::Netlist {
        crate::Netlist::parse(&format!(
            "VBIC transient focus\n\
VC C 0 4.1\n\
VB B 0 0.75\n\
Q1 C B 0 N1\n\
.MODEL N1 NPN LEVEL=4\n\
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10\n\
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2\n\
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2\n\
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD={td:.12e}\n\
.end"
        ))
        .expect("parse VBIC focus deck")
    }

    fn vbic_self_heated_focus_test_bjt() -> (crate::device::Bjt, f64, f64, f64, f64) {
        let netlist = crate::Netlist::parse(
            "VBIC transient self-heated focus\n\
VC C 0 4.1\n\
VB B 0 0.75\n\
Q1 C B 0 N1\n\
.MODEL N1 NPN LEVEL=4\n\
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10\n\
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2\n\
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2\n\
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300 SELFT=1\n\
.end",
        )
        .expect("parse VBIC focus deck");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build VBIC focus circuit");
        (circuit.bjts.devices[0].clone(), 4.1, 0.75, 0.0, 0.0)
    }

    fn vbic_self_heated_pnp_diffamp_test_bjt() -> (crate::device::Bjt, f64, f64, f64, f64) {
        let netlist = crate::Netlist::parse(
            "VBIC transient self-heated PNP focus\n\
VS S 0 1.575451\n\
VE E 0 1.94\n\
VB B 0 2.614704\n\
VC C 0 1.575451\n\
Q1 C B E S P1\n\
.MODEL P1 PNP LEVEL=4\n\
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10\n\
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2\n\
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2\n\
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300 SELFT=1\n\
.end",
        )
        .expect("parse self-heated PNP VBIC focus deck");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build self-heated PNP VBIC focus circuit");
        (
            circuit.bjts.devices[0].clone(),
            1.575_451,
            2.614_704,
            1.94,
            1.575_451,
        )
    }

    fn vbic_pnp_diffamp_mirror_test_bjt() -> (crate::device::Bjt, f64, f64, f64, f64) {
        let netlist = crate::Netlist::parse(
            "VBIC transient PNP diffamp mirror focus\n\
VS S 0 1.575451\n\
VE E 0 1.94\n\
VB B 0 2.614704\n\
VC C 0 1.575451\n\
Q1 C B E S P1\n\
.MODEL P1 PNP LEVEL=4\n\
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10\n\
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2\n\
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2\n\
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300\n\
.end",
        )
        .expect("parse PNP diffamp mirror VBIC focus deck");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build PNP diffamp mirror VBIC focus circuit");
        (
            circuit.bjts.devices[0].clone(),
            1.575_451,
            2.614_704,
            1.94,
            1.575_451,
        )
    }

    fn vbic_pnp_tied_mirror_test_bjt() -> (crate::device::Bjt, f64, f64, f64, f64) {
        let netlist = crate::Netlist::parse(
            "VBIC transient PNP tied mirror focus\n\
VB B 0 2.620627\n\
VE E 0 3.300000\n\
Q1 B B E B P1\n\
.MODEL P1 PNP LEVEL=4\n\
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10\n\
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2\n\
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2\n\
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300\n\
.end",
        )
        .expect("parse PNP tied mirror VBIC focus deck");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build PNP tied mirror VBIC focus circuit");
        (
            circuit.bjts.devices[0].clone(),
            2.620_627,
            2.620_627,
            3.3,
            2.620_627,
        )
    }

    fn vbic_npn_diffamp_input_test_bjt() -> (crate::device::Bjt, f64, f64, f64, f64) {
        let netlist = crate::Netlist::parse(
            "VBIC transient diffamp NPN input focus\n\
VC C 0 2.614704\n\
VB B 0 1.650000\n\
VE E 0 1.011054\n\
Q1 C B E 0 N1\n\
.MODEL N1 NPN LEVEL=4\n\
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10\n\
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2\n\
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2\n\
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300\n\
.end",
        )
        .expect("parse diffamp NPN VBIC focus deck");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp NPN VBIC focus circuit");
        (
            circuit.bjts.devices[0].clone(),
            2.614_704,
            1.65,
            1.011_054,
            0.0,
        )
    }

    fn vbic_npn_diffamp_input_collector_substrate_test_bjt()
    -> (crate::device::Bjt, f64, f64, f64, f64) {
        let netlist = crate::Netlist::parse(
            "VBIC transient diffamp NPN input focus with collector-tied substrate\n\
VC C 0 2.614704\n\
VB B 0 1.650000\n\
VE E 0 1.011054\n\
VS S 0 2.614704\n\
Q1 C B E S N1\n\
.MODEL N1 NPN LEVEL=4\n\
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10\n\
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2\n\
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2\n\
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300\n\
.end",
        )
        .expect("parse diffamp NPN VBIC collector-substrate focus deck");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp NPN VBIC collector-substrate focus circuit");
        (
            circuit.bjts.devices[0].clone(),
            2.614_704,
            1.65,
            1.011_054,
            2.614_704,
        )
    }

    fn vbic_npn_diffamp_input_test_bjt_with_model_patch(
        model_patch: &str,
    ) -> (crate::device::Bjt, f64, f64, f64, f64) {
        let netlist = crate::Netlist::parse(&format!(
            "VBIC transient diffamp NPN input focus\n\
VC C 0 2.614704\n\
VB B 0 1.650000\n\
VE E 0 1.011054\n\
Q1 C B E 0 N1\n\
.MODEL N1 NPN LEVEL=4\n\
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10\n\
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2\n\
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2\n\
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300 {model_patch}\n\
.end"
        ))
        .expect("parse patched diffamp NPN VBIC focus deck");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build patched diffamp NPN VBIC focus circuit");
        (
            circuit.bjts.devices[0].clone(),
            2.614_704,
            1.65,
            1.011_054,
            0.0,
        )
    }

    fn vbic_npn_diffamp_input_test_circuit() -> (crate::circuit::Circuit, f64, f64, f64, f64) {
        let netlist = crate::Netlist::parse(
            "VBIC transient diffamp NPN input focus\n\
VC C 0 2.614704\n\
VB B 0 1.650000\n\
VE E 0 1.011054\n\
Q1 C B E 0 N1\n\
.MODEL N1 NPN LEVEL=4\n\
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10\n\
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2\n\
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2\n\
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300\n\
.end",
        )
        .expect("parse diffamp NPN VBIC focus circuit");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp NPN VBIC focus circuit");
        (circuit, 2.614_704, 1.65, 1.011_054, 0.0)
    }

    fn dense_static_matrix(size: usize) -> crate::solver::StaticMatrix {
        let mut triplets = Vec::with_capacity(size * size);
        for row in 0..size {
            for col in 0..size {
                triplets.push((row, col, 0.0));
            }
        }
        crate::solver::StaticMatrix::from_triplets(size, size, &triplets)
            .expect("build dense static matrix")
    }

    fn vbic_reduced_external_current(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        history_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_prev_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_linear_prev: &VbicPredictorLinearBranchState,
        history_linear_prev_prev: &VbicPredictorLinearBranchState,
        cached_snapshot: Option<BjtChargeSnapshot>,
    ) -> (
        [Value; BJT_EXTERNAL_STATE_DIM],
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        BjtChargeSnapshot,
    ) {
        let snapshot = Engine::resolve_vbic_snapshot_for_external_bias_with_linear_history(
            bjt,
            [vc, vb, ve, vs],
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
            Some(history_prev),
            Some(history_prev_prev),
            Some(history_linear_prev),
            Some(history_linear_prev_prev),
            dt / 2.0,
            cached_snapshot,
            VbicCachedSnapshotReuse::NewtonBypass,
            1e-12,
            1e-9,
        )
        .expect("resolve VBIC transient state");
        let linearization = Engine::assemble_vbic_transient_linearization(
            bjt,
            &snapshot,
            method,
            trap_order,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        )
        .expect("assemble VBIC transient linearization");
        let (y_total, reduced_i_eq) = Engine::vbic_reduce_transient_external_system(&linearization)
            .expect("reduce VBIC transient external system");
        let external = [vc, vb, ve, vs];
        let mut currents = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            currents[row] = -reduced_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                currents[row] += y_total[row][col] * external[col];
            }
        }
        (currents, y_total, snapshot)
    }

    #[allow(clippy::too_many_arguments)]
    fn assert_vbic_dynamic_reduced_jacobian_matches_finite_difference(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        q_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        q_prev_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        cq_prev: &[Value; BJT_DYNAMIC_CHARGE_COUNT],
        history_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_prev_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        label: &str,
    ) {
        let history_linear_prev =
            Engine::vbic_predictor_linear_branch_state(bjt, [vc, vb, ve, vs], *history_prev);
        let history_linear_prev_prev =
            Engine::vbic_predictor_linear_branch_state(bjt, [vc, vb, ve, vs], *history_prev_prev);
        let seed_for = |vc: Value, vb: Value, ve: Value, vs: Value| {
            Engine::vbic_dynamic_internal_seed_from_history_with_linear_history(
                bjt,
                vc,
                vb,
                ve,
                vs,
                Some(history_prev),
                Some(history_prev_prev),
                Some(&history_linear_prev),
                Some(&history_linear_prev_prev),
                dt,
                dt / 2.0,
            )
        };

        let center_seed = seed_for(vc, vb, ve, vs);
        let (_center_currents, center_jacobian, center_snapshot_for_fd) =
            vbic_reduced_external_current(
                bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                history_prev,
                history_prev_prev,
                &history_linear_prev,
                &history_linear_prev_prev,
                None,
            );

        let mut external = [vc, vb, ve, vs];
        let eps = 1e-6;
        for col_idx in 0..BJT_EXTERNAL_STATE_DIM {
            external[col_idx] += eps;
            let plus_seed = seed_for(external[0], external[1], external[2], external[3]);
            let (plus_currents, _, _) = vbic_reduced_external_current(
                bjt,
                external[0],
                external[1],
                external[2],
                external[3],
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                history_prev,
                history_prev_prev,
                &history_linear_prev,
                &history_linear_prev_prev,
                Some(center_snapshot_for_fd),
            );

            external[col_idx] -= 2.0 * eps;
            let minus_seed = seed_for(external[0], external[1], external[2], external[3]);
            let (minus_currents, _, _) = vbic_reduced_external_current(
                bjt,
                external[0],
                external[1],
                external[2],
                external[3],
                method,
                trap_order,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
                history_prev,
                history_prev_prev,
                &history_linear_prev,
                &history_linear_prev_prev,
                Some(center_snapshot_for_fd),
            );
            external[col_idx] += eps;

            for row_idx in 0..BJT_EXTERNAL_STATE_DIM {
                let analytical = center_jacobian[row_idx][col_idx];
                let numerical = (plus_currents[row_idx] - minus_currents[row_idx]) / (2.0 * eps);
                let scale = analytical.abs().max(numerical.abs()).max(1e-8);
                let rel_err = (analytical - numerical).abs() / scale;
                if rel_err >= 5e-2 {
                    let mut plus_external = [vc, vb, ve, vs];
                    plus_external[col_idx] += eps;
                    let mut minus_external = [vc, vb, ve, vs];
                    minus_external[col_idx] -= eps;
                    let center_snapshot = Engine::solve_vbic_dynamic_snapshot(
                        bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        Some(&center_seed),
                    )
                    .expect("solve center VBIC transient state for diagnostics");
                    let plus_snapshot = Engine::solve_vbic_dynamic_snapshot(
                        bjt,
                        plus_external[0],
                        plus_external[1],
                        plus_external[2],
                        plus_external[3],
                        method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        Some(&plus_seed),
                    )
                    .expect("solve plus VBIC transient state for diagnostics");
                    let minus_snapshot = Engine::solve_vbic_dynamic_snapshot(
                        bjt,
                        minus_external[0],
                        minus_external[1],
                        minus_external[2],
                        minus_external[3],
                        method,
                        trap_order,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                        Some(&minus_seed),
                    )
                    .expect("solve minus VBIC transient state for diagnostics");
                    let center_linearization = &center_snapshot.1;
                    let (lu_internal, pivots_internal) = Engine::lu_decompose_small_dense_real(
                        &center_linearization.g_ii,
                        BJT_INTERNAL_STATE_DIM,
                    )
                    .expect("factor center VBIC internal matrix for diagnostics");
                    let mut rhs_internal = [0.0; BJT_INTERNAL_STATE_DIM];
                    for row in 0..BJT_INTERNAL_STATE_DIM {
                        rhs_internal[row] = -center_linearization.g_ie[row][col_idx];
                    }
                    let analytical_internal_sensitivity = Engine::lu_solve_small_dense_real(
                        &lu_internal,
                        &pivots_internal,
                        &rhs_internal,
                        BJT_INTERNAL_STATE_DIM,
                    )
                    .expect("solve center VBIC internal sensitivity for diagnostics");
                    let mut numerical_internal_sensitivity = [0.0; BJT_INTERNAL_STATE_DIM];
                    for idx in 0..BJT_INTERNAL_STATE_DIM {
                        numerical_internal_sensitivity[idx] =
                            (plus_snapshot.0.reduction.internal_voltages[idx]
                                - minus_snapshot.0.reduction.internal_voltages[idx])
                                / (2.0 * eps);
                    }
                    let evaluate_internal_residual =
                        |external: [Value; BJT_EXTERNAL_STATE_DIM],
                         internal: [Value; BJT_INTERNAL_STATE_DIM]| {
                            let mut snapshot = bjt.charge_snapshot_for_dynamic_state(
                                external[0],
                                external[1],
                                external[2],
                                external[3],
                                internal,
                            );
                            Engine::rebalance_vbic_dynamic_thermal_state(
                                bjt,
                                external[0],
                                external[1],
                                external[2],
                                external[3],
                                method,
                                trap_order,
                                dt,
                                q_prev,
                                q_prev_prev,
                                cq_prev,
                                &mut snapshot,
                            );
                            let linearization = Engine::assemble_vbic_transient_linearization(
                                bjt,
                                &snapshot,
                                method,
                                trap_order,
                                dt,
                                q_prev,
                                q_prev_prev,
                                cq_prev,
                            )
                            .expect(
                                "assemble VBIC transient linearization for internal diagnostics",
                            );
                            Engine::vbic_internal_equation_residual(
                                &linearization,
                                &snapshot.reduction.external_voltages,
                                &snapshot.reduction.internal_voltages,
                            )
                        };
                    let mut numerical_internal_jacobian =
                        [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
                    let center_internal = center_snapshot.0.reduction.internal_voltages;
                    let center_external = center_snapshot.0.reduction.external_voltages;
                    let center_residual = Engine::vbic_internal_equation_residual(
                        center_linearization,
                        &center_snapshot.0.reduction.external_voltages,
                        &center_snapshot.0.reduction.internal_voltages,
                    );
                    for internal_col in 0..BJT_INTERNAL_STATE_DIM {
                        let base_value = center_internal[internal_col];
                        let step = match internal_col {
                            BJT_DELAY_XF1_STATE_INDEX | BJT_DELAY_XF2_STATE_INDEX => {
                                (base_value.abs() * 1e-3).max(1e-9)
                            }
                            BJT_THERMAL_STATE_INDEX => (base_value.abs() * 1e-4).max(1e-6),
                            _ => (base_value.abs() * 1e-6).max(1e-7),
                        };
                        let mut plus_internal = center_internal;
                        plus_internal[internal_col] = base_value + step;
                        if internal_col == BJT_THERMAL_STATE_INDEX {
                            plus_internal[internal_col] =
                                plus_internal[internal_col].max(bjt.minimum_thermal_rise());
                        }
                        let plus_residual =
                            evaluate_internal_residual(center_external, plus_internal);
                        let use_central = internal_col != BJT_THERMAL_STATE_INDEX
                            || base_value - step >= bjt.minimum_thermal_rise();
                        if use_central {
                            let mut minus_internal = center_internal;
                            minus_internal[internal_col] = base_value - step;
                            if internal_col == BJT_THERMAL_STATE_INDEX {
                                minus_internal[internal_col] =
                                    minus_internal[internal_col].max(bjt.minimum_thermal_rise());
                            }
                            let minus_residual =
                                evaluate_internal_residual(center_external, minus_internal);
                            let denom = plus_internal[internal_col] - minus_internal[internal_col];
                            for internal_row in 0..BJT_INTERNAL_STATE_DIM {
                                numerical_internal_jacobian[internal_row][internal_col] =
                                    (plus_residual[internal_row] - minus_residual[internal_row])
                                        / denom;
                            }
                        } else {
                            let denom = plus_internal[internal_col] - center_internal[internal_col];
                            for internal_row in 0..BJT_INTERNAL_STATE_DIM {
                                numerical_internal_jacobian[internal_row][internal_col] =
                                    (plus_residual[internal_row] - center_residual[internal_row])
                                        / denom;
                            }
                        }
                    }
                    let mut worst_internal_jacobian_row = 0;
                    let mut worst_internal_jacobian_col = 0;
                    let mut worst_internal_jacobian_rel_err = 0.0_f64;
                    for internal_row in 0..BJT_INTERNAL_STATE_DIM {
                        for internal_col in 0..BJT_INTERNAL_STATE_DIM {
                            let analytical_value =
                                center_linearization.g_ii[internal_row][internal_col];
                            let numerical_value =
                                numerical_internal_jacobian[internal_row][internal_col];
                            let scale = analytical_value.abs().max(numerical_value.abs()).max(1e-8);
                            let rel_err = (analytical_value - numerical_value).abs() / scale;
                            if rel_err > worst_internal_jacobian_rel_err {
                                worst_internal_jacobian_rel_err = rel_err;
                                worst_internal_jacobian_row = internal_row;
                                worst_internal_jacobian_col = internal_col;
                            }
                        }
                    }
                    let raw_newton_internal = Engine::solve_vbic_internal_state_from_linearization(
                        center_linearization,
                        &center_external,
                    )
                    .expect("solve raw VBIC internal Newton step for diagnostics");
                    let limited_raw_newton_internal = bjt
                        .limit_vbic_dynamic_internal_state_to_previous(
                            raw_newton_internal,
                            center_internal,
                        );
                    let raw_newton_residual =
                        evaluate_internal_residual(center_external, raw_newton_internal);
                    let limited_raw_newton_residual =
                        evaluate_internal_residual(center_external, limited_raw_newton_internal);
                    let raw_newton_residual_norm = raw_newton_residual
                        .into_iter()
                        .fold(0.0_f64, |max_norm, value| max_norm.max(value.abs()));
                    let limited_raw_newton_residual_norm = limited_raw_newton_residual
                        .into_iter()
                        .fold(0.0_f64, |max_norm, value| max_norm.max(value.abs()));
                    let raw_newton_max_delta = raw_newton_internal
                        .iter()
                        .zip(center_internal.iter())
                        .map(|(next, current)| (next - current).abs())
                        .fold(0.0_f64, |max_delta, value| max_delta.max(value));
                    let analytical_direct = center_linearization.g_ee[row_idx][col_idx];
                    let analytical_indirect = center_linearization.g_ei[row_idx]
                        .iter()
                        .zip(analytical_internal_sensitivity.iter())
                        .map(|(g_ei, sensitivity)| g_ei * sensitivity)
                        .sum::<Value>();
                    let center_residual_norm = Engine::vbic_internal_equation_residual_norm(
                        center_linearization,
                        &center_snapshot.0.reduction.external_voltages,
                        &center_snapshot.0.reduction.internal_voltages,
                    );
                    let plus_residual_norm = Engine::vbic_internal_equation_residual_norm(
                        &plus_snapshot.1,
                        &plus_snapshot.0.reduction.external_voltages,
                        &plus_snapshot.0.reduction.internal_voltages,
                    );
                    let minus_residual_norm = Engine::vbic_internal_equation_residual_norm(
                        &minus_snapshot.1,
                        &minus_snapshot.0.reduction.external_voltages,
                        &minus_snapshot.0.reduction.internal_voltages,
                    );
                    panic!(
                        "{label} dynamic reduced Jacobian mismatch row={row_idx} col={col_idx}: analytical={analytical:.12e} numerical={numerical:.12e} rel_err={rel_err:.3e}, analytical_direct={analytical_direct:.12e}, analytical_indirect={analytical_indirect:.12e}, center_residual_norm={center_residual_norm:.12e}, plus_residual_norm={plus_residual_norm:.12e}, minus_residual_norm={minus_residual_norm:.12e}, center_currents={:?}, plus_currents={:?}, minus_currents={:?}, center_seed={:?}, plus_seed={:?}, minus_seed={:?}, center_internal={:?}, plus_internal={:?}, minus_internal={:?}, center_residual={:?}, analytical_internal_sensitivity={:?}, numerical_internal_sensitivity={:?}, worst_internal_jacobian=({worst_internal_jacobian_row},{worst_internal_jacobian_col}) rel_err={worst_internal_jacobian_rel_err:.3e}, analytical_internal_jacobian_row={:?}, numerical_internal_jacobian_row={:?}, raw_newton_max_delta={raw_newton_max_delta:.12e}, raw_newton_residual_norm={raw_newton_residual_norm:.12e}, limited_raw_newton_residual_norm={limited_raw_newton_residual_norm:.12e}, raw_newton_internal={:?}, limited_raw_newton_internal={:?}",
                        _center_currents,
                        plus_currents,
                        minus_currents,
                        center_seed,
                        plus_seed,
                        minus_seed,
                        center_snapshot.0.reduction.internal_voltages,
                        plus_snapshot.0.reduction.internal_voltages,
                        minus_snapshot.0.reduction.internal_voltages,
                        center_residual,
                        analytical_internal_sensitivity,
                        numerical_internal_sensitivity,
                        center_linearization.g_ii[worst_internal_jacobian_row],
                        numerical_internal_jacobian[worst_internal_jacobian_row],
                        raw_newton_internal,
                        limited_raw_newton_internal,
                    );
                }
                assert!(
                    rel_err < 5e-2,
                    "{label} dynamic reduced Jacobian mismatch row={row_idx} col={col_idx}: analytical={analytical:.12e} numerical={numerical:.12e} rel_err={rel_err:.3e}"
                );
            }
        }
    }

    fn bjt_node_indexed_voltage_vector(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> Vec<Value> {
        let nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        let values = [vc, vb, ve, vs];
        let len = nodes.iter().copied().max().unwrap_or(0);
        let mut voltages = vec![0.0; len];
        let mut assigned = vec![false; len];
        for (&node, &value) in nodes.iter().zip(values.iter()) {
            if node == 0 {
                continue;
            }
            let slot = &mut voltages[node - 1];
            let was_assigned = assigned[node - 1];
            assert!(
                !was_assigned || (*slot - value).abs() < 1e-15,
                "conflicting voltage assignments for shared BJT node {node}: existing={:.16e}, new={value:.16e}",
                *slot,
            );
            *slot = value;
            assigned[node - 1] = true;
        }
        voltages
    }

    fn bjt_update_for_external_bias(
        bjt: &mut crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) {
        let voltages = bjt_node_indexed_voltage_vector(bjt, vc, vb, ve, vs);
        bjt.update(&voltages);
    }

    fn bjt_direct_stamped_external_system(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (
        [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        [Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        #[derive(Default)]
        struct CaptureMatrix {
            entries: std::collections::HashMap<(usize, usize), Value>,
            rhs: std::collections::HashMap<usize, Value>,
            rhs_calls: Vec<(usize, Value)>,
        }

        impl CaptureMatrix {
            fn g(&self, row: usize, col: usize) -> Value {
                *self.entries.get(&(row, col)).unwrap_or(&0.0)
            }

            fn i(&self, node: usize) -> Value {
                *self.rhs.get(&node).unwrap_or(&0.0)
            }
        }

        impl MatrixStamper for CaptureMatrix {
            fn stamp(&mut self, row: usize, col: usize, value: Value) {
                *self.entries.entry((row, col)).or_insert(0.0) += value;
            }

            fn stamp_rhs(&mut self, index: usize, value: Value) {
                *self.rhs.entry(index).or_insert(0.0) += value;
                self.rhs_calls.push((index, value));
            }
        }

        let mut remapped = bjt.clone();
        remapped.node_collector = 1;
        remapped.node_base = 2;
        remapped.node_emitter = 3;
        remapped.node_substrate = 4;

        let mut stamped = CaptureMatrix::default();
        let mut rhs_scratch = [0.0; BJT_EXTERNAL_STATE_DIM];
        remapped.stamp_nonlinear(&[vc, vb, ve, vs], &mut stamped, &mut rhs_scratch);

        let mut g = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut rhs = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            rhs[row] = stamped.i(row + 1);
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                g[row][col] = stamped.g(row + 1, col + 1);
            }
        }
        (g, rhs)
    }

    #[test]
    fn test_vbic_pnp_self_heated_dynamic_snapshot_satisfies_reduced_internal_solve_at_diffamp_bias()
    {
        let (mut bjt, vc, vb, ve, vs) = vbic_self_heated_pnp_diffamp_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;
        let seed_internal = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            dt,
            dt / 2.0,
        );

        let (solved_snapshot, solved_linearization, _) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&seed_internal),
        )
        .expect("solve self-heated PNP VBIC transient state");
        let solved_internal = Engine::solve_vbic_internal_state_from_linearization(
            &solved_linearization,
            &solved_snapshot.reduction.external_voltages,
        )
        .expect("solve reduced PNP VBIC internal state");

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            let delta =
                (solved_snapshot.reduction.internal_voltages[idx] - solved_internal[idx]).abs();
            assert!(
                delta < 5e-10,
                "expected PNP diffamp snapshot to satisfy reduced internal solve at index {idx}; snapshot={:.16e}, solved={:.16e}, delta={delta:.3e}",
                solved_snapshot.reduction.internal_voltages[idx],
                solved_internal[idx]
            );
        }

        let solved_residual = Engine::vbic_transient_thermal_residual_and_derivative(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            solved_snapshot.reduction.internal_voltages,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .0
        .abs();
        assert!(
            solved_residual < 1e-9,
            "expected PNP diffamp transient thermal residual to converge, got {solved_residual:.3e}"
        );
    }

    #[test]
    fn test_solve_vbic_dynamic_snapshot_solves_internal_state_after_history_seed() {
        let (bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        let seed_internal = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            dt,
            dt / 2.0,
        );
        let history_seed = bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, seed_internal);
        let seed_linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &history_seed,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("seed transient linearization");
        let (solved_snapshot, solved_linearization, solved_static_g) =
            Engine::solve_vbic_dynamic_snapshot(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&seed_internal),
            )
            .expect("solve VBIC transient state");

        let rebuilt_linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &solved_snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("rebuild solved transient linearization");
        let solved_internal = Engine::solve_vbic_internal_state_from_linearization(
            &solved_linearization,
            &solved_snapshot.reduction.external_voltages,
        )
        .expect("solve reduced VBIC internal state");

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            let delta =
                (solved_snapshot.reduction.internal_voltages[idx] - solved_internal[idx]).abs();
            assert!(
                delta < 5e-12,
                "expected solved snapshot to satisfy reduced internal solve at index {idx}; snapshot={:.16e}, solved={:.16e}, delta={delta:.3e}",
                solved_snapshot.reduction.internal_voltages[idx],
                solved_internal[idx]
            );
        }
        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                assert!(
                    (solved_linearization.g_ii[row][col] - rebuilt_linearization.g_ii[row][col])
                        .abs()
                        < 1e-18,
                    "expected solved transient internal matrix to match rebuilt assembly at ({row}, {col})"
                );
            }
        }
        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                assert!(
                    (solved_linearization.g_ie[row][col] - rebuilt_linearization.g_ie[row][col])
                        .abs()
                        < 1e-18,
                    "expected solved transient coupling matrix g_ie to match rebuilt assembly at ({row}, {col})"
                );
            }
        }
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                assert!(
                    (solved_linearization.g_ei[row][col] - rebuilt_linearization.g_ei[row][col])
                        .abs()
                        < 1e-18,
                    "expected solved transient coupling matrix g_ei to match rebuilt assembly at ({row}, {col})"
                );
            }
        }
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                assert!(
                    (solved_linearization.g_ee[row][col] - rebuilt_linearization.g_ee[row][col])
                        .abs()
                        < 1e-18,
                    "expected solved transient external matrix to match rebuilt assembly at ({row}, {col})"
                );
                assert!(
                    (solved_static_g[row][col] - solved_snapshot.reduction.g_reduced[row][col])
                        .abs()
                        < 1e-18,
                    "expected solved static reduced conductance to match rebuilt snapshot at ({row}, {col})"
                );
            }
        }
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (solved_linearization.z_i[idx] - rebuilt_linearization.z_i[idx]).abs() < 1e-18,
                "expected solved transient internal source vector to match rebuilt assembly at index {idx}"
            );
        }
        for idx in 0..BJT_EXTERNAL_STATE_DIM {
            assert!(
                (solved_linearization.z_e[idx] - rebuilt_linearization.z_e[idx]).abs() < 1e-18,
                "expected solved transient external source vector to match rebuilt assembly at index {idx}"
            );
        }
        let seed_internal_delta = solved_snapshot
            .reduction
            .internal_voltages
            .iter()
            .zip(history_seed.reduction.internal_voltages.iter())
            .map(|(solved, seeded)| (solved - seeded).abs())
            .fold(0.0, Value::max);
        assert!(
            seed_internal_delta > 1e-12,
            "expected reduced internal-state solve to move away from the raw history seed; max delta was {seed_internal_delta:.3e}"
        );
        let seed_matrix_delta = (0..BJT_INTERNAL_STATE_DIM)
            .flat_map(|row| {
                (0..BJT_INTERNAL_STATE_DIM).map(move |col| {
                    (solved_linearization.g_ii[row][col] - seed_linearization.g_ii[row][col]).abs()
                })
            })
            .fold(0.0, Value::max);
        assert!(
            seed_matrix_delta > 1e-12,
            "expected solved transient internal matrix to differ from the raw seed assembly; max delta was {seed_matrix_delta:.3e}"
        );
    }

    #[test]
    fn test_vbic_dynamic_companion_stamp_adds_only_transient_delta_rhs() {
        use std::collections::HashMap;

        #[derive(Default)]
        struct CaptureMatrix {
            entries: HashMap<(usize, usize), Value>,
            rhs: HashMap<usize, Value>,
        }

        impl CaptureMatrix {
            fn g(&self, row: usize, col: usize) -> Value {
                *self.entries.get(&(row, col)).unwrap_or(&0.0)
            }

            fn i(&self, node: usize) -> Value {
                *self.rhs.get(&node).unwrap_or(&0.0)
            }
        }

        impl MatrixStamper for CaptureMatrix {
            fn stamp(&mut self, row: usize, col: usize, value: Value) {
                *self.entries.entry((row, col)).or_insert(0.0) += value;
            }

            fn stamp_rhs(&mut self, index: usize, value: Value) {
                *self.rhs.entry(index).or_insert(0.0) += value;
            }
        }

        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let (snapshot, linearization, _snapshot_static_g) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            None,
        )
        .expect("solve VBIC transient state");
        let (y_total, reduced_i_eq) = Engine::vbic_reduce_transient_external_system(&linearization)
            .expect("reduce VBIC transient external system");
        let (base_static_g, base_static_i_eq) = Engine::vbic_static_stamped_external_system(
            &bjt,
            &snapshot.reduction.external_voltages,
        );
        let (direct_row_g, direct_row_rhs) =
            bjt_direct_stamped_external_system(&bjt, vc, vb, ve, vs);

        for row in 0..BJT_EXTERNAL_STATE_DIM {
            assert!(
                (direct_row_rhs[row] - base_static_i_eq[row]).abs() < 1e-12,
                "expected direct unique-node static rhs row {row} to match reduced static rhs, actual={:.12e}, expected={:.12e}",
                direct_row_rhs[row],
                base_static_i_eq[row],
            );
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                assert!(
                    (direct_row_g[row][col] - base_static_g[row][col]).abs() < 1e-12,
                    "expected direct unique-node static conductance ({row}, {col}) to match reduced static conductance, actual={:.12e}, expected={:.12e}",
                    direct_row_g[row][col],
                    base_static_g[row][col],
                );
            }
        }

        let mut delta = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut delta_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            delta_i_eq[row] = reduced_i_eq[row] - base_static_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                delta[row][col] = y_total[row][col] - base_static_g[row][col];
            }
        }
        let delta_rhs_norm = delta_i_eq
            .iter()
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        assert!(
            delta_rhs_norm > 1e-15,
            "expected dynamic VBIC companion to contribute a non-zero rhs delta"
        );

        let nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        let voltage_vector = bjt_node_indexed_voltage_vector(&bjt, vc, vb, ve, vs);
        let mut stamped = CaptureMatrix::default();
        let mut rhs_scratch = [0.0; BJT_EXTERNAL_STATE_DIM];
        bjt.stamp_nonlinear(&voltage_vector, &mut stamped, &mut rhs_scratch);
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let node_row = nodes[row];
            if node_row == 0 {
                continue;
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let node_col = nodes[col];
                if node_col > 0 {
                    stamped.stamp(node_row, node_col, delta[row][col]);
                }
            }
            stamped.stamp_rhs(node_row, delta_i_eq[row]);
        }

        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let node_row = nodes[row];
            if node_row == 0 {
                continue;
            }
            assert!(
                (stamped.i(node_row) - reduced_i_eq[row]).abs() < 1e-12,
                "expected combined static + dynamic rhs at row {row} to match the reduced transient rhs"
            );
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let node_col = nodes[col];
                if node_col == 0 {
                    continue;
                }
                assert!(
                    (stamped.g(node_row, node_col) - y_total[row][col]).abs() < 1e-12,
                    "expected combined static + dynamic conductance at ({row}, {col}) to match the reduced transient system"
                );
            }
        }
    }

    #[test]
    fn test_vbic_pnp_self_heated_dynamic_reduced_jacobian_matches_finite_difference_at_diffamp_bias()
     {
        let (mut bjt, vc, vb, ve, vs) = vbic_self_heated_pnp_diffamp_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;

        assert_vbic_dynamic_reduced_jacobian_matches_finite_difference(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            &history_prev,
            &history_prev_prev,
            "PNP self-heated diffamp",
        );
    }

    #[test]
    fn test_vbic_npn_diffamp_dynamic_reduced_jacobian_matches_finite_difference() {
        let (mut bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;

        assert_vbic_dynamic_reduced_jacobian_matches_finite_difference(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            &history_prev,
            &history_prev_prev,
            "NPN diffamp",
        );
    }

    #[test]
    fn test_vbic_npn_diffamp_resolved_snapshot_hidden_residual_is_strictly_converged() {
        let (mut bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;

        let history_linear_prev =
            Engine::vbic_predictor_linear_branch_state(&bjt, [vc, vb, ve, vs], history_prev);
        let history_linear_prev_prev =
            Engine::vbic_predictor_linear_branch_state(&bjt, [vc, vb, ve, vs], history_prev_prev);

        let snapshot = Engine::resolve_vbic_snapshot_for_external_bias_with_linear_history(
            &bjt,
            [vc, vb, ve, vs],
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&history_prev),
            Some(&history_prev_prev),
            Some(&history_linear_prev),
            Some(&history_linear_prev_prev),
            dt / 2.0,
            None,
            VbicCachedSnapshotReuse::SeedOnly,
            1e-12,
            1e-9,
        )
        .expect("resolve grounded-substrate NPN VBIC diffamp snapshot");
        let linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble grounded-substrate NPN VBIC diffamp transient linearization");
        let residual_norm = Engine::vbic_internal_equation_residual_norm(
            &linearization,
            &snapshot.reduction.external_voltages,
            &snapshot.reduction.internal_voltages,
        );
        assert!(
            Engine::vbic_dynamic_snapshot_solution_is_acceptable(
                &linearization,
                &snapshot.reduction.external_voltages,
                &snapshot.reduction.internal_voltages,
            ),
            "expected resolved grounded-substrate NPN VBIC diffamp snapshot to satisfy hidden-state equations, got residual {residual_norm:.12e} with internal={:?}",
            snapshot.reduction.internal_voltages,
        );
    }

    #[test]
    fn test_vbic_npn_diffamp_collector_substrate_charge_homotopy_solves_hidden_state() {
        let (mut bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;

        let history_linear_prev =
            Engine::vbic_predictor_linear_branch_state(&bjt, [vc, vb, ve, vs], history_prev);
        let history_linear_prev_prev =
            Engine::vbic_predictor_linear_branch_state(&bjt, [vc, vb, ve, vs], history_prev_prev);
        let seed_internal = Engine::vbic_dynamic_internal_seed_from_history_with_linear_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            Some(&history_linear_prev),
            Some(&history_linear_prev_prev),
            dt,
            dt / 2.0,
        );

        let (snapshot, linearization, _) =
            Engine::solve_vbic_dynamic_snapshot_with_collector_substrate_charge_homotopy(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&seed_internal),
            )
            .expect("solve grounded-substrate NPN VBIC diffamp with collector/substrate charge homotopy");
        let residual_norm = Engine::vbic_internal_equation_residual_norm(
            &linearization,
            &snapshot.reduction.external_voltages,
            &snapshot.reduction.internal_voltages,
        );
        assert!(
            Engine::vbic_dynamic_snapshot_solution_is_acceptable(
                &linearization,
                &snapshot.reduction.external_voltages,
                &snapshot.reduction.internal_voltages,
            ),
            "expected collector/substrate charge homotopy to land on a strictly converged hidden-state solution, got residual {residual_norm:.12e} with internal={:?}",
            snapshot.reduction.internal_voltages,
        );
    }

    #[test]
    fn test_vbic_dynamic_internal_step_limit_allows_common_mode_static_motion() {
        let current = [0.0; BJT_INTERNAL_STATE_DIM];
        let mut target = [0.0; BJT_INTERNAL_STATE_DIM];
        for value in &mut target[..BJT_THERMAL_STATE_INDEX] {
            *value = 1.2;
        }
        target[BJT_THERMAL_STATE_INDEX] = 0.2;
        target[BJT_DELAY_XF1_STATE_INDEX] = 1.0e-5;
        target[BJT_DELAY_XF2_STATE_INDEX] = -2.0e-5;

        let limited = Engine::step_limit_vbic_dynamic_internal_target(current, target, 0, 1e-1);

        for idx in 0..BJT_THERMAL_STATE_INDEX {
            assert!(
                (limited[idx] - 1.0).abs() <= 1e-12,
                "expected static common-mode step to reach relaxed 1 V cap at index {idx}, got {:.12e}",
                limited[idx],
            );
        }
        assert!(
            (limited[BJT_THERMAL_STATE_INDEX] - target[BJT_THERMAL_STATE_INDEX] * (1.0 / 1.2))
                .abs()
                <= 1e-12,
            "expected thermal step to share static alpha, got {:.12e}",
            limited[BJT_THERMAL_STATE_INDEX],
        );
    }

    #[test]
    fn test_vbic_dynamic_internal_step_limit_allows_sub_millisteps_for_huge_targets() {
        let current = [0.0; BJT_INTERNAL_STATE_DIM];
        let mut target = [0.0; BJT_INTERNAL_STATE_DIM];
        target[BJT_VCX_STATE_INDEX] = 1.0e9;

        let limited = Engine::step_limit_vbic_dynamic_internal_target(current, target, 0, 1e-2);

        assert!(
            (limited[BJT_VCX_STATE_INDEX] - 0.5).abs() <= 1e-12,
            "expected the VBIC internal line search to allow a sub-millistep that still honors the static 0.5 V cap for this residual band"
        );
    }

    #[test]
    fn test_vbic_dynamic_internal_step_limit_keeps_grounded_substrate_newton_improving() {
        let (mut bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let seed_eval = Engine::evaluate_vbic_dynamic_internal_state(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            live_seed,
        )
        .expect("evaluate grounded-substrate live VBIC seed");
        let raw_newton =
            Engine::solve_vbic_internal_state_from_linearization(&seed_eval.1, &[vc, vb, ve, vs])
                .expect("solve grounded-substrate live VBIC Newton target");
        let limited_target =
            Engine::step_limit_vbic_dynamic_internal_target(live_seed, raw_newton, 0, seed_eval.4);
        let limited_eval = Engine::evaluate_vbic_dynamic_internal_state(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            limited_target,
        )
        .expect("evaluate grounded-substrate limited VBIC Newton target");

        assert!(
            limited_eval.4 + 1e-15 < seed_eval.4,
            "expected grounded-substrate VBIC limited Newton target to reduce residual, seed_residual={:.12e}, limited_residual={:.12e}, seed={:?}, raw_newton={:?}, limited={:?}",
            seed_eval.4,
            limited_eval.4,
            live_seed,
            raw_newton,
            limited_target,
        );
    }

    #[test]
    #[ignore]
    fn debug_vbic_npn_collector_substrate_dynamic_reduced_jacobian_matches_finite_difference() {
        let (mut bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_collector_substrate_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;

        assert_vbic_dynamic_reduced_jacobian_matches_finite_difference(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            &history_prev,
            &history_prev_prev,
            "NPN diffamp collector substrate",
        );
    }

    #[test]
    #[ignore]
    fn debug_vbic_npn_ground_substrate_no_rs_dynamic_reduced_jacobian_matches_finite_difference() {
        let (mut bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_bjt_with_model_patch("RS=0");
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;

        assert_vbic_dynamic_reduced_jacobian_matches_finite_difference(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            &history_prev,
            &history_prev_prev,
            "NPN diffamp ground substrate RS=0",
        );
    }

    #[test]
    #[ignore]
    fn debug_vbic_npn_ground_substrate_no_rbp_dynamic_reduced_jacobian_matches_finite_difference() {
        let (mut bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_bjt_with_model_patch("RBP=0");
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;

        assert_vbic_dynamic_reduced_jacobian_matches_finite_difference(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            &history_prev,
            &history_prev_prev,
            "NPN diffamp ground substrate RBP=0",
        );
    }

    #[test]
    #[ignore]
    fn debug_vbic_npn_ground_substrate_history_seed_solver_diagnostics() {
        let (mut bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;

        let history_linear_prev =
            Engine::vbic_predictor_linear_branch_state(&bjt, [vc, vb, ve, vs], history_prev);
        let history_linear_prev_prev =
            Engine::vbic_predictor_linear_branch_state(&bjt, [vc, vb, ve, vs], history_prev_prev);
        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history_with_linear_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            Some(&history_linear_prev),
            Some(&history_linear_prev_prev),
            dt,
            dt / 2.0,
        );

        let dump_seed = |label: &str, seed: [Value; BJT_INTERNAL_STATE_DIM]| {
            let evaluation = Engine::evaluate_vbic_dynamic_internal_state(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                seed,
            )
            .expect("evaluate VBIC diagnostic seed");
            let raw_newton = Engine::solve_vbic_internal_state_from_linearization(
                &evaluation.1,
                &[vc, vb, ve, vs],
            )
            .expect("solve VBIC diagnostic raw Newton step");
            let raw_newton_residual = Engine::evaluate_vbic_dynamic_internal_state(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                raw_newton,
            )
            .expect("evaluate VBIC diagnostic raw Newton state");
            let limited_newton =
                bjt.limit_vbic_dynamic_internal_state_to_previous(raw_newton, seed);
            let limited_newton_residual = Engine::evaluate_vbic_dynamic_internal_state(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                limited_newton,
            )
            .expect("evaluate VBIC diagnostic limited Newton state");
            let mut best_direction_alpha = 0.0_f64;
            let mut best_direction_residual = evaluation.4;
            for alpha in [
                1e-4, 3e-4, 1e-3, 3e-3, 1e-2, 3e-2, 1e-1, 2e-1, 3e-1, 5e-1, 7e-1, 1.0,
            ] {
                let mut candidate = seed;
                for idx in 0..BJT_INTERNAL_STATE_DIM {
                    candidate[idx] = seed[idx] + alpha * (raw_newton[idx] - seed[idx]);
                }
                let candidate_eval = Engine::evaluate_vbic_dynamic_internal_state(
                    &bjt,
                    vc,
                    vb,
                    ve,
                    vs,
                    method,
                    trap_order,
                    dt,
                    &q_prev,
                    &q_prev_prev,
                    &cq_prev,
                    candidate,
                )
                .expect("evaluate VBIC diagnostic direction sample");
                if candidate_eval.4 < best_direction_residual {
                    best_direction_alpha = alpha;
                    best_direction_residual = candidate_eval.4;
                }
            }
            let snapshot = bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, seed);
            let linearization = Engine::assemble_vbic_transient_linearization(
                &bjt,
                &snapshot,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
            )
            .expect("assemble VBIC diagnostic transient linearization");
            let residual =
                Engine::vbic_internal_equation_residual(&linearization, &[vc, vb, ve, vs], &seed);
            let mut worst_g_ie_row = 0usize;
            let mut worst_g_ie_col = 0usize;
            let mut worst_g_ie_analytical = 0.0;
            let mut worst_g_ie_numerical = 0.0;
            let mut worst_g_ie_rel_err = 0.0_f64;
            let eps = 1e-6;
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let mut plus_external = [vc, vb, ve, vs];
                plus_external[col] += eps;
                let plus_snapshot = bjt.charge_snapshot_for_dynamic_state(
                    plus_external[0],
                    plus_external[1],
                    plus_external[2],
                    plus_external[3],
                    seed,
                );
                let plus_linearization = Engine::assemble_vbic_transient_linearization(
                    &bjt,
                    &plus_snapshot,
                    method,
                    trap_order,
                    dt,
                    &q_prev,
                    &q_prev_prev,
                    &cq_prev,
                )
                .expect("assemble plus VBIC diagnostic transient linearization");
                let plus_residual = Engine::vbic_internal_equation_residual(
                    &plus_linearization,
                    &plus_external,
                    &seed,
                );
                let mut minus_external = [vc, vb, ve, vs];
                minus_external[col] -= eps;
                let minus_snapshot = bjt.charge_snapshot_for_dynamic_state(
                    minus_external[0],
                    minus_external[1],
                    minus_external[2],
                    minus_external[3],
                    seed,
                );
                let minus_linearization = Engine::assemble_vbic_transient_linearization(
                    &bjt,
                    &minus_snapshot,
                    method,
                    trap_order,
                    dt,
                    &q_prev,
                    &q_prev_prev,
                    &cq_prev,
                )
                .expect("assemble minus VBIC diagnostic transient linearization");
                let minus_residual = Engine::vbic_internal_equation_residual(
                    &minus_linearization,
                    &minus_external,
                    &seed,
                );
                for row in 0..BJT_INTERNAL_STATE_DIM {
                    let analytical = linearization.g_ie[row][col];
                    let numerical = (plus_residual[row] - minus_residual[row]) / (2.0 * eps);
                    let scale = analytical.abs().max(numerical.abs()).max(1e-8);
                    let rel_err = (analytical - numerical).abs() / scale;
                    if rel_err > worst_g_ie_rel_err {
                        worst_g_ie_row = row;
                        worst_g_ie_col = col;
                        worst_g_ie_analytical = analytical;
                        worst_g_ie_numerical = numerical;
                        worst_g_ie_rel_err = rel_err;
                    }
                }
            }
            eprintln!(
                "{label}: seed={seed:?} residual={:.12e} residual_vec={residual:?} raw_newton={raw_newton:?} raw_newton_residual={:.12e} limited_newton={limited_newton:?} limited_newton_residual={:.12e} best_direction_alpha={best_direction_alpha:.3e} best_direction_residual={best_direction_residual:.12e} worst_g_ie=({worst_g_ie_row},{worst_g_ie_col}) analytical={worst_g_ie_analytical:.12e} numerical={worst_g_ie_numerical:.12e} rel_err={worst_g_ie_rel_err:.3e}",
                evaluation.4, raw_newton_residual.4, limited_newton_residual.4,
            );
            let solved = Engine::solve_vbic_dynamic_snapshot(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&seed),
            )
            .map(|(snapshot, _, _)| snapshot.reduction.internal_voltages);
            eprintln!("{label}: solve_result={solved:?}");

            let mut loop_snapshot = bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, seed);
            Engine::rebalance_vbic_dynamic_thermal_state(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                &mut loop_snapshot,
            );
            let mut loop_linearization = Engine::assemble_vbic_transient_linearization(
                &bjt,
                &loop_snapshot,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
            )
            .expect("assemble VBIC diagnostic loop linearization");
            let mut loop_residual = Engine::vbic_internal_equation_residual_norm(
                &loop_linearization,
                &loop_snapshot.reduction.external_voltages,
                &loop_snapshot.reduction.internal_voltages,
            );
            eprintln!("{label}: refinement-start residual={loop_residual:.12e}");
            for iteration in 0..8 {
                let current_internal = loop_snapshot.reduction.internal_voltages;
                let solved_internal = Engine::solve_vbic_internal_state_from_linearization(
                    &loop_linearization,
                    &loop_snapshot.reduction.external_voltages,
                )
                .expect("solve VBIC diagnostic refinement Newton target");
                let target_internal = Engine::step_limit_vbic_dynamic_internal_target(
                    current_internal,
                    solved_internal,
                    iteration,
                    loop_residual,
                );
                let loop_residual_objective = Engine::vbic_internal_equation_residual_objective(
                    &Engine::vbic_internal_equation_residual(
                        &loop_linearization,
                        &loop_snapshot.reduction.external_voltages,
                        &loop_snapshot.reduction.internal_voltages,
                    ),
                );
                let next_state = Engine::improve_vbic_dynamic_internal_state_toward_target(
                    &bjt,
                    vc,
                    vb,
                    ve,
                    vs,
                    method,
                    trap_order,
                    dt,
                    &q_prev,
                    &q_prev_prev,
                    &cq_prev,
                    current_internal,
                    loop_residual,
                    loop_residual_objective,
                    target_internal,
                    current_internal,
                    12,
                );
                match next_state {
                    Some(next_state) => {
                        eprintln!(
                            "{label}: refinement iter={iteration} residual={:.12e} -> {:.12e}",
                            loop_residual, next_state.4,
                        );
                        loop_snapshot = next_state.0;
                        loop_linearization = next_state.1;
                        loop_residual = next_state.4;
                    }
                    None => {
                        eprintln!(
                            "{label}: refinement iter={iteration} stalled at residual={loop_residual:.12e}"
                        );
                        break;
                    }
                }
            }
            let loop_residual_vec = Engine::vbic_internal_equation_residual(
                &loop_linearization,
                &loop_snapshot.reduction.external_voltages,
                &loop_snapshot.reduction.internal_voltages,
            );
            let stalled_internal = loop_snapshot.reduction.internal_voltages;
            let stalled_external = loop_snapshot.reduction.external_voltages;
            let mut worst_g_ii_row = 0usize;
            let mut worst_g_ii_col = 0usize;
            let mut worst_g_ii_analytical = 0.0;
            let mut worst_g_ii_numerical = 0.0;
            let mut worst_g_ii_rel_err = 0.0_f64;
            for col in 0..BJT_INTERNAL_STATE_DIM {
                let base_value = stalled_internal[col];
                let step = match col {
                    BJT_DELAY_XF1_STATE_INDEX | BJT_DELAY_XF2_STATE_INDEX => {
                        (base_value.abs() * 1e-3).max(1e-9)
                    }
                    BJT_THERMAL_STATE_INDEX => (base_value.abs() * 1e-4).max(1e-6),
                    _ => (base_value.abs() * 1e-6).max(1e-7),
                };
                let mut plus_internal = stalled_internal;
                plus_internal[col] = base_value + step;
                if col == BJT_THERMAL_STATE_INDEX {
                    plus_internal[col] = plus_internal[col].max(bjt.minimum_thermal_rise());
                }
                let Some(plus_state) = Engine::evaluate_vbic_dynamic_internal_state(
                    &bjt,
                    vc,
                    vb,
                    ve,
                    vs,
                    method,
                    trap_order,
                    dt,
                    &q_prev,
                    &q_prev_prev,
                    &cq_prev,
                    plus_internal,
                ) else {
                    continue;
                };
                let use_central = col != BJT_THERMAL_STATE_INDEX
                    || base_value - step >= bjt.minimum_thermal_rise();
                if use_central {
                    let mut minus_internal = stalled_internal;
                    minus_internal[col] = base_value - step;
                    if col == BJT_THERMAL_STATE_INDEX {
                        minus_internal[col] = minus_internal[col].max(bjt.minimum_thermal_rise());
                    }
                    let Some(minus_state) = Engine::evaluate_vbic_dynamic_internal_state(
                        &bjt,
                        vc,
                        vb,
                        ve,
                        vs,
                        method,
                        trap_order,
                        dt,
                        &q_prev,
                        &q_prev_prev,
                        &cq_prev,
                        minus_internal,
                    ) else {
                        continue;
                    };
                    let denom = plus_internal[col] - minus_internal[col];
                    if denom.abs() <= 0.0 {
                        continue;
                    }
                    for row in 0..BJT_INTERNAL_STATE_DIM {
                        let analytical = loop_linearization.g_ii[row][col];
                        let numerical = (plus_state.3[row] - minus_state.3[row]) / denom;
                        let scale = analytical.abs().max(numerical.abs()).max(1e-8);
                        let rel_err = (analytical - numerical).abs() / scale;
                        if rel_err > worst_g_ii_rel_err {
                            worst_g_ii_row = row;
                            worst_g_ii_col = col;
                            worst_g_ii_analytical = analytical;
                            worst_g_ii_numerical = numerical;
                            worst_g_ii_rel_err = rel_err;
                        }
                    }
                } else {
                    let denom = plus_internal[col] - stalled_internal[col];
                    if denom.abs() <= 0.0 {
                        continue;
                    }
                    for row in 0..BJT_INTERNAL_STATE_DIM {
                        let analytical = loop_linearization.g_ii[row][col];
                        let numerical = (plus_state.3[row] - loop_residual_vec[row]) / denom;
                        let scale = analytical.abs().max(numerical.abs()).max(1e-8);
                        let rel_err = (analytical - numerical).abs() / scale;
                        if rel_err > worst_g_ii_rel_err {
                            worst_g_ii_row = row;
                            worst_g_ii_col = col;
                            worst_g_ii_analytical = analytical;
                            worst_g_ii_numerical = numerical;
                            worst_g_ii_rel_err = rel_err;
                        }
                    }
                }
            }
            eprintln!(
                "{label}: refinement-final residual={loop_residual:.12e} residual_vec={loop_residual_vec:?} worst_g_ii=({worst_g_ii_row},{worst_g_ii_col}) analytical={worst_g_ii_analytical:.12e} numerical={worst_g_ii_numerical:.12e} rel_err={worst_g_ii_rel_err:.3e}"
            );
        };

        let summarize_variant = |label: &str, mut variant: crate::device::Bjt| {
            bjt_update_for_external_bias(&mut variant, vc, vb, ve, vs);
            let variant_base_snapshot = variant.charge_snapshot(vc, vb, ve, vs);
            let mut variant_q_prev = variant_base_snapshot.branches.map(|branch| branch.charge);
            let mut variant_q_prev_prev = variant_q_prev;
            let variant_cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
            variant_q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
            variant_q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
            variant_q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
            variant_q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

            let mut variant_history_prev = variant_base_snapshot.reduction.internal_voltages;
            variant_history_prev[BJT_DELAY_XF1_STATE_INDEX] =
                0.65 * variant_history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
            variant_history_prev[BJT_DELAY_XF2_STATE_INDEX] =
                1.35 * variant_history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
            variant_history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
            let mut variant_history_prev_prev = variant_history_prev;
            variant_history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
                0.8 * variant_history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
            variant_history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;

            let variant_history_linear_prev = Engine::vbic_predictor_linear_branch_state(
                &variant,
                [vc, vb, ve, vs],
                variant_history_prev,
            );
            let variant_history_linear_prev_prev = Engine::vbic_predictor_linear_branch_state(
                &variant,
                [vc, vb, ve, vs],
                variant_history_prev_prev,
            );
            let variant_seed = Engine::vbic_dynamic_internal_seed_from_history_with_linear_history(
                &variant,
                vc,
                vb,
                ve,
                vs,
                Some(&variant_history_prev),
                Some(&variant_history_prev_prev),
                Some(&variant_history_linear_prev),
                Some(&variant_history_linear_prev_prev),
                dt,
                dt / 2.0,
            );
            let direct_result = Engine::solve_vbic_dynamic_snapshot(
                &variant,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &variant_q_prev,
                &variant_q_prev_prev,
                &variant_cq_prev,
                Some(&variant_seed),
            )
            .is_some();
            let best_effort_residual = Engine::solve_vbic_dynamic_snapshot_best_effort(
                &variant,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &variant_q_prev,
                &variant_q_prev_prev,
                &variant_cq_prev,
                Some(&variant_seed),
            )
            .map(|(_, _, _, residual)| residual);
            eprintln!(
                "{label}: direct={direct_result} best_effort_residual={best_effort_residual:?} qco={:.3e} cjcp={:.3e} ccso={:.3e} cjep={:.3e} rbp={:.3e} selft={:.3e} rth={:.3e} td={:.3e}",
                variant.qco,
                variant.cjcp,
                variant.ccso,
                variant.cjep,
                variant.rbp,
                variant.selft,
                variant.rth,
                variant.td,
            );
        };

        dump_seed("history_prev", history_prev);
        dump_seed("live_seed", live_seed);
        dump_seed("merged_seed", merged_seed);
        let mut no_epi = bjt.clone();
        no_epi.qco = 0.0;
        summarize_variant("variant_no_epi_qco", no_epi);
        let mut no_substrate_cap = bjt.clone();
        no_substrate_cap.cjcp = 0.0;
        no_substrate_cap.ccso = 0.0;
        summarize_variant("variant_no_substrate_cap", no_substrate_cap);
        let mut no_peripheral_path = bjt.clone();
        no_peripheral_path.rbp = 0.0;
        no_peripheral_path.cjep = 0.0;
        no_peripheral_path.cjcp = 0.0;
        no_peripheral_path.ccso = 0.0;
        no_peripheral_path.ibeip = 0.0;
        no_peripheral_path.ibenp = 0.0;
        no_peripheral_path.ibcip = 0.0;
        no_peripheral_path.ibcnp = 0.0;
        summarize_variant("variant_no_peripheral_path", no_peripheral_path);
        let mut no_self_heat = bjt.clone();
        no_self_heat.selft = 0.0;
        no_self_heat.rth = 0.0;
        no_self_heat.cth = 0.0;
        summarize_variant("variant_no_self_heat", no_self_heat);
        panic!("VBIC grounded-substrate history-seed diagnostics complete");
    }

    #[test]
    fn test_vbic_pnp_diffamp_dynamic_reduced_jacobian_matches_finite_difference() {
        let (mut bjt, vc, vb, ve, vs) = vbic_pnp_diffamp_mirror_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;

        assert_vbic_dynamic_reduced_jacobian_matches_finite_difference(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            &history_prev,
            &history_prev_prev,
            "PNP diffamp",
        );
    }

    #[test]
    fn test_vbic_dynamic_delta_stamp_aggregates_correctly_for_tied_external_nodes() {
        #[derive(Default)]
        struct CaptureMatrix {
            entries: std::collections::HashMap<(usize, usize), Value>,
            rhs: std::collections::HashMap<usize, Value>,
            rhs_calls: Vec<(usize, Value)>,
        }

        impl CaptureMatrix {
            fn g(&self, row: usize, col: usize) -> Value {
                *self.entries.get(&(row, col)).unwrap_or(&0.0)
            }

            fn i(&self, node: usize) -> Value {
                *self.rhs.get(&node).unwrap_or(&0.0)
            }
        }

        impl MatrixStamper for CaptureMatrix {
            fn stamp(&mut self, row: usize, col: usize, value: Value) {
                *self.entries.entry((row, col)).or_insert(0.0) += value;
            }

            fn stamp_rhs(&mut self, index: usize, value: Value) {
                *self.rhs.entry(index).or_insert(0.0) += value;
                self.rhs_calls.push((index, value));
            }
        }

        let (mut bjt, vc, vb, ve, vs) = vbic_pnp_tied_mirror_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;

        let seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            dt,
            dt,
        );
        let (snapshot, linearization, _) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&seed),
        )
        .expect("solve tied-node VBIC transient state");
        let (y_total, reduced_i_eq) = Engine::vbic_reduce_transient_external_system(&linearization)
            .expect("reduce tied-node VBIC transient external system");
        let (base_static_g, base_static_i_eq) = Engine::vbic_static_stamped_external_system(
            &bjt,
            &snapshot.reduction.external_voltages,
        );

        let mut delta = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut delta_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            delta_i_eq[row] = reduced_i_eq[row] - base_static_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                delta[row][col] = y_total[row][col] - base_static_g[row][col];
            }
        }

        let nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        let voltage_vector = bjt_node_indexed_voltage_vector(&bjt, vc, vb, ve, vs);
        let mut stamped = CaptureMatrix::default();
        let mut rhs_scratch = [0.0; BJT_EXTERNAL_STATE_DIM];
        bjt.stamp_nonlinear(&voltage_vector, &mut stamped, &mut rhs_scratch);
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let node_row = nodes[row];
            if node_row == 0 {
                continue;
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let node_col = nodes[col];
                if node_col > 0 {
                    stamped.stamp(node_row, node_col, delta[row][col]);
                }
            }
            stamped.stamp_rhs(node_row, delta_i_eq[row]);
        }

        let mut static_stamped = CaptureMatrix::default();
        let mut static_rhs_scratch = [0.0; BJT_EXTERNAL_STATE_DIM];
        bjt.stamp_nonlinear(
            &voltage_vector,
            &mut static_stamped,
            &mut static_rhs_scratch,
        );

        let mut delta_stamped = CaptureMatrix::default();
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let node_row = nodes[row];
            if node_row == 0 {
                continue;
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let node_col = nodes[col];
                if node_col > 0 {
                    delta_stamped.stamp(node_row, node_col, delta[row][col]);
                }
            }
            delta_stamped.stamp_rhs(node_row, delta_i_eq[row]);
        }

        let mut expected_static_rhs = std::collections::BTreeMap::<usize, Value>::new();
        let mut expected_static_g = std::collections::BTreeMap::<(usize, usize), Value>::new();
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let node_row = nodes[row];
            if node_row == 0 {
                continue;
            }
            *expected_static_rhs.entry(node_row).or_insert(0.0) += base_static_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let node_col = nodes[col];
                if node_col == 0 {
                    continue;
                }
                *expected_static_g.entry((node_row, node_col)).or_insert(0.0) +=
                    base_static_g[row][col];
            }
        }

        let mut expected_rhs = std::collections::BTreeMap::<usize, Value>::new();
        let mut expected_g = std::collections::BTreeMap::<(usize, usize), Value>::new();
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let node_row = nodes[row];
            if node_row == 0 {
                continue;
            }
            *expected_rhs.entry(node_row).or_insert(0.0) += reduced_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let node_col = nodes[col];
                if node_col == 0 {
                    continue;
                }
                *expected_g.entry((node_row, node_col)).or_insert(0.0) += y_total[row][col];
            }
        }

        for (&node_row, &rhs_value) in &expected_static_rhs {
            let actual = static_stamped.i(node_row);
            assert!(
                (actual - rhs_value).abs() < 1e-12,
                "expected aggregated tied-node static rhs at node {node_row} to match the reduced static rhs, actual={actual:.12e}, expected={rhs_value:.12e}, direct_rhs={:?}, reduced_static={:?}, rhs_calls={:?}, nodes={:?}, voltage_vector={:?}",
                static_rhs_scratch,
                base_static_i_eq,
                static_stamped.rhs_calls,
                nodes,
                voltage_vector,
            );
        }
        for (&(node_row, node_col), &g_value) in &expected_static_g {
            let actual = static_stamped.g(node_row, node_col);
            assert!(
                (actual - g_value).abs() < 1e-12,
                "expected aggregated tied-node static conductance at ({node_row}, {node_col}) to match the reduced static system, actual={actual:.12e}, expected={g_value:.12e}",
            );
        }
        for (&node_row, &rhs_value) in &expected_rhs {
            let actual = stamped.i(node_row);
            assert!(
                (actual - rhs_value).abs() < 1e-12,
                "expected aggregated tied-node rhs at node {node_row} to match the reduced transient rhs, actual={actual:.12e}, expected={rhs_value:.12e}, static_actual={:.12e}, delta_actual={:.12e}, static_expected={:.12e}, delta_expected={:.12e}",
                static_stamped.i(node_row),
                delta_stamped.i(node_row),
                expected_static_rhs.get(&node_row).copied().unwrap_or(0.0),
                rhs_value - expected_static_rhs.get(&node_row).copied().unwrap_or(0.0),
            );
        }
        for (&(node_row, node_col), &g_value) in &expected_g {
            let actual = stamped.g(node_row, node_col);
            assert!(
                (actual - g_value).abs() < 1e-12,
                "expected aggregated tied-node conductance at ({node_row}, {node_col}) to match the reduced transient system, actual={actual:.12e}, expected={g_value:.12e}, static_actual={:.12e}, delta_actual={:.12e}, static_expected={:.12e}, delta_expected={:.12e}",
                static_stamped.g(node_row, node_col),
                delta_stamped.g(node_row, node_col),
                expected_static_g
                    .get(&(node_row, node_col))
                    .copied()
                    .unwrap_or(0.0),
                g_value
                    - expected_static_g
                        .get(&(node_row, node_col))
                        .copied()
                        .unwrap_or(0.0),
            );
        }
    }

    #[test]
    fn test_solve_vbic_dynamic_snapshot_converges_self_heated_internal_state_after_history_seed() {
        let (bjt, vc, vb, ve, vs) = vbic_self_heated_focus_test_bjt();
        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let mut history_prev = base_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] =
            0.65 * history_prev[BJT_DELAY_XF1_STATE_INDEX] + 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] =
            1.35 * history_prev[BJT_DELAY_XF2_STATE_INDEX] - 2.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 4.0;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] =
            0.8 * history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] + 3.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 1.5;
        let seed_internal = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            dt,
            dt / 2.0,
        );
        let history_seed = bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, seed_internal);
        let seed_residual = Engine::vbic_transient_thermal_residual_and_derivative(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            history_seed.reduction.internal_voltages,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .0
        .abs();

        let (solved_snapshot, solved_linearization, solved_static_g) =
            Engine::solve_vbic_dynamic_snapshot(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&seed_internal),
            )
            .expect("solve VBIC transient state");
        let solved_residual = Engine::vbic_transient_thermal_residual_and_derivative(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            solved_snapshot.reduction.internal_voltages,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .0
        .abs();

        assert!(
            solved_residual < seed_residual,
            "expected dynamic thermal rebalance to reduce residual from {seed_residual:.3e} to {solved_residual:.3e}"
        );
        assert!(
            solved_residual < 1e-9,
            "expected self-heated dynamic thermal residual to converge, got {solved_residual:.3e}"
        );
        assert!(
            (solved_snapshot.reduction.internal_voltages[BJT_THERMAL_STATE_INDEX]
                - history_seed.reduction.internal_voltages[BJT_THERMAL_STATE_INDEX])
                .abs()
                > 1e-12,
            "expected self-heating rebalance to move the thermal state after history seeding"
        );

        let rebuilt_linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &solved_snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("rebuild solved transient linearization");
        let solved_internal = Engine::solve_vbic_internal_state_from_linearization(
            &solved_linearization,
            &solved_snapshot.reduction.external_voltages,
        )
        .expect("solve reduced self-heated VBIC internal state");
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            let delta =
                (solved_snapshot.reduction.internal_voltages[idx] - solved_internal[idx]).abs();
            assert!(
                delta < 2e-11,
                "expected self-heated solved snapshot to satisfy reduced internal solve at index {idx}; snapshot={:.16e}, solved={:.16e}, delta={delta:.3e}",
                solved_snapshot.reduction.internal_voltages[idx],
                solved_internal[idx]
            );
        }
        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                assert!(
                    (solved_linearization.g_ii[row][col] - rebuilt_linearization.g_ii[row][col])
                        .abs()
                        < 1e-18,
                    "expected solved transient internal matrix to match rebuilt assembly at ({row}, {col})"
                );
            }
        }
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                assert!(
                    (solved_static_g[row][col] - solved_snapshot.reduction.g_reduced[row][col])
                        .abs()
                        < 1e-18,
                    "expected solved static reduced conductance to match rebuilt snapshot at ({row}, {col})"
                );
            }
        }
        let non_thermal_delta = (0..BJT_INTERNAL_STATE_DIM)
            .filter(|&idx| idx != BJT_THERMAL_STATE_INDEX)
            .map(|idx| {
                (solved_snapshot.reduction.internal_voltages[idx]
                    - history_seed.reduction.internal_voltages[idx])
                    .abs()
            })
            .fold(0.0, Value::max);
        assert!(
            non_thermal_delta > 1e-12,
            "expected reduced internal-state solve to move at least one non-thermal state; max delta was {non_thermal_delta:.3e}"
        );
    }

    #[test]
    fn test_solve_vbic_dynamic_snapshot_matches_live_operating_point_seed_for_self_heating() {
        let (mut bjt, vc, vb, ve, vs) = vbic_self_heated_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let mut q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        q_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 0.35;
        q_prev_prev[BJT_DELAY_XF1_BRANCH_INDEX] *= 1.20;
        q_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 1.65;
        q_prev_prev[BJT_DELAY_XF2_BRANCH_INDEX] *= 0.50;

        let seed_internal = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let (unseeded_snapshot, unseeded_linearization, unseeded_static_g) =
            Engine::solve_vbic_dynamic_snapshot(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                None,
            )
            .expect("solve VBIC transient state without live seed");
        let (seeded_snapshot, seeded_linearization, seeded_static_g) =
            Engine::solve_vbic_dynamic_snapshot(
                &bjt,
                vc,
                vb,
                ve,
                vs,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&seed_internal),
            )
            .expect("solve VBIC transient state with live seed");

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (seeded_snapshot.reduction.internal_voltages[idx]
                    - unseeded_snapshot.reduction.internal_voltages[idx])
                    .abs()
                    < 1e-18,
                "expected live seeded snapshot to match unseeded solve at index {idx}"
            );
        }
        for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
            assert!(
                (seeded_snapshot.branches[branch_idx].charge
                    - unseeded_snapshot.branches[branch_idx].charge)
                    .abs()
                    < 1e-18,
                "expected live seeded branch charge to match unseeded solve at index {branch_idx}"
            );
        }
        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                assert!(
                    (seeded_linearization.g_ii[row][col] - unseeded_linearization.g_ii[row][col])
                        .abs()
                        < 1e-18,
                    "expected live seeded internal matrix to match unseeded solve at ({row}, {col})"
                );
            }
        }
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                assert!(
                    (seeded_static_g[row][col] - unseeded_static_g[row][col]).abs() < 1e-18,
                    "expected live seeded static reduced conductance to match unseeded solve at ({row}, {col})"
                );
            }
        }
    }

    #[test]
    fn test_vbic_snapshot_convergence_met_accepts_identical_excess_phase_snapshot() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let criteria = NonlinearConvergenceCriteria::new(1e-6, 1e-12, 1e-3);

        assert!(Engine::vbic_snapshot_convergence_met(
            &bjt,
            [vc, vb, ve, vs],
            &snapshot,
            [vc, vb, ve, vs],
            &snapshot,
            criteria,
        ));
    }

    #[test]
    fn test_vbic_snapshot_convergence_met_accounts_for_hidden_vxf2_predictor_term() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let previous_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut shifted_internal = previous_snapshot.reduction.internal_voltages;
        shifted_internal[BJT_DELAY_XF2_STATE_INDEX] += 5.0e-7;
        let current_snapshot =
            bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, shifted_internal);
        let criteria = NonlinearConvergenceCriteria::new(1e-6, 1e-12, 1e-3);

        let previous_state =
            bjt.vbic_transient_convergence_state_for_snapshot(vc, vb, ve, vs, &previous_snapshot);
        let current_state =
            bjt.vbic_transient_convergence_state_for_snapshot(vc, vb, ve, vs, &current_snapshot);
        let iciei_delta = (current_state.currents[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX]
            - previous_state.currents[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX])
            .abs();
        let iciei_tol = criteria.relative_tolerance()
            * current_state.currents[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX]
                .abs()
                .max(previous_state.currents[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX].abs())
            + criteria.current_tolerance();
        let mut delta_internal = [0.0; BJT_INTERNAL_STATE_DIM];
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            delta_internal[idx] = current_snapshot.reduction.internal_voltages[idx]
                - previous_snapshot.reduction.internal_voltages[idx];
        }
        let legacy_predicted_iciei = previous_state.currents
            [VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX]
            + previous_state.d_currents_d_internal[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX]
                .iter()
                .zip(delta_internal.iter())
                .enumerate()
                .filter(|(idx, _)| {
                    *idx != BJT_THERMAL_STATE_INDEX && *idx != BJT_DELAY_XF2_STATE_INDEX
                })
                .map(|(_, (derivative, delta))| derivative * delta)
                .sum::<Value>();
        let legacy_iciei_mismatch = (legacy_predicted_iciei
            - current_state.currents[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX])
            .abs();

        assert!(iciei_delta > iciei_tol);
        assert!(legacy_iciei_mismatch > iciei_tol);
        assert!(Engine::vbic_snapshot_convergence_met(
            &bjt,
            [vc, vb, ve, vs],
            &previous_snapshot,
            [vc, vb, ve, vs],
            &current_snapshot,
            criteria,
        ));
    }

    #[test]
    fn test_vbic_continuation_candidate_accepts_ngspice_style_snapshot_convergence() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let previous_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut shifted_internal = previous_snapshot.reduction.internal_voltages;
        shifted_internal[BJT_DELAY_XF2_STATE_INDEX] += 5.0e-7;
        let candidate_snapshot =
            bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, shifted_internal);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;
        let q_prev = previous_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let candidate_linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &candidate_snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble shifted VBIC continuation candidate");

        assert!(
            !Engine::vbic_dynamic_snapshot_solution_is_acceptable(
                &candidate_linearization,
                &candidate_snapshot.reduction.external_voltages,
                &candidate_snapshot.reduction.internal_voltages,
            ),
            "expected the shifted hidden state to miss the strict reduced-equation acceptance gate"
        );
        assert!(Engine::vbic_continuation_candidate_is_acceptable(
            &bjt,
            [vc, vb, ve, vs],
            &previous_snapshot,
            &candidate_snapshot,
            &candidate_linearization,
        ));
    }

    #[test]
    fn test_vbic_continuation_candidate_rejects_gross_hidden_state_mismatch() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let previous_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut shifted_internal = previous_snapshot.reduction.internal_voltages;
        shifted_internal[BJT_DELAY_XF2_STATE_INDEX] += 5.0e-3;
        let candidate_snapshot =
            bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, shifted_internal);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;
        let q_prev = previous_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let candidate_linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &candidate_snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble grossly shifted VBIC continuation candidate");

        assert!(
            !Engine::vbic_dynamic_snapshot_solution_is_acceptable(
                &candidate_linearization,
                &candidate_snapshot.reduction.external_voltages,
                &candidate_snapshot.reduction.internal_voltages,
            ),
            "expected the gross hidden-state mismatch to miss the strict reduced-equation acceptance gate"
        );
        assert!(
            !Engine::vbic_continuation_candidate_is_acceptable(
                &bjt,
                [vc, vb, ve, vs],
                &previous_snapshot,
                &candidate_snapshot,
                &candidate_linearization,
            ),
            "expected continuation acceptance to reject a candidate that is neither strictly solved nor ngspice-style converged"
        );
    }

    #[test]
    fn test_vbic_homotopy_candidate_accepts_ngspice_style_snapshot_convergence() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let previous_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let previous_internal = previous_snapshot.reduction.internal_voltages;
        let mut shifted_internal = previous_internal;
        shifted_internal[BJT_DELAY_XF2_STATE_INDEX] += 5.0e-7;
        let candidate_snapshot =
            bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, shifted_internal);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;
        let q_prev = previous_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let candidate_linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &candidate_snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble shifted VBIC homotopy candidate");

        assert!(
            !Engine::vbic_dynamic_snapshot_solution_is_acceptable(
                &candidate_linearization,
                &candidate_snapshot.reduction.external_voltages,
                &candidate_snapshot.reduction.internal_voltages,
            ),
            "expected the shifted hidden state to miss the strict reduced-equation acceptance gate"
        );
        assert!(Engine::vbic_homotopy_candidate_is_acceptable(
            &bjt,
            [vc, vb, ve, vs],
            previous_internal,
            &candidate_snapshot,
            &candidate_linearization,
        ));
    }

    #[test]
    fn test_vbic_homotopy_candidate_rejects_gross_hidden_state_mismatch() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let previous_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let previous_internal = previous_snapshot.reduction.internal_voltages;
        let mut shifted_internal = previous_internal;
        shifted_internal[BJT_DELAY_XF2_STATE_INDEX] += 5.0e-3;
        let candidate_snapshot =
            bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, shifted_internal);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;
        let q_prev = previous_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let candidate_linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &candidate_snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble grossly shifted VBIC homotopy candidate");

        assert!(
            !Engine::vbic_dynamic_snapshot_solution_is_acceptable(
                &candidate_linearization,
                &candidate_snapshot.reduction.external_voltages,
                &candidate_snapshot.reduction.internal_voltages,
            ),
            "expected the gross hidden-state mismatch to miss the strict reduced-equation acceptance gate"
        );
        assert!(
            !Engine::vbic_homotopy_candidate_is_acceptable(
                &bjt,
                [vc, vb, ve, vs],
                previous_internal,
                &candidate_snapshot,
                &candidate_linearization,
            ),
            "expected homotopy acceptance to reject a candidate that is neither strictly solved nor ngspice-style converged"
        );
    }

    #[test]
    fn test_transient_static_device_convergence_skips_excess_phase_vbic_bjts() {
        let netlist = vbic_focus_test_netlist(2e-11);
        let engine = Engine::default();
        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("build VBIC focus circuit");
        let mut solution = vec![0.0; circuit.matrix_size()];
        let collector_node = circuit.get_node_by_name("C").expect("find collector node");
        let base_node = circuit.get_node_by_name("B").expect("find base node");
        solution[collector_node - 1] = 4.1;
        solution[base_node - 1] = 0.75;

        circuit.update_nonlinear(&solution);
        let mut perturbed_solution = solution.clone();
        perturbed_solution[base_node - 1] += 5.0e-2;
        circuit.update_nonlinear(&perturbed_solution);

        let criteria = engine.device_convergence_criteria();
        assert!(
            !circuit.bjts.devices[0].is_converged(criteria),
            "expected static VBIC convergence to fail after perturbing the excess-phase deck"
        );
        assert!(
            engine.transient_static_device_convergence_met(&circuit, true),
            "expected transient static device convergence to defer excess-phase VBIC BJTs to the hidden-state check"
        );
        assert!(
            !engine.transient_static_device_convergence_met(&circuit, false),
            "expected the unfiltered static device convergence path to fail for the perturbed excess-phase VBIC BJT"
        );
    }

    #[test]
    fn test_transient_static_device_convergence_keeps_td_zero_vbic_bjts_in_static_check() {
        let netlist = vbic_focus_test_netlist(0.0);
        let engine = Engine::default();
        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("build VBIC focus circuit");
        let mut solution = vec![0.0; circuit.matrix_size()];
        let collector_node = circuit.get_node_by_name("C").expect("find collector node");
        let base_node = circuit.get_node_by_name("B").expect("find base node");
        solution[collector_node - 1] = 4.1;
        solution[base_node - 1] = 0.75;

        circuit.update_nonlinear(&solution);
        let mut perturbed_solution = solution.clone();
        perturbed_solution[base_node - 1] += 5.0e-2;
        circuit.update_nonlinear(&perturbed_solution);

        let criteria = engine.device_convergence_criteria();
        assert!(
            !circuit.bjts.devices[0].is_converged(criteria),
            "expected static VBIC convergence to fail after perturbing the TD=0 deck"
        );
        assert!(
            !engine.transient_static_device_convergence_met(&circuit, true),
            "expected TD=0 VBIC BJTs to remain in the ordinary static device convergence path"
        );
    }

    #[test]
    fn test_vbic_dynamic_internal_seed_from_history_restores_excess_phase_state() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let dt = 1e-11;
        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let mut history_prev = live_seed;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] += 1.5e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] -= 2.0e-6;

        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            None,
            dt,
            0.0,
        );

        let expected = expected_vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            &history_prev,
            None,
            dt,
            0.0,
        );
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (merged_seed[idx] - expected[idx]).abs() < 1e-18,
                "expected excess-phase history seed to follow the ngspice-style anchored predictor at index {idx}"
            );
        }
    }

    fn expected_vbic_dynamic_internal_seed_from_history(
        bjt: &crate::device::Bjt,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        history_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_prev_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        dt: Value,
        prev_dt: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let mut expected = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        if bjt.uses_vbic_dynamic_charges() && bjt.td > 0.0 {
            expected[BJT_DELAY_XF1_STATE_INDEX] = history_prev[BJT_DELAY_XF1_STATE_INDEX];
            expected[BJT_DELAY_XF2_STATE_INDEX] = Engine::predict_transient_history_value(
                history_prev[BJT_DELAY_XF2_STATE_INDEX],
                history_prev_prev
                    .map(|history_prev_prev| history_prev_prev[BJT_DELAY_XF2_STATE_INDEX]),
                dt,
                prev_dt,
            );
        }
        if bjt.has_vbic_self_heating() {
            expected[BJT_THERMAL_STATE_INDEX] = Engine::predict_transient_history_value(
                history_prev[BJT_THERMAL_STATE_INDEX],
                history_prev_prev
                    .map(|history_prev_prev| history_prev_prev[BJT_THERMAL_STATE_INDEX]),
                dt,
                prev_dt,
            )
            .max(bjt.minimum_thermal_rise());
        }

        bjt.limit_vbic_dynamic_internal_state_to_previous(expected, *history_prev)
    }

    fn expected_vbic_dynamic_internal_seed_from_linear_history(
        bjt: &crate::device::Bjt,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        history_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_prev_prev: Option<&[Value; BJT_INTERNAL_STATE_DIM]>,
        history_linear_prev: &VbicPredictorLinearBranchState,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        dt: Value,
        prev_dt: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let mut expected = Engine::vbic_dynamic_internal_seed_from_predicted_linear_history(
            bjt,
            target_external,
            history_prev,
            history_linear_prev,
            history_linear_prev_prev,
            dt,
            prev_dt,
        )
        .or_else(|| {
            Engine::vbic_dynamic_internal_seed_from_linear_history(
                bjt,
                target_external,
                history_prev,
                history_linear_prev,
            )
        })
        .expect("build expected VBIC linear-history seed");
        if bjt.uses_vbic_dynamic_charges() && bjt.td > 0.0 {
            expected[BJT_DELAY_XF1_STATE_INDEX] = history_prev[BJT_DELAY_XF1_STATE_INDEX];
            expected[BJT_DELAY_XF2_STATE_INDEX] = Engine::predict_transient_history_value(
                history_prev[BJT_DELAY_XF2_STATE_INDEX],
                history_prev_prev
                    .map(|history_prev_prev| history_prev_prev[BJT_DELAY_XF2_STATE_INDEX]),
                dt,
                prev_dt,
            );
        }
        if bjt.has_vbic_self_heating() {
            expected[BJT_THERMAL_STATE_INDEX] = Engine::predict_transient_history_value(
                history_prev[BJT_THERMAL_STATE_INDEX],
                history_prev_prev
                    .map(|history_prev_prev| history_prev_prev[BJT_THERMAL_STATE_INDEX]),
                dt,
                prev_dt,
            )
            .max(bjt.minimum_thermal_rise());
        }
        bjt.limit_vbic_dynamic_internal_state_to_previous(expected, *history_prev)
    }

    #[test]
    fn test_vbic_dynamic_internal_seed_from_history_keeps_static_core_on_live_bias() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let dt = 2.0e-11;
        let prev_dt = 1.0e-11;
        let mut history_prev = live_seed;
        let mut history_prev_prev = live_seed;
        history_prev[BJT_VCI_STATE_INDEX] += 4.0e-3;
        history_prev_prev[BJT_VCI_STATE_INDEX] += 1.5e-3;
        history_prev[BJT_VBI_STATE_INDEX] += 2.0e-3;
        history_prev_prev[BJT_VBI_STATE_INDEX] -= 5.0e-4;
        history_prev[BJT_VEI_STATE_INDEX] -= 3.0e-3;
        history_prev_prev[BJT_VEI_STATE_INDEX] -= 1.0e-3;

        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            dt,
            prev_dt,
        );

        let expected = expected_vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            &history_prev,
            Some(&history_prev_prev),
            dt,
            prev_dt,
        );
        for idx in [
            BJT_VCI_STATE_INDEX,
            BJT_VBI_STATE_INDEX,
            BJT_VEI_STATE_INDEX,
        ] {
            assert!(
                (merged_seed[idx] - expected[idx]).abs() < 1e-18,
                "expected reduced VBIC seed to keep the live-bias static core at index {idx}"
            );
            assert!(
                (merged_seed[idx] - live_seed[idx]).abs() < 1e-18,
                "expected reduced VBIC predictor to leave the live-bias static core unchanged at index {idx}"
            );
        }
    }

    #[test]
    fn test_vbic_dynamic_internal_seed_from_history_limits_large_predicted_dynamic_jumps() {
        let (mut bjt, vc, vb, ve, vs) = vbic_self_heated_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let dt = 2.0e-11;
        let prev_dt = 1.0e-11;
        let mut history_prev = live_seed;
        let mut history_prev_prev = live_seed;
        history_prev[BJT_VCI_STATE_INDEX] += 0.55;
        history_prev_prev[BJT_VCI_STATE_INDEX] -= 0.35;
        history_prev[BJT_VBI_STATE_INDEX] += 0.80;
        history_prev_prev[BJT_VBI_STATE_INDEX] -= 0.25;
        history_prev[BJT_VEI_STATE_INDEX] -= 0.30;
        history_prev_prev[BJT_VEI_STATE_INDEX] += 0.15;
        history_prev[BJT_THERMAL_STATE_INDEX] += 180.0;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] -= 40.0;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] += 1.25e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] = 8.0e-6;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] = 1.5e-6;

        let expected = expected_vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            &history_prev,
            Some(&history_prev_prev),
            dt,
            prev_dt,
        );
        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            dt,
            prev_dt,
        );

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (merged_seed[idx] - expected[idx]).abs() < 1e-18,
                "expected ngspice-style VBIC predictor limiting at index {idx}"
            );
        }
    }

    #[test]
    fn test_vbic_dynamic_internal_seed_from_history_anchors_xf1_and_predicts_xf2_from_two_level_history()
     {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let dt = 2.0e-11;
        let prev_dt = 1.0e-11;
        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let mut history_prev = live_seed;
        let mut history_prev_prev = live_seed;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] += 1.0e-6;
        history_prev_prev[BJT_DELAY_XF1_STATE_INDEX] -= 5.0e-7;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] = 6.0e-6;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] = 2.0e-6;

        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            dt,
            prev_dt,
        );

        let expected = expected_vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            &history_prev,
            Some(&history_prev_prev),
            dt,
            prev_dt,
        );
        assert!(
            (merged_seed[BJT_DELAY_XF1_STATE_INDEX] - expected[BJT_DELAY_XF1_STATE_INDEX]).abs()
                < 1e-18,
            "expected reduced VBIC predictor to keep xf1 anchored to the accepted excess-phase state"
        );
        assert!(
            (merged_seed[BJT_DELAY_XF2_STATE_INDEX] - expected[BJT_DELAY_XF2_STATE_INDEX]).abs()
                < 1e-18,
            "expected reduced VBIC predictor to extrapolate xf2 from accepted excess-phase history"
        );
        assert!(
            (merged_seed[BJT_DELAY_XF1_STATE_INDEX] - history_prev[BJT_DELAY_XF1_STATE_INDEX])
                .abs()
                < 1e-18,
            "expected the xf1 seed to remain pinned to the accepted xf1 state"
        );
        assert!(
            (merged_seed[BJT_DELAY_XF2_STATE_INDEX] - history_prev[BJT_DELAY_XF2_STATE_INDEX])
                .abs()
                > 1e-9,
            "expected the xf2 predictor case to move materially away from the accepted xf2 state"
        );
    }

    #[test]
    fn test_vbic_dynamic_internal_seed_from_history_predicts_self_heating_state() {
        let (mut bjt, vc, vb, ve, vs) = vbic_self_heated_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let dt = 2.0e-11;
        let prev_dt = 1.0e-11;
        let mut history_prev = live_seed;
        let mut history_prev_prev = live_seed;
        history_prev[BJT_THERMAL_STATE_INDEX] += 7.5;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] += 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] = 6.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] += 2.5;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] = 2.0e-6;

        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&history_prev),
            Some(&history_prev_prev),
            dt,
            prev_dt,
        );

        let expected = expected_vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            &history_prev,
            Some(&history_prev_prev),
            dt,
            prev_dt,
        );
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (merged_seed[idx] - expected[idx]).abs() < 1e-18,
                "expected merged self-heated VBIC seed to follow ngspice-style history prediction at index {idx}"
            );
        }
        assert!(
            (merged_seed[BJT_THERMAL_STATE_INDEX] - live_seed[BJT_THERMAL_STATE_INDEX]).abs()
                > 1e-6,
            "expected the thermal predictor seed to move away from the bias-resolved live seed"
        );
    }

    #[test]
    fn test_vbic_dynamic_internal_seed_from_history_with_linear_history_projects_static_core() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let dt = 1.0e-11;
        let target_external = [vc, vb, ve, vs];
        let history_external = [vc + 3.0e-2, vb + 2.5e-2, ve - 1.0e-2, vs];
        let history_snapshot = bjt.charge_snapshot(
            history_external[0],
            history_external[1],
            history_external[2],
            history_external[3],
        );
        let history_prev = history_snapshot.reduction.internal_voltages;
        let history_linear_prev =
            Engine::vbic_predictor_linear_branch_state(&bjt, history_external, history_prev);

        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history_with_linear_history(
            &bjt,
            target_external[0],
            target_external[1],
            target_external[2],
            target_external[3],
            Some(&history_prev),
            None,
            Some(&history_linear_prev),
            None,
            dt,
            0.0,
        );
        let expected = expected_vbic_dynamic_internal_seed_from_linear_history(
            &bjt,
            target_external,
            &history_prev,
            None,
            &history_linear_prev,
            None,
            dt,
            0.0,
        );
        let live_seed = bjt.dynamic_internal_state_seed(
            target_external[0],
            target_external[1],
            target_external[2],
            target_external[3],
        );

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (merged_seed[idx] - expected[idx]).abs() < 1e-18,
                "expected linear-history VBIC seed to match the projected predictor at index {idx}"
            );
        }
        let static_delta_from_live = (0..BJT_THERMAL_STATE_INDEX)
            .map(|idx| (merged_seed[idx] - live_seed[idx]).abs())
            .fold(0.0_f64, Value::max);
        assert!(
            static_delta_from_live > 1e-6,
            "expected linear-history seeding to move the static VBIC core away from the live-bias anchor when accepted history is available"
        );
    }

    #[test]
    fn test_vbic_dynamic_internal_seed_from_history_with_linear_history_preserves_dynamic_predictor()
     {
        let (mut bjt, vc, vb, ve, vs) = vbic_self_heated_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let dt = 2.0e-11;
        let prev_dt = 1.0e-11;
        let target_external = [vc, vb, ve, vs];
        let history_external = [vc + 2.0e-2, vb + 1.5e-2, ve - 8.0e-3, vs];
        let history_snapshot = bjt.charge_snapshot(
            history_external[0],
            history_external[1],
            history_external[2],
            history_external[3],
        );
        let mut history_prev = history_snapshot.reduction.internal_voltages;
        history_prev[BJT_DELAY_XF1_STATE_INDEX] += 1.0e-6;
        history_prev[BJT_DELAY_XF2_STATE_INDEX] = 6.0e-6;
        history_prev[BJT_THERMAL_STATE_INDEX] += 7.5;
        let mut history_prev_prev = history_prev;
        history_prev_prev[BJT_DELAY_XF1_STATE_INDEX] -= 5.0e-7;
        history_prev_prev[BJT_DELAY_XF2_STATE_INDEX] = 2.0e-6;
        history_prev_prev[BJT_THERMAL_STATE_INDEX] += 2.5;
        let history_linear_prev =
            Engine::vbic_predictor_linear_branch_state(&bjt, history_external, history_prev);
        let history_linear_prev_prev =
            Engine::vbic_predictor_linear_branch_state(&bjt, history_external, history_prev_prev);

        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history_with_linear_history(
            &bjt,
            target_external[0],
            target_external[1],
            target_external[2],
            target_external[3],
            Some(&history_prev),
            Some(&history_prev_prev),
            Some(&history_linear_prev),
            Some(&history_linear_prev_prev),
            dt,
            prev_dt,
        );
        let expected = expected_vbic_dynamic_internal_seed_from_linear_history(
            &bjt,
            target_external,
            &history_prev,
            Some(&history_prev_prev),
            &history_linear_prev,
            Some(&history_linear_prev_prev),
            dt,
            prev_dt,
        );

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (merged_seed[idx] - expected[idx]).abs() < 1e-18,
                "expected linear-history VBIC seed to preserve dynamic predictor behavior at index {idx}"
            );
        }
        assert!(
            (merged_seed[BJT_DELAY_XF1_STATE_INDEX] - history_prev[BJT_DELAY_XF1_STATE_INDEX])
                .abs()
                < 1e-18,
            "expected linear-history VBIC predictor to keep xf1 pinned to accepted history"
        );
        assert!(
            (merged_seed[BJT_DELAY_XF2_STATE_INDEX] - history_prev[BJT_DELAY_XF2_STATE_INDEX])
                .abs()
                > 1e-9,
            "expected linear-history VBIC predictor to extrapolate xf2 from accepted history"
        );
        assert!(
            (merged_seed[BJT_THERMAL_STATE_INDEX] - history_prev[BJT_THERMAL_STATE_INDEX]).abs()
                > 1e-6,
            "expected linear-history VBIC predictor to extrapolate the thermal state from accepted history"
        );
    }

    #[test]
    fn test_vbic_transient_owning_charge_branch_ignores_qbe_cross_coupling() {
        let (bjt, vc, vb, ve, vs) = vbic_self_heated_focus_test_bjt();
        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let qbe = snapshot.branches[0];

        assert!(
            qbe.d_internal[BJT_VCI_STATE_INDEX].abs() > 0.0,
            "expected the full Qbe Jacobian to include Vbci coupling before transient projection"
        );
        assert!(
            qbe.d_internal[BJT_THERMAL_STATE_INDEX].abs() > 0.0,
            "expected the full Qbe Jacobian to include self-heating coupling before transient projection"
        );

        let companion = Engine::vbic_transient_owning_charge_branch(&bjt, 0, &qbe)
            .expect("build ngspice-style Qbe transient companion");

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            let expected_nonzero = idx == BJT_VBI_STATE_INDEX || idx == BJT_VEI_STATE_INDEX;
            let value = companion.d_internal[idx];
            if expected_nonzero {
                assert!(
                    value.abs() > 0.0,
                    "expected Qbe transient companion to keep the owning Vbei derivative at index {idx}"
                );
            } else {
                assert!(
                    value.abs() < 1e-18,
                    "expected Qbe transient companion to drop non-owning derivative at index {idx}, got {value:.3e}"
                );
            }
        }
        for idx in 0..BJT_EXTERNAL_STATE_DIM {
            assert!(
                companion.d_external[idx].abs() < 1e-18,
                "expected internal Qbe transient companion to avoid external derivative at index {idx}"
            );
        }
        assert!(
            (companion.d_internal[BJT_VBI_STATE_INDEX] + companion.d_internal[BJT_VEI_STATE_INDEX])
                .abs()
                < 1e-18,
            "expected Qbe transient companion to remain a pure two-terminal branch"
        );
    }

    #[test]
    fn test_vbic_transient_owning_charge_branch_keeps_pnp_matrix_stamp_positive() {
        let (bjt, vc, vb, ve, vs) = vbic_self_heated_pnp_diffamp_test_bjt();
        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let qbe = snapshot.branches[0];
        let p = match bjt.bjt_type {
            crate::device::BjtType::Npn => 1.0,
            crate::device::BjtType::Pnp => -1.0,
        };
        let expected = -p * qbe.d_internal[BJT_VEI_STATE_INDEX];

        assert!(
            expected > 0.0,
            "expected the PNP Qbe owning derivative to reduce to a positive scalar capacitance"
        );

        let companion = Engine::vbic_transient_owning_charge_branch(&bjt, 0, &qbe)
            .expect("build ngspice-style PNP Qbe transient companion");

        assert!(
            (companion.d_internal[BJT_VBI_STATE_INDEX] - expected).abs() < 1e-18,
            "expected the PNP Qbe transient companion to stamp +dq/dv on the positive node"
        );
        assert!(
            (companion.d_internal[BJT_VEI_STATE_INDEX] + expected).abs() < 1e-18,
            "expected the PNP Qbe transient companion to stamp -dq/dv on the negative node"
        );
        assert!(
            companion.d_internal[BJT_VBI_STATE_INDEX] > 0.0
                && companion.d_internal[BJT_VEI_STATE_INDEX] < 0.0,
            "expected the PNP Qbe transient companion to keep ngspice's positive two-terminal matrix orientation"
        );
        for idx in 0..BJT_EXTERNAL_STATE_DIM {
            assert!(
                companion.d_external[idx].abs() < 1e-18,
                "expected internal PNP Qbe transient companion to avoid external derivative at index {idx}"
            );
        }
        assert!(
            (companion.d_internal[BJT_VBI_STATE_INDEX] + companion.d_internal[BJT_VEI_STATE_INDEX])
                .abs()
                < 1e-18,
            "expected the PNP Qbe transient companion to remain a pure two-terminal branch"
        );
    }

    #[test]
    fn test_vbic_truncation_cache_reuses_matching_external_snapshot() {
        let (bjt, vc, vb, ve, vs) = vbic_self_heated_pnp_diffamp_test_bjt();
        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let within_tolerance = [vc + 5e-7, vb - 5e-7, ve + 5e-7, vs - 5e-7];
        let outside_tolerance = [vc + 1e-2, vb, ve, vs];

        assert!(
            Engine::check_voltage_convergence_with_tolerances(
                &snapshot.reduction.external_voltages,
                &within_tolerance,
                1e-6,
                1e-3,
            ),
            "expected a cached VBIC snapshot to be reusable when the converged external voltages remain within solver tolerance"
        );
        assert!(
            !Engine::check_voltage_convergence_with_tolerances(
                &snapshot.reduction.external_voltages,
                &outside_tolerance,
                1e-6,
                1e-3,
            ),
            "expected a cached VBIC snapshot to be recomputed when the converged external voltages move outside solver tolerance"
        );
    }

    #[test]
    fn test_vbic_history_snapshot_resolution_rebuilds_nearby_stale_cache() {
        let (bjt, vc, vb, ve, vs) = vbic_self_heated_pnp_diffamp_test_bjt();
        let cached_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let accepted_external = [vc + 5e-4, vb - 5e-4, ve + 5e-4, vs - 5e-4];

        assert!(
            Engine::vbic_snapshot_matches_external_bias(
                &cached_snapshot,
                &accepted_external,
                1e-6,
                1e-3,
            ),
            "expected the stale cache to still fall within ordinary solver reuse tolerances"
        );
        assert!(
            !Engine::vbic_snapshot_matches_external_bias(
                &cached_snapshot,
                &accepted_external,
                VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL,
                VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL,
            ),
            "expected accepted-step history reuse to reject caches that do not match the committed external bias closely enough"
        );

        let q_prev = cached_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let resolved_snapshot = Engine::resolve_vbic_snapshot_for_external_bias(
            &bjt,
            accepted_external,
            IntegrationMethod::Trapezoidal,
            2,
            1e-11,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&cached_snapshot.reduction.internal_voltages),
            Some(&cached_snapshot.reduction.internal_voltages),
            1e-11,
            Some(cached_snapshot),
            VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL,
            VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL,
        )
        .expect("resolve accepted-step VBIC snapshot");

        for idx in 0..BJT_EXTERNAL_STATE_DIM {
            assert!(
                (resolved_snapshot.reduction.external_voltages[idx] - accepted_external[idx]).abs()
                    < 1e-18,
                "expected resolved history snapshot to match the accepted external bias at index {idx}"
            );
        }

        let max_charge_delta = resolved_snapshot
            .branches
            .iter()
            .zip(cached_snapshot.branches.iter())
            .map(|(resolved, cached)| (resolved.charge - cached.charge).abs())
            .fold(0.0, Value::max);
        assert!(
            max_charge_delta > 1e-18,
            "expected rebuilding the accepted-step snapshot to change at least one stored dynamic charge; max delta was {max_charge_delta:.3e}"
        );
    }

    #[test]
    fn test_vbic_history_snapshot_resolution_uses_cache_when_history_is_invalid() {
        let (circuit, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_circuit();
        let bjt = circuit.bjts.devices[0].clone();
        let dt = 1e-11;
        let trap_order = 2;
        let method = IntegrationMethod::Trapezoidal;
        let cached_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let target_external = [vc + 1.0e-4, vb - 2.0e-4, ve + 5.0e-5, vs];
        let nan_internal = [Value::NAN; BJT_INTERNAL_STATE_DIM];
        let nan_linear = VbicPredictorLinearBranchState {
            vrcx: Value::NAN,
            vrci: Value::NAN,
            vrbx: Value::NAN,
            vrbi: Value::NAN,
            vre: Value::NAN,
            vrbp: Value::NAN,
            vrs: Value::NAN,
        };
        let q_prev = cached_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];

        let with_cache = Engine::resolve_vbic_snapshot_for_external_bias_with_linear_history(
            &bjt,
            target_external,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&nan_internal),
            Some(&nan_internal),
            Some(&nan_linear),
            Some(&nan_linear),
            dt,
            Some(cached_snapshot),
            VbicCachedSnapshotReuse::NewtonBypass,
            VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL,
            VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL,
        )
        .expect("resolve VBIC snapshot from cached continuation");

        for (idx, expected) in target_external.iter().enumerate() {
            assert!(
                (with_cache.reduction.external_voltages[idx] - *expected).abs() < 1e-18,
                "expected cached VBIC continuation to land on the target external bias at index {idx}"
            );
        }
        assert!(
            with_cache
                .reduction
                .internal_voltages
                .iter()
                .all(|value| value.is_finite()),
            "expected cached VBIC continuation to produce a finite internal state"
        );
    }

    #[test]
    fn test_vbic_diffamp_q11_runtime_bias_snapshot_solves_from_history_seed() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.temperature = 300.15;
        let engine = Engine::new(config);
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");

        let q11 = circuit
            .bjts
            .devices
            .iter()
            .find(|device| device.name == "Q11")
            .expect("find Q11")
            .clone();
        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 5.893e-21;
        let cached_external = [6.584_984e-1, 6.584_984e-1, 0.0, 0.0];
        let target_external = [6.585_012e-1, 6.585_012e-1, 0.0, 0.0];
        let cached_snapshot = q11.charge_snapshot(
            cached_external[0],
            cached_external[1],
            cached_external[2],
            cached_external[3],
        );
        let q_prev = cached_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let history_internal_prev = cached_snapshot.reduction.internal_voltages;
        let history_linear_prev = Engine::vbic_predictor_linear_branch_state(
            &q11,
            cached_snapshot.reduction.external_voltages,
            cached_snapshot.reduction.internal_voltages,
        );

        let resolved_snapshot =
            Engine::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                &q11,
                target_external,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&history_internal_prev),
                Some(&history_internal_prev),
                Some(&history_linear_prev),
                Some(&history_linear_prev),
                dt,
                Some(cached_snapshot),
                VbicCachedSnapshotReuse::SeedOnly,
                VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL,
                VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL,
            )
            .expect("resolve Q11 runtime-bias VBIC snapshot");

        for (idx, expected) in target_external.iter().enumerate() {
            assert!(
                (resolved_snapshot.reduction.external_voltages[idx] - *expected).abs() < 1e-18,
                "expected resolved Q11 snapshot to land on the runtime external bias at index {idx}"
            );
        }
        let linearization = Engine::assemble_vbic_transient_linearization(
            &q11,
            &resolved_snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble Q11 runtime-bias transient linearization");
        let residual_norm = Engine::vbic_internal_equation_residual_norm(
            &linearization,
            &resolved_snapshot.reduction.external_voltages,
            &resolved_snapshot.reduction.internal_voltages,
        );
        assert!(
            residual_norm.is_finite()
                && Engine::vbic_dynamic_snapshot_solution_is_acceptable(
                    &linearization,
                    &resolved_snapshot.reduction.external_voltages,
                    &resolved_snapshot.reduction.internal_voltages,
                ),
            "expected Q11 runtime-bias snapshot to satisfy the reduced VBIC hidden-state equations, got residual {residual_norm:.12e}"
        );
    }

    #[test]
    fn test_vbic_snapshot_resolution_reuses_locally_acceptable_cached_internal_state() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;
        let cached_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let q_prev = cached_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let target_external = [vc + 5.0e-5, vb - 5.0e-5, ve, vs];

        let candidate_snapshot = bjt.charge_snapshot_for_dynamic_state(
            target_external[0],
            target_external[1],
            target_external[2],
            target_external[3],
            cached_snapshot.reduction.internal_voltages,
        );
        let candidate_linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &candidate_snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble locally acceptable cached-state candidate");
        assert!(
            Engine::vbic_local_candidate_is_acceptable(
                &bjt,
                cached_snapshot.reduction.external_voltages,
                &cached_snapshot,
                &candidate_snapshot,
                &candidate_linearization,
            ),
            "expected the cached VBIC hidden state to remain locally acceptable across a small external perturbation"
        );

        let resolved_snapshot =
            Engine::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                &bjt,
                target_external,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&cached_snapshot.reduction.internal_voltages),
                None,
                None,
                None,
                dt,
                Some(cached_snapshot),
                VbicCachedSnapshotReuse::NewtonBypass,
                1e-6,
                1e-3,
            )
            .expect("resolve locally acceptable cached-state snapshot");

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (resolved_snapshot.reduction.internal_voltages[idx]
                    - candidate_snapshot.reduction.internal_voltages[idx])
                    .abs()
                    < 1e-18,
                "expected locally acceptable cached-state resolution to preserve internal index {idx}"
            );
        }
    }

    #[test]
    fn test_vbic_diffamp_q11_continuation_seed_predicts_and_preserves_delay_state() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.temperature = 300.15;
        let engine = Engine::new(config);
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");

        let q11 = circuit
            .bjts
            .devices
            .iter()
            .find(|device| device.name == "Q11")
            .expect("find Q11")
            .clone();

        let history_external = [6.597_288e-1, 6.597_288e-1, 0.0, 0.0];
        let target_external = [4.584_418e-1, 4.584_418e-1, 0.0, 0.0];
        let previous_snapshot = q11.charge_snapshot(
            history_external[0],
            history_external[1],
            history_external[2],
            history_external[3],
        );
        let previous_internal = previous_snapshot.reduction.internal_voltages;
        let step = Engine::vbic_continuation_step_from_snapshot(
            &q11,
            history_external,
            previous_internal,
            target_external,
        );
        let next_external = [
            history_external[0] + (target_external[0] - history_external[0]) * step,
            history_external[1] + (target_external[1] - history_external[1]) * step,
            history_external[2] + (target_external[2] - history_external[2]) * step,
            history_external[3] + (target_external[3] - history_external[3]) * step,
        ];
        let predicted_seed = Engine::vbic_continuation_seed_from_snapshot(
            &q11,
            history_external,
            previous_internal,
            next_external,
        );

        assert!(
            q11.vbic_dynamic_internal_state_within_local_branch_envelope(
                predicted_seed,
                previous_internal,
            ),
            "expected the continuation seed to stay inside the local VBIC branch limiter envelope"
        );
        assert!(
            (predicted_seed[BJT_DELAY_XF1_STATE_INDEX]
                - previous_internal[BJT_DELAY_XF1_STATE_INDEX])
                .abs()
                < 1e-18,
            "expected Q11 continuation seeding to preserve xf1 across an external-only continuation step"
        );
        assert!(
            (predicted_seed[BJT_DELAY_XF2_STATE_INDEX]
                - previous_internal[BJT_DELAY_XF2_STATE_INDEX])
                .abs()
                < 1e-18,
            "expected Q11 continuation seeding to preserve xf2 across an external-only continuation step"
        );

        let static_delta = (0..BJT_STATIC_CORE_STATE_DIM)
            .map(|idx| (predicted_seed[idx] - previous_internal[idx]).abs())
            .fold(0.0_f64, Value::max);
        assert!(
            static_delta > 0.0,
            "expected the continuation seed to predict a new static VBIC core for the next external step"
        );
    }

    #[test]
    fn test_vbic_continuation_seed_from_accepted_path_extrapolates_static_core() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let previous_external = [vc + 4.0e-3, vb + 4.0e-3, ve, vs];
        let current_external = [vc + 2.0e-3, vb + 2.0e-3, ve, vs];
        let target_external = [vc, vb, ve, vs];
        let previous_internal = bjt
            .charge_snapshot(
                previous_external[0],
                previous_external[1],
                previous_external[2],
                previous_external[3],
            )
            .reduction
            .internal_voltages;
        let current_internal = bjt
            .charge_snapshot(
                current_external[0],
                current_external[1],
                current_external[2],
                current_external[3],
            )
            .reduction
            .internal_voltages;

        let baseline = Engine::vbic_continuation_seed_from_snapshot(
            &bjt,
            current_external,
            current_internal,
            target_external,
        );
        let anchored = Engine::vbic_continuation_seed_from_accepted_path(
            &bjt,
            Some(previous_external),
            Some(previous_internal),
            current_external,
            current_internal,
            target_external,
        );
        let continuation_scale = 1.0;

        for idx in [
            BJT_VCI_STATE_INDEX,
            BJT_VBI_STATE_INDEX,
            BJT_VEI_STATE_INDEX,
        ] {
            let secant = current_internal[idx]
                + (current_internal[idx] - previous_internal[idx]) * continuation_scale;
            assert!(
                (anchored[idx] - secant).abs() <= (baseline[idx] - secant).abs() + 1e-18,
                "expected accepted-path VBIC continuation seeding to move static index {idx} closer to the secant predictor"
            );
            assert!(
                (anchored[idx] - current_internal[idx]).abs()
                    >= (baseline[idx] - current_internal[idx]).abs() - 1e-18,
                "expected accepted-path VBIC continuation seeding to take at least as much static motion at index {idx}"
            );
        }
        assert!(
            bjt.vbic_dynamic_internal_state_within_local_branch_envelope(
                anchored,
                current_internal
            ),
            "expected accepted-path VBIC continuation seeding to remain inside the local branch limiter envelope"
        );
    }

    #[test]
    fn test_vbic_continuation_seed_from_accepted_path_shrinks_with_tiny_followup_step() {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let previous_external = [vc + 4.0e-3, vb + 4.0e-3, ve, vs];
        let current_external = [vc + 2.0e-3, vb + 2.0e-3, ve, vs];
        let target_external = [vc + 1.999e-3, vb + 1.999e-3, ve, vs];
        let previous_internal = bjt
            .charge_snapshot(
                previous_external[0],
                previous_external[1],
                previous_external[2],
                previous_external[3],
            )
            .reduction
            .internal_voltages;
        let current_internal = bjt
            .charge_snapshot(
                current_external[0],
                current_external[1],
                current_external[2],
                current_external[3],
            )
            .reduction
            .internal_voltages;

        let anchored = Engine::vbic_continuation_seed_from_accepted_path(
            &bjt,
            Some(previous_external),
            Some(previous_internal),
            current_external,
            current_internal,
            target_external,
        );

        let previous_step = previous_external
            .iter()
            .zip(current_external.iter())
            .map(|(previous, current)| (current - previous).abs())
            .fold(0.0_f64, Value::max);
        let proposed_step = target_external
            .iter()
            .zip(current_external.iter())
            .map(|(target, current)| (target - current).abs())
            .fold(0.0_f64, Value::max);
        let expected_scale = proposed_step / previous_step;

        for idx in [
            BJT_VCI_STATE_INDEX,
            BJT_VBI_STATE_INDEX,
            BJT_VEI_STATE_INDEX,
        ] {
            let expected = current_internal[idx]
                + (current_internal[idx] - previous_internal[idx]) * expected_scale;
            let anchored_delta = (anchored[idx] - current_internal[idx]).abs();
            let previous_delta = (current_internal[idx] - previous_internal[idx]).abs();
            assert!(
                (anchored[idx] - expected).abs() <= previous_delta * 0.05 + 1e-18,
                "expected tiny-step accepted-path continuation seeding to track the scaled secant at static index {idx}"
            );
            assert!(
                anchored_delta <= previous_delta * 0.05 + 1e-18,
                "expected tiny-step accepted-path continuation seeding to stay close to the current accepted state at static index {idx}"
            );
        }
        assert!(
            bjt.vbic_dynamic_internal_state_within_local_branch_envelope(
                anchored,
                current_internal
            ),
            "expected tiny-step accepted-path continuation seeding to remain inside the local branch limiter envelope"
        );
    }

    #[test]
    fn test_vbic_diffamp_q11_large_runtime_jump_best_effort_candidate_keeps_bounded_residual() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.temperature = 300.15;
        let engine = Engine::new(config);
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");

        let q11 = circuit
            .bjts
            .devices
            .iter()
            .find(|device| device.name == "Q11")
            .expect("find Q11")
            .clone();

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 5.893e-21;
        let history_external = [6.597_288e-1, 6.597_288e-1, 0.0, 0.0];
        let target_external = [4.584_418e-1, 4.584_418e-1, 0.0, 0.0];

        let previous_snapshot = q11.charge_snapshot(
            history_external[0],
            history_external[1],
            history_external[2],
            history_external[3],
        );
        let q_prev = previous_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let history_internal_prev = previous_snapshot.reduction.internal_voltages;
        let history_linear_prev = Engine::vbic_predictor_linear_branch_state(
            &q11,
            previous_snapshot.reduction.external_voltages,
            previous_snapshot.reduction.internal_voltages,
        );
        let live_target_seed = q11.dynamic_internal_state_seed(
            target_external[0],
            target_external[1],
            target_external[2],
            target_external[3],
        );
        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history_with_linear_history(
            &q11,
            target_external[0],
            target_external[1],
            target_external[2],
            target_external[3],
            Some(&history_internal_prev),
            Some(&history_internal_prev),
            Some(&history_linear_prev),
            Some(&history_linear_prev),
            dt,
            dt,
        );

        let mut candidate = Engine::choose_preferred_vbic_best_effort_result(
            Engine::solve_vbic_dynamic_snapshot_best_effort(
                &q11,
                target_external[0],
                target_external[1],
                target_external[2],
                target_external[3],
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&history_internal_prev),
            ),
            Engine::solve_vbic_dynamic_snapshot_best_effort(
                &q11,
                target_external[0],
                target_external[1],
                target_external[2],
                target_external[3],
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&live_target_seed),
            ),
            |result| {
                Engine::vbic_continuation_candidate_is_acceptable(
                    &q11,
                    previous_snapshot.reduction.external_voltages,
                    &previous_snapshot,
                    &result.0,
                    &result.1,
                )
            },
        );
        candidate = Engine::choose_preferred_vbic_best_effort_result(
            candidate,
            Engine::solve_vbic_dynamic_snapshot_best_effort(
                &q11,
                target_external[0],
                target_external[1],
                target_external[2],
                target_external[3],
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&merged_seed),
            ),
            |result| {
                Engine::vbic_continuation_candidate_is_acceptable(
                    &q11,
                    previous_snapshot.reduction.external_voltages,
                    &previous_snapshot,
                    &result.0,
                    &result.1,
                )
            },
        );

        let (candidate_snapshot, candidate_linearization, _, residual_norm) = candidate.expect(
            "expected a best-effort Q11 runtime-bias candidate across the large history jump",
        );

        assert!(
            residual_norm.is_finite() && residual_norm < 5.0e-2,
            "expected the best-effort Q11 runtime-bias candidate to keep a bounded hidden-state residual, got {residual_norm:.12e}"
        );
        assert!(
            candidate_snapshot
                .reduction
                .internal_voltages
                .iter()
                .all(|value| value.is_finite()),
            "expected the best-effort Q11 runtime-bias candidate to preserve a finite internal state"
        );
        let linearization_residual = Engine::vbic_internal_equation_residual_norm(
            &candidate_linearization,
            &candidate_snapshot.reduction.external_voltages,
            &candidate_snapshot.reduction.internal_voltages,
        );
        assert!(
            linearization_residual.is_finite() && linearization_residual <= residual_norm + 1e-18,
            "expected the assembled Q11 transient linearization residual to stay consistent with the best-effort solve, got {linearization_residual:.12e} vs {residual_norm:.12e}"
        );
    }

    #[test]
    fn test_vbic_diffamp_q11_tiny_followup_continuation_step_reuses_current_internal_seed() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.temperature = 300.15;
        let engine = Engine::new(config);
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");

        let q11 = circuit
            .bjts
            .devices
            .iter()
            .find(|device| device.name == "Q11")
            .expect("find Q11")
            .clone();

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 5.893e-21;
        let history_external = [6.597_288e-1, 6.597_288e-1, 0.0, 0.0];
        let current_external = [6.594_596_634_445_468e-1, 6.594_596_634_445_468e-1, 0.0, 0.0];
        let target_external = [4.584_418e-1, 4.584_418e-1, 0.0, 0.0];
        let step = 4.974_682_263_877e-6;
        let next_external = [
            current_external[0] + (target_external[0] - current_external[0]) * step,
            current_external[1] + (target_external[1] - current_external[1]) * step,
            current_external[2] + (target_external[2] - current_external[2]) * step,
            current_external[3] + (target_external[3] - current_external[3]) * step,
        ];

        let history_snapshot = q11.charge_snapshot(
            history_external[0],
            history_external[1],
            history_external[2],
            history_external[3],
        );
        let q_prev = history_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];

        let current_snapshot = Engine::solve_vbic_dynamic_snapshot(
            &q11,
            current_external[0],
            current_external[1],
            current_external[2],
            current_external[3],
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&history_snapshot.reduction.internal_voltages),
        )
        .map(|(snapshot, _, _)| snapshot)
        .expect("solve current Q11 continuation snapshot");

        let seed_internal = Engine::vbic_continuation_seed_from_accepted_path(
            &q11,
            Some(history_external),
            Some(history_snapshot.reduction.internal_voltages),
            current_external,
            current_snapshot.reduction.internal_voltages,
            next_external,
        );

        let continued = Engine::solve_vbic_dynamic_snapshot_for_continuation_step(
            &q11,
            current_external,
            &current_snapshot,
            next_external[0],
            next_external[1],
            next_external[2],
            next_external[3],
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&seed_internal),
        );

        assert!(
            continued.is_some(),
            "expected the tiny Q11 follow-up continuation step to remain solvable from the accepted current internal state"
        );
    }

    #[test]
    fn test_vbic_snapshot_resolution_does_not_return_unacceptable_bounded_best_effort_for_large_q11_jump()
     {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.temperature = 300.15;
        let engine = Engine::new(config);
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");

        let q11 = circuit
            .bjts
            .devices
            .iter()
            .find(|device| device.name == "Q11")
            .expect("find Q11")
            .clone();

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 5.893e-21;
        let history_external = [6.597_288e-1, 6.597_288e-1, 0.0, 0.0];
        let target_external = [4.584_418e-1, 4.584_418e-1, 0.0, 0.0];

        let previous_snapshot = q11.charge_snapshot(
            history_external[0],
            history_external[1],
            history_external[2],
            history_external[3],
        );
        let q_prev = previous_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let history_internal_prev = previous_snapshot.reduction.internal_voltages;
        let history_linear_prev = Engine::vbic_predictor_linear_branch_state(
            &q11,
            previous_snapshot.reduction.external_voltages,
            previous_snapshot.reduction.internal_voltages,
        );
        let live_target_seed = q11.dynamic_internal_state_seed(
            target_external[0],
            target_external[1],
            target_external[2],
            target_external[3],
        );
        let merged_seed = Engine::vbic_dynamic_internal_seed_from_history_with_linear_history(
            &q11,
            target_external[0],
            target_external[1],
            target_external[2],
            target_external[3],
            Some(&history_internal_prev),
            Some(&history_internal_prev),
            Some(&history_linear_prev),
            Some(&history_linear_prev),
            dt,
            dt,
        );

        let bounded_best_effort = Engine::choose_preferred_vbic_best_effort_result(
            Engine::solve_vbic_dynamic_snapshot_best_effort(
                &q11,
                target_external[0],
                target_external[1],
                target_external[2],
                target_external[3],
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&history_internal_prev),
            ),
            Engine::solve_vbic_dynamic_snapshot_best_effort(
                &q11,
                target_external[0],
                target_external[1],
                target_external[2],
                target_external[3],
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&live_target_seed),
            ),
            Engine::vbic_dynamic_snapshot_best_effort_is_bounded,
        );
        let bounded_best_effort = Engine::choose_preferred_vbic_best_effort_result(
            bounded_best_effort,
            Engine::solve_vbic_dynamic_snapshot_best_effort(
                &q11,
                target_external[0],
                target_external[1],
                target_external[2],
                target_external[3],
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
                Some(&merged_seed),
            ),
            Engine::vbic_dynamic_snapshot_best_effort_is_bounded,
        )
        .expect("expected bounded best-effort Q11 candidate across large history jump");

        assert!(Engine::vbic_dynamic_snapshot_best_effort_is_bounded(
            &bounded_best_effort
        ));
        assert!(
            !Engine::vbic_dynamic_snapshot_solution_is_acceptable(
                &bounded_best_effort.1,
                &bounded_best_effort.0.reduction.external_voltages,
                &bounded_best_effort.0.reduction.internal_voltages,
            ),
            "expected the large-jump Q11 best-effort candidate to miss the strict reduced-equation acceptance gate"
        );
        assert!(
            !Engine::vbic_local_candidate_is_acceptable(
                &q11,
                previous_snapshot.reduction.external_voltages,
                &previous_snapshot,
                &bounded_best_effort.0,
                &bounded_best_effort.1,
            ),
            "expected the large-jump Q11 best-effort candidate to miss the ngspice-style local predictor gate"
        );

        let resolved = Engine::resolve_vbic_snapshot_for_external_bias_with_linear_history(
            &q11,
            target_external,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&history_internal_prev),
            Some(&history_internal_prev),
            Some(&history_linear_prev),
            Some(&history_linear_prev),
            dt,
            Some(previous_snapshot),
            VbicCachedSnapshotReuse::NewtonBypass,
            VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL,
            VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL,
        );

        if let Some(snapshot) = resolved {
            let linearization = Engine::assemble_vbic_transient_linearization(
                &q11,
                &snapshot,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
            )
            .expect("assemble resolved Q11 transient linearization");
            let strict = Engine::vbic_dynamic_snapshot_solution_is_acceptable(
                &linearization,
                &snapshot.reduction.external_voltages,
                &snapshot.reduction.internal_voltages,
            );
            let predictor_ok = Engine::vbic_local_candidate_is_acceptable(
                &q11,
                previous_snapshot.reduction.external_voltages,
                &previous_snapshot,
                &snapshot,
                &linearization,
            );
            assert!(
                strict || predictor_ok,
                "expected resolved Q11 snapshot to be either strictly solved or locally predictor-acceptable"
            );
        }
    }

    #[test]
    #[ignore]
    fn test_vbic_diffamp_q11_continuation_returns_target_snapshot_across_large_history_jump() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.temperature = 300.15;
        let engine = Engine::new(config);
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");

        let q11 = circuit
            .bjts
            .devices
            .iter()
            .find(|device| device.name == "Q11")
            .expect("find Q11")
            .clone();

        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 5.893e-21;
        let history_external = [6.597_288e-1, 6.597_288e-1, 0.0, 0.0];
        let target_external = [4.584_418e-1, 4.584_418e-1, 0.0, 0.0];

        let previous_snapshot = q11.charge_snapshot(
            history_external[0],
            history_external[1],
            history_external[2],
            history_external[3],
        );
        let q_prev = previous_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];

        let continued_snapshot = Engine::continue_vbic_snapshot_to_external_bias_from_snapshot(
            &q11,
            previous_snapshot,
            target_external,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("continue Q11 snapshot across large runtime bias jump");

        for (idx, expected) in target_external.iter().enumerate() {
            assert!(
                (continued_snapshot.reduction.external_voltages[idx] - *expected).abs() < 1e-18,
                "expected continued Q11 snapshot to land on the target runtime bias at index {idx}"
            );
        }

        let linearization = Engine::assemble_vbic_transient_linearization(
            &q11,
            &continued_snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble continued Q11 transient linearization");
        let residual_norm = Engine::vbic_internal_equation_residual_norm(
            &linearization,
            &continued_snapshot.reduction.external_voltages,
            &continued_snapshot.reduction.internal_voltages,
        );
        assert!(
            continued_snapshot
                .reduction
                .internal_voltages
                .iter()
                .all(|value| value.is_finite())
                && residual_norm.is_finite()
                && residual_norm < 5.0e-2,
            "expected continued Q11 snapshot to reach the target bias with a bounded hidden-state residual, got residual {residual_norm:.12e}"
        );
    }

    #[test]
    fn test_finalize_vbic_continuation_target_snapshot_keeps_target_when_strict_polish_is_unavailable()
     {
        let (mut bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        bjt_update_for_external_bias(&mut bjt, vc, vb, ve, vs);

        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let q_prev = snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];

        let finalized = Engine::finalize_vbic_continuation_target_snapshot(
            &bjt,
            snapshot,
            IntegrationMethod::Trapezoidal,
            2,
            0.0,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("preserve accepted continuation target snapshot");

        for idx in 0..BJT_EXTERNAL_STATE_DIM {
            assert!(
                (finalized.reduction.external_voltages[idx]
                    - snapshot.reduction.external_voltages[idx])
                    .abs()
                    < 1e-18,
                "expected finalized target snapshot to preserve external voltage at index {idx}"
            );
        }
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (finalized.reduction.internal_voltages[idx]
                    - snapshot.reduction.internal_voltages[idx])
                    .abs()
                    < 1e-18,
                "expected finalized target snapshot to preserve internal state at index {idx}"
            );
        }
    }

    #[test]
    #[ignore]
    fn debug_vbic_diffamp_q11_runtime_bias_best_effort_diagnostics() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.temperature = 300.15;
        let engine = Engine::new(config);
        let circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");

        let q11 = circuit
            .bjts
            .devices
            .iter()
            .find(|device| device.name == "Q11")
            .expect("find Q11")
            .clone();
        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 5.893e-21;
        let criteria = NonlinearConvergenceCriteria::default();

        for (history_external, target_external) in [
            (
                [6.585_012e-1, 6.585_012e-1, 0.0, 0.0],
                [6.585_012e-1, 6.585_012e-1, 0.0, 0.0],
            ),
            (
                [6.597_288e-1, 6.597_288e-1, 0.0, 0.0],
                [6.597_288e-1, 6.597_288e-1, 0.0, 0.0],
            ),
            (
                [4.584_418e-1, 4.584_418e-1, 0.0, 0.0],
                [4.584_418e-1, 4.584_418e-1, 0.0, 0.0],
            ),
            (
                [6.597_288e-1, 6.597_288e-1, 0.0, 0.0],
                [4.584_418e-1, 4.584_418e-1, 0.0, 0.0],
            ),
        ] {
            let live_seed = q11.dynamic_internal_state_seed(
                history_external[0],
                history_external[1],
                history_external[2],
                history_external[3],
            );
            let previous_snapshot = q11.charge_snapshot_for_dynamic_state(
                history_external[0],
                history_external[1],
                history_external[2],
                history_external[3],
                live_seed,
            );
            let q_prev = previous_snapshot.branches.map(|branch| branch.charge);
            let q_prev_prev = q_prev;
            let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
            let history_internal_prev = previous_snapshot.reduction.internal_voltages;
            let history_linear_prev = Engine::vbic_predictor_linear_branch_state(
                &q11,
                previous_snapshot.reduction.external_voltages,
                previous_snapshot.reduction.internal_voltages,
            );
            let live_target_seed = q11.dynamic_internal_state_seed(
                target_external[0],
                target_external[1],
                target_external[2],
                target_external[3],
            );
            let seed_internal = Engine::vbic_dynamic_internal_seed_from_history_with_linear_history(
                &q11,
                target_external[0],
                target_external[1],
                target_external[2],
                target_external[3],
                Some(&history_internal_prev),
                Some(&history_internal_prev),
                Some(&history_linear_prev),
                Some(&history_linear_prev),
                dt,
                dt,
            );

            for (label, candidate_seed) in [
                ("history", history_internal_prev),
                ("live", live_target_seed),
                ("merged", seed_internal),
            ] {
                let best_effort = Engine::solve_vbic_dynamic_snapshot_best_effort(
                    &q11,
                    target_external[0],
                    target_external[1],
                    target_external[2],
                    target_external[3],
                    method,
                    trap_order,
                    dt,
                    &q_prev,
                    &q_prev_prev,
                    &cq_prev,
                    Some(&candidate_seed),
                );

                match best_effort {
                    Some((snapshot, linearization, _, residual_norm)) => {
                        let convergence = Engine::vbic_snapshot_convergence_met(
                            &q11,
                            previous_snapshot.reduction.external_voltages,
                            &previous_snapshot,
                            snapshot.reduction.external_voltages,
                            &snapshot,
                            criteria,
                        );
                        let linearization_residual = Engine::vbic_internal_equation_residual_norm(
                            &linearization,
                            &snapshot.reduction.external_voltages,
                            &snapshot.reduction.internal_voltages,
                        );
                        eprintln!(
                            "Q11 seed={label} history={history_external:?} target={target_external:?} best_effort_residual={residual_norm:.12e} linearization_residual={linearization_residual:.12e} acceptable={} convergence={} internal={:?}",
                            Engine::vbic_dynamic_snapshot_residual_is_acceptable(residual_norm),
                            convergence,
                            snapshot.reduction.internal_voltages,
                        );
                    }
                    None => {
                        eprintln!(
                            "Q11 seed={label} history={history_external:?} target={target_external:?} best_effort=None"
                        );
                    }
                }
            }

            let continuation = Engine::continue_vbic_snapshot_to_external_bias_from_snapshot(
                &q11,
                previous_snapshot,
                target_external,
                method,
                trap_order,
                dt,
                &q_prev,
                &q_prev_prev,
                &cq_prev,
            );
            match continuation {
                Some(snapshot) => {
                    let continuation_snapshot = q11.charge_snapshot_for_dynamic_state(
                        target_external[0],
                        target_external[1],
                        target_external[2],
                        target_external[3],
                        snapshot.reduction.internal_voltages,
                    );
                    let continuation_linearization = Engine::assemble_vbic_transient_linearization(
                        &q11,
                        &continuation_snapshot,
                        method,
                        trap_order,
                        dt,
                        &q_prev,
                        &q_prev_prev,
                        &cq_prev,
                    )
                    .expect("assemble Q11 continuation diagnostic linearization");
                    let continuation_residual = Engine::vbic_internal_equation_residual_norm(
                        &continuation_linearization,
                        &continuation_snapshot.reduction.external_voltages,
                        &continuation_snapshot.reduction.internal_voltages,
                    );
                    eprintln!(
                        "Q11 continuation history={history_external:?} target={target_external:?} residual={continuation_residual:.12e} acceptable={}",
                        Engine::vbic_dynamic_snapshot_residual_is_acceptable(continuation_residual),
                    );
                }
                None => {
                    eprintln!(
                        "Q11 continuation history={history_external:?} target={target_external:?} result=None"
                    );
                }
            }
        }

        panic!("Q11 runtime-bias best-effort diagnostics complete");
    }

    #[test]
    fn test_vbic_continuation_step_from_snapshot_uses_state_based_limiter() {
        let (circuit, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_circuit();
        let bjt = circuit.bjts.devices[0].clone();
        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let current_external = [vc, vb, ve, vs];
        let target_external = [vc - 1.7, vb + 1.1, ve + 0.8, 0.2];

        let step = Engine::vbic_continuation_step_from_snapshot(
            &bjt,
            current_external,
            snapshot.reduction.internal_voltages,
            target_external,
        );
        let current_static_internal =
            Engine::vbic_static_internal_state_from_dynamic(snapshot.reduction.internal_voltages);
        let expected = bjt
            .vbic_external_step_limit_scale_from_state(
                current_external,
                current_static_internal,
                target_external,
            )
            .expect("expected large diffamp jump to engage VBIC continuation limiter");
        let expected_with_floor = expected.max(Engine::VBIC_CONTINUATION_MIN_TRIAL_STEP);

        assert!(
            step > 0.0 && step < 1.0,
            "expected VBIC continuation to damp a large diffamp jump, got step {step:.6e}"
        );
        assert!(
            (step - expected_with_floor).abs()
                <= 1e-15 + 1e-12 * expected_with_floor.abs().max(step.abs()),
            "expected continuation step {step:.6e} to follow the state-based limiter with the VBIC trial-step floor {expected_with_floor:.6e}"
        );
    }

    #[test]
    fn test_vbic_continuation_step_after_accept_respects_state_based_limiter() {
        let (circuit, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_circuit();
        let bjt = circuit.bjts.devices[0].clone();
        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let current_external = [vc, vb, ve, vs];
        let target_external = [vc - 1.7, vb + 1.1, ve + 0.8, 0.2];
        let suggested_step = Engine::vbic_continuation_step_from_snapshot(
            &bjt,
            current_external,
            snapshot.reduction.internal_voltages,
            target_external,
        );
        let current_step = suggested_step * 0.75;
        let next_step = Engine::vbic_continuation_step_after_accept(
            current_external,
            target_external,
            current_step,
            suggested_step,
        );

        assert!(
            next_step <= suggested_step * (1.0 + 1e-12),
            "expected accepted continuation step growth to stay within the state-based limiter, got next_step={next_step:.6e} limiter={suggested_step:.6e}"
        );
        assert!(
            (next_step - suggested_step).abs()
                <= 1e-15 + 1e-12 * next_step.abs().max(suggested_step.abs()),
            "expected accepted continuation step growth to clamp back to the limiter when doubling would overshoot, got next_step={next_step:.6e} limiter={suggested_step:.6e}"
        );
    }

    #[test]
    fn test_vbic_transient_external_limiter_anchors_to_accepted_solution() {
        let (circuit, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_circuit();
        let bjt = circuit.bjts.devices[0].clone();
        let size = circuit.num_nodes() + circuit.num_branches();
        let num_nodes = circuit.num_nodes();
        let protected_nodes = vec![false; num_nodes];

        let mut accepted = vec![0.0; size];
        for (node, value) in [
            (bjt.node_collector, vc),
            (bjt.node_base, vb),
            (bjt.node_emitter, ve),
            (bjt.node_substrate, vs),
        ] {
            if node > 0 {
                accepted[node - 1] = value;
            }
        }

        let mut previous_iter = accepted.clone();
        for (node, value) in [
            (bjt.node_collector, 1.905_837),
            (bjt.node_base, 3.591_117e-1),
            (bjt.node_emitter, 1.474_497),
            (bjt.node_substrate, 0.0),
        ] {
            if node > 0 {
                previous_iter[node - 1] = value;
            }
        }

        let mut previous_only = previous_iter.clone();
        let mut anchored = previous_iter.clone();
        for (node, value) in [
            (bjt.node_collector, 1.897_259),
            (bjt.node_base, -7.177_051e-1),
            (bjt.node_emitter, 1.473_671),
            (bjt.node_substrate, 0.0),
        ] {
            if node > 0 {
                previous_only[node - 1] = value;
                anchored[node - 1] = value;
            }
        }

        let previous_only_changed = Engine::limit_vbic_external_updates(
            &circuit,
            &mut previous_only,
            &previous_iter,
            num_nodes,
            Some(&protected_nodes),
            true,
        );
        let anchored_changed = Engine::limit_vbic_transient_external_updates(
            &circuit,
            &mut anchored,
            &previous_iter,
            &accepted,
            num_nodes,
            &protected_nodes,
            0.2,
        );

        let base_idx = bjt.node_base - 1;
        assert!(
            anchored_changed,
            "expected accepted-step VBIC limiter anchor to damp the pathological diffamp base proposal"
        );
        assert!(
            anchored[base_idx] > previous_only[base_idx] || !previous_only_changed,
            "expected accepted-step anchor to keep the VBIC base closer to the committed solution: previous-only={:.6e}, anchored={:.6e}",
            previous_only[base_idx],
            anchored[base_idx],
        );
        assert!(
            (anchored[base_idx] - accepted[base_idx]).abs()
                < (previous_only[base_idx] - accepted[base_idx]).abs(),
            "expected accepted-step anchor to keep the VBIC base closer to the accepted solution: accepted={:.6e}, previous-only={:.6e}, anchored={:.6e}",
            accepted[base_idx],
            previous_only[base_idx],
            anchored[base_idx],
        );
        assert!(
            (anchored[base_idx] - accepted[base_idx]).abs() <= 0.2 + 1e-15,
            "expected VBIC transient anchor to keep the base inside the accepted-step trust region: accepted={:.6e}, anchored={:.6e}",
            accepted[base_idx],
            anchored[base_idx],
        );
    }

    #[test]
    fn test_stamp_bjt_transient_companions_reuses_cached_vbic_snapshot() {
        let (circuit, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_circuit();
        let bjt = circuit.bjts.devices[0].clone();
        let size = circuit.num_nodes() + circuit.num_branches();
        let dt = 1e-11;
        let trap_order = 2;
        let method = IntegrationMethod::Trapezoidal;
        let voltage_abstol = 1e-6;
        let reltol = 1e-3;
        let mut base_solution = vec![0.0; size];
        for (node, value) in [
            (bjt.node_collector, vc),
            (bjt.node_base, vb),
            (bjt.node_emitter, ve),
            (bjt.node_substrate, vs),
        ] {
            if node > 0 {
                base_solution[node - 1] = value;
            }
        }

        let mut history = Engine::initialize_bjt_history(&circuit, &base_solution);
        history.accepted_dt_prev = dt;
        history.accepted_dt_prev_prev = dt;

        let cached_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let q_prev = cached_snapshot.branches.map(|branch| branch.charge);
        history.charge_q_prev[0] = q_prev;
        history.charge_q_prev_prev[0] = q_prev;
        history.charge_q_prev_prev_prev[0] = q_prev;
        history.charge_cq_prev[0] = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        history.dynamic_internal_prev[0] = [Value::NAN; BJT_INTERNAL_STATE_DIM];
        history.dynamic_internal_prev_prev[0] = [Value::NAN; BJT_INTERNAL_STATE_DIM];
        history.dynamic_linear_prev[0] = VbicPredictorLinearBranchState {
            vrcx: Value::NAN,
            vrci: Value::NAN,
            vrbx: Value::NAN,
            vrbi: Value::NAN,
            vre: Value::NAN,
            vrbp: Value::NAN,
            vrs: Value::NAN,
        };
        history.dynamic_linear_prev_prev[0] = history.dynamic_linear_prev[0];

        let target_external = [vc + 1.0e-4, vb - 2.0e-4, ve + 5.0e-5, vs];
        assert!(
            Engine::vbic_snapshot_matches_external_bias(
                &cached_snapshot,
                &target_external,
                voltage_abstol,
                reltol,
            ),
            "expected the cached VBIC snapshot to stay reusable while the candidate bias remains within solver tolerance"
        );
        let mut candidate_solution = base_solution.clone();
        for (node, value) in [
            (bjt.node_collector, target_external[0]),
            (bjt.node_base, target_external[1]),
            (bjt.node_emitter, target_external[2]),
            (bjt.node_substrate, target_external[3]),
        ] {
            if node > 0 {
                candidate_solution[node - 1] = value;
            }
        }

        let mut matrix = dense_static_matrix(size);
        let mut rhs = vec![0.0; size];
        let mut vbic_snapshot_cache = vec![Some(cached_snapshot); circuit.bjts.devices.len()];
        Engine::stamp_bjt_transient_companions(
            &circuit,
            &mut matrix,
            &mut rhs,
            &candidate_solution,
            method,
            trap_order,
            dt,
            &history,
            &mut vbic_snapshot_cache,
            VbicCachedSnapshotReuse::NewtonBypass,
            voltage_abstol,
            reltol,
        );

        let stamped_snapshot = vbic_snapshot_cache[0]
            .expect("expected transient BJT stamping to preserve a reusable VBIC snapshot");
        for (idx, expected) in cached_snapshot
            .reduction
            .external_voltages
            .iter()
            .enumerate()
        {
            assert!(
                (stamped_snapshot.reduction.external_voltages[idx] - *expected).abs() < 1e-18,
                "expected transient stamping to reuse the cached VBIC snapshot at external index {idx}"
            );
        }
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (stamped_snapshot.reduction.internal_voltages[idx]
                    - cached_snapshot.reduction.internal_voltages[idx])
                    .abs()
                    < 1e-18,
                "expected transient BJT stamping to keep the cached VBIC internal state when the candidate bias stays within solver tolerance at internal index {idx}"
            );
        }
    }

    #[test]
    fn test_stamp_bjt_transient_companions_seed_only_rebuilds_nearby_vbic_snapshot() {
        let (circuit, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_circuit();
        let bjt = circuit.bjts.devices[0].clone();
        let size = circuit.num_nodes() + circuit.num_branches();
        let dt = 1e-11;
        let trap_order = 2;
        let method = IntegrationMethod::Trapezoidal;
        let voltage_abstol = 1e-6;
        let reltol = 1e-3;
        let mut base_solution = vec![0.0; size];
        for (node, value) in [
            (bjt.node_collector, vc),
            (bjt.node_base, vb),
            (bjt.node_emitter, ve),
            (bjt.node_substrate, vs),
        ] {
            if node > 0 {
                base_solution[node - 1] = value;
            }
        }

        let mut history = Engine::initialize_bjt_history(&circuit, &base_solution);
        history.accepted_dt_prev = dt;
        history.accepted_dt_prev_prev = dt;

        let cached_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let q_prev = cached_snapshot.branches.map(|branch| branch.charge);
        history.charge_q_prev[0] = q_prev;
        history.charge_q_prev_prev[0] = q_prev;
        history.charge_q_prev_prev_prev[0] = q_prev;
        history.charge_cq_prev[0] = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        history.dynamic_internal_prev[0] = [Value::NAN; BJT_INTERNAL_STATE_DIM];
        history.dynamic_internal_prev_prev[0] = [Value::NAN; BJT_INTERNAL_STATE_DIM];
        history.dynamic_linear_prev[0] = VbicPredictorLinearBranchState {
            vrcx: Value::NAN,
            vrci: Value::NAN,
            vrbx: Value::NAN,
            vrbi: Value::NAN,
            vre: Value::NAN,
            vrbp: Value::NAN,
            vrs: Value::NAN,
        };
        history.dynamic_linear_prev_prev[0] = history.dynamic_linear_prev[0];

        let target_external = [vc + 1.0e-4, vb - 2.0e-4, ve + 5.0e-5, vs];
        assert!(
            Engine::vbic_snapshot_matches_external_bias(
                &cached_snapshot,
                &target_external,
                voltage_abstol,
                reltol,
            ),
            "expected the cached VBIC snapshot to stay within ordinary solver tolerance"
        );

        let expected_snapshot =
            Engine::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                &bjt,
                target_external,
                method,
                trap_order,
                dt,
                &history.charge_q_prev[0],
                &history.charge_q_prev_prev[0],
                &history.charge_cq_prev[0],
                Some(&history.dynamic_internal_prev[0]),
                Some(&history.dynamic_internal_prev_prev[0]),
                Some(&history.dynamic_linear_prev[0]),
                Some(&history.dynamic_linear_prev_prev[0]),
                history.accepted_dt_prev,
                Some(cached_snapshot),
                VbicCachedSnapshotReuse::SeedOnly,
                voltage_abstol,
                reltol,
            )
            .expect("resolve expected VBIC snapshot in seed-only mode");

        let mut candidate_solution = base_solution.clone();
        for (node, value) in [
            (bjt.node_collector, target_external[0]),
            (bjt.node_base, target_external[1]),
            (bjt.node_emitter, target_external[2]),
            (bjt.node_substrate, target_external[3]),
        ] {
            if node > 0 {
                candidate_solution[node - 1] = value;
            }
        }

        let mut matrix = dense_static_matrix(size);
        let mut rhs = vec![0.0; size];
        let mut vbic_snapshot_cache = vec![Some(cached_snapshot); circuit.bjts.devices.len()];
        Engine::stamp_bjt_transient_companions(
            &circuit,
            &mut matrix,
            &mut rhs,
            &candidate_solution,
            method,
            trap_order,
            dt,
            &history,
            &mut vbic_snapshot_cache,
            VbicCachedSnapshotReuse::SeedOnly,
            voltage_abstol,
            reltol,
        );

        let stamped_snapshot = vbic_snapshot_cache[0]
            .expect("expected seed-only transient stamping to refresh the VBIC snapshot");
        for (idx, expected) in target_external.iter().enumerate() {
            assert!(
                (stamped_snapshot.reduction.external_voltages[idx] - *expected).abs() < 1e-18,
                "expected seed-only transient stamping to resolve the target external bias at index {idx}"
            );
        }
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (stamped_snapshot.reduction.internal_voltages[idx]
                    - expected_snapshot.reduction.internal_voltages[idx])
                    .abs()
                    < 1e-12,
                "expected seed-only transient stamping to match the resolved VBIC hidden state at internal index {idx}"
            );
        }
    }

    #[test]
    fn test_stamp_bjt_transient_companions_rebuilds_vbic_snapshot_outside_solver_tolerance() {
        let (circuit, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_circuit();
        let bjt = circuit.bjts.devices[0].clone();
        let size = circuit.num_nodes() + circuit.num_branches();
        let dt = 1e-11;
        let trap_order = 2;
        let method = IntegrationMethod::Trapezoidal;
        let voltage_abstol = 1e-6;
        let reltol = 1e-3;
        let mut base_solution = vec![0.0; size];
        for (node, value) in [
            (bjt.node_collector, vc),
            (bjt.node_base, vb),
            (bjt.node_emitter, ve),
            (bjt.node_substrate, vs),
        ] {
            if node > 0 {
                base_solution[node - 1] = value;
            }
        }

        let mut history = Engine::initialize_bjt_history(&circuit, &base_solution);
        history.accepted_dt_prev = dt;
        history.accepted_dt_prev_prev = dt;

        let cached_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let q_prev = cached_snapshot.branches.map(|branch| branch.charge);
        history.charge_q_prev[0] = q_prev;
        history.charge_q_prev_prev[0] = q_prev;
        history.charge_q_prev_prev_prev[0] = q_prev;
        history.charge_cq_prev[0] = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        history.dynamic_internal_prev[0] = [Value::NAN; BJT_INTERNAL_STATE_DIM];
        history.dynamic_internal_prev_prev[0] = [Value::NAN; BJT_INTERNAL_STATE_DIM];
        history.dynamic_linear_prev[0] = VbicPredictorLinearBranchState {
            vrcx: Value::NAN,
            vrci: Value::NAN,
            vrbx: Value::NAN,
            vrbi: Value::NAN,
            vre: Value::NAN,
            vrbp: Value::NAN,
            vrs: Value::NAN,
        };
        history.dynamic_linear_prev_prev[0] = history.dynamic_linear_prev[0];

        let target_external = [vc + 2.0e-3, vb - 2.0e-3, ve + 1.0e-3, vs];
        assert!(
            !Engine::vbic_snapshot_matches_external_bias(
                &cached_snapshot,
                &target_external,
                voltage_abstol,
                reltol,
            ),
            "expected transient stamping to rebuild the VBIC snapshot once the candidate bias leaves solver tolerance"
        );
        let mut candidate_solution = base_solution.clone();
        for (node, value) in [
            (bjt.node_collector, target_external[0]),
            (bjt.node_base, target_external[1]),
            (bjt.node_emitter, target_external[2]),
            (bjt.node_substrate, target_external[3]),
        ] {
            if node > 0 {
                candidate_solution[node - 1] = value;
            }
        }

        let expected_snapshot =
            Engine::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                &bjt,
                target_external,
                method,
                trap_order,
                dt,
                &history.charge_q_prev[0],
                &history.charge_q_prev_prev[0],
                &history.charge_cq_prev[0],
                Some(&history.dynamic_internal_prev[0]),
                Some(&history.dynamic_internal_prev_prev[0]),
                Some(&history.dynamic_linear_prev[0]),
                Some(&history.dynamic_linear_prev_prev[0]),
                history.accepted_dt_prev,
                Some(cached_snapshot),
                VbicCachedSnapshotReuse::NewtonBypass,
                voltage_abstol,
                reltol,
            )
            .expect("resolve expected VBIC snapshot outside runtime cache tolerance");

        let mut matrix = dense_static_matrix(size);
        let mut rhs = vec![0.0; size];
        let mut vbic_snapshot_cache = vec![Some(cached_snapshot); circuit.bjts.devices.len()];
        Engine::stamp_bjt_transient_companions(
            &circuit,
            &mut matrix,
            &mut rhs,
            &candidate_solution,
            method,
            trap_order,
            dt,
            &history,
            &mut vbic_snapshot_cache,
            VbicCachedSnapshotReuse::NewtonBypass,
            voltage_abstol,
            reltol,
        );

        let stamped_snapshot = vbic_snapshot_cache[0]
            .expect("expected transient BJT stamping to refresh the VBIC snapshot");
        for (idx, expected) in target_external.iter().enumerate() {
            assert!(
                (stamped_snapshot.reduction.external_voltages[idx] - *expected).abs() < 1e-18,
                "expected transient stamping cache to refresh to the candidate external bias at index {idx}"
            );
        }
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (stamped_snapshot.reduction.internal_voltages[idx]
                    - expected_snapshot.reduction.internal_voltages[idx])
                    .abs()
                    < 1e-12,
                "expected transient BJT stamping to resolve the same VBIC continuation outside solver tolerance at internal index {idx}"
            );
        }
    }

    #[test]
    fn test_vbic_startup_dynamic_system_matches_static_at_diffamp_input_bias() {
        let (bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_bjt();
        let method = IntegrationMethod::BackwardEuler;
        let trap_order = 1;
        let dt = 1e-12;
        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let (snapshot, linearization, _snapshot_static_g) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&base_snapshot.reduction.internal_voltages),
        )
        .expect("solve startup VBIC transient state at diffamp input bias");
        let (y_total, reduced_i_eq) = Engine::vbic_reduce_transient_external_system(&linearization)
            .expect("reduce startup VBIC external system");
        let (base_static_g, base_static_i_eq) = Engine::vbic_static_stamped_external_system(
            &bjt,
            &snapshot.reduction.external_voltages,
        );
        let external = snapshot.reduction.external_voltages;
        let internal = snapshot.reduction.internal_voltages;

        let mut max_current_delta: Value = 0.0;
        let mut external_deltas = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let mut total_current = -reduced_i_eq[row];
            let mut static_current = -base_static_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                total_current += y_total[row][col] * external[col];
                static_current += base_static_g[row][col] * external[col];
            }
            external_deltas[row] = total_current - static_current;
            max_current_delta = max_current_delta.max(external_deltas[row].abs());
        }

        let mut internal_residual = [0.0; BJT_INTERNAL_STATE_DIM];
        for row in 0..BJT_INTERNAL_STATE_DIM {
            internal_residual[row] = -linearization.z_i[row];
            for col in 0..BJT_INTERNAL_STATE_DIM {
                internal_residual[row] += linearization.g_ii[row][col] * internal[col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                internal_residual[row] += linearization.g_ie[row][col] * external[col];
            }
        }

        assert!(
            max_current_delta < 1e-9,
            "expected startup VBIC dynamic system to collapse back to the static diffamp input currents when history matches the DC operating point, got max delta {max_current_delta:.3e}, external_deltas={external_deltas:?}, internal_residual={internal_residual:?}, xf=({:.12e}, {:.12e})",
            internal[BJT_DELAY_XF1_STATE_INDEX],
            internal[BJT_DELAY_XF2_STATE_INDEX]
        );
    }

    #[test]
    fn test_vbic_startup_dynamic_system_matches_static_at_pnp_diffamp_bias() {
        let (bjt, vc, vb, ve, vs) = vbic_pnp_diffamp_mirror_test_bjt();
        let method = IntegrationMethod::BackwardEuler;
        let trap_order = 1;
        let dt = 1e-12;
        let base_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let q_prev = base_snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let (snapshot, linearization, _snapshot_static_g) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
            Some(&base_snapshot.reduction.internal_voltages),
        )
        .expect("solve startup PNP VBIC transient state at diffamp bias");
        let (y_total, reduced_i_eq) = Engine::vbic_reduce_transient_external_system(&linearization)
            .expect("reduce startup PNP VBIC external system");
        let (base_static_g, base_static_i_eq) = Engine::vbic_static_stamped_external_system(
            &bjt,
            &snapshot.reduction.external_voltages,
        );
        let external = snapshot.reduction.external_voltages;
        let internal = snapshot.reduction.internal_voltages;

        let mut max_current_delta: Value = 0.0;
        let mut external_deltas = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let mut total_current = -reduced_i_eq[row];
            let mut static_current = -base_static_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                total_current += y_total[row][col] * external[col];
                static_current += base_static_g[row][col] * external[col];
            }
            external_deltas[row] = total_current - static_current;
            max_current_delta = max_current_delta.max(external_deltas[row].abs());
        }

        let mut internal_residual = [0.0; BJT_INTERNAL_STATE_DIM];
        for row in 0..BJT_INTERNAL_STATE_DIM {
            internal_residual[row] = -linearization.z_i[row];
            for col in 0..BJT_INTERNAL_STATE_DIM {
                internal_residual[row] += linearization.g_ii[row][col] * internal[col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                internal_residual[row] += linearization.g_ie[row][col] * external[col];
            }
        }

        assert!(
            max_current_delta < 1e-9,
            "expected startup PNP VBIC dynamic system to collapse back to the static diffamp mirror currents when history matches the DC operating point, got max delta {max_current_delta:.3e}, external_deltas={external_deltas:?}, internal_residual={internal_residual:?}, xf=({:.12e}, {:.12e})",
            internal[BJT_DELAY_XF1_STATE_INDEX],
            internal[BJT_DELAY_XF2_STATE_INDEX]
        );
    }

    #[test]
    fn test_vbic_second_step_after_startup_promotion_stays_at_static_diffamp_bias() {
        let (bjt, vc, vb, ve, vs) = vbic_npn_diffamp_input_test_bjt();
        let startup_dt = 1e-12;
        let promoted_dt = 3e-12;

        let initial_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let initial_q = initial_snapshot.branches.map(|branch| branch.charge);
        let initial_internal = initial_snapshot.reduction.internal_voltages;

        let startup_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&initial_internal),
            Some(&initial_internal),
            startup_dt,
            0.0,
        );
        let (accepted_startup_snapshot, _, _) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            IntegrationMethod::Trapezoidal,
            1,
            startup_dt,
            &initial_q,
            &initial_q,
            &[0.0; BJT_DYNAMIC_CHARGE_COUNT],
            Some(&startup_seed),
        )
        .expect("solve accepted startup VBIC transient state");

        let startup_effective_method =
            Engine::effective_companion_method(IntegrationMethod::Trapezoidal, 1);
        let mut accepted_cq = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        for (branch_idx, branch) in accepted_startup_snapshot.branches.iter().enumerate() {
            accepted_cq[branch_idx] = Engine::jfet_companion_ccap(
                startup_effective_method,
                1,
                startup_dt,
                branch.charge,
                initial_q[branch_idx],
                initial_q[branch_idx],
                0.0,
            );
        }

        let promoted_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&accepted_startup_snapshot.reduction.internal_voltages),
            Some(&initial_internal),
            promoted_dt,
            startup_dt,
        );
        let (promoted_snapshot, promoted_linearization, _) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            IntegrationMethod::Trapezoidal,
            2,
            promoted_dt,
            &accepted_startup_snapshot
                .branches
                .map(|branch| branch.charge),
            &initial_q,
            &accepted_cq,
            Some(&promoted_seed),
        )
        .expect("solve promoted VBIC transient state at steady diffamp bias");

        let (y_total, reduced_i_eq) =
            Engine::vbic_reduce_transient_external_system(&promoted_linearization)
                .expect("reduce promoted VBIC external system");
        let (base_static_g, base_static_i_eq) = Engine::vbic_static_stamped_external_system(
            &bjt,
            &promoted_snapshot.reduction.external_voltages,
        );
        let external = promoted_snapshot.reduction.external_voltages;

        let mut max_current_delta: Value = 0.0;
        let mut external_deltas = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let mut total_current = -reduced_i_eq[row];
            let mut static_current = -base_static_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                total_current += y_total[row][col] * external[col];
                static_current += base_static_g[row][col] * external[col];
            }
            external_deltas[row] = total_current - static_current;
            max_current_delta = max_current_delta.max(external_deltas[row].abs());
        }

        assert!(
            max_current_delta < 1e-9,
            "expected promoted steady-state VBIC transient system to collapse back to the static diffamp currents, got max delta {max_current_delta:.3e}, external_deltas={external_deltas:?}, xf=({:.12e}, {:.12e})",
            promoted_snapshot.reduction.internal_voltages[BJT_DELAY_XF1_STATE_INDEX],
            promoted_snapshot.reduction.internal_voltages[BJT_DELAY_XF2_STATE_INDEX]
        );
    }

    #[test]
    fn test_vbic_pnp_second_step_after_startup_promotion_stays_at_static_diffamp_bias() {
        let (bjt, vc, vb, ve, vs) = vbic_pnp_diffamp_mirror_test_bjt();
        let startup_dt = 1e-12;
        let promoted_dt = 3e-12;

        let initial_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let initial_q = initial_snapshot.branches.map(|branch| branch.charge);
        let initial_internal = initial_snapshot.reduction.internal_voltages;

        let startup_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&initial_internal),
            Some(&initial_internal),
            startup_dt,
            0.0,
        );
        let (accepted_startup_snapshot, _, _) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            IntegrationMethod::Trapezoidal,
            1,
            startup_dt,
            &initial_q,
            &initial_q,
            &[0.0; BJT_DYNAMIC_CHARGE_COUNT],
            Some(&startup_seed),
        )
        .expect("solve accepted startup PNP VBIC transient state");

        let startup_effective_method =
            Engine::effective_companion_method(IntegrationMethod::Trapezoidal, 1);
        let mut accepted_cq = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        for (branch_idx, branch) in accepted_startup_snapshot.branches.iter().enumerate() {
            accepted_cq[branch_idx] = Engine::jfet_companion_ccap(
                startup_effective_method,
                1,
                startup_dt,
                branch.charge,
                initial_q[branch_idx],
                initial_q[branch_idx],
                0.0,
            );
        }

        let promoted_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&accepted_startup_snapshot.reduction.internal_voltages),
            Some(&initial_internal),
            promoted_dt,
            startup_dt,
        );
        let (promoted_snapshot, promoted_linearization, _) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            IntegrationMethod::Trapezoidal,
            2,
            promoted_dt,
            &accepted_startup_snapshot
                .branches
                .map(|branch| branch.charge),
            &initial_q,
            &accepted_cq,
            Some(&promoted_seed),
        )
        .expect("solve promoted PNP VBIC transient state at steady diffamp bias");

        let (y_total, reduced_i_eq) =
            Engine::vbic_reduce_transient_external_system(&promoted_linearization)
                .expect("reduce promoted PNP VBIC external system");
        let (base_static_g, base_static_i_eq) = Engine::vbic_static_stamped_external_system(
            &bjt,
            &promoted_snapshot.reduction.external_voltages,
        );
        let external = promoted_snapshot.reduction.external_voltages;

        let mut max_current_delta: Value = 0.0;
        let mut external_deltas = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let mut total_current = -reduced_i_eq[row];
            let mut static_current = -base_static_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                total_current += y_total[row][col] * external[col];
                static_current += base_static_g[row][col] * external[col];
            }
            external_deltas[row] = total_current - static_current;
            max_current_delta = max_current_delta.max(external_deltas[row].abs());
        }

        assert!(
            max_current_delta < 1e-9,
            "expected promoted steady-state PNP VBIC transient system to collapse back to the static diffamp mirror currents, got max delta {max_current_delta:.3e}, external_deltas={external_deltas:?}, xf=({:.12e}, {:.12e})",
            promoted_snapshot.reduction.internal_voltages[BJT_DELAY_XF1_STATE_INDEX],
            promoted_snapshot.reduction.internal_voltages[BJT_DELAY_XF2_STATE_INDEX]
        );
    }

    #[test]
    fn test_vbic_pnp_tied_mirror_second_step_after_startup_promotion_stays_at_static_bias() {
        let (bjt, vc, vb, ve, vs) = vbic_pnp_tied_mirror_test_bjt();
        let startup_dt = 1e-12;
        let promoted_dt = 3e-12;

        let initial_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let initial_q = initial_snapshot.branches.map(|branch| branch.charge);
        let initial_internal = initial_snapshot.reduction.internal_voltages;

        let startup_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&initial_internal),
            Some(&initial_internal),
            startup_dt,
            0.0,
        );
        let (accepted_startup_snapshot, _, _) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            IntegrationMethod::Trapezoidal,
            1,
            startup_dt,
            &initial_q,
            &initial_q,
            &[0.0; BJT_DYNAMIC_CHARGE_COUNT],
            Some(&startup_seed),
        )
        .expect("solve accepted startup tied-mirror PNP VBIC transient state");

        let startup_effective_method =
            Engine::effective_companion_method(IntegrationMethod::Trapezoidal, 1);
        let mut accepted_cq = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        for (branch_idx, branch) in accepted_startup_snapshot.branches.iter().enumerate() {
            accepted_cq[branch_idx] = Engine::jfet_companion_ccap(
                startup_effective_method,
                1,
                startup_dt,
                branch.charge,
                initial_q[branch_idx],
                initial_q[branch_idx],
                0.0,
            );
        }

        let promoted_seed = Engine::vbic_dynamic_internal_seed_from_history(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            Some(&accepted_startup_snapshot.reduction.internal_voltages),
            Some(&initial_internal),
            promoted_dt,
            startup_dt,
        );
        let (promoted_snapshot, promoted_linearization, _) = Engine::solve_vbic_dynamic_snapshot(
            &bjt,
            vc,
            vb,
            ve,
            vs,
            IntegrationMethod::Trapezoidal,
            2,
            promoted_dt,
            &accepted_startup_snapshot
                .branches
                .map(|branch| branch.charge),
            &initial_q,
            &accepted_cq,
            Some(&promoted_seed),
        )
        .expect("solve promoted tied-mirror PNP VBIC transient state at steady bias");

        let (y_total, reduced_i_eq) =
            Engine::vbic_reduce_transient_external_system(&promoted_linearization)
                .expect("reduce promoted tied-mirror PNP VBIC external system");
        let (base_static_g, base_static_i_eq) = Engine::vbic_static_stamped_external_system(
            &bjt,
            &promoted_snapshot.reduction.external_voltages,
        );
        let external = promoted_snapshot.reduction.external_voltages;

        let mut max_current_delta: Value = 0.0;
        let mut external_deltas = [0.0; BJT_EXTERNAL_STATE_DIM];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            let mut total_current = -reduced_i_eq[row];
            let mut static_current = -base_static_i_eq[row];
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                total_current += y_total[row][col] * external[col];
                static_current += base_static_g[row][col] * external[col];
            }
            external_deltas[row] = total_current - static_current;
            max_current_delta = max_current_delta.max(external_deltas[row].abs());
        }

        assert!(
            max_current_delta < 1e-9,
            "expected promoted steady-state tied-mirror PNP VBIC transient system to collapse back to the static currents, got max delta {max_current_delta:.3e}, external_deltas={external_deltas:?}, xf=({:.12e}, {:.12e})",
            promoted_snapshot.reduction.internal_voltages[BJT_DELAY_XF1_STATE_INDEX],
            promoted_snapshot.reduction.internal_voltages[BJT_DELAY_XF2_STATE_INDEX]
        );
    }

    #[test]
    fn test_vbic_transient_owning_charge_branch_keeps_delay_charge_local() {
        let (bjt, vc, vb, ve, vs) = vbic_focus_test_bjt();
        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);

        let qxf1 = Engine::vbic_transient_owning_charge_branch(
            &bjt,
            BJT_DELAY_XF1_BRANCH_INDEX,
            &snapshot.branches[BJT_DELAY_XF1_BRANCH_INDEX],
        )
        .expect("build ngspice-style Qxf1 transient companion");
        let qxf2 = Engine::vbic_transient_owning_charge_branch(
            &bjt,
            BJT_DELAY_XF2_BRANCH_INDEX,
            &snapshot.branches[BJT_DELAY_XF2_BRANCH_INDEX],
        )
        .expect("build ngspice-style Qxf2 transient companion");

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            let qxf1_expected = idx == BJT_DELAY_XF1_STATE_INDEX;
            let qxf2_expected = idx == BJT_DELAY_XF2_STATE_INDEX;
            if qxf1_expected {
                assert!(
                    qxf1.d_internal[idx].abs() > 0.0,
                    "expected Qxf1 transient companion to keep its local delay derivative"
                );
            } else {
                assert!(
                    qxf1.d_internal[idx].abs() < 1e-18,
                    "expected Qxf1 transient companion to stay local at index {idx}"
                );
            }
            if qxf2_expected {
                assert!(
                    qxf2.d_internal[idx].abs() > 0.0,
                    "expected Qxf2 transient companion to keep its local delay derivative"
                );
            } else {
                assert!(
                    qxf2.d_internal[idx].abs() < 1e-18,
                    "expected Qxf2 transient companion to stay local at index {idx}"
                );
            }
        }
    }

    #[test]
    #[ignore]
    fn debug_vbic_diffamp_q1_initial_internal_solve() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.max_iterations = config.max_iterations.max(1200);
        config.convergence_config = crate::ConvergenceConfig::robust();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.min_timestep = 1e-12;
        config.temperature = 300.15;
        let engine = Engine::new(config);
        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");
        let mut matrix = engine.build_matrix(&circuit).expect("build diffamp matrix");
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("solve diffamp dc op");
        circuit.update_nonlinear(&solution);
        let history = Engine::initialize_bjt_history(&circuit, &solution);

        let q1 = circuit
            .bjts
            .devices
            .iter()
            .find(|device| device.name == "Q1")
            .expect("find Q1")
            .clone();
        let idx = circuit
            .bjts
            .devices
            .iter()
            .position(|device| device.name == "Q1")
            .expect("find Q1 index");
        let vc = Engine::node_voltage(&solution, q1.node_collector);
        let vb = Engine::node_voltage(&solution, q1.node_base);
        let ve = Engine::node_voltage(&solution, q1.node_emitter);
        let vs = Engine::node_voltage(&solution, q1.node_substrate);
        let snapshot = q1.charge_snapshot(vc, vb, ve, vs);
        let q_prev = history.charge_q_prev[idx];
        let q_prev_prev = history.charge_q_prev_prev[idx];
        let cq_prev = history.charge_cq_prev[idx];
        let linearization = Engine::assemble_vbic_transient_linearization(
            &q1,
            &snapshot,
            IntegrationMethod::Trapezoidal,
            2,
            1e-12,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble transient linearization");
        let solved_internal = Engine::solve_vbic_internal_state_from_linearization(
            &linearization,
            &snapshot.reduction.external_voltages,
        )
        .expect("solve transient internal system");

        eprintln!(
            "Q1 diffamp OP ext=({:.12e}, {:.12e}, {:.12e}, {:.12e}) seed={:?} solved={:?}",
            vc, vb, ve, vs, snapshot.reduction.internal_voltages, solved_internal
        );
    }

    #[test]
    #[ignore]
    fn debug_vbic_diffamp_startup_transient_matrix_remains_finite() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.max_iterations = config.max_iterations.max(1200);
        config.convergence_config = crate::ConvergenceConfig::robust();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.min_timestep = 1e-15;
        config.temperature = 300.15;
        let engine = Engine::new(config);

        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");
        let mut matrix = engine.build_matrix(&circuit).expect("build diffamp matrix");
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("solve diffamp dc op");
        circuit.update_nonlinear(&solution);
        let history = Engine::initialize_bjt_history(&circuit, &solution);

        matrix.clear_values();
        let mut rhs = vec![0.0; circuit.matrix_size()];
        circuit.stamp_transient_linear_direct(&mut matrix, &mut rhs);
        let num_nodes = circuit.num_nodes();
        let dt = 1e-12;
        circuit
            .voltage_sources
            .update_transient_rhs(&mut rhs, dt, |branch_ordinal| num_nodes + branch_ordinal);
        circuit.current_sources.update_transient_rhs(&mut rhs, dt);
        circuit.stamp_nonlinear(&mut matrix, &mut rhs, &solution);
        let mut vbic_snapshot_cache = vec![None; circuit.bjts.devices.len()];
        Engine::stamp_bjt_transient_companions(
            &circuit,
            &mut matrix,
            &mut rhs,
            &solution,
            IntegrationMethod::Trapezoidal,
            1,
            dt,
            &history,
            &mut vbic_snapshot_cache,
            VbicCachedSnapshotReuse::SeedOnly,
            1e-6,
            1e-3,
        );

        for (idx, value) in rhs.iter().enumerate() {
            assert!(
                value.is_finite(),
                "expected finite startup transient rhs entry at index {idx}, got {value:?}"
            );
        }

        let solved = matrix
            .solve_dense(&rhs)
            .expect("solve dense startup diffamp transient matrix");
        for (idx, value) in solved.iter().enumerate() {
            assert!(
                value.is_finite(),
                "expected finite startup transient solution entry at index {idx}, got {value:?}"
            );
        }
        let residual = matrix
            .residual_vector(&solved, &rhs)
            .expect("compute startup diffamp transient residual");
        for (idx, value) in residual.iter().enumerate() {
            assert!(
                value.is_finite(),
                "expected finite startup transient residual entry at index {idx}, got {value:?}"
            );
        }
    }

    #[test]
    #[ignore]
    fn debug_vbic_diffamp_startup_snapshot_timings() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/vbic/diffamp.cir");
        eprintln!("reading diffamp deck from {}", deck_path.display());
        let source = std::fs::read_to_string(&deck_path).expect("read diffamp deck");
        let netlist = crate::Netlist::parse(&source).expect("parse diffamp deck");
        eprintln!("parsed diffamp deck");

        let mut config = crate::SimulationConfig::default();
        config.max_iterations = config.max_iterations.max(1200);
        config.convergence_config = crate::ConvergenceConfig::robust();
        config.integration_method = IntegrationMethod::Trapezoidal;
        config.min_timestep = 1e-15;
        config.temperature = 300.15;
        let engine = Engine::new(config);

        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("build diffamp circuit");
        eprintln!("built diffamp circuit");
        let mut matrix = engine.build_matrix(&circuit).expect("build diffamp matrix");
        eprintln!("built diffamp matrix");
        let solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("solve diffamp dc op");
        eprintln!("solved diffamp dc op");
        circuit.update_nonlinear(&solution);
        let history = Engine::initialize_bjt_history(&circuit, &solution);
        let dt = 1e-11;

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if !bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            let vc = Engine::node_voltage(&solution, bjt.node_collector);
            let vb = Engine::node_voltage(&solution, bjt.node_base);
            let ve = Engine::node_voltage(&solution, bjt.node_emitter);
            let vs = Engine::node_voltage(&solution, bjt.node_substrate);
            eprintln!(
                "starting {} ext=({:.12e}, {:.12e}, {:.12e}, {:.12e})",
                bjt.name, vc, vb, ve, vs
            );
            let t0 = std::time::Instant::now();
            let snapshot = Engine::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                bjt,
                [vc, vb, ve, vs],
                IntegrationMethod::Trapezoidal,
                1,
                dt,
                &history.charge_q_prev[idx],
                &history.charge_q_prev_prev[idx],
                &history.charge_cq_prev[idx],
                history.dynamic_internal_prev.get(idx),
                history.dynamic_internal_prev_prev.get(idx),
                history.dynamic_linear_prev.get(idx),
                history.dynamic_linear_prev_prev.get(idx),
                history.accepted_dt_prev,
                None,
                VbicCachedSnapshotReuse::SeedOnly,
                VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL,
                VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL,
            );
            eprintln!(
                "resolved {} in {:.3?}: success={} xf=({:.12e}, {:.12e})",
                bjt.name,
                t0.elapsed(),
                snapshot.is_some(),
                snapshot
                    .as_ref()
                    .map(|value| value.reduction.internal_voltages[BJT_DELAY_XF1_STATE_INDEX])
                    .unwrap_or(f64::NAN),
                snapshot
                    .as_ref()
                    .map(|value| value.reduction.internal_voltages[BJT_DELAY_XF2_STATE_INDEX])
                    .unwrap_or(f64::NAN),
            );
        }
    }

    #[test]
    fn test_assemble_vbic_transient_linearization_stamps_delay_thermal_branch() {
        let (bjt, vc, vb, ve, vs) = vbic_self_heated_focus_test_bjt();
        let method = IntegrationMethod::Trapezoidal;
        let trap_order = 2;
        let dt = 1e-11;

        let snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let q_prev = snapshot.branches.map(|branch| branch.charge);
        let q_prev_prev = q_prev;
        let cq_prev = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let linearization = Engine::assemble_vbic_transient_linearization(
            &bjt,
            &snapshot,
            method,
            trap_order,
            dt,
            &q_prev,
            &q_prev_prev,
            &cq_prev,
        )
        .expect("assemble VBIC transient linearization");

        let charge_factor = Engine::jfet_companion_geq(method, trap_order, 1.0, dt);
        let mut expected_g_ii = snapshot.reduction.g_ii;
        let mut expected_g_ie = snapshot.reduction.g_ie;
        let mut expected_g_ei = snapshot.reduction.g_ei;
        let mut expected_g_ee = snapshot.reduction.g_ee;
        let mut expected_z_i = snapshot.reduction.z_i_static;
        let mut expected_z_e = snapshot.reduction.z_e_static;
        let mut c_ii = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ie = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ei = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut c_ee = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];

        for branch in bjt.vbic_delay_static_branches(&snapshot.reduction) {
            if !branch.is_active() {
                continue;
            }
            let i_eq = branch.linearization_dot(
                &snapshot.reduction.internal_voltages,
                &snapshot.reduction.external_voltages,
            ) - branch.current;
            branch.accumulate_source(i_eq, &mut expected_z_i, &mut expected_z_e);
        }

        let thermal_branch = bjt.vbic_delay_static_thermal_branch(&snapshot.reduction);
        assert!(thermal_branch.is_active());
        thermal_branch.accumulate_derivatives(
            &mut expected_g_ii,
            &mut expected_g_ie,
            &mut expected_g_ei,
            &mut expected_g_ee,
        );
        let thermal_i_eq = thermal_branch.linearization_dot(
            &snapshot.reduction.internal_voltages,
            &snapshot.reduction.external_voltages,
        ) - thermal_branch.current;
        thermal_branch.accumulate_source(thermal_i_eq, &mut expected_z_i, &mut expected_z_e);

        for (branch_idx, branch) in snapshot.branches.iter().enumerate() {
            let Some(branch) =
                Engine::vbic_transient_owning_charge_branch(&bjt, branch_idx, branch)
            else {
                continue;
            };
            branch.accumulate_derivatives(&mut c_ii, &mut c_ie, &mut c_ei, &mut c_ee);
            let cq_curr = Engine::jfet_companion_ccap(
                method,
                trap_order,
                dt,
                branch.charge,
                q_prev[branch_idx],
                q_prev_prev[branch_idx],
                cq_prev[branch_idx],
            );
            let ccap_history_sign =
                Engine::vbic_transient_owning_charge_ccap_sign(&bjt, branch_idx);
            let i_eq = charge_factor
                * branch.linearization_dot(
                    &snapshot.reduction.internal_voltages,
                    &snapshot.reduction.external_voltages,
                )
                - ccap_history_sign * cq_curr;
            branch.accumulate_source(i_eq, &mut expected_z_i, &mut expected_z_e);
        }

        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                expected_g_ii[row][col] += charge_factor * c_ii[row][col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                expected_g_ie[row][col] += charge_factor * c_ie[row][col];
            }
        }
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                expected_g_ei[row][col] += charge_factor * c_ei[row][col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                expected_g_ee[row][col] += charge_factor * c_ee[row][col];
            }
        }

        let mut thermal_row_delta = 0.0_f64;
        for col in 0..BJT_INTERNAL_STATE_DIM {
            thermal_row_delta = thermal_row_delta.max(
                (expected_g_ii[BJT_THERMAL_STATE_INDEX][col]
                    - snapshot.reduction.g_ii[BJT_THERMAL_STATE_INDEX][col])
                    .abs(),
            );
            assert!(
                (linearization.g_ii[BJT_THERMAL_STATE_INDEX][col]
                    - expected_g_ii[BJT_THERMAL_STATE_INDEX][col])
                    .abs()
                    < 1e-18,
                "expected thermal row internal Jacobian to include excess-phase correction at column {col}"
            );
        }
        assert!(
            thermal_row_delta > 0.0,
            "expected excess-phase thermal branch to change the temperature row Jacobian"
        );
        assert!(
            (linearization.z_i[BJT_THERMAL_STATE_INDEX] - expected_z_i[BJT_THERMAL_STATE_INDEX])
                .abs()
                < 1e-18,
            "expected thermal row source to include excess-phase correction"
        );
    }

    #[test]
    fn test_should_prefer_dense_transient_solver_for_small_nonlinear_systems() {
        assert!(Engine::should_prefer_dense_transient_solver(
            false, 64, false, false
        ));
        assert!(!Engine::should_prefer_dense_transient_solver(
            false, 65, false, false
        ));
        assert!(!Engine::should_prefer_dense_transient_solver(
            false, 32, false, true
        ));
    }

    #[test]
    fn test_should_prefer_dense_transient_solver_only_for_linear_transformer_coupling() {
        assert!(Engine::should_prefer_dense_transient_solver(
            true, 80, true, false
        ));
        assert!(!Engine::should_prefer_dense_transient_solver(
            true, 80, false, false
        ));
        assert!(!Engine::should_prefer_dense_transient_solver(
            true, 200, true, false
        ));
    }

    #[test]
    fn test_transient_source_step_hint_prefers_explicit_tran_step() {
        let netlist = Netlist::parse(
            "Transient step hint\n\
             V1 in 0 DC 1\n\
             .tran 5n 100n\n\
             .end",
        )
        .expect("netlist");

        let hint = Engine::transient_source_step_hint(&netlist, 1e-6);
        assert!((hint - 5e-9).abs() < 1e-18);
    }

    #[test]
    fn test_transient_source_step_hint_falls_back_to_fraction_of_max_step() {
        let netlist = Netlist::parse(
            "Transient step fallback\n\
             V1 in 0 DC 1\n\
             .op\n\
             .end",
        )
        .expect("netlist");

        let hint = Engine::transient_source_step_hint(&netlist, 2e-6);
        assert!((hint - 2e-7).abs() < 1e-18);
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

    #[test]
    fn test_dynamic_tline_breakpoints_only_schedule_material_wave_changes() {
        let mut breakpoints = BreakpointManager::new();
        let mut dynamic_breakpoints_added = 0;
        let mut warned_dynamic_breakpoint_cap = false;

        Engine::maybe_schedule_tline_arrival_breakpoint(
            &mut breakpoints,
            0.5e-9,
            1.0e-9,
            5.0e-9,
            1.0,
            1.0 + 5.0e-7,
            1e-3,
            1e-6,
            &mut dynamic_breakpoints_added,
            &mut warned_dynamic_breakpoint_cap,
        );
        assert!(
            breakpoints.is_empty(),
            "sub-tolerance wave changes should not create arrival breakpoints"
        );

        Engine::maybe_schedule_tline_arrival_breakpoint(
            &mut breakpoints,
            0.5e-9,
            1.0e-9,
            5.0e-9,
            1.0,
            1.02,
            1e-3,
            1e-6,
            &mut dynamic_breakpoints_added,
            &mut warned_dynamic_breakpoint_cap,
        );
        assert_eq!(dynamic_breakpoints_added, 1);
        assert_eq!(breakpoints.times().len(), 1);
        assert!((breakpoints.times()[0] - 1.5e-9).abs() < 1e-21);
        assert!(!warned_dynamic_breakpoint_cap);
    }
}
