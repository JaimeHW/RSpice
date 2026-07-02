//! Transient Time-Domain Analysis
//!
//! This module provides time-domain simulation using:
//! - Adaptive timestep control with LTE estimation
//! - TrapGear method switching for stability
//! - Optional waveform compression for long simulations
//! - Cooperative abort for responsive cancellation

#![allow(clippy::too_many_arguments)]
use super::{Engine, SimulationError, SpiceDialect, TransientResult};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::transient::{
    BreakpointManager, CompanionCoefficients, IntegrationMethod, LteEstimator, TimestepController,
    TrapGearController,
};
use crate::analysis::waveform::{CompressionConfig, TransientResultCompressed, WaveformRecorder};
use crate::device::semiconductor::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeBranch,
    BjtChargeSnapshot,
};
use crate::netlist::AnalysisCommand;
use crate::{Netlist, Value};
use std::collections::HashMap;

/// Per-iteration Newton merit tracing (`RSPICE_NEWTON_DEBUG=1`), the
/// transient-Newton sibling of `RSPICE_LTE_DEBUG`.
fn newton_merit_debug_enabled() -> bool {
    std::env::var_os("RSPICE_NEWTON_DEBUG").is_some()
}

mod breakpoints;
mod checkpoint;
mod companion_stamps;
pub(self) use companion_stamps::TwoTerminalStampSlots;
mod charge_stamper;
pub(self) use charge_stamper::StaticMatrixChargeStamper;
mod globalization;
mod noise;
mod rescue;
mod residual;
mod startup;
mod state;
mod state_advanced_mos;
mod state_commit;
mod state_recovery;
mod state_transmission_lines;
mod step_control;
mod truncation;
mod vbic;

pub use checkpoint::{TransientCheckpoint, netlist_fingerprint};

mod history;
pub(self) use history::*;

#[derive(Debug, Clone, Copy)]
struct DerivedTransientBranchCurrent {
    kind: DerivedTransientBranchCurrentKind,
    index: usize,
}

#[derive(Debug, Clone, Copy)]
enum DerivedTransientBranchCurrentKind {
    LinearResistor,
    BehavioralCurrentSource,
}

impl Engine {
    fn derived_transient_branch_currents(
        circuit: &crate::circuit::CircuitData,
        existing_branch_names: &[String],
    ) -> Vec<DerivedTransientBranchCurrent> {
        let mut derived = Vec::new();
        for (index, name) in circuit.resistors.names.iter().enumerate() {
            if existing_branch_names
                .iter()
                .any(|existing| existing.eq_ignore_ascii_case(name))
            {
                continue;
            }
            derived.push(DerivedTransientBranchCurrent {
                kind: DerivedTransientBranchCurrentKind::LinearResistor,
                index,
            });
        }
        for (index, source) in circuit
            .behavioral_sources
            .current_sources
            .iter()
            .enumerate()
        {
            if existing_branch_names
                .iter()
                .any(|existing| existing.eq_ignore_ascii_case(&source.name))
                || derived.iter().any(|&branch| {
                    Self::derived_transient_branch_name(circuit, branch)
                        .eq_ignore_ascii_case(&source.name)
                })
            {
                continue;
            }
            derived.push(DerivedTransientBranchCurrent {
                kind: DerivedTransientBranchCurrentKind::BehavioralCurrentSource,
                index,
            });
        }
        derived
    }

    fn derived_transient_branch_name(
        circuit: &crate::circuit::CircuitData,
        branch: DerivedTransientBranchCurrent,
    ) -> String {
        match branch.kind {
            DerivedTransientBranchCurrentKind::LinearResistor => {
                circuit.resistors.names[branch.index].clone()
            }
            DerivedTransientBranchCurrentKind::BehavioralCurrentSource => {
                circuit.behavioral_sources.current_sources[branch.index]
                    .name
                    .clone()
            }
        }
    }

    fn derived_transient_branch_current(
        circuit: &mut crate::circuit::CircuitData,
        solution: &[Value],
        time: Value,
        branch: DerivedTransientBranchCurrent,
    ) -> Value {
        match branch.kind {
            DerivedTransientBranchCurrentKind::LinearResistor => {
                let stamp = circuit.resistors.stamps[branch.index];
                let v_pos = if stamp.pp.row == 0 {
                    0.0
                } else {
                    solution.get(stamp.pp.row - 1).copied().unwrap_or(0.0)
                };
                let v_neg = if stamp.nn.row == 0 {
                    0.0
                } else {
                    solution.get(stamp.nn.row - 1).copied().unwrap_or(0.0)
                };
                (v_pos - v_neg) * circuit.resistors.conductances[branch.index]
            }
            DerivedTransientBranchCurrentKind::BehavioralCurrentSource => {
                circuit.behavioral_sources.current_sources[branch.index].evaluate(solution, time)
            }
        }
    }

    fn initial_transient_branch_currents(
        circuit: &mut crate::circuit::CircuitData,
        solution: &[Value],
        num_nodes: usize,
        time: Value,
        derived_branches: &[DerivedTransientBranchCurrent],
    ) -> Vec<Vec<Value>> {
        let mut currents: Vec<Vec<Value>> = (0..circuit.num_branches())
            .map(|i| vec![solution.get(num_nodes + i).copied().unwrap_or(0.0)])
            .collect();
        currents.extend(derived_branches.iter().map(|&branch| {
            vec![Self::derived_transient_branch_current(
                circuit, solution, time, branch,
            )]
        }));
        currents
    }

    fn record_transient_solution_sample(
        result: &mut TransientResult,
        circuit: &mut crate::circuit::CircuitData,
        solution: &[Value],
        num_nodes: usize,
        time: Value,
        derived_branches: &[DerivedTransientBranchCurrent],
    ) {
        result.time.push(time);
        for (i, voltages) in result.voltages.iter_mut().enumerate() {
            voltages.push(solution.get(i).copied().unwrap_or(0.0));
        }

        let solved_branch_count = circuit.num_branches();
        for (i, currents) in result
            .branch_currents
            .iter_mut()
            .take(solved_branch_count)
            .enumerate()
        {
            currents.push(solution.get(num_nodes + i).copied().unwrap_or(0.0));
        }
        for (branch, currents) in derived_branches
            .iter()
            .zip(result.branch_currents.iter_mut().skip(solved_branch_count))
        {
            currents.push(Self::derived_transient_branch_current(
                circuit, solution, time, *branch,
            ));
        }
    }

    fn apply_capacitor_element_initial_conditions(
        circuit: &crate::circuit::CircuitData,
        solution: &mut [Value],
    ) {
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            if let Some(ic) = circuit.capacitors.ic[cap_idx] {
                let np = cap.pp.row;
                let nn = cap.nn.row;
                if np != 0 {
                    let base = if nn != 0 { solution[nn - 1] } else { 0.0 };
                    solution[np - 1] = base + ic;
                } else if nn != 0 {
                    solution[nn - 1] = -ic;
                }
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
        validate_transient_window(tstop, max_step)?;
        let engine = self.resolved_for_netlist(netlist);
        // TRNOISE sources expand into seeded, deterministic PWL sample
        // trains covering [0, tstop] before circuit construction; decks
        // without noise sources pass through untouched (no clone).
        match noise::expand_transient_noise(netlist, tstop).map_err(SimulationError::Circuit)? {
            Some(expanded) => {
                engine.run_tran_with_abort_resolved(&expanded, tstop, max_step, abort)
            }
            None => engine.run_tran_with_abort_resolved(netlist, tstop, max_step, abort),
        }
    }

    /// Run a transient and additionally return the end-of-run state
    /// checkpoint, for segmented long simulations: save it, then extend
    /// later with [`Engine::run_tran_resume`].
    pub fn run_tran_checkpointed(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        let engine = self.resolved_for_netlist(netlist);
        match noise::expand_transient_noise(netlist, tstop).map_err(SimulationError::Circuit)? {
            Some(expanded) => {
                engine.run_tran_resolved_with_resume(&expanded, tstop, max_step, &NoAbort, None)
            }
            None => engine.run_tran_resolved_with_resume(netlist, tstop, max_step, &NoAbort, None),
        }
    }

    /// Continue a transient from a checkpoint to a later stop time.
    ///
    /// The checkpoint must come from the same netlist (fingerprint
    /// enforced). Continuation restores the exact linear-reactive state
    /// (capacitor/inductor integrator histories) and restarts integration
    /// at the checkpoint time with absolute-time source evaluation — the
    /// same numerical regime as a breakpoint restart. Nonlinear-device
    /// iteration memories and transmission-line delay histories re-derive
    /// from the restored solution on the first step.
    ///
    /// TRNOISE decks regenerate their sample train for each segment's
    /// horizon; run noise decks unsegmented when a single continuous
    /// sample path matters.
    pub fn run_tran_resume(
        &self,
        netlist: &Netlist,
        checkpoint: &TransientCheckpoint,
        tstop: Value,
        max_step: Value,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        checkpoint
            .validate_for(netlist)
            .map_err(SimulationError::Circuit)?;
        if !tstop.is_finite() || tstop <= checkpoint.time {
            return Err(SimulationError::Circuit(format!(
                "resume stop time {tstop:e} must exceed the checkpoint time {:e}",
                checkpoint.time
            )));
        }

        let engine = self.resolved_for_netlist(netlist);
        match noise::expand_transient_noise(netlist, tstop).map_err(SimulationError::Circuit)? {
            Some(expanded) => engine.run_tran_resolved_with_resume(
                &expanded,
                tstop,
                max_step,
                &NoAbort,
                Some(checkpoint),
            ),
            None => engine.run_tran_resolved_with_resume(
                netlist,
                tstop,
                max_step,
                &NoAbort,
                Some(checkpoint),
            ),
        }
    }

    #[inline]
    fn should_enable_nonlinear_source_ramp_cap(
        circuit: &crate::circuit::Circuit,
        requires_conservative_nonlinear_limiting: bool,
    ) -> bool {
        // Native TXL and distributed-RLGC/LTRA scalar lines keep accepted-point
        // histories that are governed by transmission-line breakpoints/truncation.
        // Applying the generic nonlinear source-ramp cap to those decks forces
        // source-edge micro-steps that ngspice LTRA does not take.
        requires_conservative_nonlinear_limiting
            && !circuit
                .tlines
                .iter()
                .any(|tl| tl.has_txl_runtime() || tl.has_distributed_rlgc())
    }

    fn run_tran_with_abort_resolved(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, SimulationError> {
        self.run_tran_resolved_with_resume(netlist, tstop, max_step, abort, None)
            .map(|(result, _)| result)
    }

    /// The transient integration body. `resume` injects a checkpointed
    /// state (time, solution, reactive histories) instead of the fresh
    /// initial solution — numerically a breakpoint restart at the
    /// checkpoint time. Returns the result together with the end-of-run
    /// checkpoint for segmented continuation.
    fn run_tran_resolved_with_resume(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
        resume: Option<&TransientCheckpoint>,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        let fingerprint = netlist_fingerprint(netlist);
        let mut circuit = self.build_circuit(netlist)?;
        if circuit.num_nodes() == 0 && circuit.num_branches() == 0 {
            let result = TransientResult {
                time: vec![0.0],
                voltages: Vec::new(),
                branch_currents: Vec::new(),
                num_nodes: 0,
                node_names: Vec::new(),
                branch_names: Vec::new(),
                digital_traces: Vec::new(),
            };
            let checkpoint = TransientCheckpoint::capture(fingerprint, 0.0, &[], &circuit);
            return Ok((result, checkpoint));
        }
        Self::ensure_supported_transient_dynamic_charges(&circuit)?;
        let hinted_max_step = circuit
            .transient_max_step_hint
            .map_or(max_step, |hint| max_step.min(hint));
        // Honor an explicitly configured maximum timestep (CLI --max-step,
        // netlist options, bindings). The default constant is a sentinel for
        // "not set" -- the same convention the CLI TOML merge uses -- so
        // default configs keep the caller-provided step unchanged.
        let config_max_step = self.config.max_timestep;
        let hinted_max_step = if config_max_step.is_finite()
            && config_max_step > 0.0
            && config_max_step != crate::constants::MAX_TIMESTEP
        {
            hinted_max_step.min(config_max_step)
        } else {
            hinted_max_step
        };
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let source_step_hint = Self::transient_source_step_hint(netlist, hinted_max_step);
        circuit
            .voltage_sources
            .set_transient_context(source_step_hint, tstop);
        circuit
            .current_sources
            .set_transient_context(source_step_hint, tstop);
        circuit.set_xspice_transient_context(source_step_hint, tstop);

        // `.TRAN ... UIC` skips the operating point: integration starts
        // from zero everywhere except user-supplied .IC node voltages
        // (applied below) and per-element IC= values (applied after the
        // reactive-history seeding), matching ngspice's MODEUIC semantics.
        let uic_requested = resume.is_none()
            && netlist.analyses.iter().any(|analysis| {
                matches!(
                    analysis,
                    crate::netlist::AnalysisCommand::Tran { uic: true, .. }
                )
            });

        // Get DC operating point as initial condition.
        let (mut solution, initial_solution_mode) = if uic_requested {
            log::info!("Transient UIC startup: skipping the operating point");
            (
                vec![0.0; circuit.matrix_size()],
                startup::InitialSolutionMode::LinearizedSeed,
            )
        } else {
            self.solve_transient_initial_solution(netlist, &mut circuit, &mut matrix, abort)?
        };
        if let Some(message) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE evaluation failed: {message}"
            )));
        }

        // Resume: the standard initial-solution machinery above still ran
        // (its device-state priming is wanted), but time, solution, and the
        // reactive histories come from the checkpoint.
        let resume_time = resume.map_or(0.0, |checkpoint| checkpoint.time);
        if let Some(checkpoint) = resume {
            if checkpoint.solution.len() != circuit.matrix_size() {
                return Err(SimulationError::Circuit(format!(
                    "checkpoint solution has {} unknowns, circuit has {}; \
                     the checkpoint belongs to a different elaboration",
                    checkpoint.solution.len(),
                    circuit.matrix_size()
                )));
            }
            solution.clone_from(&checkpoint.solution);
        }
        // .IC overrides describe the t=0 state; a resumed run is already
        // mid-trajectory, so they must not re-apply.
        let applied_ic = if resume.is_none() {
            self.apply_initial_condition_overrides(netlist, &circuit, &mut solution)
        } else {
            0
        };
        // UIC: per-element IC= values shape the t=0 state itself. Writing
        // them into the solution here means the recorded first point, the
        // device priming below, and the reactive-history seeding all see
        // one consistent state (ngspice holds UIC capacitors at their IC
        // value at the first instant).
        if uic_requested {
            Self::apply_capacitor_element_initial_conditions(&circuit, &mut solution);
            let num_nodes = circuit.num_nodes();
            for (ind_idx, ic) in circuit.inductors.ic.iter().enumerate() {
                let Some(ic) = ic else {
                    continue;
                };
                let branch = circuit.inductors.branch_indices[ind_idx];
                if branch == 0 {
                    continue;
                }
                if let Some(slot) = solution.get_mut(num_nodes + branch - 1) {
                    *slot = *ic;
                }
            }
        } else if self.config.spice_dialect == SpiceDialect::Xyce {
            Self::apply_capacitor_element_initial_conditions(&circuit, &mut solution);
        }
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&solution);
        }
        circuit.refresh_jiles_atherton_inductances(&solution);

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();
        let legacy_ngspice_bjt_only_nonlinearity = !circuit.bjts.is_empty()
            && circuit
                .bjts
                .devices
                .iter()
                .all(crate::device::Bjt::uses_legacy_gummel_poon)
            && circuit.diodes.is_empty()
            && circuit.mosfets.is_empty()
            && circuit.jfets.is_empty()
            && circuit.vswitches.is_empty()
            && circuit.iswitches.is_empty()
            && circuit.generic_switches.is_empty()
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
        let nonlinear_source_ramp_cap_enabled = Self::should_enable_nonlinear_source_ramp_cap(
            &circuit,
            requires_conservative_nonlinear_limiting,
        );
        let enforce_force_candidate_safety =
            requires_conservative_nonlinear_limiting || circuit.has_xspice_devices();
        let is_strictly_linear_transient =
            !circuit.has_nonlinear_devices() && !circuit.has_xspice_devices();
        // ngspice's flat transient Newton: when junction devices replace
        // their own iterate voltages (legacy GP pnjlim in update), the full
        // node step is the algorithm; per-iteration node-delta clamps walk
        // the junction against frozen nodes and livelock turn-on edges.
        let junction_owns_steps = Self::junction_limiting_owns_newton_steps(&circuit);
        let prefer_dense_solver = Self::should_prefer_dense_transient_solver(
            is_strictly_linear_transient,
            size,
            !circuit.multi_winding_transformers.is_empty()
                || !circuit.coupled_inductor_pairs.is_empty(),
            circuit.has_xspice_devices(),
        );

        // Initialize timestep controller. BJT-heavy decks need a smaller
        // startup timestep to capture fast bias settling before transitioning
        // to larger steps.
        let has_bjts = !circuit.bjts.devices.is_empty();
        let (_startup_div, min_div) = Self::startup_timestep_divisors(has_bjts);
        let tran_step_hint = netlist.analyses.iter().find_map(|analysis| match analysis {
            AnalysisCommand::Tran { step, .. } if step.is_finite() && *step > 0.0 => Some(*step),
            _ => None,
        });
        let mut breakpoints = BreakpointManager::new_with_tolerance(
            Self::ngspice_breakpoint_tolerance(hinted_max_step),
        );
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
        breakpoints.discard_through(resume_time);
        let initial_remaining_breakpoints = breakpoints
            .times()
            .iter()
            .filter(|&&time| time > resume_time)
            .count();
        let initial_step = Self::ngspice_t0_breakpoint_limited_initial_timestep(
            Self::ngspice_initial_timestep(tstop, tran_step_hint, hinted_max_step),
            breakpoints.next_after(resume_time),
        );
        let practical_min = Self::startup_practical_min_timestep(
            has_bjts,
            hinted_max_step,
            min_div,
            tran_step_hint,
        );
        let preferred_min_dt = practical_min.max(self.config.min_timestep.max(1e-15));
        let hard_min_dt = Self::ngspice_hard_min_timestep(hinted_max_step, preferred_min_dt);
        let mut timestep = TimestepController::new_with_preferred_min(
            initial_step,
            hard_min_dt,
            preferred_min_dt,
            hinted_max_step,
        );
        let mut dynamic_tline_breakpoints_added = 0_usize;
        let mut warned_dynamic_tline_breakpoint_cap = false;
        let mut lte_estimator =
            LteEstimator::with_tolerances(self.voltage_reltol(), self.voltage_abstol());

        // Floor-dt livelock detection: dozens of consecutive accepted points
        // at the hard-minimum timestep mean the step controller is trapped —
        // forced accepts feed the truncation estimators garbage history,
        // which pins the next dt right back at the floor (observed on
        // diode-bridge dead-zone crossings, where the cap-companion/bleeder
        // conductance ratio also exceeds f64 conditioning below ~1e-13 s).
        // The streak triggers a breakpoint-style integration restart;
        // re-triggering shortly after fails the run instead of spinning.
        const LIVELOCK_STREAK_RESTART: usize = 32;
        let livelock_dt_ceiling = (timestep.hard_min_dt() * 64.0).max(1e-22);
        // Two restarts at the same wall mean the restart cannot escape it;
        // a wall further along the time axis gets its own fresh attempt.
        let livelock_restart_spacing = (tstop * 1e-6).max(timestep.hard_min_dt() * 1e4);
        let mut livelock_streak = 0_usize;
        let mut livelock_last_restart_t: Option<Value> = None;
        let mut lte_warmup_skips = 0_u8;

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

        let mut branch_names = circuit.branch_names_sorted();
        let derived_branch_currents =
            Self::derived_transient_branch_currents(&circuit, &branch_names);
        branch_names.extend(
            derived_branch_currents
                .iter()
                .map(|&branch| Self::derived_transient_branch_name(&circuit, branch)),
        );
        let mut result = TransientResult {
            time: vec![resume_time],
            voltages: (0..num_nodes)
                .map(|i| vec![solution.get(i).copied().unwrap_or(0.0)])
                .collect(),
            branch_currents: Self::initial_transient_branch_currents(
                &mut circuit,
                &solution,
                num_nodes,
                resume_time,
                &derived_branch_currents,
            ),
            num_nodes,
            node_names,
            branch_names,
            digital_traces: Vec::new(),
        };
        let mut digital_snapshot = Vec::new();
        let mut digital_trace_indices = HashMap::new();
        circuit.fill_xspice_digital_snapshot(&mut digital_snapshot);
        result.record_digital_snapshot(resume_time, &digital_snapshot, &mut digital_trace_indices);
        let mut t = resume_time;
        let force_accept_protected_nodes = circuit.force_accept_protected_nodes();
        let mut voltage_lte_excluded_nodes = circuit.xspice_transient_voltage_lte_excluded_nodes();
        let mut solution_lte_excluded = vec![false; size];
        fn mark_voltage_lte_excluded(mask: &mut [bool], node: usize) {
            if node == 0 {
                return;
            }
            if let Some(slot) = mask.get_mut(node - 1) {
                *slot = true;
            }
        }
        for &idx in &voltage_lte_excluded_nodes {
            if let Some(slot) = solution_lte_excluded.get_mut(idx) {
                *slot = true;
            }
        }
        for idx in 0..circuit.voltage_sources.len() {
            mark_voltage_lte_excluded(
                &mut solution_lte_excluded,
                circuit.voltage_sources.node_pos[idx],
            );
            mark_voltage_lte_excluded(
                &mut solution_lte_excluded,
                circuit.voltage_sources.node_neg[idx],
            );
        }
        for idx in 0..circuit.vcvs.len() {
            mark_voltage_lte_excluded(&mut solution_lte_excluded, circuit.vcvs.node_pos[idx]);
            mark_voltage_lte_excluded(&mut solution_lte_excluded, circuit.vcvs.node_neg[idx]);
        }
        for idx in 0..circuit.ccvs.len() {
            mark_voltage_lte_excluded(&mut solution_lte_excluded, circuit.ccvs.node_pos[idx]);
            mark_voltage_lte_excluded(&mut solution_lte_excluded, circuit.ccvs.node_neg[idx]);
        }
        for source in &circuit.behavioral_sources.voltage_sources {
            if !source.excludes_output_from_transient_voltage_lte() {
                continue;
            }
            for node in [source.node_pos, source.node_neg] {
                mark_voltage_lte_excluded(&mut solution_lte_excluded, node);
            }
        }
        let mut propagated_behavioral_exclusion = true;
        while propagated_behavioral_exclusion {
            propagated_behavioral_exclusion = false;
            for source in &circuit.behavioral_sources.voltage_sources {
                let bound_indices: Vec<usize> = source.bound_solution_indices().collect();
                if bound_indices.is_empty()
                    || !bound_indices
                        .iter()
                        .all(|idx| solution_lte_excluded.get(*idx).copied().unwrap_or(false))
                {
                    continue;
                }

                for node in [source.node_pos, source.node_neg] {
                    let Some(idx) = node.checked_sub(1) else {
                        continue;
                    };
                    if let Some(slot) = solution_lte_excluded.get_mut(idx)
                        && !*slot
                    {
                        *slot = true;
                        propagated_behavioral_exclusion = true;
                    }
                }
            }
        }
        voltage_lte_excluded_nodes.extend(
            solution_lte_excluded
                .iter()
                .take(num_nodes)
                .enumerate()
                .filter_map(|(idx, excluded)| (*excluded).then_some(idx)),
        );
        voltage_lte_excluded_nodes.sort_unstable();
        voltage_lte_excluded_nodes.dedup();

        // Grid-locked stepping: the accepted times are exactly the
        // configured grid (filtered to points after the start), the dt
        // sequence is the successive deltas, and the run ends at the last
        // grid point. Breakpoints, source-activity biasing, LTE control,
        // and every timestep-controller proposal are bypassed while locked;
        // Newton (with its dt-preserving rescue) is the sole acceptance
        // authority, and a step that cannot converge on its imposed dt
        // fails the run instead of sub-stepping, because history-coupled
        // devices sample accepted points and internal sub-steps would
        // perturb the trajectory under validation.
        let locked_grid: Option<Vec<Value>> = self.config.locked_time_grid.as_ref().map(|grid| {
            let mut points: Vec<Value> = grid
                .iter()
                .copied()
                .filter(|&point| point.is_finite() && point > t + 1e-30)
                .collect();
            points.sort_by(|a, b| a.total_cmp(b));
            points.dedup_by(|a, b| (*a - *b).abs() <= 1e-12 * a.abs().max(*b).max(1e-30));
            points
        });
        let mut locked_cursor = 0usize;
        let tstop = match locked_grid.as_ref().and_then(|grid| grid.last()) {
            Some(&last) => last.min(tstop),
            None => tstop,
        };
        const LOCKED_MAX_RETRIES: usize = 8;
        // Order-matching variant of the locked mode: the reference grid
        // encodes the producing binary's dt dynamics but not its integration
        // order, and ngspice drops to backward Euler on the step leaving
        // every breakpoint. With this set, locked steps that start on a
        // source breakpoint time use order 1, mirroring that behavior.
        let locked_edge_order = locked_grid.is_some()
            && std::env::var("RSPICE_GRID_LOCKED_EDGE_ORDER").as_deref() == Ok("1");

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

        // Resume: replace the flat (DC-style) reactive histories written
        // above with the exact integrator histories from the checkpoint.
        if let Some(checkpoint) = resume {
            checkpoint
                .inject(&mut circuit)
                .map_err(SimulationError::Circuit)?;
        }

        let tline_dc_refs = Self::initialize_tline_history(&mut circuit, &solution, resume_time);
        let coupled_tline_refs =
            Self::initialize_coupled_tline_history(&mut circuit, &solution, resume_time);
        let mut bjt_history = Self::initialize_bjt_history(&circuit, &solution);
        let mut vbic_snapshot_cache = vec![None; circuit.bjts.devices.len()];
        // ngspice seeds CKTdeltaOld[] with maxstep before the first transient point.
        // Mirror that here so early device-local truncation/order checks have the
        // same timestep history instead of falling back to a synthetic zero-history path.
        bjt_history.accepted_dt_prev = hinted_max_step;
        bjt_history.accepted_dt_prev_prev = hinted_max_step;
        let mut jfet_history = Self::initialize_jfet_history(&circuit, &solution);
        jfet_history.accepted_dt_prev = hinted_max_step;
        jfet_history.accepted_dt_prev_prev = hinted_max_step;
        let mut diode_history = Self::initialize_diode_history(&circuit, &solution);
        diode_history.accepted_dt_prev = hinted_max_step;
        diode_history.accepted_dt_prev_prev = hinted_max_step;
        // Companion stamp slots resolved once against the frozen pattern:
        // the per-iteration charge companions then stamp through direct CSC
        // indices instead of a hash lookup per matrix entry.
        let diode_companion_slots = Self::link_diode_companion_slots(&circuit, &matrix);
        let mosfet_companion_slots = Self::link_mosfet_companion_slots(&circuit, &matrix);
        let vdmos_companion_slots = Self::link_vdmos_companion_slots(&circuit, &matrix);
        let mut mosfet_history = Self::initialize_mosfet_history(&circuit, &solution);
        mosfet_history.accepted_dt_prev = hinted_max_step;
        mosfet_history.accepted_dt_prev_prev = hinted_max_step;
        let mut vdmos_history = Self::initialize_vdmos_history(&circuit, &solution);
        vdmos_history.accepted_dt_prev = hinted_max_step;
        vdmos_history.accepted_dt_prev_prev = hinted_max_step;
        let mut b3soi_history = Self::initialize_b3soi_history(&circuit, &solution);
        b3soi_history.accepted_dt_prev = hinted_max_step;
        b3soi_history.accepted_dt_prev_prev = hinted_max_step;
        let mut bsim3_history = Self::initialize_bsim3_history(&circuit, &solution);
        bsim3_history.accepted_dt_prev = hinted_max_step;
        bsim3_history.accepted_dt_prev_prev = hinted_max_step;
        let mut bsim4_history = Self::initialize_bsim4_history(&circuit, &solution);
        bsim4_history.accepted_dt_prev = hinted_max_step;
        bsim4_history.accepted_dt_prev_prev = hinted_max_step;
        let mut ekv26_history = Self::initialize_ekv26_history(&circuit, &solution);
        ekv26_history.accepted_dt_prev = hinted_max_step;
        ekv26_history.accepted_dt_prev_prev = hinted_max_step;
        let ideal_output_pairs = circuit.ideal_voltage_output_pairs();

        // ngspice keeps `CKTgmin` live in every analysis mode: the compact
        // models' junction parallels need the configured floor during
        // transient stepping too, independent of whatever continuation
        // level a preceding DC phase last left behind.
        circuit.set_semiconductor_junction_gmin(
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target),
        );

        // Main transient loop
        let mut retry_count = 0;
        let mut total_iterations = 0;
        let mut stale_accept_count = 0;
        let mut force_accept_cooldown = 0_usize; // Failed retries to defer dt shrink immediately after force-accept
        let mut trap_order = 1_u8; // ngspice-style trap order: start at 1, promote to 2 after accepted smooth step
        const MAX_RETRIES: usize = 200; // Maximum retries per timepoint before force-accept
        const FORCE_ACCEPT_COOLDOWN_RETRIES: usize = 2;
        const LINEARIZED_STARTUP_RECOVERY_POINTS: usize = 96;
        // Keep cancellation responsiveness tight for large transient decks where a
        // single accepted step can still be expensive.
        const ABORT_CHECK_INTERVAL: usize = 16;
        const MAX_ATTEMPTS_PER_SCHEDULED_BREAKPOINT: usize = 16;
        let estimated_steps = ((tstop / max_step).ceil().max(1.0) as usize).saturating_add(1);
        let max_total_iterations = estimated_steps
            .saturating_mul(400)
            .saturating_add(
                initial_remaining_breakpoints.saturating_mul(MAX_ATTEMPTS_PER_SCHEDULED_BREAKPOINT),
            )
            .max(50_000);
        let mut last_progress_log = crate::time_compat::Instant::now();
        let mut rhs = vec![0.0; size];
        let mut new_solution = solution.clone();
        // Newton phase accounting: cumulative stamp/solve time across the
        // whole run, reported once at completion so a single info-level run
        // splits assembly cost from linear-solve cost without a profiler.
        let transient_wall_start = crate::time_compat::Instant::now();
        let mut total_stamp_nanos: u128 = 0;
        let mut total_solve_nanos: u128 = 0;
        let mut total_trunc_nanos: u128 = 0;
        let mut total_trap_trial_nanos: u128 = 0;
        let mut total_history_nanos: u128 = 0;
        let mut total_merit_nanos: u128 = 0;
        let mut total_postsolve_nanos: u128 = 0;
        let mut total_setup_nanos: u128 = 0;
        let mut total_postloop_nanos: u128 = 0;
        let mut total_top_nanos: u128 = 0;
        let mut total_tail_nanos: u128 = 0;
        let mut total_middle_nanos: u128 = 0;
        let mut total_merit_trials: usize = 0;
        let mut total_failed_attempts: usize = 0;
        // Meyer capacitance halves captured by the device-truncation walk on
        // the candidate solution; valid only for the accept path of the same
        // loop pass (reset every attempt).
        let mut mosfet_caps_scratch: Vec<(Value, Value, Value)> = Vec::new();
        let mut mosfet_caps_valid;
        let mut failed_voltage_conv: usize = 0;
        let mut failed_device_conv: usize = 0;
        let mut failed_residual_only: usize = 0;

        // Runs after every accepted point (all acceptance paths): counts the
        // floor-dt streak and performs the livelock restart when it trips.
        // A macro rather than a helper because the restart touches a dozen
        // loop locals (histories, controller, estimator, order).
        macro_rules! livelock_check {
            ($dt:expr) => {
                if locked_grid.is_none() {
                    if $dt <= livelock_dt_ceiling {
                        livelock_streak += 1;
                    } else {
                        livelock_streak = 0;
                    }
                    if livelock_streak >= LIVELOCK_STREAK_RESTART {
                        livelock_streak = 0;
                        if !livelock_restart!() {
                            return Err(SimulationError::Circuit(format!(
                                "transient timestep pinned at the minimum near t={:.6e}s \
                                 (dt={:.3e}s, delmin={:.3e}s): integration restart did not \
                                 escape; the circuit is numerically ill-conditioned at this \
                                 operating point",
                                t,
                                $dt,
                                timestep.hard_min_dt()
                            )));
                        }
                    }
                }
            };
        }

        // Perform one breakpoint-style integration restart at `t`, unless
        // the previous restart happened within the spacing window (same
        // wall — restarting again cannot help). Returns whether it ran.
        macro_rules! livelock_restart {
            () => {{
                let same_wall =
                    livelock_last_restart_t.is_some_and(|prev| t - prev < livelock_restart_spacing);
                if same_wall {
                    false
                } else {
                    livelock_last_restart_t = Some(t);
                    Self::reseed_reactive_histories_for_restart(
                        &mut circuit,
                        &solution,
                        hinted_max_step,
                        &mut bjt_history,
                        &mut jfet_history,
                        &mut diode_history,
                        &mut mosfet_history,
                        &mut vdmos_history,
                        &mut b3soi_history,
                        &mut bsim3_history,
                        &mut bsim4_history,
                        &mut ekv26_history,
                    );
                    lte_estimator =
                        LteEstimator::with_tolerances(self.voltage_reltol(), self.voltage_abstol());
                    let restart_dt = Self::ngspice_t0_breakpoint_limited_initial_timestep(
                        Self::ngspice_initial_timestep(tstop, tran_step_hint, hinted_max_step),
                        breakpoints.next_after(t),
                    );
                    timestep.force_step(restart_dt.max(timestep.preferred_min_dt()).min(max_step));
                    trap_order = 1;
                    lte_warmup_skips = 2;
                    log::warn!(
                        "Transient stall at t={:.6e}s: integration restarted \
                         (histories re-seeded, dt -> {:.3e})",
                        t,
                        timestep.dt()
                    );
                    true
                }
            }};
        }

        while t < tstop && total_iterations < max_total_iterations {
            let attempt_top_start = crate::time_compat::Instant::now();
            mosfet_caps_valid = false;
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
                last_progress_log = crate::time_compat::Instant::now();
            }

            // Abort check - check every ABORT_CHECK_INTERVAL iterations for minimal overhead
            if total_iterations % ABORT_CHECK_INTERVAL == 0 {
                if tstop > 0.0 {
                    abort.observe_progress((t / tstop).clamp(0.0, 1.0));
                }
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
            let (dt, mut at_breakpoint) = match locked_grid.as_ref() {
                Some(grid) => {
                    let Some(&target) = grid.get(locked_cursor) else {
                        break;
                    };
                    (target - t, false)
                }
                None => breakpoints.limit_step(t, timestep.dt()),
            };
            let mut dt = dt.min(tstop - t); // Don't overshoot tstop
            let mut expected_source_delta = Self::max_expected_source_delta(&circuit, t, t + dt);
            if locked_grid.is_none() {
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
                    nonlinear_source_ramp_cap_enabled,
                );
                if biased_dt + 1e-30 < dt {
                    dt = biased_dt;
                    at_breakpoint = breakpoints.at_breakpoint(t + dt);
                    expected_source_delta = Self::max_expected_source_delta(&circuit, t, t + dt);
                }
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
                step_time,
                hinted_max_step,
                retry_floor_source_activity_delta,
                initial_step,
                timestep.preferred_min_dt(),
            );
            let newton_step_delta_limit = Self::startup_step_delta_limit(
                initial_solution_mode,
                step_time,
                hinted_max_step,
                if legacy_ngspice_bjt_only_nonlinearity {
                    LEGACY_NGSPICE_BJT_NEWTON_ITER_DELTA_V
                } else {
                    MAX_NEWTON_ITER_DELTA_V
                },
            );
            let force_accept_delta_limit = Self::startup_force_accept_delta_limit(
                initial_solution_mode,
                step_time,
                hinted_max_step,
                MAX_FORCE_ACCEPT_DELTA_V,
            );
            let current_method = current_integration_method(&trapgear);
            let locked_edge_order_reset = locked_edge_order && breakpoints.at_breakpoint(t);
            let step_trap_order = Self::step_trapezoidal_order(
                current_method,
                trap_order,
                at_breakpoint || locked_edge_order_reset,
            );
            let coeff = CompanionCoefficients::for_method(Self::effective_companion_method(
                current_method,
                step_trap_order,
            ));
            let suppress_gate_charge = false;
            for dev in &circuit.b3soi.devices {
                dev.begin_timestep_iteration();
            }
            for dev in &circuit.b3soi_fd.devices {
                dev.begin_timestep_iteration();
            }
            for dev in &circuit.b3soi_pd.devices {
                dev.begin_timestep_iteration();
            }

            let linearized_startup_recovery_points = matches!(
                initial_solution_mode,
                startup::InitialSolutionMode::LinearizedSeed
            ) && result.time.len()
                <= LINEARIZED_STARTUP_RECOVERY_POINTS;
            let startup_recovery = linearized_startup_recovery_points
                || Self::in_startup_recovery_window(
                    initial_solution_mode,
                    step_time,
                    hinted_max_step,
                );
            // ngspice's transient Newton never clamps node updates globally;
            // per-junction limiting inside the device models is what tames the
            // exponential nonlinearities, and large-signal switching steps are
            // expected to converge in a handful of full Newton corrections.
            // Keep RSpice's global trust region only for startup recovery
            // (where the linearized seed is too far from a physical state for
            // raw Newton) and as a rescue once a timepoint has already burned
            // retries; otherwise it throttles legitimate switching edges into
            // timestep cuts that ngspice does not perform, skewing waveform
            // parity at every fast edge.
            let conservative_limiting_active = requires_conservative_nonlinear_limiting
                && (startup_recovery || retry_count >= CONSERVATIVE_LIMITING_RETRY_THRESHOLD);

            total_top_nanos += attempt_top_start.elapsed().as_nanos();
            let setup_phase_start = crate::time_compat::Instant::now();
            // Prepare for Newton iteration at this timestep by seeding the full
            // algebraic solution vector from accepted history when a predictor
            // state is available. ngspice's `NIpred()` predicts every solver
            // unknown, including branch-current equations, not just node
            // voltages. Matching that behavior materially improves the initial
            // Newton guess for source-heavy compact-model decks.
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
                let protected_ideal_output = i < num_nodes
                    && force_accept_protected_nodes
                        .get(i)
                        .copied()
                        .unwrap_or(false);
                let magnitude_limit = if protected_ideal_output {
                    Value::INFINITY
                } else if i < num_nodes {
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
            if conservative_limiting_active {
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
                Self::clip_ideal_output_common_modes(
                    &solution,
                    &mut new_solution,
                    newton_step_delta_limit,
                    &ideal_output_pairs,
                );
            }
            let mut nonlinear_state_matches_new_solution = false;
            let mut had_solver_candidate = false;
            // Merit-gated Newton globalization state: the true nonlinear
            // residual norm of the previously stamped iterate, the iterate
            // itself, and any backtracking search currently walking a
            // rejected step (see transient/globalization.rs).
            let mut merit_backtrack: Option<globalization::NewtonMeritBacktrack> = None;
            let mut last_stamped_iterate: Vec<Value> = Vec::new();
            let mut last_stamped_merit = Value::INFINITY;
            let mut have_stamped_iterate = false;

            // Newton-Raphson iteration for this timestep.
            // Classic SPICE transient analysis uses the transient-specific ITL4
            // budget, not the DC operating-point iteration limit.
            let tran_max_iterations = Self::transient_newton_iteration_budget(
                self.config.transient_max_iterations,
                startup_recovery,
            );
            let mut converged = false;
            // NOTE: an earlier fast path reused the previous accepted solution
            // without solving whenever its residual on the restamped system
            // passed the Newton tolerance (linear decks, quiet sources). That
            // check is scaled by row magnitudes dominated by the reactive
            // companion sources (r_eq*i_n ~ 2L/dt * i, g_eq*v_n ~ 2C/dt * v),
            // which dwarf the signal scale, so any exponential tail whose
            // per-step change fell below reltol * |companion source| was
            // frozen mid-decay (e.g. an RL step stopped decaying at ~reltol *
            // 2L/dt * i volts and held that value forever). Solution reuse is
            // only sound when the system is bit-identical, which dynamic
            // companion histories never are; linear decks already converge in
            // exactly one direct solve below, so the bypass bought one linear
            // solve per step at the cost of wrong waveforms. Removed.
            total_setup_nanos += setup_phase_start.elapsed().as_nanos();
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
                let iteration_delta_limit =
                    Self::adaptive_transient_newton_delta_limit(newton_step_delta_limit, _iter);
                let newton_stamp_start = crate::time_compat::Instant::now();
                self.stamp_transient_system(
                    &mut circuit,
                    &mut matrix,
                    &mut rhs,
                    &new_solution,
                    t + dt,
                    dt,
                    &residual::TransientSystemContext {
                        coeff: &coeff,
                        method: current_method,
                        trap_order: step_trap_order,
                        bjt_history: &bjt_history,
                        jfet_history: &jfet_history,
                        diode_history: &diode_history,
                        diode_companion_slots: &diode_companion_slots,
                        mosfet_history: &mosfet_history,
                        mosfet_companion_slots: &mosfet_companion_slots,
                        vdmos_history: &vdmos_history,
                        vdmos_companion_slots: &vdmos_companion_slots,
                        b3soi_history: &b3soi_history,
                        bsim3_history: &bsim3_history,
                        bsim4_history: &bsim4_history,
                        ekv26_history: &ekv26_history,
                        suppress_gate_charge,
                        tline_dc_refs: &tline_dc_refs,
                        coupled_tline_refs: &coupled_tline_refs,
                    },
                    &mut vbic_snapshot_cache,
                    VbicCachedSnapshotReuse::NewtonBypass,
                    !nonlinear_state_matches_new_solution,
                    0.0,
                );

                // Merit-gated Newton globalization: the freshly stamped
                // system gives the true nonlinear residual at the current
                // iterate for one matrix-vector product. Judge the previous
                // Newton step with it and damp the step when it has left its
                // basin — the saturation-boundary limit cycles this breaks
                // are unreachable by timestep reduction alone (the cycle is
                // driven by the static nonlinearity, not by stiffness).
                let merit_phase_start = crate::time_compat::Instant::now();
                if circuit.has_nonlinear_devices() {
                    let current_merit = self
                        .residual_inf_norm(&circuit, &mut matrix, &new_solution, &rhs)
                        .unwrap_or(Value::INFINITY);
                    if newton_merit_debug_enabled() {
                        log::warn!(
                            "NEWTON-MERIT t={:.6e} dt={:.3e} iter={} merit={:.6e} prev={:.6e} searching={}",
                            t,
                            dt,
                            _iter,
                            current_merit,
                            last_stamped_merit,
                            merit_backtrack.is_some(),
                        );
                    }
                    if let Some(mut search) = merit_backtrack.take() {
                        match search.judge(current_merit) {
                            globalization::BacktrackAction::Trial(trial) => {
                                new_solution = trial;
                                circuit
                                    .enforce_ideal_voltage_constraints(&mut new_solution, t + dt);
                                nonlinear_state_matches_new_solution = false;
                                merit_backtrack = Some(search);
                                total_stamp_nanos += newton_stamp_start.elapsed().as_nanos();
                                total_merit_trials += 1;
                                continue;
                            }
                            globalization::BacktrackAction::Accept => {}
                        }
                    } else if have_stamped_iterate
                        && globalization::NewtonMeritBacktrack::step_needs_globalization(
                            last_stamped_merit,
                            current_merit,
                        )
                    {
                        static MERIT_BACKTRACK_LOG_COUNT: std::sync::atomic::AtomicUsize =
                            std::sync::atomic::AtomicUsize::new(0);
                        let log_count = MERIT_BACKTRACK_LOG_COUNT
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if log_count < 20 {
                            log::debug!(
                                "Newton merit backtrack engaged at t={:.6e}, dt={:.3e}: residual {:.3e} -> {:.3e}",
                                t,
                                dt,
                                last_stamped_merit,
                                current_merit,
                            );
                        }
                        let (search, trial) = globalization::NewtonMeritBacktrack::engage(
                            &last_stamped_iterate,
                            last_stamped_merit,
                            &new_solution,
                            current_merit,
                        );
                        new_solution = trial;
                        circuit.enforce_ideal_voltage_constraints(&mut new_solution, t + dt);
                        nonlinear_state_matches_new_solution = false;
                        merit_backtrack = Some(search);
                        total_stamp_nanos += newton_stamp_start.elapsed().as_nanos();
                        total_merit_trials += 1;
                        continue;
                    }
                    last_stamped_iterate.clone_from(&new_solution);
                    last_stamped_merit = current_merit;
                    have_stamped_iterate = true;
                }
                total_merit_nanos += merit_phase_start.elapsed().as_nanos();

                // Solve and check convergence
                let newton_stamp_elapsed = newton_stamp_start.elapsed();
                total_stamp_nanos += newton_stamp_elapsed.as_nanos();
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
                let newton_solve_start = crate::time_compat::Instant::now();
                let solve_result = if prefer_dense_solver {
                    matrix.solve_dense(&rhs)
                } else {
                    matrix.solve(&rhs)
                };
                let newton_solve_elapsed = newton_solve_start.elapsed();
                total_solve_nanos += newton_solve_elapsed.as_nanos();
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

                let postsolve_phase_start = crate::time_compat::Instant::now();
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
                            let protected_ideal_output = i < num_nodes
                                && force_accept_protected_nodes
                                    .get(i)
                                    .copied()
                                    .unwrap_or(false);
                            let magnitude_limit = if protected_ideal_output {
                                Value::INFINITY
                            } else if i < num_nodes {
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

                        if conservative_limiting_active && !junction_owns_steps {
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
                        let residual_converged =
                            self.residual_convergence_met(&circuit, &mut matrix, &sol, &rhs);
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

                        let device_converged = !circuit.has_nonlinear_devices()
                            || circuit.nonlinear_converged(self.device_convergence_criteria());
                        let behavioral_converged = circuit.behavioral_linearizations_converged(
                            &new_solution,
                            t + dt,
                            self.voltage_reltol(),
                            self.voltage_abstol(),
                            self.current_abstol(),
                        );
                        total_postsolve_nanos += postsolve_phase_start.elapsed().as_nanos();

                        if voltage_converged
                            && device_converged
                            && behavioral_converged
                            && residual_converged
                        {
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

            let postloop_phase_start = crate::time_compat::Instant::now();
            if !converged {
                retry_count += 1;
                trap_order = 1;

                // Diagnostic logging for debugging convergence issues
                static CONV_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                let count = CONV_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if count < 10 || (t > 9.5e-8 && dt < 1.0e-15) {
                    // Check what specifically didn't converge
                    let v_conv =
                        self.node_voltage_convergence_met(&solution, &new_solution, num_nodes);
                    let d_conv = !circuit.has_nonlinear_devices()
                        || circuit.nonlinear_converged(self.device_convergence_criteria());
                    let r_conv =
                        self.residual_convergence_met(&circuit, &mut matrix, &new_solution, &rhs);
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

                // Gmin-continuation rescue: a knife edge in the static
                // nonlinearity repeats at every dt, so the cut cascade
                // cannot fix it. Deform the step's system with diagonal
                // shunts, converge, and track the solution back to the
                // genuine system (transient/rescue.rs). A success flows
                // into the normal LTE acceptance machinery below.
                if retry_count >= TRANSIENT_GMIN_RESCUE_MIN_RETRIES
                    && circuit.has_nonlinear_devices()
                    && let Some(rescued) = self.rescue_transient_step_with_gmin_continuation(
                        &mut circuit,
                        &mut matrix,
                        &mut rhs,
                        &solution,
                        t + dt,
                        dt,
                        &residual::TransientSystemContext {
                            coeff: &coeff,
                            method: current_method,
                            trap_order: step_trap_order,
                            bjt_history: &bjt_history,
                            jfet_history: &jfet_history,
                            diode_history: &diode_history,
                            diode_companion_slots: &diode_companion_slots,
                            mosfet_history: &mosfet_history,
                            mosfet_companion_slots: &mosfet_companion_slots,
                            vdmos_history: &vdmos_history,
                            vdmos_companion_slots: &vdmos_companion_slots,
                            b3soi_history: &b3soi_history,
                            bsim3_history: &bsim3_history,
                            bsim4_history: &bsim4_history,
                            ekv26_history: &ekv26_history,
                            suppress_gate_charge,
                            tline_dc_refs: &tline_dc_refs,
                            coupled_tline_refs: &coupled_tline_refs,
                        },
                        &mut vbic_snapshot_cache,
                    )
                {
                    static GMIN_RESCUE_LOG_COUNT: std::sync::atomic::AtomicUsize =
                        std::sync::atomic::AtomicUsize::new(0);
                    let log_count =
                        GMIN_RESCUE_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_count < 20 {
                        log::warn!(
                            "Transient gmin-continuation rescue converged at t={:.6e}, dt={:.3e} (retry {})",
                            t,
                            dt,
                            retry_count,
                        );
                    }
                    new_solution = rescued;
                    nonlinear_state_matches_new_solution = true;
                    had_solver_candidate = true;
                    converged = true;
                }
            }

            if !converged {
                total_failed_attempts += 1;
                if self.node_voltage_convergence_met(&solution, &new_solution, num_nodes) {
                    // Voltage settled but a device/residual criterion held the
                    // point back — the interesting bucket for criteria tuning.
                    if !circuit.has_nonlinear_devices()
                        || circuit.nonlinear_converged(self.device_convergence_criteria())
                    {
                        failed_residual_only += 1;
                    } else {
                        failed_device_conv += 1;
                    }
                } else {
                    failed_voltage_conv += 1;
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

                // Grid-locked steps never change dt and never force-accept:
                // the dt-preserving Newton retries above (junction limiting,
                // gmin rescue) are the whole recovery budget, and exhausting
                // it fails the run with the offending grid time — committing
                // a non-converged point would poison the locked trajectory.
                if locked_grid.is_some() {
                    if retry_count >= LOCKED_MAX_RETRIES {
                        log::error!(
                            "Grid-locked step to t={:.12e}s (dt={:.3e}) failed Newton after {} retries",
                            t + dt,
                            dt,
                            retry_count
                        );
                        return Err(SimulationError::ConvergenceFailed(total_iterations));
                    }
                    total_postloop_nanos += postloop_phase_start.elapsed().as_nanos();
                    continue;
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
                let exhausted_at_min = at_min_dt && retry_count >= MIN_RETRIES_AT_MINIMUM_TIMESTEP;

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
                        &force_accept_protected_nodes,
                    );
                    let use_static_source_recovery_guards = !circuit
                        .has_xspice_event_driven_devices()
                        && voltage_lte_excluded_nodes.is_empty();
                    let excessive_quiet_force_candidate = use_static_source_recovery_guards
                        && Self::is_excessive_quiet_force_candidate(
                            &solution,
                            &bounded_force_candidate,
                            expected_source_delta,
                            num_nodes,
                            force_accept_delta_limit,
                        );
                    let stale_force_candidate = use_static_source_recovery_guards
                        && Self::is_stale_step(
                            &solution,
                            &bounded_force_candidate,
                            expected_source_delta,
                            num_nodes,
                            &circuit.inductors.branch_indices,
                        );
                    let stagnant_force_candidate = use_static_source_recovery_guards
                        && Self::is_stagnant_force_candidate(
                            &circuit,
                            &solution,
                            &bounded_force_candidate,
                            num_nodes,
                            self.voltage_abstol(),
                            self.current_abstol(),
                        );

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
                            // The same dead-zone walls that defeat the
                            // force-accept envelope also poison the
                            // truncation history; one clean restart at a
                            // fresh ramp dt escapes them. Only when the
                            // restart already ran at this wall is the run
                            // genuinely dead.
                            if livelock_restart!() {
                                stale_accept_count = 0;
                                retry_count = 0;
                                continue;
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
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_jfet_truncation_limit = if !circuit.jfets.is_empty() {
                        Self::jfet_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &jfet_history,
                            suppress_gate_charge,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
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
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_diode_truncation_limit = if !circuit.diodes.is_empty() {
                        Self::diode_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &diode_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
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
                                self.transient_trtol(),
                                None,
                            )
                            .filter(|limit| limit.is_finite() && *limit > 0.0)
                        } else {
                            None
                        };
                    let force_accept_vdmos_truncation_limit = if !circuit.vdmoses.is_empty() {
                        Self::vdmos_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &vdmos_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_b3soi_truncation_limit = if circuit.has_b3soi_devices() {
                        Self::b3soi_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &b3soi_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_bsim3_truncation_limit = if circuit.has_bsim3v3_devices() {
                        Self::bsim3_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &bsim3_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_bsim4_truncation_limit = if circuit.has_bsim4v8_devices() {
                        Self::bsim4_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &bsim4_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_device_truncation_limit = Self::min_truncation_limit(
                        Self::min_truncation_limit(
                            Self::min_truncation_limit(
                                Self::min_truncation_limit(
                                    Self::min_truncation_limit(
                                        Self::min_truncation_limit(
                                            Self::min_truncation_limit(
                                                force_accept_capacitor_truncation_limit,
                                                force_accept_bjt_truncation_limit,
                                            ),
                                            force_accept_jfet_truncation_limit,
                                        ),
                                        force_accept_diode_truncation_limit,
                                    ),
                                    force_accept_mosfet_truncation_limit,
                                ),
                                force_accept_vdmos_truncation_limit,
                            ),
                            force_accept_b3soi_truncation_limit,
                        ),
                        Self::min_truncation_limit(
                            force_accept_bsim3_truncation_limit,
                            force_accept_bsim4_truncation_limit,
                        ),
                    );
                    lte_estimator.record(&new_solution, dt);
                    lte_estimator.set_method_order(effective_method_order(
                        method_after_step,
                        accepted_step_trap_order,
                    ));
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
                        &mut diode_history,
                        &mut mosfet_history,
                        &mut vdmos_history,
                        &mut b3soi_history,
                        &mut bsim3_history,
                        &mut bsim4_history,
                        &mut ekv26_history,
                        Some(vbic_snapshot_cache.as_slice()),
                        None,
                        suppress_gate_charge,
                        &tline_dc_refs,
                        &coupled_tline_refs,
                        &mut breakpoints,
                        tstop,
                        self.voltage_reltol(),
                        self.voltage_abstol(),
                        self.current_abstol(),
                        &mut dynamic_tline_breakpoints_added,
                        &mut warned_dynamic_tline_breakpoint_cap,
                    );
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_transient_timestep(t, dt, &new_solution);
                        Self::collect_xspice_runtime_breakpoints(
                            &mut circuit,
                            &mut breakpoints,
                            tstop,
                        );
                    }
                    #[cfg(feature = "veriloga")]
                    if circuit.has_veriloga_devices() {
                        circuit.accept_veriloga_timestep();
                    }
                    #[cfg(feature = "veriloga-builtins")]
                    if circuit.has_generated_veriloga_devices() {
                        circuit.accept_generated_veriloga_timestep();
                    }

                    solution.clone_from(&new_solution);
                    if std::env::var_os("RSPICE_GRID_DEBUG").is_some() {
                        log::warn!("GRID force-accept t={:.12e} dt={:.6e}", t, dt);
                    }
                    Self::record_transient_solution_sample(
                        &mut result,
                        &mut circuit,
                        &solution,
                        num_nodes,
                        t,
                        &derived_branch_currents,
                    );
                    circuit.fill_xspice_digital_snapshot(&mut digital_snapshot);
                    result.record_digital_snapshot(
                        t,
                        &digital_snapshot,
                        &mut digital_trace_indices,
                    );

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
                    force_accept_cooldown = FORCE_ACCEPT_COOLDOWN_RETRIES;
                    timestep.force_step(next_force_dt);
                    if matches!(
                        current_method,
                        IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                    ) {
                        trap_order = 1;
                    }
                    livelock_check!(dt);
                }
                total_postloop_nanos += postloop_phase_start.elapsed().as_nanos();
                continue;
            }

            total_postloop_nanos += postloop_phase_start.elapsed().as_nanos();
            let truncation_phase_start = crate::time_compat::Instant::now();
            let first_accepted_transient_step =
                Self::should_skip_post_accept_timestep_control_on_first_step(result.time.len())
                    // Post-livelock-restart warmup: the re-seeded histories
                    // need two clean accepted points before the truncation
                    // estimators can difference them meaningfully.
                    || lte_warmup_skips > 0;
            let bjt_truncation_limit = if !linearized_startup_recovery_points
                && !first_accepted_transient_step
                && has_bjts
            {
                Self::bjt_ngspice_truncation_limit(
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
                    self.transient_trtol(),
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
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
                        self.transient_trtol(),
                    )
                    .filter(|limit| limit.is_finite() && *limit > 0.0)
                } else {
                    None
                };
            let jfet_truncation_limit =
                if !first_accepted_transient_step && !circuit.jfets.is_empty() {
                    Self::jfet_ngspice_truncation_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        &jfet_history,
                        suppress_gate_charge,
                        self.voltage_reltol(),
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                    )
                    .filter(|limit| limit.is_finite() && *limit > 0.0)
                } else {
                    None
                };
            let diode_truncation_limit =
                if !first_accepted_transient_step && !circuit.diodes.is_empty() {
                    Self::diode_ngspice_truncation_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        &diode_history,
                        self.voltage_reltol(),
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                    )
                    .filter(|limit| limit.is_finite() && *limit > 0.0)
                } else {
                    None
                };
            let mosfet_truncation_limit = if !first_accepted_transient_step
                && !suppress_gate_charge
                && !circuit.mosfets.is_empty()
            {
                let limit = Self::mosfet_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &mosfet_history,
                    self.voltage_reltol(),
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                    Some(&mut mosfet_caps_scratch),
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0);
                mosfet_caps_valid = mosfet_caps_scratch.len() == circuit.mosfets.devices.len();
                limit
            } else {
                None
            };
            let vdmos_truncation_limit =
                if !first_accepted_transient_step && !circuit.vdmoses.is_empty() {
                    Self::vdmos_ngspice_truncation_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        &vdmos_history,
                        self.voltage_reltol(),
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                    )
                    .filter(|limit| limit.is_finite() && *limit > 0.0)
                } else {
                    None
                };
            let b3soi_truncation_limit =
                if !first_accepted_transient_step && circuit.has_b3soi_devices() {
                    Self::b3soi_ngspice_truncation_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        &b3soi_history,
                        self.voltage_reltol(),
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                    )
                    .filter(|limit| limit.is_finite() && *limit > 0.0)
                } else {
                    None
                };
            let bsim3_truncation_limit =
                if !first_accepted_transient_step && circuit.has_bsim3v3_devices() {
                    Self::bsim3_ngspice_truncation_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        &bsim3_history,
                        self.voltage_reltol(),
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                    )
                    .filter(|limit| limit.is_finite() && *limit > 0.0)
                } else {
                    None
                };
            let bsim4_truncation_limit =
                if !first_accepted_transient_step && circuit.has_bsim4v8_devices() {
                    Self::bsim4_ngspice_truncation_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        &bsim4_history,
                        self.voltage_reltol(),
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                    )
                    .filter(|limit| limit.is_finite() && *limit > 0.0)
                } else {
                    None
                };
            let device_truncation_limit = Self::min_truncation_limit(
                Self::min_truncation_limit(
                    Self::min_truncation_limit(
                        Self::min_truncation_limit(
                            Self::min_truncation_limit(
                                Self::min_truncation_limit(
                                    Self::min_truncation_limit(
                                        capacitor_truncation_limit,
                                        bjt_truncation_limit,
                                    ),
                                    jfet_truncation_limit,
                                ),
                                diode_truncation_limit,
                            ),
                            mosfet_truncation_limit,
                        ),
                        vdmos_truncation_limit,
                    ),
                    b3soi_truncation_limit,
                ),
                Self::min_truncation_limit(bsim3_truncation_limit, bsim4_truncation_limit),
            );
            let ltra_truncation_limit = if !first_accepted_transient_step {
                Self::ltra_candidate_truncation_limit(&circuit, &new_solution, t + dt)
            } else {
                None
            };
            let activity_limit = if !first_accepted_transient_step {
                Self::nonlinear_terminal_activity_limit(
                    &circuit,
                    &solution,
                    &new_solution,
                    dt,
                    self.config.transient_node_activity_bound,
                )
            } else {
                None
            };
            let candidate_truncation_limit = Self::min_truncation_limit(
                Self::min_truncation_limit(device_truncation_limit, ltra_truncation_limit),
                activity_limit,
            );
            total_trunc_nanos += truncation_phase_start.elapsed().as_nanos();
            let middle_phase_start = crate::time_compat::Instant::now();

            if locked_grid.is_none()
                && let Some(limit) = candidate_truncation_limit
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
                            "Candidate truncation reached minimum retry step at t={:.6e}, dt={:.3e}, limit={:.3e}, retry_count={}; accepting converged solution",
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
                    // Routine LTE step-rejection diagnostics, not a user-facing
                    // condition: keep them at debug so healthy runs have a
                    // clean stderr at the default log level.
                    if log_count < 40 || (t > 9.5e-8 && dt < 1.0e-15) {
                        log::debug!(
                            "Candidate truncation reject at t={:.6e}, dt={:.3e}, limit={:.3e}, cap={:?}, bjt={:?}, jfet={:?}, dio={:?}, mos={:?}, vdmos={:?}, ltra={:?}, method={:?}, order={}",
                            t,
                            dt,
                            limit,
                            capacitor_truncation_limit,
                            bjt_truncation_limit,
                            jfet_truncation_limit,
                            diode_truncation_limit,
                            mosfet_truncation_limit,
                            vdmos_truncation_limit,
                            ltra_truncation_limit,
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
                    total_middle_nanos += middle_phase_start.elapsed().as_nanos();
                    continue;
                }
            }

            // Check LTE for physics accuracy
            let defer_voltage_lte_to_bjt_truncation =
                Self::bjt_charge_truncation_covers_transient_lte(&circuit, bjt_truncation_limit);
            let defer_voltage_lte_to_jfet_truncation = !has_bjts
                && Self::jfet_charge_truncation_covers_transient_lte(
                    &circuit,
                    jfet_truncation_limit,
                );
            let defer_voltage_lte_to_mosfet_truncation = !has_bjts
                && circuit.jfets.is_empty()
                && Self::mosfet_charge_truncation_covers_transient_lte(
                    &circuit,
                    mosfet_truncation_limit,
                );
            let defer_voltage_lte_to_ngspice_device_truncation =
                Self::ngspice_device_truncation_covers_transient_lte(
                    &circuit,
                    capacitor_truncation_limit,
                    bjt_truncation_limit,
                    jfet_truncation_limit,
                    diode_truncation_limit,
                    mosfet_truncation_limit,
                    vdmos_truncation_limit,
                );
            let device_or_startup_controls_lte = first_accepted_transient_step
                || linearized_startup_recovery_points
                || defer_voltage_lte_to_bjt_truncation
                || defer_voltage_lte_to_jfet_truncation
                || defer_voltage_lte_to_mosfet_truncation
                || defer_voltage_lte_to_ngspice_device_truncation;
            let (lte, accept) = if locked_grid.is_some() || device_or_startup_controls_lte {
                // For locked grids, first/startup recovery points, and decks
                // covered by ngspice device-local truncation (CAPtrunc,
                // MOStrunc, BJTtrunc, generated compact-model truncation,
                // etc.), a converged Newton
                // solution at the imposed dt is the acceptance criterion.
                (0.0, true)
            } else {
                Self::estimate_transient_lte(
                    &circuit,
                    &new_solution,
                    dt,
                    is_strictly_linear_transient,
                    &lte_estimator,
                    &voltage_lte_excluded_nodes,
                )
            };
            let lte_scale = if first_accepted_transient_step || is_strictly_linear_transient {
                1.0
            } else {
                lte_estimator.recommend_scale(lte)
            };
            if !accept {
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
                let exhausted_at_min = at_min_dt && retry_count >= MIN_RETRIES_AT_MINIMUM_TIMESTEP;

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
                        &force_accept_protected_nodes,
                    );
                    let use_static_source_recovery_guards = !circuit
                        .has_xspice_event_driven_devices()
                        && voltage_lte_excluded_nodes.is_empty();
                    let excessive_quiet_force_candidate = use_static_source_recovery_guards
                        && Self::is_excessive_quiet_force_candidate(
                            &solution,
                            &bounded_force_candidate,
                            expected_source_delta,
                            num_nodes,
                            force_accept_delta_limit,
                        );
                    let stale_force_candidate = use_static_source_recovery_guards
                        && Self::is_stale_step(
                            &solution,
                            &bounded_force_candidate,
                            expected_source_delta,
                            num_nodes,
                            &circuit.inductors.branch_indices,
                        );
                    let stagnant_force_candidate = use_static_source_recovery_guards
                        && Self::is_stagnant_force_candidate(
                            &circuit,
                            &solution,
                            &bounded_force_candidate,
                            num_nodes,
                            self.voltage_abstol(),
                            self.current_abstol(),
                        );

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
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_jfet_truncation_limit = if !circuit.jfets.is_empty() {
                        Self::jfet_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &jfet_history,
                            suppress_gate_charge,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
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
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_diode_truncation_limit = if !circuit.diodes.is_empty() {
                        Self::diode_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &diode_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
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
                                self.transient_trtol(),
                                None,
                            )
                            .filter(|limit| limit.is_finite() && *limit > 0.0)
                        } else {
                            None
                        };
                    let force_accept_vdmos_truncation_limit = if !circuit.vdmoses.is_empty() {
                        Self::vdmos_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &vdmos_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_b3soi_truncation_limit = if circuit.has_b3soi_devices() {
                        Self::b3soi_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &b3soi_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_bsim3_truncation_limit = if circuit.has_bsim3v3_devices() {
                        Self::bsim3_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &bsim3_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_bsim4_truncation_limit = if circuit.has_bsim4v8_devices() {
                        Self::bsim4_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            &bsim4_history,
                            self.voltage_reltol(),
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_device_truncation_limit = Self::min_truncation_limit(
                        Self::min_truncation_limit(
                            Self::min_truncation_limit(
                                Self::min_truncation_limit(
                                    Self::min_truncation_limit(
                                        Self::min_truncation_limit(
                                            Self::min_truncation_limit(
                                                force_accept_capacitor_truncation_limit,
                                                force_accept_bjt_truncation_limit,
                                            ),
                                            force_accept_jfet_truncation_limit,
                                        ),
                                        force_accept_diode_truncation_limit,
                                    ),
                                    force_accept_mosfet_truncation_limit,
                                ),
                                force_accept_vdmos_truncation_limit,
                            ),
                            force_accept_b3soi_truncation_limit,
                        ),
                        Self::min_truncation_limit(
                            force_accept_bsim3_truncation_limit,
                            force_accept_bsim4_truncation_limit,
                        ),
                    );
                    lte_estimator.record(&new_solution, dt);
                    lte_estimator.set_method_order(effective_method_order(
                        method_after_step,
                        accepted_step_trap_order,
                    ));
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
                        &mut diode_history,
                        &mut mosfet_history,
                        &mut vdmos_history,
                        &mut b3soi_history,
                        &mut bsim3_history,
                        &mut bsim4_history,
                        &mut ekv26_history,
                        Some(vbic_snapshot_cache.as_slice()),
                        None,
                        suppress_gate_charge,
                        &tline_dc_refs,
                        &coupled_tline_refs,
                        &mut breakpoints,
                        tstop,
                        self.voltage_reltol(),
                        self.voltage_abstol(),
                        self.current_abstol(),
                        &mut dynamic_tline_breakpoints_added,
                        &mut warned_dynamic_tline_breakpoint_cap,
                    );
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_transient_timestep(t, dt, &new_solution);
                        Self::collect_xspice_runtime_breakpoints(
                            &mut circuit,
                            &mut breakpoints,
                            tstop,
                        );
                    }
                    #[cfg(feature = "veriloga")]
                    if circuit.has_veriloga_devices() {
                        circuit.accept_veriloga_timestep();
                    }
                    #[cfg(feature = "veriloga-builtins")]
                    if circuit.has_generated_veriloga_devices() {
                        circuit.accept_generated_veriloga_timestep();
                    }

                    solution.clone_from(&new_solution);
                    if std::env::var_os("RSPICE_GRID_DEBUG").is_some() {
                        log::warn!("GRID force-accept t={:.12e} dt={:.6e}", t, dt);
                    }
                    Self::record_transient_solution_sample(
                        &mut result,
                        &mut circuit,
                        &solution,
                        num_nodes,
                        t,
                        &derived_branch_currents,
                    );
                    circuit.fill_xspice_digital_snapshot(&mut digital_snapshot);
                    result.record_digital_snapshot(
                        t,
                        &digital_snapshot,
                        &mut digital_trace_indices,
                    );
                    let next_force_dt = Self::force_accept_recovery_timestep(
                        dt,
                        timestep.preferred_min_dt(),
                        max_step,
                        force_accept_device_truncation_limit,
                    );
                    if std::env::var_os("RSPICE_GRID_DEBUG").is_some() && t > 9.5e-8 && dt < 1.0e-15
                    {
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
                    force_accept_cooldown = FORCE_ACCEPT_COOLDOWN_RETRIES;
                    timestep.force_step(next_force_dt);
                    if matches!(
                        current_method,
                        IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                    ) {
                        trap_order = 1;
                    }
                    livelock_check!(dt);
                }
                total_middle_nanos += middle_phase_start.elapsed().as_nanos();
                continue;
            }

            // Success - reset retry counter
            retry_count = 0;

            // Keep ideal source constraints exact before LTE and state updates.
            let projected_voltage_sources = circuit
                .voltage_sources
                .enforce_voltage_constraints(&mut new_solution, t + dt);
            if projected_voltage_sources {
                nonlinear_state_matches_new_solution = false;
            }

            if locked_grid.is_none()
                && !circuit.has_xspice_event_driven_devices()
                && Self::is_stale_step(
                    &solution,
                    &new_solution,
                    expected_source_delta,
                    num_nodes,
                    &circuit.inductors.branch_indices,
                )
            {
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
                total_middle_nanos += middle_phase_start.elapsed().as_nanos();
                continue;
            }
            stale_accept_count = 0;

            // Accept this timestep
            t += dt;
            let hit_breakpoint = if let Some(grid) = locked_grid.as_ref() {
                // Land exactly on the grid point (no floating-point drift)
                // and never engage breakpoint-restart dynamics.
                t = grid[locked_cursor];
                locked_cursor += 1;
                false
            } else {
                at_breakpoint || breakpoints.at_breakpoint(t)
            };
            if hit_breakpoint {
                t = breakpoints.snap_to_breakpoint(t);
            }
            let method_after_step = current_integration_method(&trapgear);
            lte_estimator.record(&new_solution, dt);
            lte_estimator
                .set_method_order(effective_method_order(method_after_step, step_trap_order));
            if fixed_method.is_none() {
                trapgear.update(&new_solution, dt);
            }

            if circuit.has_nonlinear_devices() && !nonlinear_state_matches_new_solution {
                circuit.update_nonlinear(&new_solution);
            }

            total_middle_nanos += middle_phase_start.elapsed().as_nanos();
            let trap_trial_phase_start = crate::time_compat::Instant::now();
            let trapezoidal_order_trial = if !first_accepted_transient_step
                && !linearized_startup_recovery_points
                && matches!(
                    current_method,
                    IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                )
                && !hit_breakpoint
            {
                if step_trap_order == 2 && device_truncation_limit.is_some() {
                    // The order-2 trial truncation walk is the order-2 device
                    // truncation walk: when this step already ran at order 2,
                    // the candidate limits were just computed above on the
                    // same solution — re-walking every device would derive
                    // the identical numbers.
                    device_truncation_limit.map(|limit| TrapezoidalOrderTrial {
                        limit,
                        promote: Self::should_promote_ngspice_charge_truncation(limit, dt),
                    })
                } else {
                    Self::trapezoidal_order_trial_timestep_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        dt,
                        is_strictly_linear_transient,
                        &bjt_history,
                        &jfet_history,
                        &diode_history,
                        &mosfet_history,
                        &vdmos_history,
                        &ekv26_history,
                        &lte_estimator,
                        &voltage_lte_excluded_nodes,
                        &vbic_snapshot_cache,
                        self.voltage_abstol(),
                        self.voltage_reltol(),
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                    )
                }
            } else {
                None
            };
            total_trap_trial_nanos += trap_trial_phase_start.elapsed().as_nanos();

            let history_phase_start = crate::time_compat::Instant::now();
            Self::update_reactive_history(
                &mut circuit,
                &new_solution,
                t,
                dt,
                current_method,
                step_trap_order,
                &mut bjt_history,
                &mut jfet_history,
                &mut diode_history,
                &mut mosfet_history,
                &mut vdmos_history,
                &mut b3soi_history,
                &mut bsim3_history,
                &mut bsim4_history,
                &mut ekv26_history,
                Some(vbic_snapshot_cache.as_slice()),
                mosfet_caps_valid.then_some(mosfet_caps_scratch.as_slice()),
                suppress_gate_charge,
                &tline_dc_refs,
                &coupled_tline_refs,
                &mut breakpoints,
                tstop,
                self.voltage_reltol(),
                self.voltage_abstol(),
                self.current_abstol(),
                &mut dynamic_tline_breakpoints_added,
                &mut warned_dynamic_tline_breakpoint_cap,
            );
            total_history_nanos += history_phase_start.elapsed().as_nanos();
            let tail_phase_start = crate::time_compat::Instant::now();
            // Accept XSPICE timestep (commit state changes)
            if circuit.has_xspice_devices() {
                circuit.accept_xspice_transient_timestep(t, dt, &new_solution);
                Self::collect_xspice_runtime_breakpoints(&mut circuit, &mut breakpoints, tstop);
            }
            #[cfg(feature = "veriloga")]
            let veriloga_discontinuity = if circuit.has_veriloga_devices() {
                circuit.accept_veriloga_timestep()
            } else {
                false
            };
            #[cfg(feature = "veriloga-builtins")]
            if circuit.has_generated_veriloga_devices() {
                circuit.accept_generated_veriloga_timestep();
            }

            solution.clone_from(&new_solution);

            if std::env::var_os("RSPICE_GRID_DEBUG").is_some() {
                log::warn!(
                    "GRID accept t={:.12e} dt={:.6e} order={} bp={}",
                    t,
                    dt,
                    step_trap_order,
                    hit_breakpoint
                );
            }

            // Store results
            Self::record_transient_solution_sample(
                &mut result,
                &mut circuit,
                &solution,
                num_nodes,
                t,
                &derived_branch_currents,
            );
            circuit.fill_xspice_digital_snapshot(&mut digital_snapshot);
            result.record_digital_snapshot(t, &digital_snapshot, &mut digital_trace_indices);
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
                if !circuit.vdmoses.is_empty() {
                    lte_warmup_skips = lte_warmup_skips.max(2);
                }
            }
            if !first_accepted_transient_step
                && let Some(limit) = candidate_truncation_limit
                && limit.is_finite()
                && limit > 0.0
                && limit + 1e-18 < timestep.dt()
            {
                if t > 9.5e-8 && dt < 1.0e-15 {
                    log::warn!(
                        "Candidate post-accept timestep cap at t={:.12e}, accepted_dt={:.3e}, requested_next={:.3e}, limit={:.3e}, order={}",
                        t,
                        dt,
                        timestep.dt(),
                        limit,
                        step_trap_order
                    );
                }
                timestep.force_step(limit);
            }
            // Verilog-A timestep control: $bound_step caps the next step;
            // a newly raised $discontinuity restarts fine like a
            // breakpoint so the corner resolves sharply
            #[cfg(feature = "veriloga")]
            if circuit.has_veriloga_devices() {
                if let Some(bound) = circuit.veriloga_timestep_bound()
                    && bound + 1e-18 < timestep.dt()
                {
                    timestep.force_step(bound.min(max_step));
                }
                if veriloga_discontinuity {
                    let restart = (dt * 0.1).max(1e-15);
                    if restart < timestep.dt() {
                        timestep.force_step(restart.min(max_step));
                    }
                }
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

            lte_warmup_skips = lte_warmup_skips.saturating_sub(1);
            livelock_check!(dt);
            total_tail_nanos += tail_phase_start.elapsed().as_nanos();
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
        if let Some(message) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE evaluation failed: {message}"
            )));
        }
        let transient_wall = transient_wall_start.elapsed();
        log::info!(
            "Transient Newton phases: {} iterations, {} merit trials, {} failed attempts (v={} d={} r={}), top {:.3}s, setup {:.3}s, stamp {:.3}s, solve {:.3}s, merit {:.3}s, postsolve {:.3}s, postloop {:.3}s, trunc {:.3}s, trap-trial {:.3}s, history {:.3}s, tail {:.3}s, middle {:.3}s, other {:.3}s (wall {:.3}s)",
            total_iterations,
            total_merit_trials,
            total_failed_attempts,
            failed_voltage_conv,
            failed_device_conv,
            failed_residual_only,
            total_top_nanos as f64 * 1e-9,
            total_setup_nanos as f64 * 1e-9,
            total_stamp_nanos as f64 * 1e-9,
            total_solve_nanos as f64 * 1e-9,
            total_merit_nanos as f64 * 1e-9,
            total_postsolve_nanos as f64 * 1e-9,
            total_postloop_nanos as f64 * 1e-9,
            total_trunc_nanos as f64 * 1e-9,
            total_trap_trial_nanos as f64 * 1e-9,
            total_history_nanos as f64 * 1e-9,
            total_tail_nanos as f64 * 1e-9,
            total_middle_nanos as f64 * 1e-9,
            (transient_wall.as_nanos().saturating_sub(
                total_top_nanos
                    + total_setup_nanos
                    + total_stamp_nanos
                    + total_solve_nanos
                    + total_merit_nanos
                    + total_postsolve_nanos
                    + total_postloop_nanos
                    + total_trunc_nanos
                    + total_trap_trial_nanos
                    + total_history_nanos
                    + total_tail_nanos
                    + total_middle_nanos
            )) as f64
                * 1e-9,
            transient_wall.as_secs_f64(),
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

        let final_checkpoint = TransientCheckpoint::capture(fingerprint, t, &solution, &circuit);
        Ok((result, final_checkpoint))
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
        self.run_tran_compressed_with_abort(netlist, tstop, max_step, compression, &NoAbort)
    }

    /// Run compressed transient analysis with abort signal for cancellation.
    pub fn run_tran_compressed_with_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        compression: CompressionConfig,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResultCompressed, SimulationError> {
        // Reuse the robust transient solver path, then apply waveform compression
        // during result marshaling. This keeps compressed and uncompressed physics
        // behavior identical, avoiding divergence between solver implementations.
        let result = self.run_tran_with_abort(netlist, tstop, max_step, abort)?;

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
        )
        .map_err(SimulationError::Circuit)?;

        for point_idx in 1..result.time.len() {
            let values: Vec<Value> = result
                .voltages
                .iter()
                .map(|wave| wave.get(point_idx).copied().unwrap_or(0.0))
                .collect();
            recorder
                .record(result.time[point_idx], &values)
                .map_err(SimulationError::Circuit)?;
        }

        let final_values: Vec<Value> = result
            .voltages
            .iter()
            .map(|wave| wave.last().copied().unwrap_or(0.0))
            .collect();
        recorder
            .finalize(*result.time.last().unwrap_or(&tstop), &final_values)
            .map_err(SimulationError::Circuit)?;

        let mut compressed = recorder.to_transient_result();
        compressed.node_names = result.node_names.clone();
        Ok(compressed)
    }
}

fn validate_transient_window(tstop: Value, max_step: Value) -> Result<(), SimulationError> {
    if !tstop.is_finite() || tstop <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Transient stop time must be a positive finite number of seconds, got {tstop}"
        )));
    }
    if !max_step.is_finite() || max_step <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Transient max_step must be a positive finite number of seconds, got {max_step}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_step_guard_counts_inductor_branch_current_motion() {
        let previous = [1.0, 2.0, 0.0];
        let node_stale_branch_active = [1.0, 2.0, 0.1];

        assert!(!Engine::is_stale_step(
            &previous,
            &node_stale_branch_active,
            SOURCE_ACTIVE_DELTA * 10.0,
            2,
            &[1],
        ));
        assert!(Engine::is_stale_step(
            &previous,
            &node_stale_branch_active,
            SOURCE_ACTIVE_DELTA * 10.0,
            2,
            &[],
        ));
    }

    fn missing_pwl_path(name: &str) -> String {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        std::env::temp_dir()
            .join(format!(
                "rspice-missing-{name}-{}-{unique}.csv",
                std::process::id()
            ))
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn scalar_line(name: &str) -> crate::device::TransmissionLine {
        crate::device::TransmissionLine::new(name.to_string(), 1, 0, 2, 0, 50.0, 1.0e-9)
    }

    #[test]
    fn transmission_source_ramp_cap_is_disabled_for_native_txl_and_ltra() {
        let circuit = crate::circuit::Circuit::new();
        assert!(!Engine::should_enable_nonlinear_source_ramp_cap(
            &circuit, false
        ));
        assert!(Engine::should_enable_nonlinear_source_ramp_cap(
            &circuit, true
        ));

        let mut lossless_circuit = crate::circuit::Circuit::new();
        lossless_circuit.tlines.push(scalar_line("TLOSSLESS"));
        assert!(Engine::should_enable_nonlinear_source_ramp_cap(
            &lossless_circuit,
            true
        ));

        let mut ltra_circuit = crate::circuit::Circuit::new();
        let mut ltra_line = scalar_line("TLTRA");
        ltra_line.set_distributed_rlgc(0.25, 4.0, 0.0, 1.0, 1.0);
        ltra_circuit.tlines.push(ltra_line);
        assert!(!Engine::should_enable_nonlinear_source_ramp_cap(
            &ltra_circuit,
            true
        ));

        let mut txl_circuit = crate::circuit::Circuit::new();
        let mut txl_line = scalar_line("TTXL");
        assert!(txl_line.enable_txl_runtime(12.45, 8.972e-9, 0.0, 0.468e-12, 16.0));
        txl_circuit.tlines.push(txl_line);
        assert!(!Engine::should_enable_nonlinear_source_ramp_cap(
            &txl_circuit,
            true
        ));
    }

    #[test]
    fn transient_newton_iteration_budget_uses_ngspice_floor() {
        // ngspice NIiter floors every Newton call to 100 iterations.
        assert_eq!(
            Engine::transient_newton_iteration_budget(10, false),
            NGSPICE_NIITER_MIN_ITERATIONS
        );
        assert_eq!(Engine::transient_newton_iteration_budget(250, false), 250);
        assert_eq!(Engine::transient_newton_iteration_budget(250, true), 128);
    }

    #[test]
    fn transient_rejects_invalid_time_window() {
        let deck = "RC step\n\
                    V1 1 0 DC 1\n\
                    R1 1 2 1k\n\
                    C1 2 0 1u\n\
                    .end\n";
        let netlist = crate::Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default();

        for stop in [0.0, -1.0e-6, f64::NAN] {
            let err = engine
                .run_tran(&netlist, stop, 1.0e-6)
                .expect_err("invalid transient stop time must raise");
            assert!(
                err.to_string().contains("positive finite"),
                "unexpected error for stop={stop:?}: {err}"
            );
        }

        for max_step in [0.0, -1.0e-6, f64::INFINITY] {
            let err = engine
                .run_tran(&netlist, 1.0e-6, max_step)
                .expect_err("invalid transient max_step must raise");
            assert!(
                err.to_string().contains("positive finite"),
                "unexpected error for max_step={max_step:?}: {err}"
            );
        }
    }

    #[test]
    fn transient_rejects_missing_pwl_file_source() {
        let path = missing_pwl_path("tran");
        let deck = format!(
            "missing PWL file\n\
             V1 in 0 PWL FILE=\"{path}\"\n\
             R1 in 0 1k\n\
             .tran 1n 10n\n\
             .end\n"
        );
        let netlist = crate::Netlist::parse(&deck).expect("deck parses");
        let err = Engine::default()
            .run_tran(&netlist, 10.0e-9, 1.0e-9)
            .expect_err("missing PWL file must fail before transient solve");

        assert!(
            err.to_string().contains("failed to load PWL file"),
            "unexpected error: {err}"
        );
        assert!(err.to_string().contains(&path));
    }

    #[test]
    fn transient_behavioral_table_source_tracks_time() {
        let deck = "Behavioral TABLE(time) source\n\
                    B1 1 0 V={TABLE(time, 0, 1, 1n, 2)}\n\
                    R1 1 0 1k\n\
                    .end\n";
        let netlist = crate::Netlist::parse(deck).expect("deck parses");
        let result = Engine::default()
            .run_tran(&netlist, 1.0e-9, 1.0e-9)
            .expect("transient runs");

        let final_v = result.voltages[0]
            .last()
            .copied()
            .expect("node 1 has samples");
        assert!(
            (final_v - 2.0).abs() <= 1.0e-9,
            "behavioral TABLE source held {final_v:.12e} instead of tracking time"
        );
    }

    /// An explicitly configured `max_timestep` must cap the accepted step
    /// even when the caller passes a coarser per-run maximum (the CLI
    /// --max-step path resolves into the config, not the argument).
    #[test]
    fn configured_max_timestep_caps_accepted_steps() {
        let deck = "RC step\n\
                    V1 1 0 DC 1\n\
                    R1 1 2 1k\n\
                    C1 2 0 1u\n\
                    .end\n";
        let netlist = crate::Netlist::parse(deck).expect("deck parses");

        let config = crate::SimulationConfig {
            max_timestep: 2.0e-6,
            ..Default::default()
        };
        let engine = Engine::new(config);
        let result = engine
            .run_tran(&netlist, 100.0e-6, 20.0e-6)
            .expect("transient runs");

        let mut worst_dt: Value = 0.0;
        for pair in result.time.windows(2) {
            worst_dt = worst_dt.max(pair[1] - pair[0]);
        }
        assert!(
            worst_dt <= 2.0e-6 + 1e-12,
            "configured max_timestep ignored: worst accepted dt {worst_dt:.3e}"
        );

        // Default config: the caller-provided maximum governs unchanged.
        let default_engine = Engine::new(crate::SimulationConfig::default());
        let free = default_engine
            .run_tran(&netlist, 100.0e-6, 20.0e-6)
            .expect("transient runs");
        let mut free_worst: Value = 0.0;
        for pair in free.time.windows(2) {
            free_worst = free_worst.max(pair[1] - pair[0]);
        }
        assert!(
            free_worst > 2.0e-6,
            "default config must not silently cap the caller's max step (worst dt {free_worst:.3e})"
        );
    }
}
