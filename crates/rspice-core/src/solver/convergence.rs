//! Convergence Helpers for DC Operating Point Analysis
//!
//! Implements multiple strategies to achieve convergence in difficult circuits:
//!
//! 1. **Source Stepping**: Gradually ramp voltage/current sources from 0 to final value
//! 2. **Gmin Stepping**: Add conductances to ground, progressively remove them
//! 3. **Pseudo-Transient**: Use timestep-based damping (capacitors to ground)
//! 4. **Continuation Methods**: Combine multiple strategies with fallback
//!
//! # Algorithm
//! The DC solver attempts convergence in this order:
//! 1. Direct Newton-Raphson (fastest)
//! 2. Source stepping if direct fails
//! 3. Gmin stepping if source stepping fails
//! 4. Pseudo-transient as last resort
//!
//! This is similar to the robust approach of "trying everything" to converge.

use crate::Value;

//=============================================================================
// Constants
//=============================================================================

/// Default initial Gmin value (conductance added to each node)
pub const GMIN_INITIAL: Value = 1e-12;
/// Default final Gmin value (should be negligible)  
pub const GMIN_FINAL: Value = 1e-15;
/// Gmin reduction factor per step
pub const GMIN_FACTOR: Value = 10.0;
/// Maximum Gmin stepping iterations
pub const GMIN_MAX_STEPS: usize = 10;

/// Default source stepping factor
pub const SOURCE_STEP_INITIAL: Value = 0.1;
/// Source step increase factor on success
pub const SOURCE_STEP_FACTOR: Value = 2.0;
/// Minimum source factor to try
pub const SOURCE_STEP_MIN: Value = 0.001;
/// Maximum source stepping iterations
pub const SOURCE_MAX_STEPS: usize = 20;

/// Default pseudo-transient timestep
pub const PTRAN_DT_INITIAL: Value = 1e-9;
/// Pseudo-transient timestep growth factor
pub const PTRAN_DT_FACTOR: Value = 2.0;
/// Maximum pseudo-transient timestep
pub const PTRAN_DT_MAX: Value = 1e-3;
/// Maximum pseudo-transient steps
pub const PTRAN_MAX_STEPS: usize = 100;

//=============================================================================
// Convergence Strategy
//=============================================================================

/// Convergence strategy that succeeded
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvergenceMethod {
    /// Direct Newton-Raphson (no aids)
    Direct,
    /// Source stepping was used
    SourceStepping,
    /// Gmin stepping was used
    GminStepping,
    /// Pseudo-transient was used
    PseudoTransient,
}

/// Result of convergence attempt
#[derive(Debug, Clone)]
pub struct ConvergenceResult {
    /// Method that succeeded
    pub method: ConvergenceMethod,
    /// Total iterations across all attempts
    pub total_iterations: usize,
    /// Number of continuation steps (source/gmin/ptran steps)
    pub continuation_steps: usize,
    /// Final Gmin value (if gmin stepping was used)
    pub final_gmin: Option<Value>,
}

//=============================================================================
// Source Stepping
//=============================================================================

/// Source Stepping Controller
///
/// Ramps all independent sources from 0 to their final values.
/// At each step, Newton-Raphson is used to find the solution.
/// If convergence fails, the step is halved.
#[derive(Debug, Clone)]
pub struct SourceStepper {
    /// Current source scaling factor (0 to 1)
    current_factor: Value,
    /// Step size for increasing factor
    step_size: Value,
    /// Minimum step size before giving up
    min_step: Value,
    /// Number of steps taken
    steps: usize,
    /// Maximum steps allowed
    max_steps: usize,
    /// Whether stepping is complete
    complete: bool,
}

impl SourceStepper {
    /// Create a new source stepper
    pub fn new() -> Self {
        Self {
            current_factor: 0.0,
            step_size: SOURCE_STEP_INITIAL,
            min_step: SOURCE_STEP_MIN,
            steps: 0,
            max_steps: SOURCE_MAX_STEPS,
            complete: false,
        }
    }

    /// Get current source scaling factor (0 to 1)
    #[inline]
    pub fn factor(&self) -> Value {
        self.current_factor
    }

    /// Check if stepping is complete (factor reached 1.0)
    #[inline]
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Get number of steps taken
    #[inline]
    pub fn steps(&self) -> usize {
        self.steps
    }

    /// Called when Newton-Raphson converged at current factor
    /// Advances to next step
    pub fn advance_on_success(&mut self) {
        if self.complete {
            return;
        }

        self.current_factor += self.step_size;
        self.steps += 1;

        // Check if we've reached the end
        if self.current_factor >= 1.0 {
            self.current_factor = 1.0;
            self.complete = true;
            return;
        }

        // Increase step size for next iteration (successful path)
        self.step_size = (self.step_size * SOURCE_STEP_FACTOR).min(1.0 - self.current_factor);
    }

    /// Called when Newton-Raphson failed at current step
    /// Reduces step size and tries again
    /// Returns false if step size is too small (give up)
    pub fn reduce_on_failure(&mut self) -> bool {
        if self.steps >= self.max_steps {
            return false;
        }

        // Reduce step size
        self.step_size *= 0.5;

        if self.step_size < self.min_step {
            return false; // Give up
        }

        // Back up to previous factor
        self.current_factor = (self.current_factor - self.step_size).max(0.0);
        true
    }

    /// Reset for a new solve attempt
    pub fn reset(&mut self) {
        self.current_factor = 0.0;
        self.step_size = SOURCE_STEP_INITIAL;
        self.steps = 0;
        self.complete = false;
    }

    /// Scale a source value by current factor
    #[inline]
    pub fn scale_source(&self, value: Value) -> Value {
        value * self.current_factor
    }
}

impl Default for SourceStepper {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Gmin Stepping
//=============================================================================

/// Gmin Stepping Controller
///
/// Adds small conductances from each node to ground to aid convergence.
/// The conductances are progressively reduced until negligible.
///
/// This is especially helpful for circuits with floating nodes or
/// high-impedance paths.
#[derive(Debug, Clone)]
pub struct GminStepper {
    /// Current Gmin value
    current_gmin: Value,
    /// Target (final) Gmin value
    target_gmin: Value,
    /// Reduction factor per step
    factor: Value,
    /// Number of steps taken
    steps: usize,
    /// Maximum steps allowed
    max_steps: usize,
    /// Whether stepping is complete
    complete: bool,
}

impl GminStepper {
    /// Create a new Gmin stepper
    pub fn new() -> Self {
        Self {
            current_gmin: GMIN_INITIAL,
            target_gmin: GMIN_FINAL,
            factor: GMIN_FACTOR,
            steps: 0,
            max_steps: GMIN_MAX_STEPS,
            complete: false,
        }
    }

    /// Create with custom parameters
    pub fn with_params(initial: Value, target: Value, factor: Value) -> Self {
        Self {
            current_gmin: initial,
            target_gmin: target,
            factor,
            steps: 0,
            max_steps: GMIN_MAX_STEPS,
            complete: false,
        }
    }

    /// Get current Gmin value
    #[inline]
    pub fn gmin(&self) -> Value {
        self.current_gmin
    }

    /// Check if stepping is complete
    #[inline]
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Get number of steps taken
    #[inline]
    pub fn steps(&self) -> usize {
        self.steps
    }

    /// Called when Newton-Raphson converged at current Gmin
    /// Reduces Gmin for next step
    pub fn advance_on_success(&mut self) {
        if self.complete {
            return;
        }

        self.current_gmin /= self.factor;
        self.steps += 1;

        if self.current_gmin <= self.target_gmin {
            self.current_gmin = self.target_gmin;
            self.complete = true;
        }
    }

    /// Called when Newton-Raphson failed
    /// Increases Gmin and retries
    /// Returns false if Gmin is already at maximum
    pub fn increase_on_failure(&mut self) -> bool {
        if self.steps >= self.max_steps {
            return false;
        }

        // Increase Gmin
        self.current_gmin *= self.factor;

        // If Gmin is getting too large, give up
        if self.current_gmin > 1e-6 {
            return false;
        }

        true
    }

    /// Reset for a new solve attempt
    pub fn reset(&mut self) {
        self.current_gmin = GMIN_INITIAL;
        self.steps = 0;
        self.complete = false;
    }

    /// Stamp Gmin to diagonal of matrix for a node
    /// Returns the conductance value to add
    #[inline]
    pub fn stamp_value(&self) -> Value {
        self.current_gmin
    }
}

impl Default for GminStepper {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Pseudo-Transient Analysis
//=============================================================================

/// Pseudo-Transient Controller
///
/// Solves DC operating point by running a "fake" transient with
/// very long timesteps. Capacitor-like elements damp the solution.
///
/// The timestep starts small and increases as the solution stabilizes.
/// When the timestep is large enough, the solution is essentially DC.
#[derive(Debug, Clone)]
pub struct PseudoTransient {
    /// Current pseudo-timestep
    dt: Value,
    /// Initial timestep
    dt_initial: Value,
    /// Maximum timestep (considered "DC")
    dt_max: Value,
    /// Growth factor for timestep
    growth_factor: Value,
    /// Number of steps taken
    steps: usize,
    /// Maximum steps allowed
    max_steps: usize,
    /// Pseudo-time accumulated
    time: Value,
    /// Whether analysis is complete
    complete: bool,
}

impl PseudoTransient {
    /// Create a new pseudo-transient controller
    pub fn new() -> Self {
        Self {
            dt: PTRAN_DT_INITIAL,
            dt_initial: PTRAN_DT_INITIAL,
            dt_max: PTRAN_DT_MAX,
            growth_factor: PTRAN_DT_FACTOR,
            steps: 0,
            max_steps: PTRAN_MAX_STEPS,
            time: 0.0,
            complete: false,
        }
    }

    /// Create with custom parameters
    pub fn with_params(dt_initial: Value, dt_max: Value, growth_factor: Value) -> Self {
        Self {
            dt: dt_initial,
            dt_initial,
            dt_max,
            growth_factor,
            steps: 0,
            max_steps: PTRAN_MAX_STEPS,
            time: 0.0,
            complete: false,
        }
    }

    /// Get current pseudo-timestep
    #[inline]
    pub fn dt(&self) -> Value {
        self.dt
    }

    /// Get accumulated pseudo-time
    #[inline]
    pub fn time(&self) -> Value {
        self.time
    }

    /// Check if pseudo-transient is complete
    #[inline]
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Get number of steps taken
    #[inline]
    pub fn steps(&self) -> usize {
        self.steps
    }

    /// Called when Newton-Raphson converged at current timestep
    /// Increases timestep for next iteration
    pub fn advance_on_success(&mut self) {
        if self.complete {
            return;
        }

        self.time += self.dt;
        self.steps += 1;

        // Increase timestep
        self.dt = (self.dt * self.growth_factor).min(self.dt_max);

        // Check if we've reached "DC" (very large timestep)
        if self.dt >= self.dt_max && self.steps > 3 {
            self.complete = true;
        }

        if self.steps >= self.max_steps {
            self.complete = true;
        }
    }

    /// Called when Newton-Raphson failed
    /// Reduces timestep and retries
    /// Returns false if timestep is too small
    pub fn reduce_on_failure(&mut self) -> bool {
        if self.steps >= self.max_steps {
            return false;
        }

        // Reduce timestep
        self.dt /= self.growth_factor;

        // If timestep is getting too small, give up
        if self.dt < 1e-15 {
            return false;
        }

        true
    }

    /// Reset for a new solve attempt
    pub fn reset(&mut self) {
        self.dt = self.dt_initial;
        self.time = 0.0;
        self.steps = 0;
        self.complete = false;
    }

    /// Calculate equivalent conductance for pseudo-capacitor
    /// G_eq = C_pseudo / dt
    /// Using C_pseudo = 1e-12 (1pF equivalent)
    #[inline]
    pub fn conductance(&self, _node: usize) -> Value {
        // Use a small pseudo-capacitance
        let c_pseudo = 1e-12;
        c_pseudo / self.dt
    }

    /// Calculate equivalent current source for pseudo-capacitor
    /// I_eq = G_eq * V_prev
    #[inline]
    pub fn current(&self, v_prev: Value) -> Value {
        let c_pseudo = 1e-12;
        let g_eq = c_pseudo / self.dt;
        g_eq * v_prev
    }
}

impl Default for PseudoTransient {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Unified Convergence Controller
//=============================================================================

/// Unified controller that manages all convergence strategies
#[derive(Debug, Clone)]
pub struct ConvergenceController {
    /// Source stepping controller
    pub source_stepper: SourceStepper,
    /// Gmin stepping controller
    pub gmin_stepper: GminStepper,
    /// Pseudo-transient controller
    pub pseudo_transient: PseudoTransient,
    /// Currently active method
    active_method: ConvergenceMethod,
    /// Total iterations across all methods
    total_iterations: usize,
    /// Whether any method succeeded
    succeeded: bool,
}

impl ConvergenceController {
    /// Create a new convergence controller
    pub fn new() -> Self {
        Self {
            source_stepper: SourceStepper::new(),
            gmin_stepper: GminStepper::new(),
            pseudo_transient: PseudoTransient::new(),
            active_method: ConvergenceMethod::Direct,
            total_iterations: 0,
            succeeded: false,
        }
    }

    /// Get currently active method
    #[inline]
    pub fn active_method(&self) -> ConvergenceMethod {
        self.active_method
    }

    /// Set active method (called when switching strategies)
    pub fn set_method(&mut self, method: ConvergenceMethod) {
        self.active_method = method;
        match method {
            ConvergenceMethod::Direct => {
                // No special setup
            }
            ConvergenceMethod::SourceStepping => {
                self.source_stepper.reset();
            }
            ConvergenceMethod::GminStepping => {
                self.gmin_stepper.reset();
            }
            ConvergenceMethod::PseudoTransient => {
                self.pseudo_transient.reset();
            }
        }
    }

    /// Record iterations from a Newton-Raphson attempt
    pub fn add_iterations(&mut self, iters: usize) {
        self.total_iterations += iters;
    }

    /// Mark as succeeded
    pub fn mark_success(&mut self) {
        self.succeeded = true;
    }

    /// Check if convergence has been achieved
    pub fn is_complete(&self) -> bool {
        match self.active_method {
            ConvergenceMethod::Direct => self.succeeded,
            ConvergenceMethod::SourceStepping => {
                self.source_stepper.is_complete() && self.succeeded
            }
            ConvergenceMethod::GminStepping => self.gmin_stepper.is_complete() && self.succeeded,
            ConvergenceMethod::PseudoTransient => {
                self.pseudo_transient.is_complete() && self.succeeded
            }
        }
    }

    /// Get result summary
    pub fn result(&self) -> ConvergenceResult {
        let continuation_steps = match self.active_method {
            ConvergenceMethod::Direct => 0,
            ConvergenceMethod::SourceStepping => self.source_stepper.steps(),
            ConvergenceMethod::GminStepping => self.gmin_stepper.steps(),
            ConvergenceMethod::PseudoTransient => self.pseudo_transient.steps(),
        };

        let final_gmin = if self.active_method == ConvergenceMethod::GminStepping {
            Some(self.gmin_stepper.gmin())
        } else {
            None
        };

        ConvergenceResult {
            method: self.active_method,
            total_iterations: self.total_iterations,
            continuation_steps,
            final_gmin,
        }
    }

    /// Reset all controllers for a new solve
    pub fn reset(&mut self) {
        self.source_stepper.reset();
        self.gmin_stepper.reset();
        self.pseudo_transient.reset();
        self.active_method = ConvergenceMethod::Direct;
        self.total_iterations = 0;
        self.succeeded = false;
    }
}

impl Default for ConvergenceController {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Tests
//=============================================================================

