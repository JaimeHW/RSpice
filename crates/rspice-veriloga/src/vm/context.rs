//! Per-instance runtime state for an executing model.
//!
//! [`VmContext`] holds everything that varies per device instance and per
//! timepoint: terminal voltages, resolved parameters, operator state for
//! `ddt`/`idt`, the filters and detectors in [`super::filters`], lookup
//! tables, and the [`IntegrationCoefficients`] the engine installs each step.
//! The compiled model stays shared and immutable; this is the only mutable
//! half, which is what lets a thousand instances share one compilation.
//!
//! The state follows a candidate/commit discipline. Newton re-evaluates the
//! same timepoint repeatedly, so evaluation computes from the last *accepted*
//! state and never advances history; the engine commits only once a step is
//! accepted. Any operator with memory must respect that or iteration count
//! would change results.

use super::error::VmError;
use super::filters::{
    CrossCheckpoint, CrossDetector, DelayBuffer, DelayCheckpoint, SlewCheckpoint, SlewFilter,
    TransitionCheckpoint, TransitionFilter,
};
use crate::codegen::LookupTable;
use crate::laplace::{LaplaceCheckpoint, StateSpaceFilter};
use crate::zfilter::ZiCheckpoint;

pub const CURRENT_PAIR_GROUND: usize = usize::MAX;

pub fn terminal_pair_current_index(pos: usize, neg: usize, num_terminals: usize) -> Option<usize> {
    if pos == CURRENT_PAIR_GROUND && neg == CURRENT_PAIR_GROUND {
        return None;
    }

    let width = current_pair_axis_width(num_terminals)?;
    let pos = current_pair_axis_index(pos, num_terminals)?;
    let neg = current_pair_axis_index(neg, num_terminals)?;
    pos.checked_mul(width)?.checked_add(neg)
}

pub fn terminal_pair_current_endpoints(
    pair_index: usize,
    num_terminals: usize,
) -> Option<(usize, usize)> {
    let width = current_pair_axis_width(num_terminals)?;
    let len = terminal_pair_current_len(num_terminals)?;
    if pair_index >= len {
        return None;
    }

    let pos = current_pair_endpoint(pair_index / width, num_terminals)?;
    let neg = current_pair_endpoint(pair_index % width, num_terminals)?;
    (pos != CURRENT_PAIR_GROUND || neg != CURRENT_PAIR_GROUND).then_some((pos, neg))
}

pub fn terminal_pair_current_len(num_terminals: usize) -> Option<usize> {
    if num_terminals == 0 {
        return Some(0);
    }
    let width = current_pair_axis_width(num_terminals)?;
    width.checked_mul(width)
}

fn current_pair_axis_width(num_terminals: usize) -> Option<usize> {
    (num_terminals != 0)
        .then(|| num_terminals.checked_add(1))
        .flatten()
}

fn current_pair_axis_index(endpoint: usize, num_terminals: usize) -> Option<usize> {
    if endpoint == CURRENT_PAIR_GROUND {
        Some(num_terminals)
    } else if endpoint < num_terminals {
        Some(endpoint)
    } else {
        None
    }
}

fn current_pair_endpoint(axis_index: usize, num_terminals: usize) -> Option<usize> {
    if axis_index == num_terminals {
        Some(CURRENT_PAIR_GROUND)
    } else if axis_index < num_terminals {
        Some(axis_index)
    } else {
        None
    }
}

fn terminal_pair_current_storage_len(num_terminals: usize) -> usize {
    terminal_pair_current_len(num_terminals)
        .expect("terminal-pair current table dimensions overflow")
}

/// Runtime companion coefficients for `ddt`, `idt`, and `idtmod`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegrationCoefficients {
    pub active: bool,
    pub derivative_scale: f64,
    pub previous_value_scale: f64,
    pub older_value_scale: f64,
    pub previous_derivative_scale: f64,
}

impl IntegrationCoefficients {
    pub const fn inactive() -> Self {
        Self {
            active: false,
            derivative_scale: 0.0,
            previous_value_scale: 0.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 0.0,
        }
    }

    pub fn backward_euler(timestep: f64) -> Self {
        if !timestep.is_finite() || timestep.abs() <= 1.0e-20 {
            return Self::inactive();
        }
        let inverse_timestep = 1.0 / timestep;
        Self {
            active: true,
            derivative_scale: inverse_timestep,
            previous_value_scale: inverse_timestep,
            older_value_scale: 0.0,
            previous_derivative_scale: 0.0,
        }
    }
}

impl Default for IntegrationCoefficients {
    fn default() -> Self {
        Self::inactive()
    }
}

/// Controls whether named Verilog-A limiter functions participate in an
/// evaluation.
///
/// Newton assembly must use [`Self::NewtonLimited`]. Physical residual
/// probes and small-signal analyses evaluate the unmodified proposal and do
/// not read or update Newton limiter history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerilogAEvaluationMode {
    #[default]
    NewtonLimited,
    StaticProbe,
    SmallSignal,
}

impl VerilogAEvaluationMode {
    /// Select the production default for an analysis code
    /// (0=dc, 1=ac, 2=tran, 3=noise, 4=ic).
    pub const fn default_for_analysis(analysis_type: u8) -> Self {
        match analysis_type {
            1 | 3 => Self::SmallSignal,
            _ => Self::NewtonLimited,
        }
    }

    pub(crate) const fn limiting_enabled(self) -> bool {
        matches!(self, Self::NewtonLimited)
    }
}

/// Execution context providing runtime state to the VM.
#[derive(Debug, Clone)]
pub struct VmContext {
    /// Node voltages (indexed by terminal)
    pub voltages: Vec<f64>,
    /// Internal node voltages (for nodes not in port list)
    pub internal_voltages: Vec<f64>,
    /// Branch currents
    pub currents: Vec<f64>,
    /// Solution values of branch-current unknowns (potential
    /// contributions), updated from the circuit solution each iteration
    pub branch_current_values: Vec<f64>,
    /// Terminal-pair branch current lookup table (flattened NxN matrix).
    terminal_pair_currents: Vec<f64>,
    /// Per-terminal instance connection mask used by `$port_connected`.
    pub port_connected: Vec<u8>,
    /// Parameter values (indexed by parameter)
    pub parameters: Vec<f64>,
    /// Whether each parameter was explicitly set on the instance
    /// One byte per parameter, rather than `Vec<bool>`'s packed bit storage.
    /// Native and secondary-WASM JIT ABIs both address this as a `u8` array.
    pub param_given: Vec<u8>,
    /// Variable values (indexed by variable)
    pub variables: Vec<f64>,
    /// Current simulation time
    pub time: f64,
    /// Temperature in Kelvin
    pub temperature: f64,
    /// State variable values (current timestep) - for ddt/idt
    pub state_values: Vec<f64>,
    /// State variable values (previous timestep) - for ddt/idt
    pub state_values_prev: Vec<f64>,
    /// State values from two accepted points ago (Gear-2 history).
    pub state_values_older: Vec<f64>,
    /// Candidate derivative/input values for the current point.
    pub state_derivatives: Vec<f64>,
    /// Derivative/input values at the previous accepted point.
    pub state_derivatives_prev: Vec<f64>,
    /// Per-slot flag marking state slots that have been written at least
    /// once (used by $limit to detect its first evaluation)
    pub state_initialized: Vec<bool>,
    /// Current timestep (delta t) for transient analysis
    pub timestep: f64,
    /// Companion coefficients selected by the transient solver.
    pub integration: IntegrationCoefficients,
    /// Lookup tables for $table_model interpolation
    pub lookup_tables: Vec<LookupTable>,
    /// Delay buffers for absdelay function
    /// Each buffer stores (time, value) pairs for interpolation
    pub delay_buffers: Vec<DelayBuffer>,
    /// Transition filters for `transition(...)` state
    pub transition_filters: Vec<TransitionFilter>,
    /// Slew filters for `slew(...)` state
    pub slew_filters: Vec<SlewFilter>,
    /// Cross detectors for `cross(...)` state
    pub cross_detectors: Vec<CrossDetector>,
    /// Current analysis type (0=dc, 1=ac, 2=tran, 3=noise)
    pub analysis_type: u8,
    /// Limiter behavior for the current device evaluation.
    pub evaluation_mode: VerilogAEvaluationMode,
    /// Set when any named limiter changed its proposal during the latest
    /// limited Newton evaluation.
    pub(crate) limiter_active: u8,
    /// Whether the current evaluation is the first point of the analysis.
    pub analysis_initial_step: bool,
    /// Whether the current evaluation is the final point of the analysis.
    pub analysis_final_step: bool,
    /// Laplace state-space filters
    pub laplace_filters: Vec<StateSpaceFilter>,
    /// Instance multiplicity ($mfactor): the number of parallel copies
    /// this instance represents
    pub multiplicity: f64,
    /// Z-domain (sampled-data) filters for the zi_* operators
    pub zi_filters: Vec<crate::zfilter::ZiFilter>,
    /// Earliest absolute timer event requested during the latest evaluation.
    pub(crate) timer_event_bound: f64,
}

/// Accepted, trajectory-affecting VM state. Solver proposals, integration
/// coefficients, terminal values, and other recomputable caches are excluded.
#[derive(Debug, Clone, PartialEq)]
pub struct VmAcceptedCheckpoint {
    pub time: f64,
    pub variables: Vec<f64>,
    pub state_values_prev: Vec<f64>,
    pub state_values_older: Vec<f64>,
    pub state_derivatives_prev: Vec<f64>,
    pub state_initialized: Vec<bool>,
    pub delay_buffers: Vec<DelayCheckpoint>,
    pub transition_filters: Vec<TransitionCheckpoint>,
    pub slew_filters: Vec<SlewCheckpoint>,
    pub cross_detectors: Vec<CrossCheckpoint>,
    pub laplace_filters: Vec<LaplaceCheckpoint>,
    pub zi_filters: Vec<ZiCheckpoint>,
    pub timer_event_bound: Option<f64>,
}

impl Default for VmContext {
    fn default() -> Self {
        Self {
            voltages: Vec::new(),
            internal_voltages: Vec::new(),
            currents: Vec::new(),
            branch_current_values: Vec::new(),
            terminal_pair_currents: Vec::new(),
            port_connected: Vec::new(),
            parameters: Vec::new(),
            param_given: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15, // 27C default
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            state_values_older: Vec::new(),
            state_derivatives: Vec::new(),
            state_derivatives_prev: Vec::new(),
            state_initialized: Vec::new(),
            timestep: 0.0,
            integration: IntegrationCoefficients::inactive(),
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            transition_filters: Vec::new(),
            slew_filters: Vec::new(),
            cross_detectors: Vec::new(),
            analysis_type: 0, // DC by default
            evaluation_mode: VerilogAEvaluationMode::NewtonLimited,
            limiter_active: 0,
            analysis_initial_step: false,
            analysis_final_step: false,
            laplace_filters: Vec::new(),
            multiplicity: 1.0,
            zi_filters: Vec::new(),
            timer_event_bound: f64::INFINITY,
        }
    }
}

impl VmContext {
    /// Create a new VM context with specified terminal count.
    pub fn new(num_terminals: usize) -> Self {
        Self {
            voltages: vec![0.0; num_terminals],
            internal_voltages: Vec::new(),
            currents: Vec::new(),
            branch_current_values: Vec::new(),
            terminal_pair_currents: vec![
                f64::NAN;
                terminal_pair_current_storage_len(num_terminals)
            ],
            port_connected: vec![1; num_terminals],
            parameters: Vec::new(),
            param_given: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            state_values_older: Vec::new(),
            state_derivatives: Vec::new(),
            state_derivatives_prev: Vec::new(),
            state_initialized: Vec::new(),
            timestep: 0.0,
            integration: IntegrationCoefficients::inactive(),
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            transition_filters: Vec::new(),
            slew_filters: Vec::new(),
            cross_detectors: Vec::new(),
            analysis_type: 0,
            evaluation_mode: VerilogAEvaluationMode::NewtonLimited,
            limiter_active: 0,
            analysis_initial_step: false,
            analysis_final_step: false,
            laplace_filters: Vec::new(),
            multiplicity: 1.0,
            zi_filters: Vec::new(),
            timer_event_bound: f64::INFINITY,
        }
    }

    /// Create with internal nodes.
    pub fn with_internal_nodes(num_terminals: usize, num_internal: usize) -> Self {
        Self {
            voltages: vec![0.0; num_terminals],
            internal_voltages: vec![0.0; num_internal],
            currents: Vec::new(),
            branch_current_values: Vec::new(),
            terminal_pair_currents: vec![
                f64::NAN;
                terminal_pair_current_storage_len(num_terminals)
            ],
            port_connected: vec![1; num_terminals],
            parameters: Vec::new(),
            param_given: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            state_values_older: Vec::new(),
            state_derivatives: Vec::new(),
            state_derivatives_prev: Vec::new(),
            state_initialized: Vec::new(),
            timestep: 0.0,
            integration: IntegrationCoefficients::inactive(),
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            transition_filters: Vec::new(),
            slew_filters: Vec::new(),
            cross_detectors: Vec::new(),
            analysis_type: 0,
            evaluation_mode: VerilogAEvaluationMode::NewtonLimited,
            limiter_active: 0,
            analysis_initial_step: false,
            analysis_final_step: false,
            laplace_filters: Vec::new(),
            multiplicity: 1.0,
            zi_filters: Vec::new(),
            timer_event_bound: f64::INFINITY,
        }
    }

    /// Create with state variables for transient analysis.
    pub fn with_states(num_terminals: usize, num_states: usize) -> Self {
        Self {
            voltages: vec![0.0; num_terminals],
            internal_voltages: Vec::new(),
            currents: Vec::new(),
            branch_current_values: Vec::new(),
            terminal_pair_currents: vec![
                f64::NAN;
                terminal_pair_current_storage_len(num_terminals)
            ],
            port_connected: vec![1; num_terminals],
            parameters: Vec::new(),
            param_given: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: vec![0.0; num_states],
            state_values_prev: vec![0.0; num_states],
            state_values_older: vec![0.0; num_states],
            state_derivatives: vec![0.0; num_states],
            state_derivatives_prev: vec![0.0; num_states],
            state_initialized: vec![false; num_states],
            timestep: 0.0,
            integration: IntegrationCoefficients::inactive(),
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            transition_filters: Vec::new(),
            slew_filters: Vec::new(),
            cross_detectors: Vec::new(),
            analysis_type: 0,
            evaluation_mode: VerilogAEvaluationMode::NewtonLimited,
            limiter_active: 0,
            analysis_initial_step: false,
            analysis_final_step: false,
            laplace_filters: Vec::new(),
            multiplicity: 1.0,
            zi_filters: Vec::new(),
            timer_event_bound: f64::INFINITY,
        }
    }

    /// Advance state for a new timestep (copy current to prev).
    pub fn advance_state(&mut self) -> Result<(), VmError> {
        self.validate_advance_state()?;
        self.apply_validated_advance_state();
        Ok(())
    }

    /// Validate every fallible accepted-state action without mutating the VM.
    pub(crate) fn validate_advance_state(&self) -> Result<(), VmError> {
        // Validate every sampled-filter commit before mutating any accepted
        // state. The second pass is deliberately infallible, preserving the
        // all-or-nothing contract without cloning filter histories (and
        // allocating) on every accepted timestep.
        let time = self.time;
        for (filter_id, filter) in self.zi_filters.iter().enumerate() {
            filter.validate_commit(time).map_err(|error| {
                VmError::InvalidNumericResult(format!(
                    "zi filter {filter_id} commit failed: {error}"
                ))
            })?;
        }

        Ok(())
    }

    /// Apply an accepted-state action after the circuit has validated every
    /// runtime-compiled instance. This phase is deliberately infallible.
    pub(crate) fn apply_validated_advance_state(&mut self) {
        let time = self.time;
        self.state_values_older.clone_from(&self.state_values_prev);
        self.state_values_prev.clone_from(&self.state_values);
        self.state_derivatives_prev
            .clone_from(&self.state_derivatives);
        for buffer in &mut self.delay_buffers {
            buffer.commit();
        }
        for filter in &mut self.transition_filters {
            filter.commit();
        }
        for filter in &mut self.slew_filters {
            filter.commit();
        }
        for detector in &mut self.cross_detectors {
            detector.commit();
        }
        for filter in &mut self.laplace_filters {
            filter.commit();
        }
        for filter in &mut self.zi_filters {
            filter.apply_validated_commit(time);
        }
    }

    pub(crate) fn accepted_checkpoint(&self) -> Result<VmAcceptedCheckpoint, VmError> {
        let invalid = |message: String| VmError::InvalidNumericResult(message);
        for (index, state) in self.delay_buffers.iter().enumerate() {
            state
                .validate_checkpoint_ready()
                .map_err(|error| invalid(format!("delay {index}: {error}")))?;
        }
        for (index, state) in self.transition_filters.iter().enumerate() {
            state
                .validate_checkpoint_ready()
                .map_err(|error| invalid(format!("transition {index}: {error}")))?;
        }
        for (index, state) in self.slew_filters.iter().enumerate() {
            state
                .validate_checkpoint_ready()
                .map_err(|error| invalid(format!("slew {index}: {error}")))?;
        }
        for (index, state) in self.cross_detectors.iter().enumerate() {
            state
                .validate_checkpoint_ready()
                .map_err(|error| invalid(format!("cross {index}: {error}")))?;
        }
        for (index, state) in self.laplace_filters.iter().enumerate() {
            state
                .validate_checkpoint_ready()
                .map_err(|error| invalid(format!("Laplace filter {index}: {error}")))?;
        }
        for (index, state) in self.zi_filters.iter().enumerate() {
            state
                .validate_checkpoint_ready()
                .map_err(|error| invalid(format!("Zi filter {index}: {error}")))?;
        }
        let checkpoint = VmAcceptedCheckpoint {
            time: self.time,
            variables: self.variables.clone(),
            state_values_prev: self.state_values_prev.clone(),
            state_values_older: self.state_values_older.clone(),
            state_derivatives_prev: self.state_derivatives_prev.clone(),
            state_initialized: self.state_initialized.clone(),
            delay_buffers: self
                .delay_buffers
                .iter()
                .map(DelayBuffer::checkpoint)
                .collect(),
            transition_filters: self
                .transition_filters
                .iter()
                .map(TransitionFilter::checkpoint)
                .collect(),
            slew_filters: self
                .slew_filters
                .iter()
                .map(SlewFilter::checkpoint)
                .collect(),
            cross_detectors: self
                .cross_detectors
                .iter()
                .map(CrossDetector::checkpoint)
                .collect(),
            laplace_filters: self
                .laplace_filters
                .iter()
                .map(StateSpaceFilter::checkpoint)
                .collect(),
            zi_filters: self
                .zi_filters
                .iter()
                .map(|filter| filter.checkpoint())
                .collect(),
            timer_event_bound: self
                .timer_event_step_bound()
                .map(|_| self.timer_event_bound),
        };
        self.validate_accepted_checkpoint(&checkpoint)?;
        Ok(checkpoint)
    }

    pub(crate) fn validate_accepted_checkpoint(
        &self,
        checkpoint: &VmAcceptedCheckpoint,
    ) -> Result<(), VmError> {
        let invalid = |message: String| VmError::InvalidNumericResult(message);
        if !checkpoint.time.is_finite() || checkpoint.time < 0.0 {
            return Err(invalid(
                "checkpoint time must be finite and non-negative".into(),
            ));
        }
        if checkpoint.variables.len() != self.variables.len()
            || checkpoint.state_values_prev.len() != self.state_values.len()
            || checkpoint.state_values_older.len() != self.state_values.len()
            || checkpoint.state_derivatives_prev.len() != self.state_derivatives.len()
            || checkpoint.state_initialized.len() != self.state_initialized.len()
            || checkpoint.delay_buffers.len() != self.delay_buffers.len()
            || checkpoint.transition_filters.len() != self.transition_filters.len()
            || checkpoint.slew_filters.len() != self.slew_filters.len()
            || checkpoint.cross_detectors.len() != self.cross_detectors.len()
            || checkpoint.laplace_filters.len() != self.laplace_filters.len()
            || checkpoint.zi_filters.len() != self.zi_filters.len()
        {
            return Err(invalid(
                "checkpoint VM/operator shape does not match the device".into(),
            ));
        }
        if checkpoint.variables.iter().any(|value| value.is_nan())
            || checkpoint
                .state_values_prev
                .iter()
                .chain(&checkpoint.state_values_older)
                .chain(&checkpoint.state_derivatives_prev)
                .any(|value| !value.is_finite())
        {
            return Err(invalid(
                "checkpoint VM state contains an invalid numeric value".into(),
            ));
        }
        if checkpoint
            .timer_event_bound
            .is_some_and(|bound| !bound.is_finite() || bound <= checkpoint.time)
        {
            return Err(invalid(
                "checkpoint timer bound must be finite and strictly future".into(),
            ));
        }
        for (index, (target, state)) in self
            .delay_buffers
            .iter()
            .zip(&checkpoint.delay_buffers)
            .enumerate()
        {
            let _ = target;
            DelayBuffer::validate_checkpoint(state)
                .map_err(|error| invalid(format!("delay {index}: {error}")))?;
            if state
                .samples
                .last()
                .is_some_and(|(time, _)| *time > checkpoint.time)
            {
                return Err(invalid(format!(
                    "delay {index} contains a sample later than checkpoint time"
                )));
            }
        }
        for (index, state) in checkpoint.transition_filters.iter().enumerate() {
            TransitionFilter::validate_checkpoint(state)
                .map_err(|error| invalid(format!("transition {index}: {error}")))?;
        }
        for (index, state) in checkpoint.slew_filters.iter().enumerate() {
            SlewFilter::validate_checkpoint(state)
                .map_err(|error| invalid(format!("slew {index}: {error}")))?;
            if state.prev_time > checkpoint.time {
                return Err(invalid(format!(
                    "slew {index} accepted time is later than checkpoint time"
                )));
            }
        }
        for (index, state) in checkpoint.cross_detectors.iter().enumerate() {
            CrossDetector::validate_checkpoint(state)
                .map_err(|error| invalid(format!("cross {index}: {error}")))?;
            if state.initialized && state.time > checkpoint.time {
                return Err(invalid(format!(
                    "cross {index} accepted time is later than checkpoint time"
                )));
            }
        }
        for (index, (filter, state)) in self
            .laplace_filters
            .iter()
            .zip(&checkpoint.laplace_filters)
            .enumerate()
        {
            filter
                .validate_checkpoint(state)
                .map_err(|error| invalid(format!("Laplace filter {index}: {error}")))?;
        }
        for (index, (filter, state)) in self
            .zi_filters
            .iter()
            .zip(&checkpoint.zi_filters)
            .enumerate()
        {
            filter
                .validate_checkpoint(state)
                .map_err(|error| invalid(format!("Zi filter {index}: {error}")))?;
            if state
                .accepted_time
                .is_some_and(|time| time != checkpoint.time)
            {
                return Err(invalid(format!(
                    "Zi filter {index} accepted time does not equal checkpoint time"
                )));
            }
        }
        Ok(())
    }

    pub(crate) fn restore_accepted_checkpoint(&mut self, checkpoint: &VmAcceptedCheckpoint) {
        self.time = checkpoint.time;
        self.variables.clone_from(&checkpoint.variables);
        self.state_values.clone_from(&checkpoint.state_values_prev);
        self.state_values_prev
            .clone_from(&checkpoint.state_values_prev);
        self.state_values_older
            .clone_from(&checkpoint.state_values_older);
        self.state_derivatives
            .clone_from(&checkpoint.state_derivatives_prev);
        self.state_derivatives_prev
            .clone_from(&checkpoint.state_derivatives_prev);
        self.state_initialized
            .clone_from(&checkpoint.state_initialized);
        for (target, state) in self.delay_buffers.iter_mut().zip(&checkpoint.delay_buffers) {
            target.restore_checkpoint(state);
        }
        for (target, state) in self
            .transition_filters
            .iter_mut()
            .zip(&checkpoint.transition_filters)
        {
            target.restore_checkpoint(state);
        }
        for (target, state) in self.slew_filters.iter_mut().zip(&checkpoint.slew_filters) {
            target.restore_checkpoint(state);
        }
        for (target, state) in self
            .cross_detectors
            .iter_mut()
            .zip(&checkpoint.cross_detectors)
        {
            target.restore_checkpoint(state);
        }
        for (target, state) in self
            .laplace_filters
            .iter_mut()
            .zip(&checkpoint.laplace_filters)
        {
            target.restore_checkpoint(state);
        }
        for (target, state) in self.zi_filters.iter_mut().zip(&checkpoint.zi_filters) {
            target.restore_checkpoint(state);
        }
        self.timestep = 0.0;
        self.integration = IntegrationCoefficients::inactive();
        self.analysis_type = 2;
        self.evaluation_mode = VerilogAEvaluationMode::NewtonLimited;
        self.limiter_active = 0;
        self.analysis_initial_step = false;
        self.analysis_final_step = false;
        self.timer_event_bound = checkpoint.timer_event_bound.unwrap_or(f64::INFINITY);
    }

    /// Invalidate speculative zi candidates before each complete device
    /// evaluation. Only candidates recreated by the final Newton pass may be
    /// committed when the point is accepted.
    pub(crate) fn begin_zi_evaluation(&mut self) {
        for filter in &mut self.zi_filters {
            filter.begin_evaluation();
        }
    }

    /// Tightest exact sampled-filter edge after the current time.
    pub(crate) fn zi_filter_step_bound(&self) -> Result<Option<f64>, VmError> {
        self.zi_filters
            .iter()
            .enumerate()
            .filter(|(_, filter)| filter.participates_in_transient_schedule())
            .map(|(filter_id, filter)| {
                filter.next_sample_step_bound(self.time).map_err(|error| {
                    VmError::InvalidNumericResult(format!(
                        "zi filter {filter_id} breakpoint failed: {error}"
                    ))
                })
            })
            .try_fold(None, |minimum, bound| {
                let bound = bound?;
                Ok(Some(
                    minimum.map_or(bound, |current: f64| current.min(bound)),
                ))
            })
    }

    /// Set the timestep for transient analysis.
    pub fn set_timestep(&mut self, dt: f64) {
        self.timestep = dt;
        self.set_integration_coefficients(IntegrationCoefficients::backward_euler(dt));
    }

    /// Select solver-provided companion coefficients for this timepoint.
    pub fn set_integration_coefficients(&mut self, coefficients: IntegrationCoefficients) {
        if !self.integration.active && coefficients.active {
            // The DC operating-point evaluation establishes each operator's
            // current state but is not an accepted transient step.  Promote
            // that state to both history lanes when transient integration
            // starts so a biased ddt() differentiates from the operating
            // point and idt() starts from its DC initial condition.
            self.state_values_prev.clone_from(&self.state_values);
            self.state_values_older.clone_from(&self.state_values);
            self.state_derivatives_prev
                .clone_from(&self.state_derivatives);
        }
        self.integration = coefficients;
    }

    /// Reset timer scheduling before a fresh device evaluation.
    pub(crate) fn clear_timer_event_bound(&mut self) {
        self.timer_event_bound = f64::INFINITY;
    }

    /// Record the earliest future timer event requested by any expression.
    pub(crate) fn request_timer_event(&mut self, event_time: f64) {
        if event_time.is_finite() && event_time > self.time {
            self.timer_event_bound = self.timer_event_bound.min(event_time);
        }
    }

    /// Maximum next step needed to land on the earliest scheduled timer.
    pub(crate) fn timer_event_step_bound(&self) -> Option<f64> {
        let bound = self.timer_event_bound - self.time;
        (bound.is_finite() && bound > 0.0).then_some(bound)
    }

    /// Allocate state variables.
    pub fn allocate_states(&mut self, count: usize) {
        self.state_values.resize(count, 0.0);
        self.state_values_prev.resize(count, 0.0);
        self.state_values_older.resize(count, 0.0);
        self.state_derivatives.resize(count, 0.0);
        self.state_derivatives_prev.resize(count, 0.0);
        self.state_initialized.resize(count, false);
    }

    /// Allocate delay buffers used by `absdelay(...)`.
    pub fn allocate_delay_buffers(&mut self, count: usize) {
        self.delay_buffers.resize_with(count, DelayBuffer::default);
    }

    /// Allocate transition filters used by `transition(...)`.
    pub fn allocate_transition_filters(&mut self, count: usize) {
        self.transition_filters
            .resize_with(count, TransitionFilter::default);
    }

    /// Allocate slew filters used by `slew(...)`.
    pub fn allocate_slew_filters(&mut self, count: usize) {
        self.slew_filters.resize_with(count, SlewFilter::default);
    }

    /// Allocate cross detectors used by `cross(...)`.
    pub fn allocate_cross_detectors(&mut self, count: usize) {
        self.cross_detectors
            .resize_with(count, CrossDetector::default);
    }

    /// Clear cached branch current values.
    pub fn clear_currents(&mut self) {
        self.currents.clear();
        self.terminal_pair_currents.fill(f64::NAN);
    }

    /// Prepare fixed-index contribution storage for a fused evaluation pass.
    ///
    /// Unlike [`Self::clear_currents`], this preserves the allocation and
    /// establishes the final logical length up front so native code can write
    /// values by contribution index. Terminal-pair probes are invalidated for
    /// the new pass exactly as they are on the scalar path.
    #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
    pub(crate) fn prepare_indexed_currents(&mut self, count: usize) {
        if self.currents.len() != count {
            self.currents.resize(count, 0.0);
        } else {
            self.currents.fill(0.0);
        }
        self.terminal_pair_currents.fill(f64::NAN);
    }

    /// Set branch current from `pos` to `neg`.
    ///
    /// Also populates the reverse direction with opposite sign.
    pub fn set_branch_current(&mut self, pos: usize, neg: usize, value: f64) {
        let n = self.voltages.len();
        let Some(idx) = terminal_pair_current_index(pos, neg, n) else {
            return;
        };
        let len = terminal_pair_current_storage_len(n);
        if self.terminal_pair_currents.len() != len {
            self.terminal_pair_currents.resize(len, f64::NAN);
        }

        self.terminal_pair_currents[idx] = value;

        if pos != neg
            && let Some(reverse_idx) = terminal_pair_current_index(neg, pos, n)
        {
            self.terminal_pair_currents[reverse_idx] = -value;
        }
    }

    /// Get the voltage difference between two nodes in the unified node
    /// space (terminals first, then internal nodes; usize::MAX is the
    /// global reference node).
    #[inline]
    pub fn voltage(&self, pos: usize, neg: usize) -> f64 {
        self.try_voltage(pos, neg).unwrap_or(0.0)
    }

    /// Checked voltage difference between two unified node indexes.
    #[inline]
    pub fn try_voltage(&self, pos: usize, neg: usize) -> Result<f64, VmError> {
        Ok(self.try_node_potential(pos)? - self.try_node_potential(neg)?)
    }

    /// Potential of a unified node index against the global reference.
    #[inline]
    pub fn node_potential(&self, node: usize) -> f64 {
        self.try_node_potential(node).unwrap_or(0.0)
    }

    /// Checked potential of a unified node index against the global reference.
    #[inline]
    pub fn try_node_potential(&self, node: usize) -> Result<f64, VmError> {
        if node == usize::MAX {
            return Ok(0.0);
        }
        let num_terminals = self.voltages.len();
        if node < num_terminals {
            Ok(self.voltages[node])
        } else {
            self.internal_voltages
                .get(node - num_terminals)
                .copied()
                .ok_or(VmError::InvalidInstruction("missing node voltage slot"))
        }
    }

    /// Get internal node voltage.
    #[inline]
    pub fn internal_voltage(&self, idx: usize) -> f64 {
        self.internal_voltages.get(idx).copied().unwrap_or(0.0)
    }

    /// Get current between terminals.
    #[inline]
    pub fn current(&self, pos: usize, neg: usize) -> f64 {
        self.try_current(pos, neg).unwrap_or(0.0)
    }

    /// Get a previously stamped terminal-pair current.
    ///
    /// `I(pos, neg)` can only read an exact terminal-pair current that was
    /// already produced by a contribution in the same evaluation context.
    /// Falling back to another branch current aliases unrelated equations and
    /// corrupts the model numerics, so missing probes are reported explicitly.
    #[inline]
    pub fn try_current(&self, pos: usize, neg: usize) -> Result<f64, VmError> {
        let n = self.voltages.len();
        if let Some(idx) = terminal_pair_current_index(pos, neg, n) {
            if idx < self.terminal_pair_currents.len() {
                let value = self.terminal_pair_currents[idx];
                if value.is_finite() {
                    return Ok(value);
                }
            }
        }
        Err(VmError::InvalidInstruction(
            "missing terminal-pair current slot",
        ))
    }

    /// Get pointer to the terminal-pair current lookup buffer.
    #[inline]
    pub fn terminal_pair_currents_ptr(&self) -> *const f64 {
        self.terminal_pair_currents.as_ptr()
    }

    /// Get length of the terminal-pair current lookup buffer.
    #[inline]
    pub fn terminal_pair_currents_len(&self) -> usize {
        self.terminal_pair_currents.len()
    }

    /// Get number of terminals in this context.
    #[inline]
    pub fn terminal_count(&self) -> usize {
        self.voltages.len()
    }

    /// Whether a terminal was explicitly connected on this instance.
    #[inline]
    pub fn port_connected(&self, terminal: usize) -> bool {
        self.port_connected.get(terminal).copied().unwrap_or(0) != 0
    }

    /// Get thermal voltage kT/q.
    #[inline]
    pub fn vt(&self) -> f64 {
        const K_BOLTZMANN: f64 = 1.380649e-23;
        const Q_ELECTRON: f64 = 1.602176634e-19;
        K_BOLTZMANN * self.temperature / Q_ELECTRON
    }

    /// Set a parameter value.
    pub fn set_param(&mut self, index: usize, value: f64) {
        if index >= self.parameters.len() {
            self.parameters.resize(index + 1, 0.0);
        }
        self.parameters[index] = value;
    }

    /// Mark a parameter as explicitly given by the instance.
    pub fn mark_param_given(&mut self, index: usize) {
        if index >= self.param_given.len() {
            self.param_given.resize(index + 1, 0);
        }
        self.param_given[index] = 1;
    }

    /// Whether a parameter was explicitly given by the instance.
    #[inline]
    pub fn is_param_given(&self, index: usize) -> bool {
        self.param_given.get(index).copied().unwrap_or(0) != 0
    }

    /// Set a variable value.
    pub fn set_variable(&mut self, index: usize, value: f64) {
        if index >= self.variables.len() {
            self.variables.resize(index + 1, 0.0);
        }
        self.variables[index] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::{VerilogAEvaluationMode, VmContext};
    use crate::zfilter::ZiFilter;

    #[test]
    fn limiter_mode_defaults_are_analysis_safe() {
        assert_eq!(
            VerilogAEvaluationMode::default_for_analysis(0),
            VerilogAEvaluationMode::NewtonLimited
        );
        assert_eq!(
            VerilogAEvaluationMode::default_for_analysis(2),
            VerilogAEvaluationMode::NewtonLimited
        );
        assert_eq!(
            VerilogAEvaluationMode::default_for_analysis(4),
            VerilogAEvaluationMode::NewtonLimited
        );
        assert_eq!(
            VerilogAEvaluationMode::default_for_analysis(1),
            VerilogAEvaluationMode::SmallSignal
        );
        assert_eq!(
            VerilogAEvaluationMode::default_for_analysis(3),
            VerilogAEvaluationMode::SmallSignal
        );
        assert!(VerilogAEvaluationMode::NewtonLimited.limiting_enabled());
        assert!(!VerilogAEvaluationMode::StaticProbe.limiting_enabled());
        assert!(!VerilogAEvaluationMode::SmallSignal.limiting_enabled());
    }

    #[test]
    fn zi_multi_filter_commit_is_atomic_without_cloning_histories() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context.time = 0.0;
        context.zi_filters = vec![
            ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap(),
            ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap(),
        ];
        context.zi_filters[0].eval(2.0, 0.0, true).unwrap();
        context.zi_filters[1].eval(3.0, 0.0, true).unwrap();
        context.zi_filters[1].begin_evaluation();

        assert!(context.advance_state().is_err());
        let first_error = context.zi_filters[0]
            .eval(9.0, 0.5, true)
            .expect_err("first filter must not commit before second filter validation fails");
        assert!(first_error.to_string().contains("sample edge"));

        context.zi_filters[0].eval(2.0, 0.0, true).unwrap();
        context.zi_filters[1].eval(3.0, 0.0, true).unwrap();
        context.advance_state().unwrap();
        assert_eq!(context.zi_filters[0].eval(9.0, 0.5, true).unwrap(), 2.0);
        assert_eq!(context.zi_filters[1].eval(9.0, 0.5, true).unwrap(), 3.0);
    }

    #[test]
    fn duplicate_zi_acceptance_leaves_every_vm_history_unchanged() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context.time = 0.0;
        context.zi_filters = vec![ZiFilter::new(vec![1.0, 0.25], vec![1.0, -0.5], 1.0).unwrap()];
        context.zi_filters[0].eval(2.0, 0.0, true).unwrap();
        context.advance_state().unwrap();

        context.state_values = vec![11.0];
        context.state_values_prev = vec![22.0];
        context.state_values_older = vec![33.0];
        context.state_derivatives = vec![44.0];
        context.state_derivatives_prev = vec![55.0];
        context.state_initialized = vec![true];

        context.allocate_delay_buffers(1);
        context.delay_buffers[0].eval(0.25, 4.0, 0.1);
        context.allocate_transition_filters(1);
        context.transition_filters[0].eval(3.0, 0.25, 0.0, 1.0, 1.0);
        context.allocate_slew_filters(1);
        context.slew_filters[0].eval(5.0, 0.25, 1.0, 1.0);
        context.allocate_cross_detectors(1);
        context.cross_detectors[0].eval(1.0, 0.25, 0);
        context
            .laplace_filters
            .push(crate::laplace::StateSpaceFilter::integrator(1.0).unwrap());
        context.laplace_filters[0].step(6.0, 0.25).unwrap();
        context.zi_filters[0].eval(9.0, 0.0, true).unwrap();

        let before = format!("{context:#?}");
        let error = context
            .advance_state()
            .expect_err("duplicate Zi acceptance must fail before any context commit");
        assert!(
            error.to_string().contains("already accepted"),
            "got: {error}"
        );
        assert_eq!(
            format!("{context:#?}"),
            before,
            "Zi, integration, delay, transition, slew, crossing, and Laplace histories must remain atomic"
        );
    }

    #[test]
    fn dormant_zi_slot_neither_schedules_nor_advances_placeholder_clock() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context.time = 0.0;
        context.zi_filters = vec![ZiFilter::new(vec![1.0], vec![1.0], 0.25).unwrap()];

        assert_eq!(context.zi_filter_step_bound().unwrap(), None);
        context.advance_state().unwrap();
        assert_eq!(context.zi_filter_step_bound().unwrap(), None);

        assert_eq!(
            context.zi_filters[0].eval(2.0, 0.0, true).unwrap(),
            2.0,
            "the dormant commit must leave the t=0 sample pending"
        );
        assert_eq!(context.zi_filter_step_bound().unwrap(), Some(0.25));
        context.advance_state().unwrap();
        assert_eq!(context.zi_filter_step_bound().unwrap(), Some(0.25));
    }

    #[test]
    fn accepted_checkpoint_refuses_every_speculative_operator_candidate() {
        let mut context = VmContext::default();
        context.time = 0.0;
        context.analysis_type = 2;
        context.allocate_delay_buffers(1);
        context.delay_buffers[0].eval(0.0, 1.0, 0.25);
        let error = context
            .accepted_checkpoint()
            .expect_err("an in-flight delay candidate must block checkpoint capture");
        assert!(error.to_string().contains("in-flight Newton candidate"));

        context.delay_buffers[0].commit();
        context.zi_filters = vec![ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap()];
        context.zi_filters[0].eval(2.0, 0.0, true).unwrap();
        let error = context
            .accepted_checkpoint()
            .expect_err("an in-flight Zi candidate must block checkpoint capture");
        assert!(error.to_string().contains("in-flight Newton candidate"));
    }

    #[test]
    fn zi_iir_checkpoint_resume_is_bit_identical_between_and_on_sample_edges() {
        let mut original = VmContext::default();
        original.analysis_type = 2;
        original.zi_filters =
            vec![ZiFilter::new_with_timing(vec![0.5, 0.25], vec![1.0, -0.5], 1.0, 0.0).unwrap()];
        original.time = 0.0;
        original.zi_filters[0]
            .eval_with_transition(2.0, 0.0, true, 0.5)
            .unwrap();
        original.advance_state().unwrap();

        let between_checkpoint = original.accepted_checkpoint().unwrap();
        let mut resumed = VmContext::default();
        resumed.analysis_type = 2;
        resumed.zi_filters =
            vec![ZiFilter::new_with_timing(vec![0.5, 0.25], vec![1.0, -0.5], 1.0, 0.0).unwrap()];
        resumed
            .validate_accepted_checkpoint(&between_checkpoint)
            .unwrap();
        resumed.restore_accepted_checkpoint(&between_checkpoint);

        original.time = 0.25;
        resumed.time = 0.25;
        let original_between = original.zi_filters[0]
            .eval_with_transition(9.0, 0.25, true, 0.5)
            .unwrap();
        let resumed_between = resumed.zi_filters[0]
            .eval_with_transition(9.0, 0.25, true, 0.5)
            .unwrap();
        assert_eq!(original_between.to_bits(), resumed_between.to_bits());
        original.advance_state().unwrap();
        resumed.advance_state().unwrap();

        original.time = 1.0;
        resumed.time = 1.0;
        let original_edge = original.zi_filters[0]
            .eval_with_transition(4.0, 1.0, true, 0.5)
            .unwrap();
        let resumed_edge = resumed.zi_filters[0]
            .eval_with_transition(4.0, 1.0, true, 0.5)
            .unwrap();
        assert_eq!(original_edge.to_bits(), resumed_edge.to_bits());
        original.advance_state().unwrap();
        resumed.advance_state().unwrap();
        assert_eq!(
            original.accepted_checkpoint().unwrap(),
            resumed.accepted_checkpoint().unwrap()
        );
    }

    #[test]
    fn transition_ramp_and_sub_attosecond_bound_survive_accepted_snapshot() {
        let mut original = VmContext::default();
        original.analysis_type = 2;
        original.time = 0.0;
        original.allocate_transition_filters(1);
        original.transition_filters[0].eval(1.0, 0.0, 0.0, 1.0, 1.0);
        original.advance_state().unwrap();
        original.zi_filters = vec![ZiFilter::new(vec![1.0], vec![1.0], 5.0e-19).unwrap()];
        original.zi_filters[0].eval(1.0, 0.0, true).unwrap();
        original.advance_state().unwrap();
        assert_eq!(original.zi_filter_step_bound().unwrap(), Some(5.0e-19));

        let checkpoint = original.accepted_checkpoint().unwrap();
        let mut resumed = VmContext::default();
        resumed.analysis_type = 2;
        resumed.allocate_transition_filters(1);
        resumed.zi_filters = vec![ZiFilter::new(vec![1.0], vec![1.0], 5.0e-19).unwrap()];
        resumed.validate_accepted_checkpoint(&checkpoint).unwrap();
        resumed.restore_accepted_checkpoint(&checkpoint);
        assert_eq!(resumed.zi_filter_step_bound().unwrap(), Some(5.0e-19));

        original.time = 0.25;
        resumed.time = 0.25;
        let expected = original.transition_filters[0].eval(1.0, 0.25, 0.0, 1.0, 1.0);
        let actual = resumed.transition_filters[0].eval(1.0, 0.25, 0.0, 1.0, 1.0);
        assert_eq!(expected.to_bits(), actual.to_bits());
    }
}
