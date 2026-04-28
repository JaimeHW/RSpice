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
/// Largest node trust-region used after repeated finite Newton corrections.
///
/// Device-local junction limiting still governs semiconductor branch voltages;
/// this cap only prevents the global MNA node update limiter from turning a
/// valid large-signal transition into hundreds of identical 10 mV iterations.
const MAX_ADAPTIVE_NEWTON_ITER_DELTA_V: Value = STARTUP_RECOVERY_DELTA_V;
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
        if !(clip_limit.is_finite() && clip_limit > 0.0) {
            return false;
        }
        let observed_delta =
            Self::max_abs_delta_prefix(previous_solution, candidate_solution, num_nodes);
        let quiet_limit = (expected_source_delta.max(0.0) * 16.0)
            .max(clip_limit * 0.25)
            .min(clip_limit * 0.5);
        observed_delta >= quiet_limit
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
    fn adaptive_transient_newton_delta_limit(
        base_limit: Value,
        iteration: usize,
        has_vbic_excess_phase: bool,
    ) -> Value {
        if has_vbic_excess_phase || !(base_limit.is_finite() && base_limit > 0.0) {
            return base_limit;
        }

        let growth_stage = iteration.saturating_sub(4) / 4;
        let multiplier = 2.0_f64.powi(growth_stage.min(5) as i32);
        (base_limit * multiplier).min(MAX_ADAPTIVE_NEWTON_ITER_DELTA_V)
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
        recovery_cap_enabled: bool,
    ) -> Value {
        let mut dt = proposed_dt.min(remaining_time);
        if at_breakpoint || !recovery_cap_enabled {
            return dt;
        }

        if expected_source_delta >= SOURCE_ACTIVE_DELTA {
            // Only recovery paths get this extra source-following cap. Normal
            // accepted-step progression is already governed by ngspice-style
            // breakpoints and truncation; shrinking every smooth ramp step here
            // phase-shifts otherwise converged waveforms.
            let active_cap = (preferred_min_dt / 8.0).max(practical_min_dt);
            if dt > active_cap {
                dt = active_cap;
            }
        }

        dt
    }

    #[inline]
    fn should_apply_active_source_recovery_cap(force_accept_cooldown: usize) -> bool {
        force_accept_cooldown > 0
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
    fn effective_trapezoidal_order(method: IntegrationMethod, trap_order: u8) -> u8 {
        match method {
            IntegrationMethod::BackwardEuler => 1,
            IntegrationMethod::Gear2 => 2,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => trap_order.clamp(1, 2),
        }
    }

    #[inline]
    fn step_trapezoidal_order(
        method: IntegrationMethod,
        trap_order: u8,
        at_breakpoint: bool,
    ) -> u8 {
        if at_breakpoint
            && matches!(
                method,
                IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
            )
        {
            1
        } else {
            Self::effective_trapezoidal_order(method, trap_order)
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
    fn ngspice_predictor_charge(
        dt: Value,
        previous_dt: Value,
        q_prev: Value,
        q_prev_prev: Value,
    ) -> Option<Value> {
        if !(dt.is_finite() && dt > 0.0 && previous_dt.is_finite() && previous_dt > 0.0) {
            return None;
        }
        let xfact = dt / previous_dt;
        let predicted = (1.0 + xfact) * q_prev - xfact * q_prev_prev;
        predicted.is_finite().then_some(predicted)
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
        if !bjt.uses_vbic_dynamic_charges() {
            return Some(bjt.charge_snapshot(external[0], external[1], external[2], external[3]));
        }

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
        if !prev_dt.is_finite() || prev_dt <= 0.0 {
            return None;
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
        loop {
            for i in 0..=j {
                let denom = deltmp[i];
                if !denom.is_finite() || denom <= 0.0 {
                    return None;
                }
                diff[i] = (diff[i] - diff[i + 1]) / denom;
            }
            if j == 0 {
                break;
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
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
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
            let candidate_external = [vc, vb, ve, vs];
            let snapshot = vbic_snapshot_cache
                .get(idx)
                .copied()
                .flatten()
                .filter(|snapshot| {
                    Self::vbic_snapshot_matches_external_bias_exact(snapshot, &candidate_external)
                })
                .unwrap_or_else(|| bjt.charge_snapshot(vc, vb, ve, vs));

            for branch_idx in [
                BJT_QBE_BRANCH_INDEX,
                BJT_QBC_BRANCH_INDEX,
                BJT_QBCP_BRANCH_INDEX,
            ] {
                let branch = snapshot.branches[branch_idx];
                if !branch.is_active() {
                    continue;
                }
                let q_curr = branch.charge;
                if !q_curr.is_finite() {
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
            vbic_snapshot_cache,
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
        let mut trace_min: Option<(&str, &str, Value, Value, Value, Value, Value, Value)> = None;

        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (vgs_eval, vds_eval, vbs_eval) = mos.eval_branch_voltages_at(candidate_solution);
            let (vgs, vgd, vgb) = mos.gate_charge_branch_voltages_at(candidate_solution);
            let (cgs_half, cgd_half, cgb_half) =
                mos.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();

            for (
                branch,
                capacitance,
                voltage,
                voltage_prev,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_prev,
            ) in [
                (
                    "qgs",
                    cgs_half + history.capgs_prev_half[idx] + cgs_ov,
                    vgs,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.qgs_prev_prev_prev[idx],
                    history.cqgs_prev[idx],
                ),
                (
                    "qgd",
                    cgd_half + history.capgd_prev_half[idx] + cgd_ov,
                    vgd,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.qgd_prev_prev_prev[idx],
                    history.cqgd_prev[idx],
                ),
                (
                    "qgb",
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
                if std::env::var_os("RSPICE_TRACE_MOS6_BRANCH").is_some() && branch_limit < limit {
                    trace_min = Some((
                        mos.name.as_str(),
                        branch,
                        branch_limit,
                        capacitance,
                        voltage,
                        q_curr,
                        q_prev,
                        cq_curr,
                    ));
                }
                limit = limit.min(branch_limit);
            }
        }

        if std::env::var_os("RSPICE_TRACE_MOS6_BRANCH").is_some()
            && dt >= 1.0e-10
            && dt <= 5.5e-10
            && let Some((name, branch, branch_limit, cap, voltage, q_curr, q_prev, cq_curr)) =
                trace_min
        {
            eprintln!(
                "trace mosbranch dt={:.12e} order={} limit={:.12e} dev={} branch={} cap={:.12e} v={:.12e} q={:.12e} qprev={:.12e} cq={:.12e}",
                dt, trap_order, branch_limit, name, branch, cap, voltage, q_curr, q_prev, cq_curr
            );
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
    #[allow(clippy::too_many_arguments)]
    fn ngspice_device_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        bjt_history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        jfet_history: &JfetTransientHistory,
        mosfet_history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let capacitor_limit = if !circuit.capacitors.is_empty() {
            Self::capacitor_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                mosfet_history.accepted_dt_prev,
                mosfet_history.accepted_dt_prev_prev,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let bjt_limit = if !circuit.bjts.devices.is_empty() {
            Self::bjt_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                bjt_history,
                vbic_snapshot_cache,
                voltage_abstol,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let jfet_limit = if !suppress_gate_charge && !circuit.jfets.is_empty() {
            Self::jfet_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                jfet_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let mosfet_limit = if !suppress_gate_charge && !circuit.mosfets.is_empty() {
            Self::mosfet_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                mosfet_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };

        Self::min_truncation_limit(
            Self::min_truncation_limit(
                Self::min_truncation_limit(capacitor_limit, bjt_limit),
                jfet_limit,
            ),
            mosfet_limit,
        )
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
    fn promoted_trapezoidal_order_timestep_limit(
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
    ) -> Option<Value> {
        if !matches!(
            method,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
        ) {
            return None;
        }
        if !(dt.is_finite() && dt > 0.0) {
            return None;
        }
        // Match ngspice startup behavior: keep order-1 through the first accepted
        // transient step, then only promote when an order-2 truncation/LTE check
        // says the current timestep remains viable.
        if !(history.accepted_dt_prev.is_finite() && history.accepted_dt_prev > 0.0) {
            return None;
        }

        if let Some(limit) = Self::ngspice_device_truncation_limit(
            circuit,
            accepted_solution,
            method,
            2,
            dt,
            history,
            vbic_snapshot_cache,
            jfet_history,
            mosfet_history,
            false,
            voltage_abstol,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        ) {
            if Self::should_promote_ngspice_charge_truncation(limit, dt) {
                return Some(limit);
            }
            return None;
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
            return None;
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
        if candidate_scale >= 0.95 {
            Some(Value::INFINITY)
        } else {
            None
        }
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
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
        Self::promoted_trapezoidal_order_timestep_limit(
            circuit,
            accepted_solution,
            method,
            dt,
            is_strictly_linear_transient,
            history,
            jfet_history,
            mosfet_history,
            voltage_lte_estimator,
            vbic_charge_lte_estimator,
            vbic_snapshot_cache,
            voltage_abstol,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        )
        .is_some()
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
            internal[BJT_VCI_STATE_INDEX] - internal[BJT_VSI_STATE_INDEX],
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
            let (snapshot_reuse_abstol, snapshot_reuse_reltol) =
                Self::vbic_runtime_snapshot_reuse_tolerances(voltage_abstol, reltol);
            let cached_snapshot = vbic_snapshot_cache.get(idx).copied().flatten();
            let Some(snapshot) = Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
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
            ) else {
                vbic_snapshot_cache[idx] = None;
                continue;
            };

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
            let (base_static_g, base_static_i_eq) =
                Self::vbic_static_stamped_external_system(bjt, &[vc, vb, ve, vs]);
            vbic_snapshot_cache[idx] = Some(snapshot);
            let Some((y_total, reduced_i_eq)) =
                Self::vbic_reduce_transient_external_system(&linearization)
            else {
                vbic_snapshot_cache[idx] = None;
                continue;
            };

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
        predict_gate_charge: bool,
    ) {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let gate_charge_prediction_dt = predict_gate_charge.then_some(history.accepted_dt_prev);
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
                let (geq_gs, ieq_gs, _qgs_curr, _cqgs_curr) = if let Some(q_pred) =
                    gate_charge_prediction_dt.and_then(|previous_dt| {
                        Self::ngspice_predictor_charge(
                            dt,
                            previous_dt,
                            history.qgs_prev[idx],
                            history.qgs_prev_prev[idx],
                        )
                    }) {
                    Self::nonlinear_charge_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgs,
                        vgs,
                        q_pred,
                        history.qgs_prev[idx],
                        history.qgs_prev_prev[idx],
                        history.cqgs_prev[idx],
                    )
                } else {
                    Self::jfet_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgs,
                        vgs,
                        history.vgs_prev[idx],
                        history.qgs_prev[idx],
                        history.qgs_prev_prev[idx],
                        history.cqgs_prev[idx],
                    )
                };
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

                let (geq_gd, ieq_gd, _qgd_curr, _cqgd_curr) = if let Some(q_pred) =
                    gate_charge_prediction_dt.and_then(|previous_dt| {
                        Self::ngspice_predictor_charge(
                            dt,
                            previous_dt,
                            history.qgd_prev[idx],
                            history.qgd_prev_prev[idx],
                        )
                    }) {
                    Self::nonlinear_charge_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgd,
                        vgd,
                        q_pred,
                        history.qgd_prev[idx],
                        history.qgd_prev_prev[idx],
                        history.cqgd_prev[idx],
                    )
                } else {
                    Self::jfet_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgd,
                        vgd,
                        history.vgd_prev[idx],
                        history.qgd_prev[idx],
                        history.qgd_prev_prev[idx],
                        history.cqgd_prev[idx],
                    )
                };
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

                let (geq_gb, ieq_gb, _qgb_curr, _cqgb_curr) = if let Some(q_pred) =
                    gate_charge_prediction_dt.and_then(|previous_dt| {
                        Self::ngspice_predictor_charge(
                            dt,
                            previous_dt,
                            history.qgb_prev[idx],
                            history.qgb_prev_prev[idx],
                        )
                    }) {
                    Self::nonlinear_charge_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgb,
                        vgb,
                        q_pred,
                        history.qgb_prev[idx],
                        history.qgb_prev_prev[idx],
                        history.cqgb_prev[idx],
                    )
                } else {
                    Self::jfet_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgb,
                        vgb,
                        history.vgb_prev[idx],
                        history.qgb_prev[idx],
                        history.qgb_prev_prev[idx],
                        history.cqgb_prev[idx],
                    )
                };
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
        source_activity_growth_cap_enabled: bool,
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

        let growth_limit = if source_activity_growth_cap_enabled {
            1.5
        } else {
            2.0
        };
        let mut next_dt = if scale > 1.0 {
            (dt * scale.min(growth_limit)).min(max_step)
        } else {
            (dt * 1.25).min(max_step)
        };
        if source_activity_growth_cap_enabled
            && expected_source_delta.is_finite()
            && expected_source_delta > 0.0
        {
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
        rejected_dt: Value,
        max_step: Value,
    ) -> Value {
        let rejected_cap = if rejected_dt.is_finite() && rejected_dt > 0.0 {
            rejected_dt.min(max_step)
        } else {
            max_step
        };
        let mut dt = proposed_dt.min(rejected_cap);
        if let Some(floor) = retry_floor_dt
            .filter(|floor| floor.is_finite() && *floor > 0.0)
            .map(|floor| floor.min(rejected_cap))
            .filter(|floor| *floor < rejected_cap * 0.999)
        {
            dt = dt.max(floor);
        }
        dt
    }

    #[inline]
    fn is_at_effective_retry_minimum(
        timestep: &TimestepController,
        _retry_floor_dt: Option<Value>,
    ) -> bool {
        // The BJT retry floor is intentionally soft: it may damp the first
        // retreat from a large failed step, but it must not masquerade as the
        // solver's true minimum and trigger force-accept before Newton has had
        // a chance to retry at smaller physical timesteps.
        timestep.is_at_minimum()
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
        if let Some(limit) =
            vbic_exact_limit.filter(|limit| limit.is_finite() && *limit > dt * 1.001)
        {
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
            let snapshot_reuse_abstol = voltage_abstol.min(VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL);
            let snapshot_reuse_reltol = voltage_reltol.min(VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL);
            let cached_snapshot = vbic_snapshots
                .and_then(|cache| cache.get(idx))
                .copied()
                .flatten();
            let Some(snapshot) = Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
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
            ) else {
                continue;
            };
            let (legacy_vbe, legacy_vbc, legacy_vcs) =
                Self::legacy_bjt_charge_branch_voltages(&snapshot);
            let mut cq_currents = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
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
                cq_currents[branch_idx] = cq_curr;
            }
            bjt_history.dynamic_internal_prev_prev[idx] = bjt_history.dynamic_internal_prev[idx];
            bjt_history.dynamic_internal_prev[idx] = snapshot.reduction.internal_voltages;
            let predictor_linear = Self::vbic_predictor_linear_branch_state(
                bjt,
                external,
                snapshot.reduction.internal_voltages,
            );
            bjt_history.dynamic_linear_prev_prev[idx] = bjt_history.dynamic_linear_prev[idx];
            bjt_history.dynamic_linear_prev[idx] = predictor_linear;

            bjt_history.vbe_prev_prev[idx] = bjt_history.vbe_prev[idx];
            bjt_history.vbe_prev[idx] = legacy_vbe;
            bjt_history.ibe_prev[idx] = cq_currents[BJT_QBE_BRANCH_INDEX];
            bjt_history.vbc_prev_prev[idx] = bjt_history.vbc_prev[idx];
            bjt_history.vbc_prev[idx] = legacy_vbc;
            bjt_history.ibc_prev[idx] = cq_currents[BJT_QBC_BRANCH_INDEX];
            bjt_history.vcs_prev_prev[idx] = bjt_history.vcs_prev[idx];
            bjt_history.vcs_prev[idx] = legacy_vcs;
            bjt_history.ics_prev[idx] = cq_currents[BJT_QBCP_BRANCH_INDEX];
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
            false,
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
                Self::should_apply_active_source_recovery_cap(force_accept_cooldown),
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
                Self::step_trapezoidal_order(current_method, trap_order, at_breakpoint);
            let coeff = CompanionCoefficients::for_method(Self::effective_companion_method(
                current_method,
                step_trap_order,
            ));
            let suppress_gate_charge = false;

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
            let mut had_solver_candidate = false;

            // Newton-Raphson iteration for this timestep.
            // Transient nonlinear regions (e.g., BJT turn-on) often need more
            // iterations than DC. Use a higher budget here to reduce force-accept.
            let tran_max_iterations = Self::transient_newton_iteration_budget(
                self.config.max_iterations,
                has_vbic_excess_phase,
                retry_count,
            );
            let first_transient_solve_step =
                Self::should_skip_post_accept_timestep_control_on_first_step(result.time.len());
            let mut converged = false;
            if expected_source_delta <= SOURCE_ACTIVE_DELTA
                && !circuit.has_xspice_devices()
                && self.transient_nonlinear_residual_converged(
                    &mut circuit,
                    &mut matrix,
                    &mut rhs,
                    &solution,
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
            {
                new_solution.clone_from(&solution);
                nonlinear_state_matches_new_solution = circuit.has_nonlinear_devices();
                converged = true;
            }
            for _iter in 0..tran_max_iterations {
                if converged {
                    break;
                }
                if _iter % ABORT_CHECK_INTERVAL == 0 && abort.is_aborted() {
                    log::info!(
                        "Transient simulation aborted during Newton solve at t={:.3e}s ({:.1}% complete, {} iterations)",
                        t,
                        (t / tstop) * 100.0,
                        total_iterations
                    );
                    return Err(SimulationError::Aborted);
                }
                let iteration_delta_limit = Self::adaptive_transient_newton_delta_limit(
                    newton_step_delta_limit,
                    _iter,
                    has_vbic_excess_phase,
                );
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
                    _iter == 0
                        && !first_transient_solve_step
                        && mosfet_history.accepted_dt_prev.is_finite()
                        && mosfet_history.accepted_dt_prev > 0.0,
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

                        let mut first_bad_index = None;
                        let mut first_bad_value = 0.0;
                        for (i, v) in sol.iter_mut().enumerate() {
                            let magnitude_limit = if i < num_nodes {
                                MAX_VOLTAGE
                            } else {
                                MAX_BRANCH_STATE_MAGNITUDE
                            };
                            if !v.is_finite() {
                                if first_bad_index.is_none() {
                                    first_bad_index = Some(i);
                                    first_bad_value = *v;
                                }
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
                                if first_bad_index.is_none() {
                                    first_bad_index = Some(i);
                                    first_bad_value = *v;
                                }
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
                                        iteration_delta_limit
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
                                iteration_delta_limit,
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
                                iteration_delta_limit,
                            );
                            if vbic_damped {
                                circuit.enforce_ideal_voltage_constraints(&mut sol, t + dt);
                            }
                            Self::clip_ideal_output_common_modes(
                                &solution,
                                &mut sol,
                                iteration_delta_limit,
                                &ideal_output_pairs,
                            );
                        }

                        // If this Newton step was numerically bad, keep the sanitized
                        // candidate and continue Newton iterations.
                        if has_bad_values {
                            if std::env::var_os("RSPICE_TRACE_RTLINV_EDGE").is_some()
                                && t >= 2.0e-9
                                && (_iter < 8 || _iter % 32 == 0)
                            {
                                let max_abs =
                                    sol.iter().map(|value| value.abs()).fold(0.0, Value::max);
                                let max_dv =
                                    Self::max_abs_delta_prefix(&new_solution, &sol, num_nodes);
                                eprintln!(
                                    "trace bad_candidate t0={:.12e} t1={:.12e} dt={:.12e} iter={} retry={} bad_idx={:?} bad_value={:.12e} max_abs={:.12e} node_step={:.12e}",
                                    t,
                                    t + dt,
                                    dt,
                                    _iter,
                                    retry_count,
                                    first_bad_index,
                                    first_bad_value,
                                    max_abs,
                                    max_dv
                                );
                            }
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
                        let trace_previous_newton_solution =
                            if std::env::var_os("RSPICE_TRACE_RTLINV_EDGE").is_some()
                                && t >= 2.0e-9
                                && (_iter < 8 || _iter % 32 == 0)
                            {
                                Some(new_solution.clone())
                            } else {
                                None
                            };
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

                        if std::env::var_os("RSPICE_TRACE_RTLINV_EDGE").is_some()
                            && t >= 2.0e-9
                            && (_iter < 8 || _iter % 32 == 0)
                        {
                            let max_dv =
                                Self::max_abs_delta_prefix(&solution, &new_solution, num_nodes);
                            let previous_newton_solution =
                                trace_previous_newton_solution.as_ref().unwrap_or(&solution);
                            let max_iter_step = Self::max_abs_delta_prefix(
                                previous_newton_solution,
                                &new_solution,
                                num_nodes,
                            );
                            let top_nodes = Self::top_abs_delta_prefix_named(
                                &solution,
                                &new_solution,
                                &result.node_names,
                                num_nodes,
                                4,
                            );
                            let top_iter_nodes = Self::top_abs_delta_prefix_named(
                                previous_newton_solution,
                                &new_solution,
                                &result.node_names,
                                num_nodes,
                                4,
                            );
                            eprintln!(
                                "trace newton t0={:.12e} t1={:.12e} dt={:.12e} iter={} retry={} vconv={} dev={} static_dev={} hidden={} lin_res={} nonlin_res={} max_from_prev={:.12e} iter_step={:.12e} top_nodes={:?} top_iter={:?}",
                                t,
                                t + dt,
                                dt,
                                _iter,
                                retry_count,
                                voltage_converged,
                                device_converged,
                                static_device_converged,
                                hidden_device_converged,
                                linearized_residual_converged,
                                nonlinear_residual_converged,
                                max_dv,
                                max_iter_step,
                                top_nodes,
                                top_iter_nodes
                            );
                        }

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
                        had_solver_candidate = false;
                        if std::env::var_os("RSPICE_TRACE_RTLINV_EDGE").is_some() && t >= 2.0e-9 {
                            eprintln!(
                                "trace solve_err t0={:.12e} t1={:.12e} dt={:.12e} retry={} err={}",
                                t,
                                t + dt,
                                dt,
                                retry_count,
                                e
                            );
                        }
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
                if std::env::var_os("RSPICE_TRACE_RTLINV_EDGE").is_some() && t >= 2.0e-9 {
                    eprintln!(
                        "trace nonconv t0={:.12e} t1={:.12e} dt={:.12e} retry={} next_dt={:.12e}",
                        t,
                        t + dt,
                        dt,
                        retry_count,
                        Self::nonconvergence_retry_timestep(dt, max_step)
                    );
                }

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
                        dt,
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
                        timestep.force_step(restart_dt.min(timestep.dt()).min(max_step));
                    }

                    // FORCE-ACCEPT: Use the bounded Newton candidate as-is.
                    // Project ideal-source constraints first, then clip source-free
                    // node movement so the committed state stays physically bounded.
                    new_solution = bounded_force_candidate;

                    if circuit.has_nonlinear_devices() {
                        circuit.update_nonlinear(&new_solution);
                    }

                    let method_after_step = current_integration_method(&trapgear);
                    let accepted_step_trap_order =
                        Self::effective_trapezoidal_order(current_method, 1);
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
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_timestep();
                    }

                    solution.clone_from(&new_solution);
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
                    if std::env::var_os("RSPICE_TRACE_RTLINV_EDGE").is_some() && t >= 2.0e-9 {
                        eprintln!(
                            "trace force_accept accepted_t={:.12e} dt={:.12e} next_dt={:.12e} node_dv={:.12e} full_dv={:.12e} stale={} stagnant={} no_candidate={} trunc_limit={:?} retry_count={}",
                            t,
                            dt,
                            next_force_dt,
                            force_candidate_node_delta,
                            force_candidate_full_delta,
                            stale_force_candidate,
                            stagnant_force_candidate,
                            !had_solver_candidate,
                            force_accept_device_truncation_limit,
                            retry_count
                        );
                    }
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
                    &vbic_snapshot_cache,
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
            if std::env::var_os("RSPICE_TRACE_MOS6").is_some() && t + dt <= 5.0e-9 {
                eprintln!(
                    "trace preaccept t0={:.12e} t1={:.12e} dt={:.12e} order={} first={} limits cap={:?} mos={:?} dev={:?}",
                    t,
                    t + dt,
                    dt,
                    step_trap_order,
                    first_accepted_transient_step,
                    capacitor_truncation_limit,
                    mosfet_truncation_limit,
                    device_truncation_limit
                );
            }

            if let Some(limit) = device_truncation_limit
                && Self::should_retry_ngspice_charge_truncation(limit, dt)
            {
                let retry_dt = limit.clamp(timestep.hard_min_dt(), max_step);
                let can_shrink = retry_dt < dt * 0.999;
                let retry_budget_available = retry_count < MAX_RETRIES;
                if !can_shrink || !retry_budget_available {
                    static DEVICE_TRUNC_MIN_ACCEPT_LOG_COUNT: std::sync::atomic::AtomicUsize =
                        std::sync::atomic::AtomicUsize::new(0);
                    let log_count = DEVICE_TRUNC_MIN_ACCEPT_LOG_COUNT
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_count < 20 {
                        log::warn!(
                            "Device charge truncation reached minimum retry step at t={:.6e}, dt={:.3e}, limit={:.3e}, retry_count={}; accepting converged solution",
                            t,
                            dt,
                            limit,
                            retry_count
                        );
                    }
                } else {
                    static DEVICE_TRUNC_REJECT_LOG_COUNT: std::sync::atomic::AtomicUsize =
                        std::sync::atomic::AtomicUsize::new(0);
                    let log_count = DEVICE_TRUNC_REJECT_LOG_COUNT
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_count < 40 || (t > 9.5e-8 && dt < 1.0e-15) {
                        log::warn!(
                            "Device charge truncation reject at t={:.6e}, dt={:.3e}, limit={:.3e}, cap={:?}, bjt={:?}, jfet={:?}, mos={:?}, method={:?}, order={}",
                            t,
                            dt,
                            limit,
                            capacitor_truncation_limit,
                            bjt_truncation_limit,
                            jfet_truncation_limit,
                            mosfet_truncation_limit,
                            current_method,
                            step_trap_order
                        );
                    }
                    retry_count += 1;
                    // Match ngspice truncation retries: keep the current integration
                    // order and only reduce the timestep.
                    trap_order =
                        Self::trapezoidal_order_after_timestep_control_reject(step_trap_order);
                    timestep.force_step(retry_dt);
                    continue;
                }
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
                if std::env::var_os("RSPICE_TRACE_RTLINV_EDGE").is_some() && t >= 2.0e-9 {
                    eprintln!(
                        "trace lte_reject t0={:.12e} t1={:.12e} dt={:.12e} retry={} lte={:.12e} scale={:.12e} order={} dev_limit={:?} bjt_limit={:?}",
                        t,
                        t + dt,
                        dt,
                        retry_count,
                        lte,
                        lte_scale,
                        step_trap_order,
                        device_truncation_limit,
                        bjt_truncation_limit
                    );
                }
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
                retry_count += 1;
                // LTE/truncation rejects in ngspice retry the same order at a
                // smaller timestep instead of forcing trapezoidal back to order 1.
                trap_order = Self::trapezoidal_order_after_timestep_control_reject(step_trap_order);
                timestep.adjust(lte / lte_scale);
                let clamped_retry_dt = Self::apply_retry_timestep_floor(
                    timestep.dt(),
                    legacy_bjt_retry_floor_dt,
                    dt,
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
                        timestep.force_step(restart_dt.min(timestep.dt()).min(max_step));
                    }
                    new_solution = bounded_force_candidate;

                    if circuit.has_nonlinear_devices() {
                        circuit.update_nonlinear(&new_solution);
                    }

                    let method_after_step = current_integration_method(&trapgear);
                    let accepted_step_trap_order =
                        Self::effective_trapezoidal_order(current_method, 1);
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
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_timestep();
                    }

                    solution.clone_from(&new_solution);
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
                continue;
            }
            stale_accept_count = 0;

            // Accept this timestep
            t += dt;
            let hit_breakpoint = at_breakpoint || breakpoints.at_breakpoint(t);
            static ACCEPT_TRACE_COUNT: std::sync::atomic::AtomicUsize =
                std::sync::atomic::AtomicUsize::new(0);
            let accept_trace_count =
                ACCEPT_TRACE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let trace_accept_edge = t >= 1.8e-9 && t <= 4.5e-9 && accept_trace_count < 256;
            if std::env::var_os("RSPICE_TRACE_ACCEPT").is_some()
                && (result.time.len() <= 16 || trace_accept_edge)
            {
                let node_delta = Self::max_abs_delta_prefix(&solution, &new_solution, num_nodes);
                let top_nodes = Self::top_abs_delta_prefix_named(
                    &solution,
                    &new_solution,
                    &result.node_names,
                    num_nodes,
                    6,
                );
                eprintln!(
                    "trace accept t={:.12e} dt={:.12e} order={} node_delta={:.12e} top_nodes={:?}",
                    t, dt, step_trap_order, node_delta, top_nodes
                );
            }
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
            }

            let promoted_trapezoidal_order_limit = if !first_accepted_transient_step
                && matches!(
                    current_method,
                    IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                )
                && !hit_breakpoint
                && !Self::should_hold_vbic_excess_phase_first_order(
                    has_vbic_excess_phase,
                    t,
                    hinted_max_step,
                    smallest_vbic_excess_phase_td,
                ) {
                Self::promoted_trapezoidal_order_timestep_limit(
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
            } else {
                None
            };

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
                    Self::should_apply_active_source_recovery_cap(force_accept_cooldown),
                    Some(lte_scale),
                );
            }
            if hit_breakpoint {
                let restart_dt = breakpoints.mark_breakpoint_solved(t);
                timestep.force_step(restart_dt.min(timestep.dt()).min(max_step));
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
                let should_promote = promoted_trapezoidal_order_limit.is_some();
                trap_order = Self::next_trapezoidal_order_after_accepted_step(
                    step_trap_order,
                    hit_breakpoint,
                    should_promote,
                );
                if trap_order == 2
                    && let Some(limit) = promoted_trapezoidal_order_limit
                    && limit.is_finite()
                    && limit > 0.0
                    && limit + 1e-18 < timestep.dt()
                {
                    timestep.force_step(limit);
                }
            }
            if std::env::var_os("RSPICE_TRACE_MOS6").is_some() && t <= 5.0e-9 {
                eprintln!(
                    "trace postaccept t={:.12e} dt={:.12e} step_order={} next_order={} next_dt={:.12e} promoted={:?} hit_bp={}",
                    t,
                    dt,
                    step_trap_order,
                    trap_order,
                    timestep.dt(),
                    promoted_trapezoidal_order_limit,
                    hit_breakpoint
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
