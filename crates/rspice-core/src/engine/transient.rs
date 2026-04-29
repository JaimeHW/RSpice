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

mod breakpoints;
mod companion_stamps;
mod residual;
mod startup;
mod state;
mod step_control;
mod truncation;
mod vbic;

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
/// Initial global trust-region limit for the ngspice legacy-BJT backend.
///
/// Legacy BJTs use ngspice-style local pnjlim limiting internally; this wider
/// nodal leash lets sharp switching steps converge without removing the global
/// guardrail completely.
const LEGACY_NGSPICE_BJT_NEWTON_ITER_DELTA_V: Value = 1.5e-2;
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
/// Largest single source movement to allow on proactive nonlinear ramp tracking.
const SOURCE_RAMP_TRACKING_DELTA: Value = 5e-2;
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
struct TrapezoidalOrderTrial {
    limit: Value,
    promote: bool,
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
        let legacy_ngspice_bjt_only_nonlinearity = Self::legacy_bjt_ngspice_backend_enabled()
            && !circuit.bjts.is_empty()
            && circuit
                .bjts
                .devices
                .iter()
                .all(|bjt| !bjt.uses_vbic_dynamic_charges())
            && circuit.diodes.is_empty()
            && circuit.mosfets.is_empty()
            && circuit.jfets.is_empty()
            && circuit.vswitches.is_empty()
            && circuit.iswitches.is_empty()
            && !circuit.has_xspice_devices()
            && {
                #[cfg(feature = "veriloga")]
                {
                    !circuit.has_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga"))]
                {
                    true
                }
            };
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
            let (dt, mut at_breakpoint) = breakpoints.limit_step(t, timestep.dt());
            let mut dt = dt.min(tstop - t); // Don't overshoot tstop
            let mut expected_source_delta = Self::max_expected_source_delta(&circuit, t, t + dt);
            let interior_source_delta = if at_breakpoint && dt.is_finite() && dt > 0.0 {
                Self::max_expected_source_delta(&circuit, t, t + 0.5 * dt)
            } else {
                expected_source_delta
            };
            let biased_dt = Self::bias_transient_step_for_source_activity(
                dt,
                tstop - t,
                at_breakpoint,
                expected_source_delta,
                interior_source_delta,
                practical_min,
                timestep.preferred_min_dt(),
                Self::should_apply_active_source_recovery_cap(force_accept_cooldown),
                requires_conservative_nonlinear_limiting,
            );
            if biased_dt + 1e-30 < dt {
                dt = biased_dt;
                at_breakpoint = breakpoints.at_breakpoint(t + dt);
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
                at_breakpoint = breakpoints.at_breakpoint(t + dt);
                expected_source_delta = Self::max_expected_source_delta(&circuit, t, t + dt);
            }
            if fixed_method.is_none() {
                trapgear.set_at_breakpoint(at_breakpoint);
            } else if let Some(method) = fixed_method {
                trapgear.force_method(method);
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
                if legacy_ngspice_bjt_only_nonlinearity {
                    LEGACY_NGSPICE_BJT_NEWTON_ITER_DELTA_V
                } else {
                    MAX_NEWTON_ITER_DELTA_V
                },
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
            // Classic SPICE transient analysis uses the transient-specific ITL4
            // budget, not the DC operating-point iteration limit.
            let tran_max_iterations = Self::transient_newton_iteration_budget(
                self.config.transient_max_iterations,
                has_vbic_excess_phase,
                retry_count,
            );
            let mut converged = false;
            // Reusing the accepted state is only valid when a full restamp
            // proves the linear companion equations are still satisfied.
            // Nonlinear compact devices carry limiter state and charge history
            // that must be re-solved at each accepted transient point.
            if !circuit.has_nonlinear_devices()
                && expected_source_delta <= SOURCE_ACTIVE_DELTA
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
                        had_solver_candidate = false;
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
                        t = breakpoints.snap_to_breakpoint(t);
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
                        t = breakpoints.snap_to_breakpoint(t);
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
            if hit_breakpoint {
                t = breakpoints.snap_to_breakpoint(t);
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

            let trapezoidal_order_trial = if !first_accepted_transient_step
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
                Self::trapezoidal_order_trial_timestep_limit(
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
                let should_promote = trapezoidal_order_trial.is_some_and(|trial| trial.promote);
                trap_order = Self::next_trapezoidal_order_after_accepted_step(
                    step_trap_order,
                    hit_breakpoint,
                    should_promote,
                );
                if let Some(trial) = trapezoidal_order_trial
                    && trial.limit.is_finite()
                    && trial.limit > 0.0
                    && trial.limit + 1e-18 < timestep.dt()
                {
                    timestep.force_step(trial.limit);
                }
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
