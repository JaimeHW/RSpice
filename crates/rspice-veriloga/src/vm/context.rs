use super::error::VmError;
use super::filters::{CrossDetector, DelayBuffer, SlewFilter, TransitionFilter};
use crate::codegen::LookupTable;
use crate::laplace::StateSpaceFilter;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerilogAEvaluationMode {
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

impl Default for VerilogAEvaluationMode {
    fn default() -> Self {
        Self::NewtonLimited
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
    pub param_given: Vec<bool>,
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
    pub fn advance_state(&mut self) {
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
        // Commit sampled-data filter candidates for the accepted step.
        let time = self.time;
        for filter in &mut self.zi_filters {
            filter.commit(time);
        }
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
            self.param_given.resize(index + 1, false);
        }
        self.param_given[index] = true;
    }

    /// Whether a parameter was explicitly given by the instance.
    #[inline]
    pub fn is_param_given(&self, index: usize) -> bool {
        self.param_given.get(index).copied().unwrap_or(false)
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
    use super::VerilogAEvaluationMode;

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
}
