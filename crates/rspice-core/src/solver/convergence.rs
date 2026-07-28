//! Continuation Controllers for Nonlinear Solves
//!
//! Two homotopy controllers that walk a hard nonlinear problem to its final
//! operating point one tractable step at a time:
//!
//! - **Source Stepping**: Gradually ramp voltage/current sources from 0 to
//!   their final values
//! - **Pseudo-Transient**: Use timestep-based damping (capacitors to ground)
//!
//! Each controller only tracks the continuation parameter and its step-size
//! policy; the caller owns the Newton loop and reports success or failure
//! back through `advance_on_success` / `reduce_on_failure`. Harmonic balance
//! drives both (see `analysis::advanced::harmonic_balance::solver`), and the
//! engine's DC convergence aids in `engine::convergence` drive the
//! pseudo-transient path.

use crate::Value;

//=============================================================================
// Constants
//=============================================================================

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
