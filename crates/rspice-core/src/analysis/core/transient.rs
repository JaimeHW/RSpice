//! Transient Time-Domain Analysis

#![allow(clippy::type_complexity)]
use crate::Value;
use crate::analysis::AnalysisConfig;

/// Transient Analysis engine
#[derive(Debug)]
#[allow(dead_code)] // Reserved for full integration
pub struct TransientAnalysis {
    config: AnalysisConfig,
    /// Simulation stop time
    stop_time: Value,
    /// Maximum time step
    max_step: Value,
    /// Initial time step
    initial_step: Value,
    /// Integration method
    method: IntegrationMethod,
    /// Use Initial Conditions (UIC) - skip DC operating point, use IC= values
    /// When true:
    ///   - Skip DC operating point calculation
    ///   - Use IC= values on capacitors and inductors directly
    ///   - Set all unspecified node voltages to 0V
    ///
    /// This is useful for oscillators and circuits that don't have a stable DC OP
    use_initial_conditions: bool,
}

type ChargeLteInputs<'a> = (&'a [Value], &'a [Value], &'a [Value], &'a [Value]);

/// Numerical integration methods for transient analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationMethod {
    /// Backward Euler (first order, very stable)
    BackwardEuler,
    /// Trapezoidal rule (second order, A-stable)
    Trapezoidal,
    /// Gear order 2 (BDF2, good for stiff systems)
    Gear2,
    /// TrapGear: Hybrid method that auto-switches between Trapezoidal and Gear2
    /// Uses Trapezoidal for smooth regions (better accuracy) and
    /// switches to Gear2 at discontinuities/oscillations (better stability)
    TrapGear,
}

impl TransientAnalysis {
    pub fn new(stop_time: Value, max_step: Value) -> Self {
        Self {
            config: AnalysisConfig::default(),
            stop_time,
            max_step,
            initial_step: max_step / 100.0,
            method: IntegrationMethod::Trapezoidal,
            use_initial_conditions: false,
        }
    }

    /// Set integration method
    pub fn with_method(mut self, method: IntegrationMethod) -> Self {
        self.method = method;
        self
    }

    /// Set initial timestep
    pub fn with_initial_step(mut self, step: Value) -> Self {
        self.initial_step = step;
        self
    }

    /// Enable Use Initial Conditions (UIC) mode
    ///
    /// When UIC is enabled:
    /// - Skip DC operating point calculation at t=0
    /// - Use IC= values on capacitors (voltage) and inductors (current)
    /// - Unspecified nodes start at 0V
    ///
    /// This is useful for:
    /// - Oscillators (no stable DC OP)
    /// - Circuits where you want to specify exact starting conditions
    /// - Faster simulation when DC OP is not needed
    pub fn with_uic(mut self, uic: bool) -> Self {
        self.use_initial_conditions = uic;
        self
    }

    /// Check if UIC (Use Initial Conditions) mode is enabled
    pub fn uic(&self) -> bool {
        self.use_initial_conditions
    }

    /// Get stop time
    pub fn stop_time(&self) -> Value {
        self.stop_time
    }

    /// Get maximum step
    pub fn max_step(&self) -> Value {
        self.max_step
    }

    /// Get integration method
    pub fn method(&self) -> IntegrationMethod {
        self.method
    }
}

impl Default for TransientAnalysis {
    fn default() -> Self {
        Self::new(1e-3, 1e-6) // 1ms simulation, 1us max step
    }
}

/// Adaptive timestep controller
#[derive(Debug)]
pub struct TimestepController {
    /// Current timestep
    current_dt: Value,
    /// Minimum allowed timestep
    min_dt: Value,
    /// Maximum allowed timestep
    max_dt: Value,
    /// Target local truncation error
    target_lte: Value,
    /// Previous timestep (for Gear methods)
    prev_dt: Value,
}

impl TimestepController {
    pub fn new(initial_dt: Value, min_dt: Value, max_dt: Value) -> Self {
        Self {
            current_dt: initial_dt,
            min_dt,
            max_dt,
            target_lte: 1e-3,
            prev_dt: initial_dt,
        }
    }

    /// Get current timestep
    pub fn dt(&self) -> Value {
        self.current_dt
    }

    /// Adjust timestep based on local truncation error estimate
    pub fn adjust(&mut self, lte_estimate: Value) -> Value {
        // Calculate new timestep using LTE estimate
        // For trapezoidal: LTE ~ O(dt^3), so dt_new = dt * (target_lte / lte_estimate)^(1/3)

        if lte_estimate < 1e-15 {
            // Error too small, increase timestep
            self.prev_dt = self.current_dt;
            self.current_dt = (self.current_dt * 2.0).min(self.max_dt);
        } else {
            let ratio = self.target_lte / lte_estimate;
            let factor = ratio.powf(1.0 / 3.0);

            // Limit growth/shrink rate
            let factor = factor.clamp(0.5, 2.0);

            self.prev_dt = self.current_dt;
            self.current_dt = (self.current_dt * factor).clamp(self.min_dt, self.max_dt);
        }

        self.current_dt
    }

    /// Force a specific timestep (for breakpoints)
    pub fn force_step(&mut self, dt: Value) {
        self.prev_dt = self.current_dt;
        self.current_dt = dt.clamp(self.min_dt, self.max_dt);
    }

    /// Check if timestep was rejected (too small)
    pub fn is_at_minimum(&self) -> bool {
        self.current_dt <= self.min_dt * 1.001
    }
}

/// Minimum timestep to use immediately after a breakpoint (for restart behavior)
const MIN_STEP_AFTER_BREAKPOINT: Value = 1e-12;

/// Tolerance for detecting exact breakpoint landing
const BREAKPOINT_TOLERANCE: Value = 1e-15;

/// Breakpoint manager for handling discontinuities
///
/// Ensures solver lands exactly on breakpoints and restarts with minimal timestep
/// immediately after, preventing numerical ringing from stepping over discontinuities.
#[derive(Debug, Default)]
pub struct BreakpointManager {
    /// Sorted list of breakpoint times
    breakpoints: Vec<Value>,
    /// Index of next unprocessed breakpoint (for efficiency)
    current_index: usize,
    /// Flag indicating we just passed a breakpoint
    just_passed_breakpoint: bool,
}

impl BreakpointManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a breakpoint time (deduplicates automatically)
    pub fn add(&mut self, time: Value) -> bool {
        // Check for duplicates within tolerance
        if self
            .breakpoints
            .iter()
            .any(|&t| (t - time).abs() < BREAKPOINT_TOLERANCE)
        {
            return false;
        }

        // Insert in sorted order
        let pos = self.breakpoints.iter().position(|&t| t > time);
        match pos {
            Some(i) => self.breakpoints.insert(i, time),
            None => self.breakpoints.push(time),
        }
        true
    }

    /// Add breakpoints from a periodic source (pulse edges, clock, etc.)
    pub fn add_periodic(&mut self, start: Value, period: Value, end: Value) {
        let mut t = start;
        while t <= end {
            self.add(t);
            t += period;
        }
    }

    /// Get next breakpoint after given time
    pub fn next_after(&self, time: Value) -> Option<Value> {
        self.breakpoints
            .iter()
            .skip(self.current_index)
            .copied()
            .find(|&t| t > time + BREAKPOINT_TOLERANCE)
    }

    /// Check if current time is exactly at a breakpoint
    pub fn at_breakpoint(&self, time: Value) -> bool {
        self.breakpoints
            .iter()
            .any(|&bp| (time - bp).abs() < BREAKPOINT_TOLERANCE)
    }

    /// Limit timestep to land exactly on next breakpoint
    ///
    /// Returns (adjusted_dt, will_land_on_breakpoint)
    pub fn limit_step(&mut self, current_time: Value, proposed_dt: Value) -> (Value, bool) {
        match self.next_after(current_time) {
            Some(bp) => {
                let time_to_bp = bp - current_time;

                if proposed_dt >= time_to_bp {
                    // Force landing exactly on breakpoint
                    self.just_passed_breakpoint = false; // Will be set after solving at BP
                    if time_to_bp < MIN_STEP_AFTER_BREAKPOINT {
                        (time_to_bp, true)
                    } else {
                        (time_to_bp.max(MIN_STEP_AFTER_BREAKPOINT), true)
                    }
                } else if proposed_dt > time_to_bp * 0.9 {
                    // Close to breakpoint - go directly there
                    (time_to_bp, true)
                } else {
                    (proposed_dt, false)
                }
            }
            None => (proposed_dt, false),
        }
    }

    /// Mark that we just solved at a breakpoint (call after solving at BP)
    /// Returns the recommended minimal timestep for restart
    pub fn mark_breakpoint_solved(&mut self, time: Value) -> Value {
        // Advance current_index past this breakpoint
        while self.current_index < self.breakpoints.len()
            && self.breakpoints[self.current_index] <= time + BREAKPOINT_TOLERANCE
        {
            self.current_index += 1;
        }
        self.just_passed_breakpoint = true;
        MIN_STEP_AFTER_BREAKPOINT
    }

    /// Check if we just passed a breakpoint and should use minimal timestep
    pub fn should_use_minimal_step(&self) -> bool {
        self.just_passed_breakpoint
    }

    /// Clear the just-passed flag (call after first step post-breakpoint)
    pub fn clear_breakpoint_flag(&mut self) {
        self.just_passed_breakpoint = false;
    }

    /// Reset the manager for a new simulation
    pub fn reset(&mut self) {
        self.current_index = 0;
        self.just_passed_breakpoint = false;
    }

    /// Get total number of breakpoints
    pub fn len(&self) -> usize {
        self.breakpoints.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.breakpoints.is_empty()
    }

    /// Borrow the sorted breakpoint schedule.
    pub fn times(&self) -> &[Value] {
        &self.breakpoints
    }
}

/// Local Truncation Error (LTE) estimator for adaptive timestep
///
/// Uses difference between predicted and calculated values to estimate
/// the integration error. This allows rejecting timesteps that converge
/// numerically but are physically inaccurate.
///
/// Supports both standard extrapolation and Richardson extrapolation for
/// higher accuracy LTE estimates.
#[derive(Debug)]
pub struct LteEstimator {
    /// Previous solution vector (t - dt)
    prev_solution: Vec<Value>,
    /// Solution before previous (t - 2*dt)
    prev_prev_solution: Vec<Value>,
    /// Solution from 3 steps ago (t - 3*dt) for Richardson extrapolation
    prev_prev_prev_solution: Vec<Value>,
    /// Previous timestep
    prev_dt: Value,
    /// Timestep before previous
    prev_prev_dt: Value,
    /// Relative LTE tolerance.
    reltol: Value,
    /// Absolute LTE tolerance floor for state variables.
    abstol: Value,
    /// Number of valid history entries
    history_count: usize,
    /// Current integration method (for order-aware scaling)
    method_order: u32,
}

impl LteEstimator {
    /// Create a new LTE estimator with legacy single-tolerance semantics.
    ///
    /// `tolerance` is used for both relative and absolute LTE control.
    pub fn new(tolerance: Value) -> Self {
        Self::with_tolerances(tolerance, tolerance)
    }

    /// Create a new LTE estimator with explicit relative and absolute tolerances.
    pub fn with_tolerances(reltol: Value, abstol: Value) -> Self {
        let reltol = if reltol.is_finite() && reltol > 0.0 {
            reltol
        } else {
            1e-3
        };
        let abstol = if abstol.is_finite() && abstol > 0.0 {
            abstol
        } else {
            reltol
        };

        Self {
            prev_solution: Vec::new(),
            prev_prev_solution: Vec::new(),
            prev_prev_prev_solution: Vec::new(),
            prev_dt: 0.0,
            prev_prev_dt: 0.0,
            reltol,
            abstol,
            history_count: 0,
            method_order: 2, // Default to trapezoidal order
        }
    }

    #[inline]
    fn lte_scale_denominator(&self, magnitude: Value) -> Value {
        // Enforce SPICE-like weighted tolerance:
        // |err| <= ABSTOL + RELTOL * |x|
        // Rearranged to a normalized metric compared against RELTOL:
        // |err| / (ABSTOL/RELTOL + |x|) <= RELTOL
        let reltol = self.reltol.max(1e-30);
        (self.abstol / reltol + magnitude.abs()).max(1e-30)
    }

    #[inline]
    fn predict_next_value(
        &self,
        prev: Value,
        prev_prev: Value,
        prev_prev_prev: Value,
        dt: Value,
    ) -> Value {
        if self.method_order >= 2 && self.history_count >= 3 && self.prev_prev_dt > 0.0 {
            // Evaluate the quadratic interpolant through the previous three
            // accepted points at the proposed next time. This keeps the LTE
            // estimate order-consistent with trapezoidal/Gear2 instead of
            // falling back to a first-order predictor that can over-restrict
            // smooth waveforms.
            let h1 = self.prev_dt;
            let h2 = self.prev_prev_dt;
            let t0 = 0.0;
            let t1 = -h1;
            let t2 = -(h1 + h2);
            let t = dt;

            let l0 = (t - t1) * (t - t2) / ((t0 - t1) * (t0 - t2));
            let l1 = (t - t0) * (t - t2) / ((t1 - t0) * (t1 - t2));
            let l2 = (t - t0) * (t - t1) / ((t2 - t0) * (t2 - t1));
            prev * l0 + prev_prev * l1 + prev_prev_prev * l2
        } else if self.history_count >= 2 && self.prev_dt > 0.0 {
            let slope = (prev - prev_prev) / self.prev_dt;
            prev + slope * dt
        } else {
            prev
        }
    }

    /// Set the integration method order for accurate timestep scaling
    /// - BackwardEuler: order 1
    /// - Trapezoidal, Gear2: order 2
    #[inline]
    pub fn set_method_order(&mut self, order: u32) {
        self.method_order = order.max(1);
    }

    /// Record a solution point for history
    pub fn record(&mut self, solution: &[Value], dt: Value) {
        // Shift history: prev_prev_prev <- prev_prev <- prev <- new
        self.prev_prev_prev_solution = std::mem::take(&mut self.prev_prev_solution);
        self.prev_prev_solution = std::mem::take(&mut self.prev_solution);
        self.prev_solution = solution.to_vec();
        self.prev_prev_dt = self.prev_dt;
        self.prev_dt = dt;
        if self.history_count < 3 {
            self.history_count += 1;
        }
    }

    /// Estimate LTE using linear extrapolation vs actual value
    /// Returns (lte_estimate, should_accept)
    pub fn estimate(&self, current: &[Value], dt: Value) -> (Value, bool) {
        // Need at least one previous point to estimate
        if self.history_count < 1 || self.prev_solution.len() != current.len() {
            return (0.0, true); // Accept, no history yet
        }

        let mut max_lte = 0.0_f64;

        // For trapezoidal, LTE ~ (dt^3 / 12) * d^3v/dt^3
        // We approximate by comparing predicted (linear extrapolation) vs actual
        for (i, &curr_val) in current.iter().enumerate() {
            let prev_val = self.prev_solution[i];
            let prev_prev_val = self.prev_prev_solution.get(i).copied().unwrap_or(prev_val);
            let prev_prev_prev_val = self
                .prev_prev_prev_solution
                .get(i)
                .copied()
                .unwrap_or(prev_prev_val);
            let predicted =
                self.predict_next_value(prev_val, prev_prev_val, prev_prev_prev_val, dt);

            // LTE estimate: |actual - predicted| with weighted SPICE-like scaling.
            let lte = (curr_val - predicted).abs();
            let scale = self.lte_scale_denominator(curr_val.abs().max(predicted.abs()));
            let normalized_lte = lte / scale;

            max_lte = max_lte.max(normalized_lte);
        }

        let accept = max_lte <= self.reltol;
        (max_lte, accept)
    }

    /// Richardson extrapolation LTE estimate (more accurate)
    ///
    /// Uses solutions computed with steps h and h/2 to estimate the true error:
    /// `LTE â‰ˆ (x_h - x_{h/2}) / (2^p - 1)` where p is the method order.
    ///
    /// This provides a more accurate LTE estimate by exploiting the known
    /// convergence order of the integration method.
    ///
    /// # Arguments
    /// * `x_full` - Solution computed with full timestep h
    /// * `x_half` - Solution computed with two half-steps h/2
    ///
    /// # Returns
    /// (lte_estimate, should_accept)
    pub fn richardson_estimate(&self, x_full: &[Value], x_half: &[Value]) -> (Value, bool) {
        if x_full.len() != x_half.len() || x_full.is_empty() {
            return (0.0, true);
        }

        let order_factor = (1u64 << self.method_order) as Value - 1.0; // 2^p - 1
        let mut max_lte = 0.0_f64;

        for (&full, &half) in x_full.iter().zip(x_half.iter()) {
            // Richardson extrapolation error estimate
            let richardson_error = (half - full).abs() / order_factor;

            let scale = self.lte_scale_denominator(half.abs().max(full.abs()));
            let normalized = richardson_error / scale;

            max_lte = max_lte.max(normalized);
        }

        let accept = max_lte <= self.reltol;
        (max_lte, accept)
    }

    /// Get recommended timestep scaling factor based on LTE
    /// Uses method order for proper scaling exponent
    pub fn recommend_scale(&self, lte: Value) -> Value {
        if !lte.is_finite() {
            0.25
        } else if lte < 1e-15 {
            2.0 // Error negligible, can increase
        } else {
            // Optimal scaling: (tol/lte)^(1/(p+1)) where p is method order
            // For order 2: exponent = 1/3, for order 1: exponent = 1/2
            let exponent = 1.0 / (self.method_order as Value + 1.0);
            let ratio = self.reltol / lte;
            ratio.powf(exponent).clamp(0.25, 2.0)
        }
    }

    /// Charge-based LTE estimation for capacitive elements (MOSFET gates)
    ///
    /// For charge-conserving models (BSIM4, etc.), the fundamental relationship is:
    /// ```text
    /// dQ/dt = I
    /// ```
    ///
    /// This method estimates LTE based on charge conservation errors rather than
    /// voltage errors. This is more accurate for capacitive elements because:
    /// 1. It directly tracks the physical quantity being integrated (charge)
    /// 2. It catches errors in current integration that voltage-based LTE might miss
    /// 3. It ensures charge conservation across the simulation
    ///
    /// # Arguments
    /// * `charges` - Current charge values at each node
    /// * `prev_charges` - Charge values from previous timestep
    /// * `currents` - Current values (dQ/dt) at each node
    /// * `dt` - Current timestep
    ///
    /// # Returns
    /// (max_charge_lte, should_accept)
    ///
    /// # Theory
    /// For trapezoidal integration: Q_{n+1} = Q_n + dt/2 * (I_n + I_{n+1})
    /// The LTE is estimated as: |Q_actual - Q_predicted| / |Q_actual|
    pub fn estimate_charge_lte(
        &self,
        charges: &[Value],
        prev_charges: &[Value],
        currents: &[Value],
        prev_currents: &[Value],
        dt: Value,
    ) -> (Value, bool) {
        if charges.len() != prev_charges.len() || charges.is_empty() {
            return (0.0, true);
        }

        if charges.len() != currents.len() || charges.len() != prev_currents.len() {
            return (0.0, true);
        }

        let mut max_lte = 0.0_f64;

        // For each charge node, check conservation error
        for i in 0..charges.len() {
            let q_curr = charges[i];
            let q_prev = prev_charges[i];
            let i_curr = currents[i];
            let i_prev = prev_currents[i];

            // Trapezoidal prediction: Q_n+1 = Q_n + dt/2 * (I_n + I_{n+1})
            let q_predicted = q_prev + dt * 0.5 * (i_prev + i_curr);

            // Charge conservation error
            let charge_error = (q_curr - q_predicted).abs();

            // Normalize by charge magnitude (with minimum to avoid division issues)
            let q_magnitude = q_curr.abs().max(q_prev.abs()).max(1e-18);
            let normalized_lte = charge_error / q_magnitude;

            max_lte = max_lte.max(normalized_lte);
        }

        let accept = max_lte <= self.reltol;
        (max_lte, accept)
    }

    /// Combined voltage and charge LTE estimation
    ///
    /// Uses both voltage-based and charge-based LTE to determine timestep.
    /// Takes the maximum of both to ensure both voltages and charges are
    /// accurately integrated.
    ///
    /// # Arguments
    /// * `voltages` - Current node voltages
    /// * `dt` - Current timestep
    /// * `charges` - Gate/junction charges (if available)
    /// * `prev_charges` - Previous gate/junction charges
    /// * `currents` - Node currents
    /// * `prev_currents` - Previous node currents
    pub fn estimate_combined(
        &self,
        voltages: &[Value],
        dt: Value,
        charges: Option<ChargeLteInputs<'_>>,
    ) -> (Value, bool) {
        // Standard voltage-based LTE
        let (v_lte, v_accept) = self.estimate(voltages, dt);

        // Charge-based LTE if charge data provided
        let (combined_lte, combined_accept) = if let Some((q_curr, q_prev, i_curr, i_prev)) =
            charges
        {
            let (q_lte, q_accept) = self.estimate_charge_lte(q_curr, q_prev, i_curr, i_prev, dt);
            (v_lte.max(q_lte), v_accept && q_accept)
        } else {
            (v_lte, v_accept)
        };

        (combined_lte, combined_accept)
    }

    /// Reset history (e.g., after discontinuity)
    pub fn reset(&mut self) {
        self.prev_solution.clear();
        self.prev_prev_solution.clear();
        self.prev_prev_prev_solution.clear();
        self.history_count = 0;
    }
}

//=============================================================================
// Companion Model Coefficients for Integration Methods
//=============================================================================

/// Companion model coefficients for numerical integration
///
/// Each integration method converts differential elements (C, L) into
/// equivalent conductances and current/voltage sources using these coefficients.
#[derive(Debug, Clone, Copy)]
pub struct CompanionCoefficients {
    /// Equivalent conductance coefficient: G_eq = coeff_g * C / dt
    pub coeff_g: Value,
    /// History coefficient for v_n (most recent)
    pub coeff_v_n: Value,
    /// History coefficient for v_{n-1}
    pub coeff_v_n_minus_1: Value,
    /// Whether v_{n-1} history is needed
    pub needs_two_history: bool,
    /// Whether i_n current history is needed (Trapezoidal)
    pub needs_current_history: bool,
}

impl CompanionCoefficients {
    /// Get coefficients for Backward Euler (first order, unconditionally stable)
    ///
    /// CÂ·dv/dt = i  â†’  CÂ·(v_{n+1} - v_n)/dt = i_{n+1}
    /// Companion: G_eq = C/dt, I_eq = G_eqÂ·v_n
    #[inline]
    pub fn backward_euler() -> Self {
        Self {
            coeff_g: 1.0,
            coeff_v_n: 1.0,
            coeff_v_n_minus_1: 0.0,
            needs_two_history: false,
            needs_current_history: false,
        }
    }

    /// Get coefficients for Trapezoidal rule (second order, A-stable)
    ///
    /// Uses average of derivatives at n and n+1:
    /// CÂ·(v_{n+1} - v_n)/dt = 0.5Â·(i_{n+1} + i_n)
    /// Companion: G_eq = 2C/dt, I_eq = G_eqÂ·v_n + i_n
    #[inline]
    pub fn trapezoidal() -> Self {
        Self {
            coeff_g: 2.0,
            coeff_v_n: 2.0,
            coeff_v_n_minus_1: 0.0,
            needs_two_history: false,
            needs_current_history: true,
        }
    }

    /// Get coefficients for Gear2/BDF2 (second order, L-stable, good for stiff)
    ///
    /// Uses backward difference formula:
    /// (3Â·v_{n+1} - 4Â·v_n + v_{n-1}) / (2Â·dt) = f_{n+1}
    /// Companion: G_eq = 3C/(2Â·dt), I_eq = (4CÂ·v_n - CÂ·v_{n-1})/(2Â·dt)
    #[inline]
    pub fn gear2() -> Self {
        Self {
            coeff_g: 1.5,            // 3/2
            coeff_v_n: 2.0,          // 4/2 = 2
            coeff_v_n_minus_1: -0.5, // -1/2
            needs_two_history: true,
            needs_current_history: false,
        }
    }

    /// Get coefficients for the specified integration method
    #[inline]
    pub fn for_method(method: IntegrationMethod) -> Self {
        match method {
            IntegrationMethod::BackwardEuler => Self::backward_euler(),
            IntegrationMethod::Trapezoidal => Self::trapezoidal(),
            IntegrationMethod::Gear2 => Self::gear2(),
            IntegrationMethod::TrapGear => Self::trapezoidal(), // Default, actual method chosen dynamically
        }
    }

    /// Calculate equivalent conductance for a capacitor
    #[inline]
    pub fn capacitor_geq(&self, capacitance: Value, dt: Value) -> Value {
        self.coeff_g * capacitance / dt
    }

    /// Calculate equivalent current source for a capacitor
    /// v_n is current voltage, v_n_minus_1 is previous voltage (for Gear2)
    #[inline]
    pub fn capacitor_ieq(
        &self,
        capacitance: Value,
        dt: Value,
        v_n: Value,
        v_n_minus_1: Value,
        i_n: Value,
    ) -> Value {
        let mut ieq = self.coeff_v_n * capacitance * v_n / dt;
        if self.needs_two_history {
            ieq += self.coeff_v_n_minus_1 * capacitance * v_n_minus_1 / dt;
        }
        if self.needs_current_history {
            ieq += i_n;
        }
        ieq
    }

    /// Calculate equivalent resistance for an inductor
    #[inline]
    pub fn inductor_req(&self, inductance: Value, dt: Value) -> Value {
        self.coeff_g * inductance / dt
    }

    /// Calculate equivalent voltage source for an inductor
    /// i_n is current current, i_n_minus_1 is previous current (for Gear2)
    #[inline]
    pub fn inductor_veq(
        &self,
        inductance: Value,
        dt: Value,
        i_n: Value,
        i_n_minus_1: Value,
        v_n: Value,
    ) -> Value {
        let base = self.coeff_g * inductance * i_n / dt + v_n;
        if self.needs_two_history {
            // For BDF2 inductor: V_eq = R_eqÂ·i_n + (4/3)Â·LÂ·i_n/dt - (1/3)Â·LÂ·i_{n-1}/dt
            base + self.coeff_v_n_minus_1 * inductance * i_n_minus_1 / dt
        } else {
            base
        }
    }
}

/// TrapGear Controller for automatic integration method switching
///
/// This implements the hybrid Trapezoidal/Gear-2 method
/// to suppress numerical oscillation (ringing) at switching transitions.
///
/// # Algorithm
/// - Track solution derivative sign changes for each node
/// - When 3+ consecutive sign changes detected (oscillation), switch to Gear-2
/// - After 2-3 smooth steps without oscillation, switch back to Trapezoidal
/// - At breakpoints (source discontinuities), preemptively use Gear-2
#[derive(Debug)]
pub struct TrapGearController {
    /// Current effective method being used
    current_method: IntegrationMethod,
    /// Previous solution values for derivative calculation
    prev_values: Vec<Value>,
    /// Previous derivative signs (true = positive, false = negative)
    prev_signs: Vec<bool>,
    /// Count of consecutive sign changes per node
    sign_change_count: Vec<usize>,
    /// Steps since last oscillation detected
    smooth_steps: usize,
    /// Whether we're at or near a breakpoint
    at_breakpoint: bool,
    /// Threshold for consecutive sign changes to trigger Gear switch
    oscillation_threshold: usize,
    /// Steps of smooth behavior before returning to Trapezoidal
    recovery_steps: usize,
}

impl TrapGearController {
    /// Create a new TrapGear controller
    pub fn new() -> Self {
        Self {
            current_method: IntegrationMethod::Trapezoidal,
            prev_values: Vec::new(),
            prev_signs: Vec::new(),
            sign_change_count: Vec::new(),
            smooth_steps: 0,
            at_breakpoint: false,
            oscillation_threshold: 3,
            recovery_steps: 2,
        }
    }

    /// Get the current effective integration method
    #[inline]
    pub fn current_method(&self) -> IntegrationMethod {
        self.current_method
    }

    /// Signal that we're approaching or at a breakpoint
    pub fn set_at_breakpoint(&mut self, at_bp: bool) {
        self.at_breakpoint = at_bp;
        if at_bp {
            // Preemptively switch to Gear-2 at breakpoints
            self.current_method = IntegrationMethod::Gear2;
            self.smooth_steps = 0;
        }
    }

    /// Update the controller with new solution values
    /// Returns true if oscillation was detected
    pub fn update(&mut self, solution: &[Value], _dt: Value) -> bool {
        // Initialize on first call
        if self.prev_values.len() != solution.len() {
            self.prev_values = solution.to_vec();
            self.prev_signs = vec![true; solution.len()];
            self.sign_change_count = vec![0; solution.len()];
            return false;
        }

        let mut oscillation_detected = false;
        let mut max_sign_changes = 0;
        let mut any_sign_change = false;

        for (i, &curr) in solution.iter().enumerate() {
            let prev = self.prev_values[i];
            let derivative = curr - prev;
            let curr_sign = derivative >= 0.0;

            // Check for sign change in derivative
            if curr_sign != self.prev_signs[i] && derivative.abs() > 1e-12 {
                self.sign_change_count[i] += 1;
                any_sign_change = true;
            } else {
                // Reset count on smooth behavior
                self.sign_change_count[i] = 0;
            }

            max_sign_changes = max_sign_changes.max(self.sign_change_count[i]);
            self.prev_signs[i] = curr_sign;
            self.prev_values[i] = curr;
        }

        // Detect oscillation if any node has too many sign changes
        if max_sign_changes >= self.oscillation_threshold {
            oscillation_detected = true;
            self.smooth_steps = 0;
            self.current_method = IntegrationMethod::Gear2;

            // Reset sign change counts after detecting oscillation
            for count in &mut self.sign_change_count {
                *count = 0;
            }
        } else if any_sign_change {
            // Still oscillating but below threshold - stay in current method, reset smooth counter
            self.smooth_steps = 0;
        } else {
            // Truly smooth - no sign changes
            self.smooth_steps += 1;

            // Return to Trapezoidal after sufficient smooth steps
            if self.smooth_steps >= self.recovery_steps && !self.at_breakpoint {
                self.current_method = IntegrationMethod::Trapezoidal;
            }
        }

        oscillation_detected
    }

    /// Force switch to a specific method (for debugging or manual control)
    pub fn force_method(&mut self, method: IntegrationMethod) {
        self.current_method = method;
    }

    /// Reset the controller state
    pub fn reset(&mut self) {
        self.prev_values.clear();
        self.prev_signs.clear();
        self.sign_change_count.clear();
        self.smooth_steps = 0;
        self.at_breakpoint = false;
        self.current_method = IntegrationMethod::Trapezoidal;
    }

    /// Get statistics for debugging
    pub fn stats(&self) -> TrapGearStats {
        TrapGearStats {
            current_method: self.current_method,
            smooth_steps: self.smooth_steps,
            max_sign_changes: self.sign_change_count.iter().copied().max().unwrap_or(0),
        }
    }
}

impl Default for TrapGearController {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics from TrapGear controller for debugging/monitoring
#[derive(Debug, Clone)]
pub struct TrapGearStats {
    pub current_method: IntegrationMethod,
    pub smooth_steps: usize,
    pub max_sign_changes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestep_controller() {
        let mut ctrl = TimestepController::new(1e-6, 1e-12, 1e-3);

        assert_eq!(ctrl.dt(), 1e-6);

        // Large error - should decrease step
        ctrl.adjust(1e-1);
        assert!(ctrl.dt() < 1e-6);

        // Small error - should increase step
        ctrl.adjust(1e-20);
        assert!(ctrl.dt() > 1e-12);
    }

    #[test]
    fn test_breakpoint_manager() {
        let mut mgr = BreakpointManager::new();
        mgr.add(1e-3);
        mgr.add(5e-4);
        mgr.add(2e-3);

        assert_eq!(mgr.next_after(0.0), Some(5e-4));
        assert_eq!(mgr.next_after(6e-4), Some(1e-3));
    }

    #[test]
    fn test_breakpoint_limit_step_lands_exactly_for_sub_minimum_remaining_time() {
        let mut mgr = BreakpointManager::new();
        mgr.add(1.0e-9);

        let current_time = 9.995e-10;
        let proposed_dt = 2.0e-12;
        let (dt, at_breakpoint) = mgr.limit_step(current_time, proposed_dt);

        let time_to_bp = 1.0e-9 - current_time;
        assert!(at_breakpoint);
        assert!(time_to_bp < MIN_STEP_AFTER_BREAKPOINT);
        assert!((dt - time_to_bp).abs() < 1e-18);
    }

    #[test]
    fn test_trapgear_smooth_signal() {
        let mut trapgear = TrapGearController::new();

        // Smooth monotonic signal should stay in Trapezoidal
        for i in 0..10 {
            let value = i as f64 * 0.1;
            trapgear.update(&[value], 1e-6);
        }

        assert_eq!(trapgear.current_method(), IntegrationMethod::Trapezoidal);
    }

    #[test]
    fn test_trapgear_oscillation_detection() {
        let mut trapgear = TrapGearController::new();

        // Oscillating signal: up, down, up, down...
        let oscillating = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0];

        for &value in &oscillating {
            trapgear.update(&[value], 1e-6);
        }

        // Should have switched to Gear2 after detecting oscillation
        assert_eq!(trapgear.current_method(), IntegrationMethod::Gear2);
    }

    #[test]
    fn test_trapgear_recovery() {
        let mut trapgear = TrapGearController::new();

        // First, cause oscillation
        for &value in &[0.0, 1.0, 0.0, 1.0, 0.0, 1.0] {
            trapgear.update(&[value], 1e-6);
        }
        assert_eq!(trapgear.current_method(), IntegrationMethod::Gear2);

        // Now, smooth signal - should recover to Trapezoidal
        for i in 0..5 {
            trapgear.update(&[i as f64 * 0.1], 1e-6);
        }
        assert_eq!(trapgear.current_method(), IntegrationMethod::Trapezoidal);
    }

    #[test]
    fn test_trapgear_breakpoint() {
        let mut trapgear = TrapGearController::new();

        // At breakpoint, should switch to Gear2
        trapgear.set_at_breakpoint(true);
        assert_eq!(trapgear.current_method(), IntegrationMethod::Gear2);

        // Clear breakpoint and smooth signal, should return to Trapezoidal
        trapgear.set_at_breakpoint(false);
        for i in 0..5 {
            trapgear.update(&[i as f64], 1e-6);
        }
        assert_eq!(trapgear.current_method(), IntegrationMethod::Trapezoidal);
    }

    #[test]
    fn test_companion_coefficients_backward_euler() {
        let coeff = CompanionCoefficients::backward_euler();

        // G_eq = C/dt
        let geq = coeff.capacitor_geq(1e-6, 1e-9);
        assert!((geq - 1e3).abs() < 1e-6, "G_eq = {} (expected 1000)", geq);

        // I_eq = G_eq * v_n = 1000 * 5.0 = 5000
        let ieq = coeff.capacitor_ieq(1e-6, 1e-9, 5.0, 0.0, 0.0);
        assert!(
            (ieq - 5000.0).abs() < 1e-6,
            "I_eq = {} (expected 5000)",
            ieq
        );
    }

    #[test]
    fn test_companion_coefficients_trapezoidal() {
        let coeff = CompanionCoefficients::trapezoidal();

        // G_eq = 2C/dt
        let geq = coeff.capacitor_geq(1e-6, 1e-9);
        assert!((geq - 2e3).abs() < 1e-6, "G_eq = {} (expected 2000)", geq);

        // I_eq = 2 * C * v_n / dt + i_n = 2 * 1e-6 * 5.0 / 1e-9 + 2000 = 12000
        let ieq = coeff.capacitor_ieq(1e-6, 1e-9, 5.0, 0.0, 2000.0);
        assert!(
            (ieq - 12000.0).abs() < 1e-6,
            "I_eq = {} (expected 12000)",
            ieq
        );
    }

    #[test]
    fn test_companion_coefficients_gear2() {
        let coeff = CompanionCoefficients::gear2();

        // G_eq = 1.5 * C / dt = 1.5 * 1e-6 / 1e-9 = 1500
        let geq = coeff.capacitor_geq(1e-6, 1e-9);
        assert!(
            (geq - 1500.0).abs() < 1e-6,
            "G_eq = {} (expected 1500)",
            geq
        );

        // I_eq = (4*C*v_n - C*v_{n-1}) / (2*dt)
        // = (4*1e-6*5.0 - 1e-6*3.0) / (2*1e-9)
        // = (20e-6 - 3e-6) / 2e-9 = 17e-6 / 2e-9 = 8500
        // With our coefficients: 2.0*C*v_n/dt + (-0.5)*C*v_{n-1}/dt
        // = 2.0*1e-6*5.0/1e-9 + (-0.5)*1e-6*3.0/1e-9
        // = 10000 - 1500 = 8500
        let ieq = coeff.capacitor_ieq(1e-6, 1e-9, 5.0, 3.0, 0.0);
        assert!(
            (ieq - 8500.0).abs() < 1e-6,
            "I_eq = {} (expected 8500)",
            ieq
        );

        assert!(coeff.needs_two_history);
    }

    #[test]
    fn test_companion_for_method() {
        let be = CompanionCoefficients::for_method(IntegrationMethod::BackwardEuler);
        assert!((be.coeff_g - 1.0).abs() < 1e-10);

        let trap = CompanionCoefficients::for_method(IntegrationMethod::Trapezoidal);
        assert!((trap.coeff_g - 2.0).abs() < 1e-10);

        let gear = CompanionCoefficients::for_method(IntegrationMethod::Gear2);
        assert!((gear.coeff_g - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_lte_method_order_scaling() {
        let mut lte = LteEstimator::new(1e-3);

        // Test order 1 (Backward Euler): exponent = 1/2
        lte.set_method_order(1);
        let scale = lte.recommend_scale(1e-5);
        // (1e-3 / 1e-5)^(1/2) = 100^0.5 = 10, clamped to 2.0
        assert!((scale - 2.0).abs() < 1e-10);

        // Test order 2 (Trapezoidal, Gear2): exponent = 1/3
        lte.set_method_order(2);
        let scale = lte.recommend_scale(1e-6);
        // (1e-3 / 1e-6)^(1/3) = 1000^0.333 â‰ˆ 10, clamped to 2.0
        assert!((scale - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_lte_with_explicit_tolerances_uses_absolute_floor() {
        let mut lte = LteEstimator::with_tolerances(1e-3, 1e-6);
        lte.record(&[0.0], 1e-9);

        // Near-zero state should be governed by abstol floor.
        let (lte_est, accept) = lte.estimate(&[5e-7], 1e-9);
        assert!(accept, "5e-7 error should pass with 1e-6 abstol floor");
        assert!(lte_est <= 1e-3);

        let (_lte_est, reject) = lte.estimate(&[2e-6], 1e-9);
        assert!(!reject, "2e-6 error should fail with 1e-6 abstol floor");
    }

    #[test]
    fn test_lte_second_order_estimate_matches_quadratic_uniform_history() {
        let mut lte = LteEstimator::with_tolerances(1e-3, 1e-6);
        lte.set_method_order(2);

        // x(t) = t^2 sampled at t = 0, 1, 2 and predicted at t = 3.
        lte.record(&[0.0], 1.0);
        lte.record(&[1.0], 1.0);
        lte.record(&[4.0], 1.0);

        let (lte_est, accept) = lte.estimate(&[9.0], 1.0);
        assert!(accept, "quadratic motion should be accepted exactly");
        assert!(
            lte_est < 1e-12,
            "second-order predictor should be exact for quadratic history, got {lte_est}"
        );
    }

    #[test]
    fn test_lte_second_order_estimate_matches_quadratic_nonuniform_history() {
        let mut lte = LteEstimator::with_tolerances(1e-3, 1e-6);
        lte.set_method_order(2);

        // x(t) = t^2 sampled at t = 0, 1, 3 and predicted at t = 6.
        lte.record(&[0.0], 1.0);
        lte.record(&[1.0], 1.0);
        lte.record(&[9.0], 2.0);

        let (lte_est, accept) = lte.estimate(&[36.0], 3.0);
        assert!(
            accept,
            "nonuniform quadratic motion should be accepted exactly"
        );
        assert!(
            lte_est < 1e-12,
            "second-order predictor should remain exact on nonuniform quadratic history, got {lte_est}"
        );
    }

    #[test]
    fn test_lte_recommend_scale_handles_non_finite_error() {
        let lte = LteEstimator::new(1e-3);
        assert!((lte.recommend_scale(f64::NAN) - 0.25).abs() < 1e-15);
        assert!((lte.recommend_scale(f64::INFINITY) - 0.25).abs() < 1e-15);
    }

    #[test]
    fn test_richardson_extrapolation() {
        // Use tolerance of 0.01 since our test data produces LTE on that scale.
        let lte = LteEstimator::new(0.01);

        // Simulate full step and half-step results
        // For order 2 method: error factor = 2^2 - 1 = 3
        let x_full = vec![1.0, 2.0, 3.0];
        let x_half = vec![1.03, 2.06, 3.09]; // Small difference

        let (lte_est, accept) = lte.richardson_estimate(&x_full, &x_half);

        // LTE â‰ˆ |x_half - x_full| / 3
        // max(|0.03|, |0.06|, |0.09|) / 3 = 0.09 / 3 = 0.03
        // Weighted normalization keeps the value in the same order as reltol.
        assert!(
            lte_est > 0.005 && lte_est < 0.01,
            "LTE should be in weighted range (0.005, 0.01), got {}",
            lte_est
        );
        assert!(accept, "Should accept: LTE {} <= tolerance 0.01", lte_est);

        // Also test rejection with tighter tolerance
        let lte_tight = LteEstimator::new(0.001);
        let (_, accept_tight) = lte_tight.richardson_estimate(&x_full, &x_half);
        assert!(!accept_tight, "Should reject with tight tolerance");
    }

    #[test]
    fn test_lte_history_tracking() {
        let mut lte = LteEstimator::new(1e-3);

        // Record 3 solution points
        lte.record(&[1.0, 2.0], 1e-9);
        assert_eq!(lte.history_count, 1);

        lte.record(&[1.1, 2.2], 1e-9);
        assert_eq!(lte.history_count, 2);

        lte.record(&[1.2, 2.4], 1e-9);
        assert_eq!(lte.history_count, 3);

        // Reset should clear all history
        lte.reset();
        assert_eq!(lte.history_count, 0);
    }

    //=========================================================================
    // Charge-Based LTE Tests
    //=========================================================================

    #[test]
    fn test_charge_lte_perfect_conservation() {
        let lte = LteEstimator::new(1e-3);

        // Perfect charge conservation: Q_new = Q_old + dt/2 * (I_old + I_new)
        let dt = 1e-9;
        let prev_charges = vec![1e-15, 2e-15]; // 1fF, 2fF
        let prev_currents = vec![1e-6, 2e-6]; // 1uA, 2uA
        let currents = vec![1e-6, 2e-6]; // Same current

        // Perfect integration: Q_new = Q_old + dt * I (since I_old = I_new)
        let charges = vec![
            prev_charges[0] + dt * prev_currents[0],
            prev_charges[1] + dt * prev_currents[1],
        ];

        let (lte_est, accept) =
            lte.estimate_charge_lte(&charges, &prev_charges, &currents, &prev_currents, dt);

        assert!(
            lte_est < 1e-10,
            "Perfect conservation should have ~0 LTE, got {}",
            lte_est
        );
        assert!(accept, "Should accept perfect conservation");
    }

    #[test]
    fn test_charge_lte_with_error() {
        let lte = LteEstimator::new(1e-3);

        let dt = 1e-9;
        let prev_charges = vec![1e-15];
        let prev_currents = vec![1e-6];
        let currents = vec![1e-6];

        // Expected: Q = 1e-15 + 1e-9 * 1e-6 = 1e-15 + 1e-15 = 2e-15
        // Actual with 10% error
        let charges = vec![2.2e-15]; // 10% error

        let (lte_est, accept) =
            lte.estimate_charge_lte(&charges, &prev_charges, &currents, &prev_currents, dt);

        // Error: |2.2e-15 - 2e-15| / 2.2e-15 = 0.2e-15 / 2.2e-15 â‰ˆ 0.09
        assert!(lte_est > 0.05, "Should detect 10% error, got {}", lte_est);
        assert!(!accept, "Should reject 10% charge error");
    }

    #[test]
    fn test_charge_lte_empty_inputs() {
        let lte = LteEstimator::new(1e-3);

        // Empty arrays should return (0.0, true)
        let (lte_est, accept) = lte.estimate_charge_lte(&[], &[], &[], &[], 1e-9);
        assert_eq!(lte_est, 0.0);
        assert!(accept);
    }

    #[test]
    fn test_charge_lte_mismatched_lengths() {
        let lte = LteEstimator::new(1e-3);

        // Mismatched array lengths should return (0.0, true)
        let (lte_est, accept) = lte.estimate_charge_lte(&[1.0, 2.0], &[1.0], &[1.0], &[1.0], 1e-9);
        assert_eq!(lte_est, 0.0);
        assert!(accept);
    }

    #[test]
    fn test_combined_lte_voltage_only() {
        let mut lte = LteEstimator::new(1e-3);

        // Record some history
        lte.record(&[1.0, 2.0], 1e-9);
        lte.record(&[1.1, 2.2], 1e-9);

        // Test combined without charges
        let (combined_lte, accept) = lte.estimate_combined(&[1.2, 2.4], 1e-9, None);

        // Should be same as voltage-only LTE
        let (v_lte, v_accept) = lte.estimate(&[1.2, 2.4], 1e-9);
        assert!((combined_lte - v_lte).abs() < 1e-15);
        assert_eq!(accept, v_accept);
    }

    #[test]
    fn test_combined_lte_with_charges() {
        let mut lte = LteEstimator::new(1e-3);

        // Record some history
        lte.record(&[1.0, 2.0], 1e-9);
        lte.record(&[1.0, 2.0], 1e-9); // Constant voltage (no voltage error)

        let dt = 1e-9;
        // Charge with significant error
        let q_curr = vec![2.5e-15];
        let q_prev = vec![1e-15];
        let i_curr = vec![1e-6];
        let i_prev = vec![1e-6];
        // Expected: 1e-15 + 1e-9 * 1e-6 = 2e-15, actual 2.5e-15 = 25% error

        let (combined_lte, accept) =
            lte.estimate_combined(&[1.0, 2.0], dt, Some((&q_curr, &q_prev, &i_curr, &i_prev)));

        // Combined should catch the charge error
        assert!(
            combined_lte > 0.1,
            "Should detect charge error, got {}",
            combined_lte
        );
        assert!(!accept, "Should reject due to charge error");
    }
}
