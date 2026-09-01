//! Transient Time-Domain Analysis
//!
//! This module provides time-domain simulation using:
//! - Adaptive timestep control with LTE estimation
//! - TrapGear method switching for stability
//! - Optional waveform compression for long simulations
//! - Cooperative abort for responsive cancellation

#![allow(clippy::too_many_arguments)]
mod fft;
use super::{Engine, SimulationError, SpiceDialect, TransientResult};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::device::semiconductor::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeBranch,
    BjtChargeSnapshot,
};
use crate::engine::waveform::{CompressionConfig, TransientResultCompressed};
use crate::netlist::{
    AnalysisCommand, OutputAnalysisKind, OutputDirectiveKind, OutputSymbolKind, SaveSet,
    SaveSignal, is_device_lead_current_accessor, measure_output_dependencies,
};
use crate::numerics::integration::{
    BreakpointManager, BreakpointStepPolicy, LteEstimator, TimestepController,
    TransientErrorControl, TrapGearController, XyceBreakpointSpanCeiling,
    xyce_iteration_step_accepts, xyce_iteration_step_scale,
};
use crate::numerics::integration::{CompanionCoefficients, IntegrationMethod};
use crate::numerics::xyce_hard_min_timestep;
use crate::{Netlist, Value};
use std::collections::{HashMap, HashSet};

use xyce_dae::{XyceOneStepOrder, XyceOneStepWorkspace};

/// How a transient analysis establishes its initial state.
///
/// This is explicit execution state rather than a property of an entire
/// netlist: a deck may contain several `.TRAN` cards with different UIC
/// selections, and checkpoint continuation must preserve the selected card's
/// mode exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransientStartupMode {
    /// Solve the DC operating point before beginning integration.
    OperatingPoint,
    /// Skip the operating point and seed state from `.IC` and device `IC=`.
    Uic,
}

impl TransientStartupMode {
    #[inline]
    pub const fn from_uic(uic: bool) -> Self {
        if uic { Self::Uic } else { Self::OperatingPoint }
    }

    #[inline]
    pub const fn is_uic(self) -> bool {
        matches!(self, Self::Uic)
    }
}

/// Reproduce `StaticMatrix::raw_residual_norms` for an already formed direct
/// Xyce DAE residual.  The scaled sum-of-squares recurrence and its initial
/// value are part of the status-test arithmetic, so this is intentionally not
/// a generic `sqrt(sum(r*r))` helper.
fn direct_xyce_dae_norms(residual: &[Value]) -> Result<(Value, Value), SimulationError> {
    let mut inf_norm: Value = 0.0;
    let mut l2_scale: Value = 0.0;
    let mut l2_sum_squares: Value = 1.0;
    for (index, &value) in residual.iter().enumerate() {
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "direct Xyce DAE residual[{index}] is non-finite: {value}"
            )));
        }
        let magnitude = value.abs();
        inf_norm = inf_norm.max(magnitude);
        if magnitude != 0.0 {
            if l2_scale < magnitude {
                let ratio = l2_scale / magnitude;
                l2_sum_squares = 1.0 + l2_sum_squares * ratio * ratio;
                l2_scale = magnitude;
            } else {
                let ratio = magnitude / l2_scale;
                l2_sum_squares += ratio * ratio;
            }
        }
    }
    let l2_norm = if l2_scale == 0.0 {
        0.0
    } else {
        l2_scale * l2_sum_squares.sqrt()
    };
    Ok((inf_norm, l2_norm))
}

/// Choose the update quantity used by Xyce's two transient nonlinear solvers.
/// DampedNewton consumes the linear solver's search direction directly. NOX
/// intentionally reconstructs `x - old_x` after its line-search update, so it
/// must retain the candidate-difference path even when the underlying linear
/// system happened to use correction form.
#[inline]
fn select_xyce_transient_update_norm(
    uses_damped_newton: bool,
    solved_correction_norm: Option<Value>,
    candidate_difference_norm: impl FnOnce() -> Option<Value>,
) -> Option<Value> {
    if uses_damped_newton {
        solved_correction_norm
    } else {
        candidate_difference_norm()
    }
}

#[inline]
const fn transient_newton_uses_correction_form(
    has_direct_correction_rhs: bool,
    uses_inductor_correction: bool,
    uses_xyce_damped_newton: bool,
) -> bool {
    has_direct_correction_rhs || uses_inductor_correction || uses_xyce_damped_newton
}

#[inline]
const fn allows_postsolve_gmin_rescue(xyce_iteration_error_control: bool) -> bool {
    // Xyce returns a failed transient DampedNewton/NOX solve directly to
    // StepErrorControl. Its ERROPTION=1 path therefore retries at /8 without
    // RSpice's post-solver system deformation.
    !xyce_iteration_error_control
}

#[inline]
const fn xyce_rejected_attempt_order(
    xyce_iteration_error_control: bool,
    attempted_order: u8,
    lte_recovery_order: u8,
) -> u8 {
    if xyce_iteration_error_control {
        attempted_order
    } else {
        lte_recovery_order
    }
}

#[inline]
const fn xyce_startup_or_restart_order(_configured_min_order: u8) -> u8 {
    // Xyce OneStep/Gear12 always initializes at order one. MINORD constrains
    // later error-control demotion; it does not alter initialization.
    1
}

#[inline]
const fn xyce_lte_recovery_order(candidate_order: u8, configured_min_order: u8) -> u8 {
    if candidate_order < configured_min_order {
        configured_min_order
    } else {
        candidate_order
    }
}

#[inline]
fn xyce_iteration_retry_timestep(rejected_dt: Value, hard_min_dt: Value, max_dt: Value) -> Value {
    (rejected_dt * 0.125).clamp(hard_min_dt, max_dt)
}

/// Return the exact requested horizon when a step consumes the remaining
/// transient interval. Floating-point subtraction followed by addition is not
/// an identity for every pair of finite values, so endpoint-sensitive sources,
/// device loads, checkpoints, and recorded samples must share this canonical
/// time instead of independently recomputing `current_time + dt`.
#[inline]
fn canonical_transient_step_time(current_time: Value, dt: Value, stop_time: Value) -> Value {
    if dt >= stop_time - current_time {
        stop_time
    } else {
        current_time + dt
    }
}

/// Preserve the exact absolute time requested by an accepted Verilog-A event.
/// Floating-point subtraction followed by addition is not an identity for
/// every pair, so the event target must remain authoritative while `dt`
/// separately supplies integration coefficients.
#[inline]
fn canonical_transient_step_time_with_device_event(
    current_time: Value,
    dt: Value,
    stop_time: Value,
    exact_device_event_time: Option<Value>,
) -> Value {
    exact_device_event_time
        .unwrap_or_else(|| canonical_transient_step_time(current_time, dt, stop_time))
}

fn accepted_veriloga_event_time(
    circuit: &crate::circuit::CircuitData,
    accepted_time: Value,
    hard_min_dt: Value,
) -> Result<Option<Value>, SimulationError> {
    let Some(target) = circuit
        .veriloga_transient_event_time(accepted_time)
        .map_err(SimulationError::Circuit)?
    else {
        return Ok(None);
    };
    validate_veriloga_event_interval(target, accepted_time, hard_min_dt)?;
    Ok(Some(target))
}

fn validate_veriloga_event_interval(
    target: Value,
    accepted_time: Value,
    hard_min_dt: Value,
) -> Result<(), SimulationError> {
    if !hard_min_dt.is_finite() || hard_min_dt <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Verilog-A event scheduling received invalid solver hard minimum {hard_min_dt}"
        )));
    }
    let event_dt = target - accepted_time;
    if !event_dt.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "Verilog-A event {target} has an invalid interval from accepted time {accepted_time}"
        )));
    }
    if event_dt < hard_min_dt {
        return Err(SimulationError::Circuit(format!(
            "Verilog-A event at t={target:.16e}s requires dt={event_dt:.16e}s below the solver hard minimum {hard_min_dt:.16e}s from accepted time {accepted_time:.16e}s"
        )));
    }
    Ok(())
}

#[inline]
const fn accepted_step_hits_breakpoint(
    landed_device_event: bool,
    solver_landed_on_breakpoint: bool,
    scheduled_breakpoint: bool,
) -> bool {
    landed_device_event || solver_landed_on_breakpoint || scheduled_breakpoint
}

#[inline]
const fn xyce_allows_order_two(max_order: u8) -> bool {
    max_order == 2
}

/// A sanitized finite NOX candidate still has a well-defined candidate
/// difference. A non-finite linear solution does not: carry an explicit
/// failure sentinel into the next ordered status pass instead of reusing a
/// stale norm from the preceding iterate.
#[inline]
fn xyce_nox_recovered_update_norm(
    had_nonfinite_solution: bool,
    candidate_difference_norm: impl FnOnce() -> Option<Value>,
) -> Option<Value> {
    if had_nonfinite_solution {
        Some(Value::INFINITY)
    } else {
        candidate_difference_norm()
    }
}

fn capture_direct_xyce_histories(
    circuit: &crate::circuit::CircuitData,
    solution: &[Value],
    time: Value,
    vectors: &mut crate::circuit::dae::XyceDaeVectors,
    q_candidate: &mut [Value],
    static_candidate: &mut [Value],
) -> Result<(), String> {
    circuit.load_direct_xyce_level2_core_dae(solution, time, 0.0, vectors)?;
    if q_candidate.len() != vectors.q().len() || static_candidate.len() != vectors.f().len() {
        return Err("direct Xyce history scratch has the wrong dimension".into());
    }
    q_candidate.copy_from_slice(vectors.q());
    for ((static_value, &f), &b) in static_candidate
        .iter_mut()
        .zip(vectors.f())
        .zip(vectors.b())
    {
        *static_value = f - b;
        if !static_value.is_finite() {
            return Err("direct Xyce accepted F-B history is non-finite".into());
        }
    }
    Ok(())
}

#[derive(Clone)]
enum TransientMeritRollback {
    ClassicMosOnly(Vec<crate::device::mosfet::MosfetNonlinearState>),
    Full {
        state: Box<crate::circuit::NonlinearDeviceStateSnapshot>,
        cached_vbic: Vec<Option<BjtChargeSnapshot>>,
    },
}

fn restore_transient_merit_rollback(
    circuit: &mut crate::circuit::CircuitData,
    vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
    rollback: &TransientMeritRollback,
) {
    match rollback {
        TransientMeritRollback::ClassicMosOnly(states) => {
            debug_assert!(circuit.has_classic_mos_only_transient_nonlinearity());
            debug_assert!(vbic_snapshot_cache.is_empty());
            circuit.mosfets.restore_nonlinear_state(states.clone());
        }
        TransientMeritRollback::Full { state, cached_vbic } => {
            if circuit.has_xyce_core_inductors() {
                // Xyce's MutIndNonLin2 has no rejectStep callback: its pending
                // MagVarUpdate remains part of the trial state while a nonlinear
                // search backs up. Preserve that carry across RSpice's merit rollback
                // as well, so the globalization path has the same device lifecycle as
                // the canonical Xyce solve.
                circuit.restore_nonlinear_state_preserving_xyce_core_level2_carry(
                    state.as_ref().clone(),
                );
            } else {
                circuit.restore_nonlinear_state(state.as_ref().clone());
            }
            vbic_snapshot_cache.clone_from_slice(cached_vbic);
        }
    }
}

fn capture_transient_merit_rollback(
    circuit: &crate::circuit::CircuitData,
    vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
    classic_mos_only: bool,
    rollback: &mut Option<TransientMeritRollback>,
) {
    if classic_mos_only {
        debug_assert!(circuit.has_classic_mos_only_transient_nonlinearity());
        debug_assert!(vbic_snapshot_cache.is_empty());
        if let Some(TransientMeritRollback::ClassicMosOnly(states)) = rollback.as_mut() {
            circuit.mosfets.nonlinear_state_snapshot_into(states);
        } else {
            *rollback = Some(TransientMeritRollback::ClassicMosOnly(
                circuit.mosfets.nonlinear_state_snapshot(),
            ));
        }
    } else if let Some(TransientMeritRollback::Full { state, cached_vbic }) = rollback.as_mut() {
        circuit.refresh_transient_trial_state_snapshot(state);
        cached_vbic.clear();
        cached_vbic.extend_from_slice(vbic_snapshot_cache);
    } else {
        *rollback = Some(TransientMeritRollback::Full {
            state: Box::new(circuit.transient_trial_state_snapshot()),
            cached_vbic: vbic_snapshot_cache.to_vec(),
        });
    }
}

mod breakpoints;
mod checkpoint;
mod companion_stamps;
pub(self) use companion_stamps::{CompactTwoTerminalStampSlots, TwoTerminalStampSlots};
mod charge_stamper;
pub(self) use charge_stamper::StaticMatrixChargeStamper;
mod damped_status;
mod globalization;
mod noise;
mod nox_status;
mod rescue;
mod residual;
mod restart;
mod startup;
mod state;
mod xyce_dae;
pub(self) use state::{
    AcceptedJunctionHistoryRestart, MosfetCompanionBranchTerms, MosfetGateCompanionCharges,
    ReactiveHistorySeed,
};
mod state_advanced_mos;
mod state_commit;
mod state_recovery;
mod state_transmission_lines;
mod step_control;
mod truncation;
pub(self) use truncation::NgspiceChargeTruncationContext;
mod vbic;

pub use self::{
    checkpoint::{
        DEFAULT_MAX_CHECKPOINT_BYTES, TransientCheckpoint, TransientCheckpointEncoding,
        netlist_fingerprint,
    },
    restart::{
        XYCE_RESTART_SCHEDULE_TOLERANCE, XyceRestartJobPlan, XyceRestartPlanError,
        xyce_restart_schedule_is_due,
    },
};
use checkpoint::{
    AcceptedIntegrationRuntime, AcceptedIntegrationRuntimeCapture,
    AcceptedIntegrationRuntimeCheckpoint, AcceptedIntegrationRuntimeTarget,
    ProposedIntegrationContinuation, RestartNormalizedIntegrationRuntimeCapture,
    RestartNormalizedIntegrationRuntimeCheckpoint, ValidatedAcceptedIntegrationRuntime,
};
pub(crate) use checkpoint::{
    netlist_checkpoint_identity, restart_checkpoint_identity, simulation_checkpoint_identity,
};

mod history;
pub(self) use history::*;

#[derive(Debug, Clone, Copy)]
struct DerivedTransientBranchCurrent {
    kind: DerivedTransientBranchCurrentKind,
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResumeValidation {
    ExactNetlist,
    AuthoredRestart,
}

/// Whether the public caller retains the final checkpoint returned by the
/// shared transient integration body. Plain and scheduled runs discard this
/// implementation artifact; checkpointed and resumed runs expose it as part
/// of their result and therefore charge it to the retained-value policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FinalCheckpointRetention {
    Discarded,
    Retained,
}

impl FinalCheckpointRetention {
    const fn is_retained(self) -> bool {
        matches!(self, Self::Retained)
    }
}

/// One checkpoint emitted by a nominal restart-save schedule.
///
/// Xyce evaluates restart saves after accepted steps. `nominal_time` is the
/// due time used in the filename, while `checkpoint.time` is the first
/// accepted solver time for which `nominal_time - checkpoint.time <=
/// XYCE_RESTART_SCHEDULE_TOLERANCE`. It may precede the nominal time by at
/// most that tolerance or follow it by an unconstrained accepted-step gap.
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduledTransientCheckpoint {
    pub nominal_time: Value,
    pub checkpoint: TransientCheckpoint,
}

#[derive(Debug, Clone, Copy)]
enum DerivedTransientBranchCurrentKind {
    LinearResistor,
    LinearCapacitor,
    IndependentCurrentSource,
    NativeDiode,
    #[cfg(feature = "veriloga-builtins-base")]
    GeneratedVerilogA,
    XyceMemristor,
    BehavioralCurrentSource,
    VoltageSwitch,
    CurrentSwitch,
    GenericSwitch,
}

/// Immutable storage projection compiled once before transient integration.
///
/// `TransientResult` retains its topology-aligned outer vectors so public node
/// and branch indices remain stable. An unselected channel is represented by
/// an empty inner vector and therefore costs no per-point memory or append
/// traffic.
#[derive(Debug)]
struct TransientCapturePlan {
    voltages: Vec<bool>,
    branch_currents: Vec<bool>,
    event_nodes: Vec<bool>,
}

/// Wall-clock timer used exclusively for transient diagnostics.
///
/// A production run must not pay for operating-system clock queries at every
/// Newton phase when debug timing is disabled. Keeping the optionality inside
/// this tiny wrapper makes the hot-loop call sites explicit while compiling
/// down to a predictable branch and zero clock reads in the disabled case.
#[derive(Debug)]
struct DiagnosticTimer(Option<crate::time_compat::Instant>);

impl DiagnosticTimer {
    #[inline]
    fn start(enabled: bool) -> Self {
        Self(enabled.then(crate::time_compat::Instant::now))
    }

    #[inline]
    fn elapsed(&self) -> std::time::Duration {
        self.0
            .as_ref()
            .map(crate::time_compat::Instant::elapsed)
            .unwrap_or_default()
    }
}

impl TransientCapturePlan {
    #[inline]
    fn canonical_symbol(name: &str) -> String {
        name.trim()
            .chars()
            .map(|character| {
                if character == ':' {
                    '.'
                } else {
                    character.to_ascii_uppercase()
                }
            })
            .collect()
    }

    fn request_selects_node(
        netlist: &Netlist,
        request: &crate::netlist::OutputRequest,
        candidate: &str,
        project_complete_wildcard: bool,
    ) -> bool {
        let save_owns_direct_patterns = request.directive == OutputDirectiveKind::Save;
        let selected = if save_owns_direct_patterns {
            request.selects_transient_node_voltage_outside_direct_save_voltage(candidate)
        } else if project_complete_wildcard {
            request.selects_transient_node_voltage_except_complete_wildcard(candidate)
        } else {
            request.selects_transient_node_voltage(candidate)
        };
        if selected {
            return true;
        }

        let candidate = Self::canonical_symbol(candidate);
        request.dependencies.iter().any(|dependency| {
            dependency.kind == OutputSymbolKind::Node
                && (!save_owns_direct_patterns
                    || dependency.expression
                    || !dependency.operator.eq_ignore_ascii_case("V"))
                && (!project_complete_wildcard || dependency.symbol.trim() != "*")
                && request
                    .analysis
                    .is_none_or(|analysis| analysis == OutputAnalysisKind::Tran)
                && Engine::resolve_hierarchical_node_name(netlist, &dependency.symbol)
                    .is_some_and(|resolved| Self::canonical_symbol(&resolved) == candidate)
        })
    }

    fn request_selects_core_winding_branch(
        request: &crate::netlist::OutputRequest,
        candidate: &str,
    ) -> bool {
        let candidate = Self::canonical_symbol(candidate);
        request.dependencies.iter().any(|dependency| {
            if dependency.kind != OutputSymbolKind::Node
                || !dependency.operator.eq_ignore_ascii_case("N")
                || !request
                    .analysis
                    .is_none_or(|analysis| analysis == OutputAnalysisKind::Tran)
            {
                return false;
            }
            let symbol = Self::canonical_symbol(&dependency.symbol);
            let Some(core_parameter) = symbol.strip_prefix("YMIN!") else {
                return false;
            };
            let Some(winding) = core_parameter.strip_suffix("_BRANCH") else {
                return false;
            };
            winding
                .rsplit_once('_')
                .map(|(_, winding)| winding == candidate)
                .unwrap_or(false)
        })
    }

    #[inline]
    fn request_requires_power_voltage(request: &crate::netlist::OutputRequest) -> bool {
        request
            .analysis
            .is_none_or(|analysis| analysis == OutputAnalysisKind::Tran)
            && request.dependencies.iter().any(|dependency| {
                dependency.kind == OutputSymbolKind::Device
                    && (dependency.operator.eq_ignore_ascii_case("P")
                        || dependency.operator.eq_ignore_ascii_case("W"))
            })
    }

    fn compile(
        netlist: &Netlist,
        node_names: &[String],
        branch_names: &[String],
        retain_xyce_voltage_source_currents: bool,
        voltage_source_names: &[String],
        external_wildcard_nodes: Option<&HashSet<String>>,
    ) -> Self {
        // Measurement evaluation currently happens after integration. Until
        // measurements become online reducers, retain all analog operands for
        // measurement decks so output projection cannot change their result.
        let retain_all = netlist.saves.keeps_everything()
            || !netlist.measurements.is_empty()
            || netlist.options.output_snapshots.unwrap_or(false);
        // P()/W() are terminal quantities, not merely branch-current aliases:
        // evaluating them after integration needs both terminal voltage
        // waveforms.  The requested device may be hierarchical and its
        // flattened element ports can be interface aliases, so retaining the
        // complete voltage namespace is the only topology-independent way to
        // preserve that output contract.
        let retain_power_voltages = netlist
            .output_requests
            .iter()
            .any(Self::request_requires_power_voltage);
        let mut explicit_saves = netlist.saves.clone();
        let save_owns_complete_wildcard = netlist
            .output_requests
            .iter()
            .any(crate::netlist::OutputRequest::has_complete_transient_save_voltage_wildcard);
        let project_xyce_complete_print_wildcard = netlist.params.expression_dialect()
            == crate::config::ExpressionDialect::Xyce
            && netlist
                .output_requests
                .iter()
                .any(crate::netlist::OutputRequest::has_complete_transient_voltage_wildcard);
        if external_wildcard_nodes.is_some() {
            explicit_saves
                .signals
                .retain(|signal| !matches!(signal, SaveSignal::Voltage(node) if node == "*"));
        }
        // The parser aggregates PRINT and SAVE operands into `netlist.saves`.
        // Reapply a direct SAVE V(*) separately so it keeps SaveSet's
        // one-level matching semantics while also intersecting the
        // parser-certified public node namespace. Builder-private nodes may
        // have no hierarchy separator (for example a DAE state variable), so
        // name-shape filtering alone is not an output-visibility boundary.
        let complete_save_wildcard = save_owns_complete_wildcard.then(|| SaveSet {
            signals: vec![SaveSignal::Voltage("*".to_string())],
        });
        let voltages = node_names
            .iter()
            .map(|name| {
                let externally_visible = external_wildcard_nodes
                    .is_some_and(|nodes| nodes.contains(&Self::canonical_symbol(name)));
                retain_all
                    || retain_power_voltages
                    || (project_xyce_complete_print_wildcard && externally_visible)
                    || complete_save_wildcard.as_ref().is_some_and(|saves| {
                        externally_visible && saves.retains_voltage_operand(name)
                    })
                    || (!explicit_saves.signals.is_empty()
                        && explicit_saves.retains_voltage_operand(name))
                    || netlist.output_requests.iter().any(|request| {
                        let project_complete_wildcard = project_xyce_complete_print_wildcard
                            && request.has_complete_transient_voltage_wildcard();
                        Self::request_selects_node(
                            netlist,
                            request,
                            name,
                            project_complete_wildcard,
                        )
                    })
            })
            .collect();
        let branch_currents = branch_names
            .iter()
            .map(|name| {
                let is_xyce_voltage_source = retain_xyce_voltage_source_currents
                    && voltage_source_names.iter().any(|source| {
                        Self::canonical_symbol(source) == Self::canonical_symbol(name)
                    });
                retain_all
                    || is_xyce_voltage_source
                    || netlist.saves.selects(&format!("I({name})"))
                    || netlist.output_requests.iter().any(|request| {
                        request.selects_transient_device_current(name)
                            || Self::request_selects_core_winding_branch(request, name)
                    })
            })
            .collect();
        // XSPICE event vectors occupy a distinct result namespace. Bare/raw
        // saves select them, while a typed V(node) request must not do so.
        // Measurements remain conservative until their reducers are online.
        let event_nodes = node_names
            .iter()
            .map(|name| retain_all || netlist.saves.selects_raw_name(name))
            .collect();
        Self {
            voltages,
            branch_currents,
            event_nodes,
        }
    }

    fn analog_values_per_sample(&self) -> usize {
        self.voltages
            .iter()
            .filter(|&&retain| retain)
            .count()
            .saturating_add(
                self.branch_currents
                    .iter()
                    .filter(|&&retain| retain)
                    .count(),
            )
    }
}

impl Engine {
    /// Apply Xyce's dynamic voltage-source timestep contract without changing
    /// native or ngspice stepping. Xyce 7.10 enables `TIMEINT USEDEVICEMAX`
    /// by default and asks each VSRC waveform for its current ceiling.
    #[inline]
    fn transient_device_max_timestep(
        &self,
        circuit: &crate::circuit::CircuitData,
        time: Value,
        global_max_step: Value,
    ) -> Value {
        if self.config.spice_dialect != SpiceDialect::Xyce
            || !self
                .config
                .transient_use_device_max_timestep
                .unwrap_or(true)
        {
            return global_max_step;
        }

        circuit
            .voltage_sources
            .xyce_max_timestep_at(time)
            .map_or(global_max_step, |device_max| {
                global_max_step.min(device_max)
            })
    }

    #[cfg(test)]
    fn normalized_locked_time_grid(grid: &[Value], resume_time: Value) -> Vec<Value> {
        let mut points: Vec<Value> = grid
            .iter()
            .copied()
            .filter(|&point| point.is_finite() && point > resume_time + 1e-30)
            .collect();
        points.sort_by(|a, b| a.total_cmp(b));
        points.dedup_by(|a, b| {
            let scale = a.abs().max(b.abs()).max(Value::MIN_POSITIVE);
            (*a - *b).abs() <= 64.0 * Value::EPSILON * scale
        });
        points
    }

    fn normalized_locked_time_schedule(
        grid: &[Value],
        step_sizes: Option<&[Value]>,
        resume_time: Value,
    ) -> (Vec<Value>, Option<Vec<Value>>) {
        let paired_steps = step_sizes.filter(|steps| steps.len() == grid.len());
        let mut points = grid
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, point)| point.is_finite() && *point > resume_time + 1e-30)
            .collect::<Vec<_>>();
        points.sort_by(|(_, left), (_, right)| left.total_cmp(right));

        let mut normalized = Vec::with_capacity(points.len());
        let mut normalized_steps = paired_steps.map(|_| Vec::with_capacity(points.len()));
        for (original_index, point) in points {
            let duplicate = normalized.last().is_some_and(|previous: &Value| {
                let scale = previous.abs().max(point.abs()).max(Value::MIN_POSITIVE);
                (point - *previous).abs() <= 64.0 * Value::EPSILON * scale
            });
            if duplicate {
                continue;
            }
            normalized.push(point);
            if let (Some(source_steps), Some(target_steps)) =
                (paired_steps, normalized_steps.as_mut())
            {
                target_steps.push(source_steps[original_index]);
            }
        }
        (normalized, normalized_steps)
    }

    /// A contraction inside a locked Xyce grid is the observable trace of an
    /// adaptive retry that restarted OneStep at order one. The locked grid
    /// contains only accepted targets, so replay that restart before the next
    /// interior target while leaving source-breakpoint landings untouched.
    #[inline]
    fn locked_grid_requires_xyce_order_restart(
        grid: &[Value],
        cursor: usize,
        target_is_breakpoint: bool,
    ) -> bool {
        if target_is_breakpoint || cursor < 2 || cursor + 1 >= grid.len() {
            return false;
        }
        let previous_step = grid[cursor - 1] - grid[cursor - 2];
        let current_step = grid[cursor] - grid[cursor - 1];
        let valid_steps = previous_step.is_finite()
            && previous_step > 0.0
            && current_step.is_finite()
            && current_step > 0.0;
        if !valid_steps {
            return false;
        }

        // A contraction is the observable trace of the accepted retry that
        // restarts OneStep at order one.  Keep the preceding interval at its
        // native order: Xyce applies the restart to the first step *after*
        // the contraction, not retroactively to the larger step that landed
        // at the contraction point.
        current_step < 0.75 * previous_step
    }

    #[inline]
    fn dialect_requires_locked_grid_order_restart(
        dialect: SpiceDialect,
        grid: &[Value],
        cursor: usize,
        target_is_breakpoint: bool,
    ) -> bool {
        dialect == SpiceDialect::Xyce
            && Self::locked_grid_requires_xyce_order_restart(grid, cursor, target_is_breakpoint)
    }

    /// Xyce IC capacitors use an operating-point-only branch unknown. Their
    /// transient lead current is reconstructed by the time integrator from
    /// the capacitor charge history, rather than being solved as an MNA
    /// branch. RSpice keeps that current in the IC branch unknown, so the two
    /// representations cannot be combined with OneStep's order-2 split
    /// (which deliberately separates the static F and dynamic dQ/dt terms).
    /// Retain the order-1 companion for every circuit containing an IC branch
    /// until the solver has a single canonical representation for this state.
    #[inline]
    fn xyce_one_step_requires_order_one_for_ic_branch(
        circuit: &crate::circuit::CircuitData,
    ) -> bool {
        circuit
            .capacitors
            .ic_branch_indices
            .iter()
            .any(Option::is_some)
    }

    /// The OneStep split keeps the static DAE history separate from each
    /// device's dynamic charge history.  RSpice's magnetic companions and
    /// hysteretic switches retain additional accepted state that is not yet
    /// represented by that static-history snapshot.  Keep those topologies on
    /// the canonical companion path until their complete DAE history mapping
    /// is available; mixing the two contracts would silently alter the
    /// physical transient equations.
    #[inline]
    fn xyce_one_step_requires_order_one_for_stateful_topology(
        circuit: &crate::circuit::CircuitData,
    ) -> bool {
        (!circuit.inductors.names.is_empty() && !circuit.has_only_xyce_core_inductors())
            || !circuit.coupled_inductor_pairs.is_empty()
            || !circuit.multi_winding_transformers.is_empty()
            || !circuit.vswitches.is_empty()
            || !circuit.iswitches.is_empty()
            || !circuit.generic_switches.is_empty()
            || !circuit.veriloga_one_step_dae_split_safe()
    }

    fn derived_transient_branch_currents(
        netlist: &Netlist,
        circuit: &crate::circuit::CircuitData,
        existing_branch_names: &[String],
    ) -> Vec<DerivedTransientBranchCurrent> {
        // An empty save set means the public engine API retains every vector.
        // Measurements are evaluated after integration and may reference a
        // current absent from .PRINT/.SAVE, so remain conservative for them.
        let retain_all = netlist.saves.keeps_everything() || !netlist.measurements.is_empty();
        let requests_derived_current = netlist
            .saves
            .signals
            .iter()
            .any(|signal| matches!(signal, SaveSignal::Current(_)))
            || netlist
                .output_requests
                .iter()
                .any(|request| request.requires_transient_device_current_operand());
        if !retain_all && !requests_derived_current {
            return Vec::new();
        }

        // Real MNA branches take precedence over synthesized lead currents,
        // and the first derived device with a duplicate name wins.  The old
        // implementation preserved that rule by linearly scanning both the
        // MNA names and every branch already appended for every candidate.
        // Large passive decks therefore spent O(devices^2) time here before
        // the first transient step, only to discard unrequested currents in a
        // second pass.  Track canonical names once and apply output selection
        // before materializing a branch instead.
        let mut seen_names = HashSet::with_capacity(
            existing_branch_names
                .len()
                .saturating_add(circuit.resistors.names.len())
                .saturating_add(circuit.capacitors.names.len())
                .saturating_add(circuit.current_sources.names.len())
                .saturating_add(circuit.diodes.devices.len())
                .saturating_add({
                    #[cfg(feature = "veriloga-builtins-base")]
                    {
                        circuit.generated_veriloga_devices().len()
                    }
                    #[cfg(not(feature = "veriloga-builtins-base"))]
                    {
                        0
                    }
                }),
        );
        seen_names.extend(
            existing_branch_names
                .iter()
                .map(|name| name.to_ascii_lowercase()),
        );

        let mut derived = Vec::new();
        let mut consider = |name: &str, branch: DerivedTransientBranchCurrent| {
            if !seen_names.insert(name.to_ascii_lowercase()) {
                return;
            }
            if retain_all
                || netlist.saves.selects(&format!("I({name})"))
                || netlist
                    .output_requests
                    .iter()
                    .any(|request| request.selects_transient_device_current(name))
            {
                derived.push(branch);
            }
        };

        for (index, name) in circuit.resistors.names.iter().enumerate() {
            consider(
                name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::LinearResistor,
                    index,
                },
            );
        }
        for (index, name) in circuit.capacitors.names.iter().enumerate() {
            if circuit.capacitors.is_internal(index) {
                continue;
            }
            consider(
                name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::LinearCapacitor,
                    index,
                },
            );
        }
        for (index, name) in circuit.current_sources.names.iter().enumerate() {
            consider(
                name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::IndependentCurrentSource,
                    index,
                },
            );
        }
        for (index, diode) in circuit.diodes.devices.iter().enumerate() {
            consider(
                &diode.name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::NativeDiode,
                    index,
                },
            );
        }
        #[cfg(feature = "veriloga-builtins-base")]
        for (index, device) in circuit.generated_veriloga_devices().iter().enumerate() {
            if device.external_terminals().len() != 2 {
                continue;
            }
            consider(
                &device.instance_name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::GeneratedVerilogA,
                    index,
                },
            );
        }
        for (index, binding) in circuit.xyce_memristors.iter().enumerate() {
            consider(
                &binding.name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::XyceMemristor,
                    index,
                },
            );
        }
        for (index, source) in circuit
            .behavioral_sources
            .current_sources
            .iter()
            .enumerate()
        {
            consider(
                &source.name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::BehavioralCurrentSource,
                    index,
                },
            );
        }
        for (index, switch) in circuit.vswitches.iter().enumerate() {
            consider(
                &switch.name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::VoltageSwitch,
                    index,
                },
            );
        }
        for (index, switch) in circuit.iswitches.iter().enumerate() {
            consider(
                &switch.name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::CurrentSwitch,
                    index,
                },
            );
        }
        for (index, switch) in circuit.generic_switches.iter().enumerate() {
            consider(
                &switch.name,
                DerivedTransientBranchCurrent {
                    kind: DerivedTransientBranchCurrentKind::GenericSwitch,
                    index,
                },
            );
        }
        derived
    }

    #[inline]
    fn derived_transient_branch_name_ref(
        circuit: &crate::circuit::CircuitData,
        branch: DerivedTransientBranchCurrent,
    ) -> &str {
        match branch.kind {
            DerivedTransientBranchCurrentKind::LinearResistor => {
                &circuit.resistors.names[branch.index]
            }
            DerivedTransientBranchCurrentKind::LinearCapacitor => {
                &circuit.capacitors.names[branch.index]
            }
            DerivedTransientBranchCurrentKind::IndependentCurrentSource => {
                &circuit.current_sources.names[branch.index]
            }
            DerivedTransientBranchCurrentKind::NativeDiode => {
                &circuit.diodes.devices[branch.index].name
            }
            #[cfg(feature = "veriloga-builtins-base")]
            DerivedTransientBranchCurrentKind::GeneratedVerilogA => circuit
                .generated_veriloga_devices()
                .two_terminal_instance_name(branch.index)
                .expect("derived generated-device branch index is topology-aligned"),
            DerivedTransientBranchCurrentKind::XyceMemristor => {
                &circuit.xyce_memristors[branch.index].name
            }
            DerivedTransientBranchCurrentKind::BehavioralCurrentSource => {
                &circuit.behavioral_sources.current_sources[branch.index].name
            }
            DerivedTransientBranchCurrentKind::VoltageSwitch => {
                &circuit.vswitches[branch.index].name
            }
            DerivedTransientBranchCurrentKind::CurrentSwitch => {
                &circuit.iswitches[branch.index].name
            }
            DerivedTransientBranchCurrentKind::GenericSwitch => {
                &circuit.generic_switches[branch.index].name
            }
        }
    }

    fn derived_transient_branch_name(
        circuit: &crate::circuit::CircuitData,
        branch: DerivedTransientBranchCurrent,
    ) -> String {
        Self::derived_transient_branch_name_ref(circuit, branch).to_owned()
    }

    fn solution_node_voltage(solution: &[Value], node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            solution.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    fn two_terminal_conductance_current(
        solution: &[Value],
        node_pos: usize,
        node_neg: usize,
        conductance: Value,
    ) -> Value {
        let v_pos = Self::solution_node_voltage(solution, node_pos);
        let v_neg = Self::solution_node_voltage(solution, node_neg);
        (v_pos - v_neg) * conductance
    }

    fn derived_transient_branch_current(
        circuit: &mut crate::circuit::CircuitData,
        solution: &[Value],
        time: Value,
        diode_history: Option<&DiodeTransientHistory>,
        branch: DerivedTransientBranchCurrent,
    ) -> Result<Value, SimulationError> {
        let current = match branch.kind {
            DerivedTransientBranchCurrentKind::LinearResistor => {
                let stamp = circuit.resistors.stamps[branch.index];
                Self::two_terminal_conductance_current(
                    solution,
                    stamp.pp.row,
                    stamp.nn.row,
                    circuit.resistors.output_conductance(branch.index),
                )
            }
            DerivedTransientBranchCurrentKind::LinearCapacitor => {
                circuit.capacitors.i_prev[branch.index]
            }
            DerivedTransientBranchCurrentKind::IndependentCurrentSource => {
                circuit.current_sources.value_at_time(branch.index, time)
            }
            DerivedTransientBranchCurrentKind::NativeDiode => {
                let diode = &circuit.diodes.devices[branch.index];
                let voltage = Self::solution_node_voltage(solution, diode.node_anode)
                    - Self::solution_node_voltage(solution, diode.node_cathode);
                let displacement_current = match diode_history {
                    Some(history) => {
                        history.cqd_prev.get(branch.index).copied().ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "accepted diode current history is missing instance '{}'",
                                diode.name
                            ))
                        })?
                    }
                    None => 0.0,
                };
                diode.stamped_conduction_current(voltage) + displacement_current
            }
            #[cfg(feature = "veriloga-builtins-base")]
            DerivedTransientBranchCurrentKind::GeneratedVerilogA => circuit
                .generated_veriloga_devices()
                .primary_terminal_current(branch.index)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "generated Verilog-A instance '{}' has no primary terminal current",
                        Self::derived_transient_branch_name_ref(circuit, branch)
                    ))
                })?,
            DerivedTransientBranchCurrentKind::XyceMemristor => {
                let binding = &circuit.xyce_memristors[branch.index];
                let v_pos = Self::solution_node_voltage(solution, binding.node_pos);
                let v_neg = Self::solution_node_voltage(solution, binding.node_neg);
                let x = Self::solution_node_voltage(solution, binding.node_x);
                let resistance_factor = binding
                    .resistance_noise
                    .as_ref()
                    .map_or(1.0, |noise| noise.resistance_factor());
                binding
                    .device
                    .current_output_with_resistance_factor(v_pos, v_neg, x, resistance_factor)
                    .map_err(|error| {
                        SimulationError::Circuit(format!(
                            "{} memristor '{}' output evaluation failed: {error}",
                            binding.device.family_name(),
                            binding.name
                        ))
                    })?
            }
            DerivedTransientBranchCurrentKind::BehavioralCurrentSource => {
                circuit.behavioral_sources.current_sources[branch.index]
                    .evaluate(solution, time)
                    .map_err(|error| SimulationError::Circuit(error.to_string()))?
            }
            DerivedTransientBranchCurrentKind::VoltageSwitch => {
                let switch = &circuit.vswitches[branch.index];
                Self::two_terminal_conductance_current(
                    solution,
                    switch.node_pos,
                    switch.node_neg,
                    1.0 / switch.resistance().max(1.0e-30),
                )
            }
            DerivedTransientBranchCurrentKind::CurrentSwitch => {
                let switch = &circuit.iswitches[branch.index];
                Self::two_terminal_conductance_current(
                    solution,
                    switch.node_pos,
                    switch.node_neg,
                    1.0 / switch.resistance().max(1.0e-30),
                )
            }
            DerivedTransientBranchCurrentKind::GenericSwitch => {
                let switch = &circuit.generic_switches[branch.index];
                Self::two_terminal_conductance_current(
                    solution,
                    switch.node_pos,
                    switch.node_neg,
                    switch.conductance(),
                )
            }
        };
        Ok(current)
    }

    fn xyce_memristor_resistance_output(
        binding: &crate::circuit::XyceMemristorBinding,
        solution: &[Value],
    ) -> Result<Option<Value>, SimulationError> {
        let v_pos = Self::solution_node_voltage(solution, binding.node_pos);
        let v_neg = Self::solution_node_voltage(solution, binding.node_neg);
        let x = Self::solution_node_voltage(solution, binding.node_x);
        let resistance_factor = binding
            .resistance_noise
            .as_ref()
            .map_or(1.0, |noise| noise.resistance_factor());
        binding
            .device
            .resistance_output_with_factor(v_pos, v_neg, x, resistance_factor)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "{} memristor '{}' resistance output evaluation failed: {error}",
                    binding.device.family_name(),
                    binding.name
                ))
            })
    }

    fn initial_transient_branch_currents(
        circuit: &mut crate::circuit::CircuitData,
        solution: &[Value],
        num_nodes: usize,
        time: Value,
        uic_requested: bool,
        derived_branches: &[DerivedTransientBranchCurrent],
    ) -> Result<Vec<Vec<Value>>, SimulationError> {
        let mut currents: Vec<Vec<Value>> = (0..circuit.num_branches())
            .map(|i| vec![solution.get(num_nodes + i).copied().unwrap_or(0.0)])
            .collect();
        for &branch in derived_branches {
            let current = if uic_requested
                && matches!(
                    branch.kind,
                    DerivedTransientBranchCurrentKind::IndependentCurrentSource
                ) {
                // Xyce's NOOP/UIC initialization emits the pre-source-load
                // lead-current state at t=0.  The source value becomes
                // observable on the first accepted transient step; ordinary
                // operating-point startup continues to report its value at
                // the initial sample.
                0.0
            } else {
                Self::derived_transient_branch_current(circuit, solution, time, None, branch)?
            };
            currents.push(vec![current]);
        }
        Ok(currents)
    }

    fn transient_result_value_count(result: &TransientResult) -> usize {
        result
            .time
            .len()
            .saturating_add(
                result
                    .voltages
                    .iter()
                    .map(Vec::len)
                    .fold(0usize, usize::saturating_add),
            )
            .saturating_add(
                result
                    .branch_currents
                    .iter()
                    .map(Vec::len)
                    .fold(0usize, usize::saturating_add),
            )
            .saturating_add(
                result
                    .device_op_traces
                    .iter()
                    .map(|trace| trace.values.len())
                    .fold(0usize, usize::saturating_add),
            )
            .saturating_add(
                result
                    .store_traces
                    .iter()
                    .map(|trace| trace.values.len())
                    .fold(0usize, usize::saturating_add),
            )
            .saturating_add(
                result
                    .digital_traces
                    .iter()
                    .map(|trace| trace.points.len().saturating_mul(2))
                    .fold(0usize, usize::saturating_add),
            )
            .saturating_add(
                result
                    .real_traces
                    .iter()
                    .map(|trace| trace.points.len().saturating_mul(2))
                    .fold(0usize, usize::saturating_add),
            )
            .saturating_add(
                result
                    .fft_results
                    .iter()
                    .map(|spectrum| {
                        let bin_values = spectrum
                            .bins
                            .len()
                            .saturating_mul(fft::FFT_RETAINED_VALUES_PER_BIN);
                        let metric_values = spectrum.metrics.as_ref().map_or(0, |metrics| {
                            7usize
                                .saturating_add(usize::from(metrics.sfdr_spur_frequency.is_some()))
                                .saturating_add(
                                    metrics
                                        .largest_harmonics
                                        .len()
                                        .saturating_mul(fft::FFT_RETAINED_VALUES_PER_HARMONIC),
                                )
                        });
                        bin_values.saturating_add(metric_values)
                    })
                    .fold(0usize, usize::saturating_add),
            )
    }

    fn ensure_transient_result_limits(
        &self,
        result: &TransientResult,
        retained_values: usize,
    ) -> Result<(), SimulationError> {
        self.ensure_analysis_points(result.time.len())?;
        self.ensure_result_values(retained_values)
    }

    fn transient_initial_trace_capacity(
        &self,
        duration: Value,
        max_step: Value,
        values_per_sample: usize,
    ) -> usize {
        const MAX_EAGER_TRACE_POINTS: usize = 65_536;
        let requested =
            if duration.is_finite() && duration > 0.0 && max_step.is_finite() && max_step > 0.0 {
                (duration / max_step).ceil() as usize
            } else {
                1
            };
        let with_headroom = requested
            .saturating_add(requested / 8)
            .saturating_add(4)
            .max(1);
        let value_bound = if values_per_sample == 0 {
            self.config.resource_limits.max_analysis_points
        } else {
            self.config.resource_limits.max_result_values / values_per_sample
        };
        with_headroom
            .min(self.config.resource_limits.max_analysis_points)
            .min(value_bound.max(1))
            .min(MAX_EAGER_TRACE_POINTS)
    }

    fn seeded_transient_trace(value: Value, retain: bool, capacity: usize) -> Vec<Value> {
        if !retain {
            return Vec::new();
        }
        let mut trace = Vec::with_capacity(capacity.max(1));
        trace.push(value);
        trace
    }

    fn ensure_transient_request_floor(
        &self,
        duration: Value,
        max_step: Value,
    ) -> Result<(), SimulationError> {
        if let Some(grid) = self.config.locked_time_grid.as_deref() {
            return self.ensure_analysis_points(grid.len());
        }
        let requested = (duration / max_step).ceil() as usize;
        self.ensure_analysis_points(requested.saturating_add(1))
    }

    fn record_transient_solution_sample(
        &self,
        result: &mut TransientResult,
        circuit: &mut crate::circuit::CircuitData,
        solution: &[Value],
        num_nodes: usize,
        time: Value,
        step_size: Value,
        derived_branches: &[DerivedTransientBranchCurrent],
        bjt_history: &BjtTransientHistory,
        diode_history: &DiodeTransientHistory,
        record_device_op_traces: bool,
        capture: &TransientCapturePlan,
        trajectory_point_count: usize,
        abort: &dyn AbortSignal,
    ) -> Result<usize, SimulationError> {
        let next_point_count = result.time.len().saturating_add(1);
        self.ensure_analysis_points(trajectory_point_count)?;
        self.ensure_result_shape(
            next_point_count,
            capture
                .analog_values_per_sample()
                .saturating_add(result.store_traces.len())
                .saturating_add(1),
        )?;
        let mut added_values = 1usize;
        result.time.push(time);
        result.step_sizes.push(step_size);
        for (i, (voltages, &retain)) in result
            .voltages
            .iter_mut()
            .zip(&capture.voltages)
            .take(num_nodes)
            .enumerate()
        {
            if retain {
                voltages.push(solution.get(i).copied().unwrap_or(0.0));
                added_values = added_values.saturating_add(1);
            }
        }
        for (index, binding) in circuit.xyce_memristors.iter_mut().enumerate() {
            let trace = result.store_traces.get_mut(index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "{} memristor '{}' resistance output channel is missing",
                    binding.device.family_name(),
                    binding.name
                ))
            })?;
            if let Some(resistance) = Self::xyce_memristor_resistance_output(binding, solution)? {
                binding.resistance_store = resistance;
            }
            trace.values.push(binding.resistance_store);
            added_values = added_values.saturating_add(1);
        }

        let solved_branch_count = circuit.num_branches();
        for (i, (currents, &retain)) in result
            .branch_currents
            .iter_mut()
            .zip(&capture.branch_currents)
            .take(solved_branch_count)
            .enumerate()
        {
            if retain {
                currents.push(solution.get(num_nodes + i).copied().unwrap_or(0.0));
                added_values = added_values.saturating_add(1);
            }
        }
        // Solved branch unknowns are recorded verbatim. In particular, a
        // Xyce IC capacitor's branch is now its physical terminal-KCL current,
        // so post-solving reconstruction would only discard precision.
        for ((branch, currents), &retain) in derived_branches
            .iter()
            .zip(result.branch_currents.iter_mut().skip(solved_branch_count))
            .zip(capture.branch_currents.iter().skip(solved_branch_count))
        {
            if retain {
                currents.push(Self::derived_transient_branch_current(
                    circuit,
                    solution,
                    time,
                    Some(diode_history),
                    *branch,
                )?);
                added_values = added_values.saturating_add(1);
            }
        }
        if record_device_op_traces {
            added_values = added_values.saturating_add(
                result.record_device_op_sample(
                    circuit
                        .transient_device_op_report(
                            solution,
                            &diode_history.cqd_prev,
                            &bjt_history.accepted_terminal_currents,
                        )
                        .map_err(SimulationError::Circuit)?,
                ),
            );
        }
        circuit.accept_generic_switch_transient_step();
        abort.observe_transient_sample(result.observable_sample());
        Ok(added_values)
    }

    fn backfill_initial_linear_capacitor_branch_currents(
        result: &mut TransientResult,
        circuit: &crate::circuit::CircuitData,
        derived_branches: &[DerivedTransientBranchCurrent],
    ) {
        if result.time.len() != 1 {
            return;
        }

        let solved_branch_count = circuit.num_branches();
        for (derived_index, branch) in derived_branches.iter().enumerate() {
            if !matches!(
                branch.kind,
                DerivedTransientBranchCurrentKind::LinearCapacitor
            ) || circuit.capacitors.ic[branch.index].is_none()
            {
                continue;
            }
            let Some(initial_current) = result
                .branch_currents
                .get_mut(solved_branch_count + derived_index)
                .and_then(|waveform| waveform.first_mut())
            else {
                continue;
            };
            *initial_current = circuit.capacitors.i_prev[branch.index];
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
    /// For cancellable simulations, use [`Self::run_tran_with_abort`] instead.
    pub fn run_tran(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
    ) -> Result<TransientResult, SimulationError> {
        self.run_tran_with_abort(netlist, tstop, max_step, &NoAbort)
    }

    /// Run transient analysis with an explicit startup contract.
    ///
    /// Frontends executing one selected `.TRAN` card should use this method so
    /// another card in the same deck cannot change whether the operating point
    /// is skipped.
    pub fn run_tran_with_startup_mode(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
    ) -> Result<TransientResult, SimulationError> {
        self.run_tran_with_startup_mode_and_abort(netlist, tstop, max_step, startup_mode, &NoAbort)
    }

    /// Return the exact discontinuity/event times authored by selected
    /// independent transient sources over `[0, tstop]`.
    ///
    /// This uses the same dialect-aware PULSE/PWL/PAT/default-resolution code
    /// and immutable external-PWL snapshots as the transient integrator. An
    /// empty `source_names` slice selects every independent source. Names are
    /// matched case-insensitively and unknown names are rejected rather than
    /// silently producing an incomplete event-aligned schedule.
    ///
    /// The returned schedule intentionally excludes internally generated
    /// switch, transmission-line, behavioral-expression, and XSPICE events;
    /// it is the source-event contract needed by analyses whose sampling is
    /// explicitly aligned to authored modulation sources.
    pub fn transient_source_event_times(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        source_names: &[String],
    ) -> Result<Vec<Value>, SimulationError> {
        self.transient_source_event_times_with_abort(
            netlist,
            tstop,
            max_step,
            source_names,
            &NoAbort,
        )
    }

    /// Cancellable, resource-bounded form of
    /// [`Self::transient_source_event_times`].
    pub fn transient_source_event_times_with_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        source_names: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        validate_transient_window(tstop, max_step)?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_transient_request_floor(tstop, max_step)?;
        match noise::expand_transient_noise(netlist, tstop).map_err(SimulationError::Circuit)? {
            Some(expanded) => engine.transient_source_event_times_resolved(
                &expanded,
                tstop,
                max_step,
                source_names,
                abort,
            ),
            None => engine.transient_source_event_times_resolved(
                netlist,
                tstop,
                max_step,
                source_names,
                abort,
            ),
        }
    }

    /// List every elaborated independent source with an authored transient
    /// waveform in deterministic canonical-name order.
    pub fn transient_source_names(
        &self,
        netlist: &Netlist,
    ) -> Result<Vec<String>, SimulationError> {
        self.transient_source_names_with_abort(netlist, &NoAbort)
    }

    /// Cancellable form of [`Self::transient_source_names`]. Names are the
    /// canonical, hierarchy-expanded circuit identities accepted by
    /// [`Self::validate_transient_source_names`].
    pub fn transient_source_names_with_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<String>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        let mut names = circuit
            .voltage_sources
            .transient_specs_named_with_pwl()
            .chain(circuit.current_sources.transient_specs_named_with_pwl())
            .map(|(name, _, _)| name.to_owned())
            .collect::<Vec<_>>();
        names.sort_by(|left, right| {
            left.to_ascii_lowercase()
                .cmp(&right.to_ascii_lowercase())
                .then_with(|| left.cmp(right))
        });
        names.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
        Ok(names)
    }

    /// Validate that every requested case-insensitive source name resolves to
    /// an elaborated independent source with an authored transient waveform.
    pub fn validate_transient_source_names(
        &self,
        netlist: &Netlist,
        source_names: &[String],
    ) -> Result<(), SimulationError> {
        self.validate_transient_source_names_with_abort(netlist, source_names, &NoAbort)
    }

    /// Cancellable form of [`Self::validate_transient_source_names`].
    pub fn validate_transient_source_names_with_abort(
        &self,
        netlist: &Netlist,
        source_names: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::validated_transient_source_selection(&circuit, source_names)?;
        Ok(())
    }

    /// Validate the closed set of independent sources that defines a driven
    /// PSS period. Every elaborated time-varying source must be named, and each
    /// source must be strictly periodic and commensurate with `fundamental`.
    /// This prevents a Tones edit from becoming display-only while an omitted
    /// source continues to drive the numerical solve.
    pub fn validate_periodic_source_contract(
        &self,
        netlist: &Netlist,
        source_names: &[String],
        fundamental: Value,
    ) -> Result<(), SimulationError> {
        self.validate_periodic_source_contract_with_abort(
            netlist,
            source_names,
            fundamental,
            &NoAbort,
        )
    }

    /// Cancellable form of [`Self::validate_periodic_source_contract`].
    pub fn validate_periodic_source_contract_with_abort(
        &self,
        netlist: &Netlist,
        source_names: &[String],
        fundamental: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !fundamental.is_finite() || fundamental <= 0.0 {
            return Err(SimulationError::Circuit(
                "periodic source fundamental must be finite and positive".to_owned(),
            ));
        }
        let engine = self.resolved_for_netlist(netlist);
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        let selected = Self::validated_transient_source_selection(&circuit, source_names)?;
        let sources = circuit
            .voltage_sources
            .transient_specs_named_with_pwl()
            .chain(circuit.current_sources.transient_specs_named_with_pwl())
            .collect::<Vec<_>>();
        let available = sources
            .iter()
            .map(|(name, _, _)| name.to_ascii_lowercase())
            .collect::<std::collections::HashSet<_>>();
        if selected != available {
            let mut omitted = sources
                .iter()
                .filter(|(name, _, _)| !selected.contains(&name.to_ascii_lowercase()))
                .map(|(name, _, _)| (*name).to_owned())
                .collect::<Vec<_>>();
            omitted.sort_by_key(|name| name.to_ascii_lowercase());
            return Err(SimulationError::Circuit(format!(
                "periodic source selection must name the complete elaborated source set; omitted: {}",
                omitted.join(", ")
            )));
        }
        for (index, (name, spec, _)) in sources.iter().enumerate() {
            if index & 0x1f == 0 && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            Self::validate_periodic_source_spec(name, spec, fundamental)?;
        }
        Ok(())
    }

    fn validate_periodic_source_spec(
        name: &str,
        spec: &crate::netlist::SourceSpec,
        fundamental: Value,
    ) -> Result<(), SimulationError> {
        use crate::netlist::SourceSpec;

        let source_frequency = match spec {
            SourceSpec::Distortion { inner, .. }
            | SourceSpec::RfPort { inner, .. }
            | SourceSpec::DcTransient {
                transient: inner, ..
            }
            | SourceSpec::DcAcTransient {
                transient: inner, ..
            } => return Self::validate_periodic_source_spec(name, inner, fundamental),
            SourceSpec::Sin {
                frequency,
                delay,
                damping,
                ..
            } => {
                if *delay != 0.0 || *damping != 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "periodic source '{name}' uses a delayed or damped SIN waveform"
                    )));
                }
                *frequency
            }
            SourceSpec::Pulse { period, delay, .. } => {
                if *delay != 0.0 || !period.is_finite() || *period <= 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "periodic source '{name}' requires an undelayed PULSE with a positive period"
                    )));
                }
                1.0 / *period
            }
            SourceSpec::Pwl { .. } | SourceSpec::PwlFile { .. } => {
                return Err(SimulationError::Circuit(format!(
                    "periodic source '{name}' uses PWL; exact PWL period authentication is unavailable"
                )));
            }
            SourceSpec::Exp { .. } => {
                return Err(SimulationError::Circuit(format!(
                    "periodic source '{name}' uses the non-periodic EXP waveform"
                )));
            }
            other => {
                return Err(SimulationError::Circuit(format!(
                    "periodic source '{name}' uses unsupported waveform {other:?}; driven PSS accepts undelayed SIN and PULSE sources"
                )));
            }
        };
        if !source_frequency.is_finite() || source_frequency <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "periodic source '{name}' has a non-positive waveform frequency"
            )));
        }
        let ratio = source_frequency / fundamental;
        let nearest = ratio.round();
        let commensurate =
            nearest >= 1.0 && (ratio - nearest).abs() <= 1.0e-9 * ratio.abs().max(1.0);
        if !commensurate {
            return Err(SimulationError::Circuit(format!(
                "periodic source '{name}' frequency {source_frequency:.17e} Hz is not an integer multiple of the PSS fundamental {fundamental:.17e} Hz"
            )));
        }
        Ok(())
    }

    fn validated_transient_source_selection(
        circuit: &crate::circuit::CircuitData,
        source_names: &[String],
    ) -> Result<std::collections::HashSet<String>, SimulationError> {
        let available = circuit
            .voltage_sources
            .names
            .iter()
            .chain(&circuit.current_sources.names)
            .map(|name| name.to_ascii_lowercase())
            .collect::<std::collections::HashSet<_>>();
        let transient = circuit
            .voltage_sources
            .transient_specs_named_with_pwl()
            .chain(circuit.current_sources.transient_specs_named_with_pwl())
            .map(|(name, _, _)| name.to_ascii_lowercase())
            .collect::<std::collections::HashSet<_>>();
        let mut selected = std::collections::HashSet::with_capacity(source_names.len());
        for source in source_names {
            let normalized = source.trim().to_ascii_lowercase();
            if normalized.is_empty() {
                return Err(SimulationError::Circuit(
                    "transient source selection contains an empty source name".to_string(),
                ));
            }
            if !selected.insert(normalized.clone()) {
                return Err(SimulationError::Circuit(format!(
                    "transient source selection repeats independent source '{normalized}'"
                )));
            }
            if !available.contains(&normalized) {
                return Err(SimulationError::Circuit(format!(
                    "transient source selection references unknown independent source '{normalized}'"
                )));
            }
            if !transient.contains(&normalized) {
                return Err(SimulationError::Circuit(format!(
                    "transient source selection '{normalized}' has no time-varying waveform"
                )));
            }
        }
        Ok(selected)
    }

    fn transient_source_event_times_resolved(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        source_names: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let circuit = self.build_circuit_with_abort(netlist, abort)?;
        let selected = Self::validated_transient_source_selection(&circuit, source_names)?;

        let hinted_max_step = circuit
            .transient_max_step_hint
            .map_or(max_step, |hint| max_step.min(hint));
        let hinted_max_step = self
            .config
            .transient_timeint_max_timestep
            .filter(|bound| bound.is_finite() && *bound > 0.0)
            .map_or(hinted_max_step, |bound| hinted_max_step.min(bound));
        let hinted_max_step =
            if self.config.max_timestep.is_finite() && self.config.max_timestep > 0.0 {
                hinted_max_step.min(self.config.max_timestep)
            } else {
                hinted_max_step
            };
        let source_step_hint = Self::transient_source_step_hint(netlist, hinted_max_step);
        let mut breakpoints = BreakpointManager::new();
        Self::collect_independent_source_breakpoints(
            &circuit,
            tstop,
            source_step_hint,
            self.config.spice_dialect,
            (!selected.is_empty()).then_some(&selected),
            &mut breakpoints,
            abort,
            self.config.resource_limits.max_analysis_points,
        )?;
        Ok(breakpoints.times().to_vec())
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
        let startup_mode = Self::inferred_transient_startup_mode(netlist)?;
        self.run_tran_with_startup_mode_and_abort(netlist, tstop, max_step, startup_mode, abort)
    }

    /// Cancellable transient analysis with an explicit selected-card startup
    /// contract.
    pub fn run_tran_with_startup_mode_and_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, SimulationError> {
        validate_transient_window(tstop, max_step)?;
        self.reset_convergence_quality();
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_transient_request_floor(tstop, max_step)?;
        // TRNOISE sources expand into seeded, deterministic PWL sample
        // trains covering [0, tstop] before circuit construction; decks
        // without noise sources pass through untouched (no clone).
        match noise::expand_transient_noise(netlist, tstop).map_err(SimulationError::Circuit)? {
            Some(expanded) => {
                engine.run_tran_with_abort_resolved(&expanded, tstop, max_step, startup_mode, abort)
            }
            None => {
                engine.run_tran_with_abort_resolved(netlist, tstop, max_step, startup_mode, abort)
            }
        }
    }

    fn inferred_transient_startup_mode(
        netlist: &Netlist,
    ) -> Result<TransientStartupMode, SimulationError> {
        let mut selected = None;
        for analysis in &netlist.analyses {
            let AnalysisCommand::Tran { uic, .. } = analysis else {
                continue;
            };
            let mode = TransientStartupMode::from_uic(*uic);
            if selected.is_some_and(|existing| existing != mode) {
                return Err(SimulationError::Circuit(
                    "netlist contains both UIC and operating-point .TRAN cards; execute the selected card with an explicit TransientStartupMode"
                        .to_string(),
                ));
            }
            selected = Some(mode);
        }
        Ok(selected.unwrap_or(TransientStartupMode::OperatingPoint))
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
        self.run_tran_checkpointed_with_abort(netlist, tstop, max_step, &NoAbort)
    }

    /// Run a checkpointed transient with cooperative cancellation.
    pub fn run_tran_checkpointed_with_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        let startup_mode = Self::inferred_transient_startup_mode(netlist)?;
        self.run_tran_checkpointed_with_startup_mode_and_abort(
            netlist,
            tstop,
            max_step,
            startup_mode,
            abort,
        )
    }

    /// Run a checkpointed transient using the startup mode of one explicitly
    /// selected `.TRAN` card.
    pub fn run_tran_checkpointed_with_startup_mode(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        self.run_tran_checkpointed_with_startup_mode_and_abort(
            netlist,
            tstop,
            max_step,
            startup_mode,
            &NoAbort,
        )
    }

    /// Cancellable checkpointed transient with an explicit startup contract.
    pub fn run_tran_checkpointed_with_startup_mode_and_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_transient_request_floor(tstop, max_step)?;
        match noise::expand_transient_noise(netlist, tstop).map_err(SimulationError::Circuit)? {
            Some(expanded) => engine
                .run_tran_resolved_with_resume(
                    &expanded,
                    netlist,
                    tstop,
                    max_step,
                    startup_mode,
                    abort,
                    None,
                    ResumeValidation::ExactNetlist,
                    FinalCheckpointRetention::Retained,
                    &[],
                )
                .and_then(Self::require_retained_final_checkpoint),
            None => engine
                .run_tran_resolved_with_resume(
                    netlist,
                    netlist,
                    tstop,
                    max_step,
                    startup_mode,
                    abort,
                    None,
                    ResumeValidation::ExactNetlist,
                    FinalCheckpointRetention::Retained,
                    &[],
                )
                .and_then(Self::require_retained_final_checkpoint),
        }
    }

    fn require_retained_final_checkpoint(
        (result, checkpoint, _): (
            TransientResult,
            Option<TransientCheckpoint>,
            Vec<ScheduledTransientCheckpoint>,
        ),
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        checkpoint.map_or_else(
            || {
                Err(SimulationError::Circuit(
                    "internal transient checkpoint retention contract was not fulfilled"
                        .to_string(),
                ))
            },
            |checkpoint| Ok((result, checkpoint)),
        )
    }

    /// Run one continuous transient and capture checkpoints at exact accepted
    /// solver points.
    ///
    /// The schedule does not add solver breakpoints. After each accepted step,
    /// the first due nominal time emits one checkpoint and all additional
    /// nominal times already due under Xyce's restart window are advanced
    /// without another file, matching Xyce 7.10. Keeping nominal filename time
    /// separate from actual accepted state time prevents checkpointing from
    /// perturbing the physical trajectory.
    pub fn run_tran_checkpoint_schedule_with_startup_mode(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        checkpoint_times: &[Value],
    ) -> Result<(TransientResult, Vec<ScheduledTransientCheckpoint>), SimulationError> {
        self.run_tran_checkpoint_schedule_with_startup_mode_and_abort(
            netlist,
            tstop,
            max_step,
            startup_mode,
            checkpoint_times,
            &NoAbort,
        )
    }

    /// Cancellable form of
    /// [`Engine::run_tran_checkpoint_schedule_with_startup_mode`].
    pub fn run_tran_checkpoint_schedule_with_startup_mode_and_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        checkpoint_times: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Vec<ScheduledTransientCheckpoint>), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        let mut previous = None;
        for (index, &time) in checkpoint_times.iter().enumerate() {
            if !time.is_finite() || time < 0.0 || time > tstop {
                return Err(SimulationError::Circuit(format!(
                    "scheduled checkpoint time {index} must be finite and within [0, {tstop:.17e}], found {time:.17e}"
                )));
            }
            if previous.is_some_and(|previous| time <= previous) {
                return Err(SimulationError::Circuit(format!(
                    "scheduled checkpoint times must be strictly increasing; found {time:.17e} after {:.17e}",
                    previous.expect("checked above")
                )));
            }
            previous = Some(time);
        }
        let retained_schedules = netlist
            .options
            .output_time_points
            .len()
            .saturating_add(
                netlist
                    .options
                    .output_interval_schedule
                    .as_ref()
                    .map_or(0, |schedule| schedule.intervals.len()),
            )
            .saturating_add(netlist.options.timeint_breakpoints.len())
            .saturating_add(
                netlist
                    .options
                    .restart
                    .as_ref()
                    .map_or(0, |restart| restart.intervals.len()),
            )
            .saturating_add(checkpoint_times.len());
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::AnalysisPoints,
            retained_schedules,
            self.config.resource_limits.max_analysis_points,
        )?;

        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_transient_request_floor(tstop, max_step)?;
        match noise::expand_transient_noise(netlist, tstop).map_err(SimulationError::Circuit)? {
            Some(expanded) => engine.run_tran_resolved_with_resume(
                &expanded,
                netlist,
                tstop,
                max_step,
                startup_mode,
                abort,
                None,
                ResumeValidation::ExactNetlist,
                FinalCheckpointRetention::Discarded,
                checkpoint_times,
            ),
            None => engine.run_tran_resolved_with_resume(
                netlist,
                netlist,
                tstop,
                max_step,
                startup_mode,
                abort,
                None,
                ResumeValidation::ExactNetlist,
                FinalCheckpointRetention::Discarded,
                checkpoint_times,
            ),
        }
        .map(|(result, _, checkpoints)| (result, checkpoints))
    }

    /// Continue a transient from a checkpoint to a later stop time.
    ///
    /// The checkpoint must come from the same netlist (fingerprint
    /// enforced). Continuation restores the captured linear-reactive state
    /// (capacitor/inductor integrator histories) and restarts integration at
    /// order one with absolute-time source evaluation. Higher-order integration
    /// resumes only after one real post-checkpoint interval has been accepted.
    /// Version-19 checkpoints restore accepted legacy-BJT/diode charge,
    /// limiter, terminal-current and engine-owned integration histories before
    /// that interval. Ordinary scalar lossless transmission-line histories are
    /// restored bit-exactly. Distributed
    /// LTRA/TXL and coupled-line runtimes fail closed until their complete
    /// convolution state has a versioned checkpoint contract.
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
        self.run_tran_resume_with_abort(netlist, checkpoint, tstop, max_step, &NoAbort)
    }

    /// Resume a checkpointed transient with cooperative cancellation.
    pub fn run_tran_resume_with_abort(
        &self,
        netlist: &Netlist,
        checkpoint: &TransientCheckpoint,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        self.run_tran_resume_validated(
            netlist,
            checkpoint,
            tstop,
            max_step,
            abort,
            ResumeValidation::ExactNetlist,
        )
    }

    /// Continue an authored Xyce-style restart deck from a checkpoint.
    ///
    /// This API is intentionally separate from [`Engine::run_tran_resume`].
    /// It permits only the differences inherent to a restart workflow: the
    /// `.TRAN` stop horizon and `.OPTIONS RESTART` file-management metadata.
    /// A collision-resistant semantic identity still binds circuit topology,
    /// values, sources, models, outputs, external dependency contents, and all
    /// trajectory-affecting transient controls. Checkpoints written before the
    /// restart identity was introduced fail closed.
    pub fn run_tran_restart_resume(
        &self,
        netlist: &Netlist,
        checkpoint: &TransientCheckpoint,
        tstop: Value,
        max_step: Value,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        self.run_tran_restart_resume_with_abort(netlist, checkpoint, tstop, max_step, &NoAbort)
    }

    /// Cancellable form of [`Engine::run_tran_restart_resume`].
    pub fn run_tran_restart_resume_with_abort(
        &self,
        netlist: &Netlist,
        checkpoint: &TransientCheckpoint,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        self.run_tran_resume_validated(
            netlist,
            checkpoint,
            tstop,
            max_step,
            abort,
            ResumeValidation::AuthoredRestart,
        )
    }

    fn run_tran_resume_validated(
        &self,
        netlist: &Netlist,
        checkpoint: &TransientCheckpoint,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
        validation: ResumeValidation,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        if !tstop.is_finite() || tstop <= checkpoint.time {
            return Err(SimulationError::Circuit(format!(
                "resume stop time {tstop:e} must exceed the checkpoint time {:e}",
                checkpoint.time
            )));
        }
        let startup_mode = checkpoint.startup_mode().ok_or_else(|| {
            SimulationError::Circuit(
                "legacy transient checkpoint does not record its startup mode; refusing to guess whether the captured trajectory used UIC"
                    .to_string(),
            )
        })?;

        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_transient_request_floor(tstop - checkpoint.time, max_step)?;
        match noise::expand_transient_noise(netlist, tstop).map_err(SimulationError::Circuit)? {
            Some(expanded) => engine
                .run_tran_resolved_with_resume(
                    &expanded,
                    netlist,
                    tstop,
                    max_step,
                    startup_mode,
                    abort,
                    Some(checkpoint),
                    validation,
                    FinalCheckpointRetention::Retained,
                    &[],
                )
                .and_then(Self::require_retained_final_checkpoint),
            None => engine
                .run_tran_resolved_with_resume(
                    netlist,
                    netlist,
                    tstop,
                    max_step,
                    startup_mode,
                    abort,
                    Some(checkpoint),
                    validation,
                    FinalCheckpointRetention::Retained,
                    &[],
                )
                .and_then(Self::require_retained_final_checkpoint),
        }
    }

    #[inline]
    fn should_enable_nonlinear_source_ramp_cap(
        circuit: &crate::circuit::CircuitData,
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

    #[inline]
    fn requires_conservative_ngspice_breakpoint_restart(
        circuit: &crate::circuit::CircuitData,
    ) -> bool {
        circuit.has_b3soi_devices()
            || !circuit.tlines.is_empty()
            || !circuit.coupled_tlines.is_empty()
            || Self::has_hfet_devices(circuit)
    }

    #[inline]
    fn has_hfet_devices(circuit: &crate::circuit::CircuitData) -> bool {
        circuit.jfets.iter().any(|device| {
            matches!(
                device.params.channel_model,
                crate::device::JfetChannelModel::Hfet1
            )
        })
    }

    #[inline]
    fn source_ramp_tracking_delta(
        circuit: &crate::circuit::CircuitData,
        configured: Value,
    ) -> Value {
        if Self::has_hfet_devices(circuit) && configured.is_finite() && configured > 0.0 {
            configured.min(HFET_SOURCE_RAMP_TRACKING_DELTA)
        } else {
            configured
        }
    }

    fn should_record_transient_device_op_traces(netlist: &Netlist) -> bool {
        netlist
            .saves
            .signals
            .iter()
            .any(|signal| matches!(signal, SaveSignal::DeviceParam { .. }))
            || netlist.output_requests.iter().any(|request| {
                request.dependencies.iter().any(|dependency| {
                    dependency.kind == OutputSymbolKind::Device
                        && is_device_lead_current_accessor(
                            &dependency.operator.to_ascii_uppercase(),
                        )
                })
            })
            || netlist.measurements.iter().any(|statement| {
                OutputAnalysisKind::from_keyword(&statement.analysis)
                    == Some(OutputAnalysisKind::Tran)
                    && measure_output_dependencies(statement)
                        .iter()
                        .any(|dependency| {
                            dependency.kind == OutputSymbolKind::Device
                                && is_device_lead_current_accessor(
                                    &dependency.operator.to_ascii_uppercase(),
                                )
                        })
            })
            || netlist.elements.iter().any(|element| {
                matches!(
                    &element.kind,
                    crate::netlist::ElementKind::Resistor {
                        instance_params,
                        model: Some(_),
                        ..
                    } if instance_params.iter().any(|(name, _)| {
                        name.eq_ignore_ascii_case("L") || name.eq_ignore_ascii_case("LENGTH")
                    }) && instance_params.iter().any(|(name, _)| {
                        name.eq_ignore_ascii_case("A") || name.eq_ignore_ascii_case("AREA")
                    })
                )
            })
            || netlist.elements.iter().any(|element| {
                matches!(
                    &element.kind,
                    crate::netlist::ElementKind::Capacitor {
                        value,
                        value_expr: Some(_),
                        ..
                    } if !value.is_finite()
                )
            })
            // Xyce nonlinear magnetic cores publish their hysteresis state
            // through the YMIN!KNAME device-op namespace.  Core state is
            // accepted-step dependent, so retain a trace whenever the
            // netlist contains a modeled Core coupling even when the deck
            // requests the probes through `.PRINT` rather than an explicit
            // `.SAVE @device[param]` card.  Multi-winding K-cards publish one
            // shared YMIN namespace entry for the whole group.
            || netlist.elements.iter().any(|element| {
                matches!(
                    &element.kind,
                    crate::netlist::ElementKind::Coupling {
                        inductors,
                        model: Some(_),
                        ..
                    } if !inductors.is_empty()
                )
            })
    }

    fn run_tran_with_abort_resolved(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, SimulationError> {
        self.run_tran_resolved_with_resume(
            netlist,
            netlist,
            tstop,
            max_step,
            startup_mode,
            abort,
            None,
            ResumeValidation::ExactNetlist,
            FinalCheckpointRetention::Discarded,
            &[],
        )
        .map(|(result, _, _)| result)
    }

    /// Engine/circuit state that is accepted-step mutable but not yet part of
    /// the exact integration-runtime wire contract. A time-zero checkpoint is
    /// exempt: no accepted interval has advanced these stores, so rebuilding
    /// the authenticated startup phase reconstructs their canonical state.
    fn exact_integration_runtime_resume_blockers(
        circuit: &crate::circuit::CircuitData,
        accepted_interval_count: usize,
    ) -> Vec<String> {
        if accepted_interval_count == 0 {
            return Vec::new();
        }

        fn block_if_present(blockers: &mut Vec<String>, present: bool, description: &'static str) {
            if present {
                blockers.push(description.to_string());
            }
        }

        let mut blockers = Vec::new();
        block_if_present(
            &mut blockers,
            !circuit.jfets.is_empty(),
            "JFET accepted transient integration history is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.mosfets.is_empty(),
            "classic MOSFET accepted transient integration history is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.vdmoses.is_empty(),
            "VDMOS accepted transient integration history is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            circuit.has_b3soi_devices(),
            "B3SOI DD/FD/PD accepted transient integration history is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            circuit.has_bsim3v3_devices(),
            "BSIM3v3 accepted transient integration history is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            circuit.has_bsim4v8_devices(),
            "BSIM4v8 accepted transient integration history is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.ekv26s.is_empty(),
            "EKV2.6 accepted transient integration history is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.ekv3s.is_empty(),
            "EKV3 accepted limiter and bypass state is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            circuit.capacitors.has_stateful_value_expressions(),
            "solution-dependent capacitor accepted SDT expression state is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            circuit.resistors.thermal.iter().any(Option::is_some),
            "thermal resistor accepted temperature state is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.coupled_inductor_pairs.is_empty(),
            "coupled-inductor accepted integration state is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.multi_winding_transformers.is_empty(),
            "multi-winding transformer accepted integration state is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.jiles_atherton_inductors.is_empty(),
            "Jiles-Atherton accepted magnetic state is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.xyce_core_groups.is_empty(),
            "Xyce nonlinear-core accepted magnetic state is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            circuit
                .behavioral_sources
                .voltage_sources
                .iter()
                .any(|source| source.program.sdt_count != 0)
                || circuit
                    .behavioral_sources
                    .current_sources
                    .iter()
                    .any(|source| source.program.sdt_count != 0),
            "behavioral-source accepted SDT state is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.vswitches.is_empty(),
            "voltage-controlled switch accepted hysteresis state is not checkpointed",
        );
        block_if_present(
            &mut blockers,
            !circuit.iswitches.is_empty(),
            "current-controlled switch accepted hysteresis state is not checkpointed",
        );
        blockers.extend(circuit.xspice_checkpoint_resume_blockers());
        blockers.sort_unstable();
        blockers.dedup();
        blockers
    }

    fn capture_scheduled_checkpoint_if_due(
        &self,
        scheduled_times: &[Value],
        cursor: &mut usize,
        accepted_time: Value,
        fingerprint: u64,
        netlist_identity: &Option<String>,
        restart_identity: &Option<String>,
        simulation_identity: &str,
        solution: &[Value],
        circuit: &crate::circuit::CircuitData,
        startup_mode: TransientStartupMode,
        integration_max_step: Value,
        integration_continuation: Option<ProposedIntegrationContinuation>,
        integration_stop_time: Value,
        pending_tline_arrivals: &[Value],
        dynamic_tline_breakpoints_added: usize,
        bjt_history: &BjtTransientHistory,
        diode_history: &DiodeTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        accepted_integration_runtime_capture: AcceptedIntegrationRuntimeCapture<'_>,
        retained_result_values: usize,
        retained_scheduled_checkpoint_values: &mut usize,
        captured: &mut Vec<ScheduledTransientCheckpoint>,
    ) -> Result<(), SimulationError> {
        self.ensure_result_values(
            retained_result_values.saturating_add(*retained_scheduled_checkpoint_values),
        )?;
        let Some(&requested_time) = scheduled_times.get(*cursor) else {
            return Ok(());
        };
        if !xyce_restart_schedule_is_due(accepted_time, requested_time) {
            return Ok(());
        }
        let at_integration_endpoint = accepted_time >= integration_stop_time
            || integration_stop_time - accepted_time
                <= crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE;
        let integration_continuation = if at_integration_endpoint {
            None
        } else {
            integration_continuation
        };
        let accepted_junction_history =
            Self::capture_accepted_junction_transient_history_checkpoint(
                circuit,
                bjt_history,
                diode_history,
                vbic_snapshot_cache,
            );
        let restart_normalized = at_integration_endpoint
            || integration_continuation
                .is_some_and(|continuation| continuation.xyce_breakpoint_restart_pending);
        let accepted_junction_history = if restart_normalized {
            if accepted_junction_history.resume_blockers.is_empty() {
                Self::normalize_accepted_junction_transient_history_checkpoint_for_order_one(
                    circuit,
                    &accepted_junction_history,
                    0.0,
                )
                .map_err(SimulationError::Circuit)?
            } else {
                accepted_junction_history
            }
        } else {
            accepted_junction_history
        };
        let lte_estimator = accepted_integration_runtime_capture.lte_estimator;
        let accepted_integration_runtime = if restart_normalized {
            AcceptedIntegrationRuntime::RestartNormalized(
                RestartNormalizedIntegrationRuntimeCheckpoint::capture(
                    accepted_time,
                    RestartNormalizedIntegrationRuntimeCapture {
                        lte_warmup_skips: accepted_integration_runtime_capture.lte_warmup_skips,
                        force_accept_cooldown: accepted_integration_runtime_capture
                            .force_accept_cooldown,
                        livelock_streak: accepted_integration_runtime_capture.livelock_streak,
                        livelock_last_restart_time: accepted_integration_runtime_capture
                            .livelock_last_restart_time,
                        accepted_interval_count: accepted_integration_runtime_capture
                            .accepted_interval_count,
                        damped_first_solver_call: accepted_integration_runtime_capture
                            .damped_first_solver_call,
                        damped_status: accepted_integration_runtime_capture.damped_status,
                        retry_count: accepted_integration_runtime_capture.retry_count,
                        xyce_step_failure_count: accepted_integration_runtime_capture
                            .xyce_step_failure_count,
                        stale_accept_count: accepted_integration_runtime_capture.stale_accept_count,
                        resume_blockers: &[],
                    },
                )
                .map_err(SimulationError::Circuit)?,
            )
        } else {
            AcceptedIntegrationRuntime::Exact(Box::new(
                AcceptedIntegrationRuntimeCheckpoint::capture(
                    solution,
                    accepted_time,
                    accepted_integration_runtime_capture,
                )
                .map_err(SimulationError::Circuit)?,
            ))
        };
        let checkpoint = TransientCheckpoint::capture_with_restart_identity(
            fingerprint,
            netlist_identity.clone(),
            restart_identity.clone(),
            simulation_identity.to_owned(),
            accepted_time,
            solution,
            circuit,
            startup_mode,
            Some(integration_max_step),
            integration_continuation,
            pending_tline_arrivals,
            dynamic_tline_breakpoints_added,
            accepted_junction_history,
            accepted_integration_runtime,
            Some(lte_estimator),
        )
        .map_err(SimulationError::Circuit)?;
        let retained_checkpoint_values = checkpoint.retained_value_count().saturating_add(1);
        self.ensure_result_values(
            retained_result_values
                .saturating_add(*retained_scheduled_checkpoint_values)
                .saturating_add(retained_checkpoint_values),
        )?;
        captured.push(ScheduledTransientCheckpoint {
            nominal_time: requested_time,
            checkpoint,
        });
        *retained_scheduled_checkpoint_values =
            retained_scheduled_checkpoint_values.saturating_add(retained_checkpoint_values);
        *cursor += 1;
        while scheduled_times
            .get(*cursor)
            .is_some_and(|time| xyce_restart_schedule_is_due(accepted_time, *time))
        {
            *cursor += 1;
        }
        Ok(())
    }

    /// The transient integration body. `resume` injects a checkpointed
    /// state (time, solution, reactive histories) instead of the fresh
    /// initial solution — numerically a breakpoint restart at the
    /// checkpoint time. The final checkpoint is captured only when the public
    /// caller retains it; scheduled checkpoint APIs retain their scheduled
    /// snapshots without materializing an otherwise discarded endpoint copy.
    fn run_tran_resolved_with_resume(
        &self,
        netlist: &Netlist,
        checkpoint_netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        abort: &dyn AbortSignal,
        resume: Option<&TransientCheckpoint>,
        resume_validation: ResumeValidation,
        final_checkpoint_retention: FinalCheckpointRetention,
        scheduled_checkpoint_times: &[Value],
    ) -> Result<
        (
            TransientResult,
            Option<TransientCheckpoint>,
            Vec<ScheduledTransientCheckpoint>,
        ),
        SimulationError,
    > {
        fft::preflight(self, netlist, tstop, abort)?;
        let trapezoidal_xmu = if self.config.spice_dialect == SpiceDialect::Xyce {
            0.5
        } else {
            netlist.options.xmu.unwrap_or(0.5)
        };
        let modified_trapezoidal_coefficients =
            CompanionCoefficients::trapezoidal_with_xmu(trapezoidal_xmu).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "XMU must be finite and within [0, 0.5], found {trapezoidal_xmu}"
                ))
            })?;
        let fingerprint = netlist_fingerprint(checkpoint_netlist);
        let netlist_identity = netlist_checkpoint_identity(checkpoint_netlist);
        let restart_identity = restart_checkpoint_identity(checkpoint_netlist);
        let simulation_identity = simulation_checkpoint_identity(&self.config);
        let mut scheduled_checkpoints = Vec::with_capacity(scheduled_checkpoint_times.len());
        let mut scheduled_checkpoint_cursor = 0_usize;
        let mut retained_scheduled_checkpoint_values = 0_usize;
        let resume_continuation = if let Some(checkpoint) = resume {
            match resume_validation {
                ResumeValidation::ExactNetlist => {
                    checkpoint.validate_for_with_config(checkpoint_netlist, &self.config)
                }
                ResumeValidation::AuthoredRestart => {
                    checkpoint.validate_for_restart_with_config(checkpoint_netlist, &self.config)
                }
            }
            .map_err(SimulationError::Circuit)?;
            checkpoint
                .validate_recorded_integration_max_step()
                .map_err(SimulationError::Circuit)?;
            checkpoint
                .validated_integration_continuation()
                .map_err(SimulationError::Circuit)?
        } else {
            None
        };
        let record_xspice_event_traces = netlist.options.xspice_event_trace_save.unwrap_or(true);
        let record_device_op_traces = Self::should_record_transient_device_op_traces(netlist);
        let mut circuit = self.build_circuit_with_abort(netlist, abort)?;
        if circuit.num_nodes() == 0 && circuit.num_branches() == 0 {
            let mut result = TransientResult {
                time: vec![0.0],
                step_sizes: vec![0.0],
                voltages: Vec::new(),
                branch_currents: Vec::new(),
                num_nodes: 0,
                node_names: Vec::new(),
                branch_names: Vec::new(),
                digital_traces: Vec::new(),
                real_traces: Vec::new(),
                device_op_traces: Vec::new(),
                store_traces: Vec::new(),
                fft_results: Vec::new(),
            };
            if scheduled_checkpoint_times.iter().any(|time| *time != 0.0) {
                return Err(SimulationError::Circuit(
                    "a topology-free transient cannot produce positive-time scheduled checkpoints"
                        .to_string(),
                ));
            }
            let final_checkpoint = if final_checkpoint_retention.is_retained()
                || !scheduled_checkpoint_times.is_empty()
            {
                let checkpoint = TransientCheckpoint::capture_with_restart_identity(
                    fingerprint,
                    netlist_identity,
                    restart_identity,
                    simulation_identity,
                    0.0,
                    &[],
                    &circuit,
                    startup_mode,
                    Some(max_step),
                    None,
                    &[],
                    0,
                    AcceptedJunctionTransientHistoryCheckpoint {
                        available: true,
                        ..AcceptedJunctionTransientHistoryCheckpoint::default()
                    },
                    AcceptedIntegrationRuntime::RestartNormalized(
                        RestartNormalizedIntegrationRuntimeCheckpoint::capture(
                            0.0,
                            RestartNormalizedIntegrationRuntimeCapture {
                                lte_warmup_skips: 0,
                                force_accept_cooldown: 0,
                                livelock_streak: 0,
                                livelock_last_restart_time: None,
                                accepted_interval_count: 0,
                                damped_first_solver_call: true,
                                damped_status: None,
                                retry_count: 0,
                                xyce_step_failure_count: 0,
                                stale_accept_count: 0,
                                resume_blockers: &[],
                            },
                        )
                        .map_err(SimulationError::Circuit)?,
                    ),
                    None,
                )
                .map_err(SimulationError::Circuit)?;
                let scheduled_checkpoint_values =
                    checkpoint.retained_value_count().saturating_add(1);
                let final_checkpoint = if final_checkpoint_retention.is_retained() {
                    scheduled_checkpoints.extend(scheduled_checkpoint_times.iter().map(
                        |&nominal_time| ScheduledTransientCheckpoint {
                            nominal_time,
                            checkpoint: checkpoint.clone(),
                        },
                    ));
                    Some(checkpoint)
                } else if let Some(&nominal_time) = scheduled_checkpoint_times.first() {
                    // A topology-free schedule can contain only its t=0
                    // checkpoint. Transfer the owned value into the returned
                    // schedule instead of briefly materializing an uncharged
                    // endpoint duplicate.
                    scheduled_checkpoints.push(ScheduledTransientCheckpoint {
                        nominal_time,
                        checkpoint,
                    });
                    None
                } else {
                    None
                };
                retained_scheduled_checkpoint_values =
                    scheduled_checkpoint_values.saturating_mul(scheduled_checkpoints.len());
                final_checkpoint
            } else {
                None
            };
            result.fft_results = fft::evaluate(self, netlist, &result, tstop, abort)?;
            self.ensure_result_values(
                Self::transient_result_value_count(&result)
                    .saturating_add(retained_scheduled_checkpoint_values)
                    .saturating_add(
                        final_checkpoint
                            .as_ref()
                            .map_or(0, TransientCheckpoint::retained_value_count),
                    ),
            )?;
            return Ok((result, final_checkpoint, scheduled_checkpoints));
        }
        Self::ensure_supported_transient_dynamic_charges(&circuit)?;
        let hinted_max_step = circuit
            .transient_max_step_hint
            .map_or(max_step, |hint| max_step.min(hint));
        let hinted_max_step = self
            .config
            .transient_timeint_max_timestep
            .filter(|bound| bound.is_finite() && *bound > 0.0)
            .map_or(hinted_max_step, |bound| hinted_max_step.min(bound));
        // Honor every finite explicitly configured maximum timestep (CLI
        // --max-step, product configuration, bindings). The core default is
        // positive infinity, so an authored value such as exactly 1 ms can
        // never collide with an "unset" sentinel.
        let config_max_step = self.config.max_timestep;
        let hinted_max_step = if config_max_step.is_finite() && config_max_step > 0.0 {
            hinted_max_step.min(config_max_step)
        } else {
            hinted_max_step
        };
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let source_step_hint = Self::transient_source_step_hint(netlist, hinted_max_step);
        circuit
            .voltage_sources
            .set_transient_context_with_dialect_and_limits(
                source_step_hint,
                tstop,
                self.config.spice_dialect,
                self.config.resource_limits,
            );
        circuit
            .current_sources
            .set_transient_context_with_dialect_and_limits(
                source_step_hint,
                tstop,
                self.config.spice_dialect,
                self.config.resource_limits,
            );
        circuit.set_xspice_transient_context(source_step_hint, tstop);

        // `.TRAN ... UIC` skips the operating point: integration starts
        // from zero everywhere except user-supplied .IC node voltages
        // (applied below) and per-element IC= values (applied after the
        // reactive-history seeding), matching ngspice's MODEUIC semantics.
        let uic_requested = startup_mode.is_uic();

        // Establish transient lifecycle state before the t=0 operating point.
        // UIC has no t=0 solve, so its first candidate carries the initial flag
        // below instead.
        #[cfg(feature = "veriloga")]
        if resume.is_none() {
            circuit
                .begin_veriloga_analysis(2)
                .map_err(SimulationError::Circuit)?;
        }
        #[cfg(feature = "veriloga")]
        circuit
            .prepare_veriloga_timepoint(
                0.0,
                0.0,
                &CompanionCoefficients::backward_euler(),
                resume.is_none() && !uic_requested,
                false,
            )
            .map_err(SimulationError::Circuit)?;
        #[cfg(feature = "veriloga-builtins-base")]
        circuit
            .generated_veriloga_devices_mut()
            .set_analysis_step(resume.is_none() && !uic_requested, false);

        // A resume rebuilds the circuit before the checkpoint's full engine
        // state can be injected. Prime accepted Verilog-A state now so the
        // intervening startup solve never evaluates an initial-step-derived
        // compact model against a zeroed procedural state. Full injection
        // below remains authoritative and restores any candidates touched by
        // this priming solve.
        if let Some(checkpoint) = resume {
            checkpoint
                .prime_veriloga_resume_startup(&mut circuit)
                .map_err(SimulationError::Circuit)?;
        }

        // Get the startup state. Ordinary transient validates the exact t=0
        // accepted equation contract below; its waveform can intentionally
        // differ from the source's separate DC value. UIC skips an operating
        // point altogether.
        let (mut solution, initial_solution_mode, accepted_transient_op) = if uic_requested {
            log::info!("Transient UIC startup: skipping the operating point");
            (
                vec![0.0; circuit.matrix_size()],
                startup::InitialSolutionMode::LinearizedSeed,
                None,
            )
        } else {
            self.solve_transient_initial_solution(netlist, &mut circuit, &mut matrix, abort)?
        };
        if let Some(contract) = accepted_transient_op {
            self.ensure_solved_transient_operating_point_paths_to_ground(
                &mut circuit,
                &mut matrix,
                &solution,
                0.0,
                contract,
            )?;
        }
        if let Some(message) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE evaluation failed: {message}"
            )));
        }

        // Resume: the standard initial-solution machinery above still ran
        // (its device-state priming is wanted), but time, solution, and the
        // reactive histories come from the checkpoint.
        let resume_time = resume.map_or(0.0, |checkpoint| checkpoint.time);
        if self.config.spice_dialect == SpiceDialect::Xyce {
            // A resumed source must observe the tolerance belonging to the
            // restored accepted state before branch-current capture or any
            // other source evaluation occurs.
            let breakpoint_tolerance = 2.0 * xyce_hard_min_timestep(resume_time);
            circuit
                .voltage_sources
                .set_xyce_breakpoint_tolerance(breakpoint_tolerance);
            circuit
                .current_sources
                .set_xyce_breakpoint_tolerance(breakpoint_tolerance);
        }
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
        // mid-trajectory, so they must not re-apply. Only UIC sets that state
        // by assignment: it skips the operating point, so this write is the
        // only thing that seeds the named nodes. An ordinary startup has
        // already solved them as clamps, and reasserting the authored value
        // there would overwrite the node an ideal source legitimately
        // outvoted -- ngspice reports the source's voltage on such a node.
        let applied_ic = if resume.is_none() && uic_requested {
            self.apply_initial_condition_overrides(netlist, &circuit, &mut solution)
        } else {
            0
        };
        // UIC: per-element IC= values shape the t=0 state itself. Writing
        // them into the solution here means the recorded first point, the
        // device priming below, and the reactive-history seeding all see
        // one consistent state (ngspice holds UIC capacitors at their IC
        // value at the first instant).
        if resume.is_none() && uic_requested {
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
        }
        // UIC skips the analog operating point, but it does not skip the
        // event-driven side's initialization. ngspice loads every device once
        // at TIME==0 even under MODEUIC, which is where a code model reads its
        // own initial-state parameters: `d_dff`'s `ic`, `d_fdiv`'s `i_count`,
        // a gate's settled output. Without this pass those branches never run,
        // so a flip-flop asked to start UNKNOWN starts LOW instead, and every
        // bridge treats the settled digital state as a transition it has to
        // ramp into from its own initial value.
        //
        // It runs after the `.IC`/element-IC writes above so bridges see the
        // same analog node voltages the first accepted point reports.
        if resume.is_none() && uic_requested && circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_analysis(
                0.0,
                0.0,
                &solution,
                crate::xspice::AnalysisType::Transient,
            );
            if let Some(message) = circuit.take_xspice_evaluation_error() {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE evaluation failed: {message}"
                )));
            }
        }

        // Xyce's implicit transient policy uses the classic diode's native
        // `origFlag` status (limiter activity only) for ENFORCEDEVICECONV.
        // Keep the DC/startup solve strict, and preserve strict behavior when
        // the deck or caller explicitly supplies ENFORCEDEVICECONV.
        let native_xyce_transient_convergence = self.config.spice_dialect == SpiceDialect::Xyce
            && self.config.transient_enforce_device_convergence.is_none();
        circuit
            .diodes
            .set_native_xyce_transient_convergence(native_xyce_transient_convergence);
        let startup_voltage_hints_active = resume.is_none()
            && !self
                .collect_node_voltage_hints(netlist, &circuit)
                .is_empty();
        // Xyce applies its GMIN continuation only while solving the DC
        // operating point.  It does not carry the final continuation shunt
        // into the transient DAE; doing so changes even an ideal resistor's
        // KCL by a low-bit amount and can materially alter hysteretic devices.
        // Other dialects retain their numerical floor even when `.IC` supplies
        // startup hints; hints do not regularize event-only or floating rows.
        let transient_baseline_diag_gmin =
            self.transient_nodal_gmin_floor(&circuit, startup_voltage_hints_active);
        if circuit.has_nonlinear_devices() {
            self.update_transient_nonlinear_devices(&mut circuit, &solution)?;
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
            && circuit.xyce_memristors.is_empty()
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
        // Xyce's DampedNewton path delegates globalization to each device's
        // native limiter.  A global nodal trust region changes the accepted
        // Newton orbit after a retry (and is not part of Xyce's transient
        // algorithm), so keep this recovery guard on the ngspice/native path.
        let requires_conservative_nonlinear_limiting = self.config.spice_dialect
            != SpiceDialect::Xyce
            && circuit.has_physical_nonlinear_devices();
        let nonlinear_source_ramp_cap_enabled = self.config.spice_dialect != SpiceDialect::Xyce
            && Self::should_enable_nonlinear_source_ramp_cap(
                &circuit,
                requires_conservative_nonlinear_limiting,
            );
        let enforce_device_convergence = self.transient_enforce_device_convergence();
        let xyce_nox_requested = self.config.spice_dialect == SpiceDialect::Xyce
            && self.config.transient_nonlinear_nox.unwrap_or(false);
        let has_shared_xyce_core_level1_ill_conditioned =
            circuit.has_xyce_core_shared_level1_ill_conditioned();
        let has_shared_xyce_core_level2 = circuit.has_xyce_core_shared_level2();
        let uses_xyce_damped_solver = self.config.spice_dialect == SpiceDialect::Xyce
            && !xyce_nox_requested
            // MutIndNonLin2's shared LEVEL=2 residual is a coupled charge
            // system; its correction-form Jacobian must be solved together
            // with the winding branches rather than accepted by the scalar
            // DampedNewton status test.
            && !has_shared_xyce_core_level2
            // A very small shared LEVEL=1 vacuum coefficient makes the
            // electrical Schur complement effectively rank deficient. Keep
            // that topology on correction-form Newton while retaining
            // DampedNewton for well-conditioned shared cores.
            && !has_shared_xyce_core_level1_ill_conditioned;
        // The direct physical DAE loader is narrower than the DampedNewton
        // solver itself.  Keep it behind the exact Xyce dialect and solver
        // gates so NOX/rescue paths never mix matrix-reconstructed and direct
        // residual contracts.
        let uses_direct_xyce_dae = self.config.spice_dialect == SpiceDialect::Xyce
            && uses_xyce_damped_solver
            && circuit.supports_direct_xyce_level2_core_dae();
        // MutIndNonLin's hidden M/R equations are part of the physical DAE,
        // not an optional convergence hint.  A failed constitutive trial must
        // reject the Newton candidate even when Xyce's general device
        // convergence status test is disabled.
        let core_trial_converged = |circuit: &crate::circuit::CircuitData| {
            !circuit.has_xyce_core_inductors()
                || (!circuit.xyce_core_trial_invalid()
                    // MutIndNonLin2's forward-Euler magnetization limiter is
                    // reported through the device's native `origFlag`.  Xyce
                    // consults that flag when ENFORCEDEVICECONV is enabled;
                    // its runtime transient default is enabled even though
                    // the option metadata advertises zero.  The resolved
                    // policy therefore governs both the hidden LEVEL=1 rows
                    // and MutIndNonLin2's limiter here.
                    && (!enforce_device_convergence
                        || !circuit.has_xyce_core_level2()
                        || circuit.xyce_core_level2_trial_converged())
                    && (!enforce_device_convergence || circuit.xyce_core_trial_converged()))
        };
        let enforce_force_candidate_safety =
            requires_conservative_nonlinear_limiting || circuit.has_xspice_devices();
        let is_strictly_linear_transient = (!circuit.has_nonlinear_devices()
            && !circuit.has_xyce_core_inductors())
            || circuit.has_only_memoryless_linear_xspice_nonlinearity();
        let uses_inductor_correction = !circuit.inductors.is_empty()
            || !circuit.coupled_inductor_pairs.is_empty()
            || !circuit.multi_winding_transformers.is_empty();
        let uses_inductor_correction = uses_inductor_correction
            && (!circuit.has_only_xyce_core_inductors() || has_shared_xyce_core_level2);
        let uses_inductor_correction = uses_inductor_correction || uses_direct_xyce_dae;
        // ngspice's flat transient Newton: when junction devices replace
        // their own iterate voltages (legacy GP pnjlim in update), the full
        // node step is the algorithm; per-iteration node-delta clamps walk
        // the junction against frozen nodes and livelock turn-on edges.
        let junction_owns_steps = Self::junction_limiting_owns_newton_steps(&circuit);
        let prefer_dense_solver = Self::should_prefer_dense_transient_solver(
            is_strictly_linear_transient,
            size,
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
        let mut breakpoints = if self.config.spice_dialect == SpiceDialect::Xyce {
            BreakpointManager::new_with_tolerance_and_policy(
                crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE,
                BreakpointStepPolicy::Xyce,
            )
        } else {
            BreakpointManager::new_with_tolerance(Self::ngspice_breakpoint_tolerance(
                hinted_max_step,
            ))
        };
        if self.config.spice_dialect != SpiceDialect::Xyce
            && Self::requires_conservative_ngspice_breakpoint_restart(&circuit)
        {
            // Native B3SOI, HFET, and delay-line histories still need the
            // established smaller warmup step to preserve transient accuracy.
            // Keep this capability guard independent of deck identity and let
            // ordinary ngspice-style circuits use the source-compatible scale.
            breakpoints.use_conservative_restart_step();
        }
        Self::collect_transient_source_breakpoints(
            &circuit,
            tstop,
            source_step_hint,
            self.config.spice_dialect,
            &mut breakpoints,
            abort,
            self.config.resource_limits.max_analysis_points,
        )?;
        Self::add_breakpoint_if_in_range(&mut breakpoints, tstop, tstop);
        let source_breakpoint_times = breakpoints.times().to_vec();
        Self::collect_transient_tline_breakpoints(
            &circuit,
            &source_breakpoint_times,
            tstop,
            &mut breakpoints,
        );
        // User and output schedules are solver stops, not physical source
        // edges. Add them only after transmission-line propagation so they
        // cannot synthesize delayed arrivals.
        Self::add_user_transient_breakpoints(
            &mut breakpoints,
            netlist,
            tstop,
            abort,
            self.config.resource_limits.max_analysis_points,
        )?;
        Self::add_breakpoint_if_in_range(&mut breakpoints, tstop, tstop);
        let mut pending_dynamic_tline_breakpoints = resume
            .map(|checkpoint| checkpoint.pending_tline_arrivals().to_vec())
            .unwrap_or_default();
        if let Some(checkpoint) = resume {
            for &arrival in checkpoint.pending_tline_arrivals() {
                if arrival <= tstop {
                    breakpoints.add(arrival);
                }
            }
        }
        breakpoints.discard_through(resume_time);
        let configured_initial_step = self
            .config
            .transient_initial_timestep
            .filter(|step| step.is_finite() && *step > 0.0);
        let fresh_initial_step = if self.config.spice_dialect == SpiceDialect::Xyce {
            Self::xyce_initial_timestep(
                resume_time,
                tstop,
                configured_initial_step.or(tran_step_hint),
                hinted_max_step,
                breakpoints.next_after(resume_time),
            )
        } else {
            configured_initial_step
                .map(|step| step.max(1e-30))
                .unwrap_or_else(|| {
                    Self::ngspice_t0_breakpoint_limited_initial_timestep(
                        Self::ngspice_initial_timestep(tstop, tran_step_hint, hinted_max_step),
                        breakpoints.next_after(resume_time),
                    )
                })
        };
        let initial_step = resume_continuation
            .map(|continuation| continuation.next_step)
            .unwrap_or(fresh_initial_step);
        let practical_min = Self::startup_practical_min_timestep(
            has_bjts,
            hinted_max_step,
            min_div,
            tran_step_hint,
        );
        let preferred_min_dt = practical_min.max(self.config.min_timestep.max(1e-15));
        let hard_min_dt = if self.config.spice_dialect == SpiceDialect::Xyce {
            xyce_hard_min_timestep(resume_time)
        } else {
            Self::ngspice_hard_min_timestep(hinted_max_step, preferred_min_dt)
        };
        let mut xyce_breakpoint_span_ceiling = XyceBreakpointSpanCeiling::new(
            self.config
                .effective_transient_min_steps_between_breakpoints(),
        );
        let startup_span_ceiling = if let Some(continuation) = resume_continuation {
            xyce_breakpoint_span_ceiling
                .restore_active_ceiling(continuation.breakpoint_span_ceiling)
                .map_err(SimulationError::Circuit)?
        } else {
            xyce_breakpoint_span_ceiling.anchor(
                resume_time,
                breakpoints.next_after(resume_time),
                tstop,
            )
        };
        let startup_raw_max_dt = if self.config.spice_dialect == SpiceDialect::Xyce {
            self.transient_device_max_timestep(&circuit, resume_time, hinted_max_step)
                .min(startup_span_ceiling.unwrap_or(Value::INFINITY))
        } else {
            configured_initial_step
                .map(|step| hinted_max_step.max(step))
                .unwrap_or(hinted_max_step)
        };
        // Restore the producing controller's effective maximum before applying
        // this resumed segment's raw device/span cap. Runtime `set_max_dt`
        // raises a raw cap below the already-established hard floor; reversing
        // that ordering would instead lower the floor at an arbitrary seam.
        // Applying the current raw cap second also preserves Xyce's contract
        // that an extended restart may select its own per-run maximum step.
        let startup_controller_max_dt = resume_continuation
            .map(|continuation| continuation.controller_max_step)
            .unwrap_or(startup_raw_max_dt);
        let mut timestep = TimestepController::new_with_preferred_min(
            initial_step,
            hard_min_dt,
            preferred_min_dt,
            startup_controller_max_dt,
        );
        if resume_continuation.is_some() {
            timestep.set_max_dt(startup_raw_max_dt);
        }
        // Accepted generated timer state owns an exact absolute event target.
        // It is reconstructed from t=0/checkpoint state below and replaced
        // only after another atomic Verilog-A acceptance.
        let mut pending_veriloga_event_time: Option<Value> = None;
        let mut dynamic_tline_breakpoints_added = resume
            .map(TransientCheckpoint::dynamic_tline_breakpoints_added)
            .unwrap_or(0);
        let mut warned_dynamic_tline_breakpoint_cap = false;
        let transient_lte_reltol = self.transient_lte_reltol();
        let transient_lte_abstol = self.transient_lte_abstol();
        let mut lte_estimator = LteEstimator::with_tolerances_and_reference(
            transient_lte_reltol,
            transient_lte_abstol,
            self.config
                .transient_lte_reference
                .unwrap_or_else(|| self.config.spice_dialect.default_transient_lte_reference()),
        );
        if lte_estimator.uses_accepted_solution_reference() {
            lte_estimator.seed_initial_solution(&solution[..size.min(solution.len())]);
        }
        if let Some(checkpoint) = resume {
            checkpoint
                .restore_lte_references(&mut lte_estimator)
                .map_err(SimulationError::Circuit)?;
        }

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
        let mut accepted_interval_count = 0_usize;
        // Xyce's global first-step phase and its first interval after a
        // breakpoint are separate controller states. A resumed result segment
        // always starts with one stored point, so its local length cannot
        // reconstruct either state.
        let mut analysis_first_step_pending = resume_continuation.map_or_else(
            || resume.is_none() || resume_time.to_bits() == 0.0_f64.to_bits(),
            |continuation| continuation.analysis_first_step_pending,
        );
        let mut xyce_lte_restart_first_step = resume_continuation.map_or_else(
            || {
                resume.is_some()
                    && resume_time.to_bits() != 0.0_f64.to_bits()
                    && lte_estimator.uses_accepted_solution_reference()
            },
            |continuation| continuation.xyce_breakpoint_restart_pending,
        );

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
        } else if xyce_lte_restart_first_step {
            // Xyce reinitializes OneStep from the accepted breakpoint
            // solution. The hybrid detector is RSpice policy layered above
            // that integrator, so it must discard pre-discontinuity evidence
            // and use the same accepted solution as its derivative origin.
            trapgear.restart_from(&solution);
        }
        // Track integration method order for LTE scaling
        let effective_method_order = |method: IntegrationMethod, trap_order: u8| -> u32 {
            match method {
                IntegrationMethod::BackwardEuler => 1,
                IntegrationMethod::Trapezoidal
                | IntegrationMethod::TrapGear
                | IntegrationMethod::Gear2
                    if trap_order <= 1 =>
                {
                    1
                }
                _ => 2, // Trapezoidal and Gear2 are both order 2
            }
        };
        let current_integration_method = |tg: &TrapGearController| -> IntegrationMethod {
            fixed_method.unwrap_or_else(|| tg.current_method())
        };
        let native_predictor_local = !lte_estimator.uses_accepted_solution_reference();
        let xyce_iteration_error_control = self.config.spice_dialect == SpiceDialect::Xyce
            && self.config.transient_error_control == TransientErrorControl::NonlinearIterations;
        let xyce_min_order = self.config.transient_timeint_min_order;
        let xyce_max_order = self.config.transient_timeint_max_order;
        let native_order_after_restart = |method: IntegrationMethod| -> u8 {
            if !native_predictor_local {
                xyce_startup_or_restart_order(xyce_min_order)
            } else if method == IntegrationMethod::Gear2 {
                2
            } else {
                1
            }
        };

        // Initialize result storage with actual MNA node names. Native Xyce
        // memristor resistance remains a typed store trace and never enters
        // this voltage namespace.
        let node_names = circuit.node_names_sorted();

        log::debug!("Transient node mapping contains {} nodes", node_names.len());
        if log::log_enabled!(log::Level::Trace) {
            for (i, name) in node_names.iter().enumerate() {
                let dc_v = solution.get(i).copied().unwrap_or(0.0);
                log::trace!("Transient node[{i}] = '{name}', V_dc = {dc_v:.4}");
            }
        }
        if applied_ic > 0 {
            log::info!(
                "Applied {} .IC node override(s) to transient initial state",
                applied_ic
            );
        }
        if resume.is_none() {
            // Xyce seeds both accepted store-vector history levels from the
            // operating-point trial before the first real transient timepoint.
            circuit.initialize_generic_switch_transient_history();
            circuit
                .initialize_xyce_team_resistance_noise(resume_time)
                .map_err(SimulationError::Circuit)?;
        }
        let mut store_traces = Vec::with_capacity(circuit.xyce_memristors.len());
        for binding in &mut circuit.xyce_memristors {
            if let Some(resistance) = Self::xyce_memristor_resistance_output(binding, &solution)? {
                binding.resistance_store = resistance;
            }
            store_traces.push(crate::engine::TransientStoreTrace {
                name: format!("{}:R", binding.name),
                values: vec![binding.resistance_store],
            });
        }

        let mut branch_names = circuit.branch_names_sorted();
        let derived_branch_currents =
            Self::derived_transient_branch_currents(netlist, &circuit, &branch_names);
        branch_names.extend(
            derived_branch_currents
                .iter()
                .map(|&branch| Self::derived_transient_branch_name(&circuit, branch)),
        );
        let retain_xyce_voltage_source_currents = self.config.spice_dialect == SpiceDialect::Xyce
            && netlist
                .output_requests
                .iter()
                .any(|request| request.requires_transient_device_current_operand());
        let has_xyce_complete_voltage_wildcard = netlist.params.expression_dialect()
            == crate::config::ExpressionDialect::Xyce
            && netlist
                .output_requests
                .iter()
                .any(crate::netlist::OutputRequest::has_complete_transient_voltage_wildcard);
        let has_complete_save_voltage_wildcard = netlist
            .output_requests
            .iter()
            .any(crate::netlist::OutputRequest::has_complete_transient_save_voltage_wildcard);
        let external_wildcard_nodes = if has_xyce_complete_voltage_wildcard
            || has_complete_save_voltage_wildcard
        {
            Some(
                crate::netlist::collect_output_node_namespace_with_limits_and_abort(
                    netlist,
                    self.config.resource_limits,
                    abort,
                )
                .map_err(|error| match error {
                    crate::netlist::ParseWithAbortError::Aborted => SimulationError::Aborted,
                    crate::netlist::ParseWithAbortError::Parse(error) => SimulationError::Netlist(
                        format!("V(*) external-node namespace could not be elaborated: {error}"),
                    ),
                })?
                .external,
            )
        } else {
            None
        };
        let capture_plan = TransientCapturePlan::compile(
            netlist,
            &node_names,
            &branch_names,
            retain_xyce_voltage_source_currents,
            &circuit.voltage_sources.names,
            external_wildcard_nodes.as_ref(),
        );
        let trace_capacity = self.transient_initial_trace_capacity(
            (tstop - resume_time).max(0.0),
            hinted_max_step,
            capture_plan
                .analog_values_per_sample()
                .saturating_add(store_traces.len())
                .saturating_add(1),
        );
        for trace in &mut store_traces {
            trace
                .values
                .reserve(trace_capacity.saturating_sub(trace.values.len()));
        }
        if resume.is_none() {
            circuit
                .behavioral_sources
                .accept_transient_step(&solution, resume_time)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        }
        let mut time = Vec::with_capacity(trace_capacity);
        time.push(resume_time);
        let mut step_sizes = Vec::with_capacity(trace_capacity);
        step_sizes.push(0.0);
        let mut branch_currents = Self::initial_transient_branch_currents(
            &mut circuit,
            &solution,
            num_nodes,
            resume_time,
            uic_requested,
            &derived_branch_currents,
        )?;
        // The operating-point and UIC startup paths both need one explicit,
        // final t=0 Verilog-A pass against the solution that will actually be
        // reported. Commit it exactly once so sampled/event operators advance
        // their origin-time state before the first positive timestep.
        if resume.is_none() && circuit.has_any_veriloga_devices() {
            let origin_state = circuit.nonlinear_state_snapshot();
            let origin_result = (|| -> Result<(), SimulationError> {
                #[cfg(feature = "veriloga")]
                if circuit.has_veriloga_devices() {
                    circuit
                        .evaluate_veriloga_timepoint(&solution)
                        .map_err(SimulationError::Circuit)?;
                }
                #[cfg(feature = "veriloga-builtins-base")]
                if circuit.has_generated_veriloga_devices() {
                    circuit
                        .evaluate_generated_veriloga_timepoint(&mut matrix, &solution)
                        .map_err(SimulationError::Circuit)?;
                }
                circuit
                    .accept_all_veriloga_timestep()
                    .map_err(SimulationError::Circuit)?;
                pending_veriloga_event_time =
                    accepted_veriloga_event_time(&circuit, resume_time, timestep.hard_min_dt())?;
                #[cfg(feature = "veriloga")]
                if circuit.has_veriloga_devices()
                    && let Some(bound) = circuit
                        .veriloga_timestep_bound()
                        .map_err(SimulationError::Circuit)?
                    && bound.max(timestep.hard_min_dt()) < timestep.dt()
                {
                    timestep.force_step(bound.max(timestep.hard_min_dt()).min(max_step));
                }
                Ok(())
            })();
            if let Err(error) = origin_result {
                circuit.restore_nonlinear_state(origin_state);
                return Err(error);
            }
        }
        for (trace, &retain) in branch_currents
            .iter_mut()
            .zip(&capture_plan.branch_currents)
        {
            if retain {
                trace.reserve(trace_capacity.saturating_sub(trace.len()));
            } else {
                trace.clear();
            }
        }
        let mut result = TransientResult {
            time,
            step_sizes,
            voltages: (0..num_nodes)
                .map(|i| {
                    Self::seeded_transient_trace(
                        solution.get(i).copied().unwrap_or(0.0),
                        capture_plan.voltages[i],
                        trace_capacity,
                    )
                })
                .collect(),
            branch_currents,
            num_nodes,
            node_names,
            branch_names,
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces,
            fft_results: Vec::new(),
        };
        if record_device_op_traces {
            result.record_device_op_sample(
                circuit
                    .initial_transient_device_op_report(&solution)
                    .map_err(SimulationError::Circuit)?,
            );
        }
        let mut digital_snapshot = Vec::new();
        let mut real_snapshot = Vec::new();
        let mut digital_trace_indices = HashMap::new();
        let mut real_trace_indices = HashMap::new();
        if record_xspice_event_traces {
            circuit.fill_xspice_digital_snapshot(&mut digital_snapshot);
            result.record_digital_snapshot(
                resume_time,
                &digital_snapshot,
                &mut digital_trace_indices,
                &capture_plan.event_nodes,
            );
            circuit.fill_xspice_real_snapshot(&mut real_snapshot);
            result.record_real_snapshot(
                resume_time,
                &real_snapshot,
                &mut real_trace_indices,
                &capture_plan.event_nodes,
            );
        }
        let mut retained_result_values = Self::transient_result_value_count(&result);
        self.ensure_transient_result_limits(&result, retained_result_values)?;
        abort.observe_transient_sample(result.observable_sample());
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
        for node in circuit.xspice_ideal_voltage_constraint_nodes() {
            mark_voltage_lte_excluded(&mut solution_lte_excluded, node);
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
        let mut xyce_lte_excluded_indices = Vec::new();
        // Capacitor lead-current branches are algebraic currents, not
        // integration states, and must never drive LTE rejection or
        // accepted-reference timestep control.
        for branch_ordinal in circuit
            .capacitors
            .ic_branch_indices
            .iter()
            .flatten()
            .copied()
        {
            let matrix_index = num_nodes + branch_ordinal - 1;
            xyce_lte_excluded_indices.push(matrix_index);
            if let Some(excluded) = solution_lte_excluded.get_mut(matrix_index) {
                *excluded = true;
            }
        }
        for binding in &circuit.coupled_inductor_pairs {
            for branch in [binding.branch1_ordinal, binding.branch2_ordinal] {
                if branch > 0 {
                    xyce_lte_excluded_indices.push(num_nodes + branch - 1);
                }
            }
        }
        for binding in &circuit.multi_winding_transformers {
            xyce_lte_excluded_indices.extend(
                binding
                    .branch_ordinals
                    .iter()
                    .filter(|branch| **branch > 0)
                    .map(|branch| num_nodes + *branch - 1),
            );
        }
        xyce_lte_excluded_indices.sort_unstable();
        xyce_lte_excluded_indices.dedup();
        let nonlinear_terminal_solution_indices =
            Self::nonlinear_terminal_solution_indices(&circuit, &solution_lte_excluded);

        // Grid-locked stepping: the configured grid (filtered to points after
        // the start) is the mandatory target-sample sequence, and the run
        // ends at its last point.  The solver may accept bounded internal
        // points between targets when the requested interval exceeds the
        // resolved maximum step; those points preserve history-coupled device
        // dynamics while the target itself is still landed on exactly.  The
        // engine records both target and internal points so consumers can
        // interpolate the requested grid. Source-activity biasing, LTE
        // rejection, and every timestep-controller proposal are bypassed while
        // locked; Newton (with its dt-preserving rescue) is the sole acceptance
        // authority. Accepted-reference modes still restart integration
        // history at source breakpoints and compute LTE for Xyce's order-
        // selection trial; the estimate cannot reject a prescribed target.
        let (locked_grid, locked_step_sizes): (Option<Vec<Value>>, Option<Vec<Value>>) = self
            .config
            .locked_time_grid
            .as_ref()
            .map(|grid| {
                let (grid, steps) = Self::normalized_locked_time_schedule(
                    grid,
                    self.config
                        .locked_time_step_sizes
                        .as_deref()
                        .map(|steps| steps.as_slice()),
                    t,
                );
                (Some(grid), steps)
            })
            .unwrap_or((None, None));
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
            log::trace!(
                "Capacitor {} init: v_dc={:.4}, np={}, nn={}",
                circuit.capacitors.names[cap_idx],
                v_dc,
                np,
                nn
            );
        }
        circuit
            .capacitors
            .initialize_solution_dependent_from_dc(&solution, 0.0);
        if record_device_op_traces {
            // The initial report is created before capacitor histories are
            // initialized. Refresh its sample so solution-dependent C probes
            // expose the DC-evaluated capacitance at t=0.
            retained_result_values = retained_result_values.saturating_add(
                result.record_device_op_sample(
                    circuit
                        .initial_transient_device_op_report(&solution)
                        .map_err(SimulationError::Circuit)?,
                ),
            );
            self.ensure_transient_result_limits(&result, retained_result_values)?;
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
                circuit.inductors.i_prev_prev_prev[l_idx] = i_dc;
            }
        }
        circuit.update_coupled_inductor_pair_state(&solution);
        circuit.update_multi_winding_transformer_state(&solution);

        // ngspice keeps `CKTgmin` live in every analysis mode. Establish the
        // production transient floor after DC startup but before checkpoint
        // injection and BJT/diode history construction. The setter can
        // invalidate nonlinear caches when the value changes, so injection
        // must remain the last writer of restored accepted device state.
        circuit.set_semiconductor_junction_gmin(
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target),
        );
        // The accepted transient boundary is never a DC operating-point
        // evaluation phase. The first transient stamp would clear these
        // modes anyway; establish that canonical phase before a possible t=0
        // checkpoint so resume and uninterrupted execution start identically.
        circuit.set_b3soi_operating_point_mode(false);
        circuit.set_xyce_memristor_operating_point_mode(false);

        // Seed transient line bookkeeping before checkpoint injection. A
        // supported scalar lossless checkpoint replaces the seed with its
        // complete accepted delay history; unsupported distributed/coupled
        // line state fails closed during injection instead of starting a new
        // wave trajectory at the seam.
        let tline_dc_refs = Self::initialize_tline_history(&mut circuit, &solution, resume_time);
        let coupled_tline_refs =
            Self::initialize_coupled_tline_history(&mut circuit, &solution, resume_time);

        // Decode every engine-owned junction history/cache into temporary
        // runtime pieces before checkpoint injection mutates the circuit. A
        // legacy checkpoint is admitted here only when the live elaboration
        // has no BJT or diode topology.
        let restored_accepted_junction_history = resume
            .map(|checkpoint| checkpoint.restore_accepted_junction_transient_history(&circuit))
            .transpose()
            .map_err(SimulationError::Circuit)?;
        let exact_runtime_resume_blockers = resume
            .filter(|checkpoint| {
                matches!(
                    checkpoint.accepted_integration_runtime(),
                    AcceptedIntegrationRuntime::Exact(_)
                )
            })
            .map_or_else(Vec::new, |_| {
                Self::exact_integration_runtime_resume_blockers(
                    &circuit,
                    usize::from(resume_time > 0.0),
                )
            });
        let (uses_xyce_static_residual, has_direct_dae_static_residual) = resume
            .and_then(
                |checkpoint| match checkpoint.accepted_integration_runtime() {
                    AcceptedIntegrationRuntime::Exact(runtime) => Some((
                        runtime.xyce_static_residual.is_some(),
                        runtime.direct_dae_static_residual.is_some(),
                    )),
                    AcceptedIntegrationRuntime::UnavailableLegacy
                    | AcceptedIntegrationRuntime::RestartNormalized(_) => None,
                },
            )
            .unwrap_or((false, false));
        let restored_accepted_integration_runtime = resume
            .map(|checkpoint| {
                checkpoint.validated_accepted_integration_runtime(
                    AcceptedIntegrationRuntimeTarget {
                        lte_estimator: &lte_estimator,
                        uses_trapgear: fixed_method.is_none(),
                        uses_xyce_static_residual,
                        uses_direct_dae: uses_direct_xyce_dae,
                        has_direct_dae_static_residual,
                        uses_damped_newton: uses_xyce_damped_solver,
                        expected_resume_blockers: &exact_runtime_resume_blockers,
                    },
                )
            })
            .transpose()
            .map_err(SimulationError::Circuit)?;
        let resume_is_restart_normalized = matches!(
            restored_accepted_integration_runtime,
            Some(ValidatedAcceptedIntegrationRuntime::RestartNormalized(_))
        );
        if resume_is_restart_normalized {
            xyce_lte_restart_first_step = true;
            if fixed_method.is_none() {
                trapgear.restart_from(&solution);
            }
        }

        // Resume: replace the flat (DC-style) reactive and transmission-line
        // histories written above with the exact checkpointed state.
        if let Some(checkpoint) = resume {
            checkpoint
                .inject(&mut circuit)
                .map_err(SimulationError::Circuit)?;
            pending_veriloga_event_time =
                accepted_veriloga_event_time(&circuit, resume_time, timestep.hard_min_dt())?;
            #[cfg(feature = "veriloga")]
            if circuit.has_veriloga_devices()
                && let Some(bound) = circuit
                    .veriloga_timestep_bound()
                    .map_err(SimulationError::Circuit)?
                && bound.max(timestep.hard_min_dt()) < timestep.dt()
            {
                timestep.force_step(bound.max(timestep.hard_min_dt()).min(max_step));
            }
            // The first resumed sample represents the accepted checkpoint
            // state. A PEM store can deliberately retain its last finite
            // resistance while the instantaneous conductance is zero, so its
            // checkpointed value must replace the freshly built binding's
            // default before the seam sample is exposed.
            for (trace, binding) in result.store_traces.iter_mut().zip(&circuit.xyce_memristors) {
                if let Some(first) = trace.values.first_mut() {
                    *first = binding.resistance_store;
                }
            }
            for (derived_index, &branch) in derived_branch_currents.iter().enumerate() {
                if !matches!(
                    branch.kind,
                    DerivedTransientBranchCurrentKind::XyceMemristor
                ) {
                    continue;
                }
                let current = Self::derived_transient_branch_current(
                    &mut circuit,
                    &solution,
                    resume_time,
                    None,
                    branch,
                )?;
                if let Some(first) = result
                    .branch_currents
                    .get_mut(circuit.num_branches() + derived_index)
                    .and_then(|waveform| waveform.first_mut())
                {
                    *first = current;
                }
            }
            // The checkpointed companion history is authoritative for the
            // accepted capacitor lead current across an integration restart.
            for (capacitor, branch_ordinal) in circuit
                .capacitors
                .ic_branch_indices
                .iter()
                .copied()
                .enumerate()
            {
                let Some(branch_ordinal) = branch_ordinal else {
                    continue;
                };
                if let Some(current) = result
                    .branch_currents
                    .get_mut(branch_ordinal - 1)
                    .and_then(|waveform| waveform.first_mut())
                {
                    *current = circuit.capacitors.i_prev[capacitor];
                }
            }
        }

        // OneStep's qHistory[0] is the accepted MutIndNonLin2 LOI vector,
        // not a value reconstructed from a later Newton iterate.  Seed the
        // Core snapshots only after DC/checkpoint histories are authoritative.
        circuit.initialize_xyce_core_q_histories();
        let mut xyce_direct_accepted_q = uses_direct_xyce_dae.then(|| Vec::with_capacity(size));
        if let Some(accepted_q) = xyce_direct_accepted_q.as_mut() {
            circuit
                .initialize_direct_xyce_accepted_q(accepted_q)
                .map_err(SimulationError::Circuit)?;
        }
        // Fresh startup retains ngspice's maxstep seed. A resumed run
        // replaces these provisional histories below with the exact decoded
        // checkpoint state, which is already normalized when the checkpoint
        // represents a breakpoint-style restart.
        let accepted_dt_seed = if resume.is_some() {
            0.0
        } else {
            hinted_max_step
        };
        // Only a fresh UIC startup stands in for ngspice's single
        // `MODEINITJCT|MODETRANOP|MODEUIC` device load, which is the one place
        // either reference reads a device-line `IC=` vector. A resumed run is
        // already mid-trajectory and an ordinary startup has a solved bias.
        let reactive_seed = if resume.is_none() && uic_requested {
            ReactiveHistorySeed::UicStartup
        } else {
            ReactiveHistorySeed::SolvedBias
        };
        let mut bjt_history = Self::initialize_bjt_history(&circuit, &solution, reactive_seed);
        let mut vbic_snapshot_cache = vec![None; circuit.bjts.devices.len()];
        // On a fresh run, ngspice seeds CKTdeltaOld[] with maxstep before the
        // first transient point. Mirror that only at startup so early
        // device-local truncation/order checks see the same history.
        bjt_history.accepted_dt_prev = accepted_dt_seed;
        bjt_history.accepted_dt_prev_prev = accepted_dt_seed;
        let mut jfet_history = Self::initialize_jfet_history(&circuit, &solution, reactive_seed);
        jfet_history.accepted_dt_prev = accepted_dt_seed;
        jfet_history.accepted_dt_prev_prev = accepted_dt_seed;
        let mut diode_history = Self::initialize_diode_history(&circuit, &solution, reactive_seed);
        diode_history.accepted_dt_prev = accepted_dt_seed;
        diode_history.accepted_dt_prev_prev = accepted_dt_seed;
        if let Some((restored_bjt, restored_diode, restored_snapshot_cache)) =
            restored_accepted_junction_history
        {
            bjt_history = restored_bjt;
            diode_history = restored_diode;
            vbic_snapshot_cache = restored_snapshot_cache;
        }

        if resume.is_some() {
            // Derived branches are initialized before checkpoint injection
            // and before integration-owned junction history exists. Rebuild
            // the seam only after both state owners are authoritative so a
            // native diode includes its exact committed dQ/dt contribution.
            let solved_branch_count = circuit.num_branches();
            for (branch, currents) in derived_branch_currents
                .iter()
                .zip(result.branch_currents.iter_mut().skip(solved_branch_count))
            {
                if let Some(first) = currents.first_mut() {
                    *first = Self::derived_transient_branch_current(
                        &mut circuit,
                        &solution,
                        resume_time,
                        Some(&diode_history),
                        *branch,
                    )?;
                }
            }
        }

        if resume.is_some() && record_device_op_traces {
            // The initial seam report was created before checkpoint injection
            // and before the accepted terminal-current history was installed.
            // Recording at the same sample index overwrites it in place with
            // restored diode/BJT caches and exact accepted BJT lead currents.
            retained_result_values = retained_result_values.saturating_add(
                result.record_device_op_sample(
                    circuit
                        .transient_device_op_report(
                            &solution,
                            &diode_history.cqd_prev,
                            &bjt_history.accepted_terminal_currents,
                        )
                        .map_err(SimulationError::Circuit)?,
                ),
            );
            self.ensure_transient_result_limits(&result, retained_result_values)?;
        }

        // Companion stamp slots resolved once against the frozen pattern:
        // the per-iteration charge companions then stamp through direct CSC
        // indices instead of a hash lookup per matrix entry.
        let diode_companion_slots = Self::link_diode_companion_slots(&circuit, &matrix);
        let mut mosfet_companion_slots = Self::link_mosfet_companion_slots(&circuit, &matrix);
        let vdmos_companion_slots = Self::link_vdmos_companion_slots(&circuit, &matrix);
        let mut mosfet_history =
            Self::initialize_mosfet_history(&circuit, &solution, reactive_seed);
        mosfet_history.accepted_dt_prev = accepted_dt_seed;
        mosfet_history.accepted_dt_prev_prev = accepted_dt_seed;
        let mut vdmos_history = Self::initialize_vdmos_history(&circuit, &solution);
        vdmos_history.accepted_dt_prev = accepted_dt_seed;
        vdmos_history.accepted_dt_prev_prev = accepted_dt_seed;
        let mut b3soi_history = Self::initialize_b3soi_history(&circuit, &solution);
        b3soi_history.accepted_dt_prev = accepted_dt_seed;
        b3soi_history.accepted_dt_prev_prev = accepted_dt_seed;
        let mut bsim3_history = Self::initialize_bsim3_history(&circuit, &solution);
        bsim3_history.accepted_dt_prev = accepted_dt_seed;
        bsim3_history.accepted_dt_prev_prev = accepted_dt_seed;
        let mut bsim4_history = Self::initialize_bsim4_history(&circuit, &solution);
        bsim4_history.accepted_dt_prev = accepted_dt_seed;
        bsim4_history.accepted_dt_prev_prev = accepted_dt_seed;
        let mut ekv26_history = Self::initialize_ekv26_history(&circuit, &solution);
        ekv26_history.accepted_dt_prev = accepted_dt_seed;
        ekv26_history.accepted_dt_prev_prev = accepted_dt_seed;
        let ideal_output_pairs = circuit.ideal_voltage_output_pairs();

        // Xyce OneStep carries the accepted physical static residual into its
        // next order-2 transient step.  The vector is refreshed only after an
        // accepted point so rejected Newton attempts never contaminate it.
        let mut xyce_static_history: Option<Vec<Value>> = None;
        // The direct Level-2 Core path owns a separate physical history.  It
        // must not consume the legacy matrix-probe residual above.
        let mut xyce_direct_static_history: Option<Vec<Value>> = None;

        // Main transient loop
        let mut retry_count = 0;
        let mut veriloga_event_refinement_count = 0_usize;
        // Xyce's OneStep/Gear12 `nef_` counts every rejected attempt, including
        // Newton failures and LTE failures, and is reset only after a point is
        // accepted.  Keep that integration-state counter separate from the
        // broader recovery budget used by the native/ngspice paths.
        let mut xyce_step_failure_count = 0_usize;
        let mut total_step_attempts = 0_usize;
        let mut stale_accept_count = 0;
        let mut force_accept_cooldown = 0_usize; // Failed retries to defer dt shrink immediately after force-accept
        let mut trap_order = native_order_after_restart(current_integration_method(&trapgear));
        // Xyce OneStep/Gear12 start at order 1; every native Gear2 path remains order 2.
        const MAX_RETRIES: usize = 200; // Maximum recovery retries per timepoint
        const MAX_VERILOGA_EVENT_REFINEMENTS: usize = 64;
        const FORCE_ACCEPT_COOLDOWN_RETRIES: usize = 2;
        const LINEARIZED_STARTUP_RECOVERY_POINTS: usize = 96;
        // Keep cancellation responsiveness tight for large transient decks where a
        // single accepted step can still be expensive.
        const ABORT_CHECK_INTERVAL: usize = 16;
        let progress_logging_enabled = log::log_enabled!(log::Level::Info);
        let mut last_progress_log = progress_logging_enabled.then(crate::time_compat::Instant::now);
        let mut rhs = vec![0.0; size];
        let mut xyce_direct_vectors =
            uses_direct_xyce_dae.then(|| crate::circuit::dae::XyceDaeVectors::new(size));
        let mut xyce_direct_workspace =
            uses_direct_xyce_dae.then(|| XyceOneStepWorkspace::new(size));
        let mut xyce_direct_q_candidate = uses_direct_xyce_dae.then(|| vec![0.0; size]);
        let mut xyce_direct_static_candidate = uses_direct_xyce_dae.then(|| vec![0.0; size]);
        // A classic-MOS/ordinary-RC transient has a run-invariant linear
        // network. Retain it once and extend it with per-attempt source and
        // capacitor terms below instead of rebuilding it per Newton iterate.
        let mut classic_mos_stamp_cache = (self.config.spice_dialect != SpiceDialect::Xyce
            && circuit.has_cacheable_classic_mos_transient_base())
        .then(|| {
            self.initialize_classic_mos_transient_stamp_cache(
                &circuit,
                &mut matrix,
                &mut rhs,
                transient_baseline_diag_gmin,
            )
        });
        if classic_mos_stamp_cache
            .as_ref()
            .is_some_and(residual::ClassicMosTransientStampCache::supports_compact_companion_stamps)
        {
            // The large classic-MOS cache owns the only companion topology
            // representation used by its hot path. Release the checked plan
            // rather than retaining two cache-sized streams.
            mosfet_companion_slots = Vec::new();
        }
        let mut diode_stamp_cache = (self.config.spice_dialect != SpiceDialect::Xyce
            && circuit.has_cacheable_diode_transient_base())
        .then(|| {
            self.initialize_diode_transient_stamp_cache(
                &circuit,
                &mut matrix,
                &mut rhs,
                transient_baseline_diag_gmin,
            )
        });
        let mut new_solution = solution.clone();
        let mut linear_solution = Vec::with_capacity(size);
        let mut correction_rhs = Vec::with_capacity(size);
        // Newton phase accounting is debug-only. In normal production runs
        // every DiagnosticTimer below avoids the underlying clock query.
        let diagnostic_timing_enabled = log::log_enabled!(log::Level::Debug);
        let transient_wall_start = DiagnosticTimer::start(diagnostic_timing_enabled);
        let mut total_stamp_nanos: u128 = 0;
        let mut total_solve_nanos: u128 = 0;
        let mut total_trunc_nanos: u128 = 0;
        let mut total_trap_trial_nanos: u128 = 0;
        let mut total_history_nanos: u128 = 0;
        let mut total_merit_nanos: u128 = 0;
        let mut total_postsolve_nanos: u128 = 0;
        let mut total_postsolve_update_nanos: u128 = 0;
        let mut total_postsolve_convergence_nanos: u128 = 0;
        let mut total_postsolve_residual_nanos: u128 = 0;
        let mut total_setup_nanos: u128 = 0;
        let mut total_postloop_nanos: u128 = 0;
        let mut total_top_nanos: u128 = 0;
        let mut total_tail_nanos: u128 = 0;
        let mut total_middle_nanos: u128 = 0;
        let mut total_merit_trials: usize = 0;
        let mut total_failed_attempts: usize = 0;
        // Xyce's default DampedNewton path takes full Newton steps.  Merit
        // globalization is retained as a deterministic retry only after a
        // full-step nonlinear attempt fails; it must not perturb a step that
        // already follows the source-faithful orbit.
        let mut xyce_damped_first_solver_call = true;
        let mut xyce_damped_status = uses_xyce_damped_solver.then(|| {
            damped_status::XyceTransientDampedStatus::new(
                self.transient_newton_iteration_budget(false),
            )
        });
        if let Some(restored_runtime) = restored_accepted_integration_runtime {
            let (
                restored_lte_warmup_skips,
                restored_force_accept_cooldown,
                restored_livelock_streak,
                restored_livelock_last_restart_time,
                restored_accepted_interval_count,
                restored_damped_first_solver_call,
                restored_damped_status,
            ) = match restored_runtime {
                ValidatedAcceptedIntegrationRuntime::Exact(runtime) => {
                    lte_estimator
                        .restore_accepted_boundary_checkpoint(&runtime.lte, &solution)
                        .map_err(SimulationError::Circuit)?;
                    trap_order = runtime.next_trap_order;
                    if let Some(snapshot) = runtime.trapgear.as_ref() {
                        trapgear
                            .restore_snapshot(snapshot, &solution)
                            .map_err(SimulationError::Circuit)?;
                    }
                    xyce_static_history.clone_from(&runtime.xyce_static_residual);
                    xyce_direct_accepted_q.clone_from(&runtime.direct_dae_accepted_q);
                    xyce_direct_static_history.clone_from(&runtime.direct_dae_static_residual);
                    (
                        runtime.lte_warmup_skips,
                        runtime.force_accept_cooldown,
                        runtime.livelock_streak,
                        runtime.livelock_last_restart_time,
                        runtime.accepted_interval_count,
                        runtime.damped_first_solver_call,
                        runtime.damped_status.as_ref(),
                    )
                }
                ValidatedAcceptedIntegrationRuntime::RestartNormalized(runtime) => (
                    runtime.lte_warmup_skips,
                    runtime.force_accept_cooldown,
                    runtime.livelock_streak,
                    runtime.livelock_last_restart_time,
                    runtime.accepted_interval_count,
                    runtime.damped_first_solver_call,
                    runtime.damped_status.as_ref(),
                ),
            };
            lte_warmup_skips = restored_lte_warmup_skips;
            force_accept_cooldown = restored_force_accept_cooldown;
            livelock_streak = restored_livelock_streak;
            livelock_last_restart_t = restored_livelock_last_restart_time;
            accepted_interval_count = restored_accepted_interval_count;
            xyce_damped_first_solver_call = restored_damped_first_solver_call;
            if let Some(restored_status) = restored_damped_status {
                xyce_damped_status
                    .as_mut()
                    .ok_or_else(|| {
                        SimulationError::Circuit(
                            "validated checkpoint DampedNewton state has no live target solver"
                                .to_string(),
                        )
                    })?
                    .restore_accepted_boundary_checkpoint(restored_status)
                    .map_err(SimulationError::Circuit)?;
            }
        }

        // The t=0/resume-time scheduled capture must observe every accepted
        // controller, history, and solver-policy owner. Capturing earlier
        // would authenticate a partially initialized continuation phase.
        let runtime_resume_blockers =
            Self::exact_integration_runtime_resume_blockers(&circuit, accepted_interval_count);
        let damped_status_checkpoint = xyce_damped_status
            .as_ref()
            .map(|status| status.capture_accepted_boundary_checkpoint())
            .transpose()
            .map_err(SimulationError::Circuit)?;
        self.capture_scheduled_checkpoint_if_due(
            scheduled_checkpoint_times,
            &mut scheduled_checkpoint_cursor,
            resume_time,
            fingerprint,
            &netlist_identity,
            &restart_identity,
            &simulation_identity,
            &solution,
            &circuit,
            startup_mode,
            max_step,
            Some(ProposedIntegrationContinuation {
                next_step: timestep.dt(),
                breakpoint_span_ceiling: xyce_breakpoint_span_ceiling.ceiling(),
                controller_max_step: timestep.max_dt(),
                analysis_first_step_pending,
                xyce_breakpoint_restart_pending: xyce_lte_restart_first_step,
            }),
            tstop,
            &pending_dynamic_tline_breakpoints,
            dynamic_tline_breakpoints_added,
            &bjt_history,
            &diode_history,
            &vbic_snapshot_cache,
            AcceptedIntegrationRuntimeCapture {
                lte_estimator: &lte_estimator,
                next_trap_order: trap_order,
                trapgear: fixed_method.is_none().then(|| trapgear.capture_snapshot()),
                xyce_static_residual: xyce_static_history.as_deref(),
                direct_dae_accepted_q: xyce_direct_accepted_q.as_deref(),
                direct_dae_static_residual: xyce_direct_static_history.as_deref(),
                lte_warmup_skips,
                force_accept_cooldown,
                livelock_streak,
                livelock_last_restart_time: livelock_last_restart_t,
                accepted_interval_count,
                damped_first_solver_call: xyce_damped_first_solver_call,
                damped_status: damped_status_checkpoint,
                retry_count,
                xyce_step_failure_count,
                stale_accept_count,
                resume_blockers: &runtime_resume_blockers,
            },
            retained_result_values,
            &mut retained_scheduled_checkpoint_values,
            &mut scheduled_checkpoints,
        )?;
        let b3soi_first_transient_handoff =
            accepted_interval_count == 0 && circuit.has_b3soi_devices();
        // Meyer capacitance halves captured by exact-residual assembly or the
        // device-truncation walk on the candidate solution; valid only for the
        // accept path of the same loop pass (reset every attempt).
        //
        // The source-faithful DampedNewton state above is independent of the
        // reusable MOS/LTE scratch below.
        let mut mosfet_caps_scratch: Vec<(Value, Value, Value)> = Vec::new();
        let mut mosfet_caps_valid;
        let mut mosfet_companion_terms_scratch: Vec<MosfetCompanionBranchTerms> = Vec::new();
        let mut mosfet_static_terms_scratch: Vec<
            crate::device::mosfet::ClassicMosCachedStaticTerms,
        > = Vec::new();
        let mut mosfet_companion_terms_valid;
        let mut mosfet_companion_charges_scratch: Vec<MosfetGateCompanionCharges> = Vec::new();
        // Ordinary-capacitor candidate voltage/current pairs captured by the
        // required CKTterr walk and consumed only by the matching accept path.
        let mut capacitor_accepted_states_scratch: Vec<CapacitorAcceptedState> = Vec::new();
        let mut capacitor_accepted_states_valid;
        #[cfg(feature = "parallel")]
        let capacitor_truncation_parallel_workers =
            self.capacitor_truncation_parallel_worker_count(&circuit);
        let mut classic_mos_residual_scratch = classic_mos_stamp_cache
            .as_ref()
            .is_some_and(residual::ClassicMosTransientStampCache::supports_direct_residual_proof)
            .then(|| (Vec::with_capacity(size), Vec::with_capacity(size)));
        #[cfg(feature = "parallel")]
        let capture_classic_mos_candidate_static_terms =
            classic_mos_stamp_cache.as_ref().is_some_and(|cache| {
                cache.supports_compact_candidate_static_stamps()
                    || cache.supports_parallel_direct_residual_proof()
            });
        #[cfg(not(feature = "parallel"))]
        let capture_classic_mos_candidate_static_terms =
            classic_mos_stamp_cache.as_ref().is_some_and(
                residual::ClassicMosTransientStampCache::supports_compact_candidate_static_stamps,
            );
        let reuse_sequential_classic_mos_newton_terms =
            classic_mos_stamp_cache.as_ref().is_some_and(|_| {
                #[cfg(feature = "parallel")]
                {
                    self.classic_mos_parallel_worker_count(circuit.mosfets.len())
                        .is_none()
                }
                #[cfg(not(feature = "parallel"))]
                {
                    true
                }
            });
        let mut cached_mosfet_truncation_limit;
        let mut cached_mosfet_truncation_limit_valid;
        let mut failed_voltage_conv: usize = 0;
        let mut failed_device_conv: usize = 0;
        let mut failed_residual_only: usize = 0;
        let mut rejected_attempt_nonlinear_state_scratch = None;

        // Xyce makes the interval after every accepted, non-final breakpoint a
        // new OneStep integration epoch.  Its restart dump is written before
        // that next-loop initialization, so scheduled checkpoints must observe
        // the accepted pre-initialization state and the uninterrupted run must
        // re-seed only after capture.  Keep the operation here, next to the
        // acceptance paths, because it intentionally resets loop-local device
        // histories and OneStep caches without discarding physical device or
        // transmission-line state.
        macro_rules! reinitialize_xyce_breakpoint_histories {
            ($hit_breakpoint:expr, $analysis_final_step:expr) => {
                if self.config.spice_dialect == SpiceDialect::Xyce
                    && $hit_breakpoint
                    && !$analysis_final_step
                {
                    Self::reseed_reactive_histories_for_restart(
                        &mut circuit,
                        &solution,
                        0.0,
                        AcceptedJunctionHistoryRestart::Preserve,
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
                    vbic_snapshot_cache.fill(None);
                    xyce_static_history = None;
                    xyce_direct_static_history = None;
                    circuit.initialize_xyce_core_q_histories();
                    if let Some(accepted_q) = xyce_direct_accepted_q.as_mut() {
                        circuit
                            .initialize_direct_xyce_accepted_q(accepted_q)
                            .map_err(SimulationError::Circuit)?;
                    }
                }
            };
        }

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
                        AcceptedJunctionHistoryRestart::Reinitialize,
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
                    if lte_estimator.uses_accepted_solution_reference() {
                        lte_estimator.restart_history_from(&solution);
                    } else {
                        lte_estimator.restart_history();
                        lte_warmup_skips = 2;
                    }
                    xyce_lte_restart_first_step = true;
                    let restart_dt = Self::ngspice_t0_breakpoint_limited_initial_timestep(
                        Self::ngspice_initial_timestep(tstop, tran_step_hint, hinted_max_step),
                        breakpoints.next_after(t),
                    );
                    timestep.force_step(restart_dt.max(timestep.preferred_min_dt()).min(max_step));
                    trap_order = native_order_after_restart(current_integration_method(&trapgear));
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

        // Adaptive integration may legitimately take far more attempts than
        // `TSTOP / DELMAX`: rejected local trials and accepted steps below the
        // ceiling are numerical work, not retained results. Termination is
        // governed by the per-point retry/minimum-step/livelock policies,
        // cooperative abort, and the explicit analysis/result resource limits.
        while t < tstop {
            let attempt_top_start = DiagnosticTimer::start(diagnostic_timing_enabled);
            mosfet_caps_valid = false;
            capacitor_accepted_states_valid = false;
            mosfet_companion_terms_valid = false;
            cached_mosfet_truncation_limit = None;
            cached_mosfet_truncation_limit_valid = false;
            if self.config.spice_dialect == SpiceDialect::Xyce {
                // Xyce 7.10 updates its machine-precision recovery floor from
                // the current accepted time before every transient advance.
                // Stiff state devices can legitimately require steps far below
                // ngspice's max-step-derived `delmin` while crossing a narrow
                // physical boundary layer.
                let hard_min_timestep = xyce_hard_min_timestep(t);
                timestep.set_hard_min_dt(hard_min_timestep);
                let breakpoint_tolerance = 2.0 * hard_min_timestep;
                circuit
                    .voltage_sources
                    .set_xyce_breakpoint_tolerance(breakpoint_tolerance);
                circuit
                    .current_sources
                    .set_xyce_breakpoint_tolerance(breakpoint_tolerance);
            }
            if let Some(target) = pending_veriloga_event_time {
                let event_dt = target - t;
                if !target.is_finite() || target <= t || !event_dt.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        "Verilog-A event target {target} is not strictly after accepted transient time {t}"
                    )));
                }
                if event_dt < timestep.hard_min_dt() {
                    return Err(SimulationError::Circuit(format!(
                        "Verilog-A event at t={target:.16e}s requires dt={event_dt:.16e}s below the solver hard minimum {:.16e}s from accepted time {t:.16e}s",
                        timestep.hard_min_dt()
                    )));
                }
            }
            // Progress logging every 2 seconds
            if last_progress_log
                .as_ref()
                .is_some_and(|started| started.elapsed().as_secs() >= 2)
            {
                log::info!(
                    "Transient progress: t={:.12e}s / {:.3e}s ({:.1}%), dt={:.3e}, retries={}, order={}, {} step attempts",
                    t,
                    tstop,
                    (t / tstop) * 100.0,
                    timestep.dt(),
                    retry_count,
                    trap_order,
                    total_step_attempts
                );
                last_progress_log = Some(crate::time_compat::Instant::now());
            }

            // Abort check every few step attempts for minimal overhead.
            if total_step_attempts % ABORT_CHECK_INTERVAL == 0 {
                if tstop > 0.0 {
                    abort.observe_progress((t / tstop).clamp(0.0, 1.0));
                }
                let is_aborted = abort.is_aborted();
                if total_step_attempts == 0 {
                    log::debug!("First abort check, is_aborted={}", is_aborted);
                }
                if is_aborted {
                    log::info!(
                        "Transient simulation aborted at t={:.3e}s ({:.1}% complete, {} step attempts)",
                        t,
                        (t / tstop) * 100.0,
                        total_step_attempts
                    );
                    // Return error indicating abort - partial results are lost
                    return Err(SimulationError::Aborted);
                }
            }

            total_step_attempts = total_step_attempts.saturating_add(1);
            if locked_grid.is_none() {
                let span_ceiling = xyce_breakpoint_span_ceiling.ceiling();
                timestep.set_max_dt(
                    self.transient_device_max_timestep(&circuit, t, hinted_max_step)
                        .min(span_ceiling.unwrap_or(Value::INFINITY)),
                );
            }
            let mut locked_step_lands_on_grid = locked_grid.is_some();
            let locked_schedule_aligned = locked_grid
                .as_ref()
                .zip(locked_step_sizes.as_ref())
                .is_some_and(|(grid, _)| {
                    locked_cursor == 0
                        || grid
                            .get(locked_cursor.saturating_sub(1))
                            .is_some_and(|&previous_target| previous_target == t)
                });
            let (mut dt, mut at_breakpoint) = match locked_grid.as_ref() {
                Some(grid) => {
                    let Some(&target) = grid.get(locked_cursor) else {
                        break;
                    };
                    let tolerance = ((target - t).abs() * 1.0e-12).max(1.0e-18);
                    let mut step_target = circuit
                        .next_xspice_event_time()
                        .filter(|event_time| {
                            *event_time > t + tolerance && *event_time < target - tolerance
                        })
                        .unwrap_or(target);
                    if let Some(breakpoint) = breakpoints
                        .next_after(t)
                        .filter(|breakpoint| *breakpoint < target - tolerance)
                    {
                        step_target = step_target.min(breakpoint);
                    }
                    // Whether `hinted_max_step` may subdivide the interval
                    // depends on which of the two locking contracts is in
                    // force, and they are told apart by whether the caller
                    // supplied recorded step sizes.
                    //
                    // *With* step sizes the grid is a bit-exact replay of a
                    // producing run's accepted-step sequence, and that
                    // sequence is the contract. Subdividing it diverges any
                    // device whose state is a function of the sequence rather
                    // than of time alone: the generic switch reads its
                    // hysteresis band from the store vector two accepted
                    // points back, so extra points between two reference
                    // samples silently pick the wrong band.
                    //
                    // *Without* them the caller knows only the times -- an
                    // ngspice `.out` table records where the reference landed,
                    // not the accepted intervals it chose, and its adaptive
                    // axis is coarse wherever it judged the waveform slow.
                    // Landing on every target is still required; how the
                    // integrator gets there is its own business, and refusing
                    // to sub-step means one trapezoidal step across whatever
                    // gap the reference happened to leave. On `mosamp.cir`
                    // that is a 65 ns step through an amplifier slew, and the
                    // truncation error shows up as a ~0.6 ns phase lead.
                    if locked_step_sizes.is_none()
                        && hinted_max_step.is_finite()
                        && hinted_max_step > 0.0
                    {
                        step_target = step_target.min(t + hinted_max_step);
                    }
                    locked_step_lands_on_grid = (step_target - target).abs() <= tolerance;
                    let mut locked_dt = step_target - t;
                    if locked_schedule_aligned
                        && locked_step_lands_on_grid
                        && let Some(&scheduled_dt) = locked_step_sizes
                            .as_ref()
                            .and_then(|steps| steps.get(locked_cursor))
                        && scheduled_dt.is_finite()
                        && scheduled_dt > 0.0
                    {
                        // A result carries the exact accepted interval used by
                        // its producing run. Prefer it over subtraction of
                        // rounded absolute timestamps so replay preserves the
                        // original integration coefficients bit-for-bit.
                        locked_dt = scheduled_dt;
                    }
                    (locked_dt, false)
                }
                None => breakpoints.limit_step(t, timestep.dt()),
            };
            dt = dt.min(tstop - t); // Don't overshoot tstop
            let mut exact_veriloga_event_time = None;
            if let Some(target) = pending_veriloga_event_time
                && target > t
                && target <= canonical_transient_step_time(t, dt, tstop)
            {
                dt = target - t;
                exact_veriloga_event_time = Some(target);
                at_breakpoint = true;
                locked_step_lands_on_grid = locked_grid
                    .as_ref()
                    .and_then(|grid| grid.get(locked_cursor))
                    .is_some_and(|grid_target| *grid_target == target);
            }
            let mut locked_replay_hidden_attempt = false;
            let locked_contraction_replay = locked_grid.as_ref().is_some_and(|grid| {
                Self::dialect_requires_locked_grid_order_restart(
                    self.config.spice_dialect,
                    grid,
                    locked_cursor,
                    false,
                )
            });
            if locked_grid.is_some()
                && pending_veriloga_event_time.is_none()
                && (retry_count > 0 || locked_contraction_replay)
                && let (Some(grid), Some(steps)) =
                    (locked_grid.as_ref(), locked_step_sizes.as_ref())
                && let (Some(&target), Some(&scheduled_dt)) =
                    (grid.get(locked_cursor), steps.get(locked_cursor))
                && !breakpoints.at_breakpoint(target)
                && !circuit.next_xspice_event_time().is_some_and(|event_time| {
                    let tolerance = (target.abs() * 1.0e-12).max(1.0e-18);
                    (event_time - target).abs() <= tolerance
                })
                && scheduled_dt.is_finite()
                && scheduled_dt > 0.0
                && timestep.dt() > scheduled_dt * (1.0 + 1.0e-12)
            {
                // A paired reference grid records accepted intervals, not the
                // adaptive candidates rejected before them. When the current
                // controller proposal exceeds a non-breakpoint reference
                // interval, replay that proposal as a hidden trial so the
                // nonlinear rollback and predictor history follow the
                // producing run before landing on the prescribed target.
                dt = timestep.dt().min(max_step).min(tstop - t);
                locked_replay_hidden_attempt = dt > scheduled_dt;
            }
            let mut candidate_step_time = canonical_transient_step_time_with_device_event(
                t,
                dt,
                tstop,
                exact_veriloga_event_time,
            );
            let mut expected_source_delta =
                Self::max_expected_source_delta(&circuit, t, candidate_step_time);
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
                    Self::source_ramp_tracking_delta(
                        &circuit,
                        self.config.transient_node_activity_bound,
                    ),
                    practical_min,
                    timestep.preferred_min_dt(),
                    Self::should_apply_active_source_recovery_cap(force_accept_cooldown),
                    nonlinear_source_ramp_cap_enabled,
                );
                if biased_dt + 1e-30 < dt {
                    dt = biased_dt;
                    exact_veriloga_event_time = None;
                    candidate_step_time = canonical_transient_step_time(t, dt, tstop);
                    at_breakpoint = breakpoints.at_breakpoint(candidate_step_time);
                    expected_source_delta =
                        Self::max_expected_source_delta(&circuit, t, candidate_step_time);
                }
            }
            if fixed_method.is_none() {
                trapgear.set_at_breakpoint(at_breakpoint);
            } else if let Some(method) = fixed_method {
                trapgear.force_method(method);
            }
            let step_time = canonical_transient_step_time_with_device_event(
                t,
                dt,
                tstop,
                exact_veriloga_event_time,
            );
            let landed_veriloga_event = exact_veriloga_event_time.is_some();
            let analysis_initial_step = uic_requested && accepted_interval_count == 0;
            let analysis_final_step = step_time == tstop;
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
            let xyce_one_step_stateful_topology =
                Self::xyce_one_step_requires_order_one_for_ic_branch(&circuit)
                    || Self::xyce_one_step_requires_order_one_for_stateful_topology(&circuit)
                    || !circuit.tlines.is_empty()
                    || !circuit.coupled_tlines.is_empty();
            let locked_reference_order_restart = !xyce_one_step_stateful_topology
                && locked_grid.as_ref().is_some_and(|grid| {
                    Self::dialect_requires_locked_grid_order_restart(
                        self.config.spice_dialect,
                        grid,
                        locked_cursor,
                        breakpoints.at_breakpoint(step_time),
                    )
                });
            // A locked oracle grid contains accepted points only.  Its interval
            // ratios therefore cannot identify the rejected candidates that
            // changed Xyce's OneStep order; the replay path above preserves
            // those candidates and the estimator's accepted-state history.
            // Resume is a breakpoint-style integration restart. The checkpoint
            // supplies the accepted solution but deliberately omits nonlinear
            // charge histories and their timestep provenance, so the first real
            // post-resume interval must be order one. Rejected attempts do not
            // append a result point and therefore remain order one; after any
            // accepted path commits the interval, native fixed Gear2 naturally
            // returns to its preserved order-two `trap_order` on the next step.
            // Xyce clips an already-proposed interval to a breakpoint without
            // changing that interval's order. Its integration reinitialize
            // happens only after the landing point is accepted, so the first
            // interval *leaving* the breakpoint is the order-one step.
            let is_first_resumed_interval = resume_is_restart_normalized && result.time.len() == 1;
            let step_trap_order = if is_first_resumed_interval {
                native_order_after_restart(current_method)
            } else {
                Self::step_trapezoidal_order(
                    current_method,
                    trap_order,
                    (at_breakpoint
                        && Self::breakpoint_landing_forces_order_one(self.config.spice_dialect))
                        || locked_edge_order_reset
                        || locked_reference_order_restart,
                )
            };
            let xyce_one_step = self.config.spice_dialect == SpiceDialect::Xyce
                && !xyce_one_step_stateful_topology
                && matches!(
                    current_method,
                    IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                )
                && (circuit.couplings.is_empty() || circuit.has_only_xyce_core_inductors());
            let xyce_one_step_order2 = xyce_one_step
                && step_trap_order == 2
                && matches!(
                    current_method,
                    IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                );
            if xyce_lte_restart_first_step {
                lte_estimator.seed_restart_timestep(dt);
            }
            let effective_companion_method =
                Self::effective_companion_method(current_method, step_trap_order);
            let coeff = if effective_companion_method == IntegrationMethod::Trapezoidal {
                modified_trapezoidal_coefficients
            } else {
                CompanionCoefficients::for_method_with_previous_step(
                    effective_companion_method,
                    dt,
                    bjt_history.accepted_dt_prev,
                )
            };
            // The native BSIM4 transient-NQS state was validated and released
            // with fixed BDF2 coefficients. Keep that compact-model contract
            // isolated from variable-step Gear12; applying the latter to the
            // charge-deficit state produces a large ngspice-46 mismatch after
            // sharp source edges. QS BSIM4 and every other device family keep
            // the requested nonuniform coefficients.
            let bsim4_trnqs_coeff = if native_predictor_local
                && effective_companion_method == IntegrationMethod::Gear2
            {
                CompanionCoefficients::gear2()
            } else {
                coeff
            };
            let suppress_gate_charge = false;
            let classic_mos_truncation_context = if classic_mos_stamp_cache.is_some()
                && Self::uses_ngspice_charge_truncation(&lte_estimator)
                && !analysis_first_step_pending
                && lte_warmup_skips == 0
                && !suppress_gate_charge
                && !truncation::lte_debug_enabled()
            {
                NgspiceChargeTruncationContext::new(
                    dt,
                    mosfet_history.accepted_dt_prev,
                    mosfet_history.accepted_dt_prev_prev,
                    effective_companion_method,
                    step_trap_order,
                    transient_lte_reltol,
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                )
            } else {
                None
            };
            if let Some(cache) = classic_mos_stamp_cache.as_mut() {
                self.prepare_classic_mos_transient_attempt(
                    cache,
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    step_time,
                    dt,
                    &coeff,
                    xyce_one_step,
                );
            }
            if let Some(cache) = diode_stamp_cache.as_mut() {
                self.prepare_diode_transient_attempt(
                    cache,
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    step_time,
                    dt,
                    &coeff,
                    xyce_one_step,
                );
            }
            let mut rejected_attempt_nonlinear_state = if circuit.has_nonlinear_devices() {
                if let Some(mut snapshot) = rejected_attempt_nonlinear_state_scratch.take() {
                    circuit.refresh_transient_trial_state_snapshot(&mut snapshot);
                    Some(snapshot)
                } else {
                    Some(circuit.transient_trial_state_snapshot())
                }
            } else {
                None
            };
            if let Err(error) = circuit.prepare_xyce_team_resistance_noise_trial(step_time) {
                if let Some(snapshot) = rejected_attempt_nonlinear_state.take() {
                    circuit.restore_nonlinear_state(snapshot);
                }
                return Err(SimulationError::Circuit(error));
            }
            macro_rules! restore_rejected_transient_nonlinear_state {
                () => {{
                    // Xyce OneStep/Gear12 intentionally does not call
                    // restoreHistory for ERROPTION=1 rejection. Its psi
                    // coefficient history advances with the failed attempt;
                    // only ERROPTION=0 performs the one-sided rollback.
                    lte_estimator.reject_xyce_attempt(!xyce_iteration_error_control);
                    if let Some(snapshot) = rejected_attempt_nonlinear_state.as_ref().cloned() {
                        if circuit.has_xyce_core_inductors() {
                            circuit.restore_nonlinear_state_preserving_xyce_core_level2_carry(
                                snapshot,
                            );
                        } else {
                            circuit.restore_nonlinear_state(snapshot);
                        }
                    }
                }};
            }
            macro_rules! reject_for_veriloga_event_refinement {
                ($candidate_solution:expr, $candidate_time:expr, $phase_start:expr) => {{
                    if circuit.has_any_veriloga_devices() {
                        #[cfg(feature = "veriloga")]
                        if circuit.has_veriloga_devices() {
                        circuit
                            .evaluate_veriloga_timepoint($candidate_solution)
                            .map_err(SimulationError::Circuit)?;
                        }
                        #[cfg(feature = "veriloga-builtins-base")]
                        if circuit.has_generated_veriloga_devices() {
                            circuit
                                .evaluate_generated_veriloga_timepoint(
                                    &mut matrix,
                                    $candidate_solution,
                                )
                                .map_err(SimulationError::Circuit)?;
                        }
                        if let Some(target) = circuit
                            .veriloga_event_refinement_time()
                            .map_err(SimulationError::Circuit)?
                        {
                            let accepted_time = t;
                            let candidate_time = $candidate_time;
                            let refinement_dt = target - accepted_time;
                            if !target.is_finite()
                                || target <= accepted_time
                                || target >= candidate_time
                                || !refinement_dt.is_finite()
                            {
                                restore_rejected_transient_nonlinear_state!();
                                return Err(SimulationError::Circuit(format!(
                                    "Verilog-A event refinement target {target} is not strictly inside transient interval ({accepted_time}, {candidate_time})"
                                )));
                            }
                            if refinement_dt < timestep.hard_min_dt() {
                                restore_rejected_transient_nonlinear_state!();
                                return Err(SimulationError::Circuit(format!(
                                    "Verilog-A event refinement at t={accepted_time:.16e}s requires dt={refinement_dt:.16e}s below the solver hard minimum {:.16e}s",
                                    timestep.hard_min_dt()
                                )));
                            }
                            veriloga_event_refinement_count =
                                veriloga_event_refinement_count.saturating_add(1);
                            if veriloga_event_refinement_count > MAX_VERILOGA_EVENT_REFINEMENTS {
                                restore_rejected_transient_nonlinear_state!();
                                return Err(SimulationError::Circuit(format!(
                                    "Verilog-A event root failed to satisfy its time and expression tolerances after {MAX_VERILOGA_EVENT_REFINEMENTS} refinements at t={accepted_time:.16e}s"
                                )));
                            }
                            pending_veriloga_event_time = Some(target);
                            retry_count = retry_count.saturating_add(1);
                            self.record_convergence(|quality| quality.record_timestep_reduction());
                            trap_order = Self::trapezoidal_order_after_timestep_control_reject(
                                step_trap_order,
                            );
                            timestep.force_step(refinement_dt);
                            restore_rejected_transient_nonlinear_state!();
                            total_middle_nanos += $phase_start.elapsed().as_nanos();
                            continue;
                        }
                        // Any accepted point changes the interpolation lower
                        // endpoint. The acceptance path below replaces an old
                        // secant target with the next accepted timer target (or
                        // `None`) after it commits the Verilog-A state.
                        veriloga_event_refinement_count = 0;
                    }
                }};
            }
            for dev in &circuit.b3soi.devices {
                dev.begin_transient_timestep_iteration(dt, b3soi_history.accepted_dt_prev);
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
            ) && accepted_interval_count.saturating_add(1)
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
            let setup_phase_start = DiagnosticTimer::start(diagnostic_timing_enabled);
            lte_estimator.begin_xyce_attempt(dt, step_trap_order);
            // Prepare for Newton iteration at this timestep by seeding the full
            // algebraic solution vector from accepted history when a predictor
            // state is available. ngspice's `NIpred()` predicts every solver
            // unknown, including branch-current equations, not just node
            // voltages. Matching that behavior materially improves the initial
            // Newton guess for source-heavy compact-model decks.
            let lte_predicted_solution =
                lte_estimator.predict_solution(dt, current_method, step_trap_order);
            if let Some(predicted_solution) = lte_predicted_solution.as_ref() {
                new_solution.clone_from(predicted_solution);
            } else {
                new_solution.clone_from(&solution);
            }
            circuit.enforce_ideal_voltage_constraints(&mut new_solution, step_time)?;
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
                } else if circuit.has_xyce_core_inductors() {
                    MAX_XYCE_CORE_BRANCH_STATE_MAGNITUDE
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
                    circuit.enforce_ideal_voltage_constraints(&mut new_solution, step_time)?;
                }
                Self::clip_ideal_output_common_modes(
                    &solution,
                    &mut new_solution,
                    newton_step_delta_limit,
                    &ideal_output_pairs,
                );
            }
            // Xyce's transient NOX solver freezes one weight per MNA unknown
            // from the initial timepoint iterate and the previously accepted
            // solution.  Keep those weights immutable across every Newton
            // correction and globalization trial for this attempted step.
            let damped_first_solver_call = xyce_damped_first_solver_call;
            if uses_xyce_damped_solver {
                // Xyce increments iNumCalls_ after every nonlinear solve,
                // including a failed/rejected attempt.  Consume the first-call
                // special case exactly once over the whole transient run.
                xyce_damped_first_solver_call = false;
            }
            let transient_newton_update_weights = if uses_xyce_damped_solver {
                damped_status::xyce_damped_transient_weights(
                    &new_solution,
                    &solution,
                    self.transient_nonlinear_reltol(),
                    self.transient_nonlinear_abstol(),
                    damped_first_solver_call,
                )
            } else {
                self.transient_newton_update_weights(&new_solution, &solution)
            };
            if b3soi_first_transient_handoff && accepted_interval_count == 0 {
                Self::reseed_b3soi_first_transient_history(
                    &circuit,
                    &new_solution,
                    &mut b3soi_history,
                );
            }
            let mut nonlinear_state_matches_new_solution = false;
            let mut had_solver_candidate = false;
            // Merit-gated Newton globalization state: the true nonlinear
            // residual norm of the previously stamped iterate, the iterate
            // itself, and any backtracking search currently walking a
            // rejected step (see transient/globalization.rs).
            let mut merit_backtrack: Option<(
                globalization::NewtonMeritBacktrack,
                TransientMeritRollback,
            )> = None;
            let mut last_stamped_iterate: Vec<Value> = Vec::new();
            let mut last_stamped_merit = Value::INFINITY;
            let mut last_stamped_rollback: Option<TransientMeritRollback> = None;

            // Newton-Raphson iteration for this timestep.
            // Classic SPICE transient analysis uses the transient-specific ITL4
            // budget, not the DC operating-point iteration limit.
            let tran_max_iterations = self.transient_newton_iteration_budget(startup_recovery);
            let uses_xyce_nox_status = self.config.spice_dialect == SpiceDialect::Xyce
                && self.config.transient_nonlinear_nox.unwrap_or(false);
            let mut xyce_nox_status = uses_xyce_nox_status
                .then(|| nox_status::XyceTransientNoxStatus::new(tran_max_iterations));
            let mut xyce_weighted_update_norm = None;
            let mut converged = false;
            // Solver-owned count for this attempted timepoint. DampedNewton's
            // `nlStep_` is one-based; NOX iteration zero is the predictor
            // before any linear solve. The outer transient attempt counter is
            // deliberately unrelated to ERROPTION step control.
            let mut nonlinear_iterations = 0_usize;
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
            // NOX evaluates the predictor at iteration zero and the candidate
            // produced by its MAXSTEP-th solve at iteration MAXSTEP.
            let newton_status_checks = tran_max_iterations + usize::from(uses_xyce_nox_status);
            for _iter in 0..newton_status_checks {
                if converged {
                    break;
                }
                if _iter % ABORT_CHECK_INTERVAL == 0 && abort.is_aborted() {
                    log::info!(
                        "Transient simulation aborted during Newton solve at t={:.3e}s ({:.1}% complete, {} step attempts)",
                        t,
                        (t / tstop) * 100.0,
                        total_step_attempts
                    );
                    return Err(SimulationError::Aborted);
                }
                let iteration_delta_limit =
                    Self::adaptive_transient_newton_delta_limit(newton_step_delta_limit, _iter);
                let newton_stamp_start = DiagnosticTimer::start(diagnostic_timing_enabled);
                let transient_system_context = residual::TransientSystemContext {
                    coeff: &coeff,
                    xyce_one_step,
                    xyce_one_step_order2,
                    xyce_static_history: xyce_static_history.as_deref(),
                    bsim4_trnqs_coeff: &bsim4_trnqs_coeff,
                    bjt_history: &bjt_history,
                    jfet_history: &jfet_history,
                    diode_history: &diode_history,
                    diode_companion_slots: &diode_companion_slots,
                    diode_attempt_cache: diode_stamp_cache.as_ref(),
                    mosfet_history: &mosfet_history,
                    mosfet_companion_slots: &mosfet_companion_slots,
                    vdmos_history: &vdmos_history,
                    vdmos_companion_slots: &vdmos_companion_slots,
                    b3soi_history: &b3soi_history,
                    b3soi_zero_first_transient_charge_derivative: b3soi_first_transient_handoff
                        && accepted_interval_count == 0
                        && _iter == 0,
                    bsim3_history: &bsim3_history,
                    bsim4_history: &bsim4_history,
                    ekv26_history: &ekv26_history,
                    suppress_gate_charge,
                    baseline_diag_gmin: transient_baseline_diag_gmin,
                    tline_dc_refs: &tline_dc_refs,
                    coupled_tline_refs: &coupled_tline_refs,
                    analysis_initial_step,
                    analysis_final_step,
                };
                if let Some(cache) = classic_mos_stamp_cache.as_ref() {
                    let refresh_classic_mos_nonlinear = !nonlinear_state_matches_new_solution;
                    let reuse_classic_mos_terms = reuse_sequential_classic_mos_newton_terms
                        && mosfet_companion_terms_valid
                        && nonlinear_state_matches_new_solution
                        && mosfet_companion_terms_scratch.len() == circuit.mosfets.devices.len();
                    if reuse_classic_mos_terms {
                        let reusable_static_terms = capture_classic_mos_candidate_static_terms
                            .then_some(mosfet_static_terms_scratch.as_slice());
                        self.stamp_classic_mos_transient_system_from_cache(
                            cache,
                            &mut circuit,
                            &mut matrix,
                            &mut rhs,
                            &new_solution,
                            dt,
                            &transient_system_context,
                            false,
                            crate::device::veriloga_builtins::GeneratedEvaluationMode::NewtonLimited,
                            Some(mosfet_companion_terms_scratch.as_slice()),
                            reusable_static_terms,
                            None,
                            None,
                            None,
                        )?;
                    } else {
                        self.stamp_classic_mos_transient_system_from_cache(
                            cache,
                            &mut circuit,
                            &mut matrix,
                            &mut rhs,
                            &new_solution,
                            dt,
                            &transient_system_context,
                            refresh_classic_mos_nonlinear,
                            crate::device::veriloga_builtins::GeneratedEvaluationMode::NewtonLimited,
                            None,
                            None,
                            Some(&mut mosfet_companion_terms_scratch),
                            Some(&mut mosfet_static_terms_scratch),
                            None,
                        )?;
                        mosfet_companion_terms_valid = reuse_sequential_classic_mos_newton_terms
                            && refresh_classic_mos_nonlinear
                            && mosfet_companion_terms_scratch.len()
                                == circuit.mosfets.devices.len();
                    }
                } else {
                    self.stamp_transient_system(
                        &mut circuit,
                        &mut matrix,
                        &mut rhs,
                        &new_solution,
                        step_time,
                        dt,
                        &transient_system_context,
                        &mut vbic_snapshot_cache,
                        VbicCachedSnapshotReuse::NewtonBypass,
                        !nonlinear_state_matches_new_solution,
                        0.0,
                    )?;
                }
                nonlinear_state_matches_new_solution = true;

                // Form the source-faithful physical residual after the
                // canonical stamp has cached the exact Core endpoint.  The
                // matrix remains the Jacobian; this RHS is the Newton
                // correction `-R` consumed by the direct path below.
                let direct_correction_rhs: Option<&[Value]> = if uses_direct_xyce_dae {
                    let vectors = xyce_direct_vectors
                        .as_mut()
                        .expect("direct Xyce DAE vectors are allocated for the gated path");
                    circuit
                        .load_direct_xyce_level2_core_dae(&new_solution, step_time, 0.0, vectors)
                        .map_err(SimulationError::Circuit)?;
                    let previous_q = xyce_direct_accepted_q.as_deref().ok_or_else(|| {
                        SimulationError::Circuit("direct Xyce accepted Q history is missing".into())
                    })?;
                    let previous_static = xyce_direct_static_history.as_deref();
                    let order = if xyce_one_step_order2 {
                        XyceOneStepOrder::Second
                    } else {
                        XyceOneStepOrder::First
                    };
                    let workspace = xyce_direct_workspace
                        .as_mut()
                        .expect("direct Xyce DAE workspace is allocated for the gated path");
                    Some(
                        workspace
                            .form_correction_rhs(vectors, previous_q, previous_static, dt, order)
                            .map_err(|error| SimulationError::Circuit(error.to_string()))?,
                    )
                } else {
                    None
                };

                if uses_xyce_damped_solver && _iter == 0 {
                    let (_, predictor_residual_l2_norm) =
                        if let Some(residual) = direct_correction_rhs {
                            direct_xyce_dae_norms(residual)?
                        } else {
                            matrix.raw_residual_norms(&new_solution, &rhs)?
                        };
                    if let Some(status) = xyce_damped_status.as_mut() {
                        status.begin_solve_with_initial_residual(
                            tran_max_iterations,
                            predictor_residual_l2_norm,
                        );
                    }
                }

                let newton_stamp_elapsed = newton_stamp_start.elapsed();
                total_stamp_nanos += newton_stamp_elapsed.as_nanos();
                static TRANSIENT_NEWTON_STAMP_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                if newton_stamp_elapsed.as_millis() >= 100 {
                    let log_count = TRANSIENT_NEWTON_STAMP_LOG_COUNT
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_count < 40 {
                        log::warn!(
                            "Slow transient Newton stamp at t={:.6e}, dt={:.3e}, step_attempt={}, elapsed={:.3?}",
                            step_time,
                            dt,
                            total_step_attempts,
                            newton_stamp_elapsed,
                        );
                    }
                }

                // Merit-gated Newton globalization: the freshly stamped
                // system gives the true nonlinear residual at the current
                // iterate for one matrix-vector product. Judge the previous
                // Newton step with it and damp the step when it has left its
                // basin — the saturation-boundary limit cycles this breaks
                // are unreachable by timestep reduction alone (the cycle is
                // driven by the static nonlinearity, not by stiffness).
                let globalization_active = !is_strictly_linear_transient
                    && ((self.config.spice_dialect != SpiceDialect::Xyce
                        && circuit.has_nonlinear_devices())
                        || (self.config.spice_dialect == SpiceDialect::Xyce
                            && circuit.has_xyce_core_inductors()
                            && uses_xyce_nox_status));
                // Iteration zero has no preceding Newton step to judge. Defer
                // its merit product and rollback capture until the post-solve
                // checks prove that a second iteration is actually needed.
                // The overwhelmingly common one-solve timestep therefore
                // performs neither globalization matrix-vector product.
                if globalization_active && _iter > 0 {
                    let merit_phase_start = DiagnosticTimer::start(diagnostic_timing_enabled);
                    let current_merit = self
                        .residual_inf_norm(&circuit, &mut matrix, &new_solution, &rhs)
                        .unwrap_or(Value::INFINITY);
                    if let Some((mut search, rollback)) = merit_backtrack.take() {
                        match search.judge(current_merit) {
                            globalization::BacktrackAction::Trial(trial) => {
                                restore_transient_merit_rollback(
                                    &mut circuit,
                                    &mut vbic_snapshot_cache,
                                    &rollback,
                                );
                                new_solution = trial;
                                circuit.enforce_ideal_voltage_constraints(
                                    &mut new_solution,
                                    step_time,
                                )?;
                                nonlinear_state_matches_new_solution = false;
                                merit_backtrack = Some((search, rollback));
                                total_merit_nanos += merit_phase_start.elapsed().as_nanos();
                                total_merit_trials += 1;
                                continue;
                            }
                            globalization::BacktrackAction::Accept => {}
                        }
                    } else if let Some(base_rollback) = last_stamped_rollback.as_ref()
                        && ((circuit.has_xyce_core_inductors()
                            && current_merit.is_finite()
                            && last_stamped_merit.is_finite()
                            && current_merit > 1.0
                            && current_merit > last_stamped_merit)
                            || globalization::NewtonMeritBacktrack::step_needs_globalization(
                                last_stamped_merit,
                                current_merit,
                            ))
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
                        let rollback = base_rollback.clone();
                        restore_transient_merit_rollback(
                            &mut circuit,
                            &mut vbic_snapshot_cache,
                            &rollback,
                        );
                        new_solution = trial;
                        circuit.enforce_ideal_voltage_constraints(&mut new_solution, step_time)?;
                        nonlinear_state_matches_new_solution = false;
                        merit_backtrack = Some((search, rollback));
                        total_merit_nanos += merit_phase_start.elapsed().as_nanos();
                        total_merit_trials += 1;
                        continue;
                    }
                    last_stamped_iterate.clone_from(&new_solution);
                    last_stamped_merit = current_merit;
                    capture_transient_merit_rollback(
                        &circuit,
                        &vbic_snapshot_cache,
                        classic_mos_stamp_cache.is_some(),
                        &mut last_stamped_rollback,
                    );
                    total_merit_nanos += merit_phase_start.elapsed().as_nanos();
                }

                if let Some(status) = xyce_nox_status.as_mut() {
                    let (residual_inf_norm, residual_l2_norm) =
                        matrix.raw_residual_norms(&new_solution, &rhs)?;
                    let device_converged = core_trial_converged(&circuit)
                        && (!enforce_device_convergence
                            || !circuit.has_nonlinear_devices()
                            || circuit.nonlinear_converged(self.device_convergence_criteria()));
                    let behavioral_converged = circuit
                        .behavioral_linearizations_converged(
                            &new_solution,
                            t,
                            self.voltage_reltol(),
                            self.voltage_abstol(),
                            self.current_abstol(),
                        )
                        .map_err(SimulationError::Circuit)?;
                    let decision = status.evaluate(
                        nox_status::XyceNoxSample {
                            iteration: _iter,
                            residual_inf_norm,
                            residual_l2_norm,
                            weighted_update_norm: xyce_weighted_update_norm,
                            device_converged,
                        },
                        self.transient_nonlinear_deltaxtol(),
                        self.transient_nonlinear_rhstol(),
                    );
                    match decision {
                        nox_status::XyceNoxDecision::Accepted { test, return_code }
                            if behavioral_converged =>
                        {
                            nonlinear_iterations = _iter;
                            log::trace!(
                                "Xyce transient NOX accepted t={:.6e}, dt={:.3e}, iter={}, test={}, code={}",
                                step_time,
                                dt,
                                _iter,
                                test,
                                return_code,
                            );
                            converged = true;
                            break;
                        }
                        nox_status::XyceNoxDecision::Accepted { .. } => {
                            // Behavioral expressions use an internal affine
                            // linearization that must agree at the candidate
                            // before any positive NOX status can be committed.
                            if _iter >= tran_max_iterations {
                                break;
                            }
                        }
                        nox_status::XyceNoxDecision::Failed { test, return_code } => {
                            log::trace!(
                                "Xyce transient NOX rejected t={:.6e}, dt={:.3e}, iter={}, test={}, code={}",
                                step_time,
                                dt,
                                _iter,
                                test,
                                return_code,
                            );
                            break;
                        }
                        nox_status::XyceNoxDecision::Continue => {}
                    }
                }

                // Solve and check convergence
                let newton_solve_start = DiagnosticTimer::start(diagnostic_timing_enabled);
                // Xyce's DampedNewton linear system solves for a Newton search
                // direction on every topology.  NOX, native, and ngspice keep
                // their established algebra except where an inductor or the
                // direct Xyce DAE path already requires correction form.
                let solve_produces_correction = transient_newton_uses_correction_form(
                    direct_correction_rhs.is_some(),
                    uses_inductor_correction,
                    uses_xyce_damped_solver,
                );
                let mut solved_weighted_correction_norm = None;
                let solve_result: Result<(), rspice_matrix::SolverError> = if let Some(direct_rhs) =
                    direct_correction_rhs
                {
                    if prefer_dense_solver {
                        matrix.solve_dense(direct_rhs).map(|solution| {
                            linear_solution = solution;
                        })
                    } else {
                        matrix.solve_into(direct_rhs, &mut linear_solution)
                    }
                    .map(|()| {
                        solved_weighted_correction_norm = self
                            .transient_newton_weighted_correction_norm(
                                &linear_solution,
                                transient_newton_update_weights.as_deref(),
                            );
                        for (correction, &iterate) in linear_solution.iter_mut().zip(&new_solution)
                        {
                            *correction += iterate;
                        }
                    })
                } else if solve_produces_correction {
                    matrix
                        .correction_rhs_into(&rhs, &new_solution, &mut correction_rhs)
                        .and_then(|()| {
                            if uses_inductor_correction {
                                circuit.stabilize_inductor_transient_correction_rhs(
                                    &mut correction_rhs,
                                    &new_solution,
                                    dt,
                                    &coeff,
                                );
                            }
                            if circuit.has_xyce_core_inductors() {
                                circuit.overwrite_xyce_core_transient_correction_rhs(
                                    &mut correction_rhs,
                                    xyce_one_step_order2,
                                    xyce_static_history.as_deref(),
                                );
                            }
                            if prefer_dense_solver {
                                linear_solution = matrix.solve_dense(&correction_rhs)?;
                            } else {
                                matrix.solve_into(&correction_rhs, &mut linear_solution)?;
                            }
                            solved_weighted_correction_norm = self
                                .transient_newton_weighted_correction_norm(
                                    &linear_solution,
                                    transient_newton_update_weights.as_deref(),
                                );
                            for (correction, &iterate) in
                                linear_solution.iter_mut().zip(&new_solution)
                            {
                                *correction += iterate;
                            }
                            Ok(())
                        })
                } else if prefer_dense_solver {
                    matrix.solve_dense(&rhs).map(|solution| {
                        linear_solution = solution;
                    })
                } else {
                    matrix.solve_into(&rhs, &mut linear_solution)
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
                            "Slow transient Newton solve at t={:.6e}, dt={:.3e}, step_attempt={}, elapsed={:.3?}",
                            t,
                            dt,
                            total_step_attempts,
                            newton_solve_elapsed,
                        );
                    }
                }

                let postsolve_phase_start = DiagnosticTimer::start(diagnostic_timing_enabled);
                // One additional Newton linear solve was attempted even when
                // factorization fails, matching DampedNewton's terminal
                // one-based `nlStep_` accounting.
                nonlinear_iterations = _iter.saturating_add(1);
                match solve_result {
                    Ok(()) => {
                        let sol = &mut linear_solution;
                        had_solver_candidate = true;
                        // Sanity check: detect and handle NaN/Inf/excessive values.
                        // IMPORTANT: Preserve the newest valid candidate when possible.
                        // If we keep the previous timestep guess here, force-accept can
                        // propagate a stale state and flatten non-source traces.
                        let mut has_bad_values = false;
                        let mut had_nonfinite_solution = false;
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
                            } else if circuit.has_xyce_core_inductors() {
                                MAX_XYCE_CORE_BRANCH_STATE_MAGNITUDE
                            } else {
                                MAX_BRANCH_STATE_MAGNITUDE
                            };
                            if !v.is_finite() {
                                had_nonfinite_solution = true;
                                if !logged_divergence {
                                    log::debug!(
                                        "Transient: Newton divergence at t={:.3e}s, state {}: {:.3e} - reducing timestep",
                                        step_time,
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
                                        step_time,
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

                        // Evaluate Xyce's update norm before RSpice's optional
                        // component-wise recovery clipping. A clipped step is
                        // a globalization aid, not evidence that the raw
                        // Newton correction is small. Native/ngspice modes
                        // retain their established post-limiting nodal test.
                        // NOX evaluates the unsolved predictor as iteration
                        // zero; this loop's first post-solve candidate is
                        // therefore nonlinear iteration one.
                        let raw_weighted_update_norm = select_xyce_transient_update_norm(
                            uses_xyce_damped_solver,
                            solved_weighted_correction_norm,
                            || {
                                self.transient_newton_weighted_update_norm(
                                    &new_solution,
                                    sol,
                                    transient_newton_update_weights.as_deref(),
                                )
                            },
                        );

                        if conservative_limiting_active && !junction_owns_steps {
                            // Trust-region damping is critical for stiff semiconductor
                            // nonlinearities, but it should not throttle linear decks or
                            // break ideal voltage-source equations by independently clipping
                            // their driven output nodes after each linear solve.
                            let damped = Self::limit_transient_node_voltage_updates(
                                sol,
                                &new_solution,
                                num_nodes,
                                iteration_delta_limit,
                                &force_accept_protected_nodes,
                            );
                            if damped {
                                circuit.enforce_ideal_voltage_constraints(sol, step_time)?;
                            }
                            Self::clip_ideal_output_common_modes(
                                &solution,
                                sol,
                                iteration_delta_limit,
                                &ideal_output_pairs,
                            );
                        }

                        // If this Newton step was numerically bad, keep the sanitized
                        // candidate and continue Newton iterations.
                        if has_bad_values {
                            if _iter == 0 && globalization_active {
                                let seed_start = DiagnosticTimer::start(diagnostic_timing_enabled);
                                last_stamped_iterate.clone_from(&new_solution);
                                last_stamped_merit = self
                                    .residual_inf_norm(
                                        &circuit,
                                        &mut matrix,
                                        &last_stamped_iterate,
                                        &rhs,
                                    )
                                    .unwrap_or(Value::INFINITY);
                                capture_transient_merit_rollback(
                                    &circuit,
                                    &vbic_snapshot_cache,
                                    classic_mos_stamp_cache.is_some(),
                                    &mut last_stamped_rollback,
                                );
                                total_merit_nanos += seed_start.elapsed().as_nanos();
                            }
                            if uses_xyce_nox_status {
                                xyce_weighted_update_norm =
                                    xyce_nox_recovered_update_norm(had_nonfinite_solution, || {
                                        self.transient_newton_weighted_update_norm(
                                            &new_solution,
                                            sol,
                                            transient_newton_update_weights.as_deref(),
                                        )
                                    });
                            }
                            new_solution.copy_from_slice(sol);
                            nonlinear_state_matches_new_solution = false;
                            continue;
                        }

                        if uses_xyce_damped_solver {
                            // DampedNewton checks the corrected candidate, not
                            // the predictor, and obtains its RHS norms from a
                            // fresh full Newton-mode assembly at that
                            // candidate. Keep this restamp separate from the
                            // StaticProbe acceptance helper: generated devices
                            // must observe the same evaluation mode as the
                            // canonical Xyce Newton loop.
                            new_solution.copy_from_slice(sol);
                            self.stamp_transient_system_with_generated_mode(
                                &mut circuit,
                                &mut matrix,
                                &mut rhs,
                                &new_solution,
                                step_time,
                                dt,
                                &transient_system_context,
                                &mut vbic_snapshot_cache,
                                VbicCachedSnapshotReuse::SeedOnly,
                                true,
                                0.0,
                                crate::device::veriloga_builtins::GeneratedEvaluationMode::NewtonLimited,
                            )?;
                            nonlinear_state_matches_new_solution = true;
                            let (residual_inf_norm, residual_l2_norm) = if uses_direct_xyce_dae {
                                let vectors = xyce_direct_vectors.as_mut().expect(
                                    "direct Xyce DAE vectors are allocated for the gated path",
                                );
                                circuit
                                    .load_direct_xyce_level2_core_dae(
                                        &new_solution,
                                        step_time,
                                        0.0,
                                        vectors,
                                    )
                                    .map_err(SimulationError::Circuit)?;
                                let previous_q =
                                    xyce_direct_accepted_q.as_deref().ok_or_else(|| {
                                        SimulationError::Circuit(
                                            "direct Xyce accepted Q history is missing".into(),
                                        )
                                    })?;
                                let order = if xyce_one_step_order2 {
                                    XyceOneStepOrder::Second
                                } else {
                                    XyceOneStepOrder::First
                                };
                                let direct_rhs = xyce_direct_workspace
                                    .as_mut()
                                    .expect(
                                        "direct Xyce DAE workspace is allocated for the gated path",
                                    )
                                    .form_correction_rhs(
                                        vectors,
                                        previous_q,
                                        xyce_direct_static_history.as_deref(),
                                        dt,
                                        order,
                                    )
                                    .map_err(|error| SimulationError::Circuit(error.to_string()))?;
                                direct_xyce_dae_norms(direct_rhs)?
                            } else {
                                matrix.raw_residual_norms(&new_solution, &rhs)?
                            };
                            let device_converged = core_trial_converged(&circuit)
                                && (!enforce_device_convergence
                                    || !circuit.has_nonlinear_devices()
                                    || circuit
                                        .nonlinear_converged(self.device_convergence_criteria()));
                            let behavioral_converged = circuit
                                .behavioral_linearizations_converged(
                                    &new_solution,
                                    step_time,
                                    self.voltage_reltol(),
                                    self.voltage_abstol(),
                                    self.current_abstol(),
                                )
                                .map_err(SimulationError::Circuit)?;
                            let decision = xyce_damped_status
                                .as_mut()
                                .expect("Xyce DampedNewton status is initialized for this path")
                                .evaluate(
                                    damped_status::XyceDampedSample {
                                        newton_step: _iter.saturating_add(1),
                                        residual_inf_norm,
                                        residual_l2_norm,
                                        weighted_update_norm: raw_weighted_update_norm
                                            .unwrap_or(Value::INFINITY),
                                        device_converged,
                                        inner_device_converged: true,
                                        linear_solve_ok: true,
                                        linear_solve_nan: false,
                                    },
                                    self.transient_nonlinear_deltaxtol(),
                                    self.transient_nonlinear_rhstol(),
                                );
                            match decision {
                                damped_status::XyceDampedDecision::Accepted { .. }
                                    if behavioral_converged =>
                                {
                                    converged = true;
                                    total_postsolve_nanos +=
                                        postsolve_phase_start.elapsed().as_nanos();
                                    break;
                                }
                                damped_status::XyceDampedDecision::Accepted { .. }
                                    if _iter.saturating_add(1) >= tran_max_iterations =>
                                {
                                    total_postsolve_nanos +=
                                        postsolve_phase_start.elapsed().as_nanos();
                                    break;
                                }
                                damped_status::XyceDampedDecision::Failed { .. } => {
                                    total_postsolve_nanos +=
                                        postsolve_phase_start.elapsed().as_nanos();
                                    break;
                                }
                                damped_status::XyceDampedDecision::Accepted { .. }
                                | damped_status::XyceDampedDecision::Continue => {}
                            }
                            total_postsolve_nanos += postsolve_phase_start.elapsed().as_nanos();
                            continue;
                        }

                        if is_strictly_linear_transient && !uses_inductor_correction {
                            // An absolute solve is exact for an ordinary linear
                            // deck. Correction-form inductive solves still run
                            // the normal fixed-point/status check so a second
                            // small correction can remove forward error from an
                            // ill-conditioned companion row.
                            new_solution.copy_from_slice(sol);
                            converged = true;
                            break;
                        }

                        if uses_xyce_nox_status {
                            // Xyce evaluates the newly solved candidate on the
                            // next status-test pass, after restamping its true
                            // nonlinear residual. This is nonlinear iteration
                            // `_iter + 1`, including the MAXSTEP candidate.
                            new_solution.copy_from_slice(sol);
                            nonlinear_state_matches_new_solution = false;
                            xyce_weighted_update_norm = raw_weighted_update_norm;
                            total_postsolve_nanos += postsolve_phase_start.elapsed().as_nanos();
                            continue;
                        }

                        let update_converged_for_acceptance = self
                            .transient_newton_update_convergence_met(
                                &new_solution,
                                sol,
                                num_nodes,
                                transient_newton_update_weights.as_deref(),
                                _iter.saturating_add(1),
                            );
                        if _iter == 0 && globalization_active && !update_converged_for_acceptance {
                            // The circuit still holds the first stamped
                            // iterate here. Capture its rollback state only
                            // after the solve proves that another Newton step
                            // is required.
                            let seed_start = DiagnosticTimer::start(diagnostic_timing_enabled);
                            last_stamped_iterate.clone_from(&new_solution);
                            last_stamped_merit = self
                                .residual_inf_norm(
                                    &circuit,
                                    &mut matrix,
                                    &last_stamped_iterate,
                                    &rhs,
                                )
                                .unwrap_or(Value::INFINITY);
                            capture_transient_merit_rollback(
                                &circuit,
                                &vbic_snapshot_cache,
                                classic_mos_stamp_cache.is_some(),
                                &mut last_stamped_rollback,
                            );
                            total_merit_nanos += seed_start.elapsed().as_nanos();
                        }
                        // CRITICAL: Update new_solution BEFORE checking device convergence
                        // Otherwise, BJT vbe/vbc are based on old guess, not new solve
                        new_solution.copy_from_slice(sol);
                        nonlinear_state_matches_new_solution = false;

                        // Update nonlinear device state to new solution for accurate convergence check
                        let postsolve_update_start =
                            DiagnosticTimer::start(diagnostic_timing_enabled);
                        let mut fused_classic_mos_device_converged = None;
                        if circuit.has_nonlinear_devices() && !nonlinear_state_matches_new_solution
                        {
                            if let Some(classic_mos_cache) = classic_mos_stamp_cache.as_ref()
                                && (update_converged_for_acceptance
                                    || circuit.mosfets.len()
                                        >= residual::CLASSIC_MOS_CACHED_CONSTANTS_THRESHOLD)
                            {
                                if update_converged_for_acceptance {
                                    let postsolve_companion_coeff = if xyce_one_step {
                                        CompanionCoefficients::backward_euler()
                                    } else {
                                        coeff
                                    };
                                    let evaluation = self.update_classic_mos_with_companion_terms(
                                        &mut circuit,
                                        &new_solution,
                                        classic_mos_cache.device_constants(),
                                        &postsolve_companion_coeff,
                                        dt,
                                        &mosfet_history,
                                        suppress_gate_charge,
                                        &mut mosfet_companion_terms_scratch,
                                        &mut mosfet_companion_charges_scratch,
                                        &mut mosfet_caps_scratch,
                                        capture_classic_mos_candidate_static_terms
                                            .then_some(&mut mosfet_static_terms_scratch),
                                        classic_mos_truncation_context.as_ref(),
                                        enforce_device_convergence
                                            .then(|| self.device_convergence_criteria()),
                                    )?;
                                    cached_mosfet_truncation_limit = evaluation.truncation_limit;
                                    fused_classic_mos_device_converged =
                                        Some(evaluation.all_devices_converged);
                                    // The charge kernel uses canonical raw branch arithmetic
                                    // and the accepted Meyer limiter state. Its companion/LTE
                                    // data remains exact even when static channel limiting
                                    // forbids reuse of the cached conduction Jacobian.
                                    mosfet_companion_terms_valid = mosfet_companion_terms_scratch
                                        .len()
                                        == circuit.mosfets.devices.len()
                                        && mosfet_companion_charges_scratch.len()
                                            == circuit.mosfets.devices.len()
                                        && mosfet_caps_scratch.len()
                                            == circuit.mosfets.devices.len();
                                    cached_mosfet_truncation_limit_valid =
                                        mosfet_companion_terms_valid
                                            && evaluation.truncation_evaluated;
                                } else {
                                    if reuse_sequential_classic_mos_newton_terms {
                                        self.update_classic_mos_with_companion_terms_only(
                                            &mut circuit,
                                            &new_solution,
                                            classic_mos_cache.device_constants(),
                                            &coeff,
                                            dt,
                                            &mosfet_history,
                                            suppress_gate_charge,
                                            &mut mosfet_companion_terms_scratch,
                                            &mut mosfet_static_terms_scratch,
                                        )?;
                                        mosfet_companion_terms_valid =
                                            mosfet_companion_terms_scratch.len()
                                                == circuit.mosfets.devices.len();
                                    } else {
                                        self.update_classic_mos_nonlinear_devices(
                                            &mut circuit,
                                            &new_solution,
                                            classic_mos_cache.device_constants(),
                                        )?;
                                        mosfet_companion_terms_valid = false;
                                    }
                                    cached_mosfet_truncation_limit_valid = false;
                                }
                            } else {
                                self.update_transient_nonlinear_devices(
                                    &mut circuit,
                                    &new_solution,
                                )?;
                                mosfet_companion_terms_valid = false;
                                cached_mosfet_truncation_limit_valid = false;
                            }
                            nonlinear_state_matches_new_solution = true;
                        }
                        total_postsolve_update_nanos += postsolve_update_start.elapsed().as_nanos();

                        let postsolve_convergence_start =
                            DiagnosticTimer::start(diagnostic_timing_enabled);
                        let mut device_converged = core_trial_converged(&circuit)
                            && (!enforce_device_convergence
                                || !circuit.has_nonlinear_devices()
                                || fused_classic_mos_device_converged.unwrap_or_else(|| {
                                    circuit.nonlinear_converged(self.device_convergence_criteria())
                                }));
                        // This is RSpice's consistency check for its internal
                        // behavioral-expression linearization, not a device
                        // `isConverged` flag controlled by Xyce's
                        // ENFORCEDEVICECONV status test.
                        let mut behavioral_converged = circuit
                            .behavioral_linearizations_converged(
                                &new_solution,
                                step_time,
                                self.voltage_reltol(),
                                self.voltage_abstol(),
                                self.current_abstol(),
                            )
                            .map_err(SimulationError::Circuit)?;
                        let mut residual_converged_for_acceptance = false;
                        total_postsolve_convergence_nanos +=
                            postsolve_convergence_start.elapsed().as_nanos();
                        if update_converged_for_acceptance
                            && device_converged
                            && behavioral_converged
                        {
                            let residual_proof_preserves_device_state =
                                circuit.has_classic_mos_only_transient_nonlinearity();
                            // A direct solve only makes the candidate satisfy
                            // the system linearized at the previous iterate.
                            // Acceptance must use the true nonlinear residual:
                            // restamp every otherwise-converged candidate at
                            // its own state before applying the solver's
                            // residual criterion.
                            let postsolve_residual_start =
                                DiagnosticTimer::start(diagnostic_timing_enabled);
                            residual_converged_for_acceptance = self
                                .transient_nonlinear_residual_converged(
                                    &mut circuit,
                                    &mut matrix,
                                    &mut rhs,
                                    &new_solution,
                                    step_time,
                                    dt,
                                    &residual::TransientSystemContext {
                                        coeff: &coeff,
                                        xyce_one_step,
                                        xyce_one_step_order2,
                                        xyce_static_history: xyce_static_history.as_deref(),
                                        bsim4_trnqs_coeff: &bsim4_trnqs_coeff,
                                        bjt_history: &bjt_history,
                                        jfet_history: &jfet_history,
                                        diode_history: &diode_history,
                                        diode_companion_slots: &diode_companion_slots,
                                        diode_attempt_cache: diode_stamp_cache.as_ref(),
                                        mosfet_history: &mosfet_history,
                                        mosfet_companion_slots: &mosfet_companion_slots,
                                        vdmos_history: &vdmos_history,
                                        vdmos_companion_slots: &vdmos_companion_slots,
                                        b3soi_history: &b3soi_history,
                                        b3soi_zero_first_transient_charge_derivative:
                                            b3soi_first_transient_handoff
                                                && accepted_interval_count == 0
                                                && _iter == 0,
                                        bsim3_history: &bsim3_history,
                                        bsim4_history: &bsim4_history,
                                        ekv26_history: &ekv26_history,
                                        suppress_gate_charge,
                                        baseline_diag_gmin: transient_baseline_diag_gmin,
                                        tline_dc_refs: &tline_dc_refs,
                                        coupled_tline_refs: &coupled_tline_refs,
                                        analysis_initial_step,
                                        analysis_final_step,
                                    },
                                    classic_mos_stamp_cache.as_ref(),
                                    &mut vbic_snapshot_cache,
                                    mosfet_companion_terms_valid
                                        .then_some(mosfet_companion_terms_scratch.as_slice()),
                                    capture_classic_mos_candidate_static_terms
                                        .then_some(mosfet_static_terms_scratch.as_slice()),
                                    if classic_mos_stamp_cache.is_some() && !suppress_gate_charge {
                                        Some(&mut mosfet_caps_scratch)
                                    } else {
                                        None
                                    },
                                    classic_mos_residual_scratch
                                        .as_mut()
                                        .map(|(row_ax, row_rhs)| (row_ax, row_rhs)),
                                )?;
                            mosfet_caps_valid = residual_converged_for_acceptance
                                && classic_mos_stamp_cache.is_some()
                                && mosfet_caps_scratch.len() == circuit.mosfets.devices.len();
                            mosfet_companion_terms_valid &= residual_converged_for_acceptance;
                            cached_mosfet_truncation_limit_valid &=
                                residual_converged_for_acceptance;
                            total_postsolve_residual_nanos +=
                                postsolve_residual_start.elapsed().as_nanos();
                            // The proof restamp refreshes nonlinear, generated,
                            // behavioral, and XSPICE trial state at the exact
                            // candidate. Re-evaluate convergence afterward so
                            // limiter state advanced by that refresh cannot be
                            // accepted through stale pre-restamp booleans.
                            let postsolve_convergence_start =
                                DiagnosticTimer::start(diagnostic_timing_enabled);
                            if !residual_proof_preserves_device_state {
                                device_converged = core_trial_converged(&circuit)
                                    && (!enforce_device_convergence
                                        || !circuit.has_nonlinear_devices()
                                        || circuit.nonlinear_converged(
                                            self.device_convergence_criteria(),
                                        ));
                                behavioral_converged = circuit
                                    .behavioral_linearizations_converged(
                                        &new_solution,
                                        step_time,
                                        self.voltage_reltol(),
                                        self.voltage_abstol(),
                                        self.current_abstol(),
                                    )
                                    .map_err(SimulationError::Circuit)?;
                            }
                            if _iter == 0
                                && globalization_active
                                && !residual_converged_for_acceptance
                            {
                                // The restamp replaced the first iterate's
                                // linearization. If the candidate still needs
                                // another Newton correction, make this
                                // restamped candidate the globalization base;
                                // accepted one-solve points avoid both merit
                                // matrix-vector products entirely.
                                last_stamped_iterate.clone_from(&new_solution);
                                last_stamped_merit = self
                                    .residual_inf_norm(
                                        &circuit,
                                        &mut matrix,
                                        &last_stamped_iterate,
                                        &rhs,
                                    )
                                    .unwrap_or(Value::INFINITY);
                                capture_transient_merit_rollback(
                                    &circuit,
                                    &vbic_snapshot_cache,
                                    classic_mos_stamp_cache.is_some(),
                                    &mut last_stamped_rollback,
                                );
                            }
                            total_postsolve_convergence_nanos +=
                                postsolve_convergence_start.elapsed().as_nanos();
                        }
                        total_postsolve_nanos += postsolve_phase_start.elapsed().as_nanos();

                        if update_converged_for_acceptance
                            && device_converged
                            && behavioral_converged
                            && residual_converged_for_acceptance
                        {
                            converged = true;
                            break;
                        }
                    }
                    Err(e) => {
                        had_solver_candidate = false;
                        log::debug!(
                            "Transient solve failed at t={:.6e}, dt={:.3e}: {}",
                            step_time,
                            dt,
                            e
                        );
                        break;
                    }
                }
            }

            let postloop_phase_start = DiagnosticTimer::start(diagnostic_timing_enabled);
            if !converged {
                // The retry count describes rejected timestep attempts, not
                // every failed pass through plain Newton. GMIN continuation
                // can still recover this same candidate and send it through
                // the ordinary LTE decision below, so retain only a local
                // ordinal until every in-step nonlinear rescue has failed.
                let nonlinear_failure_ordinal = retry_count.saturating_add(1);

                // Diagnostic logging for debugging convergence issues
                static CONV_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                let count = CONV_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if count < 10 || (t > 9.5e-8 && dt < 1.0e-15) {
                    // Check what specifically didn't converge
                    let v_conv =
                        self.node_voltage_convergence_met(&solution, &new_solution, num_nodes);
                    let d_conv = core_trial_converged(&circuit)
                        && (!enforce_device_convergence
                            || !circuit.has_nonlinear_devices()
                            || circuit.nonlinear_converged(self.device_convergence_criteria()));
                    let r_conv = self.transient_residual_convergence_met(
                        &circuit,
                        &mut matrix,
                        &new_solution,
                        &rhs,
                    );
                    let max_dv = Self::max_abs_delta_prefix(&solution, &new_solution, num_nodes);
                    let update_norm = self
                        .transient_newton_weighted_update_norm(
                            &solution,
                            &new_solution,
                            transient_newton_update_weights.as_deref(),
                        )
                        .unwrap_or(Value::NAN);
                    log::warn!(
                        "Newton non-converge at t={:.6e}, dt={:.3e}: voltage_conv={}, device_conv={}, residual_conv={}, update_norm={:.3e}, max_dv={:.3e}, iter={}",
                        t,
                        dt,
                        v_conv,
                        d_conv,
                        r_conv,
                        update_norm,
                        max_dv,
                        total_step_attempts
                    );
                }

                // Gmin-continuation rescue: a knife edge in the static
                // nonlinearity repeats at every dt, so the cut cascade
                // cannot fix it. Deform the step's system with diagonal
                // shunts, converge, and track the solution back to the
                // genuine system (transient/rescue.rs). A success flows
                // into the normal LTE acceptance machinery below.
                // GMIN continuation deforms only the nodal equations.  A
                // pure Xyce LEVEL=1 Core deck has no semiconductor junction
                // to regularize, so applying the deformation would accept a
                // different magnetic branch and leave a persistent endpoint
                // error.  LEVEL=2 Core retains the general rescue path because
                // its constitutive trial can still require globalization.
                let xyce_level1_core_only = self.config.spice_dialect == SpiceDialect::Xyce
                    && circuit.has_only_xyce_core_inductors()
                    && !circuit.has_xyce_core_level2();
                if nonlinear_failure_ordinal >= TRANSIENT_GMIN_RESCUE_MIN_RETRIES
                    // Xyce returns a failed transient DampedNewton/NOX solve
                    // directly to StepErrorControl. RSpice's post-solver GMIN
                    // deformation is not part of that path; under ERROPTION=1
                    // it would additionally replace the exact iteration count
                    // that owns timestep control. Preserve Xyce's /8 retry.
                    && allows_postsolve_gmin_rescue(xyce_iteration_error_control)
                    && !xyce_level1_core_only
                    && !uses_direct_xyce_dae
                    && circuit.has_nonlinear_devices()
                    && let Some(rescued) = self.rescue_transient_step_with_gmin_continuation(
                        &mut circuit,
                        &mut matrix,
                        &mut rhs,
                        &solution,
                        step_time,
                        dt,
                        &residual::TransientSystemContext {
                            coeff: &coeff,
                            xyce_one_step,
                            xyce_one_step_order2,
                            xyce_static_history: xyce_static_history.as_deref(),
                            bsim4_trnqs_coeff: &bsim4_trnqs_coeff,
                            bjt_history: &bjt_history,
                            jfet_history: &jfet_history,
                            diode_history: &diode_history,
                            diode_companion_slots: &diode_companion_slots,
                            diode_attempt_cache: diode_stamp_cache.as_ref(),
                            mosfet_history: &mosfet_history,
                            mosfet_companion_slots: &mosfet_companion_slots,
                            vdmos_history: &vdmos_history,
                            vdmos_companion_slots: &vdmos_companion_slots,
                            b3soi_history: &b3soi_history,
                            b3soi_zero_first_transient_charge_derivative: false,
                            bsim3_history: &bsim3_history,
                            bsim4_history: &bsim4_history,
                            ekv26_history: &ekv26_history,
                            suppress_gate_charge,
                            baseline_diag_gmin: transient_baseline_diag_gmin,
                            tline_dc_refs: &tline_dc_refs,
                            coupled_tline_refs: &coupled_tline_refs,
                            analysis_initial_step,
                            analysis_final_step,
                        },
                        &mut vbic_snapshot_cache,
                    )?
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
                            nonlinear_failure_ordinal,
                        );
                    }
                    new_solution = rescued;
                    nonlinear_state_matches_new_solution = true;
                    had_solver_candidate = true;
                    converged = true;
                }
            }

            if !converged {
                retry_count = retry_count.saturating_add(1);
                if lte_estimator.uses_accepted_solution_reference() {
                    xyce_step_failure_count = xyce_step_failure_count.saturating_add(1);
                }
                self.record_convergence(|quality| quality.record_timestep_reduction());
                let recovery_order = if lte_estimator.uses_accepted_solution_reference() {
                    xyce_min_order
                } else {
                    native_order_after_restart(current_method)
                };
                trap_order = xyce_rejected_attempt_order(
                    xyce_iteration_error_control,
                    step_trap_order,
                    recovery_order,
                );
                total_failed_attempts += 1;
                if self.node_voltage_convergence_met(&solution, &new_solution, num_nodes) {
                    // Voltage settled but a device/residual criterion held the
                    // point back — the interesting bucket for criteria tuning.
                    if core_trial_converged(&circuit)
                        && (!enforce_device_convergence
                            || !circuit.has_nonlinear_devices()
                            || circuit.nonlinear_converged(self.device_convergence_criteria()))
                    {
                        failed_residual_only += 1;
                    } else {
                        failed_device_conv += 1;
                    }
                } else {
                    failed_voltage_conv += 1;
                }
                // Diagnostic logging for debugging timestep issues
                if total_step_attempts < 100 || total_step_attempts % 10000 == 0 {
                    log::debug!(
                        "Newton non-convergence at t={:.3e}s, step_attempt={}, dt={:.3e}s, reducing to {:.3e}s",
                        t,
                        total_step_attempts,
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
                    // A paired reference schedule may request one hidden
                    // trial at the controller's larger proposal before the
                    // recorded accepted interval. A failed hidden trial is
                    // expected: restore its state and land on the scheduled
                    // interval on the next pass instead of consuming the
                    // locked retry budget against the same hidden candidate.
                    if locked_replay_hidden_attempt {
                        if let Some(&scheduled_dt) = locked_step_sizes
                            .as_ref()
                            .and_then(|steps| steps.get(locked_cursor))
                            && scheduled_dt.is_finite()
                            && scheduled_dt > 0.0
                        {
                            timestep.force_step(scheduled_dt);
                        }
                        restore_rejected_transient_nonlinear_state!();
                        total_postloop_nanos += postloop_phase_start.elapsed().as_nanos();
                        continue;
                    }
                    if retry_count >= LOCKED_MAX_RETRIES {
                        log::error!(
                            "Grid-locked step to t={:.12e}s (dt={:.3e}) failed Newton after {} retries",
                            step_time,
                            dt,
                            retry_count
                        );
                        return Err(SimulationError::ConvergenceFailed(total_step_attempts));
                    }
                    restore_rejected_transient_nonlinear_state!();
                    total_postloop_nanos += postloop_phase_start.elapsed().as_nanos();
                    continue;
                }

                // Match ngspice's non-convergence recovery: retry at one eighth
                // of the rejected timestep, unless a force-accept cooldown is
                // temporarily holding dt steady to avoid ping-pong.
                if xyce_iteration_error_control {
                    timestep.force_step(xyce_iteration_retry_timestep(
                        dt,
                        timestep.hard_min_dt(),
                        max_step,
                    ));
                } else if force_accept_cooldown > 0 {
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

                // Fail when nonlinear recovery is exhausted:
                // - After MAX_RETRIES attempts (regardless of timestep state), OR
                // - At minimum timestep AND at least MIN_RETRIES_AT_MIN have been tried
                // A rejected Newton iterate is never a publishable circuit solution.
                let at_min_dt = if xyce_iteration_error_control {
                    timestep.is_at_minimum()
                } else {
                    Self::is_at_effective_retry_minimum(&timestep, legacy_bjt_retry_floor_dt)
                };
                let exhausted_retries = retry_count >= MAX_RETRIES;
                let exhausted_at_min = at_min_dt && retry_count >= MIN_RETRIES_AT_MINIMUM_TIMESTEP;
                if exhausted_retries || exhausted_at_min {
                    log::error!(
                        "Transient Newton recovery exhausted at t={:.12e}s (dt={:.3e}, retries={})",
                        step_time,
                        dt,
                        retry_count
                    );
                    restore_rejected_transient_nonlinear_state!();
                    return Err(SimulationError::ConvergenceFailed(total_step_attempts));
                }
                restore_rejected_transient_nonlinear_state!();
                rejected_attempt_nonlinear_state_scratch = rejected_attempt_nonlinear_state.take();
                total_postloop_nanos += postloop_phase_start.elapsed().as_nanos();
                continue;
            }

            total_postloop_nanos += postloop_phase_start.elapsed().as_nanos();
            let truncation_phase_start = DiagnosticTimer::start(diagnostic_timing_enabled);
            let first_accepted_transient_step = analysis_first_step_pending
                // Post-livelock-restart warmup: the re-seeded histories need
                // two clean accepted points before the truncation estimators
                // can difference them meaningfully.
                || lte_warmup_skips > 0;
            // Xyce OneStep/Gear12 uses ck*WRMS(x_candidate-x_predictor) as
            // its sole compact-device LTE authority.  Its DAE-Q norm is
            // calculated for diagnostics but is not returned by
            // DataStore::WRMS_errorNorm, and it has no ngspice CKTterr walk.
            // Keep every ngspice compact-charge controller out of the Xyce
            // accepted-solution path; source/device maximum-step contracts
            // and transmission-line limits remain independent below.
            let use_ngspice_charge_truncation =
                Self::uses_ngspice_charge_truncation(&lte_estimator);
            let bjt_truncation_limit = if use_ngspice_charge_truncation
                && !linearized_startup_recovery_points
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
                    transient_lte_reltol,
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let capacitor_truncation_limit = if use_ngspice_charge_truncation
                && !first_accepted_transient_step
                && !circuit.capacitors.is_empty()
            {
                #[cfg(feature = "parallel")]
                let limit = if let Some(worker_count) = capacitor_truncation_parallel_workers {
                    self.capacitor_ngspice_truncation_limit_parallel(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        mosfet_history.accepted_dt_prev,
                        mosfet_history.accepted_dt_prev_prev,
                        transient_lte_reltol,
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                        worker_count,
                        &mut capacitor_accepted_states_scratch,
                    )
                } else {
                    Self::capacitor_ngspice_truncation_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        mosfet_history.accepted_dt_prev,
                        mosfet_history.accepted_dt_prev_prev,
                        transient_lte_reltol,
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                        Some(&mut capacitor_accepted_states_scratch),
                    )
                };
                #[cfg(not(feature = "parallel"))]
                let limit = Self::capacitor_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    mosfet_history.accepted_dt_prev,
                    mosfet_history.accepted_dt_prev_prev,
                    transient_lte_reltol,
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                    Some(&mut capacitor_accepted_states_scratch),
                );
                capacitor_accepted_states_valid =
                    capacitor_accepted_states_scratch.len() == circuit.capacitors.stamps.len();
                limit.filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let inductor_truncation_limit = if use_ngspice_charge_truncation
                && !first_accepted_transient_step
                && !circuit.inductors.is_empty()
            {
                Self::inductor_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    mosfet_history.accepted_dt_prev,
                    mosfet_history.accepted_dt_prev_prev,
                    transient_lte_reltol,
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let jfet_truncation_limit = if use_ngspice_charge_truncation
                && !first_accepted_transient_step
                && !circuit.jfets.is_empty()
            {
                Self::jfet_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &jfet_history,
                    suppress_gate_charge,
                    transient_lte_reltol,
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let diode_truncation_limit = if use_ngspice_charge_truncation
                && !first_accepted_transient_step
                && !circuit.diodes.is_empty()
            {
                Self::diode_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &diode_history,
                    transient_lte_reltol,
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let mosfet_truncation_limit = if use_ngspice_charge_truncation
                && !first_accepted_transient_step
                && !suppress_gate_charge
                && !circuit.mosfets.is_empty()
            {
                let limit = if cached_mosfet_truncation_limit_valid {
                    cached_mosfet_truncation_limit
                } else if mosfet_companion_terms_valid
                    && mosfet_companion_charges_scratch.len() == circuit.mosfets.devices.len()
                    && mosfet_caps_scratch.len() == circuit.mosfets.devices.len()
                    && let Some(context) = classic_mos_truncation_context.as_ref()
                    && let Some(classic_mos_cache) = classic_mos_stamp_cache.as_ref()
                {
                    context.classic_mos_gate_limit_from_cached_charges(
                        classic_mos_cache.device_constants(),
                        &mosfet_companion_charges_scratch,
                        &mosfet_caps_scratch,
                        &mosfet_history,
                    )
                } else {
                    Self::mosfet_ngspice_truncation_limit(
                        &circuit,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        &mosfet_history,
                        transient_lte_reltol,
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                        Some((&mut mosfet_caps_scratch, mosfet_caps_valid)),
                    )
                }
                .filter(|limit| limit.is_finite() && *limit > 0.0);
                mosfet_caps_valid = mosfet_caps_scratch.len() == circuit.mosfets.devices.len();
                limit
            } else {
                None
            };
            let vdmos_truncation_limit = if use_ngspice_charge_truncation
                && !first_accepted_transient_step
                && !circuit.vdmoses.is_empty()
            {
                Self::vdmos_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &vdmos_history,
                    transient_lte_reltol,
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let b3soi_truncation_limit = if use_ngspice_charge_truncation
                && !first_accepted_transient_step
                && circuit.has_b3soi_devices()
            {
                Self::b3soi_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &b3soi_history,
                    transient_lte_reltol,
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let bsim3_truncation_limit = if use_ngspice_charge_truncation
                && !first_accepted_transient_step
                && circuit.has_bsim3v3_devices()
            {
                Self::bsim3_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &bsim3_history,
                    transient_lte_reltol,
                    self.current_abstol(),
                    self.charge_abstol(),
                    self.transient_trtol(),
                )
                .filter(|limit| limit.is_finite() && *limit > 0.0)
            } else {
                None
            };
            let bsim4_truncation_limit = if use_ngspice_charge_truncation
                && !first_accepted_transient_step
                && circuit.has_bsim4v8_devices()
            {
                Self::bsim4_ngspice_truncation_limit(
                    &circuit,
                    &new_solution,
                    current_method,
                    step_trap_order,
                    dt,
                    &bsim4_history,
                    transient_lte_reltol,
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
                                        Self::min_truncation_limit(
                                            capacitor_truncation_limit,
                                            inductor_truncation_limit,
                                        ),
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
                Self::ltra_candidate_truncation_limit(&circuit, &new_solution, step_time)
            } else {
                None
            };
            let activity_limit = if !first_accepted_transient_step
                && !lte_estimator.uses_accepted_solution_reference()
            {
                Self::nonlinear_terminal_activity_limit(
                    &nonlinear_terminal_solution_indices,
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
            let middle_phase_start = DiagnosticTimer::start(diagnostic_timing_enabled);

            if !xyce_iteration_error_control
                && locked_grid.is_none()
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
                            "Candidate truncation reject at t={:.6e}, dt={:.3e}, limit={:.3e}, cap={:?}, ind={:?}, bjt={:?}, jfet={:?}, dio={:?}, mos={:?}, vdmos={:?}, ltra={:?}, method={:?}, order={}",
                            t,
                            dt,
                            limit,
                            capacitor_truncation_limit,
                            inductor_truncation_limit,
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
                    self.record_convergence(|quality| quality.record_timestep_reduction());
                    // Match ngspice truncation retries: keep the current integration
                    // order and only reduce the timestep.
                    trap_order =
                        Self::trapezoidal_order_after_timestep_control_reject(step_trap_order);
                    timestep.force_step(retry_dt);
                    restore_rejected_transient_nonlinear_state!();
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
                    inductor_truncation_limit,
                    bjt_truncation_limit,
                    jfet_truncation_limit,
                    diode_truncation_limit,
                    mosfet_truncation_limit,
                    vdmos_truncation_limit,
                );
            let legacy_xyce_breakpoint_restart_controls_lte = lte_estimator
                .uses_accepted_solution_reference()
                && xyce_lte_restart_first_step
                && !self.config.transient_new_bp_stepping;
            let device_or_startup_controls_lte = first_accepted_transient_step
                || legacy_xyce_breakpoint_restart_controls_lte
                || (!lte_estimator.uses_accepted_solution_reference()
                    && (linearized_startup_recovery_points
                        || defer_voltage_lte_to_bjt_truncation
                        || defer_voltage_lte_to_jfet_truncation
                        || defer_voltage_lte_to_mosfet_truncation
                        || defer_voltage_lte_to_ngspice_device_truncation));
            let (lte, lte_accept) = if device_or_startup_controls_lte {
                // For first/startup recovery points and decks covered by
                // ngspice device-local truncation (CAPtrunc, MOStrunc,
                // BJTtrunc, generated compact-model truncation, etc.), a
                // converged Newton solution at the imposed dt is the
                // acceptance criterion.
                (0.0, true)
            } else {
                Self::estimate_transient_lte(
                    &circuit,
                    &new_solution,
                    lte_predicted_solution.as_deref(),
                    dt,
                    current_method,
                    step_trap_order,
                    is_strictly_linear_transient,
                    &lte_estimator,
                    &voltage_lte_excluded_nodes,
                    &xyce_lte_excluded_indices,
                )
            };
            // Xyce CONSTSTEP still evaluates LTE for integration-order
            // selection, but the estimate cannot reject or resize a
            // prescribed grid step.
            let iteration_control_accepts = xyce_iteration_step_accepts(
                nonlinear_iterations,
                self.config.transient_timeint_nlmax,
                self.config.transient_timesteps_reversal,
            );
            let accept = if locked_replay_hidden_attempt {
                false
            } else {
                locked_grid.is_some()
                    || if xyce_iteration_error_control {
                        iteration_control_accepts
                    } else {
                        lte_accept
                    }
            };
            let xyce_order_two_trial_eligible = lte_estimator.uses_accepted_solution_reference()
                && xyce_allows_order_two(xyce_max_order)
                && accept
                && !first_accepted_transient_step
                && !xyce_lte_restart_first_step
                && !at_breakpoint
                && step_trap_order == 1
                && matches!(
                    current_method,
                    IntegrationMethod::Trapezoidal
                        | IntegrationMethod::TrapGear
                        | IntegrationMethod::Gear2
                );
            let xyce_promotes_order_two = xyce_order_two_trial_eligible
                && (xyce_iteration_error_control
                    || lte_estimator.xyce_should_promote_order_two(lte));
            let xyce_history_order = if xyce_promotes_order_two {
                2
            } else {
                step_trap_order
            };
            let xyce_accepted_ratio_order = if xyce_order_two_trial_eligible {
                2
            } else {
                step_trap_order
            };
            let xyce_rejected_order = xyce_lte_recovery_order(
                match current_method {
                    IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => 1,
                    IntegrationMethod::Gear2 if retry_count > 0 || xyce_step_failure_count > 0 => 1,
                    _ => step_trap_order,
                },
                xyce_min_order,
            );
            let xyce_first_failure = xyce_step_failure_count == 0;
            let lte_scale = if xyce_iteration_error_control {
                xyce_iteration_step_scale(
                    nonlinear_iterations,
                    self.config.transient_timeint_nlmin,
                    self.config.transient_timeint_nlmax,
                )
            } else if first_accepted_transient_step
                || (is_strictly_linear_transient
                    && !lte_estimator.uses_accepted_solution_reference())
            {
                1.0
            } else if lte_estimator.uses_accepted_solution_reference() {
                if accept {
                    lte_estimator.xyce_accepted_step_scale(
                        lte,
                        current_method,
                        xyce_accepted_ratio_order,
                    )
                } else {
                    lte_estimator.xyce_rejected_step_scale(
                        lte,
                        current_method,
                        xyce_rejected_order,
                        xyce_first_failure,
                    )
                }
            } else {
                lte_estimator.recommend_scale(lte)
            };
            if !accept {
                retry_count += 1;
                if lte_estimator.uses_accepted_solution_reference() {
                    xyce_step_failure_count = xyce_step_failure_count.saturating_add(1);
                }
                self.record_convergence(|quality| quality.record_timestep_reduction());
                let recovery_order = if lte_estimator.uses_accepted_solution_reference() {
                    xyce_rejected_order
                } else {
                    Self::trapezoidal_order_after_timestep_control_reject(step_trap_order)
                };
                // ERROPTION=1 rejection is only a /8 retry: unlike LTE
                // recovery it neither rolls back history nor demotes order.
                trap_order = xyce_rejected_attempt_order(
                    xyce_iteration_error_control,
                    step_trap_order,
                    recovery_order,
                );
                if xyce_iteration_error_control {
                    // Xyce OneStep.C applies the exact /8 reversal retry and
                    // clamps only to the controller's machine floor and caps.
                    timestep.force_step(xyce_iteration_retry_timestep(
                        dt,
                        timestep.hard_min_dt(),
                        max_step,
                    ));
                } else if lte_estimator.uses_accepted_solution_reference() {
                    // Xyce-mode LTE is normalized against its own TIMEINT
                    // tolerance, whereas the legacy timestep controller has a
                    // fixed 1e-3 target. Apply the estimator's order-aware
                    // scale directly so a rejected Xyce step always shrinks.
                    timestep.force_step((dt * lte_scale).clamp(timestep.hard_min_dt(), max_step));
                } else {
                    timestep.adjust(lte / lte_scale);
                }
                if !xyce_iteration_error_control {
                    let clamped_retry_dt = Self::apply_retry_timestep_floor(
                        timestep.dt(),
                        legacy_bjt_retry_floor_dt,
                        dt,
                        max_step,
                    );
                    if clamped_retry_dt > timestep.dt() + 1e-30 {
                        timestep.force_step(clamped_retry_dt);
                    }
                }

                // Force accept when recovery is unlikely:
                // - After MAX_RETRIES attempts (regardless of timestep state), OR
                // - At minimum timestep AND at least MIN_RETRIES_AT_MIN have been tried
                // This prevents both infinite loops and force-accept floods
                let at_min_dt = if xyce_iteration_error_control {
                    timestep.is_at_minimum()
                } else {
                    Self::is_at_effective_retry_minimum(&timestep, legacy_bjt_retry_floor_dt)
                };
                let exhausted_retries = retry_count >= MAX_RETRIES;
                let exhausted_at_min = at_min_dt && retry_count >= MIN_RETRIES_AT_MINIMUM_TIMESTEP;
                let mut force_accepted_rejected_lte_step = false;

                if exhausted_retries || exhausted_at_min {
                    if lte_estimator.uses_accepted_solution_reference() {
                        log::error!(
                            "Xyce transient LTE recovery exhausted at t={:.12e}s (dt={:.3e}, retries={})",
                            step_time,
                            dt,
                            retry_count
                        );
                        restore_rejected_transient_nonlinear_state!();
                        return Err(SimulationError::ConvergenceFailed(total_step_attempts));
                    }
                    let bounded_force_candidate = Self::bounded_force_accept_candidate(
                        &circuit,
                        &solution,
                        &new_solution,
                        step_time,
                        num_nodes,
                        force_accept_delta_limit,
                        &force_accept_protected_nodes,
                        &ideal_output_pairs,
                    )?;
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
                            return Err(SimulationError::ConvergenceFailed(total_step_attempts));
                        }
                        restore_rejected_transient_nonlinear_state!();
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
                    force_accepted_rejected_lte_step = true;

                    new_solution = bounded_force_candidate;

                    if circuit.has_nonlinear_devices() {
                        self.update_transient_nonlinear_devices(&mut circuit, &new_solution)?;
                    }
                    reject_for_veriloga_event_refinement!(
                        &new_solution,
                        step_time,
                        middle_phase_start
                    );

                    t = step_time;
                    let scheduled_breakpoint = breakpoints.at_breakpoint(t);
                    let hit_breakpoint = accepted_step_hits_breakpoint(
                        landed_veriloga_event,
                        at_breakpoint,
                        scheduled_breakpoint,
                    );
                    if hit_breakpoint {
                        if scheduled_breakpoint && !landed_veriloga_event && !analysis_final_step {
                            t = breakpoints.snap_to_breakpoint(t);
                        }
                        let restart_dt = if landed_veriloga_event {
                            breakpoints.mark_external_breakpoint_solved(t, dt)
                        } else {
                            breakpoints.mark_breakpoint_solved(t)
                        };
                        timestep.force_step(restart_dt.min(timestep.dt()).min(max_step));
                    }

                    let method_after_step = current_integration_method(&trapgear);
                    let accepted_step_trap_order =
                        if native_predictor_local && current_method == IntegrationMethod::Gear2 {
                            2
                        } else {
                            1
                        };
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
                            transient_lte_reltol,
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
                            transient_lte_reltol,
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
                            transient_lte_reltol,
                            self.current_abstol(),
                            self.charge_abstol(),
                            self.transient_trtol(),
                            None,
                        )
                        .filter(|limit| limit.is_finite() && *limit > 0.0)
                    } else {
                        None
                    };
                    let force_accept_inductor_truncation_limit = if !circuit.inductors.is_empty() {
                        Self::inductor_ngspice_truncation_limit(
                            &circuit,
                            &new_solution,
                            current_method,
                            accepted_step_trap_order,
                            dt,
                            mosfet_history.accepted_dt_prev,
                            mosfet_history.accepted_dt_prev_prev,
                            transient_lte_reltol,
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
                            transient_lte_reltol,
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
                                transient_lte_reltol,
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
                            transient_lte_reltol,
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
                            transient_lte_reltol,
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
                            transient_lte_reltol,
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
                            transient_lte_reltol,
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
                                                Self::min_truncation_limit(
                                                    force_accept_capacitor_truncation_limit,
                                                    force_accept_inductor_truncation_limit,
                                                ),
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
                    let capture_xyce_static_history = self.config.spice_dialect
                        == SpiceDialect::Xyce
                        && !uses_direct_xyce_dae
                        && (xyce_one_step_order2 || xyce_promotes_order_two);
                    let mut xyce_static_history_candidate = None;
                    if capture_xyce_static_history && !circuit.has_xspice_devices() {
                        xyce_static_history_candidate = Some(self.capture_xyce_static_residual(
                            &mut circuit,
                            &mut matrix,
                            &new_solution,
                            t,
                            transient_baseline_diag_gmin,
                        )?);
                    }
                    if uses_direct_xyce_dae {
                        capture_direct_xyce_histories(
                            &circuit,
                            &new_solution,
                            t,
                            xyce_direct_vectors
                                .as_mut()
                                .expect("direct Xyce DAE vectors are allocated for the gated path"),
                            xyce_direct_q_candidate
                                .as_mut()
                                .expect("direct Xyce Q scratch is allocated for the gated path"),
                            xyce_direct_static_candidate.as_mut().expect(
                                "direct Xyce static scratch is allocated for the gated path",
                            ),
                        )
                        .map_err(SimulationError::Circuit)?;
                    }
                    self.update_reactive_history(
                        &mut circuit,
                        &new_solution,
                        t,
                        dt,
                        &coeff,
                        &bsim4_trnqs_coeff,
                        &mut bjt_history,
                        &mut jfet_history,
                        &mut diode_history,
                        &mut mosfet_history,
                        &mut vdmos_history,
                        &mut b3soi_history,
                        &mut bsim3_history,
                        &mut bsim4_history,
                        &mut ekv26_history,
                        xyce_one_step_order2,
                        Some(vbic_snapshot_cache.as_slice()),
                        None,
                        None,
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
                        &mut pending_dynamic_tline_breakpoints,
                    )?;
                    if circuit.has_xspice_devices() {
                        if capture_xyce_static_history {
                            circuit.evaluate_xspice_transient_timestep_with_coefficients(
                                t,
                                dt,
                                &new_solution,
                                &coeff,
                                xyce_one_step_order2,
                            );
                            circuit.accept_xspice_timestep();
                        } else {
                            circuit.accept_xspice_transient_timestep_with_coefficients(
                                t,
                                dt,
                                &new_solution,
                                &coeff,
                                xyce_one_step_order2,
                            );
                        }
                        circuit.project_xspice_voltage_outputs(&mut new_solution, num_nodes);
                        Self::collect_xspice_runtime_breakpoints(
                            &mut circuit,
                            &mut breakpoints,
                            tstop,
                        );
                        if capture_xyce_static_history {
                            xyce_static_history_candidate =
                                Some(self.capture_xyce_static_residual(
                                    &mut circuit,
                                    &mut matrix,
                                    &new_solution,
                                    t,
                                    transient_baseline_diag_gmin,
                                )?);
                        }
                    }
                    #[cfg(feature = "veriloga")]
                    if circuit.has_veriloga_devices() {
                        circuit
                            .evaluate_veriloga_timepoint(&new_solution)
                            .map_err(SimulationError::Circuit)?;
                    }
                    #[cfg(feature = "veriloga-builtins-base")]
                    if circuit.has_generated_veriloga_devices() {
                        circuit
                            .evaluate_generated_veriloga_timepoint(&mut matrix, &new_solution)
                            .map_err(SimulationError::Circuit)?;
                    }
                    if circuit.has_any_veriloga_devices() {
                        circuit
                            .accept_all_veriloga_timestep()
                            .map_err(SimulationError::Circuit)?;
                    }
                    pending_veriloga_event_time =
                        accepted_veriloga_event_time(&circuit, t, timestep.hard_min_dt())?;

                    // XSPICE voltage outputs are projected only when their
                    // model state is accepted. Commit controller histories
                    // afterward so their latest vector is the circuit
                    // solution that is checkpointed and starts the next step.
                    if fixed_method.is_none() {
                        if hit_breakpoint {
                            trapgear.restart_from(&new_solution);
                        } else {
                            trapgear.update(&new_solution, dt);
                        }
                    }
                    lte_estimator.record(&new_solution, dt);
                    if hit_breakpoint {
                        if lte_estimator.uses_accepted_solution_reference() {
                            lte_estimator.restart_history_from(&new_solution);
                        }
                        xyce_lte_restart_first_step = true;
                    }
                    lte_estimator.set_method_order(effective_method_order(
                        method_after_step,
                        accepted_step_trap_order,
                    ));

                    solution.clone_from(&new_solution);
                    circuit
                        .resistors
                        .advance_thermal_states(&solution, dt)
                        .map_err(SimulationError::Circuit)?;
                    if let Some(history) = xyce_static_history_candidate {
                        xyce_static_history = Some(history);
                    }
                    if uses_direct_xyce_dae {
                        xyce_direct_accepted_q
                            .as_mut()
                            .expect("direct Xyce Q history is allocated for the gated path")
                            .copy_from_slice(
                                xyce_direct_q_candidate.as_deref().expect(
                                    "direct Xyce Q scratch is allocated for the gated path",
                                ),
                            );
                        xyce_direct_static_history
                            .get_or_insert_with(|| vec![0.0; size])
                            .copy_from_slice(xyce_direct_static_candidate.as_deref().expect(
                                "direct Xyce static scratch is allocated for the gated path",
                            ));
                    }
                    retry_count = 0;
                    xyce_step_failure_count = 0;
                    accepted_interval_count =
                        accepted_interval_count.checked_add(1).ok_or_else(|| {
                            SimulationError::Circuit(
                                "transient accepted-interval count overflowed".to_string(),
                            )
                        })?;
                    let trajectory_point_count =
                        accepted_interval_count.checked_add(1).ok_or_else(|| {
                            SimulationError::Circuit(
                                "transient trajectory point count overflowed".to_string(),
                            )
                        })?;
                    Self::backfill_initial_linear_capacitor_branch_currents(
                        &mut result,
                        &circuit,
                        &derived_branch_currents,
                    );
                    retained_result_values = retained_result_values.saturating_add(
                        self.record_transient_solution_sample(
                            &mut result,
                            &mut circuit,
                            &solution,
                            num_nodes,
                            t,
                            dt,
                            &derived_branch_currents,
                            &bjt_history,
                            &diode_history,
                            record_device_op_traces,
                            &capture_plan,
                            trajectory_point_count,
                            abort,
                        )?,
                    );
                    if record_xspice_event_traces {
                        circuit.fill_xspice_digital_snapshot(&mut digital_snapshot);
                        retained_result_values =
                            retained_result_values.saturating_add(result.record_digital_snapshot(
                                t,
                                &digital_snapshot,
                                &mut digital_trace_indices,
                                &capture_plan.event_nodes,
                            ));
                        circuit.fill_xspice_real_snapshot(&mut real_snapshot);
                        retained_result_values =
                            retained_result_values.saturating_add(result.record_real_snapshot(
                                t,
                                &real_snapshot,
                                &mut real_trace_indices,
                                &capture_plan.event_nodes,
                            ));
                    }
                    self.ensure_transient_result_limits(&result, retained_result_values)?;
                    let next_force_dt = Self::force_accept_recovery_timestep(
                        dt,
                        timestep.preferred_min_dt(),
                        max_step,
                        force_accept_device_truncation_limit,
                    );
                    self.record_convergence(|quality| {
                        if quality.force_accepted_points == 0 {
                            log::warn!(
                                "transient point at t={t:e} accepted after exhausting LTE recovery; Newton converged, but waveform accuracy is not guaranteed at force-accepted points"
                            );
                        }
                        quality.record_force_accept(result.time.len().saturating_sub(1))
                    });
                    force_accept_cooldown = FORCE_ACCEPT_COOLDOWN_RETRIES;
                    timestep.force_step(next_force_dt);
                    if matches!(
                        current_method,
                        IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                    ) {
                        trap_order = 1;
                    }
                    analysis_first_step_pending = false;
                    if xyce_lte_restart_first_step && !hit_breakpoint {
                        xyce_lte_restart_first_step = false;
                    }
                    livelock_check!(dt);
                    debug_assert_eq!(retry_count, 0);
                    debug_assert_eq!(xyce_step_failure_count, 0);
                    debug_assert_eq!(stale_accept_count, 0);
                    debug_assert!(!circuit.xyce_core_trial_invalid());
                    let runtime_resume_blockers = Self::exact_integration_runtime_resume_blockers(
                        &circuit,
                        accepted_interval_count,
                    );
                    let damped_status_checkpoint = xyce_damped_status
                        .as_ref()
                        .map(|status| status.capture_accepted_boundary_checkpoint())
                        .transpose()
                        .map_err(SimulationError::Circuit)?;
                    self.capture_scheduled_checkpoint_if_due(
                        scheduled_checkpoint_times,
                        &mut scheduled_checkpoint_cursor,
                        t,
                        fingerprint,
                        &netlist_identity,
                        &restart_identity,
                        &simulation_identity,
                        &solution,
                        &circuit,
                        startup_mode,
                        max_step,
                        Some(ProposedIntegrationContinuation {
                            next_step: timestep.dt(),
                            breakpoint_span_ceiling: xyce_breakpoint_span_ceiling.ceiling(),
                            controller_max_step: timestep.max_dt(),
                            analysis_first_step_pending,
                            xyce_breakpoint_restart_pending: xyce_lte_restart_first_step,
                        }),
                        tstop,
                        &pending_dynamic_tline_breakpoints,
                        dynamic_tline_breakpoints_added,
                        &bjt_history,
                        &diode_history,
                        &vbic_snapshot_cache,
                        AcceptedIntegrationRuntimeCapture {
                            lte_estimator: &lte_estimator,
                            next_trap_order: trap_order,
                            trapgear: fixed_method.is_none().then(|| trapgear.capture_snapshot()),
                            xyce_static_residual: xyce_static_history.as_deref(),
                            direct_dae_accepted_q: xyce_direct_accepted_q.as_deref(),
                            direct_dae_static_residual: xyce_direct_static_history.as_deref(),
                            lte_warmup_skips,
                            force_accept_cooldown,
                            livelock_streak,
                            livelock_last_restart_time: livelock_last_restart_t,
                            accepted_interval_count,
                            damped_first_solver_call: xyce_damped_first_solver_call,
                            damped_status: damped_status_checkpoint,
                            retry_count,
                            xyce_step_failure_count,
                            stale_accept_count,
                            resume_blockers: &runtime_resume_blockers,
                        },
                        retained_result_values,
                        &mut retained_scheduled_checkpoint_values,
                        &mut scheduled_checkpoints,
                    )?;
                    reinitialize_xyce_breakpoint_histories!(hit_breakpoint, analysis_final_step);
                }
                if !force_accepted_rejected_lte_step {
                    restore_rejected_transient_nonlinear_state!();
                }
                rejected_attempt_nonlinear_state_scratch = rejected_attempt_nonlinear_state.take();
                total_middle_nanos += middle_phase_start.elapsed().as_nanos();
                continue;
            }

            // Keep ideal source constraints exact before LTE and state updates.
            let projected_voltage_sources = circuit
                .enforce_prescribed_transient_voltage_constraints(&mut new_solution, step_time)?;
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
                    return Err(SimulationError::ConvergenceFailed(total_step_attempts));
                }
                trap_order = native_order_after_restart(current_method);
                restore_rejected_transient_nonlinear_state!();
                total_middle_nanos += middle_phase_start.elapsed().as_nanos();
                continue;
            }
            stale_accept_count = 0;
            reject_for_veriloga_event_refinement!(&new_solution, step_time, middle_phase_start);

            // Success - reset retry counter only after event-root refinement
            // has had the opportunity to reject this candidate endpoint.
            retry_count = 0;

            // Accept this timestep
            t = step_time;
            let mut hit_breakpoint = if let Some(grid) = locked_grid.as_ref() {
                // Land exactly on reference grid points, but allow pending
                // XSPICE events to split a locked interval before the next
                // recorded reference sample.
                if locked_step_lands_on_grid {
                    if !analysis_final_step {
                        t = grid[locked_cursor];
                    }
                    locked_cursor += 1;
                }
                accepted_step_hits_breakpoint(
                    landed_veriloga_event,
                    false,
                    lte_estimator.uses_accepted_solution_reference()
                        && breakpoints.at_breakpoint(t),
                )
            } else {
                accepted_step_hits_breakpoint(
                    landed_veriloga_event,
                    at_breakpoint,
                    breakpoints.at_breakpoint(t),
                )
            };
            // A locked grid is an external acceptance contract: retain its exact
            // target even when a source breakpoint is within the breakpoint
            // tolerance. Manager-scheduled landings may consume that nearby
            // point without perturbing the prescribed sample time. A refined
            // device root instead retains the source point for a strict follow-up
            // solve because its candidate was evaluated at the root's own time.
            if hit_breakpoint
                && !landed_veriloga_event
                && !locked_step_lands_on_grid
                && !analysis_final_step
            {
                t = breakpoints.snap_to_breakpoint(t);
            }
            let method_after_step = current_integration_method(&trapgear);
            if circuit.has_nonlinear_devices() && !nonlinear_state_matches_new_solution {
                self.update_transient_nonlinear_devices(&mut circuit, &new_solution)?;
            }

            total_middle_nanos += middle_phase_start.elapsed().as_nanos();
            let trap_trial_phase_start = DiagnosticTimer::start(diagnostic_timing_enabled);
            let trapezoidal_order_trial = if !first_accepted_transient_step
                && !linearized_startup_recovery_points
                && !lte_estimator.uses_accepted_solution_reference()
                && matches!(
                    current_method,
                    IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                )
                && !hit_breakpoint
            {
                if step_trap_order == 2
                    && !lte_estimator.uses_accepted_solution_reference()
                    && device_truncation_limit.is_some()
                {
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
                        &b3soi_history,
                        &bsim3_history,
                        &bsim4_history,
                        &lte_estimator,
                        &voltage_lte_excluded_nodes,
                        &xyce_lte_excluded_indices,
                        &vbic_snapshot_cache,
                        self.voltage_abstol(),
                        transient_lte_reltol,
                        self.current_abstol(),
                        self.charge_abstol(),
                        self.transient_trtol(),
                    )
                }
            } else {
                None
            };
            total_trap_trial_nanos += trap_trial_phase_start.elapsed().as_nanos();

            let capture_xyce_static_history = self.config.spice_dialect == SpiceDialect::Xyce
                && !uses_direct_xyce_dae
                && (xyce_one_step_order2 || xyce_promotes_order_two);
            let mut xyce_static_history_candidate = None;
            if capture_xyce_static_history && !circuit.has_xspice_devices() {
                xyce_static_history_candidate = Some(self.capture_xyce_static_residual(
                    &mut circuit,
                    &mut matrix,
                    &new_solution,
                    t,
                    transient_baseline_diag_gmin,
                )?);
            }
            if uses_direct_xyce_dae {
                capture_direct_xyce_histories(
                    &circuit,
                    &new_solution,
                    t,
                    xyce_direct_vectors
                        .as_mut()
                        .expect("direct Xyce DAE vectors are allocated for the gated path"),
                    xyce_direct_q_candidate
                        .as_mut()
                        .expect("direct Xyce Q scratch is allocated for the gated path"),
                    xyce_direct_static_candidate
                        .as_mut()
                        .expect("direct Xyce static scratch is allocated for the gated path"),
                )
                .map_err(SimulationError::Circuit)?;
            }
            let history_phase_start = DiagnosticTimer::start(diagnostic_timing_enabled);
            let cached_mosfet_gate_companion_charges = (mosfet_companion_terms_valid
                && mosfet_companion_charges_scratch.len() == circuit.mosfets.devices.len())
            .then_some(mosfet_companion_charges_scratch.as_slice());
            self.update_reactive_history(
                &mut circuit,
                &new_solution,
                t,
                dt,
                &coeff,
                &bsim4_trnqs_coeff,
                &mut bjt_history,
                &mut jfet_history,
                &mut diode_history,
                &mut mosfet_history,
                &mut vdmos_history,
                &mut b3soi_history,
                &mut bsim3_history,
                &mut bsim4_history,
                &mut ekv26_history,
                xyce_one_step_order2,
                Some(vbic_snapshot_cache.as_slice()),
                capacitor_accepted_states_valid
                    .then_some(capacitor_accepted_states_scratch.as_slice()),
                mosfet_caps_valid.then_some(mosfet_caps_scratch.as_slice()),
                cached_mosfet_gate_companion_charges,
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
                &mut pending_dynamic_tline_breakpoints,
            )?;
            total_history_nanos += history_phase_start.elapsed().as_nanos();
            let tail_phase_start = DiagnosticTimer::start(diagnostic_timing_enabled);
            // Accept XSPICE timestep (commit state changes)
            if circuit.has_xspice_devices() {
                if capture_xyce_static_history {
                    circuit.evaluate_xspice_transient_timestep_with_coefficients(
                        t,
                        dt,
                        &new_solution,
                        &coeff,
                        xyce_one_step_order2,
                    );
                    circuit.accept_xspice_timestep();
                } else {
                    circuit.accept_xspice_transient_timestep_with_coefficients(
                        t,
                        dt,
                        &new_solution,
                        &coeff,
                        xyce_one_step_order2,
                    );
                }
                circuit.project_xspice_voltage_outputs(&mut new_solution, num_nodes);
                Self::collect_xspice_runtime_breakpoints(&mut circuit, &mut breakpoints, tstop);
                if capture_xyce_static_history {
                    xyce_static_history_candidate = Some(self.capture_xyce_static_residual(
                        &mut circuit,
                        &mut matrix,
                        &new_solution,
                        t,
                        transient_baseline_diag_gmin,
                    )?);
                }
            }
            #[cfg(feature = "veriloga")]
            if circuit.has_veriloga_devices() {
                circuit
                    .evaluate_veriloga_timepoint(&new_solution)
                    .map_err(SimulationError::Circuit)?;
            }
            #[cfg(feature = "veriloga-builtins-base")]
            if circuit.has_generated_veriloga_devices() {
                circuit
                    .evaluate_generated_veriloga_timepoint(&mut matrix, &new_solution)
                    .map_err(SimulationError::Circuit)?;
            }
            let veriloga_discontinuity = if circuit.has_any_veriloga_devices() {
                circuit
                    .accept_all_veriloga_timestep()
                    .map_err(SimulationError::Circuit)?
            } else {
                false
            };
            pending_veriloga_event_time =
                accepted_veriloga_event_time(&circuit, t, timestep.hard_min_dt())?;
            // `$discontinuity` is an accepted boundary event. Fold it into
            // the same restart contract as source and operator breakpoints so
            // Trap/Gear and LTE history are reset, not merely the next `dt`.
            hit_breakpoint |= veriloga_discontinuity;

            if fixed_method.is_none() {
                if hit_breakpoint {
                    trapgear.restart_from(&new_solution);
                } else {
                    trapgear.update(&new_solution, dt);
                }
            }
            if lte_estimator.uses_accepted_solution_reference() {
                lte_estimator.record_with_order(
                    &new_solution,
                    new_solution.len(),
                    dt,
                    xyce_history_order,
                );
                if hit_breakpoint {
                    lte_estimator.restart_history_from(&new_solution);
                }
                lte_estimator
                    .set_method_order(effective_method_order(method_after_step, step_trap_order));
            } else {
                // Commit predictor-local history after XSPICE has projected
                // accepted voltage outputs into the solution vector.
                lte_estimator.record(&new_solution, dt);
                lte_estimator
                    .set_method_order(effective_method_order(method_after_step, step_trap_order));
            }
            if hit_breakpoint {
                xyce_lte_restart_first_step = true;
            }
            xyce_step_failure_count = 0;

            solution.clone_from(&new_solution);
            circuit
                .resistors
                .advance_thermal_states(&solution, dt)
                .map_err(SimulationError::Circuit)?;
            if let Some(history) = xyce_static_history_candidate {
                xyce_static_history = Some(history);
            }
            if uses_direct_xyce_dae {
                xyce_direct_accepted_q
                    .as_mut()
                    .expect("direct Xyce Q history is allocated for the gated path")
                    .copy_from_slice(
                        xyce_direct_q_candidate
                            .as_deref()
                            .expect("direct Xyce Q scratch is allocated for the gated path"),
                    );
                xyce_direct_static_history
                    .get_or_insert_with(|| vec![0.0; size])
                    .copy_from_slice(
                        xyce_direct_static_candidate
                            .as_deref()
                            .expect("direct Xyce static scratch is allocated for the gated path"),
                    );
            }

            accepted_interval_count = accepted_interval_count.checked_add(1).ok_or_else(|| {
                SimulationError::Circuit("transient accepted-interval count overflowed".to_string())
            })?;
            let trajectory_point_count =
                accepted_interval_count.checked_add(1).ok_or_else(|| {
                    SimulationError::Circuit(
                        "transient trajectory point count overflowed".to_string(),
                    )
                })?;

            // Store results
            Self::backfill_initial_linear_capacitor_branch_currents(
                &mut result,
                &circuit,
                &derived_branch_currents,
            );
            retained_result_values =
                retained_result_values.saturating_add(self.record_transient_solution_sample(
                    &mut result,
                    &mut circuit,
                    &solution,
                    num_nodes,
                    t,
                    dt,
                    &derived_branch_currents,
                    &bjt_history,
                    &diode_history,
                    record_device_op_traces,
                    &capture_plan,
                    trajectory_point_count,
                    abort,
                )?);
            if record_xspice_event_traces {
                circuit.fill_xspice_digital_snapshot(&mut digital_snapshot);
                retained_result_values =
                    retained_result_values.saturating_add(result.record_digital_snapshot(
                        t,
                        &digital_snapshot,
                        &mut digital_trace_indices,
                        &capture_plan.event_nodes,
                    ));
                circuit.fill_xspice_real_snapshot(&mut real_snapshot);
                retained_result_values =
                    retained_result_values.saturating_add(result.record_real_snapshot(
                        t,
                        &real_snapshot,
                        &mut real_trace_indices,
                        &capture_plan.event_nodes,
                    ));
            }
            self.ensure_transient_result_limits(&result, retained_result_values)?;
            if first_accepted_transient_step {
                let span_ceiling = xyce_breakpoint_span_ceiling.ceiling();
                let accepted_max_step = self
                    .transient_device_max_timestep(&circuit, t, hinted_max_step)
                    .min(span_ceiling.unwrap_or(Value::INFINITY));
                timestep.set_max_dt(accepted_max_step);
                let next_dt = if xyce_iteration_error_control {
                    (dt * lte_scale).min(accepted_max_step)
                } else if lte_estimator.uses_accepted_solution_reference() {
                    // Xyce does not test LTE on the first successful transient
                    // step (`TESTFIRSTSTEP=false`), then applies its normal
                    // maximum 2x growth before later breakpoint/device caps.
                    (dt * 2.0).min(accepted_max_step)
                } else {
                    // Preserve ngspice's initial repeated-delta behavior for
                    // native predictor-local control.
                    dt
                };
                timestep.force_step(next_dt);
            } else {
                let span_ceiling = xyce_breakpoint_span_ceiling.ceiling();
                let accepted_max_step = self
                    .transient_device_max_timestep(&circuit, t, hinted_max_step)
                    .min(span_ceiling.unwrap_or(Value::INFINITY));
                timestep.set_max_dt(accepted_max_step);
                if xyce_iteration_error_control {
                    // The LTE recovery helper floors an accepted Xyce shrink
                    // at 0.25. ERROPTION=1 instead requires the full /8 when
                    // the accepted solve exceeded NLMAX.
                    timestep.force_step((dt * lte_scale).min(accepted_max_step));
                } else {
                    Self::recover_timestep_after_accepted_step(
                        &mut timestep,
                        &lte_estimator,
                        &new_solution,
                        current_method,
                        step_trap_order,
                        dt,
                        accepted_max_step,
                        is_strictly_linear_transient,
                        expected_source_delta,
                        Self::should_apply_active_source_recovery_cap(force_accept_cooldown),
                        Some(lte_scale),
                    );
                }
            }
            if hit_breakpoint {
                let restart_dt = if landed_veriloga_event || veriloga_discontinuity {
                    breakpoints.mark_external_breakpoint_solved(t, dt)
                } else {
                    breakpoints.mark_breakpoint_solved(t)
                };
                let span_ceiling =
                    xyce_breakpoint_span_ceiling.anchor(t, breakpoints.next_after(t), tstop);
                let restarted_max_step = self
                    .transient_device_max_timestep(&circuit, t, hinted_max_step)
                    .min(span_ceiling.unwrap_or(Value::INFINITY));
                timestep.set_max_dt(restarted_max_step);
                timestep.force_step(restart_dt.min(timestep.dt()).min(restarted_max_step));
                if !lte_estimator.uses_accepted_solution_reference() && !circuit.vdmoses.is_empty()
                {
                    lte_warmup_skips = lte_warmup_skips.max(2);
                }
            }
            if !xyce_iteration_error_control
                && !first_accepted_transient_step
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
            // Verilog-A `$bound_step(0)` requests the solver's smallest
            // supported step. Invalid negative/non-finite requests fail in
            // the device API rather than disappearing here.
            #[cfg(feature = "veriloga")]
            if circuit.has_veriloga_devices() {
                if let Some(bound) = circuit
                    .veriloga_timestep_bound()
                    .map_err(SimulationError::Circuit)?
                    && bound.max(timestep.hard_min_dt()) < timestep.dt()
                {
                    timestep.force_step(bound.max(timestep.hard_min_dt()).min(max_step));
                }
            }
            if first_accepted_transient_step
                && matches!(
                    current_method,
                    IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
                )
            {
                trap_order = xyce_startup_or_restart_order(xyce_min_order);
            } else if matches!(
                current_method,
                IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
            ) {
                let should_promote = if lte_estimator.uses_accepted_solution_reference() {
                    xyce_promotes_order_two
                } else {
                    trapezoidal_order_trial.is_some_and(|trial| trial.promote)
                };
                trap_order = Self::next_trapezoidal_order_after_accepted_step(
                    step_trap_order,
                    hit_breakpoint,
                    should_promote,
                );
                if hit_breakpoint {
                    trap_order = xyce_startup_or_restart_order(xyce_min_order);
                }
                if let Some(trial) = trapezoidal_order_trial
                    && trial.limit.is_finite()
                    && trial.limit > 0.0
                    && trial.limit + 1e-18 < timestep.dt()
                {
                    timestep.force_step(trial.limit);
                }
            } else if current_method == IntegrationMethod::Gear2
                && lte_estimator.uses_accepted_solution_reference()
            {
                trap_order = Self::next_trapezoidal_order_after_accepted_step(
                    step_trap_order,
                    hit_breakpoint,
                    xyce_promotes_order_two,
                );
                if hit_breakpoint {
                    trap_order = xyce_startup_or_restart_order(xyce_min_order);
                }
            }

            lte_estimator.set_method_order(effective_method_order(current_method, trap_order));

            analysis_first_step_pending = false;
            if xyce_lte_restart_first_step && !hit_breakpoint {
                xyce_lte_restart_first_step = false;
            }

            lte_warmup_skips = lte_warmup_skips.saturating_sub(1);
            livelock_check!(dt);
            debug_assert_eq!(retry_count, 0);
            debug_assert_eq!(xyce_step_failure_count, 0);
            debug_assert_eq!(stale_accept_count, 0);
            debug_assert!(!circuit.xyce_core_trial_invalid());
            let runtime_resume_blockers =
                Self::exact_integration_runtime_resume_blockers(&circuit, accepted_interval_count);
            let damped_status_checkpoint = xyce_damped_status
                .as_ref()
                .map(|status| status.capture_accepted_boundary_checkpoint())
                .transpose()
                .map_err(SimulationError::Circuit)?;
            self.capture_scheduled_checkpoint_if_due(
                scheduled_checkpoint_times,
                &mut scheduled_checkpoint_cursor,
                t,
                fingerprint,
                &netlist_identity,
                &restart_identity,
                &simulation_identity,
                &solution,
                &circuit,
                startup_mode,
                max_step,
                Some(ProposedIntegrationContinuation {
                    next_step: timestep.dt(),
                    breakpoint_span_ceiling: xyce_breakpoint_span_ceiling.ceiling(),
                    controller_max_step: timestep.max_dt(),
                    analysis_first_step_pending,
                    xyce_breakpoint_restart_pending: xyce_lte_restart_first_step,
                }),
                tstop,
                &pending_dynamic_tline_breakpoints,
                dynamic_tline_breakpoints_added,
                &bjt_history,
                &diode_history,
                &vbic_snapshot_cache,
                AcceptedIntegrationRuntimeCapture {
                    lte_estimator: &lte_estimator,
                    next_trap_order: trap_order,
                    trapgear: fixed_method.is_none().then(|| trapgear.capture_snapshot()),
                    xyce_static_residual: xyce_static_history.as_deref(),
                    direct_dae_accepted_q: xyce_direct_accepted_q.as_deref(),
                    direct_dae_static_residual: xyce_direct_static_history.as_deref(),
                    lte_warmup_skips,
                    force_accept_cooldown,
                    livelock_streak,
                    livelock_last_restart_time: livelock_last_restart_t,
                    accepted_interval_count,
                    damped_first_solver_call: xyce_damped_first_solver_call,
                    damped_status: damped_status_checkpoint,
                    retry_count,
                    xyce_step_failure_count,
                    stale_accept_count,
                    resume_blockers: &runtime_resume_blockers,
                },
                retained_result_values,
                &mut retained_scheduled_checkpoint_values,
                &mut scheduled_checkpoints,
            )?;
            reinitialize_xyce_breakpoint_histories!(hit_breakpoint, analysis_final_step);
            rejected_attempt_nonlinear_state_scratch = rejected_attempt_nonlinear_state.take();
            total_tail_nanos += tail_phase_start.elapsed().as_nanos();
        }

        if t < tstop {
            log::error!(
                "Transient terminated early at t={:.6e}s / {:.6e}s after {} step attempts",
                t,
                tstop,
                total_step_attempts
            );
            return Err(SimulationError::ConvergenceFailed(total_step_attempts));
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
        log::debug!(
            "Transient Newton phases: {} step attempts, {} merit trials, {} failed attempts (v={} d={} r={}), top {:.3}s, setup {:.3}s, stamp {:.3}s, solve {:.3}s, merit {:.3}s, postsolve {:.3}s, postloop {:.3}s, trunc {:.3}s, trap-trial {:.3}s, history {:.3}s, tail {:.3}s, middle {:.3}s, other {:.3}s (wall {:.3}s)",
            total_step_attempts,
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
        log::debug!(
            "Transient postsolve detail: update {:.3}s, convergence {:.3}s, residual-proof {:.3}s",
            total_postsolve_update_nanos as f64 * 1e-9,
            total_postsolve_convergence_nanos as f64 * 1e-9,
            total_postsolve_residual_nanos as f64 * 1e-9,
        );

        if log::log_enabled!(log::Level::Debug)
            && let Some(node0_voltages) = result.voltages.first()
        {
            let v_min = node0_voltages.iter().copied().fold(f64::INFINITY, f64::min);
            let v_max = node0_voltages
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            log::debug!(
                "Stored voltages for node0: {} points, y_min={:.4}, y_max={:.4}",
                node0_voltages.len(),
                v_min,
                v_max
            );
        }

        // The devices keep the tally; the circuit is local to this body, so the
        // count has to be lifted into the run's diagnostics before it is
        // dropped. Transient is the only phase in which bypass can fire.
        let bypassed = circuit.b3soi_bypass_hits();
        self.record_convergence(|quality| quality.bypassed_device_evaluations = bypassed);

        debug_assert_eq!(retry_count, 0);
        debug_assert_eq!(xyce_step_failure_count, 0);
        debug_assert_eq!(stale_accept_count, 0);
        debug_assert!(!circuit.xyce_core_trial_invalid());
        if scheduled_checkpoint_cursor != scheduled_checkpoint_times.len() {
            return Err(SimulationError::Circuit(format!(
                "transient ended at {t:.17e}s before scheduled checkpoint {:.17e}s was captured",
                scheduled_checkpoint_times[scheduled_checkpoint_cursor]
            )));
        }
        let final_checkpoint = if final_checkpoint_retention.is_retained() {
            let final_accepted_junction_history =
                Self::capture_accepted_junction_transient_history_checkpoint(
                    &circuit,
                    &bjt_history,
                    &diode_history,
                    &vbic_snapshot_cache,
                );
            let final_accepted_junction_history =
                if final_accepted_junction_history.resume_blockers.is_empty() {
                    Self::normalize_accepted_junction_transient_history_checkpoint_for_order_one(
                        &circuit,
                        &final_accepted_junction_history,
                        0.0,
                    )
                    .map_err(SimulationError::Circuit)?
                } else {
                    final_accepted_junction_history
                };
            let final_damped_status_checkpoint = xyce_damped_status
                .as_ref()
                .map(|status| status.capture_accepted_boundary_checkpoint())
                .transpose()
                .map_err(SimulationError::Circuit)?;
            let final_accepted_integration_runtime = AcceptedIntegrationRuntime::RestartNormalized(
                RestartNormalizedIntegrationRuntimeCheckpoint::capture(
                    t,
                    RestartNormalizedIntegrationRuntimeCapture {
                        lte_warmup_skips,
                        force_accept_cooldown,
                        livelock_streak,
                        livelock_last_restart_time: livelock_last_restart_t,
                        accepted_interval_count,
                        damped_first_solver_call: xyce_damped_first_solver_call,
                        damped_status: final_damped_status_checkpoint,
                        retry_count,
                        xyce_step_failure_count,
                        stale_accept_count,
                        resume_blockers: &[],
                    },
                )
                .map_err(SimulationError::Circuit)?,
            );
            Some(
                TransientCheckpoint::capture_with_restart_identity(
                    fingerprint,
                    netlist_identity,
                    restart_identity,
                    simulation_identity,
                    t,
                    &solution,
                    &circuit,
                    startup_mode,
                    Some(max_step),
                    None,
                    &pending_dynamic_tline_breakpoints,
                    dynamic_tline_breakpoints_added,
                    final_accepted_junction_history,
                    final_accepted_integration_runtime,
                    Some(&lte_estimator),
                )
                .map_err(SimulationError::Circuit)?,
            )
        } else {
            None
        };
        result.fft_results = fft::evaluate(self, netlist, &result, tstop, abort)?;
        retained_result_values = Self::transient_result_value_count(&result);
        self.ensure_result_values(
            retained_result_values
                .saturating_add(retained_scheduled_checkpoint_values)
                .saturating_add(
                    final_checkpoint
                        .as_ref()
                        .map_or(0, TransientCheckpoint::retained_value_count),
                ),
        )?;
        Ok((result, final_checkpoint, scheduled_checkpoints))
    }

    /// Run transient analysis with waveform compression
    ///
    /// Uses multi-channel Ramer-Douglas-Peucker decimation. Every discarded
    /// sample is checked against the linearly interpolated retained waveform,
    /// using the configured absolute-plus-relative error bound.
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
        let startup_mode = Self::inferred_transient_startup_mode(netlist)?;
        self.run_tran_compressed_with_startup_mode_and_abort(
            netlist,
            tstop,
            max_step,
            startup_mode,
            compression,
            abort,
        )
    }

    /// Run compressed transient analysis for one explicitly selected startup
    /// contract.
    pub fn run_tran_compressed_with_startup_mode_and_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        compression: CompressionConfig,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResultCompressed, SimulationError> {
        // Reuse the robust transient solver path, then apply waveform compression
        // during result marshaling. This keeps compressed and uncompressed physics
        // behavior identical, avoiding divergence between solver implementations.
        let result = self.run_tran_with_startup_mode_and_abort(
            netlist,
            tstop,
            max_step,
            startup_mode,
            abort,
        )?;

        if result.time.is_empty() {
            return Ok(TransientResultCompressed {
                time: Vec::new(),
                voltages: vec![Vec::new(); result.num_nodes],
                num_nodes: result.num_nodes,
                node_names: result.node_names.clone(),
                store_traces: result.store_traces.clone(),
                fft_results: result.fft_results.clone(),
                compression_ratio: 1.0,
                input_points: 0,
            });
        }

        compress_transient_result(&result, &compression, abort)
    }
}

fn compress_transient_result(
    result: &TransientResult,
    config: &CompressionConfig,
    abort: &dyn AbortSignal,
) -> Result<TransientResultCompressed, SimulationError> {
    let point_count = result.time.len();
    if !config.abs_tol.is_finite() || config.abs_tol < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Compression abs_tol must be finite and non-negative, got {}",
            config.abs_tol
        )));
    }
    if !config.rel_tol.is_finite() || config.rel_tol < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Compression rel_tol must be finite and non-negative, got {}",
            config.rel_tol
        )));
    }
    if !config.min_interval.is_finite() || config.min_interval < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Compression maximum interval must be finite and non-negative, got {}",
            config.min_interval
        )));
    }
    if result.voltages.len() != result.num_nodes
        || result
            .voltages
            .iter()
            .any(|waveform| !waveform.is_empty() && waveform.len() != point_count)
        || result
            .store_traces
            .iter()
            .any(|trace| trace.values.len() != point_count)
    {
        return Err(SimulationError::Circuit(
            "Cannot compress malformed transient voltage or store waveforms".to_string(),
        ));
    }
    if point_count <= 2 || !config.enabled {
        return Ok(TransientResultCompressed {
            time: result.time.clone(),
            voltages: result.voltages.clone(),
            num_nodes: result.num_nodes,
            node_names: result.node_names.clone(),
            store_traces: result.store_traces.clone(),
            fft_results: result.fft_results.clone(),
            compression_ratio: 1.0,
            input_points: point_count,
        });
    }
    if result
        .time
        .windows(2)
        .any(|window| !window[0].is_finite() || window[1] <= window[0])
        || result.time.last().is_some_and(|time| !time.is_finite())
    {
        return Err(SimulationError::Circuit(
            "Cannot compress a transient with non-finite or non-increasing time points".to_string(),
        ));
    }

    let mut retained = vec![false; point_count];
    retained[0] = true;
    retained[point_count - 1] = true;
    let mut segments = vec![(0usize, point_count - 1)];
    while let Some((start, end)) = segments.pop() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if end <= start + 1 {
            continue;
        }

        // The legacy CompressionConfig field is named min_interval, but its
        // production-safe meaning here is a maximum gap between retained
        // points: a positive value prevents excessive time-axis decimation.
        let duration = result.time[end] - result.time[start];
        let interval_split = if config.min_interval > 0.0 && duration > config.min_interval {
            let target = result.time[start] + config.min_interval;
            Some(
                ((start + 1)..end)
                    .min_by(|&lhs, &rhs| {
                        (result.time[lhs] - target)
                            .abs()
                            .total_cmp(&(result.time[rhs] - target).abs())
                    })
                    .unwrap_or(start + 1),
            )
        } else {
            None
        };

        let mut worst_index = interval_split;
        let mut worst_ratio = if interval_split.is_some() {
            Value::INFINITY
        } else {
            1.0
        };
        if interval_split.is_none() {
            let t0 = result.time[start];
            let inverse_dt = 1.0 / (result.time[end] - t0);
            for point in (start + 1)..end {
                if point.is_multiple_of(4096) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let fraction = (result.time[point] - t0) * inverse_dt;
                for waveform in result
                    .voltages
                    .iter()
                    .filter(|waveform| !waveform.is_empty())
                    .chain(result.store_traces.iter().map(|trace| &trace.values))
                {
                    let actual = waveform[point];
                    let predicted = waveform[start] + fraction * (waveform[end] - waveform[start]);
                    let tolerance = config.abs_tol + config.rel_tol * actual.abs();
                    let error = (actual - predicted).abs();
                    let ratio = if !error.is_finite() {
                        Value::INFINITY
                    } else if tolerance > 0.0 {
                        error / tolerance
                    } else if error == 0.0 {
                        0.0
                    } else {
                        Value::INFINITY
                    };
                    if ratio > worst_ratio {
                        worst_ratio = ratio;
                        worst_index = Some(point);
                    }
                }
            }
        }

        if let Some(split) = worst_index
            && worst_ratio > 1.0
        {
            retained[split] = true;
            segments.push((start, split));
            segments.push((split, end));
        }
    }

    let indices = retained
        .iter()
        .enumerate()
        .filter_map(|(index, keep)| keep.then_some(index))
        .collect::<Vec<_>>();
    let stored_points = indices.len();
    Ok(TransientResultCompressed {
        time: indices.iter().map(|&index| result.time[index]).collect(),
        voltages: result
            .voltages
            .iter()
            .map(|waveform| {
                if waveform.is_empty() {
                    Vec::new()
                } else {
                    indices.iter().map(|&index| waveform[index]).collect()
                }
            })
            .collect(),
        num_nodes: result.num_nodes,
        node_names: result.node_names.clone(),
        store_traces: result
            .store_traces
            .iter()
            .map(|trace| crate::engine::TransientStoreTrace {
                name: trace.name.clone(),
                values: indices.iter().map(|&index| trace.values[index]).collect(),
            })
            .collect(),
        fft_results: result.fft_results.clone(),
        compression_ratio: point_count as Value / stored_points as Value,
        input_points: point_count,
    })
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
    use crate::{Netlist, SimulationConfig};
    use std::cell::Cell;

    #[test]
    fn canonical_final_step_time_repairs_subtraction_addition_round_trip() {
        const CURRENT_TIME: Value = Value::from_bits(0x3e02_db21_7f74_4098);
        const STOP_TIME: Value = Value::from_bits(0x3e42_ffcc_ca47_13bf);

        let remaining = STOP_TIME - CURRENT_TIME;
        assert_ne!(
            (CURRENT_TIME + remaining).to_bits(),
            STOP_TIME.to_bits(),
            "the oracle pair must retain its one-ULP subtraction/addition mismatch"
        );
        assert_eq!(
            canonical_transient_step_time(CURRENT_TIME, remaining, STOP_TIME).to_bits(),
            STOP_TIME.to_bits(),
            "a step that consumes the remaining interval must use the exact requested horizon"
        );
    }

    #[test]
    fn exact_veriloga_event_time_survives_subtraction_addition_round_trip() {
        const CURRENT_TIME: Value = Value::from_bits(0x3e02_db21_7f74_4098);
        const EVENT_TIME: Value = Value::from_bits(0x3e42_ffcc_ca47_13bf);
        let dt = EVENT_TIME - CURRENT_TIME;
        let stop_time = EVENT_TIME * 2.0;

        assert_ne!(
            canonical_transient_step_time(CURRENT_TIME, dt, stop_time).to_bits(),
            EVENT_TIME.to_bits(),
            "the oracle must miss the interior event by one ULP without its absolute target"
        );
        assert_eq!(
            canonical_transient_step_time_with_device_event(
                CURRENT_TIME,
                dt,
                stop_time,
                Some(EVENT_TIME),
            )
            .to_bits(),
            EVENT_TIME.to_bits()
        );
        assert!(accepted_step_hits_breakpoint(true, false, false));
    }

    #[test]
    fn veriloga_event_interval_validation_fails_closed_below_hard_minimum() {
        validate_veriloga_event_interval(1.25, 1.0, 0.25)
            .expect("an event exactly at the hard minimum must be schedulable");

        let error = validate_veriloga_event_interval(1.25, 1.0, 0.250_000_000_000_000_1)
            .expect_err("a Verilog-A event below the hard minimum must fail closed");
        assert!(error.to_string().contains("below the solver hard minimum"));

        let error = validate_veriloga_event_interval(1.25, 1.0, Value::NAN)
            .expect_err("an invalid hard minimum must fail closed");
        assert!(error.to_string().contains("invalid solver hard minimum"));
    }

    #[test]
    fn refined_device_root_restarts_history_without_a_scheduled_breakpoint() {
        let hit_breakpoint = accepted_step_hits_breakpoint(true, false, false);

        assert!(hit_breakpoint);
        assert_eq!(
            Engine::next_trapezoidal_order_after_accepted_step(2, hit_breakpoint, true),
            1
        );
        assert!(!accepted_step_hits_breakpoint(false, false, false));
    }

    fn run_native_bjt_total_leads(
        method: &str,
        pnp: bool,
        dynamic_charge: bool,
    ) -> TransientResult {
        let (kind, collector_bias, base_waveform) = if pnp {
            ("PNP", "-5", "PULSE(0 -1 1n 0.25n 0.25n 2n 5n)")
        } else {
            ("NPN", "5", "PULSE(0 1 1n 0.25n 0.25n 2n 5n)")
        };
        let base_waveform = if dynamic_charge {
            base_waveform
        } else if pnp {
            "-0.75"
        } else {
            "0.75"
        };
        let charge = if dynamic_charge { "1u" } else { "0" };
        let source = format!(
            "Native BJT accepted total lead currents\n\
             VE 0 E 0\n\
             VC 0 C {collector_bias}\n\
             VB 0 B {base_waveform}\n\
             VS 0 S 0\n\
             Q1 C B E S QMOD\n\
             .MODEL QMOD {kind} LEVEL=1 IS=3e-14 BF=130 BR=1 \
             RB=45 RBM=45 RC=2 RE=1 CJE={charge} CJC={charge} CJS={charge} \
             TF=0 TR=0 VJE=0.75 VJC=0.75\n\
             .OPTIONS TIMEINT METHOD={method}\n\
             .TRAN 0.5n 8n\n\
             .PRINT TRAN I(VB) I(VC) I(VE) I(VS) \
             IC(Q1) IB(Q1) IE(Q1) IS(Q1)\n\
             .END\n"
        );
        let netlist = Netlist::parse_with_options(
            &source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("native BJT lead-current deck parses");
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .run_tran(&netlist, 8.0e-9, 0.5e-9)
            .unwrap_or_else(|error| panic!("native BJT lead-current transient failed: {error}"))
    }

    fn assert_native_bjt_total_lead_kcl(result: &TransientResult, expect_dynamic: bool) {
        assert!(result.time.len() >= 2);
        assert!(result.time.windows(2).all(|pair| pair[1] > pair[0]));
        let terminal_pairs = [("VB", "IB"), ("VC", "IC"), ("VE", "IE"), ("VS", "IS")];
        let mut leads = Vec::new();
        for (source, parameter) in terminal_pairs {
            let source_current = result
                .try_branch_current_waveform_named(source)
                .unwrap_or_else(|| panic!("missing {source} branch trace"));
            let lead = result
                .try_device_op_waveform_named("q1", &parameter.to_ascii_lowercase())
                .unwrap_or_else(|| panic!("missing Q1 {parameter} trace"));
            assert_eq!(source_current.len(), result.time.len());
            assert_eq!(lead.len(), result.time.len());
            assert!(source_current.iter().all(|value| value.is_finite()));
            assert!(lead.iter().all(|value| value.is_finite()));
            for (index, (&source_value, &lead_value)) in
                source_current.iter().zip(lead.iter()).enumerate()
            {
                assert!(
                    (source_value - lead_value).abs() <= 1.0e-6,
                    "{parameter} authored-lead KCL changed at row {index}: source={source_value:e}, lead={lead_value:e}"
                );
            }
            leads.push(lead);
        }
        for index in 0..result.time.len() {
            let terminal_sum = leads.iter().map(|lead| lead[index]).sum::<Value>();
            assert!(
                terminal_sum.abs() <= 1.0e-6,
                "four-terminal BJT KCL changed at row {index}: sum={terminal_sum:e}"
            );
        }
        if expect_dynamic {
            assert!(
                leads.iter().any(|lead| {
                    lead.iter()
                        .skip(1)
                        .any(|value| (value - lead[0]).abs() > 1.0e-4)
                }),
                "large junction charges must create a non-static accepted lead waveform"
            );
        } else {
            for lead in leads {
                assert!(
                    lead.iter().all(|value| (value - lead[0]).abs() <= 1.0e-10),
                    "zero-charge constant-bias lead waveform changed: {lead:?}"
                );
            }
        }
    }

    #[test]
    fn native_bjt_total_leads_follow_trap_and_gear_companions_with_external_series() {
        for (method, pnp) in [("7", false), ("8", false), ("8", true)] {
            let result = run_native_bjt_total_leads(method, pnp, true);
            assert_native_bjt_total_lead_kcl(&result, true);
        }
    }

    #[test]
    fn native_bjt_zero_charge_leads_keep_static_t0_and_accepted_values() {
        let result = run_native_bjt_total_leads("7", false, false);
        assert_native_bjt_total_lead_kcl(&result, false);
    }

    #[test]
    fn v20_junction_history_lifecycle_preserves_origin_restart_and_seam_reports() {
        let source = "\
v20 accepted junction history lifecycle
VC 0 C 0
VB 0 B 0
VD 0 D 0
Q1 C B 0 QMOD
D1 D 0 DMOD
.MODEL QMOD NPN LEVEL=1 IS=3e-14 BF=130 CJE=0 CJC=0 TF=0 TR=0
.MODEL DMOD D IS=1e-14 CJO=0 TT=0
.TRAN 1n 2n
.PRINT TRAN IC(Q1)
.END
";
        let netlist = Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("v20 junction lifecycle deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));

        let (_, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                1.0e-9,
                1.0e-9,
                TransientStartupMode::OperatingPoint,
                &[0.0],
            )
            .expect("t=0 scheduled junction checkpoint captures");
        let origin = scheduled[0]
            .checkpoint
            .accepted_junction_transient_history();
        assert!(origin.available);
        assert!(origin.resume_blockers.is_empty());
        assert_eq!(origin.bjt_names, ["Q1"]);
        assert_eq!(origin.diode_names, ["D1"]);
        assert_eq!(origin.bjt_history.vbe_prev.len(), 1);
        assert_eq!(origin.diode_history.vd_prev.len(), 1);
        assert_eq!(origin.vbic_snapshot_cache.len(), 1);
        assert!(origin.bjt_history.accepted_dt_prev > 0.0);
        assert!(origin.diode_history.accepted_dt_prev > 0.0);
        match scheduled[0].checkpoint.accepted_integration_runtime() {
            AcceptedIntegrationRuntime::Exact(runtime) => {
                assert_eq!(runtime.accepted_interval_count, 0);
                assert!(runtime.resume_blockers.is_empty());
            }
            phase => panic!("t=0 scheduled checkpoint has unexpected runtime phase {phase:?}"),
        }

        let (first, final_checkpoint) = engine
            .run_tran_checkpointed(&netlist, 1.0e-9, 1.0e-9)
            .expect("first v20 junction segment solves");
        let final_history = final_checkpoint.accepted_junction_transient_history();
        assert_eq!(final_history.bjt_history.accepted_dt_prev.to_bits(), 0);
        assert_eq!(final_history.diode_history.accepted_dt_prev.to_bits(), 0);
        assert_eq!(
            final_history.bjt_history.charge_q_prev_prev,
            final_history.bjt_history.charge_q_prev
        );
        assert_eq!(
            final_history.diode_history.qd_prev_prev,
            final_history.diode_history.qd_prev
        );
        assert!(
            final_history
                .bjt_history
                .accepted_terminal_currents
                .iter()
                .all(Option::is_some)
        );
        assert!(
            final_history
                .vbic_snapshot_cache
                .iter()
                .all(Option::is_none)
        );
        match final_checkpoint.accepted_integration_runtime() {
            AcceptedIntegrationRuntime::RestartNormalized(runtime) => {
                assert!(runtime.accepted_interval_count > 0);
                assert!(runtime.resume_blockers.is_empty());
            }
            phase => panic!("final endpoint has unexpected runtime phase {phase:?}"),
        }

        let serialized = TransientCheckpoint::from_text(&final_checkpoint.to_text())
            .expect("v20 junction checkpoint round-trips");
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &serialized, 2.0e-9, 1.0e-9)
            .expect("v20 junction checkpoint resumes");
        for (device, parameter) in [("Q1", "IC"), ("Q1", "IB"), ("D1", "ID")] {
            let uninterrupted = first
                .try_device_op_waveform_named(device, parameter)
                .unwrap_or_else(|| panic!("missing first-segment {device}[{parameter}] trace"));
            let resumed_trace = resumed
                .try_device_op_waveform_named(device, parameter)
                .unwrap_or_else(|| panic!("missing resumed {device}[{parameter}] trace"));
            assert_eq!(
                resumed_trace[0].to_bits(),
                uninterrupted
                    .last()
                    .expect("first segment has samples")
                    .to_bits(),
                "restored seam report changed {device}[{parameter}]"
            );
        }
    }

    #[test]
    fn v20_non_breakpoint_checkpoint_round_trips_bjt_snapshot_cache_and_exact_suffix() {
        let source = "\
v20 non-breakpoint BJT snapshot-cache continuation
VC C 0 1
VB B 0 SIN(-0.2 0.02 100MEG)
VD D 0 SIN(-0.25 0.01 80MEG)
Q1 C B 0 QMOD
D1 D 0 DMOD
.MODEL QMOD NPN LEVEL=1 IS=3e-14 BF=130 CJE=1f CJC=1f TF=0 TR=0
.MODEL DMOD D IS=1e-14 CJO=1f TT=0
.TRAN 0.25n 1n
.PRINT TRAN V(B) IC(Q1) ID(D1)
.END
";
        let netlist = Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("v20 raw snapshot-cache deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));

        let (uninterrupted, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                1.0e-9,
                0.25e-9,
                TransientStartupMode::OperatingPoint,
                &[0.25e-9],
            )
            .expect("non-breakpoint v20 checkpoint trajectory solves");
        assert_eq!(scheduled.len(), 1);
        let checkpoint = &scheduled[0].checkpoint;
        assert!(checkpoint.time < 1.0e-9);
        let history = checkpoint.accepted_junction_transient_history();
        assert!(history.available);
        assert!(history.resume_blockers.is_empty());
        assert!(history.bjt_history.accepted_dt_prev > 0.0);
        assert!(history.diode_history.accepted_dt_prev > 0.0);
        assert!(
            history.vbic_snapshot_cache[0].is_some(),
            "a regular accepted legacy-BJT point must retain its reusable charge snapshot"
        );
        match checkpoint.accepted_integration_runtime() {
            AcceptedIntegrationRuntime::Exact(runtime) => {
                assert!(runtime.accepted_interval_count > 0);
                assert!(runtime.resume_blockers.is_empty());
                assert!(runtime.trapgear.is_some());
            }
            phase => panic!("non-breakpoint checkpoint has unexpected runtime phase {phase:?}"),
        }

        let serialized = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect("raw v20 snapshot-cache checkpoint round-trips");
        let continuation = serialized
            .validated_integration_continuation()
            .expect("raw checkpoint continuation validates")
            .expect("non-breakpoint checkpoint has a proposed continuation");
        assert!(
            serialized
                .accepted_junction_transient_history()
                .vbic_snapshot_cache[0]
                .is_some()
        );
        let (resumed, resumed_final) = engine
            .run_tran_resume(&netlist, &serialized, 1.0e-9, 0.25e-9)
            .expect("raw v20 snapshot-cache checkpoint resumes");
        match resumed_final.accepted_integration_runtime() {
            AcceptedIntegrationRuntime::RestartNormalized(runtime) => assert!(
                runtime.accepted_interval_count
                    > match checkpoint.accepted_integration_runtime() {
                        AcceptedIntegrationRuntime::Exact(runtime) => {
                            runtime.accepted_interval_count
                        }
                        _ => unreachable!("phase asserted above"),
                    }
            ),
            phase => panic!("resumed endpoint has unexpected runtime phase {phase:?}"),
        }
        let seam_index = uninterrupted
            .time
            .iter()
            .position(|time| time.to_bits() == checkpoint.time.to_bits())
            .expect("scheduled accepted time is present in uninterrupted output");
        let expected_time = &uninterrupted.time[seam_index..];
        let q1_uninterrupted = uninterrupted
            .try_device_op_waveform_named("Q1", "IC")
            .expect("uninterrupted Q1[IC] trace exists");
        let q1_resumed = resumed
            .try_device_op_waveform_named("Q1", "IC")
            .expect("resumed Q1[IC] trace exists");
        let first_q1_difference = q1_resumed
            .iter()
            .zip(&q1_uninterrupted[seam_index..])
            .position(|(resumed, uninterrupted)| resumed.to_bits() != uninterrupted.to_bits());
        assert_eq!(
            resumed.time.len(),
            expected_time.len(),
            "restored accepted grid changed at checkpoint {:.17e}: continuation={continuation:?}, uninterrupted suffix={expected_time:?}, resumed={:?}, first Q1[IC] difference={first_q1_difference:?}",
            checkpoint.time,
            resumed.time,
        );
        for (index, (&actual, &expected)) in resumed.time.iter().zip(expected_time).enumerate() {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "restored accepted time changed at suffix row {index}"
            );
        }
        for (device, parameter) in [("Q1", "IC"), ("D1", "ID")] {
            let uninterrupted_trace = uninterrupted
                .try_device_op_waveform_named(device, parameter)
                .unwrap_or_else(|| panic!("missing uninterrupted {device}[{parameter}] trace"));
            let resumed_trace = resumed
                .try_device_op_waveform_named(device, parameter)
                .unwrap_or_else(|| panic!("missing resumed {device}[{parameter}] trace"));
            assert_eq!(resumed_trace.len(), expected_time.len());
            for (index, (&actual, &expected)) in resumed_trace
                .iter()
                .zip(&uninterrupted_trace[seam_index..])
                .enumerate()
            {
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "restored suffix changed {device}[{parameter}] at row {index}"
                );
            }
        }
    }

    #[test]
    fn native_diode_total_current_checkpoint_resume_preserves_exact_suffix() {
        let source = "native diode total-current checkpoint continuation\n\
                      VD D 0 SIN(0 0.05 100MEG)\n\
                      D1 D 0 DMOD\n\
                      .MODEL DMOD D IS=1e-30 N=1 CJO=10p VJ=1 M=0.5 TT=0\n\
                      .TRAN 0.05n 2n\n\
                      .PRINT TRAN V(D) I(D1) ID(D1)\n\
                      .END\n";
        let netlist = Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("native diode checkpoint deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let (uninterrupted, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                2.0e-9,
                0.05e-9,
                TransientStartupMode::OperatingPoint,
                &[0.625e-9],
            )
            .expect("native diode checkpoint trajectory solves");
        assert_eq!(scheduled.len(), 1);
        let checkpoint = &scheduled[0].checkpoint;
        let junction_history = checkpoint.accepted_junction_transient_history();
        assert!(junction_history.available);
        assert!(junction_history.resume_blockers.is_empty());
        assert_eq!(junction_history.diode_history.cqd_prev.len(), 1);
        assert!(
            junction_history.diode_history.cqd_prev[0].abs() > 1.0e-8,
            "checkpoint must exercise nonzero accepted diode dQ/dt current"
        );

        let serialized = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect("native diode checkpoint round-trips");
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &serialized, 2.0e-9, 0.05e-9)
            .expect("native diode checkpoint resumes");
        let seam_index = uninterrupted
            .time
            .iter()
            .position(|time| time.to_bits() == checkpoint.time.to_bits())
            .expect("checkpoint seam is present in uninterrupted result");
        let expected_time = &uninterrupted.time[seam_index..];
        assert_eq!(resumed.time.len(), expected_time.len());
        for (index, (&actual, &expected)) in resumed.time.iter().zip(expected_time).enumerate() {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "resumed accepted time changed at suffix row {index}"
            );
        }

        let uninterrupted_generic = uninterrupted
            .try_branch_current_waveform_named("D1")
            .expect("uninterrupted I(D1) trace exists");
        let resumed_generic = resumed
            .try_branch_current_waveform_named("D1")
            .expect("resumed I(D1) trace exists");
        let uninterrupted_accepted = uninterrupted
            .try_device_op_waveform_named("D1", "ID")
            .expect("uninterrupted ID(D1) trace exists");
        let resumed_accepted = resumed
            .try_device_op_waveform_named("D1", "ID")
            .expect("resumed ID(D1) trace exists");
        for (name, actual, expected) in [
            (
                "I(D1)",
                resumed_generic,
                &uninterrupted_generic[seam_index..],
            ),
            (
                "ID(D1)",
                resumed_accepted,
                &uninterrupted_accepted[seam_index..],
            ),
        ] {
            assert_eq!(actual.len(), expected.len());
            for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "resumed {name} changed at suffix row {index}"
                );
            }
        }
        for (index, (&generic, &accepted)) in
            resumed_generic.iter().zip(resumed_accepted).enumerate()
        {
            assert_eq!(
                generic.to_bits(),
                accepted.to_bits(),
                "resumed I(D1) diverged from accepted ID(D1) at row {index}"
            );
        }
    }

    #[test]
    fn v21_exact_runtime_blockers_are_target_aware_and_time_zero_is_exempt() {
        let stateless = Netlist::parse(
            "stateless behavioral source\nV1 in 0 1\nB1 out 0 V={2*V(in)}\nR1 out 0 1k\n.end",
        )
        .expect("stateless behavioral fixture parses");
        let stateless = Engine::default()
            .build_circuit(&stateless)
            .expect("stateless behavioral fixture builds");
        assert!(Engine::exact_integration_runtime_resume_blockers(&stateless, 1).is_empty());

        let stateful = Netlist::parse(
            "stateful behavioral source\nV1 in 0 1\nB1 out 0 V={SDT(V(in))}\nR1 out 0 1k\n.end",
        )
        .expect("stateful behavioral fixture parses");
        let stateful = Engine::default()
            .build_circuit(&stateful)
            .expect("stateful behavioral fixture builds");
        let blockers = Engine::exact_integration_runtime_resume_blockers(&stateful, 1);
        assert_eq!(
            blockers,
            ["behavioral-source accepted SDT state is not checkpointed"]
        );
        assert!(Engine::exact_integration_runtime_resume_blockers(&stateful, 0).is_empty());

        let stateful_capacitor = Netlist::parse_with_options(
            "stateful capacitor expression\n\
             V1 in 0 1\n\
             R1 out 0 1k\n\
             C1 out 0 C={1p*(1+SDT(V(in)))} IC=0.1\n\
             .end",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("stateful capacitor fixture parses");
        let stateful_capacitor =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
                .build_circuit(&stateful_capacitor)
                .expect("stateful capacitor fixture builds");
        assert_eq!(
            Engine::exact_integration_runtime_resume_blockers(&stateful_capacitor, 1),
            ["solution-dependent capacitor accepted SDT expression state is not checkpointed"]
        );
    }

    #[test]
    fn v21_uic_time_zero_checkpoint_preserves_authenticated_startup_phase() {
        let netlist = Netlist::parse(
            "v21 UIC checkpoint phase\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1p IC=0.2\n.tran 0.1n 0.2n UIC\n.end",
        )
        .expect("UIC phase fixture parses");
        let engine = Engine::default();
        let (_, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                0.2e-9,
                0.1e-9,
                TransientStartupMode::Uic,
                &[0.0],
            )
            .expect("UIC t=0 checkpoint trajectory solves");
        assert_eq!(scheduled.len(), 1);
        let checkpoint = &scheduled[0].checkpoint;
        assert_eq!(checkpoint.startup_mode(), Some(TransientStartupMode::Uic));
        let continuation = checkpoint
            .validated_integration_continuation()
            .expect("UIC t=0 continuation validates")
            .expect("UIC t=0 capture proposes the first interval");
        assert!(continuation.analysis_first_step_pending);
        match checkpoint.accepted_integration_runtime() {
            AcceptedIntegrationRuntime::Exact(runtime) => {
                assert_eq!(runtime.accepted_interval_count, 0);
                assert!(runtime.resume_blockers.is_empty());
            }
            phase => panic!("UIC t=0 checkpoint has unexpected runtime phase {phase:?}"),
        }
    }

    #[test]
    fn native_bjt_authored_lead_mapping_rejects_invalid_solution_nodes() {
        let mut bjt = crate::device::Bjt::new_npn("QBAD".into(), 1, 2, 3);
        bjt.externalize_legacy_base_lead(4, 10.0);
        let error = bjt
            .authored_transient_lead_currents(&[0.0; 2], [0.0; 4])
            .expect_err("out-of-range internal lead node must fail closed");
        assert!(
            error.contains("QBAD") && error.contains("node 4"),
            "{error}"
        );
    }

    #[test]
    fn xyce_update_norm_source_distinguishes_damped_newton_from_nox() {
        let candidate_evaluated = Cell::new(false);
        let damped = select_xyce_transient_update_norm(true, Some(3.0), || {
            candidate_evaluated.set(true);
            Some(7.0)
        });
        assert_eq!(damped, Some(3.0));
        assert!(!candidate_evaluated.get());

        let nox = select_xyce_transient_update_norm(false, Some(3.0), || {
            candidate_evaluated.set(true);
            Some(7.0)
        });
        assert_eq!(nox, Some(7.0));
        assert!(candidate_evaluated.get());
    }

    #[test]
    fn xyce_damped_newton_always_solves_a_correction_system() {
        assert!(transient_newton_uses_correction_form(false, false, true));
        assert!(transient_newton_uses_correction_form(false, true, false));
        assert!(transient_newton_uses_correction_form(true, false, false));
        assert!(!transient_newton_uses_correction_form(false, false, false));
    }

    fn run_xyce_legacy_bsim_wildcard(output_cards: &str) -> TransientResult {
        let source = format!(
            "Xyce wildcard excludes legacy BSIM prime nodes\n\
             VDS D 0 0.05\n\
             VGS G 0 1.8\n\
             M1 D G 0 0 B1 L=10u W=50u\n\
             .MODEL B1 NMOS LEVEL=4 TOX=0.03 VDD=5 RSH=35\n\
             .TRAN 1n 2n\n\
             {output_cards}\n\
             .END\n"
        );
        let netlist = Netlist::parse_with_options(
            &source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce legacy BSIM wildcard deck parses");
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .run_tran(&netlist, 2.0e-9, 1.0e-9)
            .expect("legacy BSIM wildcard transient solves")
    }

    fn assert_only_external_bsim_voltage_traces(result: &TransientResult) {
        for external in ["D", "G"] {
            let trace = result
                .try_voltage_waveform_named(external)
                .unwrap_or_else(|| panic!("missing external waveform {external}"));
            assert_eq!(trace.len(), result.time.len());
        }
        for private in ["M1.__dint", "M1.__sint"] {
            let trace = result
                .try_voltage_waveform_named(private)
                .unwrap_or_else(|| panic!("missing private-node metadata {private}"));
            assert!(
                trace.is_empty(),
                "wildcard must not capture or resource-charge {private}"
            );
        }
        assert_eq!(
            result
                .voltages
                .iter()
                .filter(|trace| !trace.is_empty())
                .count(),
            2,
            "only D and G may contribute voltage traces to the result budget"
        );
    }

    #[test]
    fn save_complete_voltage_wildcard_uses_one_level_pattern_for_bsim_nodes() {
        let result = run_xyce_legacy_bsim_wildcard(".SAVE V(*)");
        assert_only_external_bsim_voltage_traces(&result);
    }

    #[test]
    fn mixed_print_and_save_complete_voltage_wildcards_do_not_capture_bsim_private_nodes() {
        let result = run_xyce_legacy_bsim_wildcard(".PRINT TRAN V(*)\n.SAVE V(*)");
        assert_only_external_bsim_voltage_traces(&result);
    }

    fn run_xyce_team_memristor_capture(output_cards: &str) -> TransientResult {
        let source = format!(
            "Xyce wildcard excludes builder-generated TEAM state\n\
             .MODEL MRM1 MEMRISTOR LEVEL=2 RON=50 ROFF=1k\n\
             YMEMRISTOR MR1 IN 0 MRM1 IVRELATION=1\n\
             V1 IN 0 DC 0.2\n\
             .TRAN 1n 4n\n\
             {output_cards}\n\
             .END\n"
        );
        let netlist = Netlist::parse_with_options(
            &source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce TEAM wildcard deck parses");
        let namespace = crate::netlist::collect_output_node_namespace_with_limits_and_abort(
            &netlist,
            crate::resource::ResourceLimits::default(),
            &NoAbort,
        )
        .expect("TEAM public-node namespace elaborates");
        assert!(namespace.external.contains("IN"));
        assert!(!namespace.external.contains("YMEMRISTOR!MR1_X"));
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .run_tran(&netlist, 4.0e-9, 1.0e-9)
            .expect("TEAM wildcard transient solves")
    }

    fn xyce_team_resistance_noise_deck(seed: i32, enabled: bool) -> Netlist {
        Netlist::parse_validated(&format!(
            "TEAM resistance RTN\n\
             V1 in 0 1\n\
             .model mrm1 memristor level=2 ron=100 roff=200 xon=0 xoff=1 \
             ion=-1 ioff=1 kon=-1 koff=1 alphaon=1 alphaoff=1 wt=0 \
             resnoise={} resseed={seed} reslambda=1 restd=0.7n reseptd=1p \
             resdelta=2 resdeltagrad=0.2\n\
             YMEMRISTOR mr1 in 0 mrm1\n\
             .tran 0.25n 8n\n\
             .end\n",
            u8::from(enabled)
        ))
        .expect("TEAM resistance-noise fixture validates")
    }

    fn team_resistance_bits(result: &TransientResult) -> Vec<u64> {
        result
            .try_store_waveform_named("YMEMRISTOR!MR1:R")
            .expect("TEAM resistance store trace")
            .iter()
            .map(|value| value.to_bits())
            .collect()
    }

    #[test]
    fn xyce_team_resistance_noise_seed_replays_and_changes_streams() {
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let deck = xyce_team_resistance_noise_deck(41, true);
        let first = engine
            .run_tran(&deck, 8.0e-9, 0.25e-9)
            .expect("first TEAM RTN transient solves");
        let repeat = engine
            .run_tran(&deck, 8.0e-9, 0.25e-9)
            .expect("repeated TEAM RTN transient solves");
        assert_eq!(first.time, repeat.time);
        assert_eq!(team_resistance_bits(&first), team_resistance_bits(&repeat));

        let different = engine
            .run_tran(&xyce_team_resistance_noise_deck(42, true), 8.0e-9, 0.25e-9)
            .expect("different-seed TEAM RTN transient solves");
        assert_ne!(
            team_resistance_bits(&first),
            team_resistance_bits(&different),
            "different RESSEED values must select different dwell trajectories"
        );
    }

    #[test]
    fn xyce_team_zero_noise_preserves_transient_bit_pattern() {
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let with_disabled_parameters = xyce_team_resistance_noise_deck(99, false);
        let baseline = Netlist::parse_validated(
            "TEAM deterministic baseline\n\
             V1 in 0 1\n\
             .model mrm1 memristor level=2 ron=100 roff=200 xon=0 xoff=1 \
             ion=-1 ioff=1 kon=-1 koff=1 alphaon=1 alphaoff=1 wt=0\n\
             YMEMRISTOR mr1 in 0 mrm1\n\
             .tran 0.25n 8n\n\
             .end\n",
        )
        .expect("deterministic TEAM baseline validates");
        let expected = engine
            .run_tran(&baseline, 8.0e-9, 0.25e-9)
            .expect("deterministic TEAM baseline solves");
        let actual = engine
            .run_tran(&with_disabled_parameters, 8.0e-9, 0.25e-9)
            .expect("disabled TEAM RTN transient solves");
        assert_eq!(expected.time, actual.time);
        assert_eq!(
            team_resistance_bits(&expected),
            team_resistance_bits(&actual)
        );
    }

    #[test]
    fn xyce_team_noise_checkpoint_resume_matches_uninterrupted_suffix() {
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let deck = xyce_team_resistance_noise_deck(71, true);
        let (uninterrupted, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &deck,
                8.0e-9,
                0.25e-9,
                TransientStartupMode::OperatingPoint,
                &[3.0e-9],
            )
            .expect("TEAM RTN checkpoint trajectory solves");
        let checkpoint = TransientCheckpoint::from_text(&scheduled[0].checkpoint.to_text())
            .expect("TEAM RTN checkpoint round-trips");
        let (resumed, _) = engine
            .run_tran_resume(&deck, &checkpoint, 8.0e-9, 0.25e-9)
            .expect("TEAM RTN checkpoint resumes");
        let seam = uninterrupted
            .time
            .iter()
            .position(|time| time.to_bits() == checkpoint.time.to_bits())
            .expect("checkpoint accepted time is present in uninterrupted result");
        assert_eq!(&uninterrupted.time[seam..], resumed.time.as_slice());
        assert_eq!(
            &team_resistance_bits(&uninterrupted)[seam..],
            team_resistance_bits(&resumed).as_slice()
        );
    }

    #[test]
    fn xyce_team_noise_honors_abort_and_checkpoint_parse_limits() {
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let deck = xyce_team_resistance_noise_deck(5, true);
        let error = engine
            .run_tran_with_abort(&deck, 8.0e-9, 0.25e-9, &crate::abort_signal::ImmediateAbort)
            .expect_err("an immediate abort must cancel TEAM RTN transient startup");
        assert!(matches!(error, SimulationError::Aborted));

        let (_, checkpoint) = engine
            .run_tran_checkpointed(&deck, 1.0e-9, 0.25e-9)
            .expect("bounded TEAM RTN checkpoint captures");
        let hostile = checkpoint.to_text().replacen(
            "xyce_team_resistance_noise_states 1",
            &format!("xyce_team_resistance_noise_states {}", usize::MAX),
            1,
        );
        let error = TransientCheckpoint::from_text(&hostile)
            .expect_err("hostile TEAM RTN checkpoint count must fail before allocation");
        assert!(
            error.contains("declares") && error.contains("rows"),
            "unexpected bounded-parser diagnostic: {error}"
        );
    }

    fn assert_team_wildcard_avoids_private_state(result: &TransientResult) {
        assert_eq!(
            result
                .try_voltage_waveform_named("IN")
                .expect("authored IN waveform exists")
                .len(),
            result.time.len()
        );
        assert!(
            result
                .try_voltage_waveform_named("YMEMRISTOR!MR1_X")
                .expect("builder-generated TEAM state remains in solver metadata")
                .is_empty(),
            "V(*) must not capture or resource-charge builder-generated TEAM state"
        );
        assert_eq!(
            result
                .voltages
                .iter()
                .filter(|trace| !trace.is_empty())
                .count(),
            1,
            "only the authored IN node may contribute a voltage trace"
        );
    }

    #[test]
    fn save_complete_voltage_wildcard_excludes_no_dot_team_private_state() {
        let wildcard = run_xyce_team_memristor_capture(".SAVE V(*)");
        assert_team_wildcard_avoids_private_state(&wildcard);

        let save_all = run_xyce_team_memristor_capture(".SAVE ALL");
        assert_eq!(save_all.time, wildcard.time);
        assert_eq!(
            save_all
                .try_voltage_waveform_named("YMEMRISTOR!MR1_X")
                .expect("SAVE ALL TEAM state waveform")
                .len(),
            save_all.time.len(),
            "SAVE ALL must preserve builder-private state capture"
        );
        assert_eq!(
            save_all.voltages.iter().map(Vec::len).sum::<usize>(),
            wildcard
                .voltages
                .iter()
                .map(Vec::len)
                .sum::<usize>()
                .saturating_add(wildcard.time.len()),
            "SAVE V(*) must avoid charging exactly one TEAM state waveform"
        );
    }

    #[test]
    fn mixed_print_and_save_wildcards_exclude_no_dot_team_private_state() {
        let wildcard = run_xyce_team_memristor_capture(".PRINT TRAN V(*)\n.SAVE V(*)");
        assert_team_wildcard_avoids_private_state(&wildcard);

        let exact_private = run_xyce_team_memristor_capture(".SAVE N(YMEMRISTOR!MR1_X)");
        assert_eq!(
            exact_private
                .try_voltage_waveform_named("YMEMRISTOR!MR1_X")
                .expect("exact TEAM state waveform")
                .len(),
            exact_private.time.len(),
            "an intentional exact private-node probe must remain capturable"
        );
    }

    #[test]
    fn xyce_print_voltage_wildcard_excludes_generated_laplace_state_from_output_and_budget() {
        let parse = |output_card: &str| {
            let source = format!(
                "Xyce V(*) excludes parser-generated LAPLACE state\n\
                 VIN IN 0 PULSE(0 1 0 1n 1n 10u 20u)\n\
                 E1 OUT 0 LAPLACE {{V(IN)}} = {{1/(1+s/1e6)}}\n\
                 RLOAD OUT 0 1k\n\
                 .TRAN 100n 1u\n\
                 {output_card}\n\
                 .END\n"
            );
            Netlist::parse_with_options(
                &source,
                crate::netlist::NetlistParseOptions {
                    expression_dialect: crate::config::ExpressionDialect::Xyce,
                    ..Default::default()
                },
            )
            .expect("Xyce LAPLACE wildcard deck parses")
        };
        let engine =
            || Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));

        let wildcard_netlist = parse(".PRINT TRAN V(*)");
        let wildcard = engine()
            .run_tran(&wildcard_netlist, 1.0e-6, 1.0e-7)
            .expect("LAPLACE wildcard transient solves");
        for external in ["IN", "OUT"] {
            assert_eq!(
                wildcard
                    .try_voltage_waveform_named(external)
                    .unwrap_or_else(|| panic!("missing external waveform {external}"))
                    .len(),
                wildcard.time.len()
            );
        }
        let state = wildcard
            .try_voltage_waveform_named("E1.__X1")
            .expect("generated LAPLACE state remains in solver metadata");
        assert!(state.is_empty(), "V(*) must not capture generated state");

        let projected = crate::analysis::evaluate_tran_output_requests_with_abort(
            &wildcard_netlist,
            &wildcard,
            crate::resource::ResourceLimits::default(),
            &NoAbort,
        )
        .expect("LAPLACE V(*) output projection succeeds");
        assert_eq!(
            projected.len(),
            2,
            "only authored IN and OUT may be emitted as wildcard columns"
        );

        let save_all_netlist = parse(".SAVE ALL");
        let save_all = engine()
            .run_tran(&save_all_netlist, 1.0e-6, 1.0e-7)
            .expect("SAVE ALL LAPLACE counterfactual solves");
        assert_eq!(save_all.time, wildcard.time);
        assert_eq!(
            save_all
                .try_voltage_waveform_named("E1.__X1")
                .expect("SAVE ALL state waveform")
                .len(),
            save_all.time.len()
        );
        let retained_voltage_values = |result: &TransientResult| {
            result
                .voltages
                .iter()
                .map(Vec::len)
                .fold(0usize, usize::saturating_add)
        };
        assert_eq!(
            retained_voltage_values(&save_all),
            retained_voltage_values(&wildcard).saturating_add(wildcard.time.len()),
            "wildcard capture must avoid charging exactly one generated state voltage waveform"
        );
        assert!(
            Engine::transient_result_value_count(&save_all)
                >= Engine::transient_result_value_count(&wildcard)
                    .saturating_add(wildcard.time.len()),
            "SAVE ALL counterfactual must charge at least the generated state waveform"
        );
    }

    #[test]
    fn xyce_print_complete_voltage_wildcard_does_not_capture_bsim_private_nodes() {
        let netlist = Netlist::parse_with_options(
            "Xyce V(*) excludes legacy BSIM prime nodes\n\
             VDS D 0 0.05\n\
             VGS G 0 1.8\n\
             M1 D G 0 0 B1 L=10u W=50u\n\
             .MODEL B1 NMOS LEVEL=4 TOX=0.03 VDD=5 RSH=35\n\
             .TRAN 1n 2n\n\
             .PRINT TRAN V(*)\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce legacy BSIM wildcard deck parses");

        let baseline =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
                .run_tran(&netlist, 2.0e-9, 1.0e-9)
                .expect("external-only wildcard transient solves");
        for external in ["D", "G"] {
            let trace = baseline
                .try_voltage_waveform_named(external)
                .unwrap_or_else(|| panic!("missing external waveform {external}"));
            assert_eq!(trace.len(), baseline.time.len());
        }
        for private in ["M1.__dint", "M1.__sint"] {
            let trace = baseline
                .try_voltage_waveform_named(private)
                .unwrap_or_else(|| panic!("missing private-node metadata {private}"));
            assert!(
                trace.is_empty(),
                "V(*) must not capture or resource-charge {private}"
            );
        }

        let retained = Engine::transient_result_value_count(&baseline);
        assert_eq!(
            baseline
                .voltages
                .iter()
                .filter(|trace| !trace.is_empty())
                .count(),
            2,
            "only D and G may contribute voltage traces to the result budget"
        );
        // Leave less than two waveform lengths of headroom for projection
        // bookkeeping. Retaining both prime-node traces would necessarily
        // cross this ceiling.
        let tight_limit = retained
            .saturating_add(baseline.time.len())
            .saturating_add(baseline.time.len() / 2);
        assert!(
            retained.saturating_add(baseline.time.len().saturating_mul(2)) > tight_limit,
            "capturing both private prime-node traces would exceed the tight budget"
        );
        let mut tight = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
        tight.resource_limits.max_result_values = tight_limit;
        let bounded = Engine::new(tight)
            .run_tran(&netlist, 2.0e-9, 1.0e-9)
            .expect("external-only capture fits the exact retained-value budget");
        assert_eq!(Engine::transient_result_value_count(&bounded), retained);
        assert!(
            bounded
                .try_voltage_waveform_named("M1.__dint")
                .is_some_and(<[_]>::is_empty)
        );
        assert!(
            bounded
                .try_voltage_waveform_named("M1.__sint")
                .is_some_and(<[_]>::is_empty)
        );
    }

    #[test]
    fn xyce_nox_bad_candidate_recovery_never_reuses_a_stale_update_norm() {
        let candidate_evaluated = Cell::new(false);
        let nonfinite = xyce_nox_recovered_update_norm(true, || {
            candidate_evaluated.set(true);
            Some(7.0)
        });
        assert_eq!(nonfinite, Some(Value::INFINITY));
        assert!(!candidate_evaluated.get());

        let finite = xyce_nox_recovered_update_norm(false, || {
            candidate_evaluated.set(true);
            Some(7.0)
        });
        assert_eq!(finite, Some(7.0));
        assert!(candidate_evaluated.get());
    }

    #[test]
    fn xyce_damped_newton_correction_form_handles_a_noninductive_diode_step() {
        let netlist = Netlist::parse(
            "Xyce noninductive DampedNewton correction form\n\
             V1 in 0 PULSE(0 1 1n 100p 100p 10n 20n)\n\
             R1 in out 1k\n\
             D1 out 0 DM\n\
             .MODEL DM D(IS=1e-12 N=1)\n\
             .TRAN 250p 3n\n\
             .END\n",
        )
        .expect("diode transient deck parses");
        let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
        config.transient_nonlinear_nox = Some(false);
        let engine = Engine::new(config);
        let result = engine
            .run_tran(&netlist, 3.0e-9, 2.5e-10)
            .expect("Xyce diode transient solves");
        let output = result
            .try_voltage_waveform_named("out")
            .expect("output waveform exists");
        assert!(output.iter().all(|value| value.is_finite()));
        assert!(
            output
                .last()
                .is_some_and(|value| *value > 0.4 && *value < 0.9),
            "diode output should settle to a physical forward voltage, got {:?}",
            output.last()
        );
    }

    #[test]
    fn xyce_correction_form_preserves_exact_prescribed_behavioral_voltage() {
        let netlist = Netlist::parse(
            "Xyce prescribed behavioral voltage projection\n\
             BV1 1 0 V={SPICE_SIN(0,10,1kHz)}\n\
             R1 1 0 1\n\
             .TRAN 1u 5m\n\
             .END\n",
        )
        .expect("behavioral sine deck parses");
        let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
        config.transient_nonlinear_nox = Some(false);
        let result = Engine::new(config)
            .run_tran(&netlist, 5.0e-3, 1.0e-6)
            .expect("behavioral sine transient solves");
        let output = result
            .try_voltage_waveform_named("1")
            .expect("behavioral source output exists");

        assert_eq!(result.time.last().copied(), Some(5.0e-3));
        assert_eq!(
            output.last().copied().map(Value::to_bits),
            Some(0xbd0b_939b_afcf_cfcb),
            "accepted output must retain the expression kernel's exact value"
        );
    }

    #[test]
    fn ngspice_and_xyce_preserve_breakpoint_landing_order_then_restart_after_acceptance() {
        assert!(!Engine::breakpoint_landing_forces_order_one(
            SpiceDialect::Xyce
        ));
        assert!(!Engine::breakpoint_landing_forces_order_one(
            SpiceDialect::Ngspice
        ));
        assert!(Engine::breakpoint_landing_forces_order_one(
            SpiceDialect::BestAvailable
        ));

        let landing_order = Engine::step_trapezoidal_order(
            IntegrationMethod::Trapezoidal,
            2,
            Engine::breakpoint_landing_forces_order_one(SpiceDialect::Ngspice),
        );
        assert_eq!(landing_order, 2);

        let leaving_order =
            Engine::next_trapezoidal_order_after_accepted_step(landing_order, true, true);
        assert_eq!(leaving_order, 1);

        assert_eq!(
            Engine::step_trapezoidal_order(IntegrationMethod::Trapezoidal, 2, false),
            2
        );
        assert_eq!(
            Engine::step_trapezoidal_order(IntegrationMethod::Trapezoidal, 2, true),
            1
        );
    }

    #[test]
    fn xyce_authored_output_and_timeint_points_land_without_replacing_adaptive_history() {
        fn run_with_options(options: &str) -> (Netlist, TransientResult) {
            let source = format!(
                "Xyce authored transient schedule\n\
                 VOFF A 0 2.0\n\
                 VEXP 1 A EXP(0.0 5.0 0 1ms 1s)\n\
                 R1 1 2 1\n\
                 C1 2 0 1\n\
                 .TRAN 1ms 5ms\n\
                 {options}\n\
                 .END\n"
            );
            let netlist = Netlist::parse(&source).expect("scheduled transient deck parses");
            let engine =
                Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
            let result = engine
                .run_tran(&netlist, 5.0e-3, 1.0e-3)
                .expect("scheduled transient solves adaptively");
            (netlist, result)
        }

        let requested = [1.0e-3, 2.0e-3, 3.0e-3, 4.0e-3];
        let (output_netlist, output_result) =
            run_with_options(".OPTIONS OUTPUT OUTPUTTIMEPOINTS=1ms,2ms,3ms,4ms");
        let (_, breakpoint_result) =
            run_with_options(".OPTIONS TIMEINT BREAKPOINTS=1ms,2ms,3ms,4ms");

        for result in [&output_result, &breakpoint_result] {
            assert_eq!(result.time.first().copied(), Some(0.0));
            assert_eq!(result.time.last().copied(), Some(5.0e-3));
            assert!(
                result.time.len() > 6,
                "authored stops must preserve adaptive accepted history: {:?}",
                result.time
            );
            for requested_time in requested {
                assert!(
                    result
                        .time
                        .binary_search_by(|time| time.total_cmp(&requested_time))
                        .is_ok(),
                    "solver missed authored breakpoint {requested_time}: {:?}",
                    result.time
                );
            }

            let voltage = result
                .try_voltage_waveform_named("1")
                .expect("source node waveform exists");
            for requested_time in requested.into_iter().chain([5.0e-3]) {
                let index = result
                    .time
                    .binary_search_by(|time| time.total_cmp(&requested_time))
                    .expect("requested time is retained");
                let expected = 2.0 + 5.0 * (1.0 - (-requested_time / 1.0e-3).exp());
                assert!(
                    (voltage[index] - expected).abs() <= 1.0e-9,
                    "t={requested_time}: got {}, expected {expected}",
                    voltage[index]
                );
            }
        }

        let output = output_result
            .output_projection(
                &output_netlist.options.output_time_points,
                output_netlist.options.output_interval_schedule.as_ref(),
                0.0,
                5.0e-3,
                SimulationConfig::default()
                    .resource_limits
                    .max_analysis_points,
            )
            .expect("OUTPUTTIMEPOINTS projection succeeds");
        assert_eq!(output.times().len(), 5);
        assert_eq!(output_result.time.len(), breakpoint_result.time.len());
    }

    #[test]
    fn xyce_output_schedule_fails_closed_for_unimplemented_time_window_edges() {
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        for analysis in [".TRAN 100u 2ms 500u", ".TRAN 100u 2ms\n.TRAN 100u 3ms"] {
            let netlist = Netlist::parse(&format!(
                "Xyce output schedule edge\n\
                 V1 1 0 1\n\
                 R1 1 0 1\n\
                 {analysis}\n\
                 .OPTIONS OUTPUT OUTPUTTIMEPOINTS=0,1ms\n\
                 .END\n"
            ))
            .expect("scheduled edge deck parses");
            let error = engine
                .run_tran(&netlist, 2.0e-3, 100.0e-6)
                .expect_err("ambiguous output window must fail closed");
            assert!(error.to_string().contains("OUTPUTTIMEPOINTS"));
        }

        let future = Netlist::parse(
            "Xyce future output schedule\n\
             V1 1 0 1\n\
             R1 1 0 1\n\
             .TRAN 100u 2ms\n\
             .OPTIONS OUTPUT OUTPUTTIMEPOINTS=1ms,3ms\n\
             .END\n",
        )
        .expect("future schedule parses");
        let error = engine
            .run_tran(&future, 2.0e-3, 100.0e-6)
            .expect_err("future output schedule must fail closed");
        assert!(error.to_string().contains("exceeds"));

        let tolerance_collision = Netlist::parse(
            "Xyce colliding output schedule\n\
             V1 1 0 1\n\
             R1 1 0 1\n\
             .TRAN 100u 2ms\n\
             .OPTIONS OUTPUT OUTPUTTIMEPOINTS=1e-21\n\
             .OPTIONS TIMEINT BREAKPOINTS=5e-21\n\
             .END\n",
        )
        .expect("colliding schedule parses");
        let error = engine
            .run_tran(&tolerance_collision, 2.0e-3, 100.0e-6)
            .expect_err("nonexact tolerance-colliding output stop must fail closed");
        assert!(error.to_string().contains("exactly representable"));
    }

    #[test]
    fn xyce_node_ic_constrains_the_complete_linear_t0_solution() {
        let netlist = Netlist::parse(
            "Xyce direct and scoped node IC constraints\n\
             V1 direct_source 0 0\n\
             R1 direct_source direct_out 1k\n\
             R2 direct_ic direct_out 10\n\
             C1 direct_tail direct_ic 1u\n\
             R3 direct_tail 0 10\n\
             .IC V(direct_ic)=0.5\n\
             V2 scoped_source 0 0\n\
             R4 scoped_source scoped_out 1k\n\
             X1 scoped_out scoped_tail IC_CELL PARAMS: VMID=0.5\n\
             R5 scoped_tail 0 10\n\
             .SUBCKT IC_CELL in out PARAMS: VMID=5\n\
             RS in mid 10\n\
             CS mid out 1u\n\
             .IC V(mid)={VMID}\n\
             .ENDS\n\
             .TRAN 0 10u\n\
             .END\n",
        )
        .expect("linear IC deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let result = engine
            .run_tran(&netlist, 10.0e-6, 1.0e-6)
            .expect("linear IC-constrained transient solves");
        let expected = 50.0 / 101.0;

        for node in ["direct_out", "scoped_out"] {
            let waveform = result
                .try_voltage_waveform_named(node)
                .expect("IC response waveform exists");
            assert!(
                (waveform[0] - expected).abs() <= 1.0e-12,
                "{node} must be solved consistently around its t=0 IC: got {:.17e}, expected {:.17e}",
                waveform[0],
                expected
            );
        }
        assert_eq!(
            result
                .try_voltage_waveform_named("direct_ic")
                .expect("direct IC node exists")[0]
                .to_bits(),
            0.5f64.to_bits()
        );
        assert_eq!(
            result
                .try_voltage_waveform_named("X1.mid")
                .expect("scoped IC node exists")[0]
                .to_bits(),
            0.5f64.to_bits()
        );
    }

    #[test]
    fn xyce_node_ic_remains_hard_during_nonlinear_t0_newton() {
        let netlist = Netlist::parse(
            "Xyce nonlinear node IC constraint\n\
             V1 source 0 0\n\
             R1 source out 1k\n\
             R2 held out 10\n\
             D1 out 0 DM\n\
             .MODEL DM D(IS=1e-12 N=1)\n\
             .IC V(held)=0.5\n\
             .TRAN 0 1u\n\
             .END\n",
        )
        .expect("nonlinear IC deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let result = engine
            .run_tran(&netlist, 1.0e-6, 1.0e-7)
            .expect("nonlinear IC-constrained transient solves");
        let held = result
            .try_voltage_waveform_named("held")
            .expect("held waveform exists")[0];
        let out = result
            .try_voltage_waveform_named("out")
            .expect("out waveform exists")[0];
        assert_eq!(held.to_bits(), 0.5f64.to_bits());

        let thermal_voltage = 0.025_864_186_384_551_46;
        let diode_current = 1.0e-12 * ((out / thermal_voltage).exp() - 1.0);
        let kcl = out / 1.0e3 + (out - held) / 10.0 + diode_current;
        assert!(
            kcl.abs() <= 1.0e-10,
            "neighboring nonlinear t=0 state violates the IC-constrained KCL: out={out:.17e}, residual={kcl:.17e}"
        );
    }

    #[test]
    fn xyce_nodeset_remains_seed_only_for_a_linear_t0_solution() {
        let netlist = Netlist::parse(
            "Xyce linear NODESET seed\n\
             V1 source 0 0\n\
             R1 source out 1k\n\
             R2 hinted out 10\n\
             R3 hinted 0 10\n\
             .NODESET V(hinted)=0.5\n\
             .TRAN 0 1u\n\
             .END\n",
        )
        .expect("linear NODESET deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let result = engine
            .run_tran(&netlist, 1.0e-6, 1.0e-7)
            .expect("linear NODESET transient solves");
        for node in ["hinted", "out"] {
            let initial = result
                .try_voltage_waveform_named(node)
                .expect("NODESET test node exists")[0];
            assert!(
                initial.abs() <= 1.0e-14,
                "NODESET must not replace the final linear equation at {node}: {initial:.17e}"
            );
        }
    }

    #[test]
    fn xyce_capacitor_ic_floating_terminal_is_constrained_at_start_then_held_by_companion() {
        let netlist = Netlist::parse(
            "floating capacitor IC transient\n\
             V1 fixed 0 1\n\
             C1 fixed floating 1 IC=0\n\
             .TRAN 100u 1m\n\
             .END\n",
        )
        .expect("deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let result = engine
            .run_tran(&netlist, 1.0e-3, 1.0e-4)
            .expect("floating IC capacitor transient solves");
        let fixed = result
            .try_voltage_waveform_named("fixed")
            .expect("fixed waveform exists");
        let floating = result
            .try_voltage_waveform_named("floating")
            .expect("floating waveform exists");

        assert!(result.time.len() > 1);
        for (sample, (&fixed, &floating)) in fixed.iter().zip(floating).enumerate() {
            assert!(
                (fixed - 1.0).abs() <= 1.0e-10,
                "sample {sample}: fixed source changed to {fixed:.17e}"
            );
            assert!(
                (floating - 1.0).abs() <= 1.0e-10,
                "sample {sample}: IC-constrained floating node changed to {floating:.17e}"
            );
        }

        let current = result
            .try_branch_current_waveform_named("C1")
            .expect("capacitor branch-current waveform exists");
        assert_eq!(current.len(), result.time.len());
        assert!(
            current.iter().all(|value| value.abs() <= 1.0e-10),
            "zero-voltage constant capacitor should carry no current: {current:?}"
        );
    }

    #[test]
    fn xyce_capacitor_ic_branch_handoff_allows_parallel_capacitor_rc_decay() {
        let netlist = Netlist::parse(
            "Xyce IC branch to capacitor companion handoff\n\
             V1 fixed 0 1\n\
             C1 fixed out 0.5 IC=0\n\
             C2 fixed out 0.5\n\
             R1 out 0 1\n\
             .OPTIONS TIMEINT ABSTOL=1e-12 RELTOL=1e-6\n\
             .TRAN 0 2 1m\n\
             .END\n",
        )
        .expect("deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let result = engine
            .run_tran(&netlist, 2.0, 1.0e-3)
            .expect("IC branch must hand off to the transient capacitor companion");
        let out = result
            .try_voltage_waveform_named("out")
            .expect("out waveform exists");
        assert!((out[0] - 1.0).abs() <= 1.0e-10);
        for (&time, &voltage) in result.time.iter().zip(out) {
            let expected = (-time).exp();
            assert!(
                (voltage - expected).abs() <= 2.0e-3,
                "at t={time:.9e}, expected {expected:.9e}, got {voltage:.9e}"
            );
        }
    }

    #[test]
    fn xyce_behavioral_capacitor_current_uses_physical_companion_current() {
        let netlist = Netlist::parse(
            "behavioral capacitor lead current\n\
             C1 n 0 1 IC=1\n\
             R1 n 0 1\n\
             B1 sense 0 I={I(C1)}\n\
             RS sense 0 1\n\
             .TRAN 0 0.1 1m\n\
             .END\n",
        )
        .expect("deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let result = engine
            .run_tran(&netlist, 0.1, 1.0e-3)
            .expect("behavioral I(C1) transient solves");
        let capacitor_node = result.try_voltage_waveform_named("n").unwrap();
        let sense = result.try_voltage_waveform_named("sense").unwrap();
        for (&expected, &actual) in capacitor_node.iter().zip(sense) {
            assert!(
                (actual - expected).abs() <= 1.0e-8,
                "behavioral I(C1) was not the physical capacitor current: expected sense={expected}, got {actual}"
            );
        }
    }

    #[test]
    fn xyce_capacitor_current_checkpoint_resume_starts_from_restored_history() {
        let netlist = Netlist::parse(
            "checkpointed capacitor current\nC1 n 0 1 IC=1\nR1 n 0 1\n.TRAN 0 0.1 1m\n.END\n",
        )
        .expect("deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let (first, checkpoint) = engine
            .run_tran_checkpointed(&netlist, 0.05, 1.0e-3)
            .expect("first segment solves");
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 0.1, 1.0e-3)
            .expect("resumed segment solves");
        let before = *first
            .try_branch_current_waveform_named("C1")
            .unwrap()
            .last()
            .unwrap();
        let after = resumed.try_branch_current_waveform_named("C1").unwrap()[0];
        assert!(
            before.abs() > 0.5,
            "test must retain nonzero current: {before}"
        );
        assert!(
            (after - before).abs() <= 1.0e-12,
            "resume current discontinuity: before={before}, resumed={after}"
        );
    }

    #[test]
    fn authored_restart_resume_allows_only_the_dedicated_compatible_identity() {
        let first = Netlist::parse(
            "authored restart API\n\
             V1 in 0 PULSE(0 1 0 1u 1u 20u 50u)\n\
             R1 in out 1k\n\
             C1 out 0 1n\n\
             .TRAN 1u 20u\n\
             .PRINT TRAN V(out)\n\
             .OPTIONS RESTART JOB=restart_api INITIAL_INTERVAL=5u\n\
             .END\n",
        )
        .expect("checkpoint-writer deck parses");
        let restarted = Netlist::parse(
            "authored restart API\n\
             V1 in 0 PULSE(0 1 0 1u 1u 20u 50u)\n\
             R1 in out 1k\n\
             C1 out 0 1n\n\
             .TRAN 1u 50u\n\
             .PRINT TRAN V(out)\n\
             .OPTIONS RESTART FILE=restart_api2e-05\n\
             .END\n",
        )
        .expect("restart-reader deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&first, 20.0e-6, 1.0e-6)
            .expect("checkpoint-writer segment solves");

        let exact_error = engine
            .run_tran_resume(&restarted, &checkpoint, 50.0e-6, 1.0e-6)
            .expect_err("ordinary resume must remain exact-source strict");
        assert!(
            exact_error.to_string().contains("different netlist"),
            "unexpected exact resume error: {exact_error}"
        );

        let (continued, _) = engine
            .run_tran_restart_resume(&restarted, &checkpoint, 50.0e-6, 1.0e-6)
            .expect("dedicated authored-restart API accepts the compatible deck");
        assert_eq!(continued.time.first().copied(), Some(checkpoint.time));
        assert_eq!(continued.time.last().copied(), Some(50.0e-6));
        assert!(
            continued.time.len() > 1,
            "restart must advance past its seam"
        );
    }

    #[test]
    fn bug_1284_scheduled_restart_preserves_nonquiescent_lossless_line_history() {
        let deck = |stop: &str, restart: &str| {
            format!(
                "Transmission Line Circuit\n\
                 VIN 1 0 PULSE(0 5 0 0.1N 0.1N 5N 25N)\n\
                 RIN 1 2 50\n\
                 TLINE 2 0 3 0 Z0=50 TD=10N\n\
                 RL 3 0 50\n\
                 .TRAN 0.25N {stop}\n\
                 .PRINT TRAN V(2) V(3)\n\
                 {restart}\n\
                 .END\n"
            )
        };
        let baseline = Netlist::parse(&deck("50N", "")).expect("baseline parses");
        let first = Netlist::parse(&deck(
            "20N",
            ".OPTIONS RESTART JOB=trans_test INITIAL_INTERVAL=5N",
        ))
        .expect("checkpoint writer parses");
        let restarted = Netlist::parse(&deck("50N", ".OPTIONS RESTART FILE=trans_test5e-09"))
            .expect("restart reader parses");
        let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
        config.integration_method = IntegrationMethod::Gear2;
        let engine = Engine::new(config);
        let baseline_result = engine
            .run_tran(&baseline, 50.0e-9, 0.25e-9)
            .expect("uninterrupted BUG_1284 baseline solves");
        let schedule = [0.0, 5.0e-9, 10.0e-9, 15.0e-9, 20.0e-9];
        let (_, checkpoints) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &first,
                20.0e-9,
                0.25e-9,
                TransientStartupMode::OperatingPoint,
                &schedule,
            )
            .expect("one continuous first run captures the restart schedule");
        assert_eq!(checkpoints.len(), schedule.len());
        for (index, (scheduled, requested)) in checkpoints.iter().zip(schedule).enumerate() {
            assert_eq!(scheduled.nominal_time.to_bits(), requested.to_bits());
            assert!(scheduled.checkpoint.time >= requested);
            if let Some(next_nominal) = schedule.get(index + 1) {
                assert!(scheduled.checkpoint.time < *next_nominal);
            }
        }

        let checkpoint = TransientCheckpoint::from_text(&checkpoints[1].checkpoint.to_text())
            .expect("the nonquiescent 5 ns checkpoint survives serialization");
        let checkpoint_time = checkpoint.time;
        let (continued, _) = engine
            .run_tran_restart_resume(&restarted, &checkpoint, 50.0e-9, 0.25e-9)
            .expect("the 5 ns lossless-line state resumes to the extended horizon");
        assert_eq!(continued.time.first().copied(), Some(checkpoint_time));
        assert_eq!(continued.time.last().copied(), Some(50.0e-9));

        fn interpolate(time: &[Value], values: &[Value], target: Value) -> Value {
            match time.binary_search_by(|probe| probe.total_cmp(&target)) {
                Ok(index) => values[index],
                Err(upper) => {
                    let lower = upper - 1;
                    let fraction = (target - time[lower]) / (time[upper] - time[lower]);
                    values[lower] + fraction * (values[upper] - values[lower])
                }
            }
        }
        fn xyce_normalized_rms(
            good_time: &[Value],
            good: &[Value],
            test_time: &[Value],
            test: &[Value],
        ) -> Value {
            let errors = test_time
                .iter()
                .zip(test)
                .map(|(&time, &actual)| {
                    let expected = interpolate(good_time, good, time);
                    let difference = expected - actual;
                    if difference.abs() < 1.0e-12 {
                        0.0
                    } else {
                        difference / (0.01 * expected.abs() + 1.0e-12)
                    }
                })
                .collect::<Vec<_>>();
            let weighted_square = test_time
                .windows(2)
                .zip(errors.windows(2))
                .map(|(time, error)| {
                    (time[1] - time[0]) * 0.5 * (error[0] * error[0] + error[1] * error[1])
                })
                .sum::<Value>();
            (weighted_square / (test_time.last().unwrap() - test_time[0])).sqrt()
        }

        for node in ["2", "3"] {
            let rms = xyce_normalized_rms(
                &baseline_result.time,
                baseline_result
                    .try_voltage_waveform_named(node)
                    .expect("baseline node exists"),
                &continued.time,
                continued
                    .try_voltage_waveform_named(node)
                    .expect("continued node exists"),
            );
            assert!(
                rms <= 1.0,
                "BUG_1284 nonquiescent 5 ns restart exceeds xyce_verify tolerance at V({node}): RMS={rms:.17e}"
            );
        }
    }

    #[test]
    fn restart_reinstates_pending_dynamic_lossless_line_arrivals() {
        let netlist = Netlist::parse(
            "dynamic line arrivals across restart\n\
             V1 src 0 PULSE(0 1 1n 100p 100p 20n 40n)\n\
             RS src drive 50\n\
             CS drive 0 10p\n\
             T1 drive 0 out 0 Z0=50 TD=5n\n\
             RL out 0 100\n\
             .TRAN 250p 12n\n\
             .END\n",
        )
        .expect("dynamic-arrival deck parses");
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let (_, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                12.0e-9,
                0.25e-9,
                TransientStartupMode::OperatingPoint,
                &[2.0e-9],
            )
            .expect("dynamic-arrival checkpoint run solves");
        let checkpoint = &scheduled[0].checkpoint;
        assert!(
            !checkpoint.pending_tline_arrivals().is_empty(),
            "the RC-shaped launch must discover future arrivals not present in the authored source schedule"
        );
        let serialized = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect("pending arrivals survive serialization");
        assert_eq!(
            serialized.pending_tline_arrivals(),
            checkpoint.pending_tline_arrivals()
        );

        let (resumed, _) = engine
            .run_tran_resume(&netlist, &serialized, 12.0e-9, 0.25e-9)
            .expect("dynamic-arrival checkpoint resumes");
        for &arrival in serialized
            .pending_tline_arrivals()
            .iter()
            .filter(|&&arrival| arrival <= 12.0e-9)
        {
            assert!(
                resumed
                    .time
                    .binary_search_by(|time| time.total_cmp(&arrival))
                    .is_ok(),
                "resumed integration missed pending line arrival {arrival:.17e}s"
            );
        }
    }

    #[test]
    fn scheduled_checkpoints_share_the_transient_result_value_budget() {
        let netlist = Netlist::parse(
            "bounded scheduled checkpoints\n\
             V1 out 0 1\n\
             R1 out 0 1k\n\
             .TRAN 1n 50n\n\
             .END\n",
        )
        .expect("bounded checkpoint deck parses");
        let mut config = SimulationConfig::default();
        config.resource_limits.max_result_values = 500;
        let engine = Engine::new(config);
        let schedule = (0..50)
            .map(|index| index as Value * 1.0e-9)
            .collect::<Vec<_>>();
        let error = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                50.0e-9,
                1.0e-9,
                TransientStartupMode::OperatingPoint,
                &schedule,
            )
            .expect_err("aggregate checkpoint retention must obey the result-value budget");
        assert!(
            error.to_string().contains("result_values"),
            "unexpected resource diagnostic: {error}"
        );
    }

    #[test]
    fn independent_current_source_has_an_exact_transient_branch_waveform() {
        let netlist = Netlist::parse(
            "independent current source output\nI1 out 0 2m\nR1 out 0 1k\n.SAVE I(I1)\n.TRAN 1u 5u\n.END\n",
        )
        .expect("deck parses");
        let result = Engine::new(SimulationConfig::default())
            .run_tran(&netlist, 5.0e-6, 1.0e-6)
            .expect("transient solves");
        let current = result
            .try_branch_current_waveform_named("I1")
            .expect("current-source branch waveform exists");
        assert_eq!(current.len(), result.time.len());
        assert!(
            current
                .iter()
                .all(|value| (*value - 2.0e-3).abs() <= 1.0e-15),
            "current-source waveform must preserve its exact source value: {current:?}"
        );
    }

    #[test]
    fn noop_transient_starts_independent_current_source_branch_at_zero() {
        let netlist = Netlist::parse(
            "NOOP independent current source output\n\
             I1 out 0 10\n\
             R1 out 0 1\n\
             .TRAN 1 2 NOOP\n\
             .END\n",
        )
        .expect("NOOP deck parses");
        let result = Engine::new(SimulationConfig::default())
            .run_tran(&netlist, 2.0, 1.0)
            .expect("NOOP transient solves");
        let current = result
            .try_branch_current_waveform_named("I1")
            .expect("current-source branch waveform exists");
        assert!(
            current.len() >= 2,
            "NOOP transient must advance: {current:?}"
        );
        assert_eq!(current[0], 0.0, "NOOP t=0 lead current: {current:?}");
        assert_eq!(
            current[1], 10.0,
            "NOOP first-step lead current: {current:?}"
        );
    }

    #[test]
    fn transient_merit_rollback_restores_circuit_and_vbic_cache() {
        let mut circuit = crate::circuit::CircuitData::new();
        circuit.inductors.add("lcore".to_string(), 1, 0, 1, 2.0e-3);

        let mut charge_snapshot = BjtChargeSnapshot::default();
        charge_snapshot.branches[0].charge = 3.25;
        let rollback = TransientMeritRollback::Full {
            state: Box::new(circuit.nonlinear_state_snapshot()),
            cached_vbic: vec![Some(charge_snapshot)],
        };

        circuit.inductors.inductances[0] = 7.0e-3;
        let mut vbic_snapshot_cache = vec![None];

        restore_transient_merit_rollback(&mut circuit, &mut vbic_snapshot_cache, &rollback);

        assert_eq!(circuit.inductors.inductances, vec![2.0e-3]);
        assert_eq!(
            vbic_snapshot_cache[0]
                .expect("VBIC charge snapshot should be restored")
                .branches[0]
                .charge,
            3.25
        );
    }

    #[test]
    fn transient_merit_rollback_uses_compact_classic_mos_state() {
        let mut circuit = crate::circuit::CircuitData::new();
        circuit.mosfets.add(crate::device::Mosfet::new_nmos(
            "m1".to_string(),
            1,
            2,
            3,
            4,
        ));
        assert!(circuit.has_classic_mos_only_transient_nonlinearity());

        let first = [1.5, 2.0, 0.1, -0.2];
        circuit.mosfets.update_all(&first);
        circuit.mosfets.update_all(&first);
        let mut rollback = None;
        capture_transient_merit_rollback(&circuit, &[], true, &mut rollback);
        assert!(matches!(
            rollback,
            Some(TransientMeritRollback::ClassicMosOnly(ref states)) if states.len() == 1
        ));

        let checkpoint = [2.5, 3.0, -0.1, 0.25];
        circuit.mosfets.update_all(&checkpoint);
        capture_transient_merit_rollback(&circuit, &[], true, &mut rollback);
        let expected = circuit.mosfets.nonlinear_state_snapshot();

        circuit.mosfets.update_all(&[0.2, 0.4, 0.0, 0.0]);
        restore_transient_merit_rollback(
            &mut circuit,
            &mut [],
            rollback.as_ref().expect("classic MOS checkpoint exists"),
        );
        assert_eq!(circuit.mosfets.nonlinear_state_snapshot(), expected);
    }

    #[test]
    fn transient_fft_returns_typed_calibrated_spectra_in_source_order() {
        let netlist = Netlist::parse(
            "transient fft activation\n\
             V1 out 0 SIN(0 1 1k)\n\
             R1 out 0 1k\n\
             .save i(V1)\n\
             .tran 1u 1m\n\
             .fft v(out) np=128 format=unorm window=rect freq=2.1k fmin=1.3k fmax=4.1k\n\
             .fft {2*v(out)} np=128 window=hann\n\
             .end\n",
        )
        .expect("valid transient .FFT parses");

        let result = Engine::new(SimulationConfig::default())
            .run_tran(&netlist, 1.0e-3, 1.0e-6)
            .expect("typed transient .FFT executes");
        assert_eq!(result.fft_results.len(), 2);
        let unnormalized = &result.fft_results[0];
        assert_eq!(unnormalized.output_name, "V(OUT)");
        assert_eq!(unnormalized.physical_type, "voltage");
        assert_eq!(unnormalized.point_count, 128);
        assert!(unnormalized.accurate_sampling);
        assert_eq!(unnormalized.bins.len(), 65);
        assert_eq!(unnormalized.format, crate::netlist::FftFormat::Unnormalized);
        assert_eq!(
            unnormalized.mode,
            crate::netlist::XyceFftMode::HspiceCompatible
        );
        assert_eq!(unnormalized.frequency_resolution, 1.0e3);
        assert_eq!(unnormalized.fundamental_bin, 2);
        assert_eq!(unnormalized.minimum_metric_bin, 1);
        assert_eq!(unnormalized.maximum_metric_bin, 4);
        assert!((unnormalized.bins[1].magnitude - 1.0).abs() < 2.0e-4);
        assert!((unnormalized.bins[1].phase_degrees + 90.0).abs() < 0.1);
        let authored_sample_time = unnormalized.start_time + 37.0 * unnormalized.sample_interval;
        assert!(
            result
                .time
                .iter()
                .any(|time| time.to_bits() == authored_sample_time.to_bits()),
            "FFT_ACCURATE default must make every uniform sample a solver stop"
        );

        let normalized_expression = &result.fft_results[1];
        assert_eq!(normalized_expression.output_name, "{2*v(out)}");
        assert_eq!(normalized_expression.physical_type, "parameter");
        assert_eq!(
            normalized_expression.format,
            crate::netlist::FftFormat::Normalized
        );
        let maximum = normalized_expression
            .bins
            .iter()
            .map(|bin| bin.magnitude)
            .fold(0.0, Value::max);
        assert!((maximum - 1.0).abs() < 1.0e-12);
        assert!(normalized_expression.coherent_gain > 0.49);
        assert!(normalized_expression.coherent_gain < 0.5);

        let compressed = Engine::new(SimulationConfig::default())
            .run_tran_compressed(&netlist, 1.0e-3, 1.0e-6, CompressionConfig::default())
            .expect("compressed transient retains pre-decimation FFT results");
        assert_eq!(compressed.fft_results, result.fft_results);
        let expanded = TransientResult::from(compressed);
        assert_eq!(expanded.fft_results, result.fft_results);
    }

    #[test]
    fn transient_fft_mode_one_selects_periodic_windows_and_unorm_default() {
        let netlist = Netlist::parse(
            "spectre-compatible fft mode\n\
             V1 out 0 SIN(0 1 1k)\n\
             R1 out 0 1k\n\
             .options fft fft_mode=1 fft_accurate=0 fftout=1\n\
             .tran 1u 1m\n\
             .fft v(out) np=128 window=hann\n\
             .end\n",
        )
        .expect("FFT_MODE=1 transient deck parses");

        let result = Engine::new(SimulationConfig::default())
            .run_tran(&netlist, 1.0e-3, 1.0e-6)
            .expect("FFT_MODE=1 transient executes");
        let spectrum = &result.fft_results[0];
        assert_eq!(
            spectrum.mode,
            crate::netlist::XyceFftMode::SpectreCompatible
        );
        assert!(!spectrum.accurate_sampling);
        assert_eq!(spectrum.format, crate::netlist::FftFormat::Unnormalized);
        assert!((spectrum.coherent_gain - 0.5).abs() < 1.0e-14);
        assert!((spectrum.bins[1].magnitude - 1.0).abs() < 2.0e-4);
        let metrics = spectrum
            .metrics
            .as_ref()
            .expect("FFTOUT=1 emits typed metrics");
        assert!((metrics.fundamental_magnitude - 1.0).abs() < 2.0e-4);
        assert!((metrics.thd_ratio - 0.5).abs() < 2.0e-4);
        assert!((metrics.sfdr_db - 20.0 * 2.0_f64.log10()).abs() < 1.0e-3);
        assert_eq!(metrics.sfdr_spur_bin, Some(2));
        assert_eq!(metrics.largest_harmonics.len(), 30);
        assert_eq!(metrics.largest_harmonics[0].bin, 1);
        assert_eq!(metrics.largest_harmonics[0].rank, 1);
    }

    #[test]
    fn transient_fft_preflights_window_and_resource_contracts() {
        let invalid_window = Netlist::parse(
            "invalid fft window\nV1 out 0 1\nR1 out 0 1k\n.tran 1u 1m\n.fft v(out) np=8 start=900u stop=800u\n.end\n",
        )
        .expect("typed invalid runtime window parses");
        let error = Engine::new(SimulationConfig::default())
            .run_tran(&invalid_window, 1.0e-3, 1.0e-6)
            .expect_err("reversed FFT windows fail before simulation");
        assert!(
            error
                .to_string()
                .contains("STOP must be finite and greater than START")
        );

        let oversized = Netlist::parse(
            "oversized fft\nV1 out 0 1\nR1 out 0 1k\n.tran 1u 1m\n.fft v(out) np=128\n.end\n",
        )
        .expect("bounded FFT deck parses");
        let mut config = SimulationConfig::default();
        config.resource_limits.max_analysis_points = 64;
        let error = Engine::new(config)
            .run_tran(&oversized, 1.0e-3, 1.0e-6)
            .expect_err("FFT point limit must be enforced");
        assert!(matches!(error, SimulationError::ResourceLimit(_)));

        let cumulative = Netlist::parse(
            "cumulative fft limit\nV1 out 0 1\nR1 out 0 1k\n.tran 1u 1m\n.fft v(out) np=8\n.fft i(V1) np=8\n.end\n",
        )
        .expect("multiple bounded FFT directives parse");
        let mut config = SimulationConfig::default();
        config.resource_limits.max_result_values = 49;
        let error = Engine::new(config)
            .run_tran(&cumulative, 1.0e-3, 1.0e-6)
            .expect_err("FFT result preflight must account for all spectra cumulatively");
        assert!(matches!(error, SimulationError::ResourceLimit(_)));

        let constant = Netlist::parse(
            "constant fft metrics\nV1 out 0 1\nR1 out 0 1k\n.options fft fftout=1\n.tran 1u 1m\n.fft v(out) np=8\n.end\n",
        )
        .expect("constant FFT deck parses");
        let error = Engine::new(SimulationConfig::default())
            .run_tran(&constant, 1.0e-3, 1.0e-6)
            .expect_err("undefined constant-signal FFTOUT metrics fail closed");
        assert!(
            error
                .to_string()
                .contains("FFTOUT metrics require a finite first-harmonic magnitude")
        );
    }

    #[test]
    fn compressed_transient_enforces_interpolation_error_bound() {
        let time = (0..=1000)
            .map(|index| index as Value / 1000.0)
            .collect::<Vec<_>>();
        let waveform = time
            .iter()
            .map(|time| 1.0 - (-8.0 * time).exp())
            .collect::<Vec<_>>();
        let result = TransientResult {
            time: time.clone(),
            step_sizes: vec![0.0; time.len()],
            voltages: vec![waveform.clone()],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };
        let config = CompressionConfig {
            abs_tol: 1e-6,
            rel_tol: 1e-3,
            enabled: true,
            min_interval: 0.0,
        };
        let compressed = compress_transient_result(&result, &config, &NoAbort)
            .expect("well-formed waveform compresses");
        assert!(compressed.time.len() < time.len() / 4);
        for (index, &sample_time) in time.iter().enumerate() {
            let reconstructed = compressed
                .interpolate(0, sample_time)
                .expect("interpolates");
            let tolerance = config.abs_tol + config.rel_tol * waveform[index].abs();
            assert!(
                (reconstructed - waveform[index]).abs() <= tolerance * (1.0 + 1e-12),
                "sample {index}: reconstructed {reconstructed}, actual {}, tolerance {tolerance}",
                waveform[index]
            );
        }
    }

    #[test]
    fn locked_time_grid_preserves_picosecond_edges_at_large_times() {
        let grid = [
            0.0,
            4.5,
            4.500_000_000_007_75,
            4.500_000_000_01,
            4.500_000_000_01 + 2.0 * Value::EPSILON,
            f64::NAN,
        ];

        let normalized = Engine::normalized_locked_time_grid(&grid, 0.0);

        assert!(normalized.contains(&4.5));
        assert!(normalized.contains(&4.500_000_000_007_75));
        assert!(normalized.contains(&4.500_000_000_01));
        assert_eq!(
            normalized
                .iter()
                .filter(|&&time| (time - 4.500_000_000_01).abs() < 1.0e-14)
                .count(),
            1,
            "ulp-scale duplicates should still be folded: {normalized:?}"
        );
    }

    #[test]
    fn locked_time_grid_preserves_attosecond_edges_at_nanosecond_times() {
        let grid = [
            0.0,
            8.35111251e-9,
            8.35111585e-9,
            8.35111752e-9,
            8.35111752e-9 + Value::EPSILON * 8.35111752e-9,
        ];

        let normalized = Engine::normalized_locked_time_grid(&grid, 0.0);

        assert!(normalized.contains(&8.35111251e-9));
        assert!(normalized.contains(&8.35111585e-9));
        assert!(normalized.contains(&8.35111752e-9));
        assert_eq!(
            normalized
                .iter()
                .filter(|&&time| (time - 8.35111752e-9).abs() < 1.0e-22)
                .count(),
            1,
            "ulp-scale duplicates should still be folded: {normalized:?}"
        );
    }

    #[test]
    fn locked_grid_order_restart_is_scoped_to_xyce_replay() {
        let grid = [0.0, 1.0, 2.0, 2.5, 3.0];

        assert!(Engine::dialect_requires_locked_grid_order_restart(
            SpiceDialect::Xyce,
            &grid,
            3,
            false
        ));
        assert!(!Engine::dialect_requires_locked_grid_order_restart(
            SpiceDialect::Ngspice,
            &grid,
            3,
            false
        ));
        assert!(!Engine::dialect_requires_locked_grid_order_restart(
            SpiceDialect::Xyce,
            &grid,
            3,
            true
        ));
    }

    #[test]
    fn locked_time_grid_uses_internal_steps_between_targets() {
        let deck = "Xyce locked-grid internal-step RC\n\
                    V1 1 0 1\n\
                    R1 1 2 1K\n\
                    C1 2 0 1U\n\
                    .TRAN 1U 4U\n\
                    .END\n";
        let netlist = crate::Netlist::parse(deck).expect("deck parses");
        let grid = vec![0.0, 2.0e-6, 4.0e-6];
        let engine = Engine::new(crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            locked_time_grid: Some(std::sync::Arc::new(grid.clone())),
            ..Default::default()
        });

        let result = engine
            .run_tran(&netlist, 4.0e-6, 0.5e-6)
            .expect("transient runs");

        assert!(
            result.time.len() > grid.len(),
            "locked targets should retain bounded internal points: {:?}",
            result.time
        );
        for &target in &grid[1..] {
            assert!(
                result
                    .time
                    .iter()
                    .any(|&time| (time - target).abs() <= 1.0e-18),
                "missing locked target {target:.12e}: {:?}",
                result.time
            );
        }
        assert!(result.time.windows(2).all(|pair| pair[1] > pair[0]));
    }

    #[test]
    fn xyce_locked_gear12_promotes_only_after_order_two_ratio_passes() {
        let deck = "Xyce Gear12 locked-grid RC ramp\n\
                    VIN 1 0 PULSE(0 1 10U 1U 1U 80U)\n\
                    R1 1 2 1K\n\
                    C1 2 0 20N\n\
                    .TRAN 0.5U 11U\n\
                    .END\n";
        let netlist = crate::Netlist::parse(deck).expect("deck parses");
        let grid = vec![
            10.0e-6, 10.1e-6, 10.2e-6, 10.3e-6, 10.4e-6, 10.5e-6, 10.6e-6, 10.7e-6, 10.8e-6,
            10.9e-6, 11.0e-6,
        ];
        let engine = Engine::new(crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            integration_method: IntegrationMethod::Gear2,
            transient_initial_timestep: Some(0.5e-6),
            locked_time_grid: Some(std::sync::Arc::new(grid)),
            ..Default::default()
        });

        let result = engine
            .run_tran(&netlist, 11.0e-6, 0.5e-6)
            .expect("transient runs");
        let node2 = result
            .node_names
            .iter()
            .position(|name| name == "2")
            .expect("node 2 is present");
        let voltage_at = |time: Value| -> Value {
            let index = result
                .time
                .iter()
                .position(|sample| (*sample - time).abs() <= 1.0e-18)
                .unwrap_or_else(|| panic!("missing sample at {time:.12e}: {:?}", result.time));
            result.voltages[node2][index]
        };

        let alpha = 0.1e-6 / (1.0e3 * 20.0e-9);
        let backward_euler =
            |previous: Value, input: Value| (previous + alpha * input) / (1.0 + alpha);
        let bdf2 = |previous: Value, previous_previous: Value, input: Value| {
            (2.0 * previous - 0.5 * previous_previous + alpha * input) / (1.5 + alpha)
        };
        let v_10_1 = backward_euler(0.0, 0.1);
        let v_10_2 = backward_euler(v_10_1, 0.2);
        let v_10_3 = backward_euler(v_10_2, 0.3);
        let v_10_4 = backward_euler(v_10_3, 0.4);
        let v_10_5 = backward_euler(v_10_4, 0.5);
        let v_10_6 = bdf2(v_10_5, v_10_4, 0.6);

        assert!((voltage_at(10.1e-6) - v_10_1).abs() < 1.0e-14);
        assert!((voltage_at(10.2e-6) - v_10_2).abs() < 1.0e-14);
        assert!((voltage_at(10.3e-6) - v_10_3).abs() < 1.0e-14);
        assert!((voltage_at(10.4e-6) - v_10_4).abs() < 1.0e-14);
        assert!((voltage_at(10.5e-6) - v_10_5).abs() < 1.0e-14);
        assert!((voltage_at(10.6e-6) - v_10_6).abs() < 1.0e-14);
    }

    #[test]
    fn gear2_order_one_charge_companions_are_backward_euler() {
        let dt = 0.5;
        let capacitance = 2.0;
        let q_curr = 3.0;
        let q_prev = 2.0;
        let q_prev_prev = 0.5;
        let cq_prev = 0.25;

        let backward_euler = CompanionCoefficients::backward_euler();
        let gear_order_one = CompanionCoefficients::for_method(Engine::effective_companion_method(
            IntegrationMethod::Gear2,
            1,
        ));
        let gear_order_two = CompanionCoefficients::gear2();
        let backward_euler_geq = Engine::jfet_companion_geq(&backward_euler, capacitance, dt);
        let gear_order_one_geq = Engine::jfet_companion_geq(&gear_order_one, capacitance, dt);
        let gear_order_two_geq = Engine::jfet_companion_geq(&gear_order_two, capacitance, dt);
        assert_eq!(gear_order_one_geq, backward_euler_geq);
        assert_ne!(gear_order_two_geq, backward_euler_geq);

        let backward_euler_ccap =
            Engine::jfet_companion_ccap(&backward_euler, dt, q_curr, q_prev, q_prev_prev, cq_prev);
        let gear_order_one_ccap =
            Engine::jfet_companion_ccap(&gear_order_one, dt, q_curr, q_prev, q_prev_prev, cq_prev);
        let gear_order_two_ccap =
            Engine::jfet_companion_ccap(&gear_order_two, dt, q_curr, q_prev, q_prev_prev, cq_prev);
        assert_eq!(gear_order_one_ccap, backward_euler_ccap);
        assert_ne!(gear_order_two_ccap, backward_euler_ccap);

        let backward_euler_ieq =
            Engine::linear_charge_history_ieq(&backward_euler, dt, q_prev, q_prev_prev, cq_prev);
        let gear_order_one_ieq =
            Engine::linear_charge_history_ieq(&gear_order_one, dt, q_prev, q_prev_prev, cq_prev);
        let gear_order_two_ieq =
            Engine::linear_charge_history_ieq(&gear_order_two, dt, q_prev, q_prev_prev, cq_prev);
        assert_eq!(gear_order_one_ieq, backward_euler_ieq);
        assert_ne!(gear_order_two_ieq, backward_euler_ieq);
    }

    #[test]
    fn unequal_step_gear2_nonlinear_charge_companion_uses_trial_coefficients() {
        let dt = 2.0;
        let previous_dt = 1.0;
        let coeff = CompanionCoefficients::gear2_variable_step(dt, previous_dt);
        let v_curr = 3.0;
        let q_curr = v_curr * v_curr;
        let q_prev = 4.0;
        let q_prev_prev = 1.0;
        let dq_dv = 2.0 * v_curr;

        let (geq, ieq, returned_q, cq) = Engine::nonlinear_charge_companion_terms(
            &coeff,
            dt,
            dq_dv,
            v_curr,
            q_curr,
            q_prev,
            q_prev_prev,
            0.0,
        );
        let expected_cq = (5.0 / 3.0 * q_curr - 3.0 * q_prev + 4.0 / 3.0 * q_prev_prev) / dt;
        let expected_geq = 5.0 / 3.0 * dq_dv / dt;

        assert!((cq - expected_cq).abs() <= 16.0 * Value::EPSILON);
        assert!((geq - expected_geq).abs() <= 16.0 * Value::EPSILON);
        assert!((ieq - (geq * v_curr - expected_cq)).abs() <= 16.0 * Value::EPSILON);
        assert_eq!(returned_q, q_curr);

        let fixed_step_cq = (1.5 * q_curr - 2.0 * q_prev + 0.5 * q_prev_prev) / dt;
        let comparison_scale = cq.abs().max(fixed_step_cq.abs()).max(1.0);
        assert!(
            (cq - fixed_step_cq).abs() > 128.0 * Value::EPSILON * comparison_scale,
            "unequal-step Gear2 must not silently reconstruct fixed-step BDF2"
        );
    }

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

    #[test]
    fn nonlinear_terminal_activity_limit_respects_voltage_lte_exclusions() {
        let mut circuit = crate::circuit::CircuitData::new();
        circuit.mosfets.add(crate::device::Mosfet::new_nmos(
            "M1".to_string(),
            1,
            2,
            3,
            0,
        ));

        let accepted = [0.0, 0.0, 0.0];
        let gate_only_step = [0.0, 1.0, 0.0];
        let included = Engine::nonlinear_terminal_solution_indices(&circuit, &[false; 3]);
        assert_eq!(included, [0, 1, 2]);
        let limited = Engine::nonlinear_terminal_activity_limit(
            &included,
            &accepted,
            &gate_only_step,
            1.0e-9,
            0.4,
        )
        .expect("unexcluded gate motion should limit timestep");
        assert!((limited - 4.0e-10).abs() < 1.0e-18);

        let excluded_gate = [false, true, false];
        let included = Engine::nonlinear_terminal_solution_indices(&circuit, &excluded_gate);
        assert_eq!(included, [0, 2]);
        assert!(
            Engine::nonlinear_terminal_activity_limit(
                &included,
                &accepted,
                &gate_only_step,
                1.0e-9,
                0.4,
            )
            .is_none(),
            "voltage-LTE-excluded ideal source nodes are not solved dynamics"
        );

        let drain_and_gate_step = [1.0, 1.0, 0.0];
        let limited = Engine::nonlinear_terminal_activity_limit(
            &included,
            &accepted,
            &drain_and_gate_step,
            1.0e-9,
            0.4,
        )
        .expect("unexcluded nonlinear terminal motion must still limit timestep");
        assert!((limited - 4.0e-10).abs() < 1.0e-18);
    }

    #[test]
    fn proactive_source_ramp_cap_matches_terminal_activity_contract() {
        let dt = 2.0e-9;
        let source_delta = 1.8;
        let limited = Engine::bias_transient_step_for_source_activity(
            dt,
            dt,
            false,
            source_delta,
            source_delta,
            crate::constants::DEVICE_ACTIVITY_STEP_BOUND,
            1.0e-15,
            1.0e-12,
            false,
            true,
        );
        let expected = dt * crate::constants::DEVICE_ACTIVITY_STEP_BOUND / source_delta;

        assert!((limited - expected).abs() < 1.0e-18);
        assert_eq!(
            Engine::bias_transient_step_for_source_activity(
                dt,
                dt,
                false,
                source_delta,
                source_delta,
                crate::constants::DEVICE_ACTIVITY_STEP_BOUND,
                1.0e-15,
                1.0e-12,
                false,
                false,
            ),
            dt
        );
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
        let circuit = crate::circuit::CircuitData::new();
        assert!(!Engine::should_enable_nonlinear_source_ramp_cap(
            &circuit, false
        ));
        assert!(Engine::should_enable_nonlinear_source_ramp_cap(
            &circuit, true
        ));

        let mut lossless_circuit = crate::circuit::CircuitData::new();
        lossless_circuit.tlines.push(scalar_line("TLOSSLESS"));
        assert!(Engine::should_enable_nonlinear_source_ramp_cap(
            &lossless_circuit,
            true
        ));

        let mut ltra_circuit = crate::circuit::CircuitData::new();
        let mut ltra_line = scalar_line("TLTRA");
        ltra_line.set_distributed_rlgc(0.25, 4.0, 0.0, 1.0, 1.0);
        ltra_circuit.tlines.push(ltra_line);
        assert!(!Engine::should_enable_nonlinear_source_ramp_cap(
            &ltra_circuit,
            true
        ));

        let mut txl_circuit = crate::circuit::CircuitData::new();
        let mut txl_line = scalar_line("TTXL");
        assert!(txl_line.enable_txl_runtime(12.45, 8.972e-9, 0.0, 0.468e-12, 16.0));
        txl_circuit.tlines.push(txl_line);
        assert!(!Engine::should_enable_nonlinear_source_ramp_cap(
            &txl_circuit,
            true
        ));
    }

    #[test]
    fn conservative_ngspice_breakpoint_restart_is_capability_scoped() {
        let empty = crate::circuit::CircuitData::new();
        assert!(!Engine::requires_conservative_ngspice_breakpoint_restart(
            &empty
        ));

        let mut delay_line = crate::circuit::CircuitData::new();
        delay_line.tlines.push(scalar_line("T1"));
        assert!(Engine::requires_conservative_ngspice_breakpoint_restart(
            &delay_line
        ));

        let hfet = Netlist::parse(
            "HFET restart policy\nZ1 1 2 0 HM L=1u W=10u\n.model HM NHFET LEVEL=5\n.end",
        )
        .expect("HFET fixture parses");
        let hfet_circuit = Engine::default()
            .build_circuit(&hfet)
            .expect("HFET fixture builds");
        assert!(Engine::requires_conservative_ngspice_breakpoint_restart(
            &hfet_circuit
        ));
        assert_eq!(
            Engine::source_ramp_tracking_delta(&hfet_circuit, 0.35),
            HFET_SOURCE_RAMP_TRACKING_DELTA
        );

        let jfet = Netlist::parse("JFET restart policy\nJ1 1 2 0 JM\n.model JM NJF\n.end")
            .expect("JFET fixture parses");
        let jfet_circuit = Engine::default()
            .build_circuit(&jfet)
            .expect("JFET fixture builds");
        assert!(!Engine::requires_conservative_ngspice_breakpoint_restart(
            &jfet_circuit
        ));
        assert_eq!(
            Engine::source_ramp_tracking_delta(&jfet_circuit, 0.35),
            0.35
        );
    }

    #[test]
    fn transient_newton_iteration_budget_is_dialect_specific() {
        // ngspice NIiter floors every Newton call to 100 iterations.
        let mut engine = Engine::default();
        assert_eq!(
            engine.transient_newton_iteration_budget(false),
            NGSPICE_NIITER_MIN_ITERATIONS
        );
        engine.config.transient_max_iterations = 250;
        assert_eq!(engine.transient_newton_iteration_budget(false), 250);
        assert_eq!(engine.transient_newton_iteration_budget(true), 128);

        engine.config.spice_dialect = SpiceDialect::Xyce;
        engine.config.transient_nonlinear_max_iterations = None;
        assert_eq!(engine.transient_newton_iteration_budget(false), 20);
        engine.config.transient_nonlinear_max_iterations = Some(7);
        assert_eq!(engine.transient_newton_iteration_budget(false), 7);
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

    #[test]
    fn derived_current_accessors_retain_transient_device_waveforms() {
        for accessor in ["IR", "II", "IM", "IP", "IDB"] {
            let deck = format!(
                "selective derived-current capture\n\
                 V1 1 0 1\n\
                 R1 1 0 1k\n\
                 .TRAN 1n 2n\n\
                 .PRINT TRAN {accessor}(R1)\n\
                 .END\n"
            );
            let netlist = crate::Netlist::parse(&deck).expect("derived-current deck parses");
            assert!(!netlist.saves.keeps_everything());
            assert!(
                netlist
                    .output_requests
                    .iter()
                    .any(|request| request.selects_transient_device_current("R1"))
            );

            let result = Engine::default()
                .run_tran(&netlist, 2.0e-9, 1.0e-9)
                .expect("derived-current transient runs");
            let branch = result
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("R1"))
                .expect("R1 raw branch current retained");
            let waveform = &result.branch_currents[branch];
            assert_eq!(waveform.len(), result.time.len());
            let current = waveform.last().copied().expect("R1 has accepted samples");
            assert!(
                (current - 1.0e-3).abs() <= 1.0e-12,
                "{accessor} retained nonphysical R1 current {current:.12e}"
            );
        }
    }

    #[test]
    fn measure_retention_is_independent_of_print_for_lead_bsrc_currents() {
        let run = |print: bool| {
            let print = if print {
                ".PRINT TRAN {I(VS)-I(R1)} {I(B2)-I(R2)}\n"
            } else {
                ""
            };
            let source = format!(
                "measurement-owned BSRC lead-current retention\n\
                 VS 1 0 PWL(0 0 1n 1 2n 4)\n\
                 R1 0 1 1\n\
                 B2 2 0 V={{SQRT(V(1))}}\n\
                 R2 0 2 1\n\
                 .TRAN 1n 2n\n\
                 {print}\
                 .MEASURE TRAN max_source MAX {{ABS(I(VS)-I(R1))}}\n\
                 .MEASURE TRAN rms_source RMS {{I(VS)-I(R1)}}\n\
                 .MEASURE TRAN max_behavioral MAX {{ABS(I(B2)-I(R2))}}\n\
                 .MEASURE TRAN rms_behavioral RMS {{I(B2)-I(R2)}}\n\
                 .END\n"
            );
            let netlist = Netlist::parse_with_options(
                &source,
                crate::netlist::NetlistParseOptions {
                    expression_dialect: crate::config::ExpressionDialect::Xyce,
                    ..Default::default()
                },
            )
            .expect("lead-BSRC measurement deck parses");
            let result =
                Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
                    .run_tran(&netlist, 2.0e-9, 1.0e-9)
                    .expect("lead-BSRC measurement transient runs");
            let measurements = crate::analysis::evaluate_tran_measurements(&netlist, &result);
            (result, measurements)
        };

        let (with_print, with_print_measurements) = run(true);
        let (without_print, without_print_measurements) = run(false);
        assert_eq!(without_print.time, with_print.time);
        assert_eq!(without_print.node_names, with_print.node_names);
        assert_eq!(without_print.voltages, with_print.voltages);
        assert_eq!(without_print.branch_names, with_print.branch_names);
        assert_eq!(without_print.branch_currents, with_print.branch_currents);
        for branch in ["VS", "R1", "B2", "R2"] {
            let waveform = without_print
                .try_branch_current_waveform_named(branch)
                .unwrap_or_else(|| panic!("measurement did not retain {branch} current"));
            assert_eq!(waveform.len(), without_print.time.len());
        }
        assert_eq!(without_print_measurements.len(), 4);
        assert_eq!(
            without_print_measurements
                .iter()
                .map(|measurement| measurement.name.as_str())
                .collect::<Vec<_>>(),
            with_print_measurements
                .iter()
                .map(|measurement| measurement.name.as_str())
                .collect::<Vec<_>>()
        );
        for measurement in without_print_measurements {
            assert!(measurement.passed, "{measurement:?}");
            assert!(
                measurement.value.is_some_and(|value| value.abs() <= 1.0e-8),
                "{measurement:?}"
            );
        }
    }

    #[test]
    fn parsed_measure_statement_directly_retains_its_device_lead_trace() {
        let source = "measurement-owned device-lead retention\n\
                      VC 0 C 5\n\
                      VB 0 B 0.75\n\
                      VE 0 E 0\n\
                      VS 0 S 0\n\
                      Q1 C B E S QMOD\n\
                      .MODEL QMOD NPN LEVEL=1 IS=3e-14 BF=130 BR=1 \
                      RB=45 RBM=45 RC=2 RE=1 CJE=0 CJC=0 CJS=0 TF=0 TR=0\n\
                      .TRAN 0.5n 1n\n\
                      .MEASURE TRAN collector MAX {ABS(IC(Q1)-I(VC))}\n\
                      .END\n";
        let mut netlist = Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("device-lead measurement deck parses");

        // A frontend may insert or preserve the typed statement without the
        // parser's provenance sidecar. Capture planning must still derive its
        // dependencies from MeasureStatement itself.
        netlist
            .output_requests
            .retain(|request| request.directive != OutputDirectiveKind::Measure);
        assert!(netlist.output_requests.is_empty());
        assert!(Engine::should_record_transient_device_op_traces(&netlist));

        let result =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
                .run_tran(&netlist, 1.0e-9, 0.5e-9)
                .expect("device-lead measurement transient runs");
        let collector = result
            .try_device_op_waveform_named("Q1", "IC")
            .expect("IC(Q1) trace retained without PRINT/SAVE or output sidecar");
        assert_eq!(collector.len(), result.time.len());
        let measurements = crate::analysis::evaluate_tran_measurements(&netlist, &result);
        assert_eq!(measurements.len(), 1);
        assert!(measurements[0].passed, "{:?}", measurements[0]);
        assert!(
            measurements[0]
                .value
                .is_some_and(|value| value.abs() <= 1.0e-6),
            "{:?}",
            measurements[0]
        );
    }

    #[test]
    fn native_diode_measurement_retains_the_accepted_total_current() {
        let source = "measurement-owned native diode total current\n\
                      VD D 0 SIN(0 0.05 100MEG)\n\
                      D1 D 0 DMOD\n\
                      .MODEL DMOD D IS=1e-30 N=1 CJO=10p VJ=1 M=0.5 TT=0\n\
                      .TRAN 0.05n 2n\n\
                      .SAVE V(D)\n\
                      .MEASURE TRAN generic_peak MAX {ABS(I(D1))}\n\
                      .MEASURE TRAN accepted_peak MAX {ABS(ID(D1))}\n\
                      .END\n";
        let mut netlist = Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("native diode measurement deck parses");

        // Preserve only the authored SAVE sidecar. Neither current is owned
        // by PRINT/SAVE after the parser's MEASURE provenance is removed.
        netlist
            .output_requests
            .retain(|request| request.directive != OutputDirectiveKind::Measure);
        assert!(!netlist.saves.keeps_everything());
        assert!(
            !netlist
                .output_requests
                .iter()
                .any(|request| request.selects_transient_device_current("D1"))
        );
        assert!(Engine::should_record_transient_device_op_traces(&netlist));

        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let result = engine
            .run_tran(&netlist, 2.0e-9, 0.05e-9)
            .expect("native diode measurement transient runs");
        let generic = result
            .try_branch_current_waveform_named("D1")
            .expect("I(D1) retained solely for its measurement");
        let accepted = result
            .try_device_op_waveform_named("D1", "ID")
            .expect("ID(D1) retained solely for its measurement");
        let voltage = result
            .try_voltage_waveform_named("D")
            .expect("authored voltage SAVE is retained");
        assert_eq!(generic.len(), result.time.len());
        assert_eq!(accepted.len(), result.time.len());
        assert_eq!(voltage.len(), result.time.len());

        let circuit = engine
            .build_circuit(&netlist)
            .expect("native diode fixture circuit builds");
        let diode = circuit
            .diodes
            .devices
            .first()
            .expect("native diode fixture contains D1");
        let mut observed_dynamic_current = false;
        for (index, ((&generic_current, &accepted_current), &diode_voltage)) in
            generic.iter().zip(accepted).zip(voltage).enumerate()
        {
            assert_eq!(
                generic_current.to_bits(),
                accepted_current.to_bits(),
                "I(D1) diverged from accepted ID(D1) at row {index}"
            );
            let static_current = diode.stamped_conduction_current(diode_voltage);
            observed_dynamic_current |= (generic_current - static_current).abs() > 1.0e-8;
        }
        assert!(
            observed_dynamic_current,
            "fixture never exercised the diode's accepted dQ/dt current"
        );

        let measurements = crate::analysis::evaluate_tran_measurements(&netlist, &result);
        assert_eq!(measurements.len(), 2);
        assert!(measurements.iter().all(|measurement| measurement.passed));
        let generic_peak = measurements[0]
            .value
            .expect("generic diode-current measurement has a value");
        let accepted_peak = measurements[1]
            .value
            .expect("accepted diode-current measurement has a value");
        assert_eq!(generic_peak.to_bits(), accepted_peak.to_bits());
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

    #[test]
    fn xyce_timeint_delmax_tightens_the_run_maximum() {
        let netlist =
            crate::Netlist::parse("TIMEINT DELMAX\nV1 1 0 1\nR1 1 2 1k\nC1 2 0 1u\n.end\n")
                .expect("deck parses");
        let engine = Engine::new(crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            transient_timeint_max_timestep: Some(3.0e-6),
            ..Default::default()
        });
        let result = engine
            .run_tran(&netlist, 30.0e-6, 20.0e-6)
            .expect("transient runs");
        let worst_dt = result
            .time
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold(0.0, Value::max);
        assert!(
            worst_dt <= 3.0e-6 + 1.0e-15,
            "TIMEINT DELMAX ignored: worst accepted dt {worst_dt:.3e}"
        );
    }

    #[test]
    fn exact_one_millisecond_configured_maximum_is_not_treated_as_unset() {
        let netlist =
            crate::Netlist::parse("explicit 1ms maximum\nV1 1 0 1\nR1 1 2 1k\nC1 2 0 1u\n.end\n")
                .expect("deck parses");
        let engine = Engine::new(crate::SimulationConfig {
            max_timestep: 1.0e-3,
            ..Default::default()
        });
        let result = engine
            .run_tran(&netlist, 4.0e-3, 2.0e-3)
            .expect("transient runs");
        let worst_dt = result
            .time
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold(0.0, Value::max);
        assert!(
            worst_dt <= 1.0e-3 + 1.0e-15,
            "exactly 1 ms was mistaken for an unset sentinel: worst dt {worst_dt:.3e}"
        );
    }

    #[test]
    fn xyce_iteration_control_disables_postsolve_gmin_deformation() {
        assert!(allows_postsolve_gmin_rescue(false));
        assert!(!allows_postsolve_gmin_rescue(true));
    }

    #[test]
    fn xyce_iteration_rejections_preserve_order_and_maxord_one_blocks_promotion() {
        assert_eq!(xyce_rejected_attempt_order(true, 2, 1), 2);
        assert_eq!(xyce_rejected_attempt_order(false, 2, 1), 1);
        assert!(!xyce_allows_order_two(1));
        assert!(xyce_allows_order_two(2));

        let netlist = crate::Netlist::parse(
            "fixed order mode one\nV1 in 0 PULSE(0 1 0 1u 1u 4u 10u)\nR1 in out 1k\nC1 out 0 1n\n.end\n",
        )
        .expect("deck parses");
        Engine::new(crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            transient_error_control: TransientErrorControl::NonlinearIterations,
            transient_timeint_min_order: 1,
            transient_timeint_max_order: 1,
            ..Default::default()
        })
        .run_tran(&netlist, 30.0e-6, 5.0e-6)
        .expect("MAXORD=1 with ERROPTION=1 runs without an order-two trial");
    }

    #[test]
    fn xyce_minord_two_does_not_change_startup_resume_or_restart_order() {
        assert_eq!(xyce_startup_or_restart_order(1), 1);
        assert_eq!(
            xyce_startup_or_restart_order(2),
            1,
            "MINORD=2 must not change startup, resume, or breakpoint restart order"
        );
    }

    #[test]
    fn xyce_mode_zero_demotion_honors_minord_two() {
        assert_eq!(
            xyce_lte_recovery_order(1, 2),
            2,
            "ERROPTION=0 demotion must not go below MINORD=2"
        );
        assert_eq!(xyce_lte_recovery_order(2, 1), 2);
    }

    #[test]
    fn xyce_iteration_retry_is_exact_eighth_below_legacy_bjt_floor() {
        let rejected_dt = 8.0e-12;
        let exact_eighth = xyce_iteration_retry_timestep(rejected_dt, 1.0e-18, 1.0);
        assert_eq!(exact_eighth, 1.0e-12);

        let legacy_floor = Some(4.0e-12);
        let legacy_raised =
            Engine::apply_retry_timestep_floor(exact_eighth, legacy_floor, rejected_dt, 1.0);
        assert_eq!(legacy_raised, 4.0e-12);
        assert!(
            exact_eighth < legacy_raised,
            "mode-1 Newton/reversal retry must bypass the legacy BJT startup floor"
        );

        assert_eq!(
            xyce_iteration_retry_timestep(rejected_dt, 2.0e-12, 1.0),
            2.0e-12,
            "the controller hard machine minimum remains authoritative"
        );
    }

    #[test]
    fn xyce_minimum_breakpoint_steps_use_fixed_span_caps_and_final_horizon() {
        let netlist = crate::Netlist::parse(
            "fixed breakpoint spans\n\
             V1 1 0 1\n\
             R1 1 0 1k\n\
             .options timeint breakpoints=0.4\n\
             .end\n",
        )
        .expect("deck parses");
        let engine = Engine::new(crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            transient_min_steps_between_breakpoints: Some(10),
            ..Default::default()
        });
        let result = engine.run_tran(&netlist, 1.0, 1.0).expect("transient runs");

        let mut before_breakpoint = 0.0_f64;
        let mut after_breakpoint = 0.0_f64;
        for window in result.time.windows(2) {
            let dt = window[1] - window[0];
            if window[1] <= 0.4 + 1.0e-12 {
                before_breakpoint = before_breakpoint.max(dt);
            } else {
                after_breakpoint = after_breakpoint.max(dt);
            }
        }
        assert!(before_breakpoint <= 0.04 + 1.0e-12, "{before_breakpoint}");
        assert!(after_breakpoint <= 0.06 + 1.0e-12, "{after_breakpoint}");
        assert!(
            result
                .time
                .iter()
                .any(|time| (*time - 0.4).abs() <= 1.0e-12),
            "the authored breakpoint must be landed on exactly: {:?}",
            result.time
        );
    }

    #[test]
    fn xyce_errop_option_one_implicitly_activates_ten_final_span_steps() {
        let netlist =
            crate::Netlist::parse("implicit breakpoint span\nV1 1 0 1\nR1 1 0 1k\n.end\n")
                .expect("deck parses");
        let engine = Engine::new(crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            transient_error_control: TransientErrorControl::NonlinearIterations,
            ..Default::default()
        });
        let result = engine.run_tran(&netlist, 1.0, 1.0).expect("transient runs");
        let worst_dt = result
            .time
            .windows(2)
            .map(|window| window[1] - window[0])
            .fold(0.0, Value::max);
        assert!(worst_dt <= 0.1 + 1.0e-12, "worst dt {worst_dt}");
    }

    #[test]
    fn xyce_errop_one_grid_is_independent_of_lte_and_charge_truncation_tolerances() {
        let netlist = crate::Netlist::parse(
            "mode-one LTE independence\nV1 in 0 PULSE(0 1 0 1u 1u 4u 10u)\nR1 in out 1k\nC1 out 0 1n\n.end\n",
        )
        .expect("RC deck parses");
        let base = crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            transient_error_control: TransientErrorControl::NonlinearIterations,
            transient_min_steps_between_breakpoints: Some(0),
            ..Default::default()
        };
        let tight = Engine::new(crate::SimulationConfig {
            transient_lte_reltol: Some(1.0e-14),
            transient_lte_abstol: Some(1.0e-18),
            transient_trtol: 1.0e-6,
            ..base.clone()
        })
        .run_tran(&netlist, 30.0e-6, 5.0e-6)
        .expect("tight-tolerance mode-one run succeeds");
        let loose = Engine::new(crate::SimulationConfig {
            transient_lte_reltol: Some(1.0),
            transient_lte_abstol: Some(1.0),
            transient_trtol: 1.0e6,
            ..base
        })
        .run_tran(&netlist, 30.0e-6, 5.0e-6)
        .expect("loose-tolerance mode-one run succeeds");

        assert_eq!(tight.time, loose.time);
        assert_eq!(tight.step_sizes, loose.step_sizes);
    }

    #[test]
    fn xyce_checkpoint_resume_reanchors_breakpoint_span_ceiling() {
        let netlist = crate::Netlist::parse(
            "checkpoint breakpoint span\nV1 1 0 1\nR1 1 0 1k\n.options timeint breakpoints=0.4\n.end\n",
        )
        .expect("deck parses");
        let engine = Engine::new(crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            transient_error_control: TransientErrorControl::NonlinearIterations,
            transient_min_steps_between_breakpoints: Some(10),
            ..Default::default()
        });
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, 0.4, 1.0)
            .expect("checkpoint seam lands on the explicit breakpoint");
        assert!((checkpoint.time - 0.4).abs() <= 1.0e-12);
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 1.0, 1.0)
            .expect("active span ceiling no longer refuses checkpoint resume");
        assert_eq!(resumed.time.first().copied(), Some(checkpoint.time));
        let worst_post_seam_dt = resumed
            .time
            .windows(2)
            .map(|window| window[1] - window[0])
            .fold(0.0, Value::max);
        assert!(
            worst_post_seam_dt <= 0.06 + 1.0e-12,
            "resume must anchor (1.0 - 0.4) / 10, got {worst_post_seam_dt}"
        );
    }

    #[test]
    fn xyce_default_errop_zero_preserves_the_existing_transient_trajectory() {
        let netlist = crate::Netlist::parse(
            "default ERROPTION\nV1 in 0 PULSE(0 1 0 1u 1u 4u 10u)\nR1 in out 1k\nC1 out 0 1n\n.end\n",
        )
        .expect("deck parses");
        let default = Engine::new(crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..Default::default()
        })
        .run_tran(&netlist, 30.0e-6, 5.0e-6)
        .expect("default Xyce transient runs");
        let explicit = Engine::new(crate::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            transient_error_control: TransientErrorControl::LocalTruncation,
            transient_min_steps_between_breakpoints: None,
            ..Default::default()
        })
        .run_tran(&netlist, 30.0e-6, 5.0e-6)
        .expect("explicit ERROPTION=0 transient runs");

        assert_eq!(default.time, explicit.time);
        assert_eq!(default.step_sizes, explicit.step_sizes);
        assert_eq!(default.voltages, explicit.voltages);
    }

    #[test]
    fn periodic_source_contract_accepts_a_complete_commensurate_sin_and_pulse_set() {
        let netlist = crate::Netlist::parse(
            "periodic sources\nVLO lo 0 SIN(0 1 1k)\nVCLK clk 0 PULSE(0 1 0 1u 1u 200u 500u)\nR1 lo 0 1k\nR2 clk 0 1k\n.end\n",
        )
        .expect("deck parses");
        let engine = Engine::new(crate::SimulationConfig::default());
        engine
            .validate_periodic_source_contract(
                &netlist,
                &["vclk".to_owned(), "VLO".to_owned()],
                1.0e3,
            )
            .expect("complete commensurate source set is valid");
    }

    #[test]
    fn periodic_source_contract_rejects_unknown_omitted_and_nonperiodic_sources() {
        let engine = Engine::new(crate::SimulationConfig::default());
        let two_sources = crate::Netlist::parse(
            "two sources\nV1 a 0 SIN(0 1 1k)\nV2 b 0 SIN(0 1 2k)\nR1 a 0 1k\nR2 b 0 1k\n.end\n",
        )
        .expect("deck parses");
        let unknown = engine
            .validate_periodic_source_contract(&two_sources, &["V3".to_owned()], 1.0e3)
            .expect_err("unknown source is rejected");
        assert!(unknown.to_string().contains("unknown independent source"));
        let omitted = engine
            .validate_periodic_source_contract(&two_sources, &["V1".to_owned()], 1.0e3)
            .expect_err("omitted driving source is rejected");
        assert!(omitted.to_string().contains("omitted: V2"));

        for (waveform, expected) in [
            ("EXP(0 1 1u 1u 2u 1u)", "non-periodic EXP"),
            ("PWL(0 0 1m 1)", "uses PWL"),
        ] {
            let deck = format!("nonperiodic source\nV1 out 0 {waveform}\nR1 out 0 1k\n.end\n");
            let netlist = crate::Netlist::parse(&deck).expect("deck parses");
            let error = engine
                .validate_periodic_source_contract(&netlist, &["V1".to_owned()], 1.0e3)
                .expect_err("non-periodic waveform is rejected");
            assert!(error.to_string().contains(expected), "{error}");
        }
    }

    #[test]
    fn periodic_source_contract_rejects_an_incommensurate_frequency() {
        let netlist = crate::Netlist::parse(
            "incommensurate source\nV1 out 0 SIN(0 1 1.1k)\nR1 out 0 1k\n.end\n",
        )
        .expect("deck parses");
        let error = Engine::new(crate::SimulationConfig::default())
            .validate_periodic_source_contract(&netlist, &["V1".to_owned()], 1.0e3)
            .expect_err("incommensurate source is rejected");
        assert!(error.to_string().contains("not an integer multiple"));
    }

    fn parse_solution_dependent_capacitor_deck(source: &str) -> Netlist {
        Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("solution-dependent capacitor deck parses")
    }

    #[test]
    fn solution_dependent_capacitor_ic_obeys_xyce_and_ngspice_startup_semantics() {
        let non_ground = parse_solution_dependent_capacitor_deck(
            "solution-dependent capacitor IC dialect semantics\n\
             VBIAS bias 0 2\n\
             VCTRL ctrl 0 1\n\
             R1 out 0 1k\n\
             C1 out bias C={1n*(1+0.25*V(ctrl))} IC=0.25\n\
             .end\n",
        );
        let xyce = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let circuit = xyce
            .build_circuit(&non_ground)
            .expect("combined Xyce expression/IC capacitor builds");
        assert!(circuit.capacitors.value_expression(0).is_some());
        assert_eq!(circuit.capacitors.ic[0], Some(0.25));
        assert!(circuit.capacitors.ic_branch_indices[0].is_some());

        let xyce_result = xyce
            .run_tran_with_startup_mode(
                &non_ground,
                1.0e-10,
                1.0e-10,
                TransientStartupMode::OperatingPoint,
            )
            .expect("Xyce constrained startup solves");
        let out = xyce_result
            .try_voltage_waveform_named("out")
            .expect("out waveform exists")[0];
        let bias = xyce_result
            .try_voltage_waveform_named("bias")
            .expect("bias waveform exists")[0];
        assert!((out - bias - 0.25).abs() <= 32.0 * Value::EPSILON);

        let grounded = parse_solution_dependent_capacitor_deck(
            "solution-dependent capacitor native seed\n\
             VCTRL ctrl 0 1\n\
             R1 out 0 1k\n\
             C1 out 0 C={1n*(1+0.25*V(ctrl))} IC=0.25\n\
             .end\n",
        );
        for dialect in [SpiceDialect::BestAvailable, SpiceDialect::Ngspice] {
            let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
            let circuit = engine.build_circuit(&grounded).unwrap_or_else(|error| {
                panic!("combined {dialect:?} expression/IC capacitor must build: {error}")
            });
            assert!(circuit.capacitors.value_expression(0).is_some());
            assert_eq!(circuit.capacitors.ic[0], Some(0.25));
            assert!(circuit.capacitors.ic_branch_indices[0].is_none());

            let ordinary = engine
                .run_tran_with_startup_mode(
                    &grounded,
                    1.0e-10,
                    1.0e-10,
                    TransientStartupMode::OperatingPoint,
                )
                .unwrap_or_else(|error| {
                    panic!("{dialect:?} operating-point startup must solve: {error}")
                });
            assert_eq!(
                ordinary
                    .try_voltage_waveform_named("out")
                    .expect("ordinary out waveform exists")[0]
                    .to_bits(),
                0.0_f64.to_bits(),
                "without UIC, {dialect:?} IC= must not become a DC voltage constraint"
            );
            let uic = engine
                .run_tran_with_startup_mode(&grounded, 1.0e-10, 1.0e-10, TransientStartupMode::Uic)
                .unwrap_or_else(|error| panic!("{dialect:?} UIC startup must solve: {error}"));
            assert_eq!(
                uic.try_voltage_waveform_named("out")
                    .expect("UIC out waveform exists")[0]
                    .to_bits(),
                0.25_f64.to_bits(),
                "UIC must seed the {dialect:?} expression-valued capacitor terminal voltage exactly"
            );
        }
    }

    #[test]
    fn rejected_solution_dependent_ic_capacitor_trial_restores_accepted_state() {
        let netlist = parse_solution_dependent_capacitor_deck(
            "solution-dependent capacitor rollback\n\
             VCTRL ctrl 0 1\n\
             R1 out 0 1k\n\
             C1 out 0 C={1n*(1+V(ctrl))} IC=0.2\n\
             .end\n",
        );
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        let mut accepted = vec![0.0; circuit.matrix_size()];
        let out = circuit.get_node_by_name("out").expect("out node");
        let ctrl = circuit.get_node_by_name("ctrl").expect("ctrl node");
        accepted[out - 1] = 0.2;
        accepted[ctrl - 1] = 1.0;
        circuit
            .capacitors
            .initialize_solution_dependent_from_dc(&accepted, 0.0);
        let accepted_states = circuit.capacitors.value_expression_states.clone();
        let accepted_effective = circuit.capacitors.effective_capacitances.clone();
        let snapshot = circuit.transient_trial_state_snapshot();

        let mut rejected = accepted.clone();
        rejected[out - 1] = -0.1;
        rejected[ctrl - 1] = 3.0;
        let mut rhs = vec![0.0; circuit.matrix_size()];
        let num_nodes = circuit.num_nodes();
        circuit
            .capacitors
            .stamp_solution_dependent_transient_companion(
                &mut matrix,
                &mut rhs,
                &rejected,
                1.0e-9,
                1.0e-9,
                &CompanionCoefficients::backward_euler(),
                num_nodes,
            )
            .expect("rejected trial stamps");
        assert_ne!(
            circuit.capacitors.effective_capacitances, accepted_effective,
            "trial evaluation must exercise mutable expression state"
        );

        circuit.restore_nonlinear_state(snapshot);
        assert_eq!(circuit.capacitors.value_expression_states, accepted_states);
        assert_eq!(
            circuit.capacitors.effective_capacitances,
            accepted_effective
        );
    }

    #[test]
    fn solution_dependent_ic_capacitor_checkpoint_resume_is_bit_exact() {
        let netlist = parse_solution_dependent_capacitor_deck(
            "solution-dependent capacitor checkpoint\n\
             VDRV src 0 PULSE(0 1 0.25n 0.25n 0.25n 1n 2n)\n\
             VCTRL ctrl 0 PULSE(0 1 0.5n 0.25n 0.25n 0.75n 2n)\n\
             R1 src out 1k\n\
             C1 out 0 C={1p*(1+0.5*V(ctrl))} IC=0.2\n\
             .end\n",
        );
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let (uninterrupted, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                4.0e-9,
                0.25e-9,
                TransientStartupMode::OperatingPoint,
                &[2.0e-9],
            )
            .expect("checkpointed trajectory solves");
        assert_eq!(scheduled.len(), 1);
        let checkpoint = TransientCheckpoint::from_text(&scheduled[0].checkpoint.to_text())
            .expect("solution-dependent capacitor checkpoint round-trips");
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 4.0e-9, 0.25e-9)
            .expect("solution-dependent capacitor checkpoint resumes");
        let seam = uninterrupted
            .time
            .iter()
            .position(|time| time.to_bits() == checkpoint.time.to_bits())
            .expect("uninterrupted result contains checkpoint seam");
        assert_eq!(resumed.time, uninterrupted.time[seam..]);
        assert_eq!(resumed.step_sizes[0].to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            resumed.step_sizes[1..],
            uninterrupted.step_sizes[seam + 1..]
        );
        for (resumed, uninterrupted) in resumed.voltages.iter().zip(&uninterrupted.voltages) {
            assert_eq!(resumed, &uninterrupted[seam..]);
        }
        for (resumed, uninterrupted) in resumed
            .branch_currents
            .iter()
            .zip(&uninterrupted.branch_currents)
        {
            assert_eq!(resumed, &uninterrupted[seam..]);
        }
    }
}
