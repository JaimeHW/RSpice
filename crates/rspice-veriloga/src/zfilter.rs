//! Z-domain (sampled-data) filters for the `zi_*` Verilog-A operators.
//!
//! The input samples every `period` seconds; on each sample instant the
//! difference equation
//!
//!   y[n] = (b0·x[n] + b1·x[n-1] + ... - a1·y[n-1] - ...) / a0
//!
//! updates, and the output holds (zero-order hold) between samples.
//! Coefficient arrays are ascending in z⁻¹ (b0, b1, ...), matching the
//! ascending-s convention of the `laplace_*` operators.
//!
//! In DC analyses the filter sits at its steady state, y = H(1)·x. The
//! small-signal Jacobian factor is H(1) at DC, b0/a0 on transient steps
//! that sample, and 0 while holding.
//!
//! Newton iterations re-evaluate the same timepoint repeatedly, so
//! evaluation never mutates committed history: it computes a candidate
//! from the last *accepted* state and the engine commits it when the
//! step is accepted (`commit`), exactly like the ddt/idt state pattern.

use serde::{Deserialize, Serialize};

/// Tolerance for sample-instant comparison, as a fraction of the period
const SAMPLE_TOL: f64 = 1e-9;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiFilter {
    /// Numerator coefficients b_k (ascending powers of z⁻¹)
    num: Vec<f64>,
    /// Denominator coefficients a_k (a_0 first); a_0 must be nonzero
    den: Vec<f64>,
    /// Sample period (s)
    period: f64,
    /// Committed input history x[n-1], x[n-2], ...
    x_hist: Vec<f64>,
    /// Committed output history y[n-1], y[n-2], ...
    y_hist: Vec<f64>,
    /// Committed held output between samples
    held: f64,
    /// Next sample instant
    next_sample: f64,
    /// Candidate sample from the in-flight evaluation
    candidate_x: f64,
    candidate_y: f64,
    /// Whether the in-flight evaluation lies on a sample instant
    sampling_now: bool,
}

impl ZiFilter {
    pub fn new(num: Vec<f64>, den: Vec<f64>, period: f64) -> Self {
        let x_len = num.len().saturating_sub(1);
        let y_len = den.len().saturating_sub(1);
        Self {
            num,
            den,
            period,
            x_hist: vec![0.0; x_len],
            y_hist: vec![0.0; y_len],
            held: 0.0,
            next_sample: 0.0,
            candidate_x: 0.0,
            candidate_y: 0.0,
            sampling_now: false,
        }
    }

    /// DC gain H(1) = Σb / Σa
    pub fn dc_gain(&self) -> f64 {
        let num: f64 = self.num.iter().sum();
        let den: f64 = self.den.iter().sum();
        if den.abs() > 0.0 { num / den } else { 0.0 }
    }

    /// Instantaneous feedthrough b0/a0 (the Jacobian factor on sampling
    /// steps)
    pub fn feedthrough(&self) -> f64 {
        let a0 = self.den.first().copied().unwrap_or(1.0);
        let b0 = self.num.first().copied().unwrap_or(0.0);
        if a0.abs() > 0.0 { b0 / a0 } else { 0.0 }
    }

    /// Whether the in-flight evaluation lies on a sample instant
    pub fn sampling_now(&self) -> bool {
        self.sampling_now
    }

    /// Evaluate at `time` with the present `input`. Repeated calls at the
    /// same timepoint (Newton iterations) recompute the candidate from
    /// committed history, so they are idempotent.
    pub fn eval(&mut self, input: f64, time: f64, transient: bool) -> f64 {
        if !transient || self.period <= 0.0 {
            // Equilibrium analyses hold the steady state y = H(1)·x
            self.sampling_now = false;
            return self.dc_gain() * input;
        }

        let tol = self.period * SAMPLE_TOL;
        if time + tol < self.next_sample {
            self.sampling_now = false;
            return self.held;
        }

        // Sample instant: y = (b·[x, x_hist] - a[1..]·y_hist) / a0
        self.sampling_now = true;
        self.candidate_x = input;
        let a0 = self.den.first().copied().unwrap_or(1.0);
        if a0.abs() == 0.0 {
            self.candidate_y = 0.0;
            return 0.0;
        }
        let mut acc = self.num.first().copied().unwrap_or(0.0) * input;
        for (k, &b) in self.num.iter().enumerate().skip(1) {
            acc += b * self.x_hist.get(k - 1).copied().unwrap_or(0.0);
        }
        for (k, &a) in self.den.iter().enumerate().skip(1) {
            acc -= a * self.y_hist.get(k - 1).copied().unwrap_or(0.0);
        }
        self.candidate_y = acc / a0;
        self.candidate_y
    }

    /// Commit the in-flight candidate after the engine accepts the step
    /// ending at `time`
    pub fn commit(&mut self, time: f64) {
        if self.period <= 0.0 {
            return;
        }
        let tol = self.period * SAMPLE_TOL;
        if time + tol < self.next_sample {
            return;
        }
        if self.sampling_now {
            if !self.x_hist.is_empty() {
                self.x_hist.rotate_right(1);
                self.x_hist[0] = self.candidate_x;
            }
            if !self.y_hist.is_empty() {
                self.y_hist.rotate_right(1);
                self.y_hist[0] = self.candidate_y;
            }
            self.held = self.candidate_y;
        }
        // Schedule the next instant strictly after the accepted time
        // (large steps skip missed instants under the ZOH assumption)
        while self.next_sample <= time + tol {
            self.next_sample += self.period;
        }
        self.sampling_now = false;
    }
}

/// Expand z-plane roots into z⁻¹-ascending polynomial coefficients:
/// Π(z - r_k) gives ascending-in-z coefficients, which reverse into the
/// difference-equation form (highest power of z becomes z⁰)
pub fn z_roots_to_coefficients(roots: &[(f64, f64)]) -> Result<Vec<f64>, String> {
    let ascending_in_z = crate::laplace::roots_to_polynomial(roots)?;
    let mut coefficients = ascending_in_z;
    coefficients.reverse();
    Ok(coefficients)
}
