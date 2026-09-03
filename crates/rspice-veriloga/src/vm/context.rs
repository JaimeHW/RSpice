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

pub(crate) const INTEGRATION_CANDIDATE_NONE: u8 = 0;
pub(crate) const INTEGRATION_CANDIDATE_VALID: u8 = 1;
pub(crate) const INTEGRATION_CANDIDATE_IDLE: u8 = 2;

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

    /// Validate a solver-provided companion rule before it reaches an
    /// integration operator.
    ///
    /// The value-history coefficients must reproduce a constant exactly up
    /// to ordinary floating-point roundoff. Besides being required by `ddt`
    /// and `idt`, this affine invariant is what makes translating every
    /// `idtmod` history lane onto a common wrap branch mathematically sound.
    pub fn validate(self) -> Result<(), VmError> {
        let scales = [
            self.derivative_scale,
            self.previous_value_scale,
            self.older_value_scale,
            self.previous_derivative_scale,
        ];
        if scales.iter().any(|value| !value.is_finite()) {
            return Err(VmError::InvalidRuntimeConfiguration(
                "integration coefficients must all be finite".to_string(),
            ));
        }

        if !self.active {
            if scales.iter().any(|value| *value != 0.0) {
                return Err(VmError::InvalidRuntimeConfiguration(
                    "inactive integration coefficients must have zero scales".to_string(),
                ));
            }
            return Ok(());
        }

        if self.derivative_scale <= 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "active integration requires a positive derivative scale, got {}",
                self.derivative_scale
            )));
        }

        // Compare the history sum to the derivative scale, not to the
        // potentially much larger individual history terms. Scaling by those
        // terms would accept catastrophic cancellation such as MAX + -MAX.
        let history_sum = self.previous_value_scale + self.older_value_scale;
        let normalized_error = history_sum / self.derivative_scale - 1.0;
        if !history_sum.is_finite()
            || !normalized_error.is_finite()
            || normalized_error.abs() > 64.0 * f64::EPSILON
        {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "integration value-history scales must sum to the derivative scale: previous {} + older {} != derivative {}",
                self.previous_value_scale, self.older_value_scale, self.derivative_scale
            )));
        }

        Ok(())
    }
}

impl Default for IntegrationCoefficients {
    fn default() -> Self {
        Self::inactive()
    }
}

/// Fold one finite circular-integrator candidate into its representable
/// interval and return the common translation that must be applied to its
/// accepted history. The translation is deliberately separate from the
/// visible result: multistep formulas are invariant only when every history
/// lane is moved by the same amount.
pub(crate) fn idtmod_wrapped_candidate(
    raw: f64,
    modulus: f64,
    offset: f64,
) -> Result<(f64, f64), &'static str> {
    if !raw.is_finite() {
        return Err("integral candidate must be finite");
    }
    if !modulus.is_finite() || modulus <= 0.0 {
        return Err("modulus must be finite and greater than zero");
    }
    if !offset.is_finite() {
        return Err("offset must be finite");
    }
    let upper = offset + modulus;
    if !upper.is_finite() || upper <= offset {
        return Err("offset and modulus must form a finite, nonempty interval");
    }

    // Avoid turning two valid finite operands into infinity when their direct
    // subtraction overflows. Reducing each operand first is algebraically
    // equivalent modulo `modulus` and keeps the fallback finite.
    let delta = raw - offset;
    let phase = if delta.is_finite() {
        delta.rem_euclid(modulus)
    } else {
        (raw.rem_euclid(modulus) - offset.rem_euclid(modulus)).rem_euclid(modulus)
    };
    let mut wrapped = offset + phase;
    if wrapped >= upper {
        // Addition can round a phase infinitesimally below the modulus to the
        // exclusive upper endpoint. That point is the lower endpoint.
        wrapped = offset;
    }
    let rebase = raw - wrapped;
    if !wrapped.is_finite() || !rebase.is_finite() {
        return Err("wrapped value or history translation is not finite");
    }
    Ok((wrapped, rebase))
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
    /// Sorted variable slots whose values persist only after an accepted
    /// point because they are written by analog event-control bodies.
    ///
    /// These slots use [`Self::accepted_event_variables`] as their committed
    /// lane. Ordinary procedural variables remain evaluation-local and are
    /// deliberately not copied on every Newton pass.
    event_state_indices: Vec<usize>,
    /// Accepted values corresponding one-for-one with
    /// [`Self::event_state_indices`]. Runtime-only: checkpoints retain the
    /// canonical full variable vector after overlaying this committed lane.
    accepted_event_variables: Vec<f64>,
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
    /// Per-slot accepted initialization state. Integration operators consult
    /// this only for accepted history; limiters also use their dedicated slots
    /// as immediate Newton-iteration history.
    pub state_initialized: Vec<bool>,
    /// Per-slot integration candidate status. Zero denotes a non-integration
    /// or not-yet-observed slot, one a candidate from the latest evaluation,
    /// and two a known integration slot with no current candidate. This state
    /// is runtime-only and is never serialized.
    pub(crate) state_candidate_valid: Vec<u8>,
    /// Exact older-history lane proposed by the current integration-state
    /// evaluation. This is transactional Newton state: `ddt` and `idt` use the
    /// logical previous value (including startup seeding), while `idtmod` uses
    /// that value translated onto the wrapped candidate's common branch.
    /// Runtime-only; accepted history lanes are serialized by checkpoints.
    pub(crate) state_older_candidate: Vec<f64>,
    /// Per-evaluation slots the CFG route's prelude publishes into.
    ///
    /// One `f64` per distinct value entry output of a plan built through
    /// `CfgPrelude`, live for exactly one evaluation: the prelude writes every
    /// one of them and each value entry then returns the one it was assigned.
    /// Empty for every postfix plan, which is what production compiles, and
    /// sized by the device from the compiled model's
    /// `NativeRequiredStorage::prelude_slots` rather than from anything the
    /// interpreter knows. Runtime-only, and never serialized: a slot has no
    /// meaning outside the evaluation that wrote it.
    ///
    /// Read only by a JIT backend's evaluation context. The interpreter has no
    /// prelude — it executes each entry's own program — so a build with neither
    /// backend carries the field and never looks at it.
    #[cfg_attr(
        not(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32"))),
        allow(dead_code)
    )]
    pub(crate) prelude_slots: Vec<f64>,
    /// Current timestep (delta t) for transient analysis
    timestep: f64,
    /// Companion coefficients selected by the transient solver.
    integration: IntegrationCoefficients,
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
            event_state_indices: Vec::new(),
            accepted_event_variables: Vec::new(),
            time: 0.0,
            temperature: 300.15, // 27C default
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            state_values_older: Vec::new(),
            state_derivatives: Vec::new(),
            state_derivatives_prev: Vec::new(),
            state_initialized: Vec::new(),
            state_candidate_valid: Vec::new(),
            state_older_candidate: Vec::new(),
            prelude_slots: Vec::new(),
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
    pub(crate) fn accepted_event_variables(&self) -> &[f64] {
        &self.accepted_event_variables
    }

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
            event_state_indices: Vec::new(),
            accepted_event_variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            state_values_older: Vec::new(),
            state_derivatives: Vec::new(),
            state_derivatives_prev: Vec::new(),
            state_initialized: Vec::new(),
            state_candidate_valid: Vec::new(),
            state_older_candidate: Vec::new(),
            prelude_slots: Vec::new(),
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
            event_state_indices: Vec::new(),
            accepted_event_variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            state_values_older: Vec::new(),
            state_derivatives: Vec::new(),
            state_derivatives_prev: Vec::new(),
            state_initialized: Vec::new(),
            state_candidate_valid: Vec::new(),
            state_older_candidate: Vec::new(),
            prelude_slots: Vec::new(),
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
            event_state_indices: Vec::new(),
            accepted_event_variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: vec![0.0; num_states],
            state_values_prev: vec![0.0; num_states],
            state_values_older: vec![0.0; num_states],
            state_derivatives: vec![0.0; num_states],
            state_derivatives_prev: vec![0.0; num_states],
            state_initialized: vec![false; num_states],
            state_candidate_valid: vec![0; num_states],
            state_older_candidate: vec![0.0; num_states],
            prelude_slots: Vec::new(),
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

    /// Configure the procedural-variable slots whose values are committed
    /// transactionally with analog operator state.
    pub(crate) fn configure_event_state_variables(
        &mut self,
        indices: &[usize],
    ) -> Result<(), VmError> {
        let mut previous = None;
        for (position, &index) in indices.iter().enumerate() {
            if index >= self.variables.len() {
                return Err(VmError::InvalidRuntimeConfiguration(format!(
                    "event-state variable index {index} at position {position} exceeds variable storage length {}",
                    self.variables.len()
                )));
            }
            if previous.is_some_and(|prior| prior >= index) {
                return Err(VmError::InvalidRuntimeConfiguration(
                    "event-state variable indices must be sorted and unique".into(),
                ));
            }
            previous = Some(index);
        }

        self.event_state_indices.clear();
        self.event_state_indices.extend_from_slice(indices);
        self.accepted_event_variables.clear();
        self.accepted_event_variables
            .extend(indices.iter().map(|&index| self.variables[index]));
        Ok(())
    }

    fn validate_event_state_layout(&self) -> Result<(), VmError> {
        let invalid = |message: String| VmError::InvalidNumericResult(message);
        if self.event_state_indices.len() != self.accepted_event_variables.len() {
            return Err(invalid(
                "accepted event-variable storage shape is inconsistent".into(),
            ));
        }
        let mut previous = None;
        for (position, &index) in self.event_state_indices.iter().enumerate() {
            if index >= self.variables.len() {
                return Err(invalid(format!(
                    "event-state variable index {index} at position {position} exceeds variable storage length {}",
                    self.variables.len()
                )));
            }
            if previous.is_some_and(|prior| prior >= index) {
                return Err(invalid(
                    "event-state variable indices are not sorted and unique".into(),
                ));
            }
            previous = Some(index);
        }
        Ok(())
    }

    fn validate_event_state_candidate(&self) -> Result<(), VmError> {
        self.validate_event_state_layout()?;
        for (&index, &accepted) in self
            .event_state_indices
            .iter()
            .zip(&self.accepted_event_variables)
        {
            if accepted.is_nan() || self.variables[index].is_nan() {
                return Err(VmError::InvalidNumericResult(format!(
                    "event-state variable {index} contains an invalid numeric value"
                )));
            }
        }
        Ok(())
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
        let invalid = |message: String| VmError::InvalidNumericResult(message);
        let time = self.time;
        if !time.is_finite() || time < 0.0 {
            return Err(invalid(format!(
                "cannot commit invalid simulation time {time}"
            )));
        }
        self.validate_event_state_candidate()?;
        let state_count = self.state_values.len();
        if self.state_values_prev.len() != state_count
            || self.state_values_older.len() != state_count
            || self.state_derivatives.len() != state_count
            || self.state_derivatives_prev.len() != state_count
            || self.state_initialized.len() != state_count
            || self.state_candidate_valid.len() != state_count
            || self.state_older_candidate.len() != state_count
        {
            return Err(invalid(
                "candidate integration-state storage shape is inconsistent".into(),
            ));
        }
        if self
            .state_candidate_valid
            .iter()
            .any(|status| *status > INTEGRATION_CANDIDATE_IDLE)
        {
            return Err(invalid(
                "candidate integration-state validity storage is malformed".into(),
            ));
        }
        if self
            .state_values_prev
            .iter()
            .chain(&self.state_values_older)
            .chain(&self.state_derivatives_prev)
            .any(|value| !value.is_finite())
        {
            return Err(invalid(
                "accepted integration history contains a non-finite value".into(),
            ));
        }
        for index in 0..state_count {
            if self.state_candidate_valid[index] == INTEGRATION_CANDIDATE_VALID
                && (!self.state_values[index].is_finite()
                    || !self.state_derivatives[index].is_finite()
                    || !self.state_older_candidate[index].is_finite())
            {
                return Err(invalid(format!(
                    "candidate integration state {index} contains a non-finite value"
                )));
            }
        }
        if self.timer_event_bound != f64::INFINITY
            && (!self.timer_event_bound.is_finite() || self.timer_event_bound <= time)
        {
            return Err(invalid(format!(
                "timer candidate bound {} is not strictly after accepted time {time}",
                self.timer_event_bound
            )));
        }
        // Preserve the existing Zi error precedence for callers while still
        // completing every validation before any accepted state is mutated.
        for (filter_id, filter) in self.zi_filters.iter().enumerate() {
            filter.validate_commit(time).map_err(|error| {
                VmError::InvalidNumericResult(format!(
                    "zi filter {filter_id} commit failed: {error}"
                ))
            })?;
        }
        for (index, buffer) in self.delay_buffers.iter().enumerate() {
            buffer
                .validate_commit(time)
                .map_err(|error| invalid(format!("delay {index} commit failed: {error}")))?;
        }
        for (index, filter) in self.transition_filters.iter().enumerate() {
            filter
                .validate_commit(time)
                .map_err(|error| invalid(format!("transition {index} commit failed: {error}")))?;
        }
        for (index, filter) in self.slew_filters.iter().enumerate() {
            filter
                .validate_commit(time)
                .map_err(|error| invalid(format!("slew {index} commit failed: {error}")))?;
        }
        for (index, detector) in self.cross_detectors.iter().enumerate() {
            detector
                .validate_commit(time)
                .map_err(|error| invalid(format!("cross {index} commit failed: {error}")))?;
        }
        for (index, filter) in self.laplace_filters.iter().enumerate() {
            filter.validate_commit().map_err(|error| {
                invalid(format!("Laplace filter {index} commit failed: {error}"))
            })?;
        }
        Ok(())
    }

    /// Apply an accepted-state action after the circuit has validated every
    /// runtime-compiled instance. This phase is deliberately infallible.
    pub(crate) fn apply_validated_advance_state(&mut self) {
        let time = self.time;
        for (&index, accepted) in self
            .event_state_indices
            .iter()
            .zip(&mut self.accepted_event_variables)
        {
            *accepted = self.variables[index];
        }
        for index in 0..self.state_candidate_valid.len() {
            match self.state_candidate_valid[index] {
                INTEGRATION_CANDIDATE_NONE => continue,
                INTEGRATION_CANDIDATE_IDLE => {
                    self.state_values[index] = self.state_values_prev[index];
                    self.state_derivatives[index] = self.state_derivatives_prev[index];
                    self.state_older_candidate[index] = 0.0;
                    continue;
                }
                INTEGRATION_CANDIDATE_VALID => {}
                _ => unreachable!("validated integration candidate status"),
            }
            self.state_values_older[index] = self.state_older_candidate[index];
            self.state_values_prev[index] = self.state_values[index];
            self.state_derivatives_prev[index] = self.state_derivatives[index];
            self.state_initialized[index] = true;
            self.state_candidate_valid[index] = INTEGRATION_CANDIDATE_IDLE;
            self.state_older_candidate[index] = 0.0;
        }
        for buffer in &mut self.delay_buffers {
            buffer.apply_validated_commit();
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
        self.validate_event_state_layout()?;
        if self.state_candidate_valid.len() != self.state_values.len()
            || self.state_older_candidate.len() != self.state_values.len()
        {
            return Err(invalid(
                "integration candidate-valid storage shape is inconsistent".into(),
            ));
        }
        if self
            .state_candidate_valid
            .iter()
            .any(|status| *status > INTEGRATION_CANDIDATE_IDLE)
        {
            return Err(invalid(
                "integration candidate-valid storage is malformed".into(),
            ));
        }
        if self
            .state_candidate_valid
            .contains(&INTEGRATION_CANDIDATE_VALID)
        {
            return Err(invalid(
                "integration state has an in-flight Newton candidate".into(),
            ));
        }
        if self.state_older_candidate.iter().any(|value| *value != 0.0) {
            return Err(invalid(
                "integration state has an unapplied older-history candidate".into(),
            ));
        }
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
        let mut variables = self.variables.clone();
        for (&index, &accepted) in self
            .event_state_indices
            .iter()
            .zip(&self.accepted_event_variables)
        {
            variables[index] = accepted;
        }
        let checkpoint = VmAcceptedCheckpoint {
            time: self.time,
            variables,
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
        self.validate_event_state_layout()?;
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
            if state.initialized && state.time != checkpoint.time {
                return Err(invalid(format!(
                    "transition {index} accepted time does not equal checkpoint time"
                )));
            }
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
        for (&index, accepted) in self
            .event_state_indices
            .iter()
            .zip(&mut self.accepted_event_variables)
        {
            *accepted = checkpoint.variables[index];
        }
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
        self.state_candidate_valid.fill(0);
        self.state_older_candidate.fill(0.0);
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

    /// Reset every device-owned trajectory and in-flight candidate for a new
    /// analysis while retaining the instance configuration and preallocated
    /// storage shapes.
    ///
    /// Parameters, connectivity, temperature, multiplicity, lookup tables,
    /// compiled filter realizations, and the caller-selected analysis type and
    /// evaluation mode deliberately survive. Terminal/internal voltages and
    /// branch-current unknown values are solver-owned inputs retained as warm
    /// starts and overwritten by the solver before evaluation. Procedural
    /// variables and every analog-operator history start from their
    /// language-defined zero state.
    pub(crate) fn reset_analysis_state(&mut self) {
        self.variables.fill(0.0);
        self.accepted_event_variables.fill(0.0);
        self.time = 0.0;
        self.state_values.fill(0.0);
        self.state_values_prev.fill(0.0);
        self.state_values_older.fill(0.0);
        self.state_derivatives.fill(0.0);
        self.state_derivatives_prev.fill(0.0);
        self.state_initialized.fill(false);
        self.state_candidate_valid.fill(0);
        self.state_older_candidate.fill(0.0);
        self.timestep = 0.0;
        self.integration = IntegrationCoefficients::inactive();

        for buffer in &mut self.delay_buffers {
            buffer.clear();
        }
        for filter in &mut self.transition_filters {
            filter.reset_analysis();
        }
        for filter in &mut self.slew_filters {
            filter.reset_analysis();
        }
        for detector in &mut self.cross_detectors {
            detector.reset_analysis();
        }
        for filter in &mut self.laplace_filters {
            filter.reset();
        }
        for filter in &mut self.zi_filters {
            // A Zi definition may be derived from instance variables and must
            // be frozen again only after the new analysis executes the
            // operator's ordered argument programs.
            filter.invalidate_definition();
        }

        self.currents.clear();
        self.terminal_pair_currents.fill(f64::NAN);
        self.limiter_active = 0;
        self.analysis_initial_step = false;
        self.analysis_final_step = false;
        self.timer_event_bound = f64::INFINITY;
    }

    /// Invalidate every speculative operator candidate before each complete
    /// device evaluation. Only candidates recreated by the final Newton pass
    /// may be committed when the point is accepted.
    pub(crate) fn begin_stateful_evaluation(&mut self) {
        for (&index, &accepted) in self
            .event_state_indices
            .iter()
            .zip(&self.accepted_event_variables)
        {
            if let Some(variable) = self.variables.get_mut(index) {
                *variable = accepted;
            }
        }
        for (status, older_candidate) in self
            .state_candidate_valid
            .iter_mut()
            .zip(&mut self.state_older_candidate)
        {
            if *status == INTEGRATION_CANDIDATE_VALID {
                *status = INTEGRATION_CANDIDATE_IDLE;
            }
            *older_candidate = 0.0;
        }
        for buffer in &mut self.delay_buffers {
            buffer.begin_evaluation();
        }
        for filter in &mut self.transition_filters {
            filter.begin_evaluation();
        }
        for filter in &mut self.slew_filters {
            filter.begin_evaluation();
        }
        for detector in &mut self.cross_detectors {
            detector.begin_evaluation();
        }
        for filter in &mut self.laplace_filters {
            filter.begin_evaluation();
        }
        for filter in &mut self.zi_filters {
            filter.begin_evaluation();
        }
        self.clear_timer_event_bound();
    }

    /// Tightest exact sampled-filter edge after the current time.
    #[cfg(test)]
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

    /// Earliest exact timer, transition corner, sampled-filter, or slew
    /// catch-up event owned by the latest accepted runtime state.
    pub(crate) fn transient_event_time(&self) -> Result<Option<f64>, VmError> {
        let transition = self
            .transition_filters
            .iter()
            .filter_map(|filter| filter.next_event_time(self.time))
            .reduce(f64::min);
        let slew = self
            .slew_filters
            .iter()
            .filter_map(|filter| filter.next_corner_time(self.time))
            .reduce(f64::min);
        let zi = self
            .zi_filters
            .iter()
            .enumerate()
            .filter(|(_, filter)| filter.participates_in_transient_schedule())
            .map(|(filter_id, filter)| {
                filter.next_event_time(self.time).map_err(|error| {
                    VmError::InvalidNumericResult(format!(
                        "zi filter {filter_id} event target failed: {error}"
                    ))
                })
            })
            .try_fold(None, |minimum, target| {
                let target = target?;
                Ok(Some(
                    minimum.map_or(target, |current: f64| current.min(target)),
                ))
            })?;
        let timer = (self.timer_event_bound.is_finite() && self.timer_event_bound > self.time)
            .then_some(self.timer_event_bound);
        Ok([timer, transition, zi, slew]
            .into_iter()
            .flatten()
            .reduce(f64::min))
    }

    /// Earliest interior zero-crossing target produced by the latest complete
    /// transient evaluation. The request is speculative and is cleared before
    /// every replacement Newton pass.
    pub(crate) fn cross_event_refinement_time(&self) -> Result<Option<f64>, VmError> {
        self.cross_detectors
            .iter()
            .enumerate()
            .filter_map(|(detector_id, detector)| {
                detector
                    .candidate_refinement_time()
                    .map(|target| (detector_id, target))
            })
            .try_fold(None, |minimum, (detector_id, target)| {
                if !target.is_finite() || target < 0.0 || target >= self.time {
                    return Err(VmError::InvalidNumericResult(format!(
                        "cross detector {detector_id} produced invalid refinement target {target} for candidate time {}",
                        self.time
                    )));
                }
                Ok(Some(
                    minimum.map_or(target, |current: f64| current.min(target)),
                ))
            })
    }

    /// Current solver-provided companion coefficients.
    #[inline]
    pub fn integration_coefficients(&self) -> IntegrationCoefficients {
        self.integration
    }

    /// Current transient timestep.
    #[inline]
    pub fn timestep(&self) -> f64 {
        self.timestep
    }

    /// Set the timestep for transient analysis.
    pub fn set_timestep(&mut self, dt: f64) {
        self.try_set_timestep(dt)
            .unwrap_or_else(|error| panic!("VM context timestep update failed: {error}"));
    }

    /// Checked timestep update. Validation completes before any runtime state
    /// or lifecycle lane is mutated.
    pub fn try_set_timestep(&mut self, dt: f64) -> Result<(), VmError> {
        if !dt.is_finite() || dt < 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "transient timestep must be finite and non-negative, got {dt}"
            )));
        }
        let coefficients = IntegrationCoefficients::backward_euler(dt);
        coefficients.validate()?;
        self.timestep = dt;
        self.apply_integration_coefficients(coefficients);
        Ok(())
    }

    /// Select solver-provided companion coefficients for this timepoint.
    pub fn set_integration_coefficients(&mut self, coefficients: IntegrationCoefficients) {
        self.try_set_integration_coefficients(coefficients)
            .unwrap_or_else(|error| {
                panic!("VM context integration-coefficient update failed: {error}")
            });
    }

    /// Checked companion-coefficient update. Validation completes before any
    /// candidate state is promoted or invalidated.
    pub fn try_set_integration_coefficients(
        &mut self,
        coefficients: IntegrationCoefficients,
    ) -> Result<(), VmError> {
        coefficients.validate()?;
        self.apply_integration_coefficients(coefficients);
        Ok(())
    }

    fn apply_integration_coefficients(&mut self, coefficients: IntegrationCoefficients) {
        if !self.integration.active && coefficients.active {
            // The DC operating-point evaluation establishes each operator's
            // current state but is not an accepted transient step.  Promote
            // that state to both history lanes when transient integration
            // starts so a biased ddt() differentiates from the operating
            // point and idt() starts from its DC initial condition.
            for index in 0..self.state_candidate_valid.len() {
                match self.state_candidate_valid[index] {
                    INTEGRATION_CANDIDATE_NONE => continue,
                    INTEGRATION_CANDIDATE_IDLE => {
                        self.state_values[index] = self.state_values_prev[index];
                        self.state_derivatives[index] = self.state_derivatives_prev[index];
                    }
                    INTEGRATION_CANDIDATE_VALID => {
                        if self.state_values[index].is_finite()
                            && self.state_derivatives[index].is_finite()
                        {
                            self.state_values_prev[index] = self.state_values[index];
                            self.state_values_older[index] = self.state_values[index];
                            self.state_derivatives_prev[index] = self.state_derivatives[index];
                            self.state_initialized[index] = true;
                        } else {
                            self.state_values[index] = self.state_values_prev[index];
                            self.state_derivatives[index] = self.state_derivatives_prev[index];
                        }
                        self.state_candidate_valid[index] = INTEGRATION_CANDIDATE_IDLE;
                        self.state_older_candidate[index] = 0.0;
                    }
                    _ => {}
                }
            }
            for filter in &mut self.slew_filters {
                filter.promote_operating_point_candidate();
            }
            for filter in &mut self.transition_filters {
                filter.promote_operating_point_candidate();
            }
            for filter in &mut self.laplace_filters {
                filter.promote_operating_point_candidate();
            }
        } else if self.integration != coefficients {
            // A candidate evaluated with different companion coefficients
            // cannot be accepted under the new integration rule.
            for index in 0..self.state_candidate_valid.len() {
                if self.state_candidate_valid[index] == INTEGRATION_CANDIDATE_VALID {
                    self.state_candidate_valid[index] = INTEGRATION_CANDIDATE_IDLE;
                    self.state_values[index] = self.state_values_prev[index];
                    self.state_derivatives[index] = self.state_derivatives_prev[index];
                    self.state_older_candidate[index] = 0.0;
                }
            }
            for filter in &mut self.laplace_filters {
                filter.begin_evaluation();
            }
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
        self.state_candidate_valid.resize(count, 0);
        self.state_older_candidate.resize(count, 0.0);
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
        if let Ok(position) = self.event_state_indices.binary_search(&index) {
            self.accepted_event_variables[position] = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        INTEGRATION_CANDIDATE_VALID, IntegrationCoefficients, VerilogAEvaluationMode, VmContext,
        VmError, idtmod_wrapped_candidate,
    };
    use crate::laplace::StateSpaceFilter;
    use crate::timing_contract::SlewRateMagnitudes;
    use crate::zfilter::ZiFilter;

    fn slew_rates(rise: f64, fall: f64) -> SlewRateMagnitudes {
        SlewRateMagnitudes { rise, fall }
    }

    #[test]
    fn integration_coefficients_validate_supported_rules_and_tiny_scales() {
        let valid = [
            IntegrationCoefficients::inactive(),
            IntegrationCoefficients::backward_euler(0.25),
            IntegrationCoefficients {
                active: true,
                derivative_scale: 2.0,
                previous_value_scale: 2.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 1.0,
            },
            IntegrationCoefficients {
                active: true,
                derivative_scale: 1.5,
                previous_value_scale: 2.0,
                older_value_scale: -0.5,
                previous_derivative_scale: 0.0,
            },
            IntegrationCoefficients {
                active: true,
                derivative_scale: 2.5,
                previous_value_scale: 10.0 / 3.0,
                older_value_scale: -5.0 / 6.0,
                previous_derivative_scale: 0.0,
            },
            IntegrationCoefficients {
                active: true,
                derivative_scale: f64::MIN_POSITIVE,
                previous_value_scale: f64::MIN_POSITIVE,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            },
            IntegrationCoefficients {
                active: true,
                derivative_scale: f64::from_bits(1),
                previous_value_scale: f64::from_bits(1),
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            },
        ];

        for coefficients in valid {
            coefficients
                .validate()
                .unwrap_or_else(|error| panic!("valid coefficients {coefficients:?}: {error}"));
        }
    }

    #[test]
    fn integration_coefficients_reject_catastrophic_history_cancellation() {
        let coefficients = IntegrationCoefficients {
            active: true,
            derivative_scale: 1.0,
            previous_value_scale: f64::MAX,
            older_value_scale: -f64::MAX,
            previous_derivative_scale: 0.0,
        };

        assert!(matches!(
            coefficients.validate(),
            Err(VmError::InvalidRuntimeConfiguration(message))
                if message.contains("must sum to the derivative scale")
        ));

        let overflowing_sum = IntegrationCoefficients {
            active: true,
            derivative_scale: f64::MAX,
            previous_value_scale: f64::MAX,
            older_value_scale: f64::MAX,
            previous_derivative_scale: 0.0,
        };
        assert!(matches!(
            overflowing_sum.validate(),
            Err(VmError::InvalidRuntimeConfiguration(message))
                if message.contains("must sum to the derivative scale")
        ));
    }

    #[test]
    fn rejected_integration_updates_do_not_mutate_runtime_state() {
        let mut context = VmContext::with_states(0, 1);
        context.try_set_timestep(0.25).unwrap();
        context.state_values[0] = 7.0;
        context.state_values_prev[0] = 3.0;
        context.state_values_older[0] = 2.0;
        context.state_derivatives[0] = 11.0;
        context.state_derivatives_prev[0] = 5.0;
        context.state_initialized[0] = true;
        context.state_candidate_valid[0] = INTEGRATION_CANDIDATE_VALID;
        context.state_older_candidate[0] = 13.0;

        let before = context.clone();
        let nonaffine = IntegrationCoefficients {
            active: true,
            derivative_scale: 1.0,
            previous_value_scale: 2.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 0.0,
        };
        assert!(context.try_set_integration_coefficients(nonaffine).is_err());
        assert_eq!(context.integration, before.integration);
        assert_eq!(context.state_values, before.state_values);
        assert_eq!(context.state_values_prev, before.state_values_prev);
        assert_eq!(context.state_values_older, before.state_values_older);
        assert_eq!(context.state_derivatives, before.state_derivatives);
        assert_eq!(
            context.state_derivatives_prev,
            before.state_derivatives_prev
        );
        assert_eq!(context.state_initialized, before.state_initialized);
        assert_eq!(context.state_candidate_valid, before.state_candidate_valid);
        assert_eq!(context.state_older_candidate, before.state_older_candidate);

        for invalid_timestep in [f64::NAN, f64::INFINITY, -1.0] {
            assert!(context.try_set_timestep(invalid_timestep).is_err());
            assert_eq!(context.timestep().to_bits(), before.timestep().to_bits());
            assert_eq!(context.integration, before.integration);
            assert_eq!(context.state_candidate_valid, before.state_candidate_valid);
            assert_eq!(context.state_older_candidate, before.state_older_candidate);
        }
    }

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
    fn fresh_analysis_reset_clears_every_dynamic_pool_and_preserves_configuration() {
        let mut context = VmContext::with_states(2, 2);
        context.voltages = vec![1.0, -2.0];
        context.internal_voltages = vec![3.0];
        context.parameters = vec![4.0];
        context.param_given = vec![1];
        context.port_connected = vec![1, 0];
        context.temperature = 325.0;
        context.multiplicity = 7.0;
        context.branch_current_values = vec![8.0];
        context.variables = vec![9.0, -10.0];
        context.time = 1.0;
        context.try_set_timestep(0.25).unwrap();
        context.analysis_type = 1;
        context.evaluation_mode = VerilogAEvaluationMode::SmallSignal;
        context.limiter_active = 1;
        context.analysis_initial_step = true;
        context.analysis_final_step = true;
        context.state_values = vec![11.0, 12.0];
        context.state_values_prev = vec![13.0, 14.0];
        context.state_values_older = vec![15.0, 16.0];
        context.state_derivatives = vec![17.0, 18.0];
        context.state_derivatives_prev = vec![19.0, 20.0];
        context.state_initialized = vec![true, true];
        context.state_candidate_valid = vec![1, 2];
        context.state_older_candidate = vec![1.0, -2.0];
        context.currents = vec![21.0];
        context.set_branch_current(0, 1, 22.0);
        context.request_timer_event(2.0);

        context.allocate_delay_buffers(1);
        context.delay_buffers[0].eval(0.0, 1.0, 0.25, None).unwrap();
        context.delay_buffers[0].commit().unwrap();
        context.delay_buffers[0].eval(1.0, 2.0, 0.25, None).unwrap();
        let delay_capacity = context.delay_buffers[0].allocation_capacity();

        context.allocate_transition_filters(1);
        context.transition_filters[0]
            .eval(1.0, 0.0, 0.0, 1.0, 1.0)
            .unwrap();
        context.transition_filters[0].commit();
        context.transition_filters[0]
            .eval(2.0, 0.5, 0.0, 1.0, 1.0)
            .unwrap();

        context.allocate_slew_filters(1);
        context.slew_filters[0].eval(1.0, 1.0, slew_rates(0.5, 0.5));
        context.slew_filters[0].commit();
        context.slew_filters[0].eval(2.0, 1.5, slew_rates(0.5, 0.5));

        context.allocate_cross_detectors(1);
        context.cross_detectors[0].eval(-1.0, 0.0, 0).unwrap();
        context.cross_detectors[0].commit();
        context.cross_detectors[0].eval(1.0, 1.0, 0).unwrap();

        let mut laplace = StateSpaceFilter::integrator(1.0).unwrap();
        laplace.step(2.0, 0.25).unwrap();
        laplace.commit();
        laplace.step(3.0, 0.25).unwrap();
        let laplace_response = laplace.frequency_response(123.0).unwrap();
        context.laplace_filters.push(laplace);

        let mut zi = ZiFilter::new(vec![1.0, 0.5], vec![1.0, -0.25], 1.0).unwrap();
        zi.eval(2.0, 0.0, true).unwrap();
        zi.commit(0.0).unwrap();
        zi.eval(3.0, 0.5, true).unwrap();
        let zi_gain = zi.dc_gain().unwrap();
        context.zi_filters.push(zi);

        context.reset_analysis_state();

        assert_eq!(context.voltages, vec![1.0, -2.0]);
        assert_eq!(context.internal_voltages, vec![3.0]);
        assert_eq!(context.parameters, vec![4.0]);
        assert_eq!(context.param_given, vec![1]);
        assert_eq!(context.port_connected, vec![1, 0]);
        assert_eq!(context.temperature, 325.0);
        assert_eq!(context.multiplicity, 7.0);
        assert_eq!(context.branch_current_values, vec![8.0]);
        assert_eq!(context.analysis_type, 1);
        assert_eq!(context.evaluation_mode, VerilogAEvaluationMode::SmallSignal);

        assert_eq!(context.variables, vec![0.0, 0.0]);
        assert_eq!(context.time, 0.0);
        assert_eq!(context.state_values, vec![0.0, 0.0]);
        assert_eq!(context.state_values_prev, vec![0.0, 0.0]);
        assert_eq!(context.state_values_older, vec![0.0, 0.0]);
        assert_eq!(context.state_derivatives, vec![0.0, 0.0]);
        assert_eq!(context.state_derivatives_prev, vec![0.0, 0.0]);
        assert_eq!(context.state_initialized, vec![false, false]);
        assert_eq!(context.state_candidate_valid, vec![0, 0]);
        assert_eq!(context.state_older_candidate, vec![0.0, 0.0]);
        assert_eq!(context.timestep(), 0.0);
        assert_eq!(
            context.integration_coefficients(),
            IntegrationCoefficients::inactive()
        );
        assert!(context.currents.is_empty());
        assert!(context.try_current(0, 1).is_err());
        assert_eq!(context.limiter_active, 0);
        assert!(!context.analysis_initial_step);
        assert!(!context.analysis_final_step);
        assert_eq!(context.timer_event_step_bound(), None);
        assert_eq!(context.zi_filter_step_bound().unwrap(), None);

        assert_eq!(
            context.delay_buffers[0].allocation_capacity(),
            delay_capacity
        );
        assert_eq!(context.delay_buffers[0].accepted_sample_count(), 0);
        assert_eq!(
            context.laplace_filters[0]
                .frequency_response(123.0)
                .unwrap(),
            laplace_response,
            "reset must retain the compiled Laplace realization"
        );
        assert_eq!(context.zi_filters[0].dc_gain().unwrap(), zi_gain);
        assert!(
            !context.zi_filters[0].definition_is_frozen(),
            "analysis-specific Zi arguments must be frozen again"
        );

        let reset = context
            .accepted_checkpoint()
            .expect("reset must remove every in-flight operator candidate");
        assert!(reset.delay_buffers[0].samples.is_empty());
        assert_eq!(reset.transition_filters[0].output, 0.0);
        assert!(!reset.transition_filters[0].initialized);
        assert!(reset.transition_filters[0].pending.is_empty());
        assert_eq!(reset.slew_filters[0].output, 0.0);
        assert_eq!(reset.slew_filters[0].prev_time, 0.0);
        assert!(!reset.cross_detectors[0].initialized);
        assert_eq!(reset.cross_detectors[0].last_crossing_time, -1.0);
        assert!(
            reset.laplace_filters[0]
                .state
                .iter()
                .all(|value| *value == 0.0)
        );
        assert!(!reset.zi_filters[0].definition_frozen);
        assert!(
            reset.zi_filters[0]
                .x_hist
                .iter()
                .chain(&reset.zi_filters[0].y_hist)
                .all(|value| *value == 0.0)
        );
        assert_eq!(reset.zi_filters[0].accepted_time, None);
        assert_eq!(reset.timer_event_bound, None);
    }

    #[test]
    fn event_variables_are_transactional_without_resetting_ordinary_variables() {
        let mut context = VmContext {
            variables: vec![10.0, 20.0, 30.0],
            ..VmContext::default()
        };
        context
            .configure_event_state_variables(&[1])
            .expect("valid event-state layout configures");

        context.variables[0] = 11.0;
        context.variables[1] = 21.0;
        context.begin_stateful_evaluation();
        assert_eq!(context.variables, vec![11.0, 20.0, 30.0]);

        context.variables[0] = 12.0;
        context.variables[1] = 22.0;
        context.advance_state().expect("candidate commits");

        context.variables[0] = 13.0;
        context.variables[1] = f64::NAN;
        let checkpoint = context
            .accepted_checkpoint()
            .expect("even an invalid speculative event value is excluded from a checkpoint");
        assert_eq!(
            checkpoint.variables,
            vec![13.0, 22.0, 30.0],
            "only event-controlled slots must be overlaid from accepted state"
        );

        context.begin_stateful_evaluation();
        assert_eq!(context.variables, vec![13.0, 22.0, 30.0]);
    }

    #[test]
    fn event_variable_checkpoint_restore_rebuilds_the_committed_lane() {
        let mut source = VmContext {
            variables: vec![1.0, 2.0, 3.0],
            ..VmContext::default()
        };
        source.configure_event_state_variables(&[0, 2]).unwrap();
        source.variables = vec![4.0, 5.0, 6.0];
        source.advance_state().unwrap();
        let checkpoint = source.accepted_checkpoint().unwrap();

        let mut restored = VmContext {
            variables: vec![0.0; 3],
            ..VmContext::default()
        };
        restored.configure_event_state_variables(&[0, 2]).unwrap();
        restored.validate_accepted_checkpoint(&checkpoint).unwrap();
        restored.restore_accepted_checkpoint(&checkpoint);
        restored.variables[0] = 40.0;
        restored.variables[1] = 50.0;
        restored.variables[2] = 60.0;
        restored.begin_stateful_evaluation();

        assert_eq!(restored.variables, vec![4.0, 50.0, 6.0]);
        restored.reset_analysis_state();
        assert_eq!(restored.variables, vec![0.0; 3]);
        assert_eq!(restored.accepted_event_variables, vec![0.0; 2]);
    }

    #[test]
    fn malformed_event_variable_layout_is_rejected_without_mutation() {
        let mut context = VmContext {
            variables: vec![1.0, 2.0],
            ..VmContext::default()
        };
        context.configure_event_state_variables(&[0]).unwrap();

        for indices in [&[1, 1][..], &[1, 0][..], &[2][..]] {
            let before = format!("{context:#?}");
            let error = context
                .configure_event_state_variables(indices)
                .expect_err("malformed event-state metadata must fail");
            assert!(
                error.to_string().contains("event-state variable"),
                "got: {error}"
            );
            assert_eq!(format!("{context:#?}"), before);
        }
    }

    #[test]
    fn idtmod_wrap_returns_a_finite_common_branch_translation() {
        let (wrapped, rebase) = idtmod_wrapped_candidate(1.2, 1.0, 0.0).unwrap();
        assert!((wrapped - 0.2).abs() <= f64::EPSILON);
        assert_eq!(rebase.to_bits(), 1.0_f64.to_bits());

        let raw = f64::MAX;
        let offset = -f64::MAX / 2.0;
        let (wrapped, rebase) = idtmod_wrapped_candidate(raw, f64::MAX, offset)
            .expect("finite operands remain reducible when raw-offset overflows");
        assert_eq!(wrapped.to_bits(), 0.0_f64.to_bits());
        assert_eq!(rebase.to_bits(), raw.to_bits());
    }

    #[test]
    fn idtmod_wrap_rejects_invalid_or_unrepresentable_intervals() {
        for modulus in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(
                idtmod_wrapped_candidate(0.0, modulus, 0.0)
                    .unwrap_err()
                    .contains("modulus")
            );
        }
        assert!(
            idtmod_wrapped_candidate(0.0, 1.0, f64::NAN)
                .unwrap_err()
                .contains("offset")
        );
        assert!(
            idtmod_wrapped_candidate(f64::MAX, 1.0, f64::MAX)
                .unwrap_err()
                .contains("interval")
        );
    }

    #[test]
    fn zi_multi_filter_commit_is_atomic_without_cloning_histories() {
        let mut context = VmContext {
            analysis_type: 2,
            time: 0.0,
            zi_filters: vec![
                ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap(),
                ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap(),
            ],
            ..VmContext::default()
        };
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
    fn final_pass_skip_discards_every_non_zi_candidate() {
        let mut context = VmContext {
            analysis_type: 2,
            time: 1.0,
            ..VmContext::default()
        };
        context.allocate_delay_buffers(1);
        context.allocate_transition_filters(1);
        context.allocate_slew_filters(1);
        context.allocate_cross_detectors(1);
        context
            .laplace_filters
            .push(StateSpaceFilter::integrator(1.0).unwrap());

        context.delay_buffers[0].eval(0.0, 0.0, 0.25, None).unwrap();
        context.delay_buffers[0].commit().unwrap();

        context.begin_stateful_evaluation();
        context.delay_buffers[0].eval(1.0, 1.0, 0.25, None).unwrap();
        context.transition_filters[0]
            .eval(1.0, 1.0, 0.0, 0.0, 0.0)
            .unwrap();
        context.slew_filters[0].eval(1.0, 1.0, slew_rates(10.0, 10.0));
        context.cross_detectors[0].eval(-1.0, 1.0, 0).unwrap();
        context.laplace_filters[0].step(1.0, 0.25).unwrap();
        context.advance_state().unwrap();
        let accepted = context.accepted_checkpoint().unwrap();

        context.begin_stateful_evaluation();
        context.delay_buffers[0].eval(1.0, 9.0, 0.25, None).unwrap();
        context.transition_filters[0]
            .eval(9.0, 1.0, 0.0, 0.0, 0.0)
            .unwrap();
        context.slew_filters[0].eval(9.0, 1.0, slew_rates(10.0, 10.0));
        context.cross_detectors[0].eval(1.0, 1.0, 0).unwrap();
        context.laplace_filters[0].step(9.0, 0.25).unwrap();
        context.request_timer_event(2.0);

        // The final complete pass skips every operator. No candidate or event
        // constraint from the earlier pass may survive acceptance.
        context.begin_stateful_evaluation();
        context.advance_state().unwrap();
        assert_eq!(context.accepted_checkpoint().unwrap(), accepted);
        assert_eq!(context.timer_event_step_bound(), None);
    }

    #[test]
    fn failed_pool_validation_is_atomic_and_a_retry_starts_clean() {
        let mut context = VmContext {
            analysis_type: 2,
            time: 1.0,
            ..VmContext::default()
        };
        context.allocate_delay_buffers(1);
        context.allocate_transition_filters(1);
        context.allocate_slew_filters(1);
        context.allocate_cross_detectors(1);
        context
            .laplace_filters
            .push(StateSpaceFilter::integrator(1.0).unwrap());

        context.delay_buffers[0].eval(0.0, 0.0, 0.25, None).unwrap();
        context.delay_buffers[0].commit().unwrap();

        context.begin_stateful_evaluation();
        context.delay_buffers[0].eval(1.0, 2.0, 0.25, None).unwrap();
        context.transition_filters[0]
            .eval(3.0, 0.5, 0.0, 1.0, 1.0)
            .unwrap();
        context.slew_filters[0].eval(4.0, 1.0, slew_rates(10.0, 10.0));
        context.cross_detectors[0].eval(1.0, 1.0, 0).unwrap();
        context.laplace_filters[0].step(5.0, 0.25).unwrap();

        let before = format!("{context:#?}");
        let error = context
            .advance_state()
            .expect_err("a transition candidate for another time must fail acceptance");
        assert!(error.to_string().contains("transition 0 commit failed"));
        assert_eq!(format!("{context:#?}"), before);

        // A failed-evaluation retry starts a new complete pass. Skipped slots
        // cannot commit the partial candidates produced by the failed pass.
        context.begin_stateful_evaluation();
        context.advance_state().unwrap();
        let accepted = context.accepted_checkpoint().unwrap();
        assert_eq!(accepted.delay_buffers[0].samples, vec![(0.0, 0.0)]);
        assert_eq!(accepted.transition_filters[0].output, 0.0);
        assert_eq!(accepted.slew_filters[0].output, 0.0);
        assert!(!accepted.cross_detectors[0].initialized);
        assert!(
            accepted.laplace_filters[0]
                .state
                .iter()
                .all(|value| *value == 0.0)
        );
    }

    #[test]
    fn duplicate_zi_acceptance_leaves_every_vm_history_unchanged() {
        let mut context = VmContext {
            analysis_type: 2,
            time: 0.0,
            zi_filters: vec![ZiFilter::new(vec![1.0, 0.25], vec![1.0, -0.5], 1.0).unwrap()],
            ..VmContext::default()
        };
        context.zi_filters[0].eval(2.0, 0.0, true).unwrap();
        context.advance_state().unwrap();

        context.state_values = vec![11.0];
        context.state_values_prev = vec![22.0];
        context.state_values_older = vec![33.0];
        context.state_derivatives = vec![44.0];
        context.state_derivatives_prev = vec![55.0];
        context.state_initialized = vec![true];
        context.state_candidate_valid = vec![0];
        context.state_older_candidate = vec![0.0];

        context.allocate_delay_buffers(1);
        context.delay_buffers[0].eval(0.0, 0.0, 0.1, None).unwrap();
        context.delay_buffers[0].commit().unwrap();
        context.delay_buffers[0].eval(0.25, 4.0, 0.1, None).unwrap();
        context.allocate_transition_filters(1);
        context.transition_filters[0]
            .eval(3.0, 0.25, 0.0, 1.0, 1.0)
            .unwrap();
        context.allocate_slew_filters(1);
        context.slew_filters[0].eval(5.0, 0.25, slew_rates(1.0, 1.0));
        context.allocate_cross_detectors(1);
        context.cross_detectors[0].eval(1.0, 0.25, 0).unwrap();
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
        let mut context = VmContext {
            analysis_type: 2,
            time: 0.0,
            zi_filters: vec![ZiFilter::new(vec![1.0], vec![1.0], 0.25).unwrap()],
            ..VmContext::default()
        };

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
        let mut context = VmContext {
            time: 0.0,
            analysis_type: 2,
            ..VmContext::default()
        };
        context.allocate_delay_buffers(1);
        context.delay_buffers[0].eval(0.0, 1.0, 0.25, None).unwrap();
        let error = context
            .accepted_checkpoint()
            .expect_err("an in-flight delay candidate must block checkpoint capture");
        assert!(error.to_string().contains("in-flight Newton candidate"));

        context.delay_buffers[0].commit().unwrap();
        context.zi_filters = vec![ZiFilter::new(vec![1.0], vec![1.0], 1.0).unwrap()];
        context.zi_filters[0].eval(2.0, 0.0, true).unwrap();
        let error = context
            .accepted_checkpoint()
            .expect_err("an in-flight Zi candidate must block checkpoint capture");
        assert!(error.to_string().contains("in-flight Newton candidate"));
    }

    #[test]
    fn accepted_checkpoint_rejects_malformed_candidate_status_storage() {
        let mut context = VmContext::with_states(0, 1);
        context.state_candidate_valid.clear();
        let error = context
            .accepted_checkpoint()
            .expect_err("candidate-status shape mismatch must block checkpoint capture");
        assert!(error.to_string().contains("shape is inconsistent"));

        context.state_candidate_valid = vec![3];
        let error = context
            .accepted_checkpoint()
            .expect_err("invalid candidate status must block checkpoint capture");
        assert!(error.to_string().contains("malformed"));
    }

    #[test]
    fn zi_iir_checkpoint_resume_is_bit_identical_between_and_on_sample_edges() {
        let mut original = VmContext {
            analysis_type: 2,
            zi_filters: vec![
                ZiFilter::new_with_timing(vec![0.5, 0.25], vec![1.0, -0.5], 1.0, 0.0).unwrap(),
            ],
            time: 0.0,
            ..VmContext::default()
        };
        original.zi_filters[0]
            .eval_with_transition(2.0, 0.0, true, 0.5)
            .unwrap();
        original.advance_state().unwrap();

        let between_checkpoint = original.accepted_checkpoint().unwrap();
        let mut resumed = VmContext {
            analysis_type: 2,
            zi_filters: vec![
                ZiFilter::new_with_timing(vec![0.5, 0.25], vec![1.0, -0.5], 1.0, 0.0).unwrap(),
            ],
            ..VmContext::default()
        };
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
        let mut original = VmContext {
            analysis_type: 2,
            time: 0.0,
            ..VmContext::default()
        };
        original.allocate_transition_filters(1);
        original.transition_filters[0]
            .eval(1.0, 0.0, 0.0, 1.0, 1.0)
            .unwrap();
        original.advance_state().unwrap();
        original.zi_filters = vec![ZiFilter::new(vec![1.0], vec![1.0], 5.0e-19).unwrap()];
        original.zi_filters[0].eval(1.0, 0.0, true).unwrap();
        original.advance_state().unwrap();
        assert_eq!(original.zi_filter_step_bound().unwrap(), Some(5.0e-19));

        let checkpoint = original.accepted_checkpoint().unwrap();
        let mut resumed = VmContext {
            analysis_type: 2,
            ..VmContext::default()
        };
        resumed.allocate_transition_filters(1);
        resumed.zi_filters = vec![ZiFilter::new(vec![1.0], vec![1.0], 5.0e-19).unwrap()];
        resumed.validate_accepted_checkpoint(&checkpoint).unwrap();
        resumed.restore_accepted_checkpoint(&checkpoint);
        assert_eq!(resumed.zi_filter_step_bound().unwrap(), Some(5.0e-19));

        original.time = 0.25;
        resumed.time = 0.25;
        let expected = original.transition_filters[0]
            .eval(1.0, 0.25, 0.0, 1.0, 1.0)
            .unwrap();
        let actual = resumed.transition_filters[0]
            .eval(1.0, 0.25, 0.0, 1.0, 1.0)
            .unwrap();
        assert_eq!(expected.to_bits(), actual.to_bits());
    }

    #[test]
    fn cross_refinement_selects_the_earliest_candidate_and_clears_on_replacement() {
        let mut context = VmContext {
            analysis_type: 2,
            time: 0.0,
            ..VmContext::default()
        };
        context.allocate_cross_detectors(2);
        context.cross_detectors[0].eval(-1.0, 0.0, 1).unwrap();
        context.cross_detectors[1].eval(-3.0, 0.0, 1).unwrap();
        context.cross_detectors[0].commit();
        context.cross_detectors[1].commit();

        context.time = 1.0;
        context.cross_detectors[0].eval(1.0, 1.0, 1).unwrap();
        context.cross_detectors[1].eval(1.0, 1.0, 1).unwrap();
        assert_eq!(
            context.cross_event_refinement_time().unwrap(),
            Some(0.5000000000000001)
        );

        context.begin_stateful_evaluation();
        assert_eq!(context.cross_event_refinement_time().unwrap(), None);
    }
}
